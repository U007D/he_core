pub mod argument;

use core::fmt::{Display, Formatter, Result as FmtResult};

use crate::consts::*;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    ArgumentError(argument::Error),
    NoPeripherals,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use Error::*;
        match self {
            ArgumentError(arg_err) => write!(f, "{arg_err}"),
            NoPeripherals => write!(f, "{}", msg::PANIC_NO_PERIPHERALS),
        }
    }
}

impl From<argument::Error> for Error {
    fn from(err: argument::Error) -> Self {
        Self::ArgumentError(err)
    }
}