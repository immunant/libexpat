extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type XML_ParserStruct;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn perror(__s: *const ::core::ffi::c_char);
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn XML_SetExternalEntityRefHandler(parser: XML_Parser, handler: XML_ExternalEntityRefHandler);
    fn XML_SetBase(parser: XML_Parser, base: *const XML_Char) -> XML_Status;
    fn XML_Parse(
        parser: XML_Parser,
        s: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        isFinal: ::core::ffi::c_int,
    ) -> XML_Status;
    fn XML_GetBuffer(parser: XML_Parser, len: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn XML_ParseBuffer(
        parser: XML_Parser,
        len: ::core::ffi::c_int,
        isFinal: ::core::ffi::c_int,
    ) -> XML_Status;
    fn XML_ExternalEntityParserCreate(
        parser: XML_Parser,
        context: *const XML_Char,
        encoding: *const XML_Char,
    ) -> XML_Parser;
    fn XML_GetErrorCode(parser: XML_Parser) -> XML_Error;
    fn XML_GetCurrentLineNumber(parser: XML_Parser) -> XML_Size;
    fn XML_GetCurrentColumnNumber(parser: XML_Parser) -> XML_Size;
    fn XML_ParserFree(parser: XML_Parser);
    fn XML_ErrorString(code: XML_Error) -> *const XML_LChar;
    fn filemap(
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
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
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
pub type XML_Char = ::core::ffi::c_char;
pub type XML_LChar = ::core::ffi::c_char;
pub type XML_Size = ::core::ffi::c_ulong;
pub type XML_Parser = *mut XML_ParserStruct;
pub type XML_Status = ::core::ffi::c_uint;
pub const XML_STATUS_SUSPENDED: XML_Status = 2;
pub const XML_STATUS_OK: XML_Status = 1;
pub const XML_STATUS_ERROR: XML_Status = 0;
pub type XML_Error = ::core::ffi::c_uint;
pub const XML_ERROR_NOT_STARTED: XML_Error = 44;
pub const XML_ERROR_AMPLIFICATION_LIMIT_BREACH: XML_Error = 43;
pub const XML_ERROR_NO_BUFFER: XML_Error = 42;
pub const XML_ERROR_INVALID_ARGUMENT: XML_Error = 41;
pub const XML_ERROR_RESERVED_NAMESPACE_URI: XML_Error = 40;
pub const XML_ERROR_RESERVED_PREFIX_XMLNS: XML_Error = 39;
pub const XML_ERROR_RESERVED_PREFIX_XML: XML_Error = 38;
pub const XML_ERROR_SUSPEND_PE: XML_Error = 37;
pub const XML_ERROR_FINISHED: XML_Error = 36;
pub const XML_ERROR_ABORTED: XML_Error = 35;
pub const XML_ERROR_NOT_SUSPENDED: XML_Error = 34;
pub const XML_ERROR_SUSPENDED: XML_Error = 33;
pub const XML_ERROR_PUBLICID: XML_Error = 32;
pub const XML_ERROR_TEXT_DECL: XML_Error = 31;
pub const XML_ERROR_XML_DECL: XML_Error = 30;
pub const XML_ERROR_INCOMPLETE_PE: XML_Error = 29;
pub const XML_ERROR_UNDECLARING_PREFIX: XML_Error = 28;
pub const XML_ERROR_UNBOUND_PREFIX: XML_Error = 27;
pub const XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING: XML_Error = 26;
pub const XML_ERROR_FEATURE_REQUIRES_XML_DTD: XML_Error = 25;
pub const XML_ERROR_ENTITY_DECLARED_IN_PE: XML_Error = 24;
pub const XML_ERROR_UNEXPECTED_STATE: XML_Error = 23;
pub const XML_ERROR_NOT_STANDALONE: XML_Error = 22;
pub const XML_ERROR_EXTERNAL_ENTITY_HANDLING: XML_Error = 21;
pub const XML_ERROR_UNCLOSED_CDATA_SECTION: XML_Error = 20;
pub const XML_ERROR_INCORRECT_ENCODING: XML_Error = 19;
pub const XML_ERROR_UNKNOWN_ENCODING: XML_Error = 18;
pub const XML_ERROR_MISPLACED_XML_PI: XML_Error = 17;
pub const XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF: XML_Error = 16;
pub const XML_ERROR_BINARY_ENTITY_REF: XML_Error = 15;
pub const XML_ERROR_BAD_CHAR_REF: XML_Error = 14;
pub const XML_ERROR_ASYNC_ENTITY: XML_Error = 13;
pub const XML_ERROR_RECURSIVE_ENTITY_REF: XML_Error = 12;
pub const XML_ERROR_UNDEFINED_ENTITY: XML_Error = 11;
pub const XML_ERROR_PARAM_ENTITY_REF: XML_Error = 10;
pub const XML_ERROR_JUNK_AFTER_DOC_ELEMENT: XML_Error = 9;
pub const XML_ERROR_DUPLICATE_ATTRIBUTE: XML_Error = 8;
pub const XML_ERROR_TAG_MISMATCH: XML_Error = 7;
pub const XML_ERROR_PARTIAL_CHAR: XML_Error = 6;
pub const XML_ERROR_UNCLOSED_TOKEN: XML_Error = 5;
pub const XML_ERROR_INVALID_TOKEN: XML_Error = 4;
pub const XML_ERROR_NO_ELEMENTS: XML_Error = 3;
pub const XML_ERROR_SYNTAX: XML_Error = 2;
pub const XML_ERROR_NO_MEMORY: XML_Error = 1;
pub const XML_ERROR_NONE: XML_Error = 0;
pub type XML_ExternalEntityRefHandler = Option<
    unsafe extern "C" fn(
        XML_Parser,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ) -> ::core::ffi::c_int,
>;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PROCESS_ARGS {
    pub parser: XML_Parser,
    pub retPtr: *mut ::core::ffi::c_int,
}
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XML_MAP_FILE: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const XML_EXTERNAL_ENTITIES: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const O_BINARY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut g_read_size_bytes: ::core::ffi::c_int =
    1024 as ::core::ffi::c_int * 8 as ::core::ffi::c_int;
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
