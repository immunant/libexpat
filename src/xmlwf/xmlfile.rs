#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:44"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types.h:44"]
pub mod types_h {
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = u64;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "194:1"]
    pub type __ssize_t = ::core::ffi::c_long;
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
#[c2rust::header_src = "/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/lib/expat_external.h:54"]
pub mod expat_external_h {
    #[c2rust::src_loc = "149:1"]
    pub type XML_Char = ::core::ffi::c_char;
    #[c2rust::src_loc = "150:1"]
    pub type XML_LChar = ::core::ffi::c_char;
    #[c2rust::src_loc = "158:1"]
    pub type XML_Size = ::core::ffi::c_ulong;
}
#[c2rust::header_src = "/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/lib/expat.h:54"]
pub mod expat_h {
    #[c2rust::src_loc = "56:1"]
    pub type XML_Parser = *mut XML_ParserStruct;
    #[c2rust::src_loc = "75:1"]
    pub type XML_Status = ::core::ffi::c_uint;
    #[c2rust::src_loc = "80:3"]
    pub const XML_STATUS_SUSPENDED: XML_Status = 2;
    #[c2rust::src_loc = "78:3"]
    pub const XML_STATUS_OK: XML_Status = 1;
    #[c2rust::src_loc = "76:3"]
    pub const XML_STATUS_ERROR: XML_Status = 0;
    #[c2rust::src_loc = "84:1"]
    pub type XML_Error = ::core::ffi::c_uint;
    #[c2rust::src_loc = "136:3"]
    pub const XML_ERROR_NOT_STARTED: XML_Error = 44;
    #[c2rust::src_loc = "134:3"]
    pub const XML_ERROR_AMPLIFICATION_LIMIT_BREACH: XML_Error = 43;
    #[c2rust::src_loc = "132:3"]
    pub const XML_ERROR_NO_BUFFER: XML_Error = 42;
    #[c2rust::src_loc = "130:3"]
    pub const XML_ERROR_INVALID_ARGUMENT: XML_Error = 41;
    #[c2rust::src_loc = "128:3"]
    pub const XML_ERROR_RESERVED_NAMESPACE_URI: XML_Error = 40;
    #[c2rust::src_loc = "127:3"]
    pub const XML_ERROR_RESERVED_PREFIX_XMLNS: XML_Error = 39;
    #[c2rust::src_loc = "126:3"]
    pub const XML_ERROR_RESERVED_PREFIX_XML: XML_Error = 38;
    #[c2rust::src_loc = "124:3"]
    pub const XML_ERROR_SUSPEND_PE: XML_Error = 37;
    #[c2rust::src_loc = "123:3"]
    pub const XML_ERROR_FINISHED: XML_Error = 36;
    #[c2rust::src_loc = "122:3"]
    pub const XML_ERROR_ABORTED: XML_Error = 35;
    #[c2rust::src_loc = "121:3"]
    pub const XML_ERROR_NOT_SUSPENDED: XML_Error = 34;
    #[c2rust::src_loc = "120:3"]
    pub const XML_ERROR_SUSPENDED: XML_Error = 33;
    #[c2rust::src_loc = "119:3"]
    pub const XML_ERROR_PUBLICID: XML_Error = 32;
    #[c2rust::src_loc = "118:3"]
    pub const XML_ERROR_TEXT_DECL: XML_Error = 31;
    #[c2rust::src_loc = "117:3"]
    pub const XML_ERROR_XML_DECL: XML_Error = 30;
    #[c2rust::src_loc = "116:3"]
    pub const XML_ERROR_INCOMPLETE_PE: XML_Error = 29;
    #[c2rust::src_loc = "115:3"]
    pub const XML_ERROR_UNDECLARING_PREFIX: XML_Error = 28;
    #[c2rust::src_loc = "113:3"]
    pub const XML_ERROR_UNBOUND_PREFIX: XML_Error = 27;
    #[c2rust::src_loc = "111:3"]
    pub const XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING: XML_Error = 26;
    #[c2rust::src_loc = "110:3"]
    pub const XML_ERROR_FEATURE_REQUIRES_XML_DTD: XML_Error = 25;
    #[c2rust::src_loc = "109:3"]
    pub const XML_ERROR_ENTITY_DECLARED_IN_PE: XML_Error = 24;
    #[c2rust::src_loc = "108:3"]
    pub const XML_ERROR_UNEXPECTED_STATE: XML_Error = 23;
    #[c2rust::src_loc = "107:3"]
    pub const XML_ERROR_NOT_STANDALONE: XML_Error = 22;
    #[c2rust::src_loc = "106:3"]
    pub const XML_ERROR_EXTERNAL_ENTITY_HANDLING: XML_Error = 21;
    #[c2rust::src_loc = "105:3"]
    pub const XML_ERROR_UNCLOSED_CDATA_SECTION: XML_Error = 20;
    #[c2rust::src_loc = "104:3"]
    pub const XML_ERROR_INCORRECT_ENCODING: XML_Error = 19;
    #[c2rust::src_loc = "103:3"]
    pub const XML_ERROR_UNKNOWN_ENCODING: XML_Error = 18;
    #[c2rust::src_loc = "102:3"]
    pub const XML_ERROR_MISPLACED_XML_PI: XML_Error = 17;
    #[c2rust::src_loc = "101:3"]
    pub const XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF: XML_Error = 16;
    #[c2rust::src_loc = "100:3"]
    pub const XML_ERROR_BINARY_ENTITY_REF: XML_Error = 15;
    #[c2rust::src_loc = "99:3"]
    pub const XML_ERROR_BAD_CHAR_REF: XML_Error = 14;
    #[c2rust::src_loc = "98:3"]
    pub const XML_ERROR_ASYNC_ENTITY: XML_Error = 13;
    #[c2rust::src_loc = "97:3"]
    pub const XML_ERROR_RECURSIVE_ENTITY_REF: XML_Error = 12;
    #[c2rust::src_loc = "96:3"]
    pub const XML_ERROR_UNDEFINED_ENTITY: XML_Error = 11;
    #[c2rust::src_loc = "95:3"]
    pub const XML_ERROR_PARAM_ENTITY_REF: XML_Error = 10;
    #[c2rust::src_loc = "94:3"]
    pub const XML_ERROR_JUNK_AFTER_DOC_ELEMENT: XML_Error = 9;
    #[c2rust::src_loc = "93:3"]
    pub const XML_ERROR_DUPLICATE_ATTRIBUTE: XML_Error = 8;
    #[c2rust::src_loc = "92:3"]
    pub const XML_ERROR_TAG_MISMATCH: XML_Error = 7;
    #[c2rust::src_loc = "91:3"]
    pub const XML_ERROR_PARTIAL_CHAR: XML_Error = 6;
    #[c2rust::src_loc = "90:3"]
    pub const XML_ERROR_UNCLOSED_TOKEN: XML_Error = 5;
    #[c2rust::src_loc = "89:3"]
    pub const XML_ERROR_INVALID_TOKEN: XML_Error = 4;
    #[c2rust::src_loc = "88:3"]
    pub const XML_ERROR_NO_ELEMENTS: XML_Error = 3;
    #[c2rust::src_loc = "87:3"]
    pub const XML_ERROR_SYNTAX: XML_Error = 2;
    #[c2rust::src_loc = "86:3"]
    pub const XML_ERROR_NO_MEMORY: XML_Error = 1;
    #[c2rust::src_loc = "85:3"]
    pub const XML_ERROR_NONE: XML_Error = 0;
    #[c2rust::src_loc = "455:1"]
    pub type XML_ExternalEntityRefHandler = Option<
        unsafe extern "C" fn(
            XML_Parser,
            *const XML_Char,
            *const XML_Char,
            *const XML_Char,
            *const XML_Char,
        ) -> ::core::ffi::c_int,
    >;
    use super::expat_external_h::XML_Char;
    use super::expat_external_h::XML_LChar;
    use super::expat_external_h::XML_Size;
    extern "C" {
        #[c2rust::src_loc = "55:1"]
        pub type XML_ParserStruct;
        #[c2rust::src_loc = "635:1"]
        pub fn XML_SetExternalEntityRefHandler(
            parser: XML_Parser,
            handler: XML_ExternalEntityRefHandler,
        );
        #[c2rust::src_loc = "732:1"]
        pub fn XML_SetBase(parser: XML_Parser, base: *const XML_Char) -> XML_Status;
        #[c2rust::src_loc = "788:1"]
        pub fn XML_Parse(
            parser: XML_Parser,
            s: *const ::core::ffi::c_char,
            len: ::core::ffi::c_int,
            isFinal: ::core::ffi::c_int,
        ) -> XML_Status;
        #[c2rust::src_loc = "791:1"]
        pub fn XML_GetBuffer(
            parser: XML_Parser,
            len: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "794:1"]
        pub fn XML_ParseBuffer(
            parser: XML_Parser,
            len: ::core::ffi::c_int,
            isFinal: ::core::ffi::c_int,
        ) -> XML_Status;
        #[c2rust::src_loc = "877:1"]
        pub fn XML_ExternalEntityParserCreate(
            parser: XML_Parser,
            context: *const XML_Char,
            encoding: *const XML_Char,
        ) -> XML_Parser;
        #[c2rust::src_loc = "927:1"]
        pub fn XML_GetErrorCode(parser: XML_Parser) -> XML_Error;
        #[c2rust::src_loc = "950:1"]
        pub fn XML_GetCurrentLineNumber(parser: XML_Parser) -> XML_Size;
        #[c2rust::src_loc = "951:1"]
        pub fn XML_GetCurrentColumnNumber(parser: XML_Parser) -> XML_Size;
        #[c2rust::src_loc = "996:1"]
        pub fn XML_ParserFree(parser: XML_Parser);
        #[c2rust::src_loc = "1000:1"]
        pub fn XML_ErrorString(code: XML_Error) -> *const XML_LChar;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:68"]
pub mod unistd_h {
    #[c2rust::src_loc = "220:1"]
    pub type ssize_t = __ssize_t;
    use super::__stddef_size_t_h::size_t;
    use super::types_h::__ssize_t;
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
#[c2rust::header_src = "/usr/include/stdio.h:44"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "150:1"]
        pub static mut stdout: *mut FILE;
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
#[c2rust::header_src = "/usr/include/stdlib.h:45"]
pub mod stdlib_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "672:1"]
        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "687:1"]
        pub fn free(__ptr: *mut ::core::ffi::c_void);
        #[c2rust::src_loc = "756:1"]
        pub fn exit(__status: ::core::ffi::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/string.h:47"]
pub mod string_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "141:1"]
        pub fn strcpy(
            __dest: *mut ::core::ffi::c_char,
            __src: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "273:1"]
        pub fn strrchr(
            __s: *const ::core::ffi::c_char,
            __c: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "407:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
#[c2rust::header_src = "/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/xmlwf/filemap.h:58"]
pub mod filemap_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "52:1"]
        pub fn filemap(
            name: *const ::core::ffi::c_char,
            processor: Option<
                unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    size_t,
                    *const ::core::ffi::c_char,
                    *mut ::core::ffi::c_void,
                ) -> (),
            >,
            arg: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/fcntl-linux.h:48"]
pub mod fcntl_linux_h {
    #[c2rust::src_loc = "43:9"]
    pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/fcntl.h:48"]
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
#[c2rust::header_src = "/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/xmlwf/xmlfile.h:56"]
pub mod xmlfile_h {
    #[c2rust::src_loc = "36:9"]
    pub const XML_MAP_FILE: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "37:9"]
    pub const XML_EXTERNAL_ENTITIES: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:68"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
}
pub use self::__stddef_size_t_h::size_t;

pub use self::expat_external_h::XML_Char;
pub use self::expat_external_h::XML_LChar;
pub use self::expat_external_h::XML_Size;
pub use self::expat_h::XML_Error;
pub use self::expat_h::XML_ErrorString;
pub use self::expat_h::XML_ExternalEntityParserCreate;
pub use self::expat_h::XML_ExternalEntityRefHandler;
pub use self::expat_h::XML_GetBuffer;
pub use self::expat_h::XML_GetCurrentColumnNumber;
pub use self::expat_h::XML_GetCurrentLineNumber;
pub use self::expat_h::XML_GetErrorCode;
pub use self::expat_h::XML_Parse;
pub use self::expat_h::XML_ParseBuffer;
pub use self::expat_h::XML_Parser;
pub use self::expat_h::XML_ParserFree;
pub use self::expat_h::XML_ParserStruct;
pub use self::expat_h::XML_SetBase;
pub use self::expat_h::XML_SetExternalEntityRefHandler;
pub use self::expat_h::XML_Status;
pub use self::expat_h::XML_ERROR_ABORTED;
pub use self::expat_h::XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
pub use self::expat_h::XML_ERROR_ASYNC_ENTITY;
pub use self::expat_h::XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF;
pub use self::expat_h::XML_ERROR_BAD_CHAR_REF;
pub use self::expat_h::XML_ERROR_BINARY_ENTITY_REF;
pub use self::expat_h::XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING;
pub use self::expat_h::XML_ERROR_DUPLICATE_ATTRIBUTE;
pub use self::expat_h::XML_ERROR_ENTITY_DECLARED_IN_PE;
pub use self::expat_h::XML_ERROR_EXTERNAL_ENTITY_HANDLING;
pub use self::expat_h::XML_ERROR_FEATURE_REQUIRES_XML_DTD;
pub use self::expat_h::XML_ERROR_FINISHED;
pub use self::expat_h::XML_ERROR_INCOMPLETE_PE;
pub use self::expat_h::XML_ERROR_INCORRECT_ENCODING;
pub use self::expat_h::XML_ERROR_INVALID_ARGUMENT;
pub use self::expat_h::XML_ERROR_INVALID_TOKEN;
pub use self::expat_h::XML_ERROR_JUNK_AFTER_DOC_ELEMENT;
pub use self::expat_h::XML_ERROR_MISPLACED_XML_PI;
pub use self::expat_h::XML_ERROR_NONE;
pub use self::expat_h::XML_ERROR_NOT_STANDALONE;
pub use self::expat_h::XML_ERROR_NOT_STARTED;
pub use self::expat_h::XML_ERROR_NOT_SUSPENDED;
pub use self::expat_h::XML_ERROR_NO_BUFFER;
pub use self::expat_h::XML_ERROR_NO_ELEMENTS;
pub use self::expat_h::XML_ERROR_NO_MEMORY;
pub use self::expat_h::XML_ERROR_PARAM_ENTITY_REF;
pub use self::expat_h::XML_ERROR_PARTIAL_CHAR;
pub use self::expat_h::XML_ERROR_PUBLICID;
pub use self::expat_h::XML_ERROR_RECURSIVE_ENTITY_REF;
pub use self::expat_h::XML_ERROR_RESERVED_NAMESPACE_URI;
pub use self::expat_h::XML_ERROR_RESERVED_PREFIX_XML;
pub use self::expat_h::XML_ERROR_RESERVED_PREFIX_XMLNS;
pub use self::expat_h::XML_ERROR_SUSPENDED;
pub use self::expat_h::XML_ERROR_SUSPEND_PE;
pub use self::expat_h::XML_ERROR_SYNTAX;
pub use self::expat_h::XML_ERROR_TAG_MISMATCH;
pub use self::expat_h::XML_ERROR_TEXT_DECL;
pub use self::expat_h::XML_ERROR_UNBOUND_PREFIX;
pub use self::expat_h::XML_ERROR_UNCLOSED_CDATA_SECTION;
pub use self::expat_h::XML_ERROR_UNCLOSED_TOKEN;
pub use self::expat_h::XML_ERROR_UNDECLARING_PREFIX;
pub use self::expat_h::XML_ERROR_UNDEFINED_ENTITY;
pub use self::expat_h::XML_ERROR_UNEXPECTED_STATE;
pub use self::expat_h::XML_ERROR_UNKNOWN_ENCODING;
pub use self::expat_h::XML_ERROR_XML_DECL;
pub use self::expat_h::XML_STATUS_ERROR;
pub use self::expat_h::XML_STATUS_OK;
pub use self::expat_h::XML_STATUS_SUSPENDED;
use self::fcntl_h::open;
pub use self::fcntl_linux_h::O_RDONLY;
use self::filemap_h::filemap;

use self::stdio_h::fprintf;
use self::stdio_h::perror;
use self::stdio_h::stderr;
use self::stdio_h::stdout;
use self::stdlib_h::exit;
use self::stdlib_h::free;
use self::stdlib_h::malloc;
use self::string_h::strcpy;
use self::string_h::strlen;
use self::string_h::strrchr;
pub use self::struct_FILE_h::_IO_codecvt;
pub use self::struct_FILE_h::_IO_lock_t;
pub use self::struct_FILE_h::_IO_marker;
pub use self::struct_FILE_h::_IO_wide_data;
pub use self::struct_FILE_h::_IO_FILE;
pub use self::types_h::__off64_t;
pub use self::types_h::__off_t;
pub use self::types_h::__ssize_t;
pub use self::types_h::__uint64_t;
pub use self::unistd_h::close;
pub use self::unistd_h::read;
pub use self::unistd_h::ssize_t;
pub use self::xmlfile_h::XML_EXTERNAL_ENTITIES;
pub use self::xmlfile_h::XML_MAP_FILE;
pub use self::FILE_h::FILE;
pub use self::__stddef_null_h::NULL;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "85:9"]
pub struct PROCESS_ARGS {
    pub parser: XML_Parser,
    pub retPtr: *mut ::core::ffi::c_int,
}
#[c2rust::src_loc = "79:13"]
pub const O_BINARY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
#[c2rust::src_loc = "83:1"]
pub static mut g_read_size_bytes: ::core::ffi::c_int =
    1024 as ::core::ffi::c_int * 8 as ::core::ffi::c_int;
#[c2rust::src_loc = "92:1"]
unsafe extern "C" fn reportError(mut parser: XML_Parser, mut filename: *const XML_Char) {
    let mut code: XML_Error = XML_GetErrorCode(parser);
    let mut message: *const XML_Char = XML_ErrorString(code) as *const XML_Char;
    if !message.is_null() {
        fprintf(
            stdout,
            b"%s:%lu:%lu: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
            XML_GetCurrentLineNumber(parser),
            XML_GetCurrentColumnNumber(parser),
            message,
        );
    } else {
        fprintf(
            stderr,
            b"%s: (unknown message %u)\n\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
            code as ::core::ffi::c_uint,
        );
    };
}
#[c2rust::src_loc = "108:1"]
unsafe extern "C" fn processFile(
    mut data: *const ::core::ffi::c_void,
    mut size: size_t,
    mut filename: *const XML_Char,
    mut args: *mut ::core::ffi::c_void,
) {
    let mut parser: XML_Parser = (*(args as *mut PROCESS_ARGS)).parser;
    let mut retPtr: *mut ::core::ffi::c_int = (*(args as *mut PROCESS_ARGS)).retPtr;
    if XML_Parse(
        parser,
        data as *const ::core::ffi::c_char,
        size as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        reportError(parser, filename);
        *retPtr = 0 as ::core::ffi::c_int;
    } else {
        *retPtr = 1 as ::core::ffi::c_int;
    };
}
#[c2rust::src_loc = "129:1"]
unsafe extern "C" fn resolveSystemId(
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut toFree: *mut *mut XML_Char,
) -> *const XML_Char {
    let mut s: *mut XML_Char = ::core::ptr::null_mut::<XML_Char>();
    *toFree = ::core::ptr::null_mut::<XML_Char>();
    if base.is_null() || *systemId as ::core::ffi::c_int == '/' as i32 {
        return systemId;
    }
    *toFree = malloc(
        strlen(base as *const ::core::ffi::c_char)
            .wrapping_add(strlen(systemId as *const ::core::ffi::c_char))
            .wrapping_add(2 as size_t)
            .wrapping_mul(::core::mem::size_of::<XML_Char>() as size_t),
    ) as *mut XML_Char;
    if (*toFree).is_null() {
        return systemId;
    }
    strcpy(*toFree, base as *const ::core::ffi::c_char);
    s = *toFree;
    if !strrchr(s, '/' as i32).is_null() {
        s = strrchr(s, '/' as i32).offset(1 as ::core::ffi::c_int as isize) as *mut XML_Char;
    }
    strcpy(
        s as *mut ::core::ffi::c_char,
        systemId as *const ::core::ffi::c_char,
    );
    return *toFree;
}
#[c2rust::src_loc = "157:1"]
unsafe extern "C" fn externalEntityRefFilemap(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    let mut s: *mut XML_Char = ::core::ptr::null_mut::<XML_Char>();
    let mut filename: *const XML_Char = ::core::ptr::null::<XML_Char>();
    let mut entParser: XML_Parser =
        XML_ExternalEntityParserCreate(parser, context, ::core::ptr::null::<XML_Char>());
    let mut filemapRes: ::core::ffi::c_int = 0;
    let mut args: PROCESS_ARGS = PROCESS_ARGS {
        parser: ::core::ptr::null_mut::<XML_ParserStruct>(),
        retPtr: ::core::ptr::null_mut::<::core::ffi::c_int>(),
    };
    args.retPtr = &raw mut result;
    args.parser = entParser;
    filename = resolveSystemId(base, systemId, &raw mut s);
    XML_SetBase(entParser, filename);
    filemapRes = filemap(
        filename as *const ::core::ffi::c_char,
        Some(
            processFile
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    size_t,
                    *const XML_Char,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        &raw mut args as *mut ::core::ffi::c_void,
    );
    match filemapRes {
        0 => {
            result = 0 as ::core::ffi::c_int;
        }
        2 => {
            fprintf(
                stderr,
                b"%s: file too large for memory-mapping, switching to streaming\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                filename,
            );
            result = processStream(filename, entParser);
        }
        _ => {}
    }
    free(s as *mut ::core::ffi::c_void);
    XML_ParserFree(entParser);
    return result;
}
#[c2rust::src_loc = "190:1"]
unsafe extern "C" fn processStream(
    mut filename: *const XML_Char,
    mut parser: XML_Parser,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !filename.is_null() {
        fd = open(filename as *const ::core::ffi::c_char, O_BINARY | O_RDONLY);
        if fd < 0 as ::core::ffi::c_int {
            perror(filename as *const ::core::ffi::c_char);
            return 0 as ::core::ffi::c_int;
        }
    }
    loop {
        let mut nread: ssize_t = 0;
        let mut buf: *mut ::core::ffi::c_char =
            XML_GetBuffer(parser, g_read_size_bytes) as *mut ::core::ffi::c_char;
        if buf.is_null() {
            if !filename.is_null() {
                close(fd);
            }
            fprintf(
                stderr,
                b"%s: out of memory\n\0" as *const u8 as *const ::core::ffi::c_char,
                if !filename.is_null() {
                    filename as *const ::core::ffi::c_char
                } else {
                    b"xmlwf\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
            return 0 as ::core::ffi::c_int;
        }
        nread = read(
            fd,
            buf as *mut ::core::ffi::c_void,
            g_read_size_bytes as size_t,
        );
        if nread < 0 as ssize_t {
            perror(if !filename.is_null() {
                filename as *const ::core::ffi::c_char
            } else {
                b"STDIN\0" as *const u8 as *const ::core::ffi::c_char
            });
            if !filename.is_null() {
                close(fd);
            }
            return 0 as ::core::ffi::c_int;
        }
        if XML_ParseBuffer(
            parser,
            nread as ::core::ffi::c_int,
            (nread == 0 as ssize_t) as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            reportError(
                parser,
                if !filename.is_null() {
                    filename
                } else {
                    b"STDIN\0" as *const u8 as *const XML_Char
                },
            );
            if !filename.is_null() {
                close(fd);
            }
            return 0 as ::core::ffi::c_int;
        }
        if !(nread == 0 as ssize_t) {
            continue;
        }
        if !filename.is_null() {
            close(fd);
        }
        break;
    }
    return 1 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "234:1"]
unsafe extern "C" fn externalEntityRefStream(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut s: *mut XML_Char = ::core::ptr::null_mut::<XML_Char>();
    let mut filename: *const XML_Char = ::core::ptr::null::<XML_Char>();
    let mut ret: ::core::ffi::c_int = 0;
    let mut entParser: XML_Parser =
        XML_ExternalEntityParserCreate(parser, context, ::core::ptr::null::<XML_Char>());
    filename = resolveSystemId(base, systemId, &raw mut s);
    XML_SetBase(entParser, filename);
    ret = processStream(filename, entParser);
    free(s as *mut ::core::ffi::c_void);
    XML_ParserFree(entParser);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "251:1"]
pub unsafe extern "C" fn XML_ProcessFile(
    mut parser: XML_Parser,
    mut filename: *const XML_Char,
    mut flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    if XML_SetBase(parser, filename) as u64 == 0 {
        fprintf(
            stderr,
            b"%s: out of memory\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
        );
        exit(1 as ::core::ffi::c_int);
    }
    if flags & XML_EXTERNAL_ENTITIES as ::core::ffi::c_uint != 0 {
        XML_SetExternalEntityRefHandler(
            parser,
            if flags & XML_MAP_FILE as ::core::ffi::c_uint != 0 {
                Some(
                    externalEntityRefFilemap
                        as unsafe extern "C" fn(
                            XML_Parser,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> ::core::ffi::c_int,
                )
            } else {
                Some(
                    externalEntityRefStream
                        as unsafe extern "C" fn(
                            XML_Parser,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> ::core::ffi::c_int,
                )
            },
        );
    }
    if flags & XML_MAP_FILE as ::core::ffi::c_uint != 0 {
        let mut filemapRes: ::core::ffi::c_int = 0;
        let mut args: PROCESS_ARGS = PROCESS_ARGS {
            parser: ::core::ptr::null_mut::<XML_ParserStruct>(),
            retPtr: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        };
        args.retPtr = &raw mut result;
        args.parser = parser;
        filemapRes = filemap(
            filename as *const ::core::ffi::c_char,
            Some(
                processFile
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        size_t,
                        *const XML_Char,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
            &raw mut args as *mut ::core::ffi::c_void,
        );
        match filemapRes {
            0 => {
                result = 0 as ::core::ffi::c_int;
            }
            2 => {
                fprintf(
                    stderr,
                    b"%s: file too large for memory-mapping, switching to streaming\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    filename,
                );
                result = processStream(filename, parser);
            }
            _ => {}
        }
    } else {
        result = processStream(filename, parser);
    }
    return result;
}
