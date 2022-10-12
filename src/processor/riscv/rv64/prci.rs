mod clocks;
mod dram;
mod eth;
mod l2_cache;
mod periphery;

use crate::{consts::*, processor::Processor, traits::IProcessor};
use fu740_hal::pac::Peripherals;
use riscv::register::mhartid;

extern "C" {
    static _BSS_START: usize;
    static _BSS_END: usize;
}

// This function is called by `Processor::start()`
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn init_core() {
    let peripherals = Peripherals::take().unwrap_or_else(|| unreachable!(msg::PANIC_NO_PERIPHERALS));

    // Init all Power Reset Clock Interrupt devices
    let _ = clocks::init(peripherals.PRCI);

    // TODO: Move impl to `fu740-hal`
//    dram::init(&peripherals.PRCI);

    // TODO: implement; move impl to `fu740-hal`
//    periphery::init();

    // TODO: implement; move impl to `fu740-hal`
//    l2_cache::init();
}

fn park_non_zero_core_id() {
    // Park all `hart`s except `hart` 0
    if mhartid::read() != 0 {
        Processor::halt()
    }
}
