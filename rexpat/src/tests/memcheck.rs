pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
use crate::stdlib::free;
use crate::stdlib::malloc;
use crate::stdlib::printf;
use crate::stdlib::realloc;

pub type AllocationEntry = allocation_entry;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct allocation_entry {
    pub next: *mut allocation_entry,
    pub prev: *mut allocation_entry,
    pub allocation: *mut ::core::ffi::c_void,
    pub num_bytes: crate::__stddef_size_t_h::size_t,
}

static mut alloc_head: *mut AllocationEntry =
    ::core::ptr::null::<AllocationEntry>() as *mut AllocationEntry;

static mut alloc_tail: *mut AllocationEntry =
    ::core::ptr::null::<AllocationEntry>() as *mut AllocationEntry;
#[no_mangle]

pub unsafe extern "C" fn tracking_malloc(
    mut size: crate::__stddef_size_t_h::size_t,
) -> *mut ::core::ffi::c_void {
    let entry: *mut AllocationEntry = crate::stdlib::malloc(
        ::core::mem::size_of::<AllocationEntry>() as crate::__stddef_size_t_h::size_t,
    ) as *mut AllocationEntry;
    if entry.is_null() {
        crate::stdlib::printf(b"Allocator failure\n\0".as_ptr() as *const ::core::ffi::c_char);
        return crate::__stddef_null_h::NULL;
    }
    (*entry).num_bytes = size;
    (*entry).allocation = crate::stdlib::malloc(size);
    if (*entry).allocation.is_null() {
        crate::stdlib::free(entry as *mut ::core::ffi::c_void);
        return crate::__stddef_null_h::NULL;
    }
    (*entry).next = ::core::ptr::null_mut::<allocation_entry>();
    if alloc_head.is_null() {
        (*entry).prev = ::core::ptr::null_mut::<allocation_entry>();
        alloc_tail = entry;
        alloc_head = alloc_tail;
    } else {
        (*entry).prev = alloc_tail as *mut allocation_entry;
        (*alloc_tail).next = entry as *mut allocation_entry;
        alloc_tail = entry;
    }
    return (*entry).allocation;
}

unsafe extern "C" fn find_allocation(mut ptr: *const ::core::ffi::c_void) -> *mut AllocationEntry {
    let mut entry: *mut AllocationEntry = ::core::ptr::null_mut::<AllocationEntry>();
    entry = alloc_head;
    while !entry.is_null() {
        if (*entry).allocation == ptr as *mut ::core::ffi::c_void {
            return entry;
        }
        entry = (*entry).next as *mut AllocationEntry;
    }
    return ::core::ptr::null_mut::<AllocationEntry>();
}
#[no_mangle]

pub unsafe extern "C" fn tracking_free(mut ptr: *mut ::core::ffi::c_void) {
    let mut entry: *mut AllocationEntry = ::core::ptr::null_mut::<AllocationEntry>();
    if ptr.is_null() {
        return;
    }
    entry = find_allocation(ptr);
    if !entry.is_null() {
        if !(*entry).prev.is_null() {
            (*(*entry).prev).next = (*entry).next;
        } else {
            alloc_head = (*entry).next as *mut AllocationEntry;
        }
        if !(*entry).next.is_null() {
            (*(*entry).next).prev = (*entry).prev;
        } else {
            alloc_tail = (*entry).next as *mut AllocationEntry;
        }
        crate::stdlib::free(entry as *mut ::core::ffi::c_void);
    } else {
        crate::stdlib::printf(
            b"Attempting to free unallocated memory at %p\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            ptr,
        );
    }
    crate::stdlib::free(ptr);
}
#[no_mangle]

pub unsafe extern "C" fn tracking_realloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: crate::__stddef_size_t_h::size_t,
) -> *mut ::core::ffi::c_void {
    let mut entry: *mut AllocationEntry = ::core::ptr::null_mut::<AllocationEntry>();
    if ptr.is_null() {
        return tracking_malloc(size);
    }
    if size == 0 as crate::__stddef_size_t_h::size_t {
        tracking_free(ptr);
        return crate::__stddef_null_h::NULL;
    }
    entry = find_allocation(ptr);
    if entry.is_null() {
        crate::stdlib::printf(
            b"Attempting to realloc unallocated memory at %p\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            ptr,
        );
        entry = crate::stdlib::malloc(
            ::core::mem::size_of::<AllocationEntry>() as crate::__stddef_size_t_h::size_t
        ) as *mut AllocationEntry;
        if entry.is_null() {
            crate::stdlib::printf(b"Reallocator failure\n\0".as_ptr() as *const ::core::ffi::c_char);
            return crate::__stddef_null_h::NULL;
        }
        (*entry).allocation = crate::stdlib::realloc(ptr, size);
        if (*entry).allocation.is_null() {
            crate::stdlib::free(entry as *mut ::core::ffi::c_void);
            return crate::__stddef_null_h::NULL;
        }
        (*entry).next = ::core::ptr::null_mut::<allocation_entry>();
        if alloc_head.is_null() {
            (*entry).prev = ::core::ptr::null_mut::<allocation_entry>();
            alloc_tail = entry;
            alloc_head = alloc_tail;
        } else {
            (*entry).prev = alloc_tail as *mut allocation_entry;
            (*alloc_tail).next = entry as *mut allocation_entry;
            alloc_tail = entry;
        }
    } else {
        let reallocated: *mut ::core::ffi::c_void =
            crate::stdlib::realloc(ptr, size) as *mut ::core::ffi::c_void;
        if reallocated.is_null() {
            return crate::__stddef_null_h::NULL;
        }
        (*entry).allocation = reallocated;
    }
    (*entry).num_bytes = size;
    return (*entry).allocation;
}
#[no_mangle]

pub unsafe extern "C" fn tracking_report() -> ::core::ffi::c_int {
    let mut entry: *mut AllocationEntry = ::core::ptr::null_mut::<AllocationEntry>();
    if alloc_head.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    entry = alloc_head;
    while !entry.is_null() {
        crate::stdlib::printf(
            b"Allocated %lu bytes at %p\n\0".as_ptr() as *const ::core::ffi::c_char,
            (*entry).num_bytes as ::core::ffi::c_ulong,
            (*entry).allocation,
        );
        entry = (*entry).next as *mut AllocationEntry;
    }
    return 0 as ::core::ffi::c_int;
}
