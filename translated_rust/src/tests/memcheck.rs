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

pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
static ALLOCATIONS: Mutex<Vec<AllocationRecord>> = Mutex::new(Vec::new());

fn allocations() -> MutexGuard<'static, Vec<AllocationRecord>> {
    ALLOCATIONS
        .lock()
        .expect("allocation tracker mutex should not be poisoned")
}

fn c_malloc(size: size_t) -> *mut ::core::ffi::c_void {
    unsafe { malloc(size) }
}

fn c_realloc(ptr: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void {
    unsafe { realloc(ptr, size) }
}

fn c_free(ptr: *mut ::core::ffi::c_void) {
    unsafe { free(ptr) }
}

fn print_allocator_failure() {
    unsafe {
        printf(b"Allocator failure\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
}

fn print_reallocator_failure() {
    unsafe {
        printf(b"Reallocator failure\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
}

fn print_untracked_free(ptr: *mut ::core::ffi::c_void) {
    unsafe {
        printf(
            b"Attempting to free unallocated memory at %p\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            ptr,
        );
    }
}

fn print_untracked_realloc(ptr: *mut ::core::ffi::c_void) {
    unsafe {
        printf(
            b"Attempting to realloc unallocated memory at %p\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            ptr,
        );
    }
}

fn print_allocation(entry: AllocationRecord) {
    unsafe {
        printf(
            b"Allocated %lu bytes at %p\n\0".as_ptr() as *const ::core::ffi::c_char,
            entry.num_bytes as ::core::ffi::c_ulong,
            entry.allocation as *mut ::core::ffi::c_void,
        );
    }
}

fn find_allocation_index(
    allocations: &[AllocationRecord],
    ptr: *const ::core::ffi::c_void,
) -> Option<usize> {
    allocations
        .iter()
        .position(|entry| entry.allocation == ptr as usize)
}

fn tracking_malloc_impl(size: size_t) -> *mut ::core::ffi::c_void {
    let mut allocations = allocations();
    if allocations.try_reserve(1).is_err() {
        print_allocator_failure();
        return NULL;
    }

    let allocation = c_malloc(size);
    if allocation.is_null() {
        return NULL;
    }

    allocations.push(AllocationRecord {
        allocation: allocation as usize,
        num_bytes: size,
    });
    allocation
}

fn tracking_free_impl(ptr: *mut ::core::ffi::c_void) {
    if ptr.is_null() {
        return;
    }

    let mut allocations = allocations();
    if let Some(index) = find_allocation_index(&allocations, ptr) {
        allocations.remove(index);
    } else {
        print_untracked_free(ptr);
    }
    drop(allocations);

    c_free(ptr);
}

fn tracking_realloc_impl(ptr: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void {
    if ptr.is_null() {
        return tracking_malloc_impl(size);
    }
    if size == 0 as size_t {
        tracking_free_impl(ptr);
        return NULL;
    }

    let mut allocations = allocations();
    if let Some(index) = find_allocation_index(&allocations, ptr) {
        let reallocated = c_realloc(ptr, size);
        if reallocated.is_null() {
            return NULL;
        }

        allocations[index] = AllocationRecord {
            allocation: reallocated as usize,
            num_bytes: size,
        };
        reallocated
    } else {
        print_untracked_realloc(ptr);
        if allocations.try_reserve(1).is_err() {
            print_reallocator_failure();
            return NULL;
        }

        let reallocated = c_realloc(ptr, size);
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
    let allocations = allocations();
    if allocations.is_empty() {
        return 1 as ::core::ffi::c_int;
    }

    for &entry in allocations.iter() {
        print_allocation(entry);
    }
    0 as ::core::ffi::c_int
}

#[no_mangle]
pub unsafe extern "C" fn tracking_malloc(size: size_t) -> *mut ::core::ffi::c_void {
    tracking_malloc_impl(size)
}

#[no_mangle]
pub unsafe extern "C" fn tracking_free(ptr: *mut ::core::ffi::c_void) {
    tracking_free_impl(ptr)
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
