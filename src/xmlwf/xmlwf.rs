#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(extern_types, raw_ref_op, register_tool)]
pub mod expat_h {
    use crate::expat_external_h::XML_Char;
    use core::ffi::c_int;
    use core::ffi::c_void;
    pub type XML_Parser = *mut libexpat::expat_h::XML_ParserStruct;

    pub type XML_Bool = core::ffi::c_uchar;

    pub type XML_StartElementHandler =
        Option<extern "C" fn(*mut c_void, *const XML_Char, *mut *const XML_Char) -> ()>;

    pub type XML_EndElementHandler = Option<extern "C" fn(*mut c_void, *const XML_Char) -> ()>;

    pub type XML_CharacterDataHandler =
        Option<extern "C" fn(*mut c_void, *const XML_Char, c_int) -> ()>;

    pub type XML_ProcessingInstructionHandler =
        Option<extern "C" fn(*mut c_void, *const XML_Char, *const XML_Char) -> ()>;

    pub type XML_CommentHandler = Option<extern "C" fn(*mut c_void, *const XML_Char) -> ()>;

    pub type XML_StartCdataSectionHandler = Option<extern "C" fn(*mut c_void) -> ()>;

    pub type XML_EndCdataSectionHandler = Option<extern "C" fn(*mut c_void) -> ()>;

    pub type XML_DefaultHandler = Option<extern "C" fn(*mut c_void, *const XML_Char, c_int) -> ()>;

    pub type XML_StartDoctypeDeclHandler = Option<
        extern "C" fn(*mut c_void, *const XML_Char, *const XML_Char, *const XML_Char, c_int) -> (),
    >;

    pub type XML_EndDoctypeDeclHandler = Option<extern "C" fn(*mut c_void) -> ()>;

    pub type XML_EntityDeclHandler = Option<
        extern "C" fn(
            *mut c_void,
            *const XML_Char,
            c_int,
            *const XML_Char,
            c_int,
            *const XML_Char,
            *const XML_Char,
            *const XML_Char,
            *const XML_Char,
        ) -> (),
    >;

    pub type XML_NotationDeclHandler = Option<
        extern "C" fn(
            *mut c_void,
            *const XML_Char,
            *const XML_Char,
            *const XML_Char,
            *const XML_Char,
        ) -> (),
    >;

    pub type XML_StartNamespaceDeclHandler =
        Option<extern "C" fn(*mut c_void, *const XML_Char, *const XML_Char) -> ()>;

    pub type XML_EndNamespaceDeclHandler =
        Option<extern "C" fn(*mut c_void, *const XML_Char) -> ()>;

    pub type XML_NotStandaloneHandler = Option<extern "C" fn(*mut c_void) -> c_int>;

    pub type XML_UnknownEncodingHandler = Option<
        extern "C" fn(*mut c_void, *const XML_Char, *mut libexpat::expat_h::XML_Encoding) -> c_int,
    >;

    pub type XML_ParamEntityParsing = core::ffi::c_uint;

    pub type XML_FeatureEnum = core::ffi::c_uint;
}
pub mod expat_external_h {
    use core::ffi::c_char;
    use core::ffi::c_long;
    pub type XML_Char = c_char;

    pub type XML_LChar = c_char;

    pub type XML_Index = c_long;

    pub type XML_Size = core::ffi::c_ulong;
}
pub mod __stddef_size_t_h {
    pub type size_t = usize;
}
pub mod xmlfile_h {
    use core::ffi::c_int;
    extern "C" {
        pub static mut g_read_size_bytes: c_int;
    }
}
pub mod stdlib {
    use crate::__stddef_size_t_h::size_t;
    use core::ffi::c_char;
    use core::ffi::c_int;
    use core::ffi::c_long;
    use core::ffi::c_void;
    extern "C" {
        pub static mut stdout: *mut FILE;

        pub static mut stderr: *mut FILE;

        pub fn remove(__filename: *const c_char) -> c_int;

        pub fn fclose(__stream: *mut FILE) -> c_int;

        pub fn fopen(__filename: *const c_char, __modes: *const c_char) -> *mut FILE;

        pub fn setvbuf(
            __stream: *mut FILE,
            __buf: *mut c_char,
            __modes: c_int,
            __n: size_t,
        ) -> c_int;

        pub fn putc(__c: c_int, __stream: *mut FILE) -> c_int;

        pub fn fputs(__s: *const c_char, __stream: *mut FILE) -> c_int;
        pub fn strtof(__nptr: *const c_char, __endptr: *mut *mut c_char) -> core::ffi::c_float;

        pub fn strtoull(
            __nptr: *const c_char,
            __endptr: *mut *mut c_char,
            __base: c_int,
        ) -> core::ffi::c_ulonglong;

        pub fn qsort(
            __base: *mut c_void,
            __nmemb: size_t,
            __size: size_t,
            __compar: crate::stdlib::__compar_fn_t,
        );
        pub fn strcat(__dest: *mut c_char, __src: *const c_char) -> *mut c_char;

        pub fn strcmp(__s1: *const c_char, __s2: *const c_char) -> c_int;

        pub fn strchr(__s: *const c_char, __c: c_int) -> *mut c_char;
        pub type _IO_marker;

        pub type _IO_codecvt;

        pub type _IO_wide_data;
    }
    pub type FILE = libexpat::stdlib::_IO_FILE;
    pub const __ASSERT_FUNCTION: [c_char; 46] =
        unsafe { core::mem::transmute::<[u8; 46], [c_char; 46]>(*b"void attributeValue(FILE *, const XML_Char *)\0") };
    pub const EINVAL: c_int = 22;

    pub const ERANGE: c_int = 34;
    pub const _IOFBF: c_int = 0;
    pub type __compar_fn_t = Option<extern "C" fn(*const c_void, *const c_void) -> c_int>;
    pub type _IO_lock_t = ();
    pub type __uint64_t = u64;

    pub type __off_t = c_long;

    pub type __off64_t = c_long;
}
#[allow(unused_imports)]
pub use crate::__stddef_size_t_h::size_t;

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
pub use libexpat::expat_h::XML_Encoding;
pub use libexpat::expat_h::XML_Feature;
pub use libexpat::expat_h::XML_ParserStruct;
pub use libexpat::expat_h::XML_FALSE;
pub use libexpat::expat_h::XML_FEATURE_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT;
pub use libexpat::expat_h::XML_FEATURE_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT;
pub use libexpat::expat_h::XML_FEATURE_ATTR_INFO;
pub use libexpat::expat_h::XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT;
pub use libexpat::expat_h::XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT;
pub use libexpat::expat_h::XML_FEATURE_CONTEXT_BYTES;
pub use libexpat::expat_h::XML_FEATURE_DTD;
pub use libexpat::expat_h::XML_FEATURE_END;
pub use libexpat::expat_h::XML_FEATURE_GE;
pub use libexpat::expat_h::XML_FEATURE_LARGE_SIZE;
pub use libexpat::expat_h::XML_FEATURE_MIN_SIZE;
pub use libexpat::expat_h::XML_FEATURE_NS;
pub use libexpat::expat_h::XML_FEATURE_SIZEOF_XML_CHAR;
pub use libexpat::expat_h::XML_FEATURE_SIZEOF_XML_LCHAR;
pub use libexpat::expat_h::XML_FEATURE_UNICODE;
pub use libexpat::expat_h::XML_FEATURE_UNICODE_WCHAR_T;
pub use libexpat::expat_h::XML_PARAM_ENTITY_PARSING_ALWAYS;
pub use libexpat::expat_h::XML_PARAM_ENTITY_PARSING_NEVER;
pub use libexpat::expat_h::XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE;
pub use libexpat::expat_h::XML_TRUE;
pub use libexpat::limits_h::INT_MAX;
pub use libexpat::src::lib::xmlparse::XML_DefaultCurrent;
pub use libexpat::src::lib::xmlparse::XML_ExpatVersion;
pub use libexpat::src::lib::xmlparse::XML_GetBase;
pub use libexpat::src::lib::xmlparse::XML_GetCurrentByteCount;
pub use libexpat::src::lib::xmlparse::XML_GetCurrentByteIndex;
pub use libexpat::src::lib::xmlparse::XML_GetCurrentColumnNumber;
pub use libexpat::src::lib::xmlparse::XML_GetCurrentLineNumber;
pub use libexpat::src::lib::xmlparse::XML_GetFeatureList;
pub use libexpat::src::lib::xmlparse::XML_GetIdAttributeIndex;
pub use libexpat::src::lib::xmlparse::XML_GetSpecifiedAttributeCount;
pub use libexpat::src::lib::xmlparse::XML_ParserCreate;
pub use libexpat::src::lib::xmlparse::XML_ParserCreateNS;
pub use libexpat::src::lib::xmlparse::XML_ParserFree;
pub use libexpat::src::lib::xmlparse::XML_SetAllocTrackerActivationThreshold;
pub use libexpat::src::lib::xmlparse::XML_SetAllocTrackerMaximumAmplification;
pub use libexpat::src::lib::xmlparse::XML_SetBillionLaughsAttackProtectionActivationThreshold;
pub use libexpat::src::lib::xmlparse::XML_SetBillionLaughsAttackProtectionMaximumAmplification;
pub use libexpat::src::lib::xmlparse::XML_SetCdataSectionHandler;
pub use libexpat::src::lib::xmlparse::XML_SetCharacterDataHandler;
pub use libexpat::src::lib::xmlparse::XML_SetCommentHandler;
pub use libexpat::src::lib::xmlparse::XML_SetDefaultHandler;
pub use libexpat::src::lib::xmlparse::XML_SetDoctypeDeclHandler;
pub use libexpat::src::lib::xmlparse::XML_SetElementHandler;
pub use libexpat::src::lib::xmlparse::XML_SetEntityDeclHandler;
pub use libexpat::src::lib::xmlparse::XML_SetNamespaceDeclHandler;
pub use libexpat::src::lib::xmlparse::XML_SetNotStandaloneHandler;
pub use libexpat::src::lib::xmlparse::XML_SetNotationDeclHandler;
pub use libexpat::src::lib::xmlparse::XML_SetParamEntityParsing;
pub use libexpat::src::lib::xmlparse::XML_SetProcessingInstructionHandler;
pub use libexpat::src::lib::xmlparse::XML_SetReparseDeferralEnabled;
pub use libexpat::src::lib::xmlparse::XML_SetUnknownEncodingHandler;
pub use libexpat::src::lib::xmlparse::XML_SetUserData;
pub use libexpat::src::lib::xmlparse::XML_UseParserAsHandlerArg;

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
pub use libexpat::__stddef_null_h::NULL;
pub use libexpat::internal::__INT_MAX__;
pub use libexpat::src::xmlwf::xmlfile::XML_ProcessFile;
pub use libexpat::src::xmlwf::xmlfile::XML_EXTERNAL_ENTITIES;
pub use libexpat::src::xmlwf::xmlfile::XML_MAP_FILE;
pub use libexpat::stdlib::exit;
pub use libexpat::stdlib::fprintf;
pub use libexpat::stdlib::free;
pub use libexpat::stdlib::malloc;

pub use libexpat::stdlib::perror;

use crate::stdlib::strcmp;
use core::ffi::{c_char, c_int, c_long, c_longlong, c_uint, c_void};
use core::mem::{size_of, transmute};
use core::ptr::{null, null_mut};
pub use libexpat::stdlib::_IO_FILE;
use libexpat::stdlib::{__errno_location, strlen, strrchr};

pub type ExitCode = c_uint;

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

#[inline]
fn xmlwf_user_data_from_parser(parser: XML_Parser) -> *mut XmlwfUserData {
    unsafe { *(parser as *mut *mut c_void) as *mut XmlwfUserData }
}

#[inline]
fn xmlwf_fp(data: *mut XmlwfUserData) -> *mut FILE {
    unsafe { (*data).fp }
}

#[inline]
fn xmlwf_fp_from_parser(parser: XML_Parser) -> *mut FILE {
    xmlwf_fp(xmlwf_user_data_from_parser(parser))
}

#[inline]
fn argv_at(argv: *mut *mut XML_Char, index: c_int) -> *mut XML_Char {
    unsafe { *argv.offset(index as isize) }
}

#[inline]
fn argv_char(argv: *mut *mut XML_Char, arg_index: c_int, char_index: c_int) -> c_int {
    unsafe { *argv_at(argv, arg_index).offset(char_index as isize) as c_int }
}

#[inline]
fn arg_offset(arg: *mut XML_Char, offset: c_int) -> *mut XML_Char {
    unsafe { arg.offset(offset as isize) }
}

#[inline]
fn xml_char_at(ptr: *const XML_Char, offset: c_int) -> c_int {
    unsafe { *ptr.offset(offset as isize) as c_int }
}

#[inline]
fn errno_value() -> c_int {
    unsafe { *__errno_location() }
}

#[inline]
fn set_errno_value(value: c_int) {
    unsafe { *__errno_location() = value }
}

#[inline]
fn c_strcmp(lhs: *const c_char, rhs: *const c_char) -> c_int {
    unsafe { strcmp(lhs, rhs) }
}

#[inline]
fn c_strtoull(
    nptr: *const c_char,
    endptr: *mut *mut c_char,
    base: c_int,
) -> core::ffi::c_ulonglong {
    unsafe { strtoull(nptr, endptr, base) }
}

#[inline]
fn c_strtof(nptr: *const c_char, endptr: *mut *mut c_char) -> core::ffi::c_float {
    unsafe { strtof(nptr, endptr) }
}

#[inline]
fn set_read_size_bytes(value: c_int) {
    unsafe { g_read_size_bytes = value }
}

#[inline]
fn c_perror(message: *const c_char) {
    unsafe { perror(message) }
}

#[inline]
fn c_exit(code: c_int) -> ! {
    unsafe { exit(code) }
}

#[inline]
fn c_strrchr(s: *const c_char, ch: c_int) -> *mut c_char {
    unsafe { strrchr(s, ch) }
}

#[inline]
fn c_strlen(s: *const c_char) -> size_t {
    unsafe { strlen(s) }
}

#[inline]
fn c_malloc(size: size_t) -> *mut c_void {
    unsafe { malloc(size) }
}

#[inline]
fn c_strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char {
    unsafe { libexpat::stdlib::strcpy(dest, src) }
}

#[inline]
fn c_strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char {
    unsafe { crate::stdlib::strcat(dest, src) }
}

#[inline]
fn c_fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE {
    unsafe { fopen(filename, mode) }
}

#[inline]
fn c_setvbuf(stream: *mut FILE, buf: *mut c_char, mode: c_int, size: size_t) -> c_int {
    unsafe { setvbuf(stream, buf, mode, size) }
}

#[inline]
fn c_fclose(stream: *mut FILE) -> c_int {
    unsafe { fclose(stream) }
}

#[inline]
fn c_remove(filename: *const c_char) -> c_int {
    unsafe { remove(filename) }
}

#[inline]
fn c_free(ptr: *mut c_void) {
    unsafe { free(ptr) }
}

#[inline]
fn c_char_ptr_offset(ptr: *const c_char, offset: isize) -> *const c_char {
    unsafe { ptr.offset(offset) }
}

#[inline]
fn unknown_encoding_handler() -> XML_UnknownEncodingHandler {
    unsafe { transmute(Some(unknownEncoding as extern "C" fn(*mut c_void, *const XML_Char, *mut XML_Encoding) -> c_int)) }
}

extern "C" fn characterData(mut userData: *mut c_void, mut s: *const XML_Char, mut len: c_int) {
    let mut fp: *mut FILE = unsafe { (*(userData as *mut XmlwfUserData)).fp };
    while len > 0 {
        let ch = unsafe { *s as c_int };
        match ch {
            38 => {
                unsafe { fputs(b"&amp;\0" as *const u8 as *const c_char, fp) };
            }
            60 => {
                unsafe { fputs(b"&lt;\0" as *const u8 as *const c_char, fp) };
            }
            62 => {
                unsafe { fputs(b"&gt;\0" as *const u8 as *const c_char, fp) };
            }
            34 => {
                unsafe { fputs(b"&quot;\0" as *const u8 as *const c_char, fp) };
            }
            9 | 10 | 13 => {
                unsafe { fprintf(fp, b"&#%d;\0" as *const u8 as *const c_char, ch) };
            }
            _ => {
                unsafe { putc(ch, fp) };
            }
        }
        len -= 1;
        s = unsafe { s.offset(1) };
    }
}

extern "C" fn attributeValue(mut fp: *mut FILE, mut s: *const XML_Char) {
    unsafe { putc('=' as i32, fp) };
    unsafe { putc('"' as i32, fp) };
    assert!(!s.is_null());
    loop {
        let ch = unsafe { *s as c_int };
        match ch {
            0 | 1 => {
                unsafe { putc('"' as i32, fp) };
                return;
            }
            38 => {
                unsafe { fputs(b"&amp;\0" as *const u8 as *const c_char, fp) };
            }
            60 => {
                unsafe { fputs(b"&lt;\0" as *const u8 as *const c_char, fp) };
            }
            34 => {
                unsafe { fputs(b"&quot;\0" as *const u8 as *const c_char, fp) };
            }
            62 => {
                unsafe { fputs(b"&gt;\0" as *const u8 as *const c_char, fp) };
            }
            9 | 10 | 13 => {
                unsafe { fprintf(fp, b"&#%d;\0" as *const u8 as *const c_char, ch) };
            }
            _ => {
                unsafe { putc(ch, fp) };
            }
        }
        s = unsafe { s.offset(1) };
    }
}

extern "C" fn attcmp(mut att1: *const c_void, mut att2: *const c_void) -> c_int {
    let lhs = unsafe { *(att1 as *const *const XML_Char) };
    let rhs = unsafe { *(att2 as *const *const XML_Char) };
    unsafe { strcmp(lhs, rhs) }
}

extern "C" fn startElement(
    mut userData: *mut c_void,
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    let mut nAtts: c_int = 0;
    let mut p: *mut *const XML_Char = null_mut::<*const XML_Char>();
    let mut fp: *mut FILE = unsafe { (*(userData as *mut XmlwfUserData)).fp };
    unsafe { putc('<' as i32, fp) };
    unsafe { fputs(name, fp) };
    p = atts;
    while !unsafe { (*p).is_null() } {
        p = unsafe { p.offset(1) };
    }
    nAtts = (unsafe { p.offset_from(atts) } as c_long >> 1) as c_int;
    if nAtts > 1 {
        let att_pair_size = (size_of::<*mut XML_Char>()).wrapping_mul(2usize);
        let cmp = Some(attcmp as extern "C" fn(*const c_void, *const c_void) -> c_int);
        unsafe { qsort(atts as *mut c_void, nAtts as size_t, att_pair_size, cmp) };
    }
    while !unsafe { (*atts).is_null() } {
        unsafe { putc(' ' as i32, fp) };
        let fresh0 = atts;
        atts = unsafe { atts.offset(1) };
        unsafe { fputs(*fresh0, fp) };
        attributeValue(fp, unsafe { *atts });
        atts = unsafe { atts.offset(1) };
    }
    unsafe { putc('>' as i32, fp) };
}

extern "C" fn endElement(mut userData: *mut c_void, mut name: *const XML_Char) {
    let mut fp: *mut FILE = unsafe { (*(userData as *mut XmlwfUserData)).fp };
    unsafe { putc('<' as i32, fp) };
    unsafe { putc('/' as i32, fp) };
    unsafe { fputs(name, fp) };
    unsafe { putc('>' as i32, fp) };
}

extern "C" fn nsattcmp(mut p1: *const c_void, mut p2: *const c_void) -> c_int {
    let mut att1: *const XML_Char = unsafe { *(p1 as *const *const XML_Char) };
    let mut att2: *const XML_Char = unsafe { *(p2 as *const *const XML_Char) };
    let mut sep1: c_int = (!unsafe { strrchr(att1, '\u{1}' as i32) }.is_null()) as c_int;
    let mut sep2: c_int = (!unsafe { strrchr(att2, '\u{1}' as i32) }.is_null()) as c_int;
    if sep1 != sep2 {
        return sep1 - sep2;
    }
    unsafe { strcmp(att1, att2) }
}

extern "C" fn startElementNS(
    mut userData: *mut c_void,
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    let mut nAtts: c_int = 0;
    let mut nsi: c_int = 0;
    let mut p: *mut *const XML_Char = null_mut::<*const XML_Char>();
    let mut fp: *mut FILE = unsafe { (*(userData as *mut XmlwfUserData)).fp };
    let mut sep: *const XML_Char = null::<XML_Char>();
    unsafe { putc('<' as i32, fp) };
    sep = unsafe { strrchr(name, '\u{1}' as i32) };
    if !sep.is_null() {
        unsafe { fputs(b"n1:\0" as *const u8 as *const c_char, fp) };
        let local_name = unsafe { sep.offset(1) };
        unsafe { fputs(local_name, fp) };
        unsafe { fputs(b" xmlns:n1\0" as *const u8 as *const c_char, fp) };
        attributeValue(fp, name);
        nsi = 2i32;
    } else {
        unsafe { fputs(name, fp) };
        nsi = 1i32;
    }
    p = atts;
    while !unsafe { (*p).is_null() } {
        p = unsafe { p.offset(1) };
    }
    nAtts = (unsafe { p.offset_from(atts) } as c_long >> 1) as c_int;
    if nAtts > 1 {
        let att_pair_size = (size_of::<*mut XML_Char>()).wrapping_mul(2usize);
        let cmp = Some(nsattcmp as extern "C" fn(*const c_void, *const c_void) -> c_int);
        unsafe { qsort(atts as *mut c_void, nAtts as size_t, att_pair_size, cmp) };
    }
    while !unsafe { (*atts).is_null() } {
        let fresh1 = atts;
        atts = unsafe { atts.offset(1) };
        name = unsafe { *fresh1 };
        sep = unsafe { strrchr(name, '\u{1}' as i32) };
        unsafe { putc(' ' as i32, fp) };
        if !sep.is_null() {
            unsafe { fprintf(fp, b"n%d:\0" as *const u8 as *const c_char, nsi) };
            let attr_local_name = unsafe { sep.offset(1isize) };
            unsafe { fputs(attr_local_name, fp) };
        } else {
            unsafe { fputs(name, fp) };
        }
        attributeValue(fp, unsafe { *atts });
        if !sep.is_null() {
            let fresh2 = nsi;
            nsi += 1;
            unsafe { fprintf(fp, b" xmlns:n%d\0" as *const u8 as *const c_char, fresh2) };
            attributeValue(fp, name);
        }
        atts = unsafe { atts.offset(1) };
    }
    unsafe { putc('>' as i32, fp) };
}

extern "C" fn endElementNS(mut userData: *mut c_void, mut name: *const XML_Char) {
    let mut fp: *mut FILE = unsafe { (*(userData as *mut XmlwfUserData)).fp };
    let mut sep: *const XML_Char = null::<XML_Char>();
    unsafe { putc('<' as i32, fp) };
    unsafe { putc('/' as i32, fp) };
    sep = unsafe { strrchr(name, '\u{1}' as i32) };
    if !sep.is_null() {
        unsafe { fputs(b"n1:\0" as *const u8 as *const c_char, fp) };
        unsafe { fputs(sep.offset(1isize), fp) };
    } else {
        unsafe { fputs(name, fp) };
    }
    unsafe { putc('>' as i32, fp) };
}

extern "C" fn processingInstruction(
    mut userData: *mut c_void,
    mut target: *const XML_Char,
    mut data: *const XML_Char,
) {
    let mut fp: *mut FILE = unsafe { (*(userData as *mut XmlwfUserData)).fp };
    unsafe { putc('<' as i32, fp) };
    unsafe { putc('?' as i32, fp) };
    unsafe { fputs(target, fp) };
    unsafe { putc(' ' as i32, fp) };
    unsafe { fputs(data, fp) };
    unsafe { putc('?' as i32, fp) };
    unsafe { putc('>' as i32, fp) };
}

extern "C" fn xcsdup(mut s: *const XML_Char) -> *mut XML_Char {
    let mut result: *mut XML_Char = null_mut::<XML_Char>();
    let mut count: c_int = 0;
    let mut numBytes: size_t = 0;
    loop {
        let fresh3 = count;
        count += 1;
        if unsafe { *s.offset(fresh3 as isize) as c_int } == 0 {
            break;
        }
    }
    numBytes = (count as usize).wrapping_mul(size_of::<XML_Char>());
    result = unsafe { malloc(numBytes) as *mut XML_Char };
    if result.is_null() {
        return null_mut::<XML_Char>();
    }
    unsafe { libexpat::stdlib::memcpy(result as *mut c_void, s as *const c_void, numBytes) };
    result
}

extern "C" fn startDoctypeDecl(
    mut userData: *mut c_void,
    mut doctypeName: *const XML_Char,
    mut _sysid: *const XML_Char,
    mut _publid: *const XML_Char,
    mut _has_internal_subset: c_int,
) {
    let mut data: *mut XmlwfUserData = userData as *mut XmlwfUserData;
    unsafe { (*data).currentDoctypeName = xcsdup(doctypeName) };
}

extern "C" fn freeNotations(mut data: *mut XmlwfUserData) {
    let mut notationListHead: *mut NotationList = unsafe { (*data).notationListHead };
    while !notationListHead.is_null() {
        let mut next: *mut NotationList = unsafe { (*notationListHead).next };
        unsafe { free((*notationListHead).notationName as *mut c_void) };
        unsafe { free((*notationListHead).systemId as *mut c_void) };
        unsafe { free((*notationListHead).publicId as *mut c_void) };
        unsafe { free(notationListHead as *mut c_void) };
        notationListHead = next;
    }
    unsafe { (*data).notationListHead = null_mut::<NotationList>() };
}

extern "C" fn cleanupUserData(mut userData: *mut XmlwfUserData) {
    unsafe { free((*userData).currentDoctypeName as *mut c_void) };
    unsafe { (*userData).currentDoctypeName = null::<XML_Char>() };
    freeNotations(userData);
}

extern "C" fn xcscmp(mut xs: *const XML_Char, mut xt: *const XML_Char) -> c_int {
    while unsafe { *xs as c_int } != 0 && unsafe { *xt as c_int } != 0 {
        if (unsafe { *xs as c_int }) < unsafe { *xt as c_int } {
            return -(1i32);
        }
        if unsafe { *xs as c_int } > unsafe { *xt as c_int } {
            return 1i32;
        }
        xs = unsafe { xs.offset(1) };
        xt = unsafe { xt.offset(1) };
    }
    if (unsafe { *xs as c_int }) < unsafe { *xt as c_int } {
        return -(1i32);
    }
    if unsafe { *xs as c_int } > unsafe { *xt as c_int } {
        return 1i32;
    }
    0
}

extern "C" fn notationCmp(mut a: *const c_void, mut b: *const c_void) -> c_int {
    let n1: *const NotationList = unsafe { *(a as *const *const NotationList) };
    let n2: *const NotationList = unsafe { *(b as *const *const NotationList) };
    xcscmp(unsafe { (*n1).notationName }, unsafe { (*n2).notationName })
}

extern "C" fn endDoctypeDecl(mut userData: *mut c_void) {
    let mut data: *mut XmlwfUserData = userData as *mut XmlwfUserData;
    let mut notations: *mut *mut NotationList = null_mut::<*mut NotationList>();
    let mut notationCount: c_int = 0;
    let mut p: *mut NotationList = unsafe { (*data).notationListHead };
    let mut i: c_int = 0;
    while !p.is_null() {
        notationCount += 1;
        p = unsafe { (*p).next };
    }
    if notationCount != 0 {
        let notations_size = (notationCount as size_t).wrapping_mul(size_of::<*mut NotationList>());
        notations = unsafe { malloc(notations_size) as *mut *mut NotationList };
        if notations.is_null() {
            let sort_error_fmt = b"Unable to sort notations\0" as *const u8 as *const c_char;
            unsafe { fprintf(stderr, sort_error_fmt) };
        } else {
            let fp = unsafe { (*data).fp };
            p = unsafe { (*data).notationListHead };
            i = 0;
            while i < notationCount {
                unsafe { *notations.offset(i as isize) = p };
                p = unsafe { (*p).next };
                i += 1;
            }
            let cmp = Some(notationCmp as extern "C" fn(*const c_void, *const c_void) -> c_int);
            unsafe { qsort(notations as *mut c_void, notationCount as size_t, size_of::<*mut NotationList>(), cmp) };
            unsafe { fputs(b"<!DOCTYPE \0" as *const u8 as *const c_char, fp) };
            unsafe { fputs((*data).currentDoctypeName, fp) };
            unsafe { fputs(b" [\n\0" as *const u8 as *const c_char, fp) };
            i = 0;
            while i < notationCount {
                let notation = unsafe { *notations.offset(i as isize) };
                unsafe { fputs(b"<!NOTATION \0" as *const u8 as *const c_char, fp) };
                unsafe { fputs((*notation).notationName, fp) };
                if !unsafe { (*notation).publicId.is_null() } {
                    unsafe { fputs(b" PUBLIC '\0" as *const u8 as *const c_char, fp) };
                    unsafe { fputs((*notation).publicId, fp) };
                    unsafe { putc('\'' as i32, fp) };
                    if !unsafe { (*notation).systemId.is_null() } {
                        unsafe { putc(' ' as i32, fp) };
                        unsafe { putc('\'' as i32, fp) };
                        unsafe { fputs((*notation).systemId, fp) };
                        unsafe { putc('\'' as i32, fp) };
                    }
                } else if !unsafe { (*notation).systemId.is_null() } {
                    unsafe { fputs(b" SYSTEM '\0" as *const u8 as *const c_char, fp) };
                    unsafe { fputs((*notation).systemId, fp) };
                    unsafe { putc('\'' as i32, fp) };
                }
                unsafe { putc('>' as i32, fp) };
                unsafe { putc('\n' as i32, fp) };
                i += 1;
            }
            unsafe { fputs(b"]>\n\0" as *const u8 as *const c_char, fp) };
            unsafe { free(notations as *mut c_void) };
        }
    }
    freeNotations(data);
    unsafe { free((*data).currentDoctypeName as *mut c_void) };
    unsafe { (*data).currentDoctypeName = null::<XML_Char>() };
}

extern "C" fn notationDecl(
    mut userData: *mut c_void,
    mut notationName: *const XML_Char,
    mut _base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) {
    let mut data: *mut XmlwfUserData = userData as *mut XmlwfUserData;
    let mut entry: *mut NotationList =
        unsafe { malloc(size_of::<NotationList>()) as *mut NotationList };
    let mut errorMessage: *const c_char =
        b"Unable to store NOTATION for output\n\0" as *const u8 as *const c_char;
    if entry.is_null() {
        unsafe { fputs(errorMessage, stderr) };
        return;
    }
    unsafe { (*entry).notationName = xcsdup(notationName) };
    if unsafe { (*entry).notationName.is_null() } {
        unsafe { fputs(errorMessage, stderr) };
        unsafe { free(entry as *mut c_void) };
        return;
    }
    if !systemId.is_null() {
        unsafe { (*entry).systemId = xcsdup(systemId) };
        if unsafe { (*entry).systemId.is_null() } {
            unsafe { fputs(errorMessage, stderr) };
            unsafe { free((*entry).notationName as *mut c_void) };
            unsafe { free(entry as *mut c_void) };
            return;
        }
    } else {
        unsafe { (*entry).systemId = null::<XML_Char>() };
    }
    if !publicId.is_null() {
        unsafe { (*entry).publicId = xcsdup(publicId) };
        if unsafe { (*entry).publicId.is_null() } {
            unsafe { fputs(errorMessage, stderr) };
            unsafe { free((*entry).systemId as *mut c_void) };
            unsafe { free((*entry).notationName as *mut c_void) };
            unsafe { free(entry as *mut c_void) };
            return;
        }
    } else {
        unsafe { (*entry).publicId = null::<XML_Char>() };
    }
    unsafe { (*entry).next = (*data).notationListHead };
    unsafe { (*data).notationListHead = entry };
}

extern "C" fn defaultCharacterData(
    mut userData: *mut c_void,
    mut _s: *const XML_Char,
    mut _len: c_int,
) {
    XML_DefaultCurrent(userData as XML_Parser);
}

extern "C" fn defaultStartElement(
    mut userData: *mut c_void,
    mut _name: *const XML_Char,
    mut _atts: *mut *const XML_Char,
) {
    XML_DefaultCurrent(userData as XML_Parser);
}

extern "C" fn defaultEndElement(mut userData: *mut c_void, mut _name: *const XML_Char) {
    XML_DefaultCurrent(userData as XML_Parser);
}

extern "C" fn defaultProcessingInstruction(
    mut userData: *mut c_void,
    mut _target: *const XML_Char,
    mut _data: *const XML_Char,
) {
    XML_DefaultCurrent(userData as XML_Parser);
}

extern "C" fn nopCharacterData(
    mut _userData: *mut c_void,
    mut _s: *const XML_Char,
    mut _len: c_int,
) {
}

extern "C" fn nopStartElement(
    mut _userData: *mut c_void,
    mut _name: *const XML_Char,
    mut _atts: *mut *const XML_Char,
) {
}

extern "C" fn nopEndElement(mut _userData: *mut c_void, mut _name: *const XML_Char) {}

extern "C" fn nopProcessingInstruction(
    mut _userData: *mut c_void,
    mut _target: *const XML_Char,
    mut _data: *const XML_Char,
) {
}

extern "C" fn markup(mut userData: *mut c_void, mut s: *const XML_Char, mut len: c_int) {
    let mut fp: *mut FILE = xmlwf_fp_from_parser(userData as XML_Parser);
    while len > 0 {
        unsafe { putc(*s as c_int, fp) };
        len -= 1;
        s = unsafe { s.offset(1) };
    }
}

extern "C" fn metaLocation(mut parser: XML_Parser) {
    let mut uri: *const XML_Char = XML_GetBase(parser);
    let mut fp: *mut FILE = xmlwf_fp_from_parser(parser);
    if !uri.is_null() {
        unsafe { fprintf(fp, b" uri=\"%s\"\0" as *const u8 as *const c_char, uri) };
    }
    let location_fmt =
        b" byte=\"%ld\" nbytes=\"%d\" line=\"%lu\" col=\"%lu\"\0" as *const u8 as *const c_char;
    let current_byte_index = XML_GetCurrentByteIndex(parser);
    let current_byte_count = XML_GetCurrentByteCount(parser);
    let current_line = XML_GetCurrentLineNumber(parser);
    let current_col = XML_GetCurrentColumnNumber(parser);
    unsafe { fprintf(fp, location_fmt, current_byte_index, current_byte_count, current_line, current_col) };
}

extern "C" fn metaStartDocument(mut userData: *mut c_void) {
    let fp = xmlwf_fp_from_parser(userData as XML_Parser);
    unsafe { fputs(b"<document>\n\0" as *const u8 as *const c_char, fp) };
}

extern "C" fn metaEndDocument(mut userData: *mut c_void) {
    let fp = xmlwf_fp_from_parser(userData as XML_Parser);
    unsafe { fputs(b"</document>\n\0" as *const u8 as *const c_char, fp) };
}

extern "C" fn metaStartElement(
    mut userData: *mut c_void,
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData = xmlwf_user_data_from_parser(parser);
    let mut fp: *mut FILE = xmlwf_fp(data);
    let mut specifiedAttsEnd: *mut *const XML_Char =
        unsafe { atts.offset(XML_GetSpecifiedAttributeCount(parser) as isize) };
    let mut idAttPtr: *mut *const XML_Char = null_mut::<*const XML_Char>();
    let mut idAttIndex: c_int = XML_GetIdAttributeIndex(parser);
    if idAttIndex < 0 {
        idAttPtr = null_mut::<*const XML_Char>();
    } else {
        idAttPtr = unsafe { atts.offset(idAttIndex as isize) };
    }
    let starttag_fmt = b"<starttag name=\"%s\"\0" as *const u8 as *const c_char;
    unsafe { fprintf(fp, starttag_fmt, name) };
    metaLocation(parser);
    if !unsafe { (*atts).is_null() } {
        unsafe { fputs(b">\n\0" as *const u8 as *const c_char, fp) };
        loop {
            let att_name = unsafe { *atts.offset(0isize) };
            let att_value = unsafe { *atts.offset(1) };
            let attribute_fmt = b"<attribute name=\"%s\" value=\"\0" as *const u8 as *const c_char;
            unsafe { fprintf(fp, attribute_fmt, att_name) };
            characterData(data as *mut c_void, att_value, unsafe { strlen(att_value) }
                as c_int);
            if atts >= specifiedAttsEnd {
                unsafe { fputs(b"\" defaulted=\"yes\"/>\n\0" as *const u8 as *const c_char, fp) };
            } else if atts == idAttPtr {
                unsafe { fputs(b"\" id=\"yes\"/>\n\0" as *const u8 as *const c_char, fp) };
            } else {
                unsafe { fputs(b"\"/>\n\0" as *const u8 as *const c_char, fp) };
            }
            atts = unsafe { atts.offset(2) };
            if unsafe { (*atts).is_null() } {
                break;
            }
        }
        unsafe { fputs(b"</starttag>\n\0" as *const u8 as *const c_char, fp) };
    } else {
        unsafe { fputs(b"/>\n\0" as *const u8 as *const c_char, fp) };
    };
}

extern "C" fn metaEndElement(mut userData: *mut c_void, mut name: *const XML_Char) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut fp: *mut FILE = xmlwf_fp_from_parser(parser);
    let endtag_fmt = b"<endtag name=\"%s\"\0" as *const u8 as *const c_char;
    unsafe { fprintf(fp, endtag_fmt, name) };
    metaLocation(parser);
    unsafe { fputs(b"/>\n\0" as *const u8 as *const c_char, fp) };
}

extern "C" fn metaProcessingInstruction(
    mut userData: *mut c_void,
    mut target: *const XML_Char,
    mut data: *const XML_Char,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut usrData: *mut XmlwfUserData = xmlwf_user_data_from_parser(parser);
    let mut fp: *mut FILE = xmlwf_fp(usrData);
    let pi_fmt = b"<pi target=\"%s\" data=\"\0" as *const u8 as *const c_char;
    unsafe { fprintf(fp, pi_fmt, target) };
    characterData(usrData as *mut c_void, data, unsafe { strlen(data) }
        as c_int);
    unsafe { putc('"' as i32, fp) };
    metaLocation(parser);
    unsafe { fputs(b"/>\n\0" as *const u8 as *const c_char, fp) };
}

extern "C" fn metaComment(mut userData: *mut c_void, mut data: *const XML_Char) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut usrData: *mut XmlwfUserData = xmlwf_user_data_from_parser(parser);
    let mut fp: *mut FILE = xmlwf_fp(usrData);
    unsafe { fputs(b"<comment data=\"\0" as *const u8 as *const c_char, fp) };
    characterData(usrData as *mut c_void, data, unsafe { strlen(data) }
        as c_int);
    unsafe { putc('"' as i32, fp) };
    metaLocation(parser);
    unsafe { fputs(b"/>\n\0" as *const u8 as *const c_char, fp) };
}

extern "C" fn metaStartCdataSection(mut userData: *mut c_void) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut fp: *mut FILE = xmlwf_fp_from_parser(parser);
    unsafe { fputs(b"<startcdata\0" as *const u8 as *const c_char, fp) };
    metaLocation(parser);
    unsafe { fputs(b"/>\n\0" as *const u8 as *const c_char, fp) };
}

extern "C" fn metaEndCdataSection(mut userData: *mut c_void) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut fp: *mut FILE = xmlwf_fp_from_parser(parser);
    unsafe { fputs(b"<endcdata\0" as *const u8 as *const c_char, fp) };
    metaLocation(parser);
    unsafe { fputs(b"/>\n\0" as *const u8 as *const c_char, fp) };
}

extern "C" fn metaCharacterData(mut userData: *mut c_void, mut s: *const XML_Char, mut len: c_int) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData = xmlwf_user_data_from_parser(parser);
    let mut fp: *mut FILE = xmlwf_fp(data);
    unsafe { fputs(b"<chars str=\"\0" as *const u8 as *const c_char, fp) };
    characterData(data as *mut c_void, s, len);
    unsafe { putc('"' as i32, fp) };
    metaLocation(parser);
    unsafe { fputs(b"/>\n\0" as *const u8 as *const c_char, fp) };
}

extern "C" fn metaStartDoctypeDecl(
    mut userData: *mut c_void,
    mut doctypeName: *const XML_Char,
    mut _sysid: *const XML_Char,
    mut _pubid: *const XML_Char,
    mut _has_internal_subset: c_int,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut fp: *mut FILE = xmlwf_fp_from_parser(parser);
    let start_doctype_fmt = b"<startdoctype name=\"%s\"\0" as *const u8 as *const c_char;
    unsafe { fprintf(fp, start_doctype_fmt, doctypeName) };
    metaLocation(parser);
    unsafe { fputs(b"/>\n\0" as *const u8 as *const c_char, fp) };
}

extern "C" fn metaEndDoctypeDecl(mut userData: *mut c_void) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut fp: *mut FILE = xmlwf_fp_from_parser(parser);
    unsafe { fputs(b"<enddoctype\0" as *const u8 as *const c_char, fp) };
    metaLocation(parser);
    unsafe { fputs(b"/>\n\0" as *const u8 as *const c_char, fp) };
}

extern "C" fn metaNotationDecl(
    mut userData: *mut c_void,
    mut notationName: *const XML_Char,
    mut _base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData = xmlwf_user_data_from_parser(parser);
    let mut fp: *mut FILE = xmlwf_fp(data);
    let notation_fmt = b"<notation name=\"%s\"\0" as *const u8 as *const c_char;
    unsafe { fprintf(fp, notation_fmt, notationName) };
    if !publicId.is_null() {
        let public_fmt = b" public=\"%s\"\0" as *const u8 as *const c_char;
        unsafe { fprintf(fp, public_fmt, publicId) };
    }
    if !systemId.is_null() {
        unsafe { fputs(b" system=\"\0" as *const u8 as *const c_char, fp) };
        characterData(data as *mut c_void, systemId, unsafe { strlen(systemId) }
            as c_int);
        unsafe { putc('"' as i32, fp) };
    }
    metaLocation(parser);
    unsafe { fputs(b"/>\n\0" as *const u8 as *const c_char, fp) };
}

extern "C" fn metaEntityDecl(
    mut userData: *mut c_void,
    mut entityName: *const XML_Char,
    mut _is_param: c_int,
    mut value: *const XML_Char,
    mut value_length: c_int,
    mut _base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
    mut notationName: *const XML_Char,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData = xmlwf_user_data_from_parser(parser);
    let mut fp: *mut FILE = xmlwf_fp(data);
    let entity_fmt = b"<entity name=\"%s\"\0" as *const u8 as *const c_char;
    if !value.is_null() {
        unsafe { fprintf(fp, entity_fmt, entityName) };
        metaLocation(parser);
        unsafe { putc('>' as i32, fp) };
        characterData(data as *mut c_void, value, value_length);
        unsafe { fputs(b"</entity/>\n\0" as *const u8 as *const c_char, fp) };
    } else if !notationName.is_null() {
        unsafe { fprintf(fp, entity_fmt, entityName) };
        if !publicId.is_null() {
            let public_fmt = b" public=\"%s\"\0" as *const u8 as *const c_char;
            unsafe { fprintf(fp, public_fmt, publicId) };
        }
        unsafe { fputs(b" system=\"\0" as *const u8 as *const c_char, fp) };
        characterData(data as *mut c_void, systemId, unsafe { strlen(systemId) }
            as c_int);
        unsafe { putc('"' as i32, fp) };
        let notation_attr_fmt = b" notation=\"%s\"\0" as *const u8 as *const c_char;
        unsafe { fprintf(fp, notation_attr_fmt, notationName) };
        metaLocation(parser);
        unsafe { fputs(b"/>\n\0" as *const u8 as *const c_char, fp) };
    } else {
        unsafe { fprintf(fp, entity_fmt, entityName) };
        if !publicId.is_null() {
            let public_fmt = b" public=\"%s\"\0" as *const u8 as *const c_char;
            unsafe { fprintf(fp, public_fmt, publicId) };
        }
        unsafe { fputs(b" system=\"\0" as *const u8 as *const c_char, fp) };
        characterData(data as *mut c_void, systemId, unsafe { strlen(systemId) }
            as c_int);
        unsafe { putc('"' as i32, fp) };
        metaLocation(parser);
        unsafe { fputs(b"/>\n\0" as *const u8 as *const c_char, fp) };
    };
}

extern "C" fn metaStartNamespaceDecl(
    mut userData: *mut c_void,
    mut prefix: *const XML_Char,
    mut uri: *const XML_Char,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut data: *mut XmlwfUserData = xmlwf_user_data_from_parser(parser);
    let mut fp: *mut FILE = xmlwf_fp(data);
    unsafe { fputs(b"<startns\0" as *const u8 as *const c_char, fp) };
    if !prefix.is_null() {
        let prefix_fmt = b" prefix=\"%s\"\0" as *const u8 as *const c_char;
        unsafe { fprintf(fp, prefix_fmt, prefix) };
    }
    if !uri.is_null() {
        unsafe { fputs(b" ns=\"\0" as *const u8 as *const c_char, fp) };
        characterData(data as *mut c_void, uri, unsafe { strlen(uri) } as c_int);
        unsafe { fputs(b"\"/>\n\0" as *const u8 as *const c_char, fp) };
    } else {
        unsafe { fputs(b"/>\n\0" as *const u8 as *const c_char, fp) };
    };
}

extern "C" fn metaEndNamespaceDecl(mut userData: *mut c_void, mut prefix: *const XML_Char) {
    let mut parser: XML_Parser = userData as XML_Parser;
    let mut fp: *mut FILE = xmlwf_fp_from_parser(parser);
    if prefix.is_null() {
        unsafe { fputs(b"<endns/>\n\0" as *const u8 as *const c_char, fp) };
    } else {
        let end_ns_fmt = b"<endns prefix=\"%s\"/>\n\0" as *const u8 as *const c_char;
        unsafe { fprintf(fp, end_ns_fmt, prefix) };
    };
}

extern "C" fn unknownEncodingConvert(mut data: *mut c_void, mut p: *const c_char) -> c_int {
    unsafe { libexpat::src::xmlwf::codepage::codepageConvert(*(data as *mut c_int), p) }
}

extern "C" fn stdlib_free(ptr: *mut c_void) {
    unsafe { free(ptr) }
}

extern "C" fn unknownEncoding(
    mut _userData: *mut c_void,
    mut name: *const XML_Char,
    mut info: *mut XML_Encoding,
) -> c_int {
    let mut cp: c_int = 0;
    static prefixL: [XML_Char; 9] = unsafe { transmute::<[u8; 9], [XML_Char; 9]>(*b"windows-\0") };
    static prefixU: [XML_Char; 9] = unsafe { transmute::<[u8; 9], [XML_Char; 9]>(*b"WINDOWS-\0") };
    let mut i: c_int = 0;
    while prefixU[i as usize] != 0 {
        let ch = unsafe { *name.offset(i as isize) as c_int };
        if ch != prefixU[i as usize] as c_int && ch != prefixL[i as usize] as c_int {
            return 0i32;
        }
        i += 1;
    }
    cp = 0;
    while unsafe { *name.offset(i as isize) } != 0 {
        static digits: [XML_Char; 11] =
            unsafe { transmute::<[u8; 11], [XML_Char; 11]>(*b"0123456789\0") };
        let current_char = unsafe { *name.offset(i as isize) as c_int };
        let mut s: *const XML_Char =
            unsafe { crate::stdlib::strchr(&raw const digits as *const c_char, current_char) };
        if s.is_null() {
            return 0i32;
        }
        cp *= 10;
        cp += unsafe { s.offset_from(&raw const digits as *const XML_Char) as c_int };
        if cp >= 0x10000 {
            return 0i32;
        }
        i += 1;
    }
    let map_ptr = unsafe { &raw mut (*info).map as *mut c_int };
    if libexpat::src::xmlwf::codepage::codepageMap(cp, map_ptr) == 0 {
        return 0i32;
    }
    let convert = Some(unknownEncodingConvert as extern "C" fn(*mut c_void, *const c_char) -> c_int);
    unsafe { (*info).convert = convert };
    unsafe { (*info).release = Some(stdlib_free) };
    unsafe { (*info).data = malloc(size_of::<c_int>()) };
    if unsafe { (*info).data.is_null() } {
        return 0i32;
    }
    unsafe { *((*info).data as *mut c_int) = cp };
    1
}

extern "C" fn notStandalone(mut _userData: *mut c_void) -> c_int {
    0
}

extern "C" fn showVersion(mut prog: *mut XML_Char) {
    let mut s: *mut XML_Char = prog;
    let mut ch: XML_Char = 0;
    let mut features: *const XML_Feature = XML_GetFeatureList();
    loop {
        ch = unsafe { *s };
        if ch as c_int == 0 {
            break;
        }
        if ch as c_int == '/' as i32 {
            prog = unsafe { s.offset(1isize) };
        }
        s = unsafe { s.offset(1) };
    }
    let version_fmt = b"%s using %s\n\0" as *const u8 as *const c_char;
    unsafe { fprintf(stdout, version_fmt, prog, XML_ExpatVersion()) };
    if !features.is_null() && unsafe { (*features.offset(0)).feature } != XML_FEATURE_END {
        let mut i: c_int = 1;
        let feature_name_fmt = b"%s\0" as *const u8 as *const c_char;
        unsafe { fprintf(stdout, feature_name_fmt, (*features.offset(0isize)).name) };
        if unsafe { (*features.offset(0)).value } != 0 {
            let feature_value_fmt = b"=%ld\0" as *const u8 as *const c_char;
            unsafe { fprintf(stdout, feature_value_fmt, (*features.offset(0isize)).value) };
        }
        while unsafe { (*features.offset(i as isize)).feature } != XML_FEATURE_END {
            let comma_feature_fmt = b", %s\0" as *const u8 as *const c_char;
            unsafe { fprintf(stdout, comma_feature_fmt, (*features.offset(i as isize)).name) };
            if unsafe { (*features.offset(i as isize)).value } != 0 {
                let feature_value_fmt = b"=%ld\0" as *const u8 as *const c_char;
                unsafe { fprintf(stdout, feature_value_fmt, (*features.offset(i as isize)).value) };
            }
            i += 1;
        }
        unsafe { fprintf(stdout, b"\n\0" as *const u8 as *const c_char) };
    }
}

extern "C" fn usage(mut prog: *const XML_Char, mut rc: c_int) -> ! {
    let usage_fmt = b"usage:\n  %s [OPTIONS] [FILE ...]\n  %s -h|--help\n  %s -v|--version\n\nxmlwf - Determines if an XML document is well-formed\n\npositional arguments:\n  FILE           file to process (default: STDIN)\n\ninput control arguments:\n  -s             print an error if the document is not [s]tandalone\n  -n             enable [n]amespace processing\n  -p             enable processing of external DTDs and [p]arameter entities\n  -x             enable processing of e[x]ternal entities\n                 (CAREFUL! This makes xmlwf vulnerable to external entity attacks (XXE).)\n  -e ENCODING    override any in-document [e]ncoding declaration\n  -w             enable support for [W]indows code pages\n  -r             disable memory-mapping and use [r]ead calls instead\n  -g BYTES       buffer size to request per call pair to XML_[G]etBuffer and read (default: 8 KiB)\n  -k             when processing multiple files, [k]eep processing after first file with error\n\noutput control arguments:\n  -d DIRECTORY   output [d]estination directory\n  -c             write a [c]opy of input XML, not canonical XML\n  -m             write [m]eta XML, not canonical XML\n  -t             write no XML output for [t]iming of plain parsing\n  -N             enable adding doctype and [n]otation declarations\n\namplification attack protection (e.g. billion laughs):\n  NOTE: If you ever need to increase these values for non-attack payload, please file a bug report.\n\n  -a FACTOR      set maximum tolerated [a]mplification factor (default: 100.0)\n  -b BYTES       set number of output [b]ytes needed to activate (default: 8 MiB/64 MiB)\n\nreparse deferral:\n  -q             disable reparse deferral, and allow [q]uadratic parse runtime with large tokens\n\ninfo arguments:\n  -h, --help     show this [h]elp message and exit\n  -v, --version  show program's [v]ersion number and exit\n\nenvironment variables:\n  EXPAT_ACCOUNTING_DEBUG=(0|1|2|3)\n                 Control verbosity of accounting debugging (default: 0)\n  EXPAT_ENTITY_DEBUG=(0|1)\n                 Control verbosity of entity debugging (default: 0)\n  EXPAT_ENTROPY_DEBUG=(0|1)\n                 Control verbosity of entropy debugging (default: 0)\n  EXPAT_MALLOC_DEBUG=(0|1|2)\n                 Control verbosity of allocation tracker (default: 0)\n\nexit status:\n  0              the input files are well-formed and the output (if requested) was written successfully\n  1              could not allocate data structures, signals a serious problem with execution environment\n  2              one or more input files were not well-formed\n  3              could not create an output file\n  4              command-line argument error\n\nxmlwf of libexpat is software libre, licensed under the MIT license.\nPlease report bugs at https://github.com/libexpat/libexpat/issues -- thank you!\n\0"
        as *const u8 as *const c_char;
    unsafe { fprintf(stderr, usage_fmt, prog, prog, prog) };
    unsafe { exit(rc) };
}

fn main_0(mut argc: c_int, mut argv: *mut *mut XML_Char) -> c_int {
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut outputDir: *const XML_Char = null::<XML_Char>();
    let mut encoding: *const XML_Char = null::<XML_Char>();
    let mut processFlags: c_uint = XML_MAP_FILE as c_uint;
    let mut windowsCodePages: c_int = 0;
    let mut outputType: c_int = 0;
    let mut useNamespaces: c_int = 0;
    let mut requireStandalone: c_int = 0;
    let mut requiresNotations: c_int = 0;
    let mut continueOnError: c_int = 0;
    let mut attackMaximumAmplification: core::ffi::c_float = -1.0;
    let mut attackThresholdBytes: core::ffi::c_ulonglong = 0;
    let mut attackThresholdGiven: XML_Bool = XML_FALSE;
    let mut disableDeferral: XML_Bool = XML_FALSE;
    let mut exitCode: c_int = XMLWF_EXIT_SUCCESS as c_int;
    let mut paramEntityParsing: XML_ParamEntityParsing = XML_PARAM_ENTITY_PARSING_NEVER;
    let mut useStdin: c_int = 0;
    let mut userData: XmlwfUserData = xmlwfUserData {
        fp: null_mut::<FILE>(),
        notationListHead: null_mut::<NotationList>(),
        currentDoctypeName: null::<XML_Char>(),
    };
    i = 1;
    j = 0;
    while i < argc {
        let arg = argv_at(argv, i);
        if j == 0 {
            if argv_char(argv, i, 0) != '-' as i32 {
                break;
            }
            if argv_char(argv, i, 1) == '-' as i32 {
                if argv_char(argv, i, 2) == '\0' as i32 {
                    i += 1;
                    break;
                } else if c_strcmp(arg_offset(arg, 2), b"help\0" as *const u8 as *const c_char) == 0 {
                    usage(argv_at(argv, 0), XMLWF_EXIT_SUCCESS as c_int);
                } else if c_strcmp(
                    arg_offset(arg, 2),
                    b"version\0" as *const u8 as *const c_char,
                ) == 0
                {
                    showVersion(argv_at(argv, 0));
                    return XMLWF_EXIT_SUCCESS as c_int;
                }
            }
            j += 1;
        }
        let mut current_block_122: u64;
        match argv_char(argv, i, j) {
                114 => {
                    processFlags &= !XML_MAP_FILE as c_uint;
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
                    if argv_char(argv, i, j + 1) == '\0' as i32 {
                        i += 1;
                        if i == argc {
                            usage(argv_at(argv, 0), XMLWF_EXIT_USAGE_ERROR as c_int);
                        }
                        outputDir = argv_at(argv, i);
                    } else {
                        outputDir = arg_offset(arg, j + 1);
                    }
                    i += 1;
                    j = 0;
                    current_block_122 = 8602574157404971894;
                }
                101 => {
                    if argv_char(argv, i, j + 1) == '\0' as i32 {
                        i += 1;
                        if i == argc {
                            usage(argv_at(argv, 0), XMLWF_EXIT_USAGE_ERROR as c_int);
                        }
                        encoding = argv_at(argv, i);
                    } else {
                        encoding = arg_offset(arg, j + 1);
                    }
                    i += 1;
                    j = 0;
                    current_block_122 = 8602574157404971894;
                }
                104 => {
                    usage(argv_at(argv, 0), XMLWF_EXIT_SUCCESS as c_int);
                }
                118 => {
                    showVersion(argv_at(argv, 0));
                    return XMLWF_EXIT_SUCCESS as c_int;
                }
                103 => {
                    let mut valueText: *const XML_Char = null::<XML_Char>();
                    if argv_char(argv, i, j + 1) == '\0' as i32 {
                        i += 1;
                        if i == argc {
                            usage(argv_at(argv, 0), XMLWF_EXIT_USAGE_ERROR as c_int);
                        }
                        valueText = argv_at(argv, i);
                    } else {
                        valueText = arg_offset(arg, j + 1);
                    }
                    i += 1;
                    j = 0;
                    set_errno_value(0);
                    let mut afterValueText: *mut XML_Char = valueText as *mut XML_Char;
                    let read_size_bytes_candidate: c_longlong =
                        c_strtoull(valueText, &raw mut afterValueText, 10) as c_longlong;
                    if errno_value() != 0
                        || xml_char_at(afterValueText as *const XML_Char, 0) != '\0' as i32
                        || read_size_bytes_candidate < 1
                        || read_size_bytes_candidate > (INT_MAX / 2 + 1) as c_longlong
                    {
                        set_errno_value(ERANGE);
                        c_perror(
                        b"invalid buffer size (needs an integer from 1 to INT_MAX/2+1 i.e. 1,073,741,824 on most platforms)\0"
                            as *const u8 as *const c_char,
                    );
                        c_exit(XMLWF_EXIT_USAGE_ERROR as c_int);
                    }
                    set_read_size_bytes(read_size_bytes_candidate as c_int);
                    current_block_122 = 8602574157404971894;
                }
                107 => {
                    continueOnError = 1;
                    j += 1;
                    current_block_122 = 8602574157404971894;
                }
                97 => {
                    let mut valueText_0: *const XML_Char = null::<XML_Char>();
                    if argv_char(argv, i, j + 1) == '\0' as i32 {
                        i += 1;
                        if i == argc {
                            usage(argv_at(argv, 0), XMLWF_EXIT_USAGE_ERROR as c_int);
                        }
                        valueText_0 = argv_at(argv, i);
                    } else {
                        valueText_0 = arg_offset(arg, j + 1);
                    }
                    i += 1;
                    j = 0;
                    set_errno_value(0);
                    let mut afterValueText_0: *mut XML_Char = null_mut::<XML_Char>();
                    attackMaximumAmplification = c_strtof(valueText_0, &raw mut afterValueText_0);
                    if errno_value() != 0
                        || xml_char_at(afterValueText_0 as *const XML_Char, 0) != '\0' as i32
                        || attackMaximumAmplification.is_nan() as i32 != 0
                        || attackMaximumAmplification < 1.0
                    {
                        set_errno_value(ERANGE);
                        c_perror(
                        b"invalid amplification limit (needs a floating point number greater or equal than 1.0)\0"
                            as *const u8 as *const c_char,
                    );
                        c_exit(XMLWF_EXIT_USAGE_ERROR as c_int);
                    }
                    current_block_122 = 8602574157404971894;
                }
                98 => {
                    let mut valueText_1: *const XML_Char = null::<XML_Char>();
                    if argv_char(argv, i, j + 1) == '\0' as i32 {
                        i += 1;
                        if i == argc {
                            usage(argv_at(argv, 0), XMLWF_EXIT_USAGE_ERROR as c_int);
                        }
                        valueText_1 = argv_at(argv, i);
                    } else {
                        valueText_1 = arg_offset(arg, j + 1);
                    }
                    i += 1;
                    j = 0;
                    set_errno_value(0);
                    let mut afterValueText_1: *mut XML_Char = valueText_1 as *mut XML_Char;
                    attackThresholdBytes = c_strtoull(valueText_1, &raw mut afterValueText_1, 10);
                    if errno_value() != 0
                        || xml_char_at(afterValueText_1 as *const XML_Char, 0) != '\0' as i32
                    {
                        set_errno_value(ERANGE);
                        c_perror(
                            b"invalid ignore threshold (needs an integer from 0 to 2^64-1)\0"
                                as *const u8 as *const c_char,
                        );
                        c_exit(XMLWF_EXIT_USAGE_ERROR as c_int);
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
                processFlags |= XML_EXTERNAL_ENTITIES as c_uint;
                j += 1;
            }
            15955764443707486316 => {
                usage(argv_at(argv, 0), XMLWF_EXIT_USAGE_ERROR as c_int);
            }
            _ => {}
        }
    }
    if i == argc {
        useStdin = 1;
        processFlags &= !XML_MAP_FILE as c_uint;
        i -= 1;
    }
    let mut current_block_219: u64;
    while i < argc {
        let mut outName: *mut XML_Char = null_mut::<XML_Char>();
        let mut result: c_int = 0;
        let mut parser: XML_Parser = null_mut::<XML_ParserStruct>();
        if useNamespaces != 0 {
            parser = XML_ParserCreateNS(encoding, '\u{1}' as XML_Char);
        } else {
            parser = XML_ParserCreate(encoding);
        }
        if parser.is_null() {
            c_perror(b"Could not instantiate parser\0" as *const u8 as *const c_char);
            c_exit(XMLWF_EXIT_INTERNAL_ERROR as c_int);
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
                set_errno_value(EINVAL);
                c_perror(b"Failed to disable reparse deferral\0" as *const u8 as *const c_char);
                c_exit(XMLWF_EXIT_INTERNAL_ERROR as c_int);
            }
        }
        if requireStandalone != 0 {
            XML_SetNotStandaloneHandler(
                parser,
                Some(notStandalone as extern "C" fn(*mut c_void) -> c_int),
            );
        }
        XML_SetParamEntityParsing(parser, paramEntityParsing);
        if outputType == 't' as i32 {
            outputDir = null::<XML_Char>();
            XML_SetElementHandler(
                parser,
                Some(
                    nopStartElement
                        as extern "C" fn(*mut c_void, *const XML_Char, *mut *const XML_Char) -> (),
                ),
                Some(nopEndElement as extern "C" fn(*mut c_void, *const XML_Char) -> ()),
            );
            XML_SetCharacterDataHandler(
                parser,
                Some(nopCharacterData as extern "C" fn(*mut c_void, *const XML_Char, c_int) -> ()),
            );
            XML_SetProcessingInstructionHandler(
                parser,
                Some(
                    nopProcessingInstruction
                        as extern "C" fn(*mut c_void, *const XML_Char, *const XML_Char) -> (),
                ),
            );
            current_block_219 = 9952640327414195044;
        } else if !outputDir.is_null() {
            let mut delim: *const XML_Char = b"/\0" as *const u8 as *const XML_Char;
            let mut file: *const XML_Char = if useStdin != 0 {
                b"STDIN\0" as *const u8 as *const XML_Char
            } else {
                argv_at(argv, i) as *const XML_Char
            };
            if useStdin == 0 {
                let delim_char = xml_char_at(delim, 0);
                let mut lastDelim: *const XML_Char = c_strrchr(file, delim_char);
                if !lastDelim.is_null() {
                    file = c_char_ptr_offset(lastDelim, 1);
                }
            }
            let out_name_size = c_strlen(outputDir)
                .wrapping_add(c_strlen(file))
                .wrapping_add(2usize)
                .wrapping_mul(size_of::<XML_Char>());
            outName = c_malloc(out_name_size) as *mut XML_Char;
            if outName.is_null() {
                c_perror(b"Could not allocate memory\0" as *const u8 as *const c_char);
                c_exit(XMLWF_EXIT_INTERNAL_ERROR as c_int);
            }
            c_strcpy(outName, outputDir);
            c_strcat(outName, delim);
            c_strcat(outName, file);
            userData.fp = c_fopen(outName, b"wb\0" as *const u8 as *const c_char);
            if userData.fp.is_null() {
                c_perror(outName);
                exitCode = XMLWF_EXIT_OUTPUT_ERROR as c_int;
                c_free(outName as *mut c_void);
                XML_ParserFree(parser);
                if continueOnError == 0 {
                    break;
                }
                current_block_219 = 15947798178928648489;
            } else {
                c_setvbuf(userData.fp, null_mut::<c_char>(), _IOFBF, 16384);
                XML_SetUserData(parser, &raw mut userData as *mut c_void);
                match outputType {
                        109 => {
                            XML_UseParserAsHandlerArg(parser);
                            XML_SetElementHandler(
                                parser,
                                Some(
                                    metaStartElement
                                        as extern "C" fn(
                                            *mut c_void,
                                            *const XML_Char,
                                            *mut *const XML_Char,
                                        )
                                            -> (),
                                ),
                                Some(
                                    metaEndElement
                                        as extern "C" fn(*mut c_void, *const XML_Char) -> (),
                                ),
                            );
                            XML_SetProcessingInstructionHandler(
                                parser,
                                Some(
                                    metaProcessingInstruction
                                        as extern "C" fn(
                                            *mut c_void,
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
                                        as extern "C" fn(*mut c_void, *const XML_Char) -> (),
                                ),
                            );
                            XML_SetCdataSectionHandler(
                                parser,
                                Some(metaStartCdataSection as extern "C" fn(*mut c_void) -> ()),
                                Some(metaEndCdataSection as extern "C" fn(*mut c_void) -> ()),
                            );
                            XML_SetCharacterDataHandler(
                                parser,
                                Some(
                                    metaCharacterData
                                        as extern "C" fn(*mut c_void, *const XML_Char, c_int) -> (),
                                ),
                            );
                            XML_SetDoctypeDeclHandler(
                                parser,
                                Some(
                                    metaStartDoctypeDecl
                                        as extern "C" fn(
                                            *mut c_void,
                                            *const XML_Char,
                                            *const XML_Char,
                                            *const XML_Char,
                                            c_int,
                                        )
                                            -> (),
                                ),
                                Some(metaEndDoctypeDecl as extern "C" fn(*mut c_void) -> ()),
                            );
                            XML_SetEntityDeclHandler(
                                parser,
                                Some(
                                    metaEntityDecl
                                        as extern "C" fn(
                                            *mut c_void,
                                            *const XML_Char,
                                            c_int,
                                            *const XML_Char,
                                            c_int,
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
                                        as extern "C" fn(
                                            *mut c_void,
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
                                        as extern "C" fn(
                                            *mut c_void,
                                            *const XML_Char,
                                            *const XML_Char,
                                        )
                                            -> (),
                                ),
                                Some(
                                    metaEndNamespaceDecl
                                        as extern "C" fn(*mut c_void, *const XML_Char) -> (),
                                ),
                            );
                            metaStartDocument(parser as *mut c_void);
                        }
                        99 => {
                            XML_UseParserAsHandlerArg(parser);
                            XML_SetDefaultHandler(
                                parser,
                                Some(
                                    markup
                                        as extern "C" fn(*mut c_void, *const XML_Char, c_int) -> (),
                                ),
                            );
                            XML_SetElementHandler(
                                parser,
                                Some(
                                    defaultStartElement
                                        as extern "C" fn(
                                            *mut c_void,
                                            *const XML_Char,
                                            *mut *const XML_Char,
                                        )
                                            -> (),
                                ),
                                Some(
                                    defaultEndElement
                                        as extern "C" fn(*mut c_void, *const XML_Char) -> (),
                                ),
                            );
                            XML_SetCharacterDataHandler(
                                parser,
                                Some(
                                    defaultCharacterData
                                        as extern "C" fn(*mut c_void, *const XML_Char, c_int) -> (),
                                ),
                            );
                            XML_SetProcessingInstructionHandler(
                                parser,
                                Some(
                                    defaultProcessingInstruction
                                        as extern "C" fn(
                                            *mut c_void,
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
                                            as extern "C" fn(
                                                *mut c_void,
                                                *const XML_Char,
                                                *mut *const XML_Char,
                                            )
                                                -> (),
                                    ),
                                    Some(
                                        endElementNS
                                            as extern "C" fn(*mut c_void, *const XML_Char) -> (),
                                    ),
                                );
                            } else {
                                XML_SetElementHandler(
                                    parser,
                                    Some(
                                        startElement
                                            as extern "C" fn(
                                                *mut c_void,
                                                *const XML_Char,
                                                *mut *const XML_Char,
                                            )
                                                -> (),
                                    ),
                                    Some(
                                        endElement
                                            as extern "C" fn(*mut c_void, *const XML_Char) -> (),
                                    ),
                                );
                            }
                            XML_SetCharacterDataHandler(
                                parser,
                                Some(
                                    characterData
                                        as extern "C" fn(*mut c_void, *const XML_Char, c_int) -> (),
                                ),
                            );
                            XML_SetProcessingInstructionHandler(
                                parser,
                                Some(
                                    processingInstruction
                                        as extern "C" fn(
                                            *mut c_void,
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
                                            as extern "C" fn(
                                                *mut c_void,
                                                *const XML_Char,
                                                *const XML_Char,
                                                *const XML_Char,
                                                c_int,
                                            )
                                                -> (),
                                    ),
                                    Some(endDoctypeDecl as extern "C" fn(*mut c_void) -> ()),
                                );
                                XML_SetNotationDeclHandler(
                                    parser,
                                    Some(
                                        notationDecl
                                            as extern "C" fn(
                                                *mut c_void,
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
        if current_block_219 == 9952640327414195044 {
            if windowsCodePages != 0 {
                XML_SetUnknownEncodingHandler(parser, unknown_encoding_handler(), null_mut::<c_void>());
            }
            result = XML_ProcessFile(
                parser,
                if useStdin != 0 {
                    null_mut::<XML_Char>()
                } else {
                    argv_at(argv, i)
                },
                processFlags,
            );
            if !outputDir.is_null() {
                if outputType == 'm' as i32 {
                    metaEndDocument(parser as *mut c_void);
                }
                c_fclose(userData.fp);
                if result == 0 {
                    c_remove(outName);
                }
                c_free(outName as *mut c_void);
            }
            XML_ParserFree(parser);
            if result == 0 {
                exitCode = XMLWF_EXIT_NOT_WELLFORMED as c_int;
                cleanupUserData(&raw mut userData);
                if continueOnError == 0 {
                    break;
                }
            }
        }
        i += 1;
    }
    exitCode
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = std::env::args()
        .map(|arg| {
            std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut c_char)
        .chain(core::iter::once(null_mut()))
        .collect();
    std::process::exit(main_0(
        (args_ptrs.len() - 1) as c_int,
        args_ptrs.as_mut_ptr(),
    ))
}
