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

pub use crate::stdlib::ssize_t_1;
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

pub const O_BINARY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]

pub unsafe extern "C" fn filemap(
    mut name: *const ::core::ffi::c_char,
    mut processor: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    mut arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut nbytes: crate::__stddef_size_t_h::size_t = 0;
    let mut fd: ::core::ffi::c_int = 0;
    let mut n: crate::stdlib::ssize_t_1 = 0;
    let mut sb: crate::stdlib::stat = crate::stdlib::stat {
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
    let mut p: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    fd = crate::stdlib::open(name, crate::stdlib::O_RDONLY | O_BINARY);
    if fd < 0 as ::core::ffi::c_int {
        crate::stdlib::perror(name);
        return 0 as ::core::ffi::c_int;
    }
    if crate::stdlib::fstat(fd, &raw mut sb) < 0 as ::core::ffi::c_int {
        crate::stdlib::perror(name);
        crate::stdlib::close(fd);
        return 0 as ::core::ffi::c_int;
    }
    if !(sb.st_mode & crate::stdlib::__S_IFMT as crate::stdlib::__mode_t
        == 0o100000 as crate::stdlib::__mode_t)
    {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s: not a regular file\n\0" as *const u8 as *const ::core::ffi::c_char,
            name,
        );
        crate::stdlib::close(fd);
        return 0 as ::core::ffi::c_int;
    }
    if sb.st_size > crate::filemap_h::XML_MAX_CHUNK_LEN as crate::stdlib::__off_t {
        crate::stdlib::close(fd);
        return 2 as ::core::ffi::c_int;
    }
    nbytes = sb.st_size as crate::__stddef_size_t_h::size_t;
    if nbytes == 0 as crate::__stddef_size_t_h::size_t {
        static mut c: ::core::ffi::c_char = '\0' as i32 as ::core::ffi::c_char;
        processor.expect("non-null function pointer")(
            &raw const c as *const ::core::ffi::c_void,
            0 as crate::__stddef_size_t_h::size_t,
            name,
            arg,
        );
        crate::stdlib::close(fd);
        return 1 as ::core::ffi::c_int;
    }
    p = crate::stdlib::malloc(nbytes);
    if p.is_null() {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s: out of memory\n\0" as *const u8 as *const ::core::ffi::c_char,
            name,
        );
        crate::stdlib::close(fd);
        return 0 as ::core::ffi::c_int;
    }
    n = crate::stdlib::read_1(fd, p, nbytes);
    if n < 0 as crate::stdlib::ssize_t_1 {
        crate::stdlib::perror(name);
        crate::stdlib::free(p);
        crate::stdlib::close(fd);
        return 0 as ::core::ffi::c_int;
    }
    if n != nbytes as crate::stdlib::ssize_t_1 {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s: read unexpected number of bytes\n\0" as *const u8 as *const ::core::ffi::c_char,
            name,
        );
        crate::stdlib::free(p);
        crate::stdlib::close(fd);
        return 0 as ::core::ffi::c_int;
    }
    processor.expect("non-null function pointer")(p, nbytes, name, arg);
    crate::stdlib::free(p);
    crate::stdlib::close(fd);
    return 1 as ::core::ffi::c_int;
}
