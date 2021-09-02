mod core_clock;
mod eth;
mod sdram;

use crate::{consts::*, traits::IProcessor};
use fu740_hal::pac::Peripherals;

extern "Rust" {
  fn main(peripherals: Peripherals) -> !;
}

pub struct Processor;

impl IProcessor for Processor {
  // Define `.boot` section so linker can explicitly place the `boot` function.
  #[link_section = ".boot"]
  // `[no_mangle]` is both `unsafe` and required in order for the entry point to be recognized by
  // the linker
  #[naked]
  #[allow(unsafe_code)]
  #[no_mangle]
  // Enable use of linker-defined values in inline `asm!`--all other labels defined per
  // [Rust inline `asm!` documentation](https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels)
  #[allow(named_asm_labels)]
  extern "C" fn boot() -> ! {
    #[allow(unsafe_code)]
        unsafe {
      // There are a number of ways to import the linker symbols we want to use into inline
      // assembly. Inline `asm!` uses `RISC-V`'s "relative" addressing modes, which limits
      // address to +/- ~524_287 bytes away from the current program counter.  Setting the
      // stack pointer to the end of 16GB RAM far exceeds this limit.  Thus we use a less
      // straightforward method of loading addresses--load them as ordinary 64-bit values and
      // operate on the in address-indirect mode in assembly.
      #[rustfmt::skip]
      asm! {
      // Park all `hart`s except `hart` 0
      "csrr t0, mhartid",
      "bnez t0, park",

      // Initialize `.bss`
      // Ensure symbol offsets compile even when RISC-V code model does not support full 64-bit offsets
      "la t0, BSS_START",
      "ld t0, 0(t0)",
      "la t1, BSS_END",
      "ld t1, 0(t1)",
      // If `.bss_end` is 0, exclusive end wrapped address space or `BSS` size is 0.  Either way, skip negative check.
      "beq t1, zero, 2f",
      // Is `.bss` size negative (when `.bss_end` is non-zero)?  If yes, memory layout is misconfigured.
      "bgtu t0, t1, 5f",
      "2:",
      // Are *both* `.bss_start` and `.bss_end` 0? Zero-sized block; `BSS` initialization will not occur
      "or t2, t0, t1",
      "beq t2, zero, 3f",
      // Make `.bss_end` inclusive to eliminate wrapping artifacts
      "addi t1, t1, -1",

      // Zero out BSS
      "3:",
      "bgtu t0, t1, 4f",
      "sd zero, (t0)",
      "addi t0, t0, 8",
      "j 3b",
      "4:",

      // Initialize the stack
      // Ensure symbol offsets compile even when RISC-V code model does not support full 64-bit offsets
      "la t0, STACK_BASE",
      "ld sp, 0(t0)",

      // Finish hardware initialization and run `main()`
      "j finalize",

      // Whoops bad configuration; halt
      // TODO: Indicate error condition using LEDs
      "5:",
      "unimp",

      // `.data`
      "BSS_START: .dword _BSS_START",
      "BSS_END: .dword _BSS_END",
      "STACK_BASE: .dword _STACK_BASE",
      options(noreturn)
      }
    }
  }

  // `[no_mangle]` is both `unsafe` and required in order for the entry point to be recognized by
  // the linker
  #[allow(unsafe_code)]
  #[naked]
  #[no_mangle]
  extern "C" fn park() -> ! {
    #[allow(unsafe_code)]
        unsafe {
      #[rustfmt::skip]
      asm! {
      // Put the `hart` to sleep (wait for `ifi`)
      "wfi",
      "j park",
      options(noreturn)
      }
    }
  }
}

// `[no_mangle]` is both `unsafe` and required in order for the entry point to be recognized by
// the linker
#[allow(unsafe_code)]
#[no_mangle]
extern "C" fn finalize() -> ! {
  let mut peripherals = Peripherals::take().expect(msg::PANIC_NO_PERIPHERALS);

  // Set up clocks, serial port, DRAM controller, ethernet et. al
  core_clock::init(&mut peripherals);
  sdram::init(&mut peripherals);
  eth::init(&mut peripherals);

  // Jump to `main()`
  // Re: `unsafe`: Rust's native ABI is unstable.  Developer must ensure both this crate and `main` are compiled by the
  // same Rust compiler version to ensure ABI compatibility/avoid UB.  `Peripherals` is not FFI-safe (not declared
  // `repr(C)` or `repr(transparent))`, so `extern "C"` is not a solution here.
  unsafe { main(peripherals) }
}
