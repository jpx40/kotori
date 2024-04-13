use crate::{prelude::*, reader};
use strum::{Display, EnumString};
use tauri::EventTarget;

#[derive(Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Event {
  BookAdded(Json),
  BookRemoved(i32),
  CoverExtracted { id: i32, path: PathBuf },
  PageDeleted { window_id: u16, page: usize },
  RatingUpdated { id: i32, rating: u8 },
}

impl Event {
  pub fn emit(self, app: &AppHandle) -> Result<()> {
    let event = self.to_string();
    match self {
      Event::PageDeleted { window_id, .. } => {
        let target = Target::ReaderWindow(window_id);
        app.emit_to(target, &event, Json::from(self))?;
      }
      _ => app.emit_to(Target::MainWindow, &event, Json::from(self))?,
    };

    Ok(())
  }
}

impl From<Event> for Json {
  fn from(event: Event) -> Self {
    match event {
      Event::BookAdded(value) => value,
      Event::BookRemoved(id) => json!({ "id": id }),
      Event::CoverExtracted { id, path } => json!({ "id": id, "path": path }),
      Event::PageDeleted { page, .. } => json!({ "page": page }),
      Event::RatingUpdated { id, rating } => json!({ "id": id, "rating": rating }),
    }
  }
}

#[derive(Debug)]
pub enum Target {
  MainWindow,
  ReaderWindow(u16),
}

impl From<Target> for EventTarget {
  fn from(target: Target) -> Self {
    match target {
      Target::MainWindow => EventTarget::WebviewWindow {
        label: "main".into(),
      },
      Target::ReaderWindow(window_id) => EventTarget::WebviewWindow {
        label: reader::label(window_id),
      },
    }
  }
}
