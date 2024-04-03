use crate::prelude::*;
use crate::reader::BookMap;
use crate::VERSION;
use axum::extract::{Path, State};
use axum::http::{HeaderValue, StatusCode};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use dioxus_ssr::render_element;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

pub fn serve(app: &AppHandle) {
  let app = app.clone();
  thread::spawn(move || {
    async_runtime::block_on(async move {
      let kotori = app.state::<Kotori>();
      let reader = kotori.reader.lock().await;
      let books = reader.books();

      drop(reader);

      let mut router = Router::new()
        .route("/library/:book/cover", get(book_cover))
        .route("/reader", get(reader_root))
        .route("/reader/:book/:page", get(book_page))
        .with_state(books);

      if tauri::dev() {
        let origin = HeaderValue::from_static("http://localhost:1422");
        let layer = CorsLayer::new().allow_origin(origin);
        router = router.layer(layer);
      }

      let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
      axum::serve(listener, router).await.unwrap();
    });
  });
}

async fn reader_root(State(books): State<BookMap>) -> Html<String> {
  use dioxus::prelude::*;

  let books = books.lock().await;
  let amount = books.len();

  let rows = books
    .iter()
    .sorted_unstable_by_key(|(id, _)| *id)
    .map(|(id, (book, _))| (id, &book.title));

  let html = rsx! {
    head { title { "Kotori {VERSION}" } }
    body {
      p { "Active books: {amount}" }
      table {
        for (id, title) in rows {
          tr { td { "{id}" } td { "{title}" } }
        }
      }
    }
  };

  Html(render_element(html))
}

async fn book_cover(State(books): State<BookMap>, Path(book): Path<u16>) -> Response {
  let mut books = books.lock().await;
  let book = books.get_mut(&book).map(|(b, _)| b);

  let Some(book) = book else {
    return err!(BookNotFound).into_response();
  };

  match book.file.get_cover_as_bytes() {
    Ok(bytes) => (StatusCode::OK, bytes).into_response(),
    Err(err) => err.into_response(),
  }
}

async fn book_page(
  State(books): State<BookMap>,
  Path((book, page)): Path<(u16, usize)>,
) -> Response {
  let mut books = books.lock().await;
  let book = books.get_mut(&book).map(|(b, _)| b);

  let Some(book) = book else {
    return err!(BookNotFound).into_response();
  };

  match book.file.get_page_as_bytes(page) {
    Ok(bytes) => (StatusCode::OK, bytes).into_response(),
    Err(err) => err.into_response(),
  }
}
