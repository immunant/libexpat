#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, raw_ref_op)]
#[allow(unused_imports)]
use ::libexpat;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type XML_ParserStruct;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn remove(__filename: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut ::core::ffi::c_char,
        __modes: ::core::ffi::c_int,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn putc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn perror(__s: *const ::core::ffi::c_char);
    fn strtof(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_float;
    fn strtoull(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn XML_ParserCreate(encoding: *const XML_Char) -> XML_Parser;
    fn XML_ParserCreateNS(encoding: *const XML_Char, namespaceSeparator: XML_Char) -> XML_Parser;
    fn XML_SetEntityDeclHandler(parser: XML_Parser, handler: XML_EntityDeclHandler);
    fn XML_SetElementHandler(
        parser: XML_Parser,
        start: XML_StartElementHandler,
        end: XML_EndElementHandler,
    );
    fn XML_SetCharacterDataHandler(parser: XML_Parser, handler: XML_CharacterDataHandler);
    fn XML_SetProcessingInstructionHandler(
        parser: XML_Parser,
        handler: XML_ProcessingInstructionHandler,
    );
    fn XML_SetCommentHandler(parser: XML_Parser, handler: XML_CommentHandler);
    fn XML_SetCdataSectionHandler(
        parser: XML_Parser,
        start: XML_StartCdataSectionHandler,
        end: XML_EndCdataSectionHandler,
    );
    fn XML_SetDefaultHandler(parser: XML_Parser, handler: XML_DefaultHandler);
    fn XML_SetDoctypeDeclHandler(
        parser: XML_Parser,
        start: XML_StartDoctypeDeclHandler,
        end: XML_EndDoctypeDeclHandler,
    );
    fn XML_SetNotationDeclHandler(parser: XML_Parser, handler: XML_NotationDeclHandler);
    fn XML_SetNamespaceDeclHandler(
        parser: XML_Parser,
        start: XML_StartNamespaceDeclHandler,
        end: XML_EndNamespaceDeclHandler,
    );
    fn XML_SetNotStandaloneHandler(parser: XML_Parser, handler: XML_NotStandaloneHandler);
    fn XML_SetUnknownEncodingHandler(
        parser: XML_Parser,
        handler: XML_UnknownEncodingHandler,
        encodingHandlerData: *mut ::core::ffi::c_void,
    );
    fn XML_DefaultCurrent(parser: XML_Parser);
    fn XML_SetUserData(parser: XML_Parser, userData: *mut ::core::ffi::c_void);
    fn XML_UseParserAsHandlerArg(parser: XML_Parser);
    fn XML_GetBase(parser: XML_Parser) -> *const XML_Char;
    fn XML_GetSpecifiedAttributeCount(parser: XML_Parser) -> ::core::ffi::c_int;
    fn XML_GetIdAttributeIndex(parser: XML_Parser) -> ::core::ffi::c_int;
    fn XML_SetParamEntityParsing(
        parser: XML_Parser,
        parsing: XML_ParamEntityParsing,
    ) -> ::core::ffi::c_int;
    fn XML_GetCurrentLineNumber(parser: XML_Parser) -> XML_Size;
    fn XML_GetCurrentColumnNumber(parser: XML_Parser) -> XML_Size;
    fn XML_GetCurrentByteIndex(parser: XML_Parser) -> XML_Index;
    fn XML_GetCurrentByteCount(parser: XML_Parser) -> ::core::ffi::c_int;
    fn XML_ParserFree(parser: XML_Parser);
    fn XML_ExpatVersion() -> *const XML_LChar;
    fn XML_GetFeatureList() -> *const XML_Feature;
    fn XML_SetBillionLaughsAttackProtectionMaximumAmplification(
        parser: XML_Parser,
        maximumAmplificationFactor: ::core::ffi::c_float,
    ) -> XML_Bool;
    fn XML_SetBillionLaughsAttackProtectionActivationThreshold(
        parser: XML_Parser,
        activationThresholdBytes: ::core::ffi::c_ulonglong,
    ) -> XML_Bool;
    fn XML_SetAllocTrackerMaximumAmplification(
        parser: XML_Parser,
        maximumAmplificationFactor: ::core::ffi::c_float,
    ) -> XML_Bool;
    fn XML_SetAllocTrackerActivationThreshold(
        parser: XML_Parser,
        activationThresholdBytes: ::core::ffi::c_ulonglong,
    ) -> XML_Bool;
    fn XML_SetReparseDeferralEnabled(parser: XML_Parser, enabled: XML_Bool) -> XML_Bool;
    fn codepageMap(cp: ::core::ffi::c_int, map: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn codepageConvert(cp: ::core::ffi::c_int, p: *const ::core::ffi::c_char)
        -> ::core::ffi::c_int;
    static mut g_read_size_bytes: ::core::ffi::c_int;
    fn XML_ProcessFile(
        parser: XML_Parser,
        filename: *const XML_Char,
        flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type XML_Char = ::core::ffi::c_char;
pub type XML_LChar = ::core::ffi::c_char;
pub type XML_Index = ::core::ffi::c_long;
pub type XML_Size = ::core::ffi::c_ulong;
pub type XML_Parser = *mut XML_ParserStruct;
pub type XML_Bool = ::core::ffi::c_uchar;
pub type XML_StartElementHandler = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, *mut *const XML_Char) -> (),
>;
pub type XML_EndElementHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> ()>;
pub type XML_CharacterDataHandler = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, ::core::ffi::c_int) -> (),
>;
pub type XML_ProcessingInstructionHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, *const XML_Char) -> ()>;
pub type XML_CommentHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> ()>;
pub type XML_StartCdataSectionHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type XML_EndCdataSectionHandler = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type XML_DefaultHandler = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, ::core::ffi::c_int) -> (),
>;
pub type XML_StartDoctypeDeclHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type XML_EndDoctypeDeclHandler = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type XML_EntityDeclHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        ::core::ffi::c_int,
        *const XML_Char,
        ::core::ffi::c_int,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ) -> (),
>;
pub type XML_NotationDeclHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ) -> (),
>;
pub type XML_StartNamespaceDeclHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, *const XML_Char) -> ()>;
pub type XML_EndNamespaceDeclHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> ()>;
pub type XML_NotStandaloneHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XML_Encoding {
    pub map: [::core::ffi::c_int; 256],
    pub data: *mut ::core::ffi::c_void,
    pub convert: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub release: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
pub type XML_UnknownEncodingHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        *mut XML_Encoding,
    ) -> ::core::ffi::c_int,
>;
pub type XML_ParamEntityParsing = ::core::ffi::c_uint;
pub const XML_PARAM_ENTITY_PARSING_ALWAYS: XML_ParamEntityParsing = 2;
pub const XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE: XML_ParamEntityParsing = 1;
pub const XML_PARAM_ENTITY_PARSING_NEVER: XML_ParamEntityParsing = 0;
pub type XML_FeatureEnum = ::core::ffi::c_uint;
pub const XML_FEATURE_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT: XML_FeatureEnum = 15;
pub const XML_FEATURE_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT: XML_FeatureEnum = 14;
pub const XML_FEATURE_GE: XML_FeatureEnum = 13;
pub const XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT:
    XML_FeatureEnum = 12;
pub const XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT:
    XML_FeatureEnum = 11;
pub const XML_FEATURE_ATTR_INFO: XML_FeatureEnum = 10;
pub const XML_FEATURE_LARGE_SIZE: XML_FeatureEnum = 9;
pub const XML_FEATURE_NS: XML_FeatureEnum = 8;
pub const XML_FEATURE_SIZEOF_XML_LCHAR: XML_FeatureEnum = 7;
pub const XML_FEATURE_SIZEOF_XML_CHAR: XML_FeatureEnum = 6;
pub const XML_FEATURE_MIN_SIZE: XML_FeatureEnum = 5;
pub const XML_FEATURE_CONTEXT_BYTES: XML_FeatureEnum = 4;
pub const XML_FEATURE_DTD: XML_FeatureEnum = 3;
pub const XML_FEATURE_UNICODE_WCHAR_T: XML_FeatureEnum = 2;
pub const XML_FEATURE_UNICODE: XML_FeatureEnum = 1;
pub const XML_FEATURE_END: XML_FeatureEnum = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XML_Feature {
    pub feature: XML_FeatureEnum,
    pub name: *const XML_LChar,
    pub value: ::core::ffi::c_long,
}
pub type ExitCode = ::core::ffi::c_uint;
pub const XMLWF_EXIT_USAGE_ERROR: ExitCode = 4;
pub const XMLWF_EXIT_OUTPUT_ERROR: ExitCode = 3;
pub const XMLWF_EXIT_NOT_WELLFORMED: ExitCode = 2;
pub const XMLWF_EXIT_INTERNAL_ERROR: ExitCode = 1;
pub const XMLWF_EXIT_SUCCESS: ExitCode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NotationList {
    pub next: *mut NotationList,
    pub notationName: *const XML_Char,
    pub systemId: *const XML_Char,
    pub publicId: *const XML_Char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlwfUserData {
    pub fp: *mut FILE,
    pub notationListHead: *mut NotationList,
    pub currentDoctypeName: *const XML_Char,
}
pub type XmlwfUserData = xmlwfUserData;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 46] = unsafe {
    ::core::mem::transmute::<[u8; 46], [::core::ffi::c_char; 46]>(
        *b"void attributeValue(FILE *, const XML_Char *)\0",
    )
};
pub const _IOFBF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XML_TRUE: XML_Bool = 1 as ::core::ffi::c_int as XML_Bool;
pub const XML_FALSE: XML_Bool = 0 as ::core::ffi::c_int as XML_Bool;
pub const XML_MAP_FILE: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const XML_EXTERNAL_ENTITIES: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
unsafe extern "C" fn characterData(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const XML_Char,
    mut len: ::core::ffi::c_int,
) {
    let mut fp: *mut FILE = (*(userData as *mut XmlwfUserData)).fp;
    while len > 0 as ::core::ffi::c_int {
        match *s as ::core::ffi::c_int {
            38 => {
                fputs(b"&amp;\0" as *const u8 as *const ::core::ffi::c_char, fp);
            }
            60 => {
                fputs(b"&lt;\0" as *const u8 as *const ::core::ffi::c_char, fp);
            }
            62 => {
                fputs(b"&gt;\0" as *const u8 as *const ::core::ffi::c_char, fp);
            }
            34 => {
                fputs(b"&quot;\0" as *const u8 as *const ::core::ffi::c_char, fp);
            }
            9 | 10 | 13 => {
                fprintf(
                    fp,
                    b"&#%d;\0" as *const u8 as *const ::core::ffi::c_char,
                    *s as ::core::ffi::c_int,
                );
            }
            _ => {
                putc(*s as ::core::ffi::c_int, fp);
            }
        }
        len -= 1;
        s = s.offset(1);
    }
}
unsafe extern "C" fn attributeValue(mut fp: *mut FILE, mut s: *const XML_Char) {
    putc('=' as i32, fp);
    putc('"' as i32, fp);
    if !s.is_null() {
    } else {
        __assert_fail(
            b"s\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/perl/Work/libexpat/expat/xmlwf/xmlwf.c\0" as *const u8
                as *const ::core::ffi::c_char,
            134 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    loop {
        match *s as ::core::ffi::c_int {
            0 | 1 => {
                putc('"' as i32, fp);
                return;
            }
            38 => {
                fputs(b"&amp;\0" as *const u8 as *const ::core::ffi::c_char, fp);
            }
            60 => {
                fputs(b"&lt;\0" as *const u8 as *const ::core::ffi::c_char, fp);
            }
            34 => {
                fputs(b"&quot;\0" as *const u8 as *const ::core::ffi::c_char, fp);
            }
            62 => {
                fputs(b"&gt;\0" as *const u8 as *const ::core::ffi::c_char, fp);
            }
            9 | 10 | 13 => {
                fprintf(
                    fp,
                    b"&#%d;\0" as *const u8 as *const ::core::ffi::c_char,
                    *s as ::core::ffi::c_int,
                );
            }
            _ => {
                putc(*s as ::core::ffi::c_int, fp);
            }
        }
        s = s.offset(1);
    }
}
unsafe extern "C" fn attcmp(
    mut att1: *const ::core::ffi::c_void,
    mut att2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp(
        *(att1 as *const *const XML_Char),
        *(att2 as *const *const XML_Char),
    );
}
unsafe extern "C" fn startElement(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    let mut nAtts: ::core::ffi::c_int = 0;
    let mut p: *mut *const XML_Char = ::core::ptr::null_mut::<*const XML_Char>();
    let mut fp: *mut FILE = (*(userData as *mut XmlwfUserData)).fp;
    putc('<' as i32, fp);
    fputs(name as *const ::core::ffi::c_char, fp);
    p = atts;
    while !(*p).is_null() {
        p = p.offset(1);
    }
    nAtts = (p.offset_from(atts) as ::core::ffi::c_long >> 1 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    if nAtts > 1 as ::core::ffi::c_int {
        qsort(
            atts as *mut ::core::ffi::c_void,
            nAtts as size_t,
            (::core::mem::size_of::<*mut XML_Char>() as size_t).wrapping_mul(2 as size_t),
            Some(
                attcmp
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
    }
    while !(*atts).is_null() {
        putc(' ' as i32, fp);
        let fresh0 = atts;
        atts = atts.offset(1);
        fputs(*fresh0, fp);
        attributeValue(fp, *atts);
        atts = atts.offset(1);
    }
    putc('>' as i32, fp);
}
unsafe extern "C" fn endElement(mut userData: *mut ::core::ffi::c_void, mut name: *const XML_Char) {
    let mut fp: *mut FILE = (*(userData as *mut XmlwfUserData)).fp;
    putc('<' as i32, fp);
    putc('/' as i32, fp);
    fputs(name as *const ::core::ffi::c_char, fp);
    putc('>' as i32, fp);
}
unsafe extern "C" fn nsattcmp(
    mut p1: *const ::core::ffi::c_void,
    mut p2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut att1: *const XML_Char = *(p1 as *const *const XML_Char);
    let mut att2: *const XML_Char = *(p2 as *const *const XML_Char);
    let mut sep1: ::core::ffi::c_int = (strrchr(att1 as *const ::core::ffi::c_char, '\u{1}' as i32)
        != ::core::ptr::null_mut::<::core::ffi::c_char>())
        as ::core::ffi::c_int;
    let mut sep2: ::core::ffi::c_int = (strrchr(att2 as *const ::core::ffi::c_char, '\u{1}' as i32)
        != ::core::ptr::null_mut::<::core::ffi::c_char>())
        as ::core::ffi::c_int;
    if sep1 != sep2 {
        return sep1 - sep2;
    }
    return strcmp(
        att1 as *const ::core::ffi::c_char,
        att2 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn startElementNS(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    let mut nAtts: ::core::ffi::c_int = 0;
    let mut nsi: ::core::ffi::c_int = 0;
    let mut p: *mut *const XML_Char = ::core::ptr::null_mut::<*const XML_Char>();
    let mut fp: *mut FILE = (*(userData as *mut XmlwfUserData)).fp;
    let mut sep: *const XML_Char = ::core::ptr::null::<XML_Char>();
    putc('<' as i32, fp);
    sep = strrchr(name as *const ::core::ffi::c_char, '\u{1}' as i32);
    if !sep.is_null() {
        fputs(b"n1:\0" as *const u8 as *const ::core::ffi::c_char, fp);
        fputs(sep.offset(1 as ::core::ffi::c_int as isize), fp);
        fputs(
            b" xmlns:n1\0" as *const u8 as *const ::core::ffi::c_char,
            fp,
        );
        attributeValue(fp, name);
        nsi = 2 as ::core::ffi::c_int;
    } else {
        fputs(name as *const ::core::ffi::c_char, fp);
        nsi = 1 as ::core::ffi::c_int;
    }
    p = atts;
    while !(*p).is_null() {
        p = p.offset(1);
    }
    nAtts = (p.offset_from(atts) as ::core::ffi::c_long >> 1 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    if nAtts > 1 as ::core::ffi::c_int {
        qsort(
            atts as *mut ::core::ffi::c_void,
            nAtts as size_t,
            (::core::mem::size_of::<*mut XML_Char>() as size_t).wrapping_mul(2 as size_t),
            Some(
                nsattcmp
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
    }
    while !(*atts).is_null() {
        let fresh1 = atts;
        atts = atts.offset(1);
        name = *fresh1;
        sep = strrchr(name as *const ::core::ffi::c_char, '\u{1}' as i32);
        putc(' ' as i32, fp);
        if !sep.is_null() {
            fprintf(
                fp,
                b"n%d:\0" as *const u8 as *const ::core::ffi::c_char,
                nsi,
            );
            fputs(sep.offset(1 as ::core::ffi::c_int as isize), fp);
        } else {
            fputs(name as *const ::core::ffi::c_char, fp);
        }
        attributeValue(fp, *atts);
        if !sep.is_null() {
            let fresh2 = nsi;
            nsi = nsi + 1;
            fprintf(
                fp,
                b" xmlns:n%d\0" as *const u8 as *const ::core::ffi::c_char,
                fresh2,
            );
            attributeValue(fp, name);
        }
        atts = atts.offset(1);
    }
    putc('>' as i32, fp);
}
unsafe extern "C" fn endElementNS(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
) {
    let mut fp: *mut FILE = (*(userData as *mut XmlwfUserData)).fp;
    let mut sep: *const XML_Char = ::core::ptr::null::<XML_Char>();
    putc('<' as i32, fp);
    putc('/' as i32, fp);
    sep = strrchr(name as *const ::core::ffi::c_char, '\u{1}' as i32);
    if !sep.is_null() {
        fputs(b"n1:\0" as *const u8 as *const ::core::ffi::c_char, fp);
        fputs(sep.offset(1 as ::core::ffi::c_int as isize), fp);
    } else {
        fputs(name as *const ::core::ffi::c_char, fp);
    }
    putc('>' as i32, fp);
}
unsafe extern "C" fn processingInstruction(
    mut userData: *mut ::core::ffi::c_void,
    mut target: *const XML_Char,
    mut data: *const XML_Char,
) {
    let mut fp: *mut FILE = (*(userData as *mut XmlwfUserData)).fp;
    putc('<' as i32, fp);
    putc('?' as i32, fp);
    fputs(target as *const ::core::ffi::c_char, fp);
    putc(' ' as i32, fp);
    fputs(data as *const ::core::ffi::c_char, fp);
    putc('?' as i32, fp);
    putc('>' as i32, fp);
}
unsafe extern "C" fn xcsdup(mut s: *const XML_Char) -> *mut XML_Char {
    let mut result: *mut XML_Char = ::core::ptr::null_mut::<XML_Char>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut numBytes: size_t = 0;
    loop {
        let fresh3 = count;
        count = count + 1;
        if !(*s.offset(fresh3 as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
            break;
        }
    }
    numBytes = (count as usize).wrapping_mul(::core::mem::size_of::<XML_Char>() as usize) as size_t;
    result = malloc(numBytes) as *mut XML_Char;
    if result.is_null() {
        return ::core::ptr::null_mut::<XML_Char>();
    }
    memcpy(
        result as *mut ::core::ffi::c_void,
        s as *const ::core::ffi::c_void,
        numBytes,
    );
    return result;
}
unsafe extern "C" fn startDoctypeDecl(
    mut userData: *mut ::core::ffi::c_void,
    mut doctypeName: *const XML_Char,
    mut _sysid: *const XML_Char,
    mut _publid: *const XML_Char,
    mut _has_internal_subset: ::core::ffi::c_int,
) {
    let mut data: *mut XmlwfUserData = userData as *mut XmlwfUserData;
    (*data).currentDoctypeName = xcsdup(doctypeName);
}
unsafe extern "C" fn freeNotations(mut data: *mut XmlwfUserData) {
    let mut notationListHead: *mut NotationList = (*data).notationListHead;
    while !notationListHead.is_null() {
        let mut next: *mut NotationList = (*notationListHead).next as *mut NotationList;
        free((*notationListHead).notationName as *mut ::core::ffi::c_void);
        free((*notationListHead).systemId as *mut ::core::ffi::c_void);
        free((*notationListHead).publicId as *mut ::core::ffi::c_void);
        free(notationListHead as *mut ::core::ffi::c_void);
        notationListHead = next;
    }
    (*data).notationListHead = ::core::ptr::null_mut::<NotationList>();
}
unsafe extern "C" fn cleanupUserData(mut userData: *mut XmlwfUserData) {
    free((*userData).currentDoctypeName as *mut ::core::ffi::c_void);
    (*userData).currentDoctypeName = ::core::ptr::null::<XML_Char>();
    freeNotations(userData);
}
unsafe extern "C" fn xcscmp(
    mut xs: *const XML_Char,
    mut xt: *const XML_Char,
) -> ::core::ffi::c_int {
    while *xs as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        && *xt as ::core::ffi::c_int != 0 as ::core::ffi::c_int
    {
        if (*xs as ::core::ffi::c_int) < *xt as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        if *xs as ::core::ffi::c_int > *xt as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        xs = xs.offset(1);
        xt = xt.offset(1);
    }
    if (*xs as ::core::ffi::c_int) < *xt as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if *xs as ::core::ffi::c_int > *xt as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn notationCmp(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let n1: *const NotationList = *(a as *const *const NotationList);
    let n2: *const NotationList = *(b as *const *const NotationList);
    return xcscmp((*n1).notationName, (*n2).notationName);
}
unsafe extern "C" fn endDoctypeDecl(mut userData: *mut ::core::ffi::c_void) {
    let mut data: *mut XmlwfUserData = userData as *mut XmlwfUserData;
    let mut notations: *mut *mut NotationList = ::core::ptr::null_mut::<*mut NotationList>();
    let mut notationCount: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: *mut NotationList = ::core::ptr::null_mut::<NotationList>();
    let mut i: ::core::ffi::c_int = 0;
    p = (*data).notationListHead;
    while !p.is_null() {
        notationCount += 1;
        p = (*p).next as *mut NotationList;
    }
    if !(notationCount == 0 as ::core::ffi::c_int) {
        notations = malloc(
            (notationCount as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut NotationList>() as size_t),
        ) as *mut *mut NotationList;
        if notations.is_null() {
            fprintf(
                stderr,
                b"Unable to sort notations\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            p = (*data).notationListHead;
            i = 0 as ::core::ffi::c_int;
            while i < notationCount {
                let ref mut fresh4 = *notations.offset(i as isize);
                *fresh4 = p;
                p = (*p).next as *mut NotationList;
                i += 1;
            }
            qsort(
                notations as *mut ::core::ffi::c_void,
                notationCount as size_t,
                ::core::mem::size_of::<*mut NotationList>() as size_t,
                Some(
                    notationCmp
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_void,
                            *const ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
            );
            fputs(
                b"<!DOCTYPE \0" as *const u8 as *const ::core::ffi::c_char,
                (*data).fp,
            );
            fputs(
                (*data).currentDoctypeName as *const ::core::ffi::c_char,
                (*data).fp,
            );
            fputs(
                b" [\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*data).fp,
            );
            i = 0 as ::core::ffi::c_int;
            while i < notationCount {
                fputs(
                    b"<!NOTATION \0" as *const u8 as *const ::core::ffi::c_char,
                    (*data).fp,
                );
                fputs(
                    (**notations.offset(i as isize)).notationName as *const ::core::ffi::c_char,
                    (*data).fp,
                );
                if !(**notations.offset(i as isize)).publicId.is_null() {
                    fputs(
                        b" PUBLIC '\0" as *const u8 as *const ::core::ffi::c_char,
                        (*data).fp,
                    );
                    fputs(
                        (**notations.offset(i as isize)).publicId as *const ::core::ffi::c_char,
                        (*data).fp,
                    );
                    putc('\'' as i32, (*data).fp);
                    if !(**notations.offset(i as isize)).systemId.is_null() {
                        putc(' ' as i32, (*data).fp);
                        putc('\'' as i32, (*data).fp);
                        fputs(
                            (**notations.offset(i as isize)).systemId as *const ::core::ffi::c_char,
                            (*data).fp,
                        );
                        putc('\'' as i32, (*data).fp);
                    }
                } else if !(**notations.offset(i as isize)).systemId.is_null() {
                    fputs(
                        b" SYSTEM '\0" as *const u8 as *const ::core::ffi::c_char,
                        (*data).fp,
                    );
                    fputs(
                        (**notations.offset(i as isize)).systemId as *const ::core::ffi::c_char,
                        (*data).fp,
                    );
                    putc('\'' as i32, (*data).fp);
                }
                putc('>' as i32, (*data).fp);
                putc('\n' as i32, (*data).fp);
                i += 1;
            }
            fputs(
                b"]>\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*data).fp,
            );
            free(notations as *mut ::core::ffi::c_void);
        }
    }
    freeNotations(data);
    free((*data).currentDoctypeName as *mut ::core::ffi::c_void);
    (*data).currentDoctypeName = ::core::ptr::null::<XML_Char>();
}
unsafe extern "C" fn notationDecl(
    mut userData: *mut ::core::ffi::c_void,
    mut notationName: *const XML_Char,
    mut _base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) {
    let mut data: *mut XmlwfUserData = userData as *mut XmlwfUserData;
    let mut entry: *mut NotationList =
        malloc(::core::mem::size_of::<NotationList>() as size_t) as *mut NotationList;
    let mut errorMessage: *const ::core::ffi::c_char =
        b"Unable to store NOTATION for output\n\0" as *const u8 as *const ::core::ffi::c_char;
    if entry.is_null() {
        fputs(errorMessage, stderr);
        return;
    }
    (*entry).notationName = xcsdup(notationName);
    if (*entry).notationName.is_null() {
        fputs(errorMessage, stderr);
        free(entry as *mut ::core::ffi::c_void);
        return;
    }
    if !systemId.is_null() {
        (*entry).systemId = xcsdup(systemId);
        if (*entry).systemId.is_null() {
            fputs(errorMessage, stderr);
            free((*entry).notationName as *mut ::core::ffi::c_void);
            free(entry as *mut ::core::ffi::c_void);
            return;
        }
    } else {
        (*entry).systemId = ::core::ptr::null::<XML_Char>();
    }
    if !publicId.is_null() {
        (*entry).publicId = xcsdup(publicId);
        if (*entry).publicId.is_null() {
            fputs(errorMessage, stderr);
            free((*entry).systemId as *mut ::core::ffi::c_void);
            free((*entry).notationName as *mut ::core::ffi::c_void);
            free(entry as *mut ::core::ffi::c_void);
            return;
        }
    } else {
        (*entry).publicId = ::core::ptr::null::<XML_Char>();
    }
    (*entry).next = (*data).notationListHead as *mut NotationList;
    (*data).notationListHead = entry;
}
unsafe extern "C" fn defaultCharacterData(
    mut userData: *mut ::core::ffi::c_void,
    mut _s: *const XML_Char,
    mut _len: ::core::ffi::c_int,
) {
    XML_DefaultCurrent(userData as XML_Parser);
}
unsafe extern "C" fn defaultStartElement(
    mut userData: *mut ::core::ffi::c_void,
    mut _name: *const XML_Char,
    mut _atts: *mut *const XML_Char,
) {
    XML_DefaultCurrent(userData as XML_Parser);
}
unsafe extern "C" fn defaultEndElement(
    mut userData: *mut ::core::ffi::c_void,
    mut _name: *const XML_Char,
) {
    XML_DefaultCurrent(userData as XML_Parser);
}
unsafe extern "C" fn defaultProcessingInstruction(
    mut userData: *mut ::core::ffi::c_void,
    mut _target: *const XML_Char,
    mut _data: *const XML_Char,
) {
    XML_DefaultCurrent(userData as XML_Parser);
}
unsafe extern "C" fn nopCharacterData(
    mut _userData: *mut ::core::ffi::c_void,
    mut _s: *const XML_Char,
    mut _len: ::core::ffi::c_int,
) {
}
unsafe extern "C" fn nopStartElement(
    mut _userData: *mut ::core::ffi::c_void,
    mut _name: *const XML_Char,
    mut _atts: *mut *const XML_Char,
) {
}
unsafe extern "C" fn nopEndElement(
    mut _userData: *mut ::core::ffi::c_void,
    mut _name: *const XML_Char,
) {
}
unsafe extern "C" fn nopProcessingInstruction(
    mut _userData: *mut ::core::ffi::c_void,
    mut _target: *const XML_Char,
    mut _data: *const XML_Char,
) {
}
unsafe extern "C" fn markup(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const XML_Char,
    mut len: ::core::ffi::c_int,
) {
    let mut fp: *mut FILE =
        (*(*(userData as XML_Parser as *mut *mut ::core::ffi::c_void) as *mut XmlwfUserData)).fp;
    while len > 0 as ::core::ffi::c_int {
        putc(*s as ::core::ffi::c_int, fp);
        len -= 1;
        s = s.offset(1);
    }
}
unsafe extern "C" fn metaLocation(mut parser: XML_Parser) {
    let mut uri: *const XML_Char = XML_GetBase(parser);
    let mut fp: *mut FILE =
        (*(*(parser as *mut *mut ::core::ffi::c_void) as *mut XmlwfUserData)).fp;
    if !uri.is_null() {
        fprintf(
            fp,
            b" uri=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            uri,
        );
    }
    fprintf(
        fp,
        b" byte=\"%ld\" nbytes=\"%d\" line=\"%lu\" col=\"%lu\"\0" as *const u8
            as *const ::core::ffi::c_char,
        XML_GetCurrentByteIndex(parser),
        XML_GetCurrentByteCount(parser),
        XML_GetCurrentLineNumber(parser),
        XML_GetCurrentColumnNumber(parser),
    );
}
unsafe extern "C" fn metaStartDocument(mut userData: *mut ::core::ffi::c_void) {
    fputs(
        b"<document>\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*(*(userData as XML_Parser as *mut *mut ::core::ffi::c_void) as *mut XmlwfUserData)).fp,
    );
}
unsafe extern "C" fn metaEndDocument(mut userData: *mut ::core::ffi::c_void) {
    fputs(
        b"</document>\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*(*(userData as XML_Parser as *mut *mut ::core::ffi::c_void) as *mut XmlwfUserData)).fp,
    );
}
unsafe extern "C" fn metaStartElement(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*data).fp;
    let mut specifiedAttsEnd: *mut *const XML_Char =
        atts.offset(XML_GetSpecifiedAttributeCount(parser) as isize);
    let mut idAttPtr: *mut *const XML_Char = ::core::ptr::null_mut::<*const XML_Char>();
    let mut idAttIndex: ::core::ffi::c_int = XML_GetIdAttributeIndex(parser);
    if idAttIndex < 0 as ::core::ffi::c_int {
        idAttPtr = ::core::ptr::null_mut::<*const XML_Char>();
    } else {
        idAttPtr = atts.offset(idAttIndex as isize);
    }
    fprintf(
        fp,
        b"<starttag name=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
        name,
    );
    metaLocation(parser);
    if !(*atts).is_null() {
        fputs(b">\n\0" as *const u8 as *const ::core::ffi::c_char, fp);
        loop {
            fprintf(
                fp,
                b"<attribute name=\"%s\" value=\"\0" as *const u8 as *const ::core::ffi::c_char,
                *atts.offset(0 as ::core::ffi::c_int as isize),
            );
            characterData(
                data as *mut ::core::ffi::c_void,
                *atts.offset(1 as ::core::ffi::c_int as isize),
                strlen(*atts.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char)
                    as ::core::ffi::c_int,
            );
            if atts >= specifiedAttsEnd {
                fputs(
                    b"\" defaulted=\"yes\"/>\n\0" as *const u8 as *const ::core::ffi::c_char,
                    fp,
                );
            } else if atts == idAttPtr {
                fputs(
                    b"\" id=\"yes\"/>\n\0" as *const u8 as *const ::core::ffi::c_char,
                    fp,
                );
            } else {
                fputs(b"\"/>\n\0" as *const u8 as *const ::core::ffi::c_char, fp);
            }
            atts = atts.offset(2 as ::core::ffi::c_int as isize);
            if (*atts).is_null() {
                break;
            }
        }
        fputs(
            b"</starttag>\n\0" as *const u8 as *const ::core::ffi::c_char,
            fp,
        );
    } else {
        fputs(b"/>\n\0" as *const u8 as *const ::core::ffi::c_char, fp);
    };
}
unsafe extern "C" fn metaEndElement(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*data).fp;
    fprintf(
        fp,
        b"<endtag name=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
        name,
    );
    metaLocation(parser);
    fputs(b"/>\n\0" as *const u8 as *const ::core::ffi::c_char, fp);
}
unsafe extern "C" fn metaProcessingInstruction(
    mut userData: *mut ::core::ffi::c_void,
    mut target: *const XML_Char,
    mut data: *const XML_Char,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut usrData: *mut XmlwfUserData =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*usrData).fp;
    fprintf(
        fp,
        b"<pi target=\"%s\" data=\"\0" as *const u8 as *const ::core::ffi::c_char,
        target,
    );
    characterData(
        usrData as *mut ::core::ffi::c_void,
        data,
        strlen(data as *const ::core::ffi::c_char) as ::core::ffi::c_int,
    );
    putc('"' as i32, fp);
    metaLocation(parser);
    fputs(b"/>\n\0" as *const u8 as *const ::core::ffi::c_char, fp);
}
unsafe extern "C" fn metaComment(
    mut userData: *mut ::core::ffi::c_void,
    mut data: *const XML_Char,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut usrData: *mut XmlwfUserData =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*usrData).fp;
    fputs(
        b"<comment data=\"\0" as *const u8 as *const ::core::ffi::c_char,
        fp,
    );
    characterData(
        usrData as *mut ::core::ffi::c_void,
        data,
        strlen(data as *const ::core::ffi::c_char) as ::core::ffi::c_int,
    );
    putc('"' as i32, fp);
    metaLocation(parser);
    fputs(b"/>\n\0" as *const u8 as *const ::core::ffi::c_char, fp);
}
unsafe extern "C" fn metaStartCdataSection(mut userData: *mut ::core::ffi::c_void) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*data).fp;
    fputs(
        b"<startcdata\0" as *const u8 as *const ::core::ffi::c_char,
        fp,
    );
    metaLocation(parser);
    fputs(b"/>\n\0" as *const u8 as *const ::core::ffi::c_char, fp);
}
unsafe extern "C" fn metaEndCdataSection(mut userData: *mut ::core::ffi::c_void) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*data).fp;
    fputs(
        b"<endcdata\0" as *const u8 as *const ::core::ffi::c_char,
        fp,
    );
    metaLocation(parser);
    fputs(b"/>\n\0" as *const u8 as *const ::core::ffi::c_char, fp);
}
unsafe extern "C" fn metaCharacterData(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const XML_Char,
    mut len: ::core::ffi::c_int,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*data).fp;
    fputs(
        b"<chars str=\"\0" as *const u8 as *const ::core::ffi::c_char,
        fp,
    );
    characterData(data as *mut ::core::ffi::c_void, s, len);
    putc('"' as i32, fp);
    metaLocation(parser);
    fputs(b"/>\n\0" as *const u8 as *const ::core::ffi::c_char, fp);
}
unsafe extern "C" fn metaStartDoctypeDecl(
    mut userData: *mut ::core::ffi::c_void,
    mut doctypeName: *const XML_Char,
    mut _sysid: *const XML_Char,
    mut _pubid: *const XML_Char,
    mut _has_internal_subset: ::core::ffi::c_int,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*data).fp;
    fprintf(
        fp,
        b"<startdoctype name=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
        doctypeName,
    );
    metaLocation(parser);
    fputs(b"/>\n\0" as *const u8 as *const ::core::ffi::c_char, fp);
}
unsafe extern "C" fn metaEndDoctypeDecl(mut userData: *mut ::core::ffi::c_void) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*data).fp;
    fputs(
        b"<enddoctype\0" as *const u8 as *const ::core::ffi::c_char,
        fp,
    );
    metaLocation(parser);
    fputs(b"/>\n\0" as *const u8 as *const ::core::ffi::c_char, fp);
}
unsafe extern "C" fn metaNotationDecl(
    mut userData: *mut ::core::ffi::c_void,
    mut notationName: *const XML_Char,
    mut _base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*data).fp;
    fprintf(
        fp,
        b"<notation name=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
        notationName,
    );
    if !publicId.is_null() {
        fprintf(
            fp,
            b" public=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            publicId,
        );
    }
    if !systemId.is_null() {
        fputs(
            b" system=\"\0" as *const u8 as *const ::core::ffi::c_char,
            fp,
        );
        characterData(
            data as *mut ::core::ffi::c_void,
            systemId,
            strlen(systemId as *const ::core::ffi::c_char) as ::core::ffi::c_int,
        );
        putc('"' as i32, fp);
    }
    metaLocation(parser);
    fputs(b"/>\n\0" as *const u8 as *const ::core::ffi::c_char, fp);
}
unsafe extern "C" fn metaEntityDecl(
    mut userData: *mut ::core::ffi::c_void,
    mut entityName: *const XML_Char,
    mut _is_param: ::core::ffi::c_int,
    mut value: *const XML_Char,
    mut value_length: ::core::ffi::c_int,
    mut _base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
    mut notationName: *const XML_Char,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*data).fp;
    if !value.is_null() {
        fprintf(
            fp,
            b"<entity name=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            entityName,
        );
        metaLocation(parser);
        putc('>' as i32, fp);
        characterData(data as *mut ::core::ffi::c_void, value, value_length);
        fputs(
            b"</entity/>\n\0" as *const u8 as *const ::core::ffi::c_char,
            fp,
        );
    } else if !notationName.is_null() {
        fprintf(
            fp,
            b"<entity name=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            entityName,
        );
        if !publicId.is_null() {
            fprintf(
                fp,
                b" public=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
                publicId,
            );
        }
        fputs(
            b" system=\"\0" as *const u8 as *const ::core::ffi::c_char,
            fp,
        );
        characterData(
            data as *mut ::core::ffi::c_void,
            systemId,
            strlen(systemId as *const ::core::ffi::c_char) as ::core::ffi::c_int,
        );
        putc('"' as i32, fp);
        fprintf(
            fp,
            b" notation=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            notationName,
        );
        metaLocation(parser);
        fputs(b"/>\n\0" as *const u8 as *const ::core::ffi::c_char, fp);
    } else {
        fprintf(
            fp,
            b"<entity name=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            entityName,
        );
        if !publicId.is_null() {
            fprintf(
                fp,
                b" public=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
                publicId,
            );
        }
        fputs(
            b" system=\"\0" as *const u8 as *const ::core::ffi::c_char,
            fp,
        );
        characterData(
            data as *mut ::core::ffi::c_void,
            systemId,
            strlen(systemId as *const ::core::ffi::c_char) as ::core::ffi::c_int,
        );
        putc('"' as i32, fp);
        metaLocation(parser);
        fputs(b"/>\n\0" as *const u8 as *const ::core::ffi::c_char, fp);
    };
}
unsafe extern "C" fn metaStartNamespaceDecl(
    mut userData: *mut ::core::ffi::c_void,
    mut prefix: *const XML_Char,
    mut uri: *const XML_Char,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*data).fp;
    fputs(b"<startns\0" as *const u8 as *const ::core::ffi::c_char, fp);
    if !prefix.is_null() {
        fprintf(
            fp,
            b" prefix=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            prefix,
        );
    }
    if !uri.is_null() {
        fputs(b" ns=\"\0" as *const u8 as *const ::core::ffi::c_char, fp);
        characterData(
            data as *mut ::core::ffi::c_void,
            uri,
            strlen(uri as *const ::core::ffi::c_char) as ::core::ffi::c_int,
        );
        fputs(b"\"/>\n\0" as *const u8 as *const ::core::ffi::c_char, fp);
    } else {
        fputs(b"/>\n\0" as *const u8 as *const ::core::ffi::c_char, fp);
    };
}
unsafe extern "C" fn metaEndNamespaceDecl(
    mut userData: *mut ::core::ffi::c_void,
    mut prefix: *const XML_Char,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut XmlwfUserData;
    let mut fp: *mut FILE = (*data).fp;
    if prefix.is_null() {
        fputs(
            b"<endns/>\n\0" as *const u8 as *const ::core::ffi::c_char,
            fp,
        );
    } else {
        fprintf(
            fp,
            b"<endns prefix=\"%s\"/>\n\0" as *const u8 as *const ::core::ffi::c_char,
            prefix,
        );
    };
}
unsafe extern "C" fn unknownEncodingConvert(
    mut data: *mut ::core::ffi::c_void,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return codepageConvert(*(data as *mut ::core::ffi::c_int), p);
}
unsafe extern "C" fn unknownEncoding(
    mut _userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
    mut info: *mut XML_Encoding,
) -> ::core::ffi::c_int {
    let mut cp: ::core::ffi::c_int = 0;
    static mut prefixL: [XML_Char; 9] =
        unsafe { ::core::mem::transmute::<[u8; 9], [XML_Char; 9]>(*b"windows-\0") };
    static mut prefixU: [XML_Char; 9] =
        unsafe { ::core::mem::transmute::<[u8; 9], [XML_Char; 9]>(*b"WINDOWS-\0") };
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while prefixU[i as usize] != 0 {
        if *name.offset(i as isize) as ::core::ffi::c_int
            != prefixU[i as usize] as ::core::ffi::c_int
            && *name.offset(i as isize) as ::core::ffi::c_int
                != prefixL[i as usize] as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    cp = 0 as ::core::ffi::c_int;
    while *name.offset(i as isize) != 0 {
        static mut digits: [XML_Char; 11] =
            unsafe { ::core::mem::transmute::<[u8; 11], [XML_Char; 11]>(*b"0123456789\0") };
        let mut s: *const XML_Char = strchr(
            &raw const digits as *const ::core::ffi::c_char,
            *name.offset(i as isize) as ::core::ffi::c_int,
        );
        if s.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        cp *= 10 as ::core::ffi::c_int;
        cp += s.offset_from(&raw const digits as *const XML_Char) as ::core::ffi::c_long
            as ::core::ffi::c_int;
        if cp >= 0x10000 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    if codepageMap(cp, &raw mut (*info).map as *mut ::core::ffi::c_int) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*info).convert = Some(
        unknownEncodingConvert
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >;
    (*info).release = Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
        as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    (*info).data = malloc(::core::mem::size_of::<::core::ffi::c_int>() as size_t);
    if (*info).data.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    *((*info).data as *mut ::core::ffi::c_int) = cp;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn notStandalone(mut _userData: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn showVersion(mut prog: *mut XML_Char) {
    let mut s: *mut XML_Char = prog;
    let mut ch: XML_Char = 0;
    let mut features: *const XML_Feature = XML_GetFeatureList();
    loop {
        ch = *s;
        if !(ch as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
            break;
        }
        if ch as ::core::ffi::c_int == '/' as i32 {
            prog = s.offset(1 as ::core::ffi::c_int as isize);
        }
        s = s.offset(1);
    }
    fprintf(
        stdout,
        b"%s using %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        prog,
        XML_ExpatVersion(),
    );
    if !features.is_null()
        && (*features.offset(0 as ::core::ffi::c_int as isize)).feature as ::core::ffi::c_uint
            != XML_FEATURE_END as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        fprintf(
            stdout,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            (*features.offset(0 as ::core::ffi::c_int as isize)).name,
        );
        if (*features.offset(0 as ::core::ffi::c_int as isize)).value != 0 {
            fprintf(
                stdout,
                b"=%ld\0" as *const u8 as *const ::core::ffi::c_char,
                (*features.offset(0 as ::core::ffi::c_int as isize)).value,
            );
        }
        while (*features.offset(i as isize)).feature as ::core::ffi::c_uint
            != XML_FEATURE_END as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            fprintf(
                stdout,
                b", %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*features.offset(i as isize)).name,
            );
            if (*features.offset(i as isize)).value != 0 {
                fprintf(
                    stdout,
                    b"=%ld\0" as *const u8 as *const ::core::ffi::c_char,
                    (*features.offset(i as isize)).value,
                );
            }
            i += 1;
        }
        fprintf(stdout, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
}
unsafe extern "C" fn usage(mut prog: *const XML_Char, mut rc: ::core::ffi::c_int) -> ! {
    fprintf(
        stderr,
        b"usage:\n  %s [OPTIONS] [FILE ...]\n  %s -h|--help\n  %s -v|--version\n\nxmlwf - Determines if an XML document is well-formed\n\npositional arguments:\n  FILE           file to process (default: STDIN)\n\ninput control arguments:\n  -s             print an error if the document is not [s]tandalone\n  -n             enable [n]amespace processing\n  -p             enable processing of external DTDs and [p]arameter entities\n  -x             enable processing of e[x]ternal entities\n                 (CAREFUL! This makes xmlwf vulnerable to external entity attacks (XXE).)\n  -e ENCODING    override any in-document [e]ncoding declaration\n  -w             enable support for [W]indows code pages\n  -r             disable memory-mapping and use [r]ead calls instead\n  -g BYTES       buffer size to request per call pair to XML_[G]etBuffer and read (default: 8 KiB)\n  -k             when processing multiple files, [k]eep processing after first file with error\n\noutput control arguments:\n  -d DIRECTORY   output [d]estination directory\n  -c             write a [c]opy of input XML, not canonical XML\n  -m             write [m]eta XML, not canonical XML\n  -t             write no XML output for [t]iming of plain parsing\n  -N             enable adding doctype and [n]otation declarations\n\namplification attack protection (e.g. billion laughs):\n  NOTE: If you ever need to increase these values for non-attack payload, please file a bug report.\n\n  -a FACTOR      set maximum tolerated [a]mplification factor (default: 100.0)\n  -b BYTES       set number of output [b]ytes needed to activate (default: 8 MiB/64 MiB)\n\nreparse deferral:\n  -q             disable reparse deferral, and allow [q]uadratic parse runtime with large tokens\n\ninfo arguments:\n  -h, --help     show this [h]elp message and exit\n  -v, --version  show program's [v]ersion number and exit\n\nenvironment variables:\n  EXPAT_ACCOUNTING_DEBUG=(0|1|2|3)\n                 Control verbosity of accounting debugging (default: 0)\n  EXPAT_ENTITY_DEBUG=(0|1)\n                 Control verbosity of entity debugging (default: 0)\n  EXPAT_ENTROPY_DEBUG=(0|1)\n                 Control verbosity of entropy debugging (default: 0)\n  EXPAT_MALLOC_DEBUG=(0|1|2)\n                 Control verbosity of allocation tracker (default: 0)\n\nexit status:\n  0              the input files are well-formed and the output (if requested) was written successfully\n  1              could not allocate data structures, signals a serious problem with execution environment\n  2              one or more input files were not well-formed\n  3              could not create an output file\n  4              command-line argument error\n\nxmlwf of libexpat is software libre, licensed under the MIT license.\nPlease report bugs at https://github.com/libexpat/libexpat/issues -- thank you!\n\0"
            as *const u8 as *const ::core::ffi::c_char,
        prog,
        prog,
        prog,
    );
    exit(rc);
}
unsafe fn main_0(mut argc: ::core::ffi::c_int, mut argv: *mut *mut XML_Char) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut outputDir: *const XML_Char = ::core::ptr::null::<XML_Char>();
    let mut encoding: *const XML_Char = ::core::ptr::null::<XML_Char>();
    let mut processFlags: ::core::ffi::c_uint = XML_MAP_FILE as ::core::ffi::c_uint;
    let mut windowsCodePages: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut outputType: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut useNamespaces: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut requireStandalone: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut requiresNotations: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut continueOnError: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut attackMaximumAmplification: ::core::ffi::c_float = -1.0f32;
    let mut attackThresholdBytes: ::core::ffi::c_ulonglong = 0 as ::core::ffi::c_ulonglong;
    let mut attackThresholdGiven: XML_Bool = XML_FALSE;
    let mut disableDeferral: XML_Bool = XML_FALSE;
    let mut exitCode: ::core::ffi::c_int = XMLWF_EXIT_SUCCESS as ::core::ffi::c_int;
    let mut paramEntityParsing: XML_ParamEntityParsing = XML_PARAM_ENTITY_PARSING_NEVER;
    let mut useStdin: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut userData: XmlwfUserData = xmlwfUserData {
        fp: ::core::ptr::null_mut::<FILE>(),
        notationListHead: ::core::ptr::null_mut::<NotationList>(),
        currentDoctypeName: ::core::ptr::null::<XML_Char>(),
    };
    i = 1 as ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    while i < argc {
        if j == 0 as ::core::ffi::c_int {
            if *(*argv.offset(i as isize)).offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                != '-' as i32
            {
                break;
            }
            if *(*argv.offset(i as isize)).offset(1 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                == '-' as i32
            {
                if *(*argv.offset(i as isize)).offset(2 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\0' as i32
                {
                    i += 1;
                    break;
                } else if strcmp(
                    (*argv.offset(i as isize)).offset(2 as ::core::ffi::c_int as isize),
                    b"help\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    usage(
                        *argv.offset(0 as ::core::ffi::c_int as isize),
                        XMLWF_EXIT_SUCCESS as ::core::ffi::c_int,
                    );
                } else if strcmp(
                    (*argv.offset(i as isize)).offset(2 as ::core::ffi::c_int as isize),
                    b"version\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    showVersion(*argv.offset(0 as ::core::ffi::c_int as isize));
                    return XMLWF_EXIT_SUCCESS as ::core::ffi::c_int;
                }
            }
            j += 1;
        }
        let mut current_block_122: u64;
        match *(*argv.offset(i as isize)).offset(j as isize) as ::core::ffi::c_int {
            114 => {
                processFlags &= !XML_MAP_FILE as ::core::ffi::c_uint;
                j += 1;
                current_block_122 = 8602574157404971894;
            }
            115 => {
                requireStandalone = 1 as ::core::ffi::c_int;
                j += 1;
                current_block_122 = 8602574157404971894;
            }
            110 => {
                useNamespaces = 1 as ::core::ffi::c_int;
                j += 1;
                current_block_122 = 8602574157404971894;
            }
            112 => {
                paramEntityParsing = XML_PARAM_ENTITY_PARSING_ALWAYS;
                current_block_122 = 8584636603070058492;
            }
            120 => {
                current_block_122 = 8584636603070058492;
            }
            119 => {
                windowsCodePages = 1 as ::core::ffi::c_int;
                j += 1;
                current_block_122 = 8602574157404971894;
            }
            109 => {
                outputType = 'm' as i32;
                j += 1;
                current_block_122 = 8602574157404971894;
            }
            99 => {
                outputType = 'c' as i32;
                useNamespaces = 0 as ::core::ffi::c_int;
                j += 1;
                current_block_122 = 8602574157404971894;
            }
            116 => {
                outputType = 't' as i32;
                j += 1;
                current_block_122 = 8602574157404971894;
            }
            78 => {
                requiresNotations = 1 as ::core::ffi::c_int;
                j += 1;
                current_block_122 = 8602574157404971894;
            }
            100 => {
                if *(*argv.offset(i as isize)).offset((j + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == '\0' as i32
                {
                    i += 1;
                    if i == argc {
                        usage(
                            *argv.offset(0 as ::core::ffi::c_int as isize),
                            XMLWF_EXIT_USAGE_ERROR as ::core::ffi::c_int,
                        );
                    }
                    outputDir = *argv.offset(i as isize);
                } else {
                    outputDir = (*argv.offset(i as isize))
                        .offset(j as isize)
                        .offset(1 as ::core::ffi::c_int as isize);
                }
                i += 1;
                j = 0 as ::core::ffi::c_int;
                current_block_122 = 8602574157404971894;
            }
            101 => {
                if *(*argv.offset(i as isize)).offset((j + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == '\0' as i32
                {
                    i += 1;
                    if i == argc {
                        usage(
                            *argv.offset(0 as ::core::ffi::c_int as isize),
                            XMLWF_EXIT_USAGE_ERROR as ::core::ffi::c_int,
                        );
                    }
                    encoding = *argv.offset(i as isize);
                } else {
                    encoding = (*argv.offset(i as isize))
                        .offset(j as isize)
                        .offset(1 as ::core::ffi::c_int as isize);
                }
                i += 1;
                j = 0 as ::core::ffi::c_int;
                current_block_122 = 8602574157404971894;
            }
            104 => {
                usage(
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                    XMLWF_EXIT_SUCCESS as ::core::ffi::c_int,
                );
            }
            118 => {
                showVersion(*argv.offset(0 as ::core::ffi::c_int as isize));
                return XMLWF_EXIT_SUCCESS as ::core::ffi::c_int;
            }
            103 => {
                let mut valueText: *const XML_Char = ::core::ptr::null::<XML_Char>();
                if *(*argv.offset(i as isize)).offset((j + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == '\0' as i32
                {
                    i += 1;
                    if i == argc {
                        usage(
                            *argv.offset(0 as ::core::ffi::c_int as isize),
                            XMLWF_EXIT_USAGE_ERROR as ::core::ffi::c_int,
                        );
                    }
                    valueText = *argv.offset(i as isize);
                } else {
                    valueText = (*argv.offset(i as isize))
                        .offset(j as isize)
                        .offset(1 as ::core::ffi::c_int as isize);
                }
                i += 1;
                j = 0 as ::core::ffi::c_int;
                *__errno_location() = 0 as ::core::ffi::c_int;
                let mut afterValueText: *mut XML_Char = valueText as *mut XML_Char;
                let read_size_bytes_candidate: ::core::ffi::c_longlong = strtoull(
                    valueText as *const ::core::ffi::c_char,
                    &raw mut afterValueText,
                    10 as ::core::ffi::c_int,
                )
                    as ::core::ffi::c_longlong;
                if *__errno_location() != 0 as ::core::ffi::c_int
                    || *afterValueText.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        != '\0' as i32
                    || read_size_bytes_candidate < 1 as ::core::ffi::c_longlong
                    || read_size_bytes_candidate
                        > (INT_MAX / 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_longlong
                {
                    *__errno_location() = ERANGE;
                    perror(
                        b"invalid buffer size (needs an integer from 1 to INT_MAX/2+1 i.e. 1,073,741,824 on most platforms)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    exit(XMLWF_EXIT_USAGE_ERROR as ::core::ffi::c_int);
                }
                g_read_size_bytes = read_size_bytes_candidate as ::core::ffi::c_int;
                current_block_122 = 8602574157404971894;
            }
            107 => {
                continueOnError = 1 as ::core::ffi::c_int;
                j += 1;
                current_block_122 = 8602574157404971894;
            }
            97 => {
                let mut valueText_0: *const XML_Char = ::core::ptr::null::<XML_Char>();
                if *(*argv.offset(i as isize)).offset((j + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == '\0' as i32
                {
                    i += 1;
                    if i == argc {
                        usage(
                            *argv.offset(0 as ::core::ffi::c_int as isize),
                            XMLWF_EXIT_USAGE_ERROR as ::core::ffi::c_int,
                        );
                    }
                    valueText_0 = *argv.offset(i as isize);
                } else {
                    valueText_0 = (*argv.offset(i as isize))
                        .offset(j as isize)
                        .offset(1 as ::core::ffi::c_int as isize);
                }
                i += 1;
                j = 0 as ::core::ffi::c_int;
                *__errno_location() = 0 as ::core::ffi::c_int;
                let mut afterValueText_0: *mut XML_Char = ::core::ptr::null_mut::<XML_Char>();
                attackMaximumAmplification = strtof(
                    valueText_0 as *const ::core::ffi::c_char,
                    &raw mut afterValueText_0,
                );
                if *__errno_location() != 0 as ::core::ffi::c_int
                    || *afterValueText_0.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        != '\0' as i32
                    || attackMaximumAmplification.is_nan() as i32 != 0
                    || attackMaximumAmplification < 1.0f32
                {
                    *__errno_location() = ERANGE;
                    perror(
                        b"invalid amplification limit (needs a floating point number greater or equal than 1.0)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    exit(XMLWF_EXIT_USAGE_ERROR as ::core::ffi::c_int);
                }
                current_block_122 = 8602574157404971894;
            }
            98 => {
                let mut valueText_1: *const XML_Char = ::core::ptr::null::<XML_Char>();
                if *(*argv.offset(i as isize)).offset((j + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == '\0' as i32
                {
                    i += 1;
                    if i == argc {
                        usage(
                            *argv.offset(0 as ::core::ffi::c_int as isize),
                            XMLWF_EXIT_USAGE_ERROR as ::core::ffi::c_int,
                        );
                    }
                    valueText_1 = *argv.offset(i as isize);
                } else {
                    valueText_1 = (*argv.offset(i as isize))
                        .offset(j as isize)
                        .offset(1 as ::core::ffi::c_int as isize);
                }
                i += 1;
                j = 0 as ::core::ffi::c_int;
                *__errno_location() = 0 as ::core::ffi::c_int;
                let mut afterValueText_1: *mut XML_Char = valueText_1 as *mut XML_Char;
                attackThresholdBytes = strtoull(
                    valueText_1 as *const ::core::ffi::c_char,
                    &raw mut afterValueText_1,
                    10 as ::core::ffi::c_int,
                );
                if *__errno_location() != 0 as ::core::ffi::c_int
                    || *afterValueText_1.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        != '\0' as i32
                {
                    *__errno_location() = ERANGE;
                    perror(
                        b"invalid ignore threshold (needs an integer from 0 to 2^64-1)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    exit(XMLWF_EXIT_USAGE_ERROR as ::core::ffi::c_int);
                }
                attackThresholdGiven = XML_TRUE;
                current_block_122 = 8602574157404971894;
            }
            113 => {
                disableDeferral = XML_TRUE;
                j += 1;
                current_block_122 = 8602574157404971894;
            }
            0 => {
                if j > 1 as ::core::ffi::c_int {
                    i += 1;
                    j = 0 as ::core::ffi::c_int;
                    current_block_122 = 8602574157404971894;
                } else {
                    current_block_122 = 11328332973165170756;
                }
            }
            _ => {
                current_block_122 = 11328332973165170756;
            }
        }
        match current_block_122 {
            8584636603070058492 => {
                processFlags |= XML_EXTERNAL_ENTITIES as ::core::ffi::c_uint;
                j += 1;
            }
            11328332973165170756 => {
                usage(
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                    XMLWF_EXIT_USAGE_ERROR as ::core::ffi::c_int,
                );
            }
            _ => {}
        }
    }
    if i == argc {
        useStdin = 1 as ::core::ffi::c_int;
        processFlags &= !XML_MAP_FILE as ::core::ffi::c_uint;
        i -= 1;
    }
    let mut current_block_219: u64;
    while i < argc {
        let mut outName: *mut XML_Char = ::core::ptr::null_mut::<XML_Char>();
        let mut result: ::core::ffi::c_int = 0;
        let mut parser: XML_Parser = ::core::ptr::null_mut::<XML_ParserStruct>();
        if useNamespaces != 0 {
            parser = XML_ParserCreateNS(encoding, '\u{1}' as i32 as XML_Char);
        } else {
            parser = XML_ParserCreate(encoding);
        }
        if parser.is_null() {
            perror(b"Could not instantiate parser\0" as *const u8 as *const ::core::ffi::c_char);
            exit(XMLWF_EXIT_INTERNAL_ERROR as ::core::ffi::c_int);
        }
        if attackMaximumAmplification != -1.0f32 {
            XML_SetBillionLaughsAttackProtectionMaximumAmplification(
                parser,
                attackMaximumAmplification,
            );
            XML_SetAllocTrackerMaximumAmplification(parser, attackMaximumAmplification);
        }
        if attackThresholdGiven != 0 {
            XML_SetBillionLaughsAttackProtectionActivationThreshold(parser, attackThresholdBytes);
            XML_SetAllocTrackerActivationThreshold(parser, attackThresholdBytes);
        }
        if disableDeferral != 0 {
            let success: XML_Bool = XML_SetReparseDeferralEnabled(parser, XML_FALSE) as XML_Bool;
            if success == 0 {
                *__errno_location() = EINVAL;
                perror(
                    b"Failed to disable reparse deferral\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                exit(XMLWF_EXIT_INTERNAL_ERROR as ::core::ffi::c_int);
            }
        }
        if requireStandalone != 0 {
            XML_SetNotStandaloneHandler(
                parser,
                Some(
                    notStandalone
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
                ),
            );
        }
        XML_SetParamEntityParsing(parser, paramEntityParsing);
        if outputType == 't' as i32 {
            outputDir = ::core::ptr::null::<XML_Char>();
            XML_SetElementHandler(
                parser,
                Some(
                    nopStartElement
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *mut *const XML_Char,
                        ) -> (),
                ),
                Some(
                    nopEndElement
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
                ),
            );
            XML_SetCharacterDataHandler(
                parser,
                Some(
                    nopCharacterData
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            ::core::ffi::c_int,
                        ) -> (),
                ),
            );
            XML_SetProcessingInstructionHandler(
                parser,
                Some(
                    nopProcessingInstruction
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> (),
                ),
            );
            current_block_219 = 9952640327414195044;
        } else if !outputDir.is_null() {
            let mut delim: *const XML_Char = b"/\0" as *const u8 as *const XML_Char;
            let mut file: *const XML_Char = if useStdin != 0 {
                b"STDIN\0" as *const u8 as *const XML_Char
            } else {
                *argv.offset(i as isize) as *const XML_Char
            };
            if useStdin == 0 {
                let mut lastDelim: *const XML_Char = strrchr(
                    file as *const ::core::ffi::c_char,
                    *delim.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                );
                if !lastDelim.is_null() {
                    file = lastDelim.offset(1 as ::core::ffi::c_int as isize);
                }
            }
            outName = malloc(
                strlen(outputDir as *const ::core::ffi::c_char)
                    .wrapping_add(strlen(file as *const ::core::ffi::c_char))
                    .wrapping_add(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<XML_Char>() as size_t),
            ) as *mut XML_Char;
            if outName.is_null() {
                perror(b"Could not allocate memory\0" as *const u8 as *const ::core::ffi::c_char);
                exit(XMLWF_EXIT_INTERNAL_ERROR as ::core::ffi::c_int);
            }
            strcpy(
                outName as *mut ::core::ffi::c_char,
                outputDir as *const ::core::ffi::c_char,
            );
            strcat(
                outName as *mut ::core::ffi::c_char,
                delim as *const ::core::ffi::c_char,
            );
            strcat(
                outName as *mut ::core::ffi::c_char,
                file as *const ::core::ffi::c_char,
            );
            userData.fp =
                fopen(outName, b"wb\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
            if userData.fp.is_null() {
                perror(outName);
                exitCode = XMLWF_EXIT_OUTPUT_ERROR as ::core::ffi::c_int;
                free(outName as *mut ::core::ffi::c_void);
                XML_ParserFree(parser);
                if !(continueOnError != 0) {
                    break;
                }
                current_block_219 = 15947798178928648489;
            } else {
                setvbuf(
                    userData.fp,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    _IOFBF,
                    16384 as size_t,
                );
                XML_SetUserData(parser, &raw mut userData as *mut ::core::ffi::c_void);
                match outputType {
                    109 => {
                        XML_UseParserAsHandlerArg(parser);
                        XML_SetElementHandler(
                            parser,
                            Some(
                                metaStartElement
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const XML_Char,
                                        *mut *const XML_Char,
                                    )
                                        -> (),
                            ),
                            Some(
                                metaEndElement
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const XML_Char,
                                    )
                                        -> (),
                            ),
                        );
                        XML_SetProcessingInstructionHandler(
                            parser,
                            Some(
                                metaProcessingInstruction
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const XML_Char,
                                        *const XML_Char,
                                    )
                                        -> (),
                            ),
                        );
                        XML_SetCommentHandler(
                            parser,
                            Some(
                                metaComment
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const XML_Char,
                                    )
                                        -> (),
                            ),
                        );
                        XML_SetCdataSectionHandler(
                            parser,
                            Some(
                                metaStartCdataSection
                                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                            ),
                            Some(
                                metaEndCdataSection
                                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                            ),
                        );
                        XML_SetCharacterDataHandler(
                            parser,
                            Some(
                                metaCharacterData
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const XML_Char,
                                        ::core::ffi::c_int,
                                    )
                                        -> (),
                            ),
                        );
                        XML_SetDoctypeDeclHandler(
                            parser,
                            Some(
                                metaStartDoctypeDecl
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const XML_Char,
                                        *const XML_Char,
                                        *const XML_Char,
                                        ::core::ffi::c_int,
                                    )
                                        -> (),
                            ),
                            Some(
                                metaEndDoctypeDecl
                                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                            ),
                        );
                        XML_SetEntityDeclHandler(
                            parser,
                            Some(
                                metaEntityDecl
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const XML_Char,
                                        ::core::ffi::c_int,
                                        *const XML_Char,
                                        ::core::ffi::c_int,
                                        *const XML_Char,
                                        *const XML_Char,
                                        *const XML_Char,
                                        *const XML_Char,
                                    )
                                        -> (),
                            ),
                        );
                        XML_SetNotationDeclHandler(
                            parser,
                            Some(
                                metaNotationDecl
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const XML_Char,
                                        *const XML_Char,
                                        *const XML_Char,
                                        *const XML_Char,
                                    )
                                        -> (),
                            ),
                        );
                        XML_SetNamespaceDeclHandler(
                            parser,
                            Some(
                                metaStartNamespaceDecl
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const XML_Char,
                                        *const XML_Char,
                                    )
                                        -> (),
                            ),
                            Some(
                                metaEndNamespaceDecl
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const XML_Char,
                                    )
                                        -> (),
                            ),
                        );
                        metaStartDocument(parser as *mut ::core::ffi::c_void);
                    }
                    99 => {
                        XML_UseParserAsHandlerArg(parser);
                        XML_SetDefaultHandler(
                            parser,
                            Some(
                                markup
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const XML_Char,
                                        ::core::ffi::c_int,
                                    )
                                        -> (),
                            ),
                        );
                        XML_SetElementHandler(
                            parser,
                            Some(
                                defaultStartElement
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const XML_Char,
                                        *mut *const XML_Char,
                                    )
                                        -> (),
                            ),
                            Some(
                                defaultEndElement
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const XML_Char,
                                    )
                                        -> (),
                            ),
                        );
                        XML_SetCharacterDataHandler(
                            parser,
                            Some(
                                defaultCharacterData
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const XML_Char,
                                        ::core::ffi::c_int,
                                    )
                                        -> (),
                            ),
                        );
                        XML_SetProcessingInstructionHandler(
                            parser,
                            Some(
                                defaultProcessingInstruction
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const XML_Char,
                                        *const XML_Char,
                                    )
                                        -> (),
                            ),
                        );
                    }
                    _ => {
                        if useNamespaces != 0 {
                            XML_SetElementHandler(
                                parser,
                                Some(
                                    startElementNS
                                        as unsafe extern "C" fn(
                                            *mut ::core::ffi::c_void,
                                            *const XML_Char,
                                            *mut *const XML_Char,
                                        )
                                            -> (),
                                ),
                                Some(
                                    endElementNS
                                        as unsafe extern "C" fn(
                                            *mut ::core::ffi::c_void,
                                            *const XML_Char,
                                        )
                                            -> (),
                                ),
                            );
                        } else {
                            XML_SetElementHandler(
                                parser,
                                Some(
                                    startElement
                                        as unsafe extern "C" fn(
                                            *mut ::core::ffi::c_void,
                                            *const XML_Char,
                                            *mut *const XML_Char,
                                        )
                                            -> (),
                                ),
                                Some(
                                    endElement
                                        as unsafe extern "C" fn(
                                            *mut ::core::ffi::c_void,
                                            *const XML_Char,
                                        )
                                            -> (),
                                ),
                            );
                        }
                        XML_SetCharacterDataHandler(
                            parser,
                            Some(
                                characterData
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const XML_Char,
                                        ::core::ffi::c_int,
                                    )
                                        -> (),
                            ),
                        );
                        XML_SetProcessingInstructionHandler(
                            parser,
                            Some(
                                processingInstruction
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const XML_Char,
                                        *const XML_Char,
                                    )
                                        -> (),
                            ),
                        );
                        if requiresNotations != 0 {
                            XML_SetDoctypeDeclHandler(
                                parser,
                                Some(
                                    startDoctypeDecl
                                        as unsafe extern "C" fn(
                                            *mut ::core::ffi::c_void,
                                            *const XML_Char,
                                            *const XML_Char,
                                            *const XML_Char,
                                            ::core::ffi::c_int,
                                        )
                                            -> (),
                                ),
                                Some(
                                    endDoctypeDecl
                                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                                ),
                            );
                            XML_SetNotationDeclHandler(
                                parser,
                                Some(
                                    notationDecl
                                        as unsafe extern "C" fn(
                                            *mut ::core::ffi::c_void,
                                            *const XML_Char,
                                            *const XML_Char,
                                            *const XML_Char,
                                            *const XML_Char,
                                        )
                                            -> (),
                                ),
                            );
                        }
                    }
                }
                current_block_219 = 9952640327414195044;
            }
        } else {
            current_block_219 = 9952640327414195044;
        }
        match current_block_219 {
            9952640327414195044 => {
                if windowsCodePages != 0 {
                    XML_SetUnknownEncodingHandler(
                        parser,
                        Some(
                            unknownEncoding
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *const XML_Char,
                                    *mut XML_Encoding,
                                )
                                    -> ::core::ffi::c_int,
                        ),
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    );
                }
                result = XML_ProcessFile(
                    parser,
                    if useStdin != 0 {
                        ::core::ptr::null_mut::<XML_Char>()
                    } else {
                        *argv.offset(i as isize)
                    },
                    processFlags,
                );
                if !outputDir.is_null() {
                    if outputType == 'm' as i32 {
                        metaEndDocument(parser as *mut ::core::ffi::c_void);
                    }
                    fclose(userData.fp);
                    if result == 0 {
                        remove(outName);
                    }
                    free(outName as *mut ::core::ffi::c_void);
                }
                XML_ParserFree(parser);
                if result == 0 {
                    exitCode = XMLWF_EXIT_NOT_WELLFORMED as ::core::ffi::c_int;
                    cleanupUserData(&raw mut userData);
                    if continueOnError == 0 {
                        break;
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    return exitCode;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut XML_Char,
        ) as i32)
    }
}
