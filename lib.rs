#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(raw_ref_op)]
#![feature(register_tool)]
#![register_tool(c2rust)]

pub mod siphash_h {
    use crate::stdlib::uint64_t;
    use core::ffi::c_uchar;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct siphash {
        pub v0: uint64_t,
        pub v1: uint64_t,
        pub v2: uint64_t,
        pub v3: uint64_t,
        pub buf: [c_uchar; 8],
        pub p: *mut c_uchar,
        pub c: uint64_t,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct sipkey {
        pub k: [uint64_t; 2],
    }
}
pub mod expat_external_h {
    use core::ffi::c_char;
    use core::ffi::c_long;
    use core::ffi::c_ulong;
    pub type XML_Char = c_char;

    pub type XML_LChar = c_char;

    pub type XML_Index = c_long;

    pub type XML_Size = c_ulong;
}
pub mod __stddef_size_t_h {
    pub type size_t = usize;
}
pub mod __stddef_ptrdiff_t_h {
    pub type ptrdiff_t = isize;
}
pub mod filemap_h {
    use core::ffi::c_int;
    pub const XML_MAX_CHUNK_LEN: c_int = crate::limits_h::INT_MAX / 2 + 1;
}
pub mod xmltok_impl_c {
    use crate::xmltok_impl_h::C2RustUnnamed_3;
    pub const other: C2RustUnnamed_3 = 0;

    pub const other_0: C2RustUnnamed_3 = 0;

    pub const other_1: C2RustUnnamed_3 = 0;

    pub const inName: C2RustUnnamed_3 = 1;

    pub const inName_0: C2RustUnnamed_3 = 1;

    pub const inName_1: C2RustUnnamed_3 = 1;

    pub const inValue: C2RustUnnamed_3 = 2;

    pub const inValue_0: C2RustUnnamed_3 = 2;

    pub const inValue_1: C2RustUnnamed_3 = 2;
}
pub mod xmltok_impl_h {
    pub type C2RustUnnamed_3 = core::ffi::c_uint;

    pub const BT_NONXML: C2RustUnnamed_3 = 0;

    pub const BT_MALFORM: C2RustUnnamed_3 = 1;

    pub const BT_LT: C2RustUnnamed_3 = 2;

    pub const BT_AMP: C2RustUnnamed_3 = 3;

    pub const BT_RSQB: C2RustUnnamed_3 = 4;

    pub const BT_LEAD2: C2RustUnnamed_3 = 5;

    pub const BT_LEAD3: C2RustUnnamed_3 = 6;

    pub const BT_LEAD4: C2RustUnnamed_3 = 7;

    pub const BT_TRAIL: C2RustUnnamed_3 = 8;

    pub const BT_CR: C2RustUnnamed_3 = 9;

    pub const BT_LF: C2RustUnnamed_3 = 10;

    pub const BT_GT: C2RustUnnamed_3 = 11;

    pub const BT_QUOT: C2RustUnnamed_3 = 12;

    pub const BT_APOS: C2RustUnnamed_3 = 13;

    pub const BT_EQUALS: C2RustUnnamed_3 = 14;

    pub const BT_QUEST: C2RustUnnamed_3 = 15;

    pub const BT_EXCL: C2RustUnnamed_3 = 16;

    pub const BT_SOL: C2RustUnnamed_3 = 17;

    pub const BT_SEMI: C2RustUnnamed_3 = 18;

    pub const BT_NUM: C2RustUnnamed_3 = 19;

    pub const BT_LSQB: C2RustUnnamed_3 = 20;

    pub const BT_S: C2RustUnnamed_3 = 21;

    pub const BT_NMSTRT: C2RustUnnamed_3 = 22;

    pub const BT_COLON_0: C2RustUnnamed_3 = 23;

    pub const BT_HEX: C2RustUnnamed_3 = 24;

    pub const BT_DIGIT: C2RustUnnamed_3 = 25;

    pub const BT_NAME: C2RustUnnamed_3 = 26;

    pub const BT_MINUS: C2RustUnnamed_3 = 27;

    pub const BT_OTHER: C2RustUnnamed_3 = 28;

    pub const BT_NONASCII: C2RustUnnamed_3 = 29;

    pub const BT_PERCNT: C2RustUnnamed_3 = 30;

    pub const BT_LPAR: C2RustUnnamed_3 = 31;

    pub const BT_RPAR: C2RustUnnamed_3 = 32;

    pub const BT_AST: C2RustUnnamed_3 = 33;

    pub const BT_PLUS: C2RustUnnamed_3 = 34;

    pub const BT_COMMA: C2RustUnnamed_3 = 35;

    pub const BT_VERBAR: C2RustUnnamed_3 = 36;
}
pub mod internal {
    use core::ffi::c_int;
    pub const __INT_MAX__: c_int = 2147483647;
}
pub mod ascii_h {
    use core::ffi::c_int;
    pub const ASCII_A: c_int = 0x41;

    pub const ASCII_B: c_int = 0x42;

    pub const ASCII_B_1: c_int = 66;

    pub const ASCII_C: c_int = 0x43;

    pub const ASCII_D: c_int = 0x44;

    pub const ASCII_E: c_int = 0x45;

    pub const ASCII_E_1: c_int = 69;

    pub const ASCII_F: c_int = 0x46;

    pub const ASCII_F_1: c_int = 70;

    pub const ASCII_G: c_int = 0x47;

    pub const ASCII_I: c_int = 0x49;

    pub const ASCII_K: c_int = 0x4b;

    pub const ASCII_L: c_int = 0x4c;

    pub const ASCII_L_1: c_int = 76;

    pub const ASCII_M: c_int = 0x4d;

    pub const ASCII_M_1: c_int = 77;

    pub const ASCII_N: c_int = 0x4e;

    pub const ASCII_O: c_int = 0x4f;

    pub const ASCII_P: c_int = 0x50;

    pub const ASCII_Q: c_int = 0x51;

    pub const ASCII_R: c_int = 0x52;

    pub const ASCII_S: c_int = 0x53;

    pub const ASCII_T: c_int = 0x54;

    pub const ASCII_U: c_int = 0x55;

    pub const ASCII_X: c_int = 0x58;

    pub const ASCII_X_1: c_int = 88;

    pub const ASCII_Y: c_int = 0x59;

    pub const ASCII_Z: c_int = 0x5a;

    pub const ASCII_a: c_int = 0x61;

    pub const ASCII_a_1: c_int = 97;

    pub const ASCII_b: c_int = 98;

    pub const ASCII_c: c_int = 0x63;

    pub const ASCII_c_1: c_int = 99;

    pub const ASCII_d: c_int = 100;

    pub const ASCII_e: c_int = 0x65;

    pub const ASCII_e_1: c_int = 101;

    pub const ASCII_f: c_int = 102;

    pub const ASCII_g: c_int = 0x67;

    pub const ASCII_g_1: c_int = 103;

    pub const ASCII_h: c_int = 0x68;

    pub const ASCII_i: c_int = 0x69;

    pub const ASCII_l: c_int = 0x6c;

    pub const ASCII_l_1: c_int = 108;

    pub const ASCII_m: c_int = 0x6d;

    pub const ASCII_m_1: c_int = 109;

    pub const ASCII_n: c_int = 0x6e;

    pub const ASCII_o: c_int = 0x6f;

    pub const ASCII_p: c_int = 0x70;

    pub const ASCII_q: c_int = 113;

    pub const ASCII_r: c_int = 0x72;

    pub const ASCII_s: c_int = 0x73;

    pub const ASCII_t: c_int = 0x74;

    pub const ASCII_v: c_int = 0x76;

    pub const ASCII_w: c_int = 0x77;

    pub const ASCII_x: c_int = 0x78;

    pub const ASCII_x_1: c_int = 120;

    pub const ASCII_y: c_int = 0x79;

    pub const ASCII_z: c_int = 0x7a;

    pub const ASCII_0: c_int = 0x30;

    pub const ASCII_1: c_int = 0x31;

    pub const ASCII_1_1: c_int = 49;

    pub const ASCII_2: c_int = 0x32;

    pub const ASCII_2_1: c_int = 50;

    pub const ASCII_3: c_int = 0x33;

    pub const ASCII_3_1: c_int = 51;

    pub const ASCII_4: c_int = 52;

    pub const ASCII_5: c_int = 53;

    pub const ASCII_6: c_int = 54;

    pub const ASCII_7: c_int = 55;

    pub const ASCII_8: c_int = 0x38;

    pub const ASCII_8_1: c_int = 56;

    pub const ASCII_9: c_int = 0x39;

    pub const ASCII_9_1: c_int = 57;

    pub const ASCII_SPACE: c_int = 0x20;

    pub const ASCII_EXCL: c_int = 0x21;

    pub const ASCII_QUOT: c_int = 0x22;

    pub const ASCII_AMP: c_int = 0x26;

    pub const ASCII_APOS: c_int = 0x27;

    pub const ASCII_MINUS: c_int = 0x2d;

    pub const ASCII_PERIOD: c_int = 0x2e;

    pub const ASCII_COLON: c_int = 0x3a;

    pub const ASCII_LT: c_int = 0x3c;

    pub const ASCII_EQUALS: c_int = 0x3d;

    pub const ASCII_GT: c_int = 0x3e;

    pub const ASCII_LSQB: c_int = 0x5b;

    pub const ASCII_UNDERSCORE: c_int = 0x5f;

    pub const ASCII_LPAREN: c_int = 0x28;

    pub const ASCII_SLASH: c_int = 0x2f;

    pub const ASCII_HASH: c_int = 0x23;

    pub const ASCII_PIPE: c_int = 0x7c;

    pub const ASCII_COMMA: c_int = 0x2c;
}
pub mod __stddef_null_h {
    use core::ffi::c_void;
    pub const NULL: *mut c_void = core::ptr::null_mut::<c_void>();
}
pub mod stdbool_h {
    use core::ffi::c_int;
    pub const true_0: c_int = 1;

    pub const false_0: c_int = 0;
}
pub mod expat_config_h {
    use core::ffi::c_int;
    pub const XML_CONTEXT_BYTES: c_int = 1024;
}
pub mod limits_h {
    use core::ffi::c_int;
    use core::ffi::c_uint;
    pub const INT_MAX: c_int = crate::internal::__INT_MAX__;

    pub const UINT_MAX: c_uint = (crate::internal::__INT_MAX__ as c_uint)
        .wrapping_mul(2u32)
        .wrapping_add(1u32);
}
pub mod internal_h {
    use crate::__stddef_size_t_h::size_t;
    use core::ffi::c_float;
    use core::ffi::c_int;
    use core::ffi::c_longlong;
    use core::mem::size_of;
    pub const EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT: c_float = 100.0;

    pub const EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT: c_int = 8388608;

    pub const EXPAT_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT: c_float = 100.0;

    pub const EXPAT_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT: c_int = 67108864;

    pub const EXPAT_MALLOC_ALIGNMENT: usize = size_of::<c_longlong>();

    pub const EXPAT_MALLOC_PADDING: usize =
        (size_of::<c_longlong>()).wrapping_sub(size_of::<size_t>());
}
pub mod expat_h {
    use crate::__stddef_size_t_h::size_t;
    use crate::expat_external_h::XML_Char;
    pub use crate::src::lib::xmlparse::XML_ParserStruct;
    use core::ffi::c_char;
    use core::ffi::c_int;
    use core::ffi::c_long;
    use core::ffi::c_uchar;
    use core::ffi::c_uint;
    use core::ffi::c_void;

    pub type XML_Parser = *mut XML_ParserStruct;

    pub type XML_Bool = c_uchar;

    pub const XML_TRUE: XML_Bool = 1;

    pub const XML_FALSE: XML_Bool = 0;

    pub type XML_Status = c_uint;

    pub const XML_STATUS_ERROR: XML_Status = 0;

    pub const XML_STATUS_OK: XML_Status = 1;

    pub const XML_STATUS_SUSPENDED: XML_Status = 2;

    pub type XML_Error = c_uint;

    pub const XML_ERROR_NONE: XML_Error = 0;

    pub const XML_ERROR_NO_MEMORY: XML_Error = 1;

    pub const XML_ERROR_SYNTAX: XML_Error = 2;

    pub const XML_ERROR_NO_ELEMENTS: XML_Error = 3;

    pub const XML_ERROR_INVALID_TOKEN: XML_Error = 4;

    pub const XML_ERROR_UNCLOSED_TOKEN: XML_Error = 5;

    pub const XML_ERROR_PARTIAL_CHAR: XML_Error = 6;

    pub const XML_ERROR_TAG_MISMATCH: XML_Error = 7;

    pub const XML_ERROR_DUPLICATE_ATTRIBUTE: XML_Error = 8;

    pub const XML_ERROR_JUNK_AFTER_DOC_ELEMENT: XML_Error = 9;

    pub const XML_ERROR_PARAM_ENTITY_REF: XML_Error = 10;

    pub const XML_ERROR_UNDEFINED_ENTITY: XML_Error = 11;

    pub const XML_ERROR_RECURSIVE_ENTITY_REF: XML_Error = 12;

    pub const XML_ERROR_ASYNC_ENTITY: XML_Error = 13;

    pub const XML_ERROR_BAD_CHAR_REF: XML_Error = 14;

    pub const XML_ERROR_BINARY_ENTITY_REF: XML_Error = 15;

    pub const XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF: XML_Error = 16;

    pub const XML_ERROR_MISPLACED_XML_PI: XML_Error = 17;

    pub const XML_ERROR_UNKNOWN_ENCODING: XML_Error = 18;

    pub const XML_ERROR_INCORRECT_ENCODING: XML_Error = 19;

    pub const XML_ERROR_UNCLOSED_CDATA_SECTION: XML_Error = 20;

    pub const XML_ERROR_EXTERNAL_ENTITY_HANDLING: XML_Error = 21;

    pub const XML_ERROR_NOT_STANDALONE: XML_Error = 22;

    pub const XML_ERROR_UNEXPECTED_STATE: XML_Error = 23;

    pub const XML_ERROR_ENTITY_DECLARED_IN_PE: XML_Error = 24;

    pub const XML_ERROR_FEATURE_REQUIRES_XML_DTD: XML_Error = 25;

    pub const XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING: XML_Error = 26;

    pub const XML_ERROR_UNBOUND_PREFIX: XML_Error = 27;

    pub const XML_ERROR_UNDECLARING_PREFIX: XML_Error = 28;

    pub const XML_ERROR_INCOMPLETE_PE: XML_Error = 29;

    pub const XML_ERROR_XML_DECL: XML_Error = 30;

    pub const XML_ERROR_TEXT_DECL: XML_Error = 31;

    pub const XML_ERROR_PUBLICID: XML_Error = 32;

    pub const XML_ERROR_SUSPENDED: XML_Error = 33;

    pub const XML_ERROR_NOT_SUSPENDED: XML_Error = 34;

    pub const XML_ERROR_ABORTED: XML_Error = 35;

    pub const XML_ERROR_FINISHED: XML_Error = 36;

    pub const XML_ERROR_SUSPEND_PE: XML_Error = 37;

    pub const XML_ERROR_RESERVED_PREFIX_XML: XML_Error = 38;

    pub const XML_ERROR_RESERVED_PREFIX_XMLNS: XML_Error = 39;

    pub const XML_ERROR_RESERVED_NAMESPACE_URI: XML_Error = 40;

    pub const XML_ERROR_INVALID_ARGUMENT: XML_Error = 41;

    pub const XML_ERROR_NO_BUFFER: XML_Error = 42;

    pub const XML_ERROR_AMPLIFICATION_LIMIT_BREACH: XML_Error = 43;

    pub const XML_ERROR_NOT_STARTED: XML_Error = 44;

    pub type XML_Content_Type = c_uint;

    pub const XML_CTYPE_EMPTY: XML_Content_Type = 1;

    pub const XML_CTYPE_ANY: XML_Content_Type = 2;

    pub const XML_CTYPE_MIXED: XML_Content_Type = 3;

    pub const XML_CTYPE_NAME: XML_Content_Type = 4;

    pub const XML_CTYPE_CHOICE: XML_Content_Type = 5;

    pub const XML_CTYPE_SEQ: XML_Content_Type = 6;

    pub type XML_Content_Quant = c_uint;

    pub const XML_CQUANT_NONE: XML_Content_Quant = 0;

    pub const XML_CQUANT_OPT: XML_Content_Quant = 1;

    pub const XML_CQUANT_REP: XML_Content_Quant = 2;

    pub const XML_CQUANT_PLUS: XML_Content_Quant = 3;

    pub type XML_Content = crate::expat_h::XML_cp;

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct XML_cp {
        pub type_0: XML_Content_Type,
        pub quant: XML_Content_Quant,
        pub name: *mut XML_Char,
        pub numchildren: c_uint,
        pub children: *mut crate::expat_h::XML_Content,
    }

    pub type XML_ElementDeclHandler = Option<
        unsafe extern "C" fn(*mut c_void, *const XML_Char, *mut crate::expat_h::XML_Content) -> (),
    >;

    pub type XML_AttlistDeclHandler = Option<
        unsafe extern "C" fn(
            *mut c_void,
            *const XML_Char,
            *const XML_Char,
            *const XML_Char,
            *const XML_Char,
            c_int,
        ) -> (),
    >;

    pub type XML_XmlDeclHandler =
        Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, *const XML_Char, c_int) -> ()>;

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct XML_Memory_Handling_Suite {
        pub malloc_fcn: Option<unsafe extern "C" fn(size_t) -> *mut c_void>,
        pub realloc_fcn: Option<unsafe extern "C" fn(*mut c_void, size_t) -> *mut c_void>,
        pub free_fcn: Option<unsafe extern "C" fn(*mut c_void) -> ()>,
    }

    pub type XML_StartElementHandler =
        Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, *mut *const XML_Char) -> ()>;

    pub type XML_EndElementHandler =
        Option<unsafe extern "C" fn(*mut c_void, *const XML_Char) -> ()>;

    pub type XML_CharacterDataHandler =
        Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, c_int) -> ()>;

    pub type XML_ProcessingInstructionHandler =
        Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, *const XML_Char) -> ()>;

    pub type XML_CommentHandler = Option<unsafe extern "C" fn(*mut c_void, *const XML_Char) -> ()>;

    pub type XML_StartCdataSectionHandler = Option<unsafe extern "C" fn(*mut c_void) -> ()>;

    pub type XML_EndCdataSectionHandler = Option<unsafe extern "C" fn(*mut c_void) -> ()>;

    pub type XML_DefaultHandler =
        Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, c_int) -> ()>;

    pub type XML_StartDoctypeDeclHandler = Option<
        unsafe extern "C" fn(
            *mut c_void,
            *const XML_Char,
            *const XML_Char,
            *const XML_Char,
            c_int,
        ) -> (),
    >;

    pub type XML_EndDoctypeDeclHandler = Option<unsafe extern "C" fn(*mut c_void) -> ()>;

    pub type XML_EntityDeclHandler = Option<
        unsafe extern "C" fn(
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

    pub type XML_UnparsedEntityDeclHandler = Option<
        unsafe extern "C" fn(
            *mut c_void,
            *const XML_Char,
            *const XML_Char,
            *const XML_Char,
            *const XML_Char,
            *const XML_Char,
        ) -> (),
    >;

    pub type XML_NotationDeclHandler = Option<
        unsafe extern "C" fn(
            *mut c_void,
            *const XML_Char,
            *const XML_Char,
            *const XML_Char,
            *const XML_Char,
        ) -> (),
    >;

    pub type XML_StartNamespaceDeclHandler =
        Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, *const XML_Char) -> ()>;

    pub type XML_EndNamespaceDeclHandler =
        Option<unsafe extern "C" fn(*mut c_void, *const XML_Char) -> ()>;

    pub type XML_NotStandaloneHandler = Option<unsafe extern "C" fn(*mut c_void) -> c_int>;

    pub type XML_ExternalEntityRefHandler = Option<
        unsafe extern "C" fn(
            crate::expat_h::XML_Parser,
            *const XML_Char,
            *const XML_Char,
            *const XML_Char,
            *const XML_Char,
        ) -> c_int,
    >;

    pub type XML_SkippedEntityHandler =
        Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, c_int) -> ()>;

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct XML_Encoding {
        pub map: [c_int; 256],
        pub data: *mut c_void,
        pub convert: Option<unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int>,
        pub release: Option<unsafe extern "C" fn(*mut c_void) -> ()>,
    }

    pub type XML_UnknownEncodingHandler = Option<
        unsafe extern "C" fn(
            *mut c_void,
            *const XML_Char,
            *mut crate::expat_h::XML_Encoding,
        ) -> c_int,
    >;

    pub type XML_Parsing = c_uint;

    pub const XML_INITIALIZED: XML_Parsing = 0;

    pub const XML_PARSING: XML_Parsing = 1;

    pub const XML_FINISHED: XML_Parsing = 2;

    pub const XML_SUSPENDED: XML_Parsing = 3;

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct XML_ParsingStatus {
        pub parsing: XML_Parsing,
        pub finalBuffer: XML_Bool,
    }

    pub type XML_ParamEntityParsing = c_uint;

    pub const XML_PARAM_ENTITY_PARSING_NEVER: XML_ParamEntityParsing = 0;

    pub const XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE: XML_ParamEntityParsing = 1;

    pub const XML_PARAM_ENTITY_PARSING_ALWAYS: XML_ParamEntityParsing = 2;

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct XML_Expat_Version {
        pub major: c_int,
        pub minor: c_int,
        pub micro: c_int,
    }

    pub type XML_FeatureEnum = c_uint;

    pub const XML_FEATURE_END: XML_FeatureEnum = 0;

    pub const XML_FEATURE_UNICODE: XML_FeatureEnum = 1;

    pub const XML_FEATURE_UNICODE_WCHAR_T: XML_FeatureEnum = 2;

    pub const XML_FEATURE_DTD: XML_FeatureEnum = 3;

    pub const XML_FEATURE_CONTEXT_BYTES: XML_FeatureEnum = 4;

    pub const XML_FEATURE_MIN_SIZE: XML_FeatureEnum = 5;

    pub const XML_FEATURE_SIZEOF_XML_CHAR: XML_FeatureEnum = 6;

    pub const XML_FEATURE_SIZEOF_XML_LCHAR: XML_FeatureEnum = 7;

    pub const XML_FEATURE_NS: XML_FeatureEnum = 8;

    pub const XML_FEATURE_LARGE_SIZE: XML_FeatureEnum = 9;

    pub const XML_FEATURE_ATTR_INFO: XML_FeatureEnum = 10;

    pub const XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT:
        XML_FeatureEnum = 11;

    pub const XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT:
        XML_FeatureEnum = 12;

    pub const XML_FEATURE_GE: XML_FeatureEnum = 13;

    pub const XML_FEATURE_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT: XML_FeatureEnum = 14;

    pub const XML_FEATURE_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT: XML_FeatureEnum = 15;

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct XML_Feature {
        pub feature: XML_FeatureEnum,
        pub name: *const crate::expat_external_h::XML_LChar,
        pub value: c_long,
    }

    pub const XML_MAJOR_VERSION: c_int = 2;

    pub const XML_MINOR_VERSION: c_int = 7;

    pub const XML_MICRO_VERSION: c_int = 4;
}
pub mod stdlib {
    use crate::__stddef_size_t_h::size_t;
    use core::ffi::c_char;
    use core::ffi::c_int;
    use core::ffi::c_long;
    use core::ffi::c_uint;
    use core::ffi::c_ulong;
    use core::ffi::c_void;
    extern "C" {
        pub fn __assert_fail(
            __assertion: *const c_char,
            __file: *const c_char,
            __line: c_uint,
            __function: *const c_char,
        ) -> !;
        pub fn __errno_location() -> *mut c_int;
        pub fn open(__file: *const c_char, __oflag: c_int, ...) -> c_int;
        pub fn getrandom(
            __buffer: *mut c_void,
            __length: size_t,
            __flags: c_uint,
        ) -> crate::stdlib::ssize_t;
        pub fn fstat(__fd: c_int, __buf: *mut crate::stdlib::stat) -> c_int;
        pub static mut stdout: *mut FILE;

        pub static mut stderr: *mut FILE;

        pub fn fprintf(__stream: *mut FILE, __format: *const c_char, ...) -> c_int;

        pub fn perror(__s: *const c_char);
        pub fn strtoul(__nptr: *const c_char, __endptr: *mut *mut c_char, __base: c_int)
            -> c_ulong;

        pub fn malloc(__size: size_t) -> *mut c_void;

        pub fn realloc(__ptr: *mut c_void, __size: size_t) -> *mut c_void;

        pub fn free(__ptr: *mut c_void);

        pub fn exit(__status: c_int) -> !;

        pub fn getenv(__name: *const c_char) -> *mut c_char;
        pub fn memcpy(__dest: *mut c_void, __src: *const c_void, __n: size_t) -> *mut c_void;

        pub fn memmove(__dest: *mut c_void, __src: *const c_void, __n: size_t) -> *mut c_void;

        pub fn memset(__s: *mut c_void, __c: c_int, __n: size_t) -> *mut c_void;

        pub fn memcmp(__s1: *const c_void, __s2: *const c_void, __n: size_t) -> c_int;

        pub fn strcpy(__dest: *mut c_char, __src: *const c_char) -> *mut c_char;

        pub fn strrchr(__s: *const c_char, __c: c_int) -> *mut c_char;

        pub fn strlen(__s: *const c_char) -> size_t;
        pub type _IO_marker;

        pub type _IO_codecvt;

        pub type _IO_wide_data;
        pub fn gettimeofday(__tv: *mut crate::stdlib::timeval, __tz: *mut c_void) -> c_int;
        pub fn close(__fd: c_int) -> c_int;

        pub fn read(__fd: c_int, __buf: *mut c_void, __nbytes: size_t) -> crate::stdlib::ssize_t;

        pub fn getpid() -> crate::stdlib::__pid_t;
    }
    pub type FILE = _IO_FILE;
    pub const __S_IFMT: c_int = 0o170000;
    pub const EINTR: c_int = 4;
    pub const O_RDONLY: c_int = 0;
    pub const GRND_NONBLOCK: c_int = 0x1;
    pub const SIZE_MAX: c_ulong = 18446744073709551615;
    pub type uint64_t = crate::stdlib::__uint64_t;
    pub type ssize_t = crate::stdlib::__ssize_t;
    pub type _IO_lock_t = ();

    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]

    pub struct _IO_FILE {
        pub _flags: c_int,
        pub _IO_read_ptr: *mut c_char,
        pub _IO_read_end: *mut c_char,
        pub _IO_read_base: *mut c_char,
        pub _IO_write_base: *mut c_char,
        pub _IO_write_ptr: *mut c_char,
        pub _IO_write_end: *mut c_char,
        pub _IO_buf_base: *mut c_char,
        pub _IO_buf_end: *mut c_char,
        pub _IO_save_base: *mut c_char,
        pub _IO_backup_base: *mut c_char,
        pub _IO_save_end: *mut c_char,
        pub _markers: *mut crate::stdlib::_IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: c_int,
        #[bitfield(name = "_flags2", ty = "c_int", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [c_char; 1],
        pub _old_offset: crate::stdlib::__off_t,
        pub _cur_column: core::ffi::c_ushort,
        pub _vtable_offset: core::ffi::c_schar,
        pub _shortbuf: [c_char; 1],
        pub _lock: *mut c_void,
        pub _offset: crate::stdlib::__off64_t,
        pub _codecvt: *mut crate::stdlib::_IO_codecvt,
        pub _wide_data: *mut crate::stdlib::_IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut c_void,
        pub _prevchain: *mut *mut _IO_FILE,
        pub _mode: c_int,
        pub _unused3: c_int,
        pub _total_written: crate::stdlib::__uint64_t,
        pub _unused2: [c_char; 8],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct stat {
        pub st_dev: crate::stdlib::__dev_t,
        pub st_ino: crate::stdlib::__ino_t,
        pub st_nlink: crate::stdlib::__nlink_t,
        pub st_mode: crate::stdlib::__mode_t,
        pub st_uid: crate::stdlib::__uid_t,
        pub st_gid: crate::stdlib::__gid_t,
        pub __pad0: c_int,
        pub st_rdev: crate::stdlib::__dev_t,
        pub st_size: crate::stdlib::__off_t,
        pub st_blksize: crate::stdlib::__blksize_t,
        pub st_blocks: crate::stdlib::__blkcnt_t,
        pub st_atime: __time_t,
        pub st_atimensec: __syscall_ulong_t,
        pub st_mtime: __time_t,
        pub st_mtimensec: __syscall_ulong_t,
        pub st_ctime: __time_t,
        pub st_ctimensec: __syscall_ulong_t,
        pub __glibc_reserved: [crate::stdlib::__syscall_slong_t; 3],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct timeval {
        pub tv_sec: __time_t,
        pub tv_usec: crate::stdlib::__suseconds_t,
    }
    pub type __uint64_t = u64;

    pub type __dev_t = c_ulong;

    pub type __uid_t = c_uint;

    pub type __gid_t = c_uint;

    pub type __ino_t = c_ulong;

    pub type __mode_t = c_uint;

    pub type __nlink_t = c_ulong;

    pub type __off_t = c_long;

    pub type __off64_t = c_long;

    pub type __pid_t = c_int;

    pub type __time_t = c_long;

    pub type __suseconds_t = c_long;

    pub type __blksize_t = c_long;

    pub type __blkcnt_t = c_long;

    pub type __ssize_t = c_long;

    pub type __syscall_slong_t = c_long;

    pub type __syscall_ulong_t = c_ulong;
}
#[macro_use]
extern crate c2rust_bitfields;

pub mod src {
    pub mod lib {
        pub mod xmlparse;
        pub mod xmlrole;
        pub mod xmltok;
    } // mod lib
    pub mod xmlwf {
        pub mod codepage;
        pub mod readfilemap;
        pub mod xmlfile;
    } // mod xmlwf
} // mod src
