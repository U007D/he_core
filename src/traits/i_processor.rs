pub trait IProcessor {
    fn disable_mmu();
    fn enable_mmu();
    fn init_proc();
    fn init_stack(stack_start_addr: usize);
    fn park() -> !;
}
