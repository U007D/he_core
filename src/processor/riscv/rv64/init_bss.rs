use crate::consts::*;

extern "C" {
    static _BSS_START: *const usize;
    static _BSS_END: *const usize;
}

// This function is called by `Processor::start()`
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn init_bss() {
    // Extract machine-word-aligned `.bss` start and (exclusive) end addresses
    let (sbss, ebss) = (unsafe { *_BSS_START }, unsafe { *_BSS_END });

    // Ensure `sbss` and `ebss` are machine-word-aligned
    assert!(sbss & !ARCH_WORD_SIZE_MASK == 0);
    assert!(ebss & !ARCH_WORD_SIZE_MASK == 0);

    // Ensure `.bss` start and end addresses are sane or `ebss` is exactly one-past-the-end of addressable memory
    assert!(sbss != 0 && (sbss <= ebss || ebss == 0));

    // Compute inclusive `ebss` (fixes out-of-bounds issue at word limit with one-past-the-end iterators)
    let inclusive_ebss = ebss.wrapping_sub(ARCH_WORD_SIZE);

    // Zero-out `.bss` section
    (sbss..=inclusive_ebss).step_by(ARCH_WORD_SIZE).for_each(|addr| unsafe {
        *(addr as *mut usize) = 0;
    });
}
