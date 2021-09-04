use core::fmt::{Display, Formatter, Result as FmtResult};

use crate::consts::*;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    NoPeripherals,
    HalError(fu740_hal::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", match self {
            NoPeripherals => msg::PANIC_NO_PERIPHERALS,
            //HardwareError(hal_err) =>
        })
    }
}

impl From<fu740_hal::Error> for Error {
    fn from(error: fu740_hal::Error) -> Self {
        Self::HalError(error)
    }
}
