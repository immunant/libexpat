#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
pub mod expat_h {
    pub type XML_Parser = *mut ::expat::expat_h::XML_ParserStruct;

    pub type XML_Bool = ::core::ffi::c_uchar;

    pub type XML_StartElementHandler = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const crate::expat_external_h::XML_Char,
            *mut *const crate::expat_external_h::XML_Char,
        ) -> (),
    >;

    pub type XML_EndElementHandler = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const crate::expat_external_h::XML_Char,
        ) -> (),
    >;

    pub type XML_CharacterDataHandler = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const crate::expat_external_h::XML_Char,
            ::core::ffi::c_int,
        ) -> (),
    >;

    pub type XML_ProcessingInstructionHandler = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const crate::expat_external_h::XML_Char,
            *const crate::expat_external_h::XML_Char,
        ) -> (),
    >;

    pub type XML_CommentHandler = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const crate::expat_external_h::XML_Char,
        ) -> (),
    >;

    pub type XML_StartCdataSectionHandler =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;

    pub type XML_EndCdataSectionHandler =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;

    pub type XML_DefaultHandler = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const crate::expat_external_h::XML_Char,
            ::core::ffi::c_int,
        ) -> (),
    >;

    pub type XML_StartDoctypeDeclHandler = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const crate::expat_external_h::XML_Char,
            *const crate::expat_external_h::XML_Char,
            *const crate::expat_external_h::XML_Char,
            ::core::ffi::c_int,
        ) -> (),
    >;

    pub type XML_EndDoctypeDeclHandler =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;

    pub type XML_EntityDeclHandler = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const crate::expat_external_h::XML_Char,
            ::core::ffi::c_int,
            *const crate::expat_external_h::XML_Char,
            ::core::ffi::c_int,
            *const crate::expat_external_h::XML_Char,
            *const crate::expat_external_h::XML_Char,
            *const crate::expat_external_h::XML_Char,
            *const crate::expat_external_h::XML_Char,
        ) -> (),
    >;

    pub type XML_NotationDeclHandler = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const crate::expat_external_h::XML_Char,
            *const crate::expat_external_h::XML_Char,
            *const crate::expat_external_h::XML_Char,
            *const crate::expat_external_h::XML_Char,
        ) -> (),
    >;

    pub type XML_StartNamespaceDeclHandler = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const crate::expat_external_h::XML_Char,
            *const crate::expat_external_h::XML_Char,
        ) -> (),
    >;

    pub type XML_EndNamespaceDeclHandler = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const crate::expat_external_h::XML_Char,
        ) -> (),
    >;

    pub type XML_NotStandaloneHandler =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;

    pub type XML_UnknownEncodingHandler = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const crate::expat_external_h::XML_Char,
            *mut ::expat::expat_h::XML_Encoding,
        ) -> ::core::ffi::c_int,
    >;

    pub type XML_ParamEntityParsing = ::core::ffi::c_uint;

    pub type XML_FeatureEnum = ::core::ffi::c_uint;
}
pub mod expat_external_h {
    pub type XML_Char = ::core::ffi::c_char;

    pub type XML_LChar = ::core::ffi::c_char;

    pub type XML_Index = ::core::ffi::c_long;

    pub type XML_Size = ::core::ffi::c_ulong;
}
pub mod __stddef_size_t_h {
    pub type size_t = usize;
}
pub mod xmlfile_h {
    extern "C" {
        pub static mut g_read_size_bytes: ::core::ffi::c_int;
    }
}
pub mod stdlib {
    extern "C" {
        pub static mut stdout: *mut crate::stdlib::FILE;

        pub static mut stderr: *mut crate::stdlib::FILE;

        pub fn remove(__filename: *const ::core::ffi::c_char) -> ::core::ffi::c_int;

        pub fn fclose(__stream: *mut crate::stdlib::FILE) -> ::core::ffi::c_int;

        pub fn fopen(
            __filename: *const ::core::ffi::c_char,
            __modes: *const ::core::ffi::c_char,
        ) -> *mut crate::stdlib::FILE;

        pub fn setvbuf(
            __stream: *mut crate::stdlib::FILE,
            __buf: *mut ::core::ffi::c_char,
            __modes: ::core::ffi::c_int,
            __n: crate::__stddef_size_t_h::size_t,
        ) -> ::core::ffi::c_int;

        pub fn putc(
            __c: ::core::ffi::c_int,
            __stream: *mut crate::stdlib::FILE,
        ) -> ::core::ffi::c_int;

        pub fn fputs(
            __s: *const ::core::ffi::c_char,
            __stream: *mut crate::stdlib::FILE,
        ) -> ::core::ffi::c_int;
        pub fn strtof(
            __nptr: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_float;

        pub fn strtoull(
            __nptr: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
            __base: ::core::ffi::c_int,
        ) -> ::core::ffi::c_ulonglong;

        pub fn qsort(
            __base: *mut ::core::ffi::c_void,
            __nmemb: crate::__stddef_size_t_h::size_t,
            __size: crate::__stddef_size_t_h::size_t,
            __compar: crate::stdlib::__compar_fn_t,
        );
        pub fn strcat(
            __dest: *mut ::core::ffi::c_char,
            __src: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;

        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;

        pub fn strchr(
            __s: *const ::core::ffi::c_char,
            __c: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_char;
        pub type _IO_marker;

        pub type _IO_codecvt;

        pub type _IO_wide_data;
    }
    pub type FILE = ::expat::stdlib::_IO_FILE;
    pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 46] = unsafe {
        ::core::mem::transmute::<[u8; 46], [::core::ffi::c_char; 46]>(
            *b"void attributeValue(FILE *, const XML_Char *)\0",
        )
    };
    pub const EINVAL: ::core::ffi::c_int = 22;

    pub const ERANGE: ::core::ffi::c_int = 34;
    pub const _IOFBF: ::core::ffi::c_int = 0;
    pub type __compar_fn_t = Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >;
    pub type _IO_lock_t = ();
    pub type __uint64_t = u64;

    pub type __off_t = ::core::ffi::c_long;

    pub type __off64_t = ::core::ffi::c_long;
}
extern crate c2rust_bitfields;

pub use crate::__stddef_size_t_h::size_t;

pub use crate::stdlib::__ASSERT_FUNCTION;

pub use ::expat::stdlib::__assert_fail;

pub use crate::expat_external_h::XML_Char;
pub use crate::expat_external_h::XML_Index;
pub use crate::expat_external_h::XML_LChar;
pub use crate::expat_external_h::XML_Size;
pub use crate::expat_h::XML_Bool;
pub use crate::expat_h::XML_CharacterDataHandler;
pub use crate::expat_h::XML_CommentHandler;
pub use crate::expat_h::XML_DefaultHandler;
pub use crate::expat_h::XML_EndCdataSectionHandler;
pub use crate::expat_h::XML_EndDoctypeDeclHandler;
pub use crate::expat_h::XML_EndElementHandler;
pub use crate::expat_h::XML_EndNamespaceDeclHandler;
pub use crate::expat_h::XML_EntityDeclHandler;
pub use crate::expat_h::XML_FeatureEnum;
pub use crate::expat_h::XML_NotStandaloneHandler;
pub use crate::expat_h::XML_NotationDeclHandler;
pub use crate::expat_h::XML_ParamEntityParsing;
pub use crate::expat_h::XML_Parser;
pub use crate::expat_h::XML_ProcessingInstructionHandler;
pub use crate::expat_h::XML_StartCdataSectionHandler;
pub use crate::expat_h::XML_StartDoctypeDeclHandler;
pub use crate::expat_h::XML_StartElementHandler;
pub use crate::expat_h::XML_StartNamespaceDeclHandler;
pub use crate::expat_h::XML_UnknownEncodingHandler;
pub use ::expat::expat_h::XML_Encoding;
pub use ::expat::expat_h::XML_Feature;
pub use ::expat::expat_h::XML_ParserStruct;
pub use ::expat::expat_h::XML_FALSE;
pub use ::expat::expat_h::XML_FEATURE_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT;
pub use ::expat::expat_h::XML_FEATURE_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT;
pub use ::expat::expat_h::XML_FEATURE_ATTR_INFO;
pub use ::expat::expat_h::XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT;
pub use ::expat::expat_h::XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT;
pub use ::expat::expat_h::XML_FEATURE_CONTEXT_BYTES;
pub use ::expat::expat_h::XML_FEATURE_DTD;
pub use ::expat::expat_h::XML_FEATURE_END;
pub use ::expat::expat_h::XML_FEATURE_GE;
pub use ::expat::expat_h::XML_FEATURE_LARGE_SIZE;
pub use ::expat::expat_h::XML_FEATURE_MIN_SIZE;
pub use ::expat::expat_h::XML_FEATURE_NS;
pub use ::expat::expat_h::XML_FEATURE_SIZEOF_XML_CHAR;
pub use ::expat::expat_h::XML_FEATURE_SIZEOF_XML_LCHAR;
pub use ::expat::expat_h::XML_FEATURE_UNICODE;
pub use ::expat::expat_h::XML_FEATURE_UNICODE_WCHAR_T;
pub use ::expat::expat_h::XML_PARAM_ENTITY_PARSING_ALWAYS;
pub use ::expat::expat_h::XML_PARAM_ENTITY_PARSING_NEVER;
pub use ::expat::expat_h::XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE;
pub use ::expat::expat_h::XML_TRUE;
pub use ::expat::limits_h::INT_MAX;
pub use ::expat::src::lib::xmlparse::XML_DefaultCurrent;
pub use ::expat::src::lib::xmlparse::XML_ExpatVersion;
pub use ::expat::src::lib::xmlparse::XML_GetBase;
pub use ::expat::src::lib::xmlparse::XML_GetCurrentByteCount;
pub use ::expat::src::lib::xmlparse::XML_GetCurrentByteIndex;
pub use ::expat::src::lib::xmlparse::XML_GetCurrentColumnNumber;
pub use ::expat::src::lib::xmlparse::XML_GetCurrentLineNumber;
pub use ::expat::src::lib::xmlparse::XML_GetFeatureList;
pub use ::expat::src::lib::xmlparse::XML_GetIdAttributeIndex;
pub use ::expat::src::lib::xmlparse::XML_GetSpecifiedAttributeCount;
pub use ::expat::src::lib::xmlparse::XML_ParserCreate;
pub use ::expat::src::lib::xmlparse::XML_ParserCreateNS;
pub use ::expat::src::lib::xmlparse::XML_ParserFree;
pub use ::expat::src::lib::xmlparse::XML_SetAllocTrackerActivationThreshold;
pub use ::expat::src::lib::xmlparse::XML_SetAllocTrackerMaximumAmplification;
pub use ::expat::src::lib::xmlparse::XML_SetBillionLaughsAttackProtectionActivationThreshold;
pub use ::expat::src::lib::xmlparse::XML_SetBillionLaughsAttackProtectionMaximumAmplification;
pub use ::expat::src::lib::xmlparse::XML_SetCdataSectionHandler;
pub use ::expat::src::lib::xmlparse::XML_SetCharacterDataHandler;
pub use ::expat::src::lib::xmlparse::XML_SetCommentHandler;
pub use ::expat::src::lib::xmlparse::XML_SetDefaultHandler;
pub use ::expat::src::lib::xmlparse::XML_SetDoctypeDeclHandler;
pub use ::expat::src::lib::xmlparse::XML_SetElementHandler;
pub use ::expat::src::lib::xmlparse::XML_SetEntityDeclHandler;
pub use ::expat::src::lib::xmlparse::XML_SetNamespaceDeclHandler;
pub use ::expat::src::lib::xmlparse::XML_SetNotStandaloneHandler;
pub use ::expat::src::lib::xmlparse::XML_SetNotationDeclHandler;
pub use ::expat::src::lib::xmlparse::XML_SetParamEntityParsing;
pub use ::expat::src::lib::xmlparse::XML_SetProcessingInstructionHandler;
pub use ::expat::src::lib::xmlparse::XML_SetReparseDeferralEnabled;
pub use ::expat::src::lib::xmlparse::XML_SetUnknownEncodingHandler;
pub use ::expat::src::lib::xmlparse::XML_SetUserData;
pub use ::expat::src::lib::xmlparse::XML_UseParserAsHandlerArg;

pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__compar_fn_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__uint64_t;
pub use crate::stdlib::fclose;
pub use crate::stdlib::fopen;
pub use crate::stdlib::fputs;
pub use crate::stdlib::putc;
pub use crate::stdlib::qsort;
pub use crate::stdlib::remove;
pub use crate::stdlib::setvbuf;
pub use crate::stdlib::stderr;
pub use crate::stdlib::stdout;

pub use crate::stdlib::strtof;
pub use crate::stdlib::strtoull;
pub use crate::stdlib::EINVAL;
pub use crate::stdlib::ERANGE;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IOFBF;
pub use crate::xmlfile_h::g_read_size_bytes;
pub use ::expat::__stddef_null_h::NULL;
pub use ::expat::internal::__INT_MAX__;
use ::expat::src::xmlwf::xmlfile::XML_ProcessFile;
pub use ::expat::src::xmlwf::xmlfile::XML_EXTERNAL_ENTITIES;
pub use ::expat::src::xmlwf::xmlfile::XML_MAP_FILE;
pub use ::expat::stdlib::exit;
pub use ::expat::stdlib::fprintf;
pub use ::expat::stdlib::free;
pub use ::expat::stdlib::malloc;

pub use ::expat::stdlib::perror;

pub use ::expat::stdlib::_IO_FILE;

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

unsafe extern "C" fn characterData(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const XML_Char,
    mut len: ::core::ffi::c_int,
) {
    let mut fp: *mut FILE = (*(userData as *mut XmlwfUserData)).fp;
    while len > 0 {
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
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/xmlwf/xmlwf.c\0" as *const u8
                as *const ::core::ffi::c_char,
            134u32,
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
    return crate::stdlib::strcmp(
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
    fputs(name, fp);
    p = atts;
    while !(*p).is_null() {
        p = p.offset(1);
    }
    nAtts = (p.offset_from(atts) as ::core::ffi::c_long >> 1) as ::core::ffi::c_int;
    if nAtts > 1 {
        qsort(
            atts as *mut ::core::ffi::c_void,
            nAtts as size_t,
            (::core::mem::size_of::<*mut XML_Char>()).wrapping_mul(2usize),
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
    fputs(name, fp);
    putc('>' as i32, fp);
}

unsafe extern "C" fn nsattcmp(
    mut p1: *const ::core::ffi::c_void,
    mut p2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut att1: *const XML_Char = *(p1 as *const *const XML_Char);
    let mut att2: *const XML_Char = *(p2 as *const *const XML_Char);
    let mut sep1: ::core::ffi::c_int = (::expat::stdlib::strrchr(att1, '\u{1}' as i32)
        != ::core::ptr::null_mut::<::core::ffi::c_char>())
        as ::core::ffi::c_int;
    let mut sep2: ::core::ffi::c_int = (::expat::stdlib::strrchr(att2, '\u{1}' as i32)
        != ::core::ptr::null_mut::<::core::ffi::c_char>())
        as ::core::ffi::c_int;
    if sep1 != sep2 {
        return sep1 - sep2;
    }
    return crate::stdlib::strcmp(att1, att2);
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
    sep = ::expat::stdlib::strrchr(name, '\u{1}' as i32);
    if !sep.is_null() {
        fputs(b"n1:\0" as *const u8 as *const ::core::ffi::c_char, fp);
        fputs(sep.offset(1), fp);
        fputs(
            b" xmlns:n1\0" as *const u8 as *const ::core::ffi::c_char,
            fp,
        );
        attributeValue(fp, name);
        nsi = 2i32;
    } else {
        fputs(name, fp);
        nsi = 1i32;
    }
    p = atts;
    while !(*p).is_null() {
        p = p.offset(1);
    }
    nAtts = (p.offset_from(atts) as ::core::ffi::c_long >> 1) as ::core::ffi::c_int;
    if nAtts > 1 {
        qsort(
            atts as *mut ::core::ffi::c_void,
            nAtts as size_t,
            (::core::mem::size_of::<*mut XML_Char>()).wrapping_mul(2usize),
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
        sep = ::expat::stdlib::strrchr(name, '\u{1}' as i32);
        putc(' ' as i32, fp);
        if !sep.is_null() {
            fprintf(
                fp,
                b"n%d:\0" as *const u8 as *const ::core::ffi::c_char,
                nsi,
            );
            fputs(sep.offset(1isize), fp);
        } else {
            fputs(name, fp);
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
    sep = ::expat::stdlib::strrchr(name, '\u{1}' as i32);
    if !sep.is_null() {
        fputs(b"n1:\0" as *const u8 as *const ::core::ffi::c_char, fp);
        fputs(sep.offset(1isize), fp);
    } else {
        fputs(name, fp);
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
    fputs(target, fp);
    putc(' ' as i32, fp);
    fputs(data, fp);
    putc('?' as i32, fp);
    putc('>' as i32, fp);
}

unsafe extern "C" fn xcsdup(mut s: *const XML_Char) -> *mut XML_Char {
    let mut result: *mut XML_Char = ::core::ptr::null_mut::<XML_Char>();
    let mut count: ::core::ffi::c_int = 0;
    let mut numBytes: size_t = 0;
    loop {
        let fresh3 = count;
        count = count + 1;
        if !(*s.offset(fresh3 as isize) as ::core::ffi::c_int != 0) {
            break;
        }
    }
    numBytes = (count as usize).wrapping_mul(::core::mem::size_of::<XML_Char>());
    result = malloc(numBytes) as *mut XML_Char;
    if result.is_null() {
        return ::core::ptr::null_mut::<XML_Char>();
    }
    ::expat::stdlib::memcpy(
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
        let mut next: *mut NotationList = (*notationListHead).next;
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
    while *xs as ::core::ffi::c_int != 0 && *xt as ::core::ffi::c_int != 0 {
        if (*xs as ::core::ffi::c_int) < *xt as ::core::ffi::c_int {
            return -(1i32);
        }
        if *xs as ::core::ffi::c_int > *xt as ::core::ffi::c_int {
            return 1i32;
        }
        xs = xs.offset(1);
        xt = xt.offset(1);
    }
    if (*xs as ::core::ffi::c_int) < *xt as ::core::ffi::c_int {
        return -(1i32);
    }
    if *xs as ::core::ffi::c_int > *xt as ::core::ffi::c_int {
        return 1i32;
    }
    return 0;
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
    let mut notationCount: ::core::ffi::c_int = 0;
    let mut p: *mut NotationList = ::core::ptr::null_mut::<NotationList>();
    let mut i: ::core::ffi::c_int = 0;
    p = (*data).notationListHead;
    while !p.is_null() {
        notationCount += 1;
        p = (*p).next;
    }
    if !(notationCount == 0) {
        notations = malloc(
            (notationCount as size_t).wrapping_mul(::core::mem::size_of::<*mut NotationList>()),
        ) as *mut *mut NotationList;
        if notations.is_null() {
            fprintf(
                stderr,
                b"Unable to sort notations\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            p = (*data).notationListHead;
            i = 0;
            while i < notationCount {
                let ref mut fresh4 = *notations.offset(i as isize);
                *fresh4 = p;
                p = (*p).next;
                i += 1;
            }
            qsort(
                notations as *mut ::core::ffi::c_void,
                notationCount as size_t,
                ::core::mem::size_of::<*mut NotationList>(),
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
            fputs((*data).currentDoctypeName, (*data).fp);
            fputs(
                b" [\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*data).fp,
            );
            i = 0;
            while i < notationCount {
                fputs(
                    b"<!NOTATION \0" as *const u8 as *const ::core::ffi::c_char,
                    (*data).fp,
                );
                fputs((**notations.offset(i as isize)).notationName, (*data).fp);
                if !(**notations.offset(i as isize)).publicId.is_null() {
                    fputs(
                        b" PUBLIC '\0" as *const u8 as *const ::core::ffi::c_char,
                        (*data).fp,
                    );
                    fputs((**notations.offset(i as isize)).publicId, (*data).fp);
                    putc('\'' as i32, (*data).fp);
                    if !(**notations.offset(i as isize)).systemId.is_null() {
                        putc(' ' as i32, (*data).fp);
                        putc('\'' as i32, (*data).fp);
                        fputs((**notations.offset(i as isize)).systemId, (*data).fp);
                        putc('\'' as i32, (*data).fp);
                    }
                } else if !(**notations.offset(i as isize)).systemId.is_null() {
                    fputs(
                        b" SYSTEM '\0" as *const u8 as *const ::core::ffi::c_char,
                        (*data).fp,
                    );
                    fputs((**notations.offset(i as isize)).systemId, (*data).fp);
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
        malloc(::core::mem::size_of::<NotationList>()) as *mut NotationList;
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
    (*entry).next = (*data).notationListHead;
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
        (*(*(userData as *mut *mut ::core::ffi::c_void) as *mut XmlwfUserData)).fp;
    while len > 0 {
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
        (*(*(userData as *mut *mut ::core::ffi::c_void) as *mut XmlwfUserData)).fp,
    );
}

unsafe extern "C" fn metaEndDocument(mut userData: *mut ::core::ffi::c_void) {
    fputs(
        b"</document>\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*(*(userData as *mut *mut ::core::ffi::c_void) as *mut XmlwfUserData)).fp,
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
    if idAttIndex < 0 {
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
                *atts.offset(0isize),
            );
            characterData(
                data as *mut ::core::ffi::c_void,
                *atts.offset(1),
                ::expat::stdlib::strlen(*atts.offset(1)) as ::core::ffi::c_int,
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
            atts = atts.offset(2);
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
        ::expat::stdlib::strlen(data) as ::core::ffi::c_int,
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
        ::expat::stdlib::strlen(data) as ::core::ffi::c_int,
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
            ::expat::stdlib::strlen(systemId) as ::core::ffi::c_int,
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
            ::expat::stdlib::strlen(systemId) as ::core::ffi::c_int,
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
            ::expat::stdlib::strlen(systemId) as ::core::ffi::c_int,
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
            ::expat::stdlib::strlen(uri) as ::core::ffi::c_int,
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
    return ::expat::src::xmlwf::codepage::codepageConvert(*(data as *mut ::core::ffi::c_int), p);
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
    i = 0;
    while prefixU[i as usize] != 0 {
        if *name.offset(i as isize) as ::core::ffi::c_int
            != prefixU[i as usize] as ::core::ffi::c_int
            && *name.offset(i as isize) as ::core::ffi::c_int
                != prefixL[i as usize] as ::core::ffi::c_int
        {
            return 0i32;
        }
        i += 1;
    }
    cp = 0;
    while *name.offset(i as isize) != 0 {
        static mut digits: [XML_Char; 11] =
            unsafe { ::core::mem::transmute::<[u8; 11], [XML_Char; 11]>(*b"0123456789\0") };
        let mut s: *const XML_Char = crate::stdlib::strchr(
            &raw const digits as *const ::core::ffi::c_char,
            *name.offset(i as isize) as ::core::ffi::c_int,
        );
        if s.is_null() {
            return 0i32;
        }
        cp *= 10;
        cp += s.offset_from(&raw const digits as *const XML_Char) as ::core::ffi::c_int;
        if cp >= 0x10000 {
            return 0i32;
        }
        i += 1;
    }
    if ::expat::src::xmlwf::codepage::codepageMap(
        cp,
        &raw mut (*info).map as *mut ::core::ffi::c_int,
    ) == 0
    {
        return 0i32;
    }
    (*info).convert = Some(
        unknownEncodingConvert
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
    );
    (*info).release = Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ());
    (*info).data = malloc(::core::mem::size_of::<::core::ffi::c_int>());
    if (*info).data.is_null() {
        return 0i32;
    }
    *((*info).data as *mut ::core::ffi::c_int) = cp;
    return 1;
}

unsafe extern "C" fn notStandalone(mut _userData: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    return 0;
}

unsafe extern "C" fn showVersion(mut prog: *mut XML_Char) {
    let mut s: *mut XML_Char = prog;
    let mut ch: XML_Char = 0;
    let mut features: *const XML_Feature = XML_GetFeatureList();
    loop {
        ch = *s;
        if !(ch as ::core::ffi::c_int != 0) {
            break;
        }
        if ch as ::core::ffi::c_int == '/' as i32 {
            prog = s.offset(1isize);
        }
        s = s.offset(1);
    }
    fprintf(
        stdout,
        b"%s using %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        prog,
        XML_ExpatVersion(),
    );
    if !features.is_null() && (*features.offset(0)).feature != XML_FEATURE_END {
        let mut i: ::core::ffi::c_int = 1;
        fprintf(
            stdout,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            (*features.offset(0isize)).name,
        );
        if (*features.offset(0)).value != 0 {
            fprintf(
                stdout,
                b"=%ld\0" as *const u8 as *const ::core::ffi::c_char,
                (*features.offset(0isize)).value,
            );
        }
        while (*features.offset(i as isize)).feature != XML_FEATURE_END {
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
    let mut windowsCodePages: ::core::ffi::c_int = 0;
    let mut outputType: ::core::ffi::c_int = 0;
    let mut useNamespaces: ::core::ffi::c_int = 0;
    let mut requireStandalone: ::core::ffi::c_int = 0;
    let mut requiresNotations: ::core::ffi::c_int = 0;
    let mut continueOnError: ::core::ffi::c_int = 0;
    let mut attackMaximumAmplification: ::core::ffi::c_float = -1.0;
    let mut attackThresholdBytes: ::core::ffi::c_ulonglong = 0;
    let mut attackThresholdGiven: XML_Bool = XML_FALSE;
    let mut disableDeferral: XML_Bool = XML_FALSE;
    let mut exitCode: ::core::ffi::c_int = XMLWF_EXIT_SUCCESS as ::core::ffi::c_int;
    let mut paramEntityParsing: XML_ParamEntityParsing = XML_PARAM_ENTITY_PARSING_NEVER;
    let mut useStdin: ::core::ffi::c_int = 0;
    let mut userData: XmlwfUserData = xmlwfUserData {
        fp: ::core::ptr::null_mut::<FILE>(),
        notationListHead: ::core::ptr::null_mut::<NotationList>(),
        currentDoctypeName: ::core::ptr::null::<XML_Char>(),
    };
    i = 1;
    j = 0;
    while i < argc {
        if j == 0 {
            if *(*argv.offset(i as isize)).offset(0) as ::core::ffi::c_int != '-' as i32 {
                break;
            }
            if *(*argv.offset(i as isize)).offset(1) as ::core::ffi::c_int == '-' as i32 {
                if *(*argv.offset(i as isize)).offset(2) as ::core::ffi::c_int == '\0' as i32 {
                    i += 1;
                    break;
                } else if crate::stdlib::strcmp(
                    (*argv.offset(i as isize)).offset(2),
                    b"help\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    usage(
                        *argv.offset(0isize),
                        XMLWF_EXIT_SUCCESS as ::core::ffi::c_int,
                    );
                } else if crate::stdlib::strcmp(
                    (*argv.offset(i as isize)).offset(2),
                    b"version\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    showVersion(*argv.offset(0));
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
                requireStandalone = 1;
                j += 1;
                current_block_122 = 8602574157404971894;
            }
            110 => {
                useNamespaces = 1;
                j += 1;
                current_block_122 = 8602574157404971894;
            }
            112 => {
                paramEntityParsing = XML_PARAM_ENTITY_PARSING_ALWAYS;
                current_block_122 = 12538682772167414182;
            }
            120 => {
                current_block_122 = 12538682772167414182;
            }
            119 => {
                windowsCodePages = 1;
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
                useNamespaces = 0;
                j += 1;
                current_block_122 = 8602574157404971894;
            }
            116 => {
                outputType = 't' as i32;
                j += 1;
                current_block_122 = 8602574157404971894;
            }
            78 => {
                requiresNotations = 1;
                j += 1;
                current_block_122 = 8602574157404971894;
            }
            100 => {
                if *(*argv.offset(i as isize)).offset((j + 1) as isize) as ::core::ffi::c_int
                    == '\0' as i32
                {
                    i += 1;
                    if i == argc {
                        usage(
                            *argv.offset(0isize),
                            XMLWF_EXIT_USAGE_ERROR as ::core::ffi::c_int,
                        );
                    }
                    outputDir = *argv.offset(i as isize);
                } else {
                    outputDir = (*argv.offset(i as isize)).offset(j as isize).offset(1isize);
                }
                i += 1;
                j = 0;
                current_block_122 = 8602574157404971894;
            }
            101 => {
                if *(*argv.offset(i as isize)).offset((j + 1) as isize) as ::core::ffi::c_int
                    == '\0' as i32
                {
                    i += 1;
                    if i == argc {
                        usage(
                            *argv.offset(0isize),
                            XMLWF_EXIT_USAGE_ERROR as ::core::ffi::c_int,
                        );
                    }
                    encoding = *argv.offset(i as isize);
                } else {
                    encoding = (*argv.offset(i as isize)).offset(j as isize).offset(1isize);
                }
                i += 1;
                j = 0;
                current_block_122 = 8602574157404971894;
            }
            104 => {
                usage(
                    *argv.offset(0isize),
                    XMLWF_EXIT_SUCCESS as ::core::ffi::c_int,
                );
            }
            118 => {
                showVersion(*argv.offset(0));
                return XMLWF_EXIT_SUCCESS as ::core::ffi::c_int;
            }
            103 => {
                let mut valueText: *const XML_Char = ::core::ptr::null::<XML_Char>();
                if *(*argv.offset(i as isize)).offset((j + 1) as isize) as ::core::ffi::c_int
                    == '\0' as i32
                {
                    i += 1;
                    if i == argc {
                        usage(
                            *argv.offset(0isize),
                            XMLWF_EXIT_USAGE_ERROR as ::core::ffi::c_int,
                        );
                    }
                    valueText = *argv.offset(i as isize);
                } else {
                    valueText = (*argv.offset(i as isize)).offset(j as isize).offset(1isize);
                }
                i += 1;
                j = 0;
                *::expat::stdlib::__errno_location() = 0;
                let mut afterValueText: *mut XML_Char = valueText as *mut XML_Char;
                let read_size_bytes_candidate: ::core::ffi::c_longlong =
                    strtoull(valueText, &raw mut afterValueText, 10) as ::core::ffi::c_longlong;
                if *::expat::stdlib::__errno_location() != 0
                    || *afterValueText.offset(0) as ::core::ffi::c_int != '\0' as i32
                    || read_size_bytes_candidate < 1
                    || read_size_bytes_candidate > (INT_MAX / 2 + 1) as ::core::ffi::c_longlong
                {
                    *::expat::stdlib::__errno_location() = ERANGE;
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
                continueOnError = 1;
                j += 1;
                current_block_122 = 8602574157404971894;
            }
            97 => {
                let mut valueText_0: *const XML_Char = ::core::ptr::null::<XML_Char>();
                if *(*argv.offset(i as isize)).offset((j + 1) as isize) as ::core::ffi::c_int
                    == '\0' as i32
                {
                    i += 1;
                    if i == argc {
                        usage(
                            *argv.offset(0isize),
                            XMLWF_EXIT_USAGE_ERROR as ::core::ffi::c_int,
                        );
                    }
                    valueText_0 = *argv.offset(i as isize);
                } else {
                    valueText_0 = (*argv.offset(i as isize)).offset(j as isize).offset(1isize);
                }
                i += 1;
                j = 0;
                *::expat::stdlib::__errno_location() = 0;
                let mut afterValueText_0: *mut XML_Char = ::core::ptr::null_mut::<XML_Char>();
                attackMaximumAmplification = strtof(valueText_0, &raw mut afterValueText_0);
                if *::expat::stdlib::__errno_location() != 0
                    || *afterValueText_0.offset(0) as ::core::ffi::c_int != '\0' as i32
                    || attackMaximumAmplification.is_nan() as i32 != 0
                    || attackMaximumAmplification < 1.0
                {
                    *::expat::stdlib::__errno_location() = ERANGE;
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
                if *(*argv.offset(i as isize)).offset((j + 1) as isize) as ::core::ffi::c_int
                    == '\0' as i32
                {
                    i += 1;
                    if i == argc {
                        usage(
                            *argv.offset(0isize),
                            XMLWF_EXIT_USAGE_ERROR as ::core::ffi::c_int,
                        );
                    }
                    valueText_1 = *argv.offset(i as isize);
                } else {
                    valueText_1 = (*argv.offset(i as isize)).offset(j as isize).offset(1isize);
                }
                i += 1;
                j = 0;
                *::expat::stdlib::__errno_location() = 0;
                let mut afterValueText_1: *mut XML_Char = valueText_1 as *mut XML_Char;
                attackThresholdBytes = strtoull(valueText_1, &raw mut afterValueText_1, 10);
                if *::expat::stdlib::__errno_location() != 0
                    || *afterValueText_1.offset(0) as ::core::ffi::c_int != '\0' as i32
                {
                    *::expat::stdlib::__errno_location() = ERANGE;
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
                if j > 1 {
                    i += 1;
                    j = 0;
                    current_block_122 = 8602574157404971894;
                } else {
                    current_block_122 = 15955764443707486316;
                }
            }
            _ => {
                current_block_122 = 15955764443707486316;
            }
        }
        match current_block_122 {
            12538682772167414182 => {
                processFlags |= XML_EXTERNAL_ENTITIES as ::core::ffi::c_uint;
                j += 1;
            }
            15955764443707486316 => {
                usage(
                    *argv.offset(0isize),
                    XMLWF_EXIT_USAGE_ERROR as ::core::ffi::c_int,
                );
            }
            _ => {}
        }
    }
    if i == argc {
        useStdin = 1;
        processFlags &= !XML_MAP_FILE as ::core::ffi::c_uint;
        i -= 1;
    }
    let mut current_block_219: u64;
    while i < argc {
        let mut outName: *mut XML_Char = ::core::ptr::null_mut::<XML_Char>();
        let mut result: ::core::ffi::c_int = 0;
        let mut parser: XML_Parser = ::core::ptr::null_mut::<XML_ParserStruct>();
        if useNamespaces != 0 {
            parser = XML_ParserCreateNS(encoding, '\u{1}' as XML_Char);
        } else {
            parser = XML_ParserCreate(encoding);
        }
        if parser.is_null() {
            perror(b"Could not instantiate parser\0" as *const u8 as *const ::core::ffi::c_char);
            exit(XMLWF_EXIT_INTERNAL_ERROR as ::core::ffi::c_int);
        }
        if attackMaximumAmplification != -1.0 {
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
            let success: XML_Bool = XML_SetReparseDeferralEnabled(parser, XML_FALSE);
            if success == 0 {
                *::expat::stdlib::__errno_location() = EINVAL;
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
                let mut lastDelim: *const XML_Char =
                    ::expat::stdlib::strrchr(file, *delim.offset(0) as ::core::ffi::c_int);
                if !lastDelim.is_null() {
                    file = lastDelim.offset(1isize);
                }
            }
            outName = malloc(
                ::expat::stdlib::strlen(outputDir)
                    .wrapping_add(::expat::stdlib::strlen(file))
                    .wrapping_add(2usize)
                    .wrapping_mul(::core::mem::size_of::<XML_Char>()),
            ) as *mut XML_Char;
            if outName.is_null() {
                perror(b"Could not allocate memory\0" as *const u8 as *const ::core::ffi::c_char);
                exit(XMLWF_EXIT_INTERNAL_ERROR as ::core::ffi::c_int);
            }
            ::expat::stdlib::strcpy(outName, outputDir);
            crate::stdlib::strcat(outName, delim);
            crate::stdlib::strcat(outName, file);
            userData.fp = fopen(outName, b"wb\0" as *const u8 as *const ::core::ffi::c_char);
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
                    16384,
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
                        ::core::mem::transmute(Some(
                            unknownEncoding
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *const XML_Char,
                                    *mut XML_Encoding,
                                )
                                    -> ::core::ffi::c_int,
                        )),
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
            args_ptrs.as_mut_ptr(),
        ))
    }
}
