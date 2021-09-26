use core::fmt::{Display, Formatter, Result as FmtResult};

use crate::consts::*;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    NoPeripherals,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::NoPeripherals => write!(f, "{}", msg::PANIC_NO_PERIPHERALS),
        }
    }
}
