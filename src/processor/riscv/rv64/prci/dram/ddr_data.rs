pub use ddr_controller::{
    DDR_CTL_BASE_PTR, DDR_CTL_CONFIG, DDR_CTL_LEN, DDR_CTL_REGISTER_0_START_MASK, DDR_CTL_REGISTER_132,
    DDR_CTL_REGISTER_132_INT_STATUS_8, DDR_CTL_REGISTER_136, DDR_CTL_REGISTER_136_DISABLE_ALL_INTERRUPTS,
    DDR_PHYSICAL_FILTER, DDR_PHYSICAL_FILTER_PMP_0_INIT,
};
pub use ddr_phy::{DDR_PHY_BASE_PTR, DDR_PHY_CONFIG, DDR_PHY_LEN};

mod ddr_controller;
mod ddr_phy;
