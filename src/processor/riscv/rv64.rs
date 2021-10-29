mod prci;
mod init_bss;

use crate::{consts::*, traits::IProcessor};
use static_assertions::*;

const_assert!(ARCH_WORD_SIZE < u32::MAX as usize);
// Used in `start()`s inline assembly (not recognized by compiler)
#[allow(dead_code)]
// `const_assert!` ensures no truncation can occur
#[allow(clippy::cast_possible_truncation)]
const ARCH_WORD_SIZE_U32: u32 = ARCH_WORD_SIZE as u32;

pub struct Processor;

impl IProcessor for Processor {
    /// i) Init CPU w/`hart_id` 0; ii) Set up core-local `sp` for all cores; iii) Load & invoke 2BL, 3BL w/`hart_id` 0
    #[allow(named_asm_labels, unsafe_code)]
    // Define `.start` section so linker can explicitly place the `start` function.
    #[link_section = ".start"]
    #[naked]
    // `[no_mangle]` is both `unsafe` and required in order for the entry point to be recognized by the linker
    #[no_mangle]
    // [Rust inline `asm!` documentation](https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels)
    extern "C" fn start() -> ! {
        unsafe {
            asm! { "
                // Store `hart_id` as arg 0
                csrr s0, mhartid
                // If not `hart_id` 0: skip init, configure core-local `sp` & park
                bne s0, zero, 2f

                // `hart_id` 0 only: Set temporary stack pointer to end of `L2_LIM`
                la sp, SRAM_END
                ld sp, 0(sp)

                // Init `.bss`
                jal init_bss

                // Init CPU clock & DRAM controller
                jal init_core

                // Initialize per-core `sp`
                2:
                // Step 1: Store machine-word size
                lui t0, %hi(ARCH_WORD_SIZE_U32)
                addi t0, t0, %lo(ARCH_WORD_SIZE_U32)

                // Step 2: Load stack stack size (in `XLEN`-bit words)
                la t1, STACK_SIZE_WORDS
                ld t1, 0(t1)

                // Step 3: Compute stack size in bytes: `t2 = STACK_SIZE_WORDS.checked_mul(ARCH_WORD_SIZE_U32)`
                mulhu t2, t1, t0
                bne t2, zero, 89f      // 64-bit overflow occurred, jump to stack size overflow handler
                mul t2, t1, t0

                // Step 3: Compute current core's stack base offset: `t1 = STACK_SIZE.checked_mul(core_id)`
                mulhu t1, t2, s0
                bne t1, zero, 88f      // 64-bit overflow occurred; jump to stack base offset overflow handler
                mul t1, t2, s0

                // Step 4: Compute absolute address of core-local stack base relative to `DRAM_END`:
                //         `t0 = DRAM_END.checked_sub(stack_base_offset)`
                la t0, DRAM_END
                ld t0, 0(t0)
                sub sp, t0, t1

                // If DRAM_END is at the end of address space, one-past-the-end pointer will be 0--skip overflow check.
                beq t0, zero, 3f
                // Otherwise, ensure stack did not wrap (assert!(`STACK_BASE` >= `STACK_BASE` - `stack base offset`))
                bltu t0, sp, 87f       // Core-local stack offset wrapped, jump to stack offset overflow handler

                // `sp` is set; Park all non-zero `hart_id`s
                3:
                bne s0, zero, 4f

                // TODO: Copy 2BL (Rust SBI) from flash to DRAM 0x8000_0000
                // TODO: Copy 3BL from flash to DRAM 0x8000_2000
                // TODO: `jal` to Rust SBI
                // TODO: Tempoary--remove `j main`; `jal` or ensure Rust SBI `j/jal`s to 3BL
                j main

                // TODO: Indicate unexpected return from 3BL as error condition and `park`
                4:
                j park

                // Stack offset overflow handler
                87:
                addi t0, zero, 3        // Set error condition 3
                j 99f

                // Stack base offset overflow handler
                88:
                addi t0, zero, 1        // Set error condition 1
                j 99f

                // Stack size overflow handler
                89:
                addi t0, zero, 2        // Set error condition 2
                j 99f

                // Overflow handler
                99:
                // TODO: Indicate error condition indicated in t0
                unimp                   // Crash the core (stack setup failed, so can't safely jump to `park()`)
                ",
            ".align 8",                 // Very important as GAS `.dword` keywords do not align
                                        // (for more, see https://github.com/riscv/riscv-asm-manual/issues/12)
            "ARCH_WORD_SIZE_U32: .word ARCH_WORD_SIZE_U32",
            "DRAM_END: .dword _DRAM_END",
            "SRAM_END: .dword _SRAM_END",
            "STACK_SIZE_WORDS: .dword _STACK_SIZE_WORDS",
            options(noreturn),
            }
        }
    }

    // TODO: Send `Result` to `park()` to provide error indication to user?
    // `[no_mangle]` is both `unsafe` and required in order for the entry point to be recognized by
    // the linker
    #[allow(unsafe_code)]
    #[naked]
    #[no_mangle]
    extern "C" fn park() -> ! {
        #[allow(unsafe_code)]
        unsafe {
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
