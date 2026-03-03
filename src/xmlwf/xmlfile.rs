// =============== BEGIN xmlfile_h ================
pub const XML_MAP_FILE: c_int = 0o1;

pub const XML_EXTERNAL_ENTITIES: c_int = 0o2;

pub use crate::__stddef_size_t_h::size_t;

pub use crate::expat_external_h::XML_Char;
pub use crate::expat_external_h::XML_LChar;
pub use crate::expat_external_h::XML_Size;
pub use crate::expat_h::XML_Error;
pub use crate::expat_h::XML_ExternalEntityRefHandler;
pub use crate::expat_h::XML_Parser;
pub use crate::expat_h::XML_ParserStruct;
pub use crate::expat_h::XML_Status;
pub use crate::expat_h::XML_ERROR_ABORTED;
pub use crate::expat_h::XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
pub use crate::expat_h::XML_ERROR_ASYNC_ENTITY;
pub use crate::expat_h::XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF;
pub use crate::expat_h::XML_ERROR_BAD_CHAR_REF;
pub use crate::expat_h::XML_ERROR_BINARY_ENTITY_REF;
pub use crate::expat_h::XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING;
pub use crate::expat_h::XML_ERROR_DUPLICATE_ATTRIBUTE;
pub use crate::expat_h::XML_ERROR_ENTITY_DECLARED_IN_PE;
pub use crate::expat_h::XML_ERROR_EXTERNAL_ENTITY_HANDLING;
pub use crate::expat_h::XML_ERROR_FEATURE_REQUIRES_XML_DTD;
pub use crate::expat_h::XML_ERROR_FINISHED;
pub use crate::expat_h::XML_ERROR_INCOMPLETE_PE;
pub use crate::expat_h::XML_ERROR_INCORRECT_ENCODING;
pub use crate::expat_h::XML_ERROR_INVALID_ARGUMENT;
pub use crate::expat_h::XML_ERROR_INVALID_TOKEN;
pub use crate::expat_h::XML_ERROR_JUNK_AFTER_DOC_ELEMENT;
pub use crate::expat_h::XML_ERROR_MISPLACED_XML_PI;
pub use crate::expat_h::XML_ERROR_NONE;
pub use crate::expat_h::XML_ERROR_NOT_STANDALONE;
pub use crate::expat_h::XML_ERROR_NOT_STARTED;
pub use crate::expat_h::XML_ERROR_NOT_SUSPENDED;
pub use crate::expat_h::XML_ERROR_NO_BUFFER;
pub use crate::expat_h::XML_ERROR_NO_ELEMENTS;
pub use crate::expat_h::XML_ERROR_NO_MEMORY;
pub use crate::expat_h::XML_ERROR_PARAM_ENTITY_REF;
pub use crate::expat_h::XML_ERROR_PARTIAL_CHAR;
pub use crate::expat_h::XML_ERROR_PUBLICID;
pub use crate::expat_h::XML_ERROR_RECURSIVE_ENTITY_REF;
pub use crate::expat_h::XML_ERROR_RESERVED_NAMESPACE_URI;
pub use crate::expat_h::XML_ERROR_RESERVED_PREFIX_XML;
pub use crate::expat_h::XML_ERROR_RESERVED_PREFIX_XMLNS;
pub use crate::expat_h::XML_ERROR_SUSPENDED;
pub use crate::expat_h::XML_ERROR_SUSPEND_PE;
pub use crate::expat_h::XML_ERROR_SYNTAX;
pub use crate::expat_h::XML_ERROR_TAG_MISMATCH;
pub use crate::expat_h::XML_ERROR_TEXT_DECL;
pub use crate::expat_h::XML_ERROR_UNBOUND_PREFIX;
pub use crate::expat_h::XML_ERROR_UNCLOSED_CDATA_SECTION;
pub use crate::expat_h::XML_ERROR_UNCLOSED_TOKEN;
pub use crate::expat_h::XML_ERROR_UNDECLARING_PREFIX;
pub use crate::expat_h::XML_ERROR_UNDEFINED_ENTITY;
pub use crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
pub use crate::expat_h::XML_ERROR_UNKNOWN_ENCODING;
pub use crate::expat_h::XML_ERROR_XML_DECL;
pub use crate::expat_h::XML_STATUS_ERROR;
pub use crate::expat_h::XML_STATUS_OK;
pub use crate::expat_h::XML_STATUS_SUSPENDED;
pub use crate::src::lib::xmlparse::XML_ErrorString;
pub use crate::src::lib::xmlparse::XML_ExternalEntityParserCreate;
pub use crate::src::lib::xmlparse::XML_GetBuffer;
pub use crate::src::lib::xmlparse::XML_GetCurrentColumnNumber;
pub use crate::src::lib::xmlparse::XML_GetCurrentLineNumber;
pub use crate::src::lib::xmlparse::XML_GetErrorCode;
pub use crate::src::lib::xmlparse::XML_Parse;
pub use crate::src::lib::xmlparse::XML_ParseBuffer;
pub use crate::src::lib::xmlparse::XML_ParserFree;
pub use crate::src::lib::xmlparse::XML_SetBase;
pub use crate::src::lib::xmlparse::XML_SetExternalEntityRefHandler;

pub use crate::stdlib::O_RDONLY;

pub use crate::__stddef_null_h::NULL;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__ssize_t;
pub use crate::stdlib::__uint64_t;
pub use crate::stdlib::close;

pub use crate::stdlib::read;
pub use crate::stdlib::ssize_t;

pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

use crate::stdlib::fprintf;
use crate::stdlib::stderr;
use core::ffi::c_char;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;
use core::ptr::null;
use core::ptr::null_mut;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct PROCESS_ARGS {
    pub parser: XML_Parser,
    pub retPtr: *mut c_int,
}

pub const O_BINARY: c_int = 0;
#[no_mangle]
pub static mut g_read_size_bytes: c_int = 1024 * 8;

extern "C" fn reportError(mut parser: XML_Parser, mut filename: *const XML_Char) {
    let mut code: XML_Error = XML_GetErrorCode(parser);
    let mut message: *const XML_Char = XML_ErrorString(code);
    if !message.is_null() {
        unsafe {
            fprintf(
                crate::stdlib::stdout,
                b"%s:%lu:%lu: %s\n\0" as *const u8 as *const c_char,
                filename,
                XML_GetCurrentLineNumber(parser),
                XML_GetCurrentColumnNumber(parser),
                message,
            )
        };
    } else {
        unsafe {
            fprintf(
                stderr,
                b"%s: (unknown message %u)\n\0" as *const u8 as *const c_char,
                filename,
                code,
            )
        };
    };
}

extern "C" fn processFile(
    mut data: *const c_void,
    mut size: size_t,
    mut filename: *const XML_Char,
    mut args: *mut c_void,
) {
    let mut parser: XML_Parser = unsafe { (*(args as *mut PROCESS_ARGS)).parser };
    let mut retPtr: *mut c_int = unsafe { (*(args as *mut PROCESS_ARGS)).retPtr };
    if XML_Parse(parser, data as *const c_char, size as c_int, 1) == XML_STATUS_ERROR {
        reportError(parser, filename);
        unsafe { *retPtr = 0i32 };
    } else {
        unsafe { *retPtr = 1i32 };
    };
}

extern "C" fn resolveSystemId(
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut toFree: *mut *mut XML_Char,
) -> *const XML_Char {
    let mut s: *mut XML_Char = null_mut::<XML_Char>();
    unsafe { *toFree = null_mut::<XML_Char>() };
    if base.is_null() || unsafe { *systemId as c_int == '/' as i32 } {
        return systemId;
    }
    unsafe {
        *toFree = crate::stdlib::malloc(
            crate::stdlib::strlen(base)
                .wrapping_add(crate::stdlib::strlen(systemId))
                .wrapping_add(2usize)
                .wrapping_mul(::core::mem::size_of::<XML_Char>()),
        ) as *mut XML_Char
    };
    if unsafe { (*toFree).is_null() } {
        return systemId;
    }
    unsafe { crate::stdlib::strcpy(*toFree, base) };
    s = unsafe { *toFree };
    if !unsafe { crate::stdlib::strrchr(s, '/' as i32) }.is_null() {
        s = unsafe { crate::stdlib::strrchr(s, '/' as i32).offset(1isize) };
    }
    unsafe { crate::stdlib::strcpy(s, systemId) };
    unsafe { *toFree }
}

extern "C" fn externalEntityRefFilemap(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut result: c_int = 0;
    let mut s: *mut XML_Char = null_mut::<XML_Char>();
    let mut filename: *const XML_Char = null::<XML_Char>();
    let mut entParser: XML_Parser =
        XML_ExternalEntityParserCreate(parser, context, null::<XML_Char>());
    let mut filemapRes: c_int = 0;
    let mut args: PROCESS_ARGS = PROCESS_ARGS {
        parser: null_mut::<XML_ParserStruct>(),
        retPtr: null_mut::<c_int>(),
    };
    args.retPtr = &raw mut result;
    args.parser = entParser;
    filename = resolveSystemId(base, systemId, &raw mut s);
    XML_SetBase(entParser, filename);
    filemapRes = crate::src::xmlwf::readfilemap::filemap(
        filename,
        Some(
            processFile as extern "C" fn(*const c_void, size_t, *const XML_Char, *mut c_void) -> (),
        ),
        &raw mut args as *mut c_void,
    );
    match filemapRes {
        0 => {
            result = 0i32;
        }
        2 => {
            unsafe {
                fprintf(
                    stderr,
                    b"%s: file too large for memory-mapping, switching to streaming\n\0"
                        as *const u8 as *const c_char,
                    filename,
                )
            };
            result = processStream(filename, entParser);
        }
        _ => {}
    }
    unsafe { crate::stdlib::free(s as *mut c_void) };
    XML_ParserFree(entParser);
    result
}

extern "C" fn processStream(mut filename: *const XML_Char, mut parser: XML_Parser) -> c_int {
    let mut fd: c_int = 0;
    if !filename.is_null() {
        fd = unsafe { crate::stdlib::open(filename, O_BINARY | O_RDONLY) };
        if fd < 0 {
            unsafe { crate::stdlib::perror(filename) };
            return 0i32;
        }
    }
    loop {
        let mut nread: ssize_t = 0;
        let read_size = unsafe { g_read_size_bytes };
        let mut buf: *mut c_char = XML_GetBuffer(parser, read_size) as *mut c_char;
        if buf.is_null() {
            if !filename.is_null() {
                unsafe { close(fd) };
            }
            unsafe {
                fprintf(
                    stderr,
                    b"%s: out of memory\n\0" as *const u8 as *const c_char,
                    if !filename.is_null() {
                        filename
                    } else {
                        b"xmlwf\0" as *const u8 as *const c_char
                    },
                )
            };
            return 0i32;
        }
        nread = unsafe { read(fd, buf as *mut c_void, read_size as size_t) };
        if nread < 0 {
            unsafe {
                crate::stdlib::perror(if !filename.is_null() {
                    filename
                } else {
                    b"STDIN\0" as *const u8 as *const c_char
                })
            };
            if !filename.is_null() {
                unsafe { close(fd) };
            }
            return 0i32;
        }
        if XML_ParseBuffer(parser, nread as c_int, (nread == 0) as c_int) == XML_STATUS_ERROR {
            reportError(
                parser,
                if !filename.is_null() {
                    filename
                } else {
                    b"STDIN\0" as *const u8 as *const XML_Char
                },
            );
            if !filename.is_null() {
                unsafe { close(fd) };
            }
            return 0i32;
        }
        if nread != 0 {
            continue;
        }
        if !filename.is_null() {
            unsafe { close(fd) };
        }
        break;
    }
    1
}

extern "C" fn externalEntityRefStream(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut _publicId: *const XML_Char,
) -> c_int {
    let mut s: *mut XML_Char = null_mut::<XML_Char>();
    let mut filename: *const XML_Char = null::<XML_Char>();
    let mut ret: c_int = 0;
    let mut entParser: XML_Parser =
        XML_ExternalEntityParserCreate(parser, context, null::<XML_Char>());
    filename = resolveSystemId(base, systemId, &raw mut s);
    XML_SetBase(entParser, filename);
    ret = processStream(filename, entParser);
    unsafe { crate::stdlib::free(s as *mut c_void) };
    XML_ParserFree(entParser);
    ret
}
pub fn XML_ProcessFile(
    mut parser: XML_Parser,
    mut filename: *const XML_Char,
    mut flags: c_uint,
) -> c_int {
    let mut result: c_int = 0;
    if XML_SetBase(parser, filename) as u64 == 0 {
        unsafe {
            fprintf(
                stderr,
                b"%s: out of memory\0" as *const u8 as *const c_char,
                filename,
            )
        };
        unsafe { crate::stdlib::exit(1i32) };
    }
    if flags & crate::src::xmlwf::xmlfile::XML_EXTERNAL_ENTITIES as c_uint != 0 {
        XML_SetExternalEntityRefHandler(
            parser,
            if flags & crate::src::xmlwf::xmlfile::XML_MAP_FILE as c_uint != 0 {
                Some(
                    externalEntityRefFilemap
                        as extern "C" fn(
                            XML_Parser,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> c_int,
                )
            } else {
                Some(
                    externalEntityRefStream
                        as extern "C" fn(
                            XML_Parser,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> c_int,
                )
            },
        );
    }
    if flags & crate::src::xmlwf::xmlfile::XML_MAP_FILE as c_uint != 0 {
        let mut filemapRes: c_int = 0;
        let mut args: PROCESS_ARGS = PROCESS_ARGS {
            parser: null_mut::<XML_ParserStruct>(),
            retPtr: null_mut::<c_int>(),
        };
        args.retPtr = &raw mut result;
        args.parser = parser;
        filemapRes = crate::src::xmlwf::readfilemap::filemap(
            filename,
            Some(
                processFile
                    as extern "C" fn(*const c_void, size_t, *const XML_Char, *mut c_void) -> (),
            ),
            &raw mut args as *mut c_void,
        );
        match filemapRes {
            0 => {
                result = 0i32;
            }
            2 => {
                unsafe {
                    fprintf(
                        stderr,
                        b"%s: file too large for memory-mapping, switching to streaming\n\0"
                            as *const u8 as *const c_char,
                        filename,
                    )
                };
                result = processStream(filename, parser);
            }
            _ => {}
        }
    } else {
        result = processStream(filename, parser);
    }
    result
}
