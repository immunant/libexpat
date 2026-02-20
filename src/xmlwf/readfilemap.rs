#[c2rust::header_src = "/usr/include/bits/types.h:40"]
pub mod types_h {
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = u64;
    #[c2rust::src_loc = "145:1"]
    pub type __dev_t = ::core::ffi::c_ulong;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = ::core::ffi::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = ::core::ffi::c_uint;
    #[c2rust::src_loc = "148:1"]
    pub type __ino_t = ::core::ffi::c_ulong;
    #[c2rust::src_loc = "150:1"]
    pub type __mode_t = ::core::ffi::c_uint;
    #[c2rust::src_loc = "151:1"]
    pub type __nlink_t = ::core::ffi::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "175:1"]
    pub type __blksize_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "180:1"]
    pub type __blkcnt_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "197:1"]
    pub type __syscall_slong_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "199:1"]
    pub type __syscall_ulong_t = ::core::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/include/sys/types.h:40"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = isize;
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:40"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/struct_stat.h:41"]
pub mod struct_stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "26:1"]
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
    use super::types_h::__blkcnt_t;
    use super::types_h::__blksize_t;
    use super::types_h::__dev_t;
    use super::types_h::__gid_t;
    use super::types_h::__ino_t;
    use super::types_h::__mode_t;
    use super::types_h::__nlink_t;
    use super::types_h::__off_t;
    use super::types_h::__syscall_slong_t;
    use super::types_h::__syscall_ulong_t;
    use super::types_h::__time_t;
    use super::types_h::__uid_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:44"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "51:1"]
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
        #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [::core::ffi::c_char; 1],
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
        pub _prevchain: *mut *mut _IO_FILE,
        pub _mode: ::core::ffi::c_int,
        pub _unused3: ::core::ffi::c_int,
        pub _total_written: __uint64_t,
        pub _unused2: [::core::ffi::c_char; 8],
    }
    #[c2rust::src_loc = "45:1"]
    pub type _IO_lock_t = ();
    use super::types_h::__off64_t;
    use super::types_h::__off_t;
    use super::types_h::__uint64_t;
    extern "C" {
        #[c2rust::src_loc = "40:1"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "39:1"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "38:1"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:44"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/sys/stat.h:41"]
pub mod stat_h {
    use super::struct_stat_h::stat;
    extern "C" {
        #[c2rust::src_loc = "210:1"]
        pub fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:43"]
pub mod stdlib_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "672:1"]
        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "687:1"]
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:44"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "151:1"]
        pub static mut stderr: *mut FILE;
        #[c2rust::src_loc = "360:1"]
        pub fn fprintf(
            __stream: *mut FILE,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "868:1"]
        pub fn perror(__s: *const ::core::ffi::c_char);
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:48"]
pub mod unistd_h {
    use super::__stddef_size_t_h::size_t;
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[c2rust::src_loc = "358:1"]
        pub fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "371:1"]
        pub fn read(
            __fd: ::core::ffi::c_int,
            __buf: *mut ::core::ffi::c_void,
            __nbytes: size_t,
        ) -> ssize_t;
    }
}
#[c2rust::header_src = "/usr/lib/clang/21/include/limits.h:83"]
pub mod limits_h {
    #[c2rust::src_loc = "50:9"]
    pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
    use super::internal::__INT_MAX__;
}
#[c2rust::header_src = "/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/xmlwf/filemap.h:84"]
pub mod filemap_h {
    #[c2rust::src_loc = "45:9"]
    pub const XML_MAX_CHUNK_LEN: ::core::ffi::c_int =
        INT_MAX / 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    use super::limits_h::INT_MAX;
}
#[c2rust::header_src = "/usr/include/bits/fcntl-linux.h:41"]
pub mod fcntl_linux_h {
    #[c2rust::src_loc = "43:9"]
    pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/fcntl.h:42"]
pub mod fcntl_h {
    extern "C" {
        #[c2rust::src_loc = "209:1"]
        pub fn open(
            __file: *const ::core::ffi::c_char,
            __oflag: ::core::ffi::c_int,
            ...
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "60:9"]
    pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stat.h:0"]
pub mod bits_stat_h {
    #[c2rust::src_loc = "29:9"]
    pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
}
pub use self::__stddef_size_t_h::size_t;
pub use self::bits_stat_h::__S_IFMT;
use self::fcntl_h::open;
pub use self::fcntl_linux_h::O_RDONLY;
pub use self::filemap_h::XML_MAX_CHUNK_LEN;
pub use self::internal::__INT_MAX__;
pub use self::limits_h::INT_MAX;
use self::stat_h::fstat;

use self::stdio_h::fprintf;
use self::stdio_h::perror;
use self::stdio_h::stderr;
use self::stdlib_h::free;
use self::stdlib_h::malloc;
pub use self::struct_FILE_h::_IO_codecvt;
pub use self::struct_FILE_h::_IO_lock_t;
pub use self::struct_FILE_h::_IO_marker;
pub use self::struct_FILE_h::_IO_wide_data;
pub use self::struct_FILE_h::_IO_FILE;
pub use self::struct_stat_h::stat;
pub use self::sys_types_h::ssize_t;

pub use self::types_h::__blkcnt_t;
pub use self::types_h::__blksize_t;
pub use self::types_h::__dev_t;
pub use self::types_h::__gid_t;
pub use self::types_h::__ino_t;
pub use self::types_h::__mode_t;
pub use self::types_h::__nlink_t;
pub use self::types_h::__off64_t;
pub use self::types_h::__off_t;
pub use self::types_h::__syscall_slong_t;
pub use self::types_h::__syscall_ulong_t;
pub use self::types_h::__time_t;
pub use self::types_h::__uid_t;
pub use self::types_h::__uint64_t;
use self::unistd_h::close;
use self::unistd_h::read;
pub use self::FILE_h::FILE;
#[c2rust::src_loc = "79:13"]
pub const O_BINARY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
#[c2rust::src_loc = "86:1"]
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
