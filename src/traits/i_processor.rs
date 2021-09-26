pub trait IProcessor {
    extern "C" fn start() -> !;
    extern "C" fn park() -> !;
}
