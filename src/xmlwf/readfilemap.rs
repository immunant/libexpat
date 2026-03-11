use ::core::ffi::{c_char, c_int, c_void};

pub use crate::__stddef_size_t_h::size_t;
pub use crate::filemap_h::XML_MAX_CHUNK_LEN;
pub use crate::internal::__INT_MAX__;
pub use crate::limits_h::INT_MAX;

pub use crate::stdlib::O_RDONLY;
pub use crate::stdlib::__S_IFMT;

pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;

pub use crate::stdlib::ssize_t;
pub use crate::stdlib::stat;

pub use crate::stdlib::_IO_FILE;

pub use crate::stdlib::__blkcnt_t;
pub use crate::stdlib::__blksize_t;
pub use crate::stdlib::__dev_t;
pub use crate::stdlib::__gid_t;
pub use crate::stdlib::__ino_t;
pub use crate::stdlib::__mode_t;
pub use crate::stdlib::__nlink_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__syscall_slong_t;
pub use crate::stdlib::__syscall_ulong_t;
pub use crate::stdlib::__time_t;
pub use crate::stdlib::__uid_t;
pub use crate::stdlib::__uint64_t;

pub use crate::stdlib::FILE;
use crate::stdlib::{close, fprintf, free, perror, stderr};

pub const O_BINARY: c_int = 0;

pub(crate) unsafe extern "C" fn filemap(
    mut name: *const c_char,
    mut processor: Option<
        unsafe extern "C" fn(*const c_void, size_t, *const c_char, *mut c_void) -> (),
    >,
    mut arg: *mut c_void,
) -> c_int {
    let mut nbytes: size_t = 0;
    let mut fd: c_int = 0;
    let mut n: ssize_t = 0;
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atime: 0,
        st_atimensec: 0,
        st_mtime: 0,
        st_mtimensec: 0,
        st_ctime: 0,
        st_ctimensec: 0,
        __glibc_reserved: [0; 3],
    };
    let mut p: *mut c_void = ::core::ptr::null_mut::<c_void>();
    fd = crate::stdlib::open(name, O_RDONLY | O_BINARY);
    if fd < 0 {
        perror(name);
        return 0i32;
    }
    if crate::stdlib::fstat(fd, &raw mut sb) < 0 {
        perror(name);
        close(fd);
        return 0i32;
    }
    if !(sb.st_mode & __S_IFMT as __mode_t == 0o100000) {
        fprintf(
            stderr,
            b"%s: not a regular file\n\0" as *const u8 as *const c_char,
            name,
        );
        close(fd);
        return 0i32;
    }
    if sb.st_size > XML_MAX_CHUNK_LEN as __off_t {
        close(fd);
        return 2i32;
    }
    nbytes = sb.st_size as size_t;
    if nbytes == 0 {
        static mut c: c_char = '\0' as c_char;
        processor.expect("non-null function pointer")(
            &raw const c as *const c_void,
            0usize,
            name,
            arg,
        );
        close(fd);
        return 1i32;
    }
    p = crate::stdlib::malloc(nbytes);
    if p.is_null() {
        fprintf(
            stderr,
            b"%s: out of memory\n\0" as *const u8 as *const c_char,
            name,
        );
        close(fd);
        return 0i32;
    }
    n = crate::stdlib::read(fd, p, nbytes);
    if n < 0 {
        perror(name);
        free(p);
        close(fd);
        return 0i32;
    }
    if n != nbytes as ssize_t {
        fprintf(
            stderr,
            b"%s: read unexpected number of bytes\n\0" as *const u8 as *const c_char,
            name,
        );
        free(p);
        close(fd);
        return 0i32;
    }
    processor.expect("non-null function pointer")(p, nbytes, name, arg);
    free(p);
    close(fd);
    return 1;
}
