// =============== BEGIN xmlfile_h ================
pub const XML_MAP_FILE: ::core::ffi::c_int = 0o1;

pub const XML_EXTERNAL_ENTITIES: ::core::ffi::c_int = 0o2;

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
#[derive(Copy, Clone)]
#[repr(C)]

pub struct PROCESS_ARGS {
    pub parser: XML_Parser,
    pub retPtr: *mut ::core::ffi::c_int,
}

pub const O_BINARY: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut g_read_size_bytes: ::core::ffi::c_int = 1024 * 8;

unsafe extern "C" fn reportError(mut parser: XML_Parser, mut filename: *const XML_Char) {
    let mut code: XML_Error = XML_GetErrorCode(parser);
    let mut message: *const XML_Char = XML_ErrorString(code);
    if !message.is_null() {
        crate::stdlib::fprintf(
            crate::stdlib::stdout,
            b"%s:%lu:%lu: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
            XML_GetCurrentLineNumber(parser),
            XML_GetCurrentColumnNumber(parser),
            message,
        );
    } else {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s: (unknown message %u)\n\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
            code,
        );
    };
}

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
        1,
    ) == XML_STATUS_ERROR
    {
        reportError(parser, filename);
        *retPtr = 0i32;
    } else {
        *retPtr = 1i32;
    };
}

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
    *toFree = crate::stdlib::malloc(
        crate::stdlib::strlen(base)
            .wrapping_add(crate::stdlib::strlen(systemId))
            .wrapping_add(2usize)
            .wrapping_mul(::core::mem::size_of::<XML_Char>()),
    ) as *mut XML_Char;
    if (*toFree).is_null() {
        return systemId;
    }
    crate::stdlib::strcpy(*toFree, base);
    s = *toFree;
    if !crate::stdlib::strrchr(s, '/' as i32).is_null() {
        s = crate::stdlib::strrchr(s, '/' as i32).offset(1isize);
    }
    crate::stdlib::strcpy(s, systemId);
    return *toFree;
}

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
    filemapRes = crate::src::xmlwf::readfilemap::filemap(
        filename,
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
            result = 0i32;
        }
        2 => {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: file too large for memory-mapping, switching to streaming\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                filename,
            );
            result = processStream(filename, entParser);
        }
        _ => {}
    }
    crate::stdlib::free(s as *mut ::core::ffi::c_void);
    XML_ParserFree(entParser);
    return result;
}

unsafe extern "C" fn processStream(
    mut filename: *const XML_Char,
    mut parser: XML_Parser,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = 0;
    if !filename.is_null() {
        fd = crate::stdlib::open(filename, O_BINARY | O_RDONLY);
        if fd < 0 {
            crate::stdlib::perror(filename);
            return 0i32;
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
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: out of memory\n\0" as *const u8 as *const ::core::ffi::c_char,
                if !filename.is_null() {
                    filename
                } else {
                    b"xmlwf\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
            return 0i32;
        }
        nread = read(
            fd,
            buf as *mut ::core::ffi::c_void,
            g_read_size_bytes as size_t,
        );
        if nread < 0 {
            crate::stdlib::perror(if !filename.is_null() {
                filename
            } else {
                b"STDIN\0" as *const u8 as *const ::core::ffi::c_char
            });
            if !filename.is_null() {
                close(fd);
            }
            return 0i32;
        }
        if XML_ParseBuffer(
            parser,
            nread as ::core::ffi::c_int,
            (nread == 0) as ::core::ffi::c_int,
        ) == XML_STATUS_ERROR
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
            return 0i32;
        }
        if !(nread == 0) {
            continue;
        }
        if !filename.is_null() {
            close(fd);
        }
        break;
    }
    return 1;
}

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
    crate::stdlib::free(s as *mut ::core::ffi::c_void);
    XML_ParserFree(entParser);
    return ret;
}
pub unsafe fn XML_ProcessFile(
    mut parser: XML_Parser,
    mut filename: *const XML_Char,
    mut flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    if XML_SetBase(parser, filename) as u64 == 0 {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s: out of memory\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
        );
        crate::stdlib::exit(1i32);
    }
    if flags & crate::src::xmlwf::xmlfile::XML_EXTERNAL_ENTITIES as ::core::ffi::c_uint != 0 {
        XML_SetExternalEntityRefHandler(
            parser,
            if flags & crate::src::xmlwf::xmlfile::XML_MAP_FILE as ::core::ffi::c_uint != 0 {
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
    if flags & crate::src::xmlwf::xmlfile::XML_MAP_FILE as ::core::ffi::c_uint != 0 {
        let mut filemapRes: ::core::ffi::c_int = 0;
        let mut args: PROCESS_ARGS = PROCESS_ARGS {
            parser: ::core::ptr::null_mut::<XML_ParserStruct>(),
            retPtr: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        };
        args.retPtr = &raw mut result;
        args.parser = parser;
        filemapRes = crate::src::xmlwf::readfilemap::filemap(
            filename,
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
                result = 0i32;
            }
            2 => {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
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
