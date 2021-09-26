use crate::{consts::WORD_SIZE, processor::Processor, traits::IProcessor};

#[allow(unsafe_code)]
pub fn init_bss(sbss: usize, ebss: usize) {
    // Is `.bss` start zero?  If yes, memory layout is misconfigured
    if sbss == 0 {
        // TODO: Indicate error condition (use `Result`)
        Processor::park()
    }

    // Is `.bss` end equal to start?  If yes, `.bss` is zero-sized, so skip prci
    if ebss != sbss {
        // Convert one-past-end iterator to inclusive-end iterator (no wrapping simplifies range arithmetic)
        let incl_ebss = ebss.wrapping_sub(1);
        // Is `.bss` size negative?  If yes, memory layout is misconfigured.
        if incl_ebss < sbss {
            // TODO: Indicate error condition (use `Result`)
            Processor::park()
        }

        // Zero out BSS
        (sbss..=incl_ebss).step_by(WORD_SIZE).for_each(|addr| unsafe {
            *(addr as *mut usize) = 0;
        });
    }
}
