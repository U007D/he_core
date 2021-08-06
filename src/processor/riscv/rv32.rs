use crate::traits::IProcessor;

pub struct Processor;

impl IProcessor for Processor {
  #[naked]
  #[no_mangle]
  extern "C" fn boot() -> ! {
    #[allow(unsafe_code)]
        unsafe {
      #[rustfmt::skip]
      asm! {
      // Disable MMU (should already be disabled, but now we're certain)
      "csrw satp, zero",

      // Park all `hart`s except `hart` 0
      "csrr t0, mhartid",
      "bnez t0, park",

      // Initialize `.bss`
      // Ensure symbol offsets compile even when RISC-V code model does not support full 64-bit offsets
      "BSS_START: .word _BSS_START",
      "BSS_END: .word _BSS_END",
      "la t0, BSS_START",
      "ld t0, 0(t0)",
      "la t1, BSS_END",
      "ld t1, 0(t1)",
      "addi t1, t1, -1",
      // Is `.bss` size negative?
      "bgtu t0, t1, negative_sized_bss",
      // Begin initialization
      "init_bss:",
      "bgeu t0, t1, finished_init_bss",
      "sd zero, (t0)",
      "addi t0, t0, 4",
      "j init_bss",
      "finished_init_bss:",

      // Initialize the stack
      // Ensure symbol offsets compile even when RISC-V code model does not support full 64-bit offsets
      "STACK_START: .dword _STACK_START",
      "la t0, STACK_START",
      "ld sp, 0(t0)",

      // Jump to `main()`
      "j main",

      // Whoops
      "negative_sized_bss:",
      "unimp",
      options(noreturn)
      }
    }
  }

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
