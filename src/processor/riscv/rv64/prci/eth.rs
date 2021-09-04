use fu740_hal::pac::Peripherals;

// Required to write to registers whose `.svd` definition does not specify valid vs invalid bit patterns
#[allow(unsafe_code)]
pub fn init(peripherals: &mut Peripherals) {
    // Configure `gemgxlpll` (Ethernet) for 125Mhz (see fu740-c000-manual-v1p2, p. 84)
    peripherals.PRCI.gemgxl_pllcfg.modify(|_, w| unsafe {
        w.pllr().bits(0);
        w.pllf().bits(76);
        w.pllq().bits(5)
    });
    // Busy wait for pll lock
    while peripherals.PRCI.gemgxl_pllcfg.read().plllock().bit_is_set() {}

    // Release lock gate (clock glitch suppressor)
    peripherals.PRCI.gemgxl_plloutdiv.modify(|_, w| w.pllcke().set_bit());
    // Release reset
    peripherals.PRCI.devices_reset_n.write(|w| w.gemgxl_reset_n().set_bit());
}
