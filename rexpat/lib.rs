#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(raw_ref_op)]
#![feature(register_tool)]
#![register_tool(c2rust)]

pub mod __stdarg_va_list_h {
    pub type va_list = crate::internal::__builtin_va_list;
}
pub mod siphash_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct siphash {
        pub v0: crate::stdlib::uint64_t,
        pub v1: crate::stdlib::uint64_t,
        pub v2: crate::stdlib::uint64_t,
        pub v3: crate::stdlib::uint64_t,
        pub buf: [::core::ffi::c_uchar; 8],
        pub p: *mut ::core::ffi::c_uchar,
        pub c: crate::stdlib::uint64_t,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct sipkey {
        pub k: [crate::stdlib::uint64_t; 2],
    }
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
pub mod __stddef_ptrdiff_t_h {
    pub type ptrdiff_t = isize;
}
pub mod common_h {
    pub const ALLOC_ALWAYS_SUCCEED: ::core::ffi::c_int = -1 as ::core::ffi::c_int;

    pub const REALLOC_ALWAYS_SUCCEED: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
}
pub mod xmltok_impl_c {
    pub const other: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 0;

    pub const other_0: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 0;

    pub const other_1: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 0;

    pub const inName: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 1;

    pub const inName_0: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 1;

    pub const inName_1: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 1;

    pub const inValue: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 2;

    pub const inValue_0: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 2;

    pub const inValue_1: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 2;
}
pub mod xmltok_impl_h {
    pub type C2Rust_Unnamed_3 = ::core::ffi::c_uint;

    pub const BT_NONXML: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 0;

    pub const BT_MALFORM: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 1;

    pub const BT_LT: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 2;

    pub const BT_AMP: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 3;

    pub const BT_RSQB: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 4;

    pub const BT_LEAD2: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 5;

    pub const BT_LEAD3: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 6;

    pub const BT_LEAD4: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 7;

    pub const BT_TRAIL: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 8;

    pub const BT_CR: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 9;

    pub const BT_LF: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 10;

    pub const BT_GT: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 11;

    pub const BT_QUOT: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 12;

    pub const BT_APOS: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 13;

    pub const BT_EQUALS: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 14;

    pub const BT_QUEST: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 15;

    pub const BT_EXCL: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 16;

    pub const BT_SOL: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 17;

    pub const BT_SEMI: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 18;

    pub const BT_NUM: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 19;

    pub const BT_LSQB: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 20;

    pub const BT_S: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 21;

    pub const BT_NMSTRT: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 22;

    pub const BT_COLON_0: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 23;

    pub const BT_HEX: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 24;

    pub const BT_DIGIT: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 25;

    pub const BT_NAME: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 26;

    pub const BT_MINUS: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 27;

    pub const BT_OTHER: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 28;

    pub const BT_NONASCII: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 29;

    pub const BT_PERCNT: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 30;

    pub const BT_LPAR: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 31;

    pub const BT_RPAR: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 32;

    pub const BT_AST: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 33;

    pub const BT_PLUS: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 34;

    pub const BT_COMMA: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 35;

    pub const BT_VERBAR: crate::xmltok_impl_h::C2Rust_Unnamed_3 = 36;
}
pub mod internal {
    pub type __builtin_va_list = [crate::internal::__va_list_tag; 1];

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct __va_list_tag {
        pub gp_offset: ::core::ffi::c_uint,
        pub fp_offset: ::core::ffi::c_uint,
        pub overflow_arg_area: *mut ::core::ffi::c_void,
        pub reg_save_area: *mut ::core::ffi::c_void,
    }

    pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub mod ascii_h {
    pub const ASCII_A: ::core::ffi::c_int = 0x41 as ::core::ffi::c_int;

    pub const ASCII_B: ::core::ffi::c_int = 0x42 as ::core::ffi::c_int;

    pub const ASCII_B_1: ::core::ffi::c_int = 66;

    pub const ASCII_C: ::core::ffi::c_int = 0x43 as ::core::ffi::c_int;

    pub const ASCII_D: ::core::ffi::c_int = 0x44 as ::core::ffi::c_int;

    pub const ASCII_E: ::core::ffi::c_int = 0x45 as ::core::ffi::c_int;

    pub const ASCII_E_1: ::core::ffi::c_int = 69;

    pub const ASCII_F: ::core::ffi::c_int = 0x46 as ::core::ffi::c_int;

    pub const ASCII_F_1: ::core::ffi::c_int = 70;

    pub const ASCII_G: ::core::ffi::c_int = 0x47 as ::core::ffi::c_int;

    pub const ASCII_I: ::core::ffi::c_int = 0x49 as ::core::ffi::c_int;

    pub const ASCII_K: ::core::ffi::c_int = 0x4b as ::core::ffi::c_int;

    pub const ASCII_L: ::core::ffi::c_int = 0x4c as ::core::ffi::c_int;

    pub const ASCII_L_1: ::core::ffi::c_int = 76;

    pub const ASCII_M: ::core::ffi::c_int = 0x4d as ::core::ffi::c_int;

    pub const ASCII_M_1: ::core::ffi::c_int = 77;

    pub const ASCII_N: ::core::ffi::c_int = 0x4e as ::core::ffi::c_int;

    pub const ASCII_O: ::core::ffi::c_int = 0x4f as ::core::ffi::c_int;

    pub const ASCII_P: ::core::ffi::c_int = 0x50 as ::core::ffi::c_int;

    pub const ASCII_Q: ::core::ffi::c_int = 0x51 as ::core::ffi::c_int;

    pub const ASCII_R: ::core::ffi::c_int = 0x52 as ::core::ffi::c_int;

    pub const ASCII_S: ::core::ffi::c_int = 0x53 as ::core::ffi::c_int;

    pub const ASCII_T: ::core::ffi::c_int = 0x54 as ::core::ffi::c_int;

    pub const ASCII_U: ::core::ffi::c_int = 0x55 as ::core::ffi::c_int;

    pub const ASCII_X: ::core::ffi::c_int = 0x58 as ::core::ffi::c_int;

    pub const ASCII_X_1: ::core::ffi::c_int = 88;

    pub const ASCII_Y: ::core::ffi::c_int = 0x59 as ::core::ffi::c_int;

    pub const ASCII_Z: ::core::ffi::c_int = 0x5a as ::core::ffi::c_int;

    pub const ASCII_a: ::core::ffi::c_int = 0x61 as ::core::ffi::c_int;

    pub const ASCII_a_1: ::core::ffi::c_int = 97;

    pub const ASCII_b: ::core::ffi::c_int = 98;

    pub const ASCII_c: ::core::ffi::c_int = 0x63 as ::core::ffi::c_int;

    pub const ASCII_c_1: ::core::ffi::c_int = 99;

    pub const ASCII_d: ::core::ffi::c_int = 100;

    pub const ASCII_e: ::core::ffi::c_int = 0x65 as ::core::ffi::c_int;

    pub const ASCII_e_1: ::core::ffi::c_int = 101;

    pub const ASCII_f: ::core::ffi::c_int = 102;

    pub const ASCII_g: ::core::ffi::c_int = 0x67 as ::core::ffi::c_int;

    pub const ASCII_g_1: ::core::ffi::c_int = 103;

    pub const ASCII_h: ::core::ffi::c_int = 0x68 as ::core::ffi::c_int;

    pub const ASCII_i: ::core::ffi::c_int = 0x69 as ::core::ffi::c_int;

    pub const ASCII_l: ::core::ffi::c_int = 0x6c as ::core::ffi::c_int;

    pub const ASCII_l_1: ::core::ffi::c_int = 108;

    pub const ASCII_m: ::core::ffi::c_int = 0x6d as ::core::ffi::c_int;

    pub const ASCII_m_1: ::core::ffi::c_int = 109;

    pub const ASCII_n: ::core::ffi::c_int = 0x6e as ::core::ffi::c_int;

    pub const ASCII_o: ::core::ffi::c_int = 0x6f as ::core::ffi::c_int;

    pub const ASCII_p: ::core::ffi::c_int = 0x70 as ::core::ffi::c_int;

    pub const ASCII_q: ::core::ffi::c_int = 113;

    pub const ASCII_r: ::core::ffi::c_int = 0x72 as ::core::ffi::c_int;

    pub const ASCII_s: ::core::ffi::c_int = 0x73 as ::core::ffi::c_int;

    pub const ASCII_t: ::core::ffi::c_int = 0x74 as ::core::ffi::c_int;

    pub const ASCII_v: ::core::ffi::c_int = 0x76 as ::core::ffi::c_int;

    pub const ASCII_w: ::core::ffi::c_int = 0x77 as ::core::ffi::c_int;

    pub const ASCII_x: ::core::ffi::c_int = 0x78 as ::core::ffi::c_int;

    pub const ASCII_x_1: ::core::ffi::c_int = 120;

    pub const ASCII_y: ::core::ffi::c_int = 0x79 as ::core::ffi::c_int;

    pub const ASCII_z: ::core::ffi::c_int = 0x7a as ::core::ffi::c_int;

    pub const ASCII_0: ::core::ffi::c_int = 0x30 as ::core::ffi::c_int;

    pub const ASCII_1: ::core::ffi::c_int = 0x31 as ::core::ffi::c_int;

    pub const ASCII_1_1: ::core::ffi::c_int = 49;

    pub const ASCII_2: ::core::ffi::c_int = 0x32 as ::core::ffi::c_int;

    pub const ASCII_2_1: ::core::ffi::c_int = 50;

    pub const ASCII_3: ::core::ffi::c_int = 0x33 as ::core::ffi::c_int;

    pub const ASCII_3_1: ::core::ffi::c_int = 51;

    pub const ASCII_4: ::core::ffi::c_int = 52;

    pub const ASCII_5: ::core::ffi::c_int = 53;

    pub const ASCII_6: ::core::ffi::c_int = 54;

    pub const ASCII_7: ::core::ffi::c_int = 55;

    pub const ASCII_8: ::core::ffi::c_int = 0x38 as ::core::ffi::c_int;

    pub const ASCII_8_1: ::core::ffi::c_int = 56;

    pub const ASCII_9: ::core::ffi::c_int = 0x39 as ::core::ffi::c_int;

    pub const ASCII_9_1: ::core::ffi::c_int = 57;

    pub const ASCII_SPACE: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

    pub const ASCII_EXCL: ::core::ffi::c_int = 0x21 as ::core::ffi::c_int;

    pub const ASCII_QUOT: ::core::ffi::c_int = 0x22 as ::core::ffi::c_int;

    pub const ASCII_AMP: ::core::ffi::c_int = 0x26 as ::core::ffi::c_int;

    pub const ASCII_APOS: ::core::ffi::c_int = 0x27 as ::core::ffi::c_int;

    pub const ASCII_MINUS: ::core::ffi::c_int = 0x2d as ::core::ffi::c_int;

    pub const ASCII_PERIOD: ::core::ffi::c_int = 0x2e as ::core::ffi::c_int;

    pub const ASCII_COLON: ::core::ffi::c_int = 0x3a as ::core::ffi::c_int;

    pub const ASCII_LT: ::core::ffi::c_int = 0x3c as ::core::ffi::c_int;

    pub const ASCII_EQUALS: ::core::ffi::c_int = 0x3d as ::core::ffi::c_int;

    pub const ASCII_GT: ::core::ffi::c_int = 0x3e as ::core::ffi::c_int;

    pub const ASCII_LSQB: ::core::ffi::c_int = 0x5b as ::core::ffi::c_int;

    pub const ASCII_UNDERSCORE: ::core::ffi::c_int = 0x5f as ::core::ffi::c_int;

    pub const ASCII_LPAREN: ::core::ffi::c_int = 0x28 as ::core::ffi::c_int;

    pub const ASCII_SLASH: ::core::ffi::c_int = 0x2f as ::core::ffi::c_int;

    pub const ASCII_HASH: ::core::ffi::c_int = 0x23 as ::core::ffi::c_int;

    pub const ASCII_PIPE: ::core::ffi::c_int = 0x7c as ::core::ffi::c_int;

    pub const ASCII_COMMA: ::core::ffi::c_int = 0x2c as ::core::ffi::c_int;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod expat_config_h {
    pub const XML_CONTEXT_BYTES: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
}
pub mod limits_h {
    pub const INT_MAX: ::core::ffi::c_int = crate::internal::__INT_MAX__;

    pub const UINT_MAX: ::core::ffi::c_uint = (crate::internal::__INT_MAX__ as ::core::ffi::c_uint)
        .wrapping_mul(2 as ::core::ffi::c_uint)
        .wrapping_add(1 as ::core::ffi::c_uint);
}
pub mod internal_h {
    pub const EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT:
        ::core::ffi::c_float = 100.0f32;

    pub const EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT:
        ::core::ffi::c_int = 8388608 as ::core::ffi::c_int;

    pub const EXPAT_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT: ::core::ffi::c_float = 100.0f32;

    pub const EXPAT_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT: ::core::ffi::c_int =
        67108864 as ::core::ffi::c_int;

    pub const EXPAT_MALLOC_ALIGNMENT: usize = ::core::mem::size_of::<::core::ffi::c_longlong>();

    pub const EXPAT_MALLOC_PADDING: usize = (::core::mem::size_of::<::core::ffi::c_longlong>()
        as usize)
        .wrapping_sub(::core::mem::size_of::<crate::__stddef_size_t_h::size_t>() as usize);
}
pub mod expat_h {
    pub use crate::src::lib::xmlparse::XML_ParserStruct;

    pub type XML_Parser = *mut XML_ParserStruct;

    pub type XML_Bool = ::core::ffi::c_uchar;

    pub const XML_TRUE: crate::expat_h::XML_Bool =
        1 as ::core::ffi::c_int as crate::expat_h::XML_Bool;

    pub const XML_FALSE: crate::expat_h::XML_Bool =
        0 as ::core::ffi::c_int as crate::expat_h::XML_Bool;

    pub type XML_Status = ::core::ffi::c_uint;

    pub const XML_STATUS_ERROR: crate::expat_h::XML_Status = 0;

    pub const XML_STATUS_OK: crate::expat_h::XML_Status = 1;

    pub const XML_STATUS_SUSPENDED: crate::expat_h::XML_Status = 2;

    pub type XML_Error = ::core::ffi::c_uint;

    pub const XML_ERROR_NONE: crate::expat_h::XML_Error = 0;

    pub const XML_ERROR_NO_MEMORY: crate::expat_h::XML_Error = 1;

    pub const XML_ERROR_SYNTAX: crate::expat_h::XML_Error = 2;

    pub const XML_ERROR_NO_ELEMENTS: crate::expat_h::XML_Error = 3;

    pub const XML_ERROR_INVALID_TOKEN: crate::expat_h::XML_Error = 4;

    pub const XML_ERROR_UNCLOSED_TOKEN: crate::expat_h::XML_Error = 5;

    pub const XML_ERROR_PARTIAL_CHAR: crate::expat_h::XML_Error = 6;

    pub const XML_ERROR_TAG_MISMATCH: crate::expat_h::XML_Error = 7;

    pub const XML_ERROR_DUPLICATE_ATTRIBUTE: crate::expat_h::XML_Error = 8;

    pub const XML_ERROR_JUNK_AFTER_DOC_ELEMENT: crate::expat_h::XML_Error = 9;

    pub const XML_ERROR_PARAM_ENTITY_REF: crate::expat_h::XML_Error = 10;

    pub const XML_ERROR_UNDEFINED_ENTITY: crate::expat_h::XML_Error = 11;

    pub const XML_ERROR_RECURSIVE_ENTITY_REF: crate::expat_h::XML_Error = 12;

    pub const XML_ERROR_ASYNC_ENTITY: crate::expat_h::XML_Error = 13;

    pub const XML_ERROR_BAD_CHAR_REF: crate::expat_h::XML_Error = 14;

    pub const XML_ERROR_BINARY_ENTITY_REF: crate::expat_h::XML_Error = 15;

    pub const XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF: crate::expat_h::XML_Error = 16;

    pub const XML_ERROR_MISPLACED_XML_PI: crate::expat_h::XML_Error = 17;

    pub const XML_ERROR_UNKNOWN_ENCODING: crate::expat_h::XML_Error = 18;

    pub const XML_ERROR_INCORRECT_ENCODING: crate::expat_h::XML_Error = 19;

    pub const XML_ERROR_UNCLOSED_CDATA_SECTION: crate::expat_h::XML_Error = 20;

    pub const XML_ERROR_EXTERNAL_ENTITY_HANDLING: crate::expat_h::XML_Error = 21;

    pub const XML_ERROR_NOT_STANDALONE: crate::expat_h::XML_Error = 22;

    pub const XML_ERROR_UNEXPECTED_STATE: crate::expat_h::XML_Error = 23;

    pub const XML_ERROR_ENTITY_DECLARED_IN_PE: crate::expat_h::XML_Error = 24;

    pub const XML_ERROR_FEATURE_REQUIRES_XML_DTD: crate::expat_h::XML_Error = 25;

    pub const XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING: crate::expat_h::XML_Error = 26;

    pub const XML_ERROR_UNBOUND_PREFIX: crate::expat_h::XML_Error = 27;

    pub const XML_ERROR_UNDECLARING_PREFIX: crate::expat_h::XML_Error = 28;

    pub const XML_ERROR_INCOMPLETE_PE: crate::expat_h::XML_Error = 29;

    pub const XML_ERROR_XML_DECL: crate::expat_h::XML_Error = 30;

    pub const XML_ERROR_TEXT_DECL: crate::expat_h::XML_Error = 31;

    pub const XML_ERROR_PUBLICID: crate::expat_h::XML_Error = 32;

    pub const XML_ERROR_SUSPENDED: crate::expat_h::XML_Error = 33;

    pub const XML_ERROR_NOT_SUSPENDED: crate::expat_h::XML_Error = 34;

    pub const XML_ERROR_ABORTED: crate::expat_h::XML_Error = 35;

    pub const XML_ERROR_FINISHED: crate::expat_h::XML_Error = 36;

    pub const XML_ERROR_SUSPEND_PE: crate::expat_h::XML_Error = 37;

    pub const XML_ERROR_RESERVED_PREFIX_XML: crate::expat_h::XML_Error = 38;

    pub const XML_ERROR_RESERVED_PREFIX_XMLNS: crate::expat_h::XML_Error = 39;

    pub const XML_ERROR_RESERVED_NAMESPACE_URI: crate::expat_h::XML_Error = 40;

    pub const XML_ERROR_INVALID_ARGUMENT: crate::expat_h::XML_Error = 41;

    pub const XML_ERROR_NO_BUFFER: crate::expat_h::XML_Error = 42;

    pub const XML_ERROR_AMPLIFICATION_LIMIT_BREACH: crate::expat_h::XML_Error = 43;

    pub const XML_ERROR_NOT_STARTED: crate::expat_h::XML_Error = 44;

    pub type XML_Content_Type = ::core::ffi::c_uint;

    pub const XML_CTYPE_EMPTY: crate::expat_h::XML_Content_Type = 1;

    pub const XML_CTYPE_ANY: crate::expat_h::XML_Content_Type = 2;

    pub const XML_CTYPE_MIXED: crate::expat_h::XML_Content_Type = 3;

    pub const XML_CTYPE_NAME: crate::expat_h::XML_Content_Type = 4;

    pub const XML_CTYPE_CHOICE: crate::expat_h::XML_Content_Type = 5;

    pub const XML_CTYPE_SEQ: crate::expat_h::XML_Content_Type = 6;

    pub type XML_Content_Quant = ::core::ffi::c_uint;

    pub const XML_CQUANT_NONE: crate::expat_h::XML_Content_Quant = 0;

    pub const XML_CQUANT_OPT: crate::expat_h::XML_Content_Quant = 1;

    pub const XML_CQUANT_REP: crate::expat_h::XML_Content_Quant = 2;

    pub const XML_CQUANT_PLUS: crate::expat_h::XML_Content_Quant = 3;

    pub type XML_Content = crate::expat_h::XML_cp;

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct XML_cp {
        pub type_0: crate::expat_h::XML_Content_Type,
        pub quant: crate::expat_h::XML_Content_Quant,
        pub name: *mut crate::expat_external_h::XML_Char,
        pub numchildren: ::core::ffi::c_uint,
        pub children: *mut crate::expat_h::XML_Content,
    }

    pub type XML_ElementDeclHandler = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const crate::expat_external_h::XML_Char,
            *mut crate::expat_h::XML_Content,
        ) -> (),
    >;

    pub type XML_AttlistDeclHandler = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const crate::expat_external_h::XML_Char,
            *const crate::expat_external_h::XML_Char,
            *const crate::expat_external_h::XML_Char,
            *const crate::expat_external_h::XML_Char,
            ::core::ffi::c_int,
        ) -> (),
    >;

    pub type XML_XmlDeclHandler = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const crate::expat_external_h::XML_Char,
            *const crate::expat_external_h::XML_Char,
            ::core::ffi::c_int,
        ) -> (),
    >;

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct XML_Memory_Handling_Suite {
        pub malloc_fcn: Option<
            unsafe extern "C" fn(crate::__stddef_size_t_h::size_t) -> *mut ::core::ffi::c_void,
        >,
        pub realloc_fcn: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                crate::__stddef_size_t_h::size_t,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub free_fcn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    }

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

    pub type XML_UnparsedEntityDeclHandler = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const crate::expat_external_h::XML_Char,
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

    pub type XML_ExternalEntityRefHandler = Option<
        unsafe extern "C" fn(
            crate::expat_h::XML_Parser,
            *const crate::expat_external_h::XML_Char,
            *const crate::expat_external_h::XML_Char,
            *const crate::expat_external_h::XML_Char,
            *const crate::expat_external_h::XML_Char,
        ) -> ::core::ffi::c_int,
    >;

    pub type XML_SkippedEntityHandler = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const crate::expat_external_h::XML_Char,
            ::core::ffi::c_int,
        ) -> (),
    >;

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
            *const crate::expat_external_h::XML_Char,
            *mut crate::expat_h::XML_Encoding,
        ) -> ::core::ffi::c_int,
    >;

    pub type XML_Parsing = ::core::ffi::c_uint;

    pub const XML_INITIALIZED: crate::expat_h::XML_Parsing = 0;

    pub const XML_PARSING: crate::expat_h::XML_Parsing = 1;

    pub const XML_FINISHED: crate::expat_h::XML_Parsing = 2;

    pub const XML_SUSPENDED: crate::expat_h::XML_Parsing = 3;

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct XML_ParsingStatus {
        pub parsing: crate::expat_h::XML_Parsing,
        pub finalBuffer: crate::expat_h::XML_Bool,
    }

    pub type XML_ParamEntityParsing = ::core::ffi::c_uint;

    pub const XML_PARAM_ENTITY_PARSING_NEVER: crate::expat_h::XML_ParamEntityParsing = 0;

    pub const XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE: crate::expat_h::XML_ParamEntityParsing =
        1;

    pub const XML_PARAM_ENTITY_PARSING_ALWAYS: crate::expat_h::XML_ParamEntityParsing = 2;

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct XML_Expat_Version {
        pub major: ::core::ffi::c_int,
        pub minor: ::core::ffi::c_int,
        pub micro: ::core::ffi::c_int,
    }

    pub type XML_FeatureEnum = ::core::ffi::c_uint;

    pub const XML_FEATURE_END: crate::expat_h::XML_FeatureEnum = 0;

    pub const XML_FEATURE_UNICODE: crate::expat_h::XML_FeatureEnum = 1;

    pub const XML_FEATURE_UNICODE_WCHAR_T: crate::expat_h::XML_FeatureEnum = 2;

    pub const XML_FEATURE_DTD: crate::expat_h::XML_FeatureEnum = 3;

    pub const XML_FEATURE_CONTEXT_BYTES: crate::expat_h::XML_FeatureEnum = 4;

    pub const XML_FEATURE_MIN_SIZE: crate::expat_h::XML_FeatureEnum = 5;

    pub const XML_FEATURE_SIZEOF_XML_CHAR: crate::expat_h::XML_FeatureEnum = 6;

    pub const XML_FEATURE_SIZEOF_XML_LCHAR: crate::expat_h::XML_FeatureEnum = 7;

    pub const XML_FEATURE_NS: crate::expat_h::XML_FeatureEnum = 8;

    pub const XML_FEATURE_LARGE_SIZE: crate::expat_h::XML_FeatureEnum = 9;

    pub const XML_FEATURE_ATTR_INFO: crate::expat_h::XML_FeatureEnum = 10;

    pub const XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT:
        crate::expat_h::XML_FeatureEnum = 11;

    pub const XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT:
        crate::expat_h::XML_FeatureEnum = 12;

    pub const XML_FEATURE_GE: crate::expat_h::XML_FeatureEnum = 13;

    pub const XML_FEATURE_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT:
        crate::expat_h::XML_FeatureEnum = 14;

    pub const XML_FEATURE_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT:
        crate::expat_h::XML_FeatureEnum = 15;

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct XML_Feature {
        pub feature: crate::expat_h::XML_FeatureEnum,
        pub name: *const crate::expat_external_h::XML_LChar,
        pub value: ::core::ffi::c_long,
    }

    pub const XML_MAJOR_VERSION: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

    pub const XML_MINOR_VERSION: ::core::ffi::c_int = 7 as ::core::ffi::c_int;

    pub const XML_MICRO_VERSION: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
}
pub mod stdlib {
    extern "C" {
        pub fn __assert_fail(
            __assertion: *const ::core::ffi::c_char,
            __file: *const ::core::ffi::c_char,
            __line: ::core::ffi::c_uint,
            __function: *const ::core::ffi::c_char,
        ) -> !;

        pub fn __assert_single_arg(_: bool) -> bool;
        pub fn __errno_location() -> *mut ::core::ffi::c_int;
        pub fn open(
            __file: *const ::core::ffi::c_char,
            __oflag: ::core::ffi::c_int,
            ...
        ) -> ::core::ffi::c_int;
        pub fn _setjmp(__env: *mut crate::stdlib::__jmp_buf_tag) -> ::core::ffi::c_int;

        pub fn longjmp(__env: *mut crate::stdlib::__jmp_buf_tag, __val: ::core::ffi::c_int) -> !;
        pub fn getrandom(
            __buffer: *mut ::core::ffi::c_void,
            __length: crate::__stddef_size_t_h::size_t,
            __flags: ::core::ffi::c_uint,
        ) -> crate::stdlib::ssize_t;
        pub static mut stderr: *mut crate::stdlib::FILE;

        pub fn fprintf(
            __stream: *mut crate::stdlib::FILE,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;

        pub fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;

        pub fn snprintf(
            __s: *mut ::core::ffi::c_char,
            __maxlen: crate::__stddef_size_t_h::size_t,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;

        pub fn vsnprintf(
            __s: *mut ::core::ffi::c_char,
            __maxlen: crate::__stddef_size_t_h::size_t,
            __format: *const ::core::ffi::c_char,
            __arg: ::core::ffi::VaList,
        ) -> ::core::ffi::c_int;
        pub fn strtoul(
            __nptr: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
            __base: ::core::ffi::c_int,
        ) -> ::core::ffi::c_ulong;

        pub fn malloc(__size: crate::__stddef_size_t_h::size_t) -> *mut ::core::ffi::c_void;

        pub fn calloc(
            __nmemb: crate::__stddef_size_t_h::size_t,
            __size: crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void;

        pub fn realloc(
            __ptr: *mut ::core::ffi::c_void,
            __size: crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void;

        pub fn free(__ptr: *mut ::core::ffi::c_void);

        pub fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void;

        pub fn memmove(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void;

        pub fn memset(
            __s: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void;

        pub fn memcmp(
            __s1: *const ::core::ffi::c_void,
            __s2: *const ::core::ffi::c_void,
            __n: crate::__stddef_size_t_h::size_t,
        ) -> ::core::ffi::c_int;

        pub fn memchr(
            __s: *const ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void;

        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;

        pub fn strncmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
            __n: crate::__stddef_size_t_h::size_t,
        ) -> ::core::ffi::c_int;

        pub fn strlen(__s: *const ::core::ffi::c_char) -> crate::__stddef_size_t_h::size_t;
        pub type _IO_marker;

        pub type _IO_codecvt;

        pub type _IO_wide_data;
        pub fn gettimeofday(
            __tv: *mut crate::stdlib::timeval,
            __tz: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
        pub fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;

        pub fn read(
            __fd: ::core::ffi::c_int,
            __buf: *mut ::core::ffi::c_void,
            __nbytes: crate::__stddef_size_t_h::size_t,
        ) -> crate::stdlib::ssize_t;

        pub fn getpid() -> crate::stdlib::__pid_t;
    }
    pub type FILE = crate::stdlib::_IO_FILE;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __sigset_t {
        pub __val: [::core::ffi::c_ulong; 16],
    }
    pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 37] = unsafe {
        ::core::mem::transmute::<[u8; 37], [::core::ffi::c_char; 37]>(
            *b"void test_alloc_realloc_buffer(void)\0",
        )
    };

    pub const __ASSERT_FUNCTION_1: [::core::ffi::c_char; 76] = unsafe {
        ::core::mem::transmute::<[u8; 76], [::core::ffi::c_char; 76]>(
            *b"enum XML_Status _XML_Parse_SINGLE_BYTES(XML_Parser, const char *, int, int)\0",
        )
    };

    pub const __ASSERT_FUNCTION_2: [::core::ffi::c_char; 29] = unsafe {
        ::core::mem::transmute::<[u8; 29], [::core::ffi::c_char; 29]>(
            *b"void test_misc_version(void)\0",
        )
    };

    pub const __ASSERT_FUNCTION_3: [::core::ffi::c_char; 37] = unsafe {
        ::core::mem::transmute::<[u8; 37], [::core::ffi::c_char; 37]>(
            *b"void test_nsalloc_parse_buffer(void)\0",
        )
    };
    pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

    pub const ENOMEM: ::core::ffi::c_int = 12 as ::core::ffi::c_int;

    pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
    pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    pub type jmp_buf = [crate::stdlib::__jmp_buf_tag; 1];
    pub const GRND_NONBLOCK: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    pub type __jmp_buf = [::core::ffi::c_long; 8];
    pub type intptr_t = isize;

    pub type uintptr_t = usize;

    pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
    pub type uint32_t = crate::stdlib::__uint32_t;

    pub type uint64_t = crate::stdlib::__uint64_t;
    pub type ssize_t = crate::stdlib::__ssize_t;
    pub type _IO_lock_t = ();

    #[derive(Copy, Clone, BitfieldStruct)]
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
        pub _markers: *mut crate::stdlib::_IO_marker,
        pub _chain: *mut crate::stdlib::_IO_FILE,
        pub _fileno: ::core::ffi::c_int,
        #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [::core::ffi::c_char; 1],
        pub _old_offset: crate::stdlib::__off_t,
        pub _cur_column: ::core::ffi::c_ushort,
        pub _vtable_offset: ::core::ffi::c_schar,
        pub _shortbuf: [::core::ffi::c_char; 1],
        pub _lock: *mut ::core::ffi::c_void,
        pub _offset: crate::stdlib::__off64_t,
        pub _codecvt: *mut crate::stdlib::_IO_codecvt,
        pub _wide_data: *mut crate::stdlib::_IO_wide_data,
        pub _freeres_list: *mut crate::stdlib::_IO_FILE,
        pub _freeres_buf: *mut ::core::ffi::c_void,
        pub _prevchain: *mut *mut crate::stdlib::_IO_FILE,
        pub _mode: ::core::ffi::c_int,
        pub _unused3: ::core::ffi::c_int,
        pub _total_written: crate::stdlib::__uint64_t,
        pub _unused2: [::core::ffi::c_char; 8],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __jmp_buf_tag {
        pub __jmpbuf: crate::stdlib::__jmp_buf,
        pub __mask_was_saved: ::core::ffi::c_int,
        pub __saved_mask: crate::stdlib::__sigset_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct timeval {
        pub tv_sec: crate::stdlib::__time_t,
        pub tv_usec: crate::stdlib::__suseconds_t,
    }
    pub type __uint32_t = u32;

    pub type __uint64_t = u64;

    pub type __off_t = ::core::ffi::c_long;

    pub type __off64_t = ::core::ffi::c_long;

    pub type __pid_t = ::core::ffi::c_int;

    pub type __time_t = ::core::ffi::c_long;

    pub type __suseconds_t = ::core::ffi::c_long;

    pub type __ssize_t = ::core::ffi::c_long;
}
#[macro_use]
extern crate c2rust_bitfields;

pub mod src {
    pub mod lib {
        pub mod xmlparse;
        pub mod xmlrole;
        pub mod xmltok;
    } // mod lib
    pub mod tests {
        pub mod acc_tests;
        pub mod alloc_tests;
        pub mod basic_tests;
        pub mod chardata;
        pub mod common;
        pub mod dummy;
        pub mod handlers;
        pub mod memcheck;
        pub mod minicheck;
        pub mod misc_tests;
        pub mod ns_tests;
        pub mod nsalloc_tests;
        pub mod structdata;
    } // mod tests
} // mod src
