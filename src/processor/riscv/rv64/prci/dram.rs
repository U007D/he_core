use core::arch::asm;
use fu740_hal::pac::PRCI;

mod ddr_data;

// Required to write to registers whose `.svd` definition does not specify valid vs invalid bit patterns
#[allow(unsafe_code)]
pub fn init(prci: &PRCI) {
    // Following initialization procedure described in
    // [fu740 manual](https://sifive.cdn.prismic.io/sifive/de1491e5-077c-461d-9605-e8a0ce57337d_fu740-c000-manual-v1p3.pdf).
    // Step 1 - Configure `ddr_pllcfg` (DRAM control clock PLL)
    init_ddr_pllcfg(&prci);

    // Step 2 - Bring DDR subsystem out of reset
    take_ddr_out_of_reset(&prci);

    // Step 3 + 6 - Configure DDR controller
    configure_ddr_controller();

    // Step 4 + 5 - Configure DDR PHY
    configure_ddr_phy();

    // Step 7 - Disable DDR interrupts
    disable_ddr_interrupts();

    // Step 8 + 9 + 10 - initialize DDR subsystem
    init_ddr_subsystem();

    // TODO: Determine where this goes
    // Release lock gate (clock glitch suppressor)
    prci.ddr_plloutdiv.modify(|_, w| w.pllcke().set_bit());

    // TODO: Enable L2 cache is for all cores, once primary bootloader is runnng from flash)
}

#[allow(unsafe_code)]
fn configure_ddr_controller() {
    use ddr_data::{DDR_CTL_BASE_PTR, DDR_CTL_CONFIG};

    DDR_CTL_CONFIG.iter().enumerate().for_each(|(offset, &data)| {
        unsafe { *DDR_CTL_BASE_PTR.add(offset) = data };
    });
}

#[allow(unsafe_code)]
fn configure_ddr_phy() {
    use ddr_data::{DDR_PHY_BASE_PTR, DDR_PHY_CONFIG};

    DDR_PHY_CONFIG.iter().enumerate().for_each(|(offset, &data)| {
        unsafe { *DDR_PHY_BASE_PTR.add(offset) = data };
    });
}

#[allow(unsafe_code)]
fn disable_ddr_interrupts() {
    use ddr_data::{DDR_CTL_BASE_PTR, DDR_CTL_REGISTER_136, DDR_CTL_REGISTER_136_DISABLE_ALL_INTERRUPTS};

    unsafe { *DDR_CTL_BASE_PTR.add(DDR_CTL_REGISTER_136) = DDR_CTL_REGISTER_136_DISABLE_ALL_INTERRUPTS };
}

#[allow(unsafe_code)]
fn init_ddr_pllcfg(prci: &PRCI) {
    prci.ddr_pllcfg.modify(|_, w| unsafe {
        // values computed via [solver](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=6f78bf1b6134600ac481b64fbf7e0339)
        w.pllr().bits(0);
        w.pllf().bits(45);
        w.pllq().bits(2)
    });
    // Busy wait for pll lock
    while prci.ddr_pllcfg.read().plllock().bit_is_set() {}
}

#[allow(unsafe_code)]
fn init_ddr_subsystem() {
    use ddr_data::{
        DDR_CTL_BASE_PTR, DDR_CTL_REGISTER_0_START_MASK, DDR_CTL_REGISTER_132, DDR_CTL_REGISTER_132_INT_STATUS_8,
        DDR_PHYSICAL_FILTER, DDR_PHYSICAL_FILTER_PMP_0_INIT,
    };

    unsafe {
        *DDR_CTL_BASE_PTR |= DDR_CTL_REGISTER_0_START_MASK;
        while *DDR_CTL_BASE_PTR.add(DDR_CTL_REGISTER_132) & DDR_CTL_REGISTER_132_INT_STATUS_8 != 0 {}
        *DDR_PHYSICAL_FILTER = DDR_PHYSICAL_FILTER_PMP_0_INIT;
    }
}

#[allow(unsafe_code)]
fn take_ddr_out_of_reset(prci: &PRCI) {
    // Step 2a - Release DDR controller reset
    prci.devices_reset_n.write(|w| w.ddrctrl_reset_n().set_bit());

    // Step 2b - Wait (at least) one `ddrctrlclk` cycle (@600MHz), per § 23.3 step 2b
    //           https://sifive.cdn.prismic.io/sifive/18febb04-50b6-4880-9bf3-631e40daa809_fu740-c000-manual-v1p2.pdf
    //           CPU @ 1GHz => ceil(1GHz / 600MHz * 1 cycle)) ~= 2 `coreclk` cycles
    unsafe {
        asm! { "
            // Init `mcycle`
            csrw mcycle, zero         // set cycle counter to 0 (ensure no counter wrapping)

            // Set wait duration in clock cycles
            addi t1, zero, 2          // set exit condition to 1 `ddrctrlclk` ~= 2 `corectrlclk` cycles into the future
                                    // (almost certainly already elapsed)

            // Busy wait for the required duration
            2:
            csrr t0, mcycle           // read current cycle count
            bltu t0, t1, 2b           // exit when target number of cycles have elapsed
        " }
    }

    // Step 2c - Release DDR controller register interface reset and DDR Subsystem PHY reset
    prci.devices_reset_n.write(|w| w.ddraxi_reset_n().set_bit().ddrahb_reset_n().set_bit().ddrphy_reset_n().set_bit());

    // Step 2d - Wait (at least) 256 `ddrctrlclk` cycles (@600MHz), per § 23.3 step 2d
    //           https://sifive.cdn.prismic.io/sifive/18febb04-50b6-4880-9bf3-631e40daa809_fu740-c000-manual-v1p2.pdf
    //           CPU @ 1GHz => ceil(1GHz / 600MHz * 256 cycles) ~= 427 `coreclk` cycles
    unsafe {
        asm! { "
            // Set wait duration in clock cycles
            csrr t0, mcycle
            addi t1, t0, 427          // set exit condition to 256 `ddrctrlclk` ~= 427 `corectrlclk` cycles into the future

            // Busy wait for the required duration
            3:
            csrr t0, mcycle           // read current cycle count
            bltu t0, t1, 3b           // exit when target number of cycles have elapsed
        " }
    }
}
