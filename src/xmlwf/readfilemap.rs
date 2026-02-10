extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn perror(__s: *const ::core::ffi::c_char);
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
}
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type __syscall_ulong_t = ::core::ffi::c_ulong;
pub type ssize_t = isize;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atime: __time_t,
    pub st_atimensec: __syscall_ulong_t,
    pub st_mtime: __time_t,
    pub st_mtimensec: __syscall_ulong_t,
    pub st_ctime: __time_t,
    pub st_ctimensec: __syscall_ulong_t,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_BINARY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const XML_MAX_CHUNK_LEN: ::core::ffi::c_int =
    INT_MAX / 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn filemap(
    mut name: *const ::core::ffi::c_char,
    mut processor: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            size_t,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    mut arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut nbytes: size_t = 0;
    let mut fd: ::core::ffi::c_int = 0;
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
    let mut p: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    fd = open(name, O_RDONLY | O_BINARY);
    if fd < 0 as ::core::ffi::c_int {
        perror(name);
        return 0 as ::core::ffi::c_int;
    }
    if fstat(fd, &raw mut sb) < 0 as ::core::ffi::c_int {
        perror(name);
        close(fd);
        return 0 as ::core::ffi::c_int;
    }
    if !(sb.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t) {
        fprintf(
            stderr,
            b"%s: not a regular file\n\0" as *const u8 as *const ::core::ffi::c_char,
            name,
        );
        close(fd);
        return 0 as ::core::ffi::c_int;
    }
    if sb.st_size > XML_MAX_CHUNK_LEN as __off_t {
        close(fd);
        return 2 as ::core::ffi::c_int;
    }
    nbytes = sb.st_size as size_t;
    if nbytes == 0 as size_t {
        static mut c: ::core::ffi::c_char = '\0' as i32 as ::core::ffi::c_char;
        processor.expect("non-null function pointer")(
            &raw const c as *const ::core::ffi::c_void,
            0 as size_t,
            name,
            arg,
        );
        close(fd);
        return 1 as ::core::ffi::c_int;
    }
    p = malloc(nbytes);
    if p.is_null() {
        fprintf(
            stderr,
            b"%s: out of memory\n\0" as *const u8 as *const ::core::ffi::c_char,
            name,
        );
        close(fd);
        return 0 as ::core::ffi::c_int;
    }
    n = read(fd, p, nbytes);
    if n < 0 as ssize_t {
        perror(name);
        free(p);
        close(fd);
        return 0 as ::core::ffi::c_int;
    }
    if n != nbytes as ssize_t {
        fprintf(
            stderr,
            b"%s: read unexpected number of bytes\n\0" as *const u8 as *const ::core::ffi::c_char,
            name,
        );
        free(p);
        close(fd);
        return 0 as ::core::ffi::c_int;
    }
    processor.expect("non-null function pointer")(p, nbytes, name, arg);
    free(p);
    close(fd);
    return 1 as ::core::ffi::c_int;
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
