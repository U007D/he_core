#[cfg(any(target_arch = "riscv32", target_arch = "riscv64", target_arch = "riscv128"))]
mod riscv;

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64", target_arch = "riscv128"))]
pub use riscv::Processor;
