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

#[no_mangle]
pub unsafe extern "C" fn tracking_malloc(size: size_t) -> *mut ::core::ffi::c_void {
    let mut allocations = allocations();
    if allocations.try_reserve(1).is_err() {
        unsafe {
            printf(b"Allocator failure\n\0".as_ptr() as *const ::core::ffi::c_char);
        }
        return NULL;
    }

    let allocation = unsafe { malloc(size) };
    if allocation.is_null() {
        return NULL;
    }

    allocations.push(AllocationRecord {
        allocation: allocation as usize,
        num_bytes: size,
    });
    allocation
}

#[no_mangle]
pub unsafe extern "C" fn tracking_free(ptr: *mut ::core::ffi::c_void) {
    if ptr.is_null() {
        return;
    }

    if tracking_free_impl(ptr) {
        unsafe {
            printf(
                b"Attempting to free unallocated memory at %p\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ptr,
            );
        }
    }

    unsafe {
        free(ptr);
    }
}

#[no_mangle]
pub unsafe extern "C" fn tracking_realloc(
    ptr: *mut ::core::ffi::c_void,
    size: size_t,
) -> *mut ::core::ffi::c_void {
    if ptr.is_null() {
        return unsafe { tracking_malloc(size) };
    }
    if size == 0 as size_t {
        unsafe {
            tracking_free(ptr);
        }
        return NULL;
    }

    let mut allocations = allocations();
    if let Some(index) = find_allocation_index(&allocations, ptr) {
        let reallocated = unsafe { realloc(ptr, size) };
        if reallocated.is_null() {
            return NULL;
        }

        allocations[index] = AllocationRecord {
            allocation: reallocated as usize,
            num_bytes: size,
        };
        reallocated
    } else {
        unsafe {
            printf(
                b"Attempting to realloc unallocated memory at %p\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ptr,
            );
        }
        if allocations.try_reserve(1).is_err() {
            unsafe {
                printf(b"Reallocator failure\n\0".as_ptr() as *const ::core::ffi::c_char);
            }
            return NULL;
        }

        let reallocated = unsafe { realloc(ptr, size) };
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

#[no_mangle]
pub unsafe extern "C" fn tracking_report() -> ::core::ffi::c_int {
    let allocations = tracked_allocations();
    if allocations.is_empty() {
        return 1 as ::core::ffi::c_int;
    }

    for entry in allocations {
        unsafe {
            printf(
                b"Allocated %lu bytes at %p\n\0".as_ptr() as *const ::core::ffi::c_char,
                entry.num_bytes as ::core::ffi::c_ulong,
                entry.allocation as *mut ::core::ffi::c_void,
            );
        }
    }
    0 as ::core::ffi::c_int
}
