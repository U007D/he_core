mod core_clock;
mod eth;
mod sdram;

use crate::{consts::*, traits::IProcessor};
use fu740_hal::pac::Peripherals;
use riscv::register::mhartid;

extern "C" {
  static _BSS_START: *const usize;
  static _BSS_END: *const usize;
  static _STACK_BASE: *const usize;
}

extern "Rust" {
  fn main(peripherals: Peripherals) -> !;
}

pub struct Processor;

impl IProcessor for Processor {
  #[allow(unsafe_code, unsupported_naked_functions)]
  // Define `.boot` section so linker can explicitly place the `boot` function.
  #[link_section = ".boot"]
  // `[no_mangle]` is both `unsafe` and required in order for the entry point to be recognized by
  // the linker
  // TODO: update function to init stack pointer in pure `asm!{}` as required by `#[naked]` to eliminate stack prelude.
  #[naked]
  #[no_mangle]
  // Enable use of linker-defined values in inline `asm!`--all other labels defined per
  // [Rust inline `asm!` documentation](https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels)
  extern "C" fn boot() -> ! {
    // Set the core-local stack pointer
    let core_stack_base = (unsafe { _STACK_BASE } as usize)
        .checked_sub(STACK_SIZE.checked_mul(mhartid::read()).expect(msg::PANIC_STACK_PTR_ADDRESS_OVERFLOW))
        .expect(msg::PANIC_STACK_PTR_ADDRESS_OVERFLOW) as *const usize;

    // Park all cores except core 0
    park_non_zero_core_id();

    // Initialize `.bss` section with zeros
    let (sbss, ebss) = unsafe { (*_BSS_START, *_BSS_END) };
    init_bss(sbss, ebss);

    // Init CPU
    let mut peripherals = Peripherals::take().expect(msg::PANIC_NO_PERIPHERALS);

    core_clock::init(&mut peripherals);
    sdram::init(&mut peripherals);
    eth::init(&mut peripherals);

    // Initialization complete-- jump to `main()`
    // The following call to `main` is `unsafe` because `main()` will be defined in another crate and Rust's native ABI
    // is unstable. The developer must ensure both this crate and `main` are compiled by the same Rust compiler
    // version to ensure ABI compatibility/avoid UB.
    unsafe { main(peripherals) }
  }

  // TODO: Send `Result` to `park()` to provide error indication to user
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

#[allow(unsafe_code)]
fn init_bss(sbss: usize, ebss: usize) {
  // Is `.bss` start zero?  If yes, memory layout is misconfigured
  if sbss == 0 {
    // TODO: Indicate error condition
    Processor::park()
  }

  // Is `.bss` end equal to start?  If yes, `.bss` is zero-sized, so skip init
  if ebss != sbss {
    // Convert one-past-end iterator to inclusive-end iterator (no wrapping simplifies range arithmetic)
    let incl_ebss = ebss.wrapping_sub(1);
    // Is `.bss` size negative?  If yes, memory layout is misconfigured.
    if incl_ebss < sbss {
      // TODO: Indicate error condition
      Processor::park()
    }

    // Zero out BSS
    (sbss..=incl_ebss).step_by(WORD_SIZE).for_each(|addr| unsafe {
      *(addr as *mut usize) = 0;
    });
  }
}

fn park_non_zero_core_id() {
  // Park all `hart`s except `hart` 0
  if mhartid::read() != 0 {
    Processor::park()
  }
}
