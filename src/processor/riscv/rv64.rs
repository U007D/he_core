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
      #[rustfmt::skip]
      asm! { "
          // Park all `hart`s except `hart` 0
          csrr t0, mhartid
          bnez t0, park

          // Initialize `.bss` section
          // Load and validate `.bss` start value
          la t0, BSS_START
          ld t0, 0(t0)
          // Is `.bss` start zero?  If yes, memory layout is misconfigured
          beq t0, zero, 4f

          // Load and validate `.bss` end value
          la t1, BSS_END
          ld t1, 0(t1)
          // Is `.bss` end equal to start?  If yes, `.bss is zero-sized`, skip init
          beq t0, t1, 3f
          // Convert one-past-end iterator to inclusive-end iterator (simplifies range arithmetic)
          addi t1, t1, -1
          // Is `.bss` size negative?  If yes, memory layout is misconfigured.
          bgtu t0, t1, 4f

          // Zero out BSS
          2:
          sd zero, (t0)
          addi t0, t0, 8
          ble t0, t1, 2b

          3:
          // Initialize the stack
          // Ensure symbol offsets compile even when RISC-V code model does not support full 64-bit offsets
          la t0, STACK_BASE
          ld sp, 0(t0)

          // Finish hardware initialization and run `main()`
          jal zero, finalize
          // TODO: Encode unexpected return from finalize condition

          // Whoops bad configuration; halt
          // TODO: Indicate error condition using LEDs
          4:
          unimp
        ",

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
