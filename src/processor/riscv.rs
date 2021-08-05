use crate::traits::IProcessor;

pub struct Processor;

impl IProcessor for Processor {
    // Minimize stack usage since it may not yet be explicitly set up
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn disable_mmu() {
        #[allow(unsafe_code)]
        unsafe {
            asm! {
                "csrw satp, zero",
            }
        }
    }

    // Minimize stack usage since it may not yet be explicitly set up
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn enable_mmu() { unimplemented!() }

    // Minimize stack usage since it may not yet be explicitly set up
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn init_proc() {
        #[allow(unsafe_code)]
        unsafe {
            asm! {
                // Park all `hart`s except `hart` 0
                "csrr t0, mhartid",
                "bnez t0, park",
            }
        }
    }

    // Minimize stack usage since it is not yet explicitly set up
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn init_stack(stack_start_addr: usize) {
        #[allow(unsafe_code)]
        unsafe {
            asm! {
                "la sp, {}", in(reg) stack_start_addr,
            }
        }
    }

    // Minimize stack usage since it may not yet be explicitly set up
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn park() -> ! {
        #[allow(unsafe_code)]
        unsafe {
            asm! {
                // Put the `hart` to sleep (wait for `ifi`)
                "park:",
                "wfi",
                "j park",
            }
        }
        unreachable!()
    }
}
