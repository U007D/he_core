mod bss;
mod clocks;
mod dram;
mod eth;
mod l2_cache;
mod periphery;

use crate::{consts::*, processor::Processor, traits::IProcessor};
use bss::init_bss;
use fu740_hal::pac::Peripherals;
use riscv::register::mhartid;

extern "C" {
    static _BSS_START: usize;
    static _BSS_END: usize;
}

// This function is called by `Processor::boot()`, the entry point into the program, as specified by the linker script.
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn init() {
    // Park all cores except core 0
    park_non_zero_core_id();

    // Initialize `.bss` section with zeros
    let (sbss, ebss) = unsafe { (&_BSS_START as *const usize as usize, &_BSS_END as *const usize as usize) };
    init_bss(sbss, ebss);

    // TODO: move bootloader to flash and set `fu740_hal::DEVICE_PERIPHERALS` static storage to L2_LIM at 0x0800_0000
    let peripherals = Peripherals::take().unwrap_or_else(|| unreachable!(msg::PANIC_NO_PERIPHERALS));

    // TODO: Move impl to `fu740-hal`
//    dram::init(&peripherals.PRCI);

    // Init all Power Reset Clock Interrupt devices
    let _ = clocks::init(peripherals.PRCI);

    // TODO: implement; move impl to `fu740-hal`
//    periphery::init();

    // TODO: implement; move impl to `fu740-hal`
//    l2_cache::init();
}

fn park_non_zero_core_id() {
    // Park all `hart`s except `hart` 0
    if mhartid::read() != 0 {
        Processor::park()
    }
}
