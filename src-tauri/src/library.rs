use crate::book::{ActiveBook, Cover, IntoValue, LibraryBook};
use crate::database::prelude::*;
use crate::event::Event;
use crate::prelude::*;
use crate::utils::glob;
use tauri_plugin_dialog::{DialogExt, FileDialogBuilder};
use walkdir::WalkDir;

pub struct Library {
  app: AppHandle,
}

impl Library {
  pub fn new(app: &AppHandle) -> Self {
    Self { app: app.clone() }
  }

  pub async fn add_from_dialog(&self) -> Result<()> {
    let (tx, rx) = oneshot::channel();
    let dialog = self.app.dialog().clone();

    FileDialogBuilder::new(dialog).pick_folders(move |response| {
      tx.send(response.unwrap_or_default()).ok();
    });

    let folders = rx.await?;
    if folders.is_empty() {
      return Ok(());
    }

    let globset = glob::book();
    let mut books = Vec::new();

    for folder in folders {
      for entry in WalkDir::new(&folder).into_iter().flatten() {
        let path = entry.into_path();
        if path.is_file() && globset.is_match(&path) {
          books.push(path);
        }
      }
    }

    Self::save_books(&self.app, books).await
  }

  async fn save_book(app: &AppHandle, path: &Path) -> Result<()> {
    let path = path
      .to_str()
      .map(ToOwned::to_owned)
      .ok_or_else(|| err!(InvalidPath, "{}", path.display()))?;

    let model = BookActiveModel {
      id: NotSet,
      path: Set(path),
      rating: NotSet,
      cover: NotSet,
    };

    let on_conflict = OnConflict::column(BookColumn::Path)
      .do_nothing()
      .to_owned();

    let kotori = app.state::<Kotori>();
    let book = Book::insert(model)
      .on_conflict(on_conflict)
      .exec_with_returning(&kotori.db)
      .await?;

    let payload = LibraryBook(app, &book).into_value().await?;
    Event::BookAdded(payload).emit(app)?;

    let active_book = ActiveBook::with_model(&book)?;
    let cover = Cover::path(app, book.id)?;
    active_book.extract_cover(app, cover);

    Ok(())
  }

  async fn save_books<I>(app: &AppHandle, paths: I) -> Result<()>
  where
    I: IntoIterator<Item = PathBuf>,
  {
    let tasks = paths.into_iter().map(|path| {
      let app = app.clone();
      async_runtime::spawn(async move {
        Self::save_book(&app, &path).await?;
        Ok::<(), Error>(())
      })
    });

    join_all(tasks).await;

    Ok(())
  }
}
