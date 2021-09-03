mod core_clock;
mod eth;
mod sdram;

use crate::{consts::*, traits::IProcessor};
use fu740_hal::pac::Peripherals;
use riscv::register::mhartid;

extern "C" {
  static _BSS_START: *const usize;
  static _BSS_END: *const usize;
}

extern "Rust" {
  fn main(peripherals: Peripherals) -> !;
}

pub struct Processor;

impl IProcessor for Processor {
  #[allow(named_asm_labels, unsafe_code)]
  // Define `.boot` section so linker can explicitly place the `boot` function.
  #[link_section = ".boot"]
  // `[no_mangle]` is both `unsafe` and required in order for the entry point to be recognized by the linker
  #[naked]
  #[no_mangle]
  // [Rust inline `asm!` documentation](https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels)
  extern "C" fn boot() -> ! {
    unsafe {
      #[rustfmt::skip]
      asm! { "
          // Load initial stack base and per-core stack size (in XLEN words)
          la a0, STACK_BASE
          ld t0, 0(a0)
          la t2, STACK_SIZE_WORDS

          // Compute stack size in bytes: `t3 = STACK_SIZE_WORDS.checked_mul(WORD_SIZE)`
          addi t3, zero, 0      // Init checked_mul accumulator
          addi t4, zero, 8      // `WORD_SIZE` == 8

          20:
          add t3, t3, t2
          bltu t3, t2, 50f      // Overflow occurred, jump to stack size overflow handler
          addi t4, t4, -1
          bgtu t4, zero, 20b

          // Compute current core's stack base offset: `t1 = STACK_SIZE.checked_mul(core_id)`_
          csrr t4, mhartid
          addi t1, t0, 0

          30:
          beq t4, zero, 40f
          sub t1, t1, t3
          bgtu t1, t0, 60f      // Overflow occurred; jump to stack offset overflow handler
          add t4, t4, -1
          j 30b

          // Set core-local stack pointer
          40:
          addi sp, t1, 0

          // Stack pointer is set; jump to `init_core()`
          j init_core

          // Stack size overflow handler
          50:
          addi t0, zero, 1         // Set error condition 1
          j 70f

          // Stack base offset overflow handler
          60:
          addi t0, zero, 2              // Set error condition 2
          j 70f

          // Overflow handler
          70:
          // TODO: Indicate error condition indicated in t0
          unimp                // Crash the core (stack setup failed, so can't safely jump to `park()`)
        ",
      ".align 8",
      "STACK_BASE: .dword _STACK_BASE",
      "STACK_SIZE_WORDS: .dword _STACK_SIZE_WORDS",
      options(noreturn),
      }
    }
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
      // Put the `hart` to sleep (wait for `ifi`)
      asm! { "
          wfi
          j park
        ",
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

#[no_mangle]
#[allow(unsafe_code)]
extern "C" fn init_core() -> ! {
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

fn park_non_zero_core_id() {
  // Park all `hart`s except `hart` 0
  if mhartid::read() != 0 {
    Processor::park()
  }
}
