pub mod library {
  pub mod book {
    use crate::book::ActiveBook;
    use crate::menu::prelude::*;
    use crate::prelude::*;

    #[derive(Display, EnumString)]
    enum Id {
      OpenBook,
    }

    pub fn build<M, R>(app: &M) -> Result<Menu<R>>
    where
      R: Runtime,
      M: Manager<R>,
    {
      let menu = MenuBuilder::new(app)
        .items(&[&menu_item!(app, Id::OpenBook, "Open")?])
        .build()?;

      Ok(menu)
    }

    pub fn on_menu_event<R>(app: &AppHandle, book_id: i32) -> impl Fn(&Window<R>, MenuEvent)
    where
      R: Runtime,
    {
      let app = app.clone();
      move |_, event| {
        if let Ok(id) = Id::from_str(event.id.0.as_str()) {
          match id {
            Id::OpenBook => open_book(&app, book_id),
          }
        }
      }
    }

    pub fn open_book(app: &AppHandle, id: i32) {
      let app = app.clone();
      async_runtime::spawn(async move {
        if let Ok(book) = ActiveBook::from_id(&app, id).await {
          book.open(&app).await.ok();
        }
      });
    }
  }
}
