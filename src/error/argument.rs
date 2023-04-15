use core::{
    ffi::CStr,
    fmt::{Display, Formatter, Result as FmtResult},
    str::Utf8Error,
};

use crate::consts::msg;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    NonUtf8Arg(&'static CStr, Utf8Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use Error::*;
        match self {
            NonUtf8Arg(arg, utf8_err) => write!(f, "{}: {:?} ({:?})", msg::ERR_NON_UTF8_ARG, arg, utf8_err),
        }
    }
}

impl From<(&'static CStr, Utf8Error)> for Error {
    fn from((c_str, utf8_err): (&'static CStr, Utf8Error)) -> Self {
        Self::NonUtf8Arg(c_str, utf8_err)
    }
}