#[cfg(target_arch = "riscv32")]
mod rv32;
#[cfg(target_arch = "riscv64")]
mod rv64;

#[cfg(target_arch = "riscv128")]
mod rv128;

#[cfg(target_arch = "riscv32")]
pub use rv32::Processor;

#[cfg(target_arch = "riscv64")]
pub use rv64::Processor;

#[cfg(target_arch = "riscv32")]
pub use rv128::Processor;
