mod prci;

use crate::traits::IProcessor;

pub struct Processor;

impl IProcessor for Processor {
    #[allow(named_asm_labels, unsafe_code)]
    // Define `.boot` section so linker can explicitly place the `boot` function.
    #[link_section = ".start"]
    // `[no_mangle]` is both `unsafe` and required in order for the entry point to be recognized by the linker
    #[naked]
    #[no_mangle]
    // [Rust inline `asm!` documentation](https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels)
    extern "C" fn start() -> ! {
        unsafe {
            asm! { "
                la a1, _BSS_START
                ld t1, 0(a1)
                la a2, _BSS_END
                ld t2, 0(a2)

                // Load initial stack base and per-core stack size (in `XLEN`-bit words)
                la a0, STACK_BASE
                ld t0, 0(a0)
                la t2, STACK_SIZE_WORDS

                // Compute stack size in bytes: `t3 = STACK_SIZE_WORDS.checked_mul(WORD_SIZE)`
                addi t3, zero, 0        // Init checked_mul accumulator
                addi t4, zero, 8        // For rv64, `WORD_SIZE` == 8

                20:
                add t3, t3, t2
                bltu t3, t2, 50f        // Overflow occurred, jump to stack size overflow handler
                addi t4, t4, -1
                bgtu t4, zero, 20b

                // Compute current core's stack base offset: `t1 = STACK_SIZE.checked_mul(core_id)`
                csrr t4, mhartid
                addi t1, t0, 0

                30:
                beq t4, zero, 40f
                sub t1, t1, t3
                bgtu t1, t0, 60f        // Overflow occurred; jump to stack offset overflow handler
                add t4, t4, -1
                j 30b

                // Set core-local stack pointer
                40:
                addi sp, t1, 0

                // Stack pointer is set; jump to `init()` for core peripherals
                jal init

                // TODO: Copy 2BL (Rust SBI) to DRAM 0x8000_0000 and `jal`?
                // TODO: Copy 3BL to DRAM 0x8000_2000 and `jal`
                // TODO:    Tempoary--remove `j main` and jump to 3BL at 0x8000_2000
                j main

                // TODO: Indicate unexpected return from 3BL as error condition and `park`
                j park

                // Stack size overflow handler
                50:
                addi t0, zero, 1        // Set error condition 1
                j 70f

                // Stack base offset overflow handler
                60:
                addi t0, zero, 2        // Set error condition 2
                j 70f

                // Overflow handler
                70:
                // TODO: Indicate error condition indicated in t0
                unimp                   // Crash the core (stack setup failed, so can't safely jump to `park()`)
                ",
            ".align 8",                 // Very important as GAS `.dword` keywords do not align
                                        // (for more, see https://github.com/riscv/riscv-asm-manual/issues/12)
            "STACK_BASE: .dword _STACK_BASE",
            "STACK_SIZE_WORDS: .dword _STACK_SIZE_WORDS",
            "DRAM_ORIGIN: .dword _DRAM_ORIGIN",
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
