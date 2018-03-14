//! System calls.

/// Change the data segment. See `man brk`.
///
/// On success, the new program break is returned. On failure, the old program break is returned.
///
/// # Note
///
/// This is the `brk` **syscall**, not the library function.
#[cfg(not(any(target_os = "switch", target_os = "redox")))]
pub unsafe fn brk(ptr: *const u8) -> *const u8 {
    syscall!(BRK, ptr) as *const u8
}

/// Voluntarily give a time slice to the scheduler.
#[cfg(not(any(target_os = "switch", target_os = "redox")))]
pub fn sched_yield() -> usize {
    unsafe { syscall!(SCHED_YIELD) }
}

/// Change the data segment. See `man brk`.
///
/// On success, the new program break is returned. On failure, the old program break is returned.
///
/// # Note
///
/// This is the `brk` **syscall**, not the library function.
#[cfg(target_os = "redox")]
pub unsafe fn brk(ptr: *const u8) -> *const u8 {
    let old = ::syscall::brk(0).unwrap_or(0);
    ::syscall::brk(ptr as usize).unwrap_or(old) as *const u8
}

/// Voluntarily give a time slice to the scheduler.
#[cfg(target_os = "redox")]
pub fn sched_yield() -> usize {
    ::syscall::Error::mux(::syscall::sched_yield())
}

/// Change the data segment. See `man brk`.
///
/// On success, the new program break is returned. On failure, the old program break is returned.
///
/// # Note
///
/// This is the `brk` **syscall**, not the library function.
#[cfg(target_os = "switch")]
pub unsafe fn brk(ptr: *const u8) -> *const u8 {
    extern crate spin;
    use megaton_hammer::loader::{self, HeapStrategy};
    use core::sync::atomic::{AtomicUsize, Ordering};
    use core::ptr;

    static ALLOC_STRATEGY : spin::Once<HeapStrategy> = spin::Once::new();

    match ALLOC_STRATEGY.call_once(|| loader::acquire_heap_strategy().unwrap()) {
        &HeapStrategy::OverrideHeap(heap) => {
            static HEAP_POS: AtomicUsize = AtomicUsize::new(0);
            let new_size = (ptr as usize) - (&(*heap.as_ptr())[0] as *const u8 as usize);
            if new_size < heap.as_ref().len() {
                // if brk is in bounds
                HEAP_POS.store(new_size, Ordering::Relaxed);
                ptr
            } else {
                ((&(*heap.as_ptr())[0] as *const u8 as usize) + HEAP_POS.load(Ordering::Relaxed)) as *const u8
            }
        },
        &HeapStrategy::SetHeapSize => {
            let mut base = 0;
            let mut size = 0;
            // TODO: Cache this information
            if ::megaton_hammer::kernel::svc::get_info(&mut base, 4, ::megaton_hammer::kernel::svc::CURRENT_PROCESS, 0) != 0 {
                // TODO: Panic ?
                return ptr::null();
            }
            if ::megaton_hammer::kernel::svc::get_info(&mut size, 5, ::megaton_hammer::kernel::svc::CURRENT_PROCESS, 0) != 0 {
                // TODO: Panic ?
                return ptr::null();
            }

            let mut new_addr = ptr::null_mut();
            // TODO: Ensure ptr - base is page-aligned
            if ::megaton_hammer::kernel::svc::set_heap_size(&mut new_addr, ptr as u32 - base as u32) != 0 {
                return (base + size) as *mut u8;
            }
            return ptr;
        }
    }
}

/// Voluntarily give a time slice to the scheduler.
#[cfg(target_os = "switch")]
pub fn sched_yield() -> usize {
    unsafe { ::megaton_hammer::kernel::svc::sleep_thread(0) };
    0
}
