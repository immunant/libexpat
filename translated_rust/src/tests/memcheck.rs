use std::sync::{Mutex, MutexGuard};

extern "C" {
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type AllocationEntry = allocation_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct allocation_entry {
    pub next: *mut allocation_entry,
    pub prev: *mut allocation_entry,
    pub allocation: *mut ::core::ffi::c_void,
    pub num_bytes: size_t,
}

#[derive(Copy, Clone)]
struct AllocationRecord {
    allocation: usize,
    num_bytes: size_t,
}

macro_rules! call_printf {
    ($message:expr) => {{
        unsafe {
            printf($message.as_ptr() as *const ::core::ffi::c_char);
        }
    }};
    ($message:expr, $($arg:expr),+ $(,)?) => {{
        unsafe {
            printf(
                $message.as_ptr() as *const ::core::ffi::c_char,
                $($arg),+
            );
        }
    }};
}

macro_rules! call_malloc {
    ($size:expr) => {{
        unsafe { malloc($size) }
    }};
}

macro_rules! call_realloc {
    ($ptr:expr, $size:expr) => {{
        unsafe { realloc($ptr, $size) }
    }};
}

macro_rules! call_free {
    ($ptr:expr) => {{
        unsafe {
            free($ptr);
        }
    }};
}

pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
static ALLOCATIONS: Mutex<Vec<AllocationRecord>> = Mutex::new(Vec::new());

fn allocations() -> MutexGuard<'static, Vec<AllocationRecord>> {
    ALLOCATIONS
        .lock()
        .expect("allocation tracker mutex should not be poisoned")
}

fn print_message(message: &'static [u8]) {
    call_printf!(message)
}

fn print_pointer_message(message: &'static [u8], ptr: *mut ::core::ffi::c_void) {
    call_printf!(message, ptr)
}

fn print_allocation_message(entry: AllocationRecord) {
    call_printf!(
        b"Allocated %lu bytes at %p\n\0",
        entry.num_bytes as ::core::ffi::c_ulong,
        entry.allocation as *mut ::core::ffi::c_void,
    )
}

fn allocate(size: size_t) -> *mut ::core::ffi::c_void {
    call_malloc!(size)
}

fn reallocate(ptr: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void {
    call_realloc!(ptr, size)
}

fn release(ptr: *mut ::core::ffi::c_void) {
    call_free!(ptr)
}

fn find_allocation_index(
    allocations: &[AllocationRecord],
    ptr: *const ::core::ffi::c_void,
) -> Option<usize> {
    allocations
        .iter()
        .position(|entry| entry.allocation == ptr as usize)
}

fn tracking_free_impl(ptr: *mut ::core::ffi::c_void) -> bool {
    if ptr.is_null() {
        return false;
    }

    let mut allocations = allocations();
    if let Some(index) = find_allocation_index(&allocations, ptr) {
        allocations.remove(index);
        false
    } else {
        true
    }
}

fn tracked_allocations() -> Vec<AllocationRecord> {
    allocations().iter().copied().collect()
}

fn tracking_malloc_impl(size: size_t) -> *mut ::core::ffi::c_void {
    let mut allocations = allocations();
    if allocations.try_reserve(1).is_err() {
        print_message(b"Allocator failure\n\0");
        return NULL;
    }

    let allocation = allocate(size);
    if allocation.is_null() {
        return NULL;
    }

    allocations.push(AllocationRecord {
        allocation: allocation as usize,
        num_bytes: size,
    });
    allocation
}

fn tracking_free_and_report_impl(ptr: *mut ::core::ffi::c_void) {
    if ptr.is_null() {
        return;
    }

    if tracking_free_impl(ptr) {
        print_pointer_message(b"Attempting to free unallocated memory at %p\n\0", ptr);
    }

    release(ptr);
}

fn tracking_realloc_impl(ptr: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void {
    if ptr.is_null() {
        return tracking_malloc_impl(size);
    }
    if size == 0 as size_t {
        tracking_free_and_report_impl(ptr);
        return NULL;
    }

    let mut allocations = allocations();
    if let Some(index) = find_allocation_index(&allocations, ptr) {
        let reallocated = reallocate(ptr, size);
        if reallocated.is_null() {
            return NULL;
        }

        allocations[index] = AllocationRecord {
            allocation: reallocated as usize,
            num_bytes: size,
        };
        reallocated
    } else {
        print_pointer_message(b"Attempting to realloc unallocated memory at %p\n\0", ptr);
        if allocations.try_reserve(1).is_err() {
            print_message(b"Reallocator failure\n\0");
            return NULL;
        }

        let reallocated = reallocate(ptr, size);
        if reallocated.is_null() {
            return NULL;
        }

        allocations.push(AllocationRecord {
            allocation: reallocated as usize,
            num_bytes: size,
        });
        reallocated
    }
}

fn tracking_report_impl() -> ::core::ffi::c_int {
    let allocations = tracked_allocations();
    if allocations.is_empty() {
        return 1 as ::core::ffi::c_int;
    }

    for entry in allocations {
        print_allocation_message(entry);
    }
    0 as ::core::ffi::c_int
}

#[no_mangle]
pub unsafe extern "C" fn tracking_malloc(size: size_t) -> *mut ::core::ffi::c_void {
    tracking_malloc_impl(size)
}

#[no_mangle]
pub unsafe extern "C" fn tracking_free(ptr: *mut ::core::ffi::c_void) {
    tracking_free_and_report_impl(ptr)
}

#[no_mangle]
pub unsafe extern "C" fn tracking_realloc(
    ptr: *mut ::core::ffi::c_void,
    size: size_t,
) -> *mut ::core::ffi::c_void {
    tracking_realloc_impl(ptr, size)
}

#[no_mangle]
pub unsafe extern "C" fn tracking_report() -> ::core::ffi::c_int {
    tracking_report_impl()
}
