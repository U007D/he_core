use fu740_hal::{clock::PrciExt, pac::Peripherals, time::U32Ext};
use fu740_hal::pac::PRCI;

use crate::{Error, Result};

mod dram;
mod eth;

pub fn init() -> Result<()> {
    let peripherals = Peripherals::take().ok_or_else(|| Error::NoPeripherals)?;
    let clock_setup = peripherals.PRCI.setup();
    let clock_setup = clock_setup.coreclk(1001.mhz());
    let _clocks = clock_setup.freeze()?;
    Ok(())
}
