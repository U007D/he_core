use fu740_hal::pac::Peripherals;

// Required to write to registers whose `.svd` definition does not specify valid vs invalid bit patterns
#[allow(unsafe_code, clippy::similar_names)]
pub fn init(peripherals: &mut Peripherals) {
  enum CoreClockSource {
    CoreClkPll = 0,
    HfXOsc = 1,
  }

  // TODO: Determine if hart must be switched to external oscillator while setting core_clk, determine `hfxoscen` sense
  // // Enable hfx (high-frequency external) oscillator
  // peripherals.PRCI.hfxosccfg.modify(|_, w| w.hfxoscen().set_bit());
  // // Busy wait for frequency lock
  // while peripherals.PRCI.hfxosccfg.read().hfxoscrdy().bit_is_clear() {}
  // // (Temporarily) select external high frequency clock as coreclk source
  // peripherals.PRCI.core_clk_sel_reg.modify(|_, w| unsafe { w.bits(CoreClockSource::HfXOsc as _) });

  // Configure `corepll` for 1 GHz (see fu740-c000-manual-v1p2, p. 84)
  peripherals.PRCI.core_pllcfg.modify(|_, w| unsafe {
    w.pllr().bits(0);
    w.pllf().bits(76);
    w.pllq().bits(2)
  });
  // Busy wait for pll lock
  // TODO: determine `plllock` sense: active low?
  while peripherals.PRCI.core_pllcfg.read().plllock().bit_is_set() {}

  // Select coreclkpll as coreclk source
  peripherals.PRCI.core_clk_sel_reg.modify(|_, w| unsafe { w.bits(CoreClockSource::CoreClkPll as _) });
}

