#[cfg(feature = "ENABLE_SMP")]
#[link(name = "kernel_all.c")]
extern "C" {
    pub fn remoteTCBStall(tcb: *mut tcb_t);
    pub fn handleIPI(irq: usize, irq_path: bool);
    pub fn ipi_get_irq() -> usize;
    pub fn ipi_clear_irq(irq: usize);
    pub fn migrateTCB(tcb: *mut tcb_t, new_core: usize);
    pub fn clh_lock_init();
    pub fn clh_is_self_in_queue() -> bool;
    pub fn clh_lock_release(cpu: usize);
    pub fn clh_lock_acquire(cpu_idx: usize, irq_path: bool);

}
