use crate::types::{args::Args, never::Never};

pub trait IProcessor {
    extern "C" fn boot<F>(main: F) -> !
    where
        F: FnOnce(Args) -> Never;
    extern "C" fn halt() -> !;
}
