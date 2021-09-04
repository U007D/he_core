pub trait IProcessor {
    extern "C" fn boot() -> !;
    extern "C" fn park() -> !;
}
