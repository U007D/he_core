#![allow(clippy::unwrap_used)]

use core::num::NonZeroUsize;

pub mod msg;

// Set per-core stack size in word-sized elements
// (typestate approach leverages MISU principle, ensuring correct-aligned sizing)
const STACK_SIZE_WORDS: usize = 262_144;

// CPU word size as power-of-two exponent
// (typestate approach leverages MISU principle, ensuring only valid word sizes can be specified)
// Unwrap in const context does not panic, but halts the build

#[cfg(target_arch = "riscv32")]
const WORD_SIZE_BIT: NonZeroUsize = NonZeroUsize::new(2).unwrap();

#[cfg(target_arch = "riscv64")]
const WORD_SIZE_BIT: NonZeroUsize = NonZeroUsize::new(3).unwrap();

#[cfg(target_arch = "riscv128")]
const WORD_SIZE_BIT: NonZeroUsize = NonZeroUsize::new(4).unwrap;

//-- Generated constants --
// Unwrap in const context does not panic, but halts the build
pub const STACK_SIZE: usize = STACK_SIZE_WORDS.checked_mul(WORD_SIZE).unwrap();
pub const WORD_SIZE: usize = 0x1 << WORD_SIZE_BIT.get();

// Ensure invariants are upheld
#[allow(clippy::assertions_on_constants)]
const fn _const_invariants() {
  // Invariant: ensure `WORD_SIZE` is non-zero
  debug_assert!(WORD_SIZE != 0);
}
