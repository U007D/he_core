use fu740_hal::{
    clock::{Clocks, PrciExt},
    pac::PRCI,
    time::U32Ext,
};

pub fn init(prci: PRCI) -> Clocks {
    let clock_setup = prci.setup().coreclk(1001.mhz());
    clock_setup.freeze()
}
