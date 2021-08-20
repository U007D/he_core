use crate::traits::IProcessor;

// extern "C" {
//   static _BSS_START: u64;
//   static _BSS_END: u64;
//   static _STACK_START: u64;
// }

pub struct Processor;

impl IProcessor for Processor {
  // `[no_mangle]` is both `unsafe` and required in order for the entry point to be recognized by
  // the linker
  #[allow(unsafe_code)]
  #[naked]
  #[no_mangle]
  // Enable use of linker-defined values in inline `asm!`--all other labels defined per
  // [Rust inline `asm!` documentation](https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels)
  #[allow(named_asm_labels)]
  extern "C" fn boot() -> ! {
    #[allow(unsafe_code)]
        unsafe {
      // There are a number of ways to import the linker symbols we want to use into inline assembly.
      // Inline `asm!` uses `RISC-V`'s "relative" addressing modes, which limits address to +/- ~524_287 bytes
      // away from the current program counter.  Setting the stack pointer to the end of 16GB RAM far exceeds
      // this limit.  Thus we use a less straightforward method of loading addresses--load them as ordinary
      // 64-bit values and operate on the in address-indirect mode in assembly.
      #[rustfmt::skip]
      asm! {
      // Disable MMU (should already be disabled, but now we can be certain)
      "csrw satp, zero",

      // Park all `hart`s except `hart` 0
      "csrr t0, mhartid",
      "bnez t0, park",

      // Initialize `.bss`
      // Ensure symbol offsets compile even when RISC-V code model does not support full 64-bit offsets
      "la t0, BSS_START",
      "ld t0, 0(t0)",
      "la t1, BSS_END",
      "ld t1, 0(t1)",
      "addi t1, t1, -1",
      // Is `.bss` size negative?
      "bgtu t0, t1, 4f",
      // Begin initialization
      "2:",
      "bgeu t0, t1, 3f",
      "sd zero, (t0)",
      "addi t0, t0, 8",
      "j 2b",
      "3:",

      // Initialize the stack
      // Ensure symbol offsets compile even when RISC-V code model does not support full 64-bit offsets
      "la t0, STACK_START",
      "ld sp, 0(t0)",

      // Jump to `main()`
      "j main",

      // Whoops bad configuration; halt
      // TODO: Indicate error condition using LEDs
      "4:",
      "unimp",

      // `.data`
      "BSS_START: .dword _BSS_START",
      "BSS_END: .dword _BSS_END",
      "STACK_START: .dword _STACK_START",
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
