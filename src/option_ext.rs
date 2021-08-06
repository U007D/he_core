pub trait OptionExt {
  fn expect_none(self, msg: &str);
}

impl<T> OptionExt for Option<T> {
  fn expect_none(self, msg: &str) {
    match self {
      None => (),
      Some(_) => panic!("{}", msg),
    }
  }
}
