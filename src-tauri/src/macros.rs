#[macro_export]
macro_rules! err {
  ($e:ident) => {
    $crate::error::Error::$e
  };
  ($e:ident, $($arg:tt)*) => {
    $crate::error::Error::$e(format!($($arg)*))
  };
}

#[macro_export]
macro_rules! bail {
  ($e:ident) => {
    return Err($crate::err!($e));
  };
  ($e:ident, $($arg:tt)*) => {
    return Err($crate::err!($e, $($arg)*));
  };
}

#[macro_export]
macro_rules! get_windows {
  ($app:expr) => {{
    let kotori = $app.state::<$crate::Kotori>();
    let reader = kotori.reader.read().await;
    reader.windows()
  }};
}
