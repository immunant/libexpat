#[c2rust::header_src = "/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/lib/expat_external.h:50"]
pub mod expat_external_h {
    #[c2rust::src_loc = "158:1"]
    pub type XML_Size = ::core::ffi::c_ulong;
}
#[c2rust::header_src = "/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/lib/xmltok.h:51"]
pub mod xmltok_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "146:9"]
    pub struct position {
        pub lineNumber: XML_Size,
        pub columnNumber: XML_Size,
    }
    #[c2rust::src_loc = "146:1"]
    pub type POSITION = position;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "152:9"]
    pub struct ATTRIBUTE {
        pub name: *const ::core::ffi::c_char,
        pub valuePtr: *const ::core::ffi::c_char,
        pub valueEnd: *const ::core::ffi::c_char,
        pub normalized: ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "172:1"]
    pub struct encoding {
        pub scanners: [SCANNER; 4],
        pub literalScanners: [SCANNER; 2],
        pub nameMatchesAscii: Option<
            unsafe extern "C" fn(
                *const ENCODING,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub nameLength: Option<
            unsafe extern "C" fn(*const ENCODING, *const ::core::ffi::c_char) -> ::core::ffi::c_int,
        >,
        pub skipS: Option<
            unsafe extern "C" fn(
                *const ENCODING,
                *const ::core::ffi::c_char,
            ) -> *const ::core::ffi::c_char,
        >,
        pub getAtts: Option<
            unsafe extern "C" fn(
                *const ENCODING,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut ATTRIBUTE,
            ) -> ::core::ffi::c_int,
        >,
        pub charRefNumber: Option<
            unsafe extern "C" fn(*const ENCODING, *const ::core::ffi::c_char) -> ::core::ffi::c_int,
        >,
        pub predefinedEntityName: Option<
            unsafe extern "C" fn(
                *const ENCODING,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub updatePosition: Option<
            unsafe extern "C" fn(
                *const ENCODING,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut POSITION,
            ) -> (),
        >,
        pub isPublicId: Option<
            unsafe extern "C" fn(
                *const ENCODING,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub utf8Convert: Option<
            unsafe extern "C" fn(
                *const ENCODING,
                *mut *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
                *const ::core::ffi::c_char,
            ) -> XML_Convert_Result,
        >,
        pub utf16Convert: Option<
            unsafe extern "C" fn(
                *const ENCODING,
                *mut *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_ushort,
                *const ::core::ffi::c_ushort,
            ) -> XML_Convert_Result,
        >,
        pub minBytesPerChar: ::core::ffi::c_int,
        pub isUtf8: ::core::ffi::c_char,
        pub isUtf16: ::core::ffi::c_char,
    }
    #[c2rust::src_loc = "160:1"]
    pub type ENCODING = encoding;
    #[c2rust::src_loc = "165:1"]
    pub type XML_Convert_Result = ::core::ffi::c_uint;
    #[c2rust::src_loc = "168:3"]
    pub const XML_CONVERT_OUTPUT_EXHAUSTED: XML_Convert_Result = 2;
    #[c2rust::src_loc = "167:3"]
    pub const XML_CONVERT_INPUT_INCOMPLETE: XML_Convert_Result = 1;
    #[c2rust::src_loc = "166:3"]
    pub const XML_CONVERT_COMPLETED: XML_Convert_Result = 0;
    #[c2rust::src_loc = "162:1"]
    pub type SCANNER = Option<
        unsafe extern "C" fn(
            *const ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >;
    #[c2rust::src_loc = "51:11"]
    pub const XML_TOK_NONE: ::core::ffi::c_int = -4;
    #[c2rust::src_loc = "76:11"]
    pub const XML_TOK_PI: ::core::ffi::c_int = 11;
    #[c2rust::src_loc = "77:11"]
    pub const XML_TOK_XML_DECL: ::core::ffi::c_int = 12;
    #[c2rust::src_loc = "78:11"]
    pub const XML_TOK_COMMENT: ::core::ffi::c_int = 13;
    #[c2rust::src_loc = "79:11"]
    pub const XML_TOK_BOM: ::core::ffi::c_int = 14;
    #[c2rust::src_loc = "82:11"]
    pub const XML_TOK_PROLOG_S: ::core::ffi::c_int = 15;
    #[c2rust::src_loc = "83:11"]
    pub const XML_TOK_DECL_OPEN: ::core::ffi::c_int = 16;
    #[c2rust::src_loc = "84:11"]
    pub const XML_TOK_DECL_CLOSE: ::core::ffi::c_int = 17;
    #[c2rust::src_loc = "85:11"]
    pub const XML_TOK_NAME: ::core::ffi::c_int = 18;
    #[c2rust::src_loc = "86:11"]
    pub const XML_TOK_NMTOKEN: ::core::ffi::c_int = 19;
    #[c2rust::src_loc = "87:11"]
    pub const XML_TOK_POUND_NAME: ::core::ffi::c_int = 20;
    #[c2rust::src_loc = "88:11"]
    pub const XML_TOK_OR: ::core::ffi::c_int = 21;
    #[c2rust::src_loc = "89:11"]
    pub const XML_TOK_PERCENT: ::core::ffi::c_int = 22;
    #[c2rust::src_loc = "90:11"]
    pub const XML_TOK_OPEN_PAREN: ::core::ffi::c_int = 23;
    #[c2rust::src_loc = "91:11"]
    pub const XML_TOK_CLOSE_PAREN: ::core::ffi::c_int = 24;
    #[c2rust::src_loc = "92:11"]
    pub const XML_TOK_OPEN_BRACKET: ::core::ffi::c_int = 25;
    #[c2rust::src_loc = "93:11"]
    pub const XML_TOK_CLOSE_BRACKET: ::core::ffi::c_int = 26;
    #[c2rust::src_loc = "94:11"]
    pub const XML_TOK_LITERAL: ::core::ffi::c_int = 27;
    #[c2rust::src_loc = "95:11"]
    pub const XML_TOK_PARAM_ENTITY_REF: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
    #[c2rust::src_loc = "96:11"]
    pub const XML_TOK_INSTANCE_START: ::core::ffi::c_int = 29;
    #[c2rust::src_loc = "99:11"]
    pub const XML_TOK_NAME_QUESTION: ::core::ffi::c_int = 30;
    #[c2rust::src_loc = "100:11"]
    pub const XML_TOK_NAME_ASTERISK: ::core::ffi::c_int = 31;
    #[c2rust::src_loc = "101:11"]
    pub const XML_TOK_NAME_PLUS: ::core::ffi::c_int = 32;
    #[c2rust::src_loc = "102:11"]
    pub const XML_TOK_COND_SECT_OPEN: ::core::ffi::c_int = 33;
    #[c2rust::src_loc = "103:11"]
    pub const XML_TOK_COND_SECT_CLOSE: ::core::ffi::c_int = 34;
    #[c2rust::src_loc = "104:11"]
    pub const XML_TOK_CLOSE_PAREN_QUESTION: ::core::ffi::c_int = 35;
    #[c2rust::src_loc = "105:11"]
    pub const XML_TOK_CLOSE_PAREN_ASTERISK: ::core::ffi::c_int = 36;
    #[c2rust::src_loc = "106:11"]
    pub const XML_TOK_CLOSE_PAREN_PLUS: ::core::ffi::c_int = 37;
    #[c2rust::src_loc = "107:11"]
    pub const XML_TOK_COMMA: ::core::ffi::c_int = 38;
    #[c2rust::src_loc = "118:11"]
    pub const XML_TOK_PREFIXED_NAME: ::core::ffi::c_int = 41;
    use super::expat_external_h::XML_Size;
}
#[c2rust::header_src = "/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/lib/xmlrole.h:51"]
pub mod xmlrole_h {
    #[c2rust::src_loc = "45:1"]
    pub type C2RustUnnamed_2 = ::core::ffi::c_int;
    #[c2rust::src_loc = "109:3"]
    pub const XML_ROLE_PARAM_ENTITY_REF: C2RustUnnamed_2 = 60;
    #[c2rust::src_loc = "107:3"]
    pub const XML_ROLE_INNER_PARAM_ENTITY_REF: C2RustUnnamed_2 = 59;
    #[c2rust::src_loc = "106:3"]
    pub const XML_ROLE_IGNORE_SECT: C2RustUnnamed_2 = 58;
    #[c2rust::src_loc = "105:3"]
    pub const XML_ROLE_TEXT_DECL: C2RustUnnamed_2 = 57;
    #[c2rust::src_loc = "103:3"]
    pub const XML_ROLE_COMMENT: C2RustUnnamed_2 = 56;
    #[c2rust::src_loc = "102:3"]
    pub const XML_ROLE_PI: C2RustUnnamed_2 = 55;
    #[c2rust::src_loc = "101:3"]
    pub const XML_ROLE_CONTENT_ELEMENT_PLUS: C2RustUnnamed_2 = 54;
    #[c2rust::src_loc = "100:3"]
    pub const XML_ROLE_CONTENT_ELEMENT_OPT: C2RustUnnamed_2 = 53;
    #[c2rust::src_loc = "99:3"]
    pub const XML_ROLE_CONTENT_ELEMENT_REP: C2RustUnnamed_2 = 52;
    #[c2rust::src_loc = "98:3"]
    pub const XML_ROLE_CONTENT_ELEMENT: C2RustUnnamed_2 = 51;
    #[c2rust::src_loc = "97:3"]
    pub const XML_ROLE_GROUP_SEQUENCE: C2RustUnnamed_2 = 50;
    #[c2rust::src_loc = "96:3"]
    pub const XML_ROLE_GROUP_CHOICE: C2RustUnnamed_2 = 49;
    #[c2rust::src_loc = "95:3"]
    pub const XML_ROLE_GROUP_CLOSE_PLUS: C2RustUnnamed_2 = 48;
    #[c2rust::src_loc = "94:3"]
    pub const XML_ROLE_GROUP_CLOSE_OPT: C2RustUnnamed_2 = 47;
    #[c2rust::src_loc = "93:3"]
    pub const XML_ROLE_GROUP_CLOSE_REP: C2RustUnnamed_2 = 46;
    #[c2rust::src_loc = "92:3"]
    pub const XML_ROLE_GROUP_CLOSE: C2RustUnnamed_2 = 45;
    #[c2rust::src_loc = "91:3"]
    pub const XML_ROLE_GROUP_OPEN: C2RustUnnamed_2 = 44;
    #[c2rust::src_loc = "90:3"]
    pub const XML_ROLE_CONTENT_PCDATA: C2RustUnnamed_2 = 43;
    #[c2rust::src_loc = "89:3"]
    pub const XML_ROLE_CONTENT_EMPTY: C2RustUnnamed_2 = 42;
    #[c2rust::src_loc = "88:3"]
    pub const XML_ROLE_CONTENT_ANY: C2RustUnnamed_2 = 41;
    #[c2rust::src_loc = "87:3"]
    pub const XML_ROLE_ELEMENT_NAME: C2RustUnnamed_2 = 40;
    #[c2rust::src_loc = "86:3"]
    pub const XML_ROLE_ELEMENT_NONE: C2RustUnnamed_2 = 39;
    #[c2rust::src_loc = "85:3"]
    pub const XML_ROLE_FIXED_ATTRIBUTE_VALUE: C2RustUnnamed_2 = 38;
    #[c2rust::src_loc = "84:3"]
    pub const XML_ROLE_DEFAULT_ATTRIBUTE_VALUE: C2RustUnnamed_2 = 37;
    #[c2rust::src_loc = "83:3"]
    pub const XML_ROLE_REQUIRED_ATTRIBUTE_VALUE: C2RustUnnamed_2 = 36;
    #[c2rust::src_loc = "82:3"]
    pub const XML_ROLE_IMPLIED_ATTRIBUTE_VALUE: C2RustUnnamed_2 = 35;
    #[c2rust::src_loc = "81:3"]
    pub const XML_ROLE_ATTLIST_ELEMENT_NAME: C2RustUnnamed_2 = 34;
    #[c2rust::src_loc = "80:3"]
    pub const XML_ROLE_ATTLIST_NONE: C2RustUnnamed_2 = 33;
    #[c2rust::src_loc = "79:3"]
    pub const XML_ROLE_ATTRIBUTE_NOTATION_VALUE: C2RustUnnamed_2 = 32;
    #[c2rust::src_loc = "78:3"]
    pub const XML_ROLE_ATTRIBUTE_ENUM_VALUE: C2RustUnnamed_2 = 31;
    #[c2rust::src_loc = "77:3"]
    pub const XML_ROLE_ATTRIBUTE_TYPE_NMTOKENS: C2RustUnnamed_2 = 30;
    #[c2rust::src_loc = "76:3"]
    pub const XML_ROLE_ATTRIBUTE_TYPE_NMTOKEN: C2RustUnnamed_2 = 29;
    #[c2rust::src_loc = "75:3"]
    pub const XML_ROLE_ATTRIBUTE_TYPE_ENTITIES: C2RustUnnamed_2 = 28;
    #[c2rust::src_loc = "74:3"]
    pub const XML_ROLE_ATTRIBUTE_TYPE_ENTITY: C2RustUnnamed_2 = 27;
    #[c2rust::src_loc = "73:3"]
    pub const XML_ROLE_ATTRIBUTE_TYPE_IDREFS: C2RustUnnamed_2 = 26;
    #[c2rust::src_loc = "72:3"]
    pub const XML_ROLE_ATTRIBUTE_TYPE_IDREF: C2RustUnnamed_2 = 25;
    #[c2rust::src_loc = "71:3"]
    pub const XML_ROLE_ATTRIBUTE_TYPE_ID: C2RustUnnamed_2 = 24;
    #[c2rust::src_loc = "70:3"]
    pub const XML_ROLE_ATTRIBUTE_TYPE_CDATA: C2RustUnnamed_2 = 23;
    #[c2rust::src_loc = "69:3"]
    pub const XML_ROLE_ATTRIBUTE_NAME: C2RustUnnamed_2 = 22;
    #[c2rust::src_loc = "68:3"]
    pub const XML_ROLE_NOTATION_PUBLIC_ID: C2RustUnnamed_2 = 21;
    #[c2rust::src_loc = "67:3"]
    pub const XML_ROLE_NOTATION_NO_SYSTEM_ID: C2RustUnnamed_2 = 20;
    #[c2rust::src_loc = "66:3"]
    pub const XML_ROLE_NOTATION_SYSTEM_ID: C2RustUnnamed_2 = 19;
    #[c2rust::src_loc = "65:3"]
    pub const XML_ROLE_NOTATION_NAME: C2RustUnnamed_2 = 18;
    #[c2rust::src_loc = "64:3"]
    pub const XML_ROLE_NOTATION_NONE: C2RustUnnamed_2 = 17;
    #[c2rust::src_loc = "63:3"]
    pub const XML_ROLE_ENTITY_NOTATION_NAME: C2RustUnnamed_2 = 16;
    #[c2rust::src_loc = "62:3"]
    pub const XML_ROLE_ENTITY_COMPLETE: C2RustUnnamed_2 = 15;
    #[c2rust::src_loc = "61:3"]
    pub const XML_ROLE_ENTITY_PUBLIC_ID: C2RustUnnamed_2 = 14;
    #[c2rust::src_loc = "60:3"]
    pub const XML_ROLE_ENTITY_SYSTEM_ID: C2RustUnnamed_2 = 13;
    #[c2rust::src_loc = "59:3"]
    pub const XML_ROLE_ENTITY_VALUE: C2RustUnnamed_2 = 12;
    #[c2rust::src_loc = "58:3"]
    pub const XML_ROLE_ENTITY_NONE: C2RustUnnamed_2 = 11;
    #[c2rust::src_loc = "57:3"]
    pub const XML_ROLE_PARAM_ENTITY_NAME: C2RustUnnamed_2 = 10;
    #[c2rust::src_loc = "56:3"]
    pub const XML_ROLE_GENERAL_ENTITY_NAME: C2RustUnnamed_2 = 9;
    #[c2rust::src_loc = "55:3"]
    pub const XML_ROLE_DOCTYPE_CLOSE: C2RustUnnamed_2 = 8;
    #[c2rust::src_loc = "54:3"]
    pub const XML_ROLE_DOCTYPE_INTERNAL_SUBSET: C2RustUnnamed_2 = 7;
    #[c2rust::src_loc = "53:3"]
    pub const XML_ROLE_DOCTYPE_PUBLIC_ID: C2RustUnnamed_2 = 6;
    #[c2rust::src_loc = "52:3"]
    pub const XML_ROLE_DOCTYPE_SYSTEM_ID: C2RustUnnamed_2 = 5;
    #[c2rust::src_loc = "51:3"]
    pub const XML_ROLE_DOCTYPE_NAME: C2RustUnnamed_2 = 4;
    #[c2rust::src_loc = "50:3"]
    pub const XML_ROLE_DOCTYPE_NONE: C2RustUnnamed_2 = 3;
    #[c2rust::src_loc = "49:3"]
    pub const XML_ROLE_INSTANCE_START: C2RustUnnamed_2 = 2;
    #[c2rust::src_loc = "48:3"]
    pub const XML_ROLE_XML_DECL: C2RustUnnamed_2 = 1;
    #[c2rust::src_loc = "47:3"]
    pub const XML_ROLE_NONE: C2RustUnnamed_2 = 0;
    #[c2rust::src_loc = "46:3"]
    pub const XML_ROLE_ERROR: C2RustUnnamed_2 = -1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:9"]
    pub struct prolog_state {
        pub handler: Option<
            unsafe extern "C" fn(
                *mut prolog_state,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ENCODING,
            ) -> ::core::ffi::c_int,
        >,
        pub level: ::core::ffi::c_uint,
        pub role_none: ::core::ffi::c_int,
        pub includeLevel: ::core::ffi::c_uint,
        pub documentEntity: ::core::ffi::c_int,
        pub inEntityValue: ::core::ffi::c_int,
    }
    #[c2rust::src_loc = "112:1"]
    pub type PROLOG_STATE = prolog_state;
    use super::xmltok_h::ENCODING;
}
#[c2rust::header_src = "/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/lib/ascii.h:52"]
pub mod ascii_h {
    #[c2rust::src_loc = "36:9"]
    pub const ASCII_A: ::core::ffi::c_int = 0x41 as ::core::ffi::c_int;
    #[c2rust::src_loc = "37:9"]
    pub const ASCII_B: ::core::ffi::c_int = 0x42 as ::core::ffi::c_int;
    #[c2rust::src_loc = "38:9"]
    pub const ASCII_C: ::core::ffi::c_int = 0x43 as ::core::ffi::c_int;
    #[c2rust::src_loc = "39:9"]
    pub const ASCII_D: ::core::ffi::c_int = 0x44 as ::core::ffi::c_int;
    #[c2rust::src_loc = "40:9"]
    pub const ASCII_E: ::core::ffi::c_int = 0x45 as ::core::ffi::c_int;
    #[c2rust::src_loc = "41:9"]
    pub const ASCII_F: ::core::ffi::c_int = 0x46 as ::core::ffi::c_int;
    #[c2rust::src_loc = "42:9"]
    pub const ASCII_G: ::core::ffi::c_int = 0x47 as ::core::ffi::c_int;
    #[c2rust::src_loc = "44:9"]
    pub const ASCII_I: ::core::ffi::c_int = 0x49 as ::core::ffi::c_int;
    #[c2rust::src_loc = "46:9"]
    pub const ASCII_K: ::core::ffi::c_int = 0x4b as ::core::ffi::c_int;
    #[c2rust::src_loc = "47:9"]
    pub const ASCII_L: ::core::ffi::c_int = 0x4c as ::core::ffi::c_int;
    #[c2rust::src_loc = "48:9"]
    pub const ASCII_M: ::core::ffi::c_int = 0x4d as ::core::ffi::c_int;
    #[c2rust::src_loc = "49:9"]
    pub const ASCII_N: ::core::ffi::c_int = 0x4e as ::core::ffi::c_int;
    #[c2rust::src_loc = "50:9"]
    pub const ASCII_O: ::core::ffi::c_int = 0x4f as ::core::ffi::c_int;
    #[c2rust::src_loc = "51:9"]
    pub const ASCII_P: ::core::ffi::c_int = 0x50 as ::core::ffi::c_int;
    #[c2rust::src_loc = "52:9"]
    pub const ASCII_Q: ::core::ffi::c_int = 0x51 as ::core::ffi::c_int;
    #[c2rust::src_loc = "53:9"]
    pub const ASCII_R: ::core::ffi::c_int = 0x52 as ::core::ffi::c_int;
    #[c2rust::src_loc = "54:9"]
    pub const ASCII_S: ::core::ffi::c_int = 0x53 as ::core::ffi::c_int;
    #[c2rust::src_loc = "55:9"]
    pub const ASCII_T: ::core::ffi::c_int = 0x54 as ::core::ffi::c_int;
    #[c2rust::src_loc = "56:9"]
    pub const ASCII_U: ::core::ffi::c_int = 0x55 as ::core::ffi::c_int;
    #[c2rust::src_loc = "59:9"]
    pub const ASCII_X: ::core::ffi::c_int = 0x58 as ::core::ffi::c_int;
    #[c2rust::src_loc = "60:9"]
    pub const ASCII_Y: ::core::ffi::c_int = 0x59 as ::core::ffi::c_int;
}
pub use self::ascii_h::ASCII_A;
pub use self::ascii_h::ASCII_B;
pub use self::ascii_h::ASCII_C;
pub use self::ascii_h::ASCII_D;
pub use self::ascii_h::ASCII_E;
pub use self::ascii_h::ASCII_F;
pub use self::ascii_h::ASCII_G;
pub use self::ascii_h::ASCII_I;
pub use self::ascii_h::ASCII_K;
pub use self::ascii_h::ASCII_L;
pub use self::ascii_h::ASCII_M;
pub use self::ascii_h::ASCII_N;
pub use self::ascii_h::ASCII_O;
pub use self::ascii_h::ASCII_P;
pub use self::ascii_h::ASCII_Q;
pub use self::ascii_h::ASCII_R;
pub use self::ascii_h::ASCII_S;
pub use self::ascii_h::ASCII_T;
pub use self::ascii_h::ASCII_U;
pub use self::ascii_h::ASCII_X;
pub use self::ascii_h::ASCII_Y;
pub use self::expat_external_h::XML_Size;

pub use self::xmlrole_h::prolog_state;
pub use self::xmlrole_h::C2RustUnnamed_2;
pub use self::xmlrole_h::PROLOG_STATE;
pub use self::xmlrole_h::XML_ROLE_ATTLIST_ELEMENT_NAME;
pub use self::xmlrole_h::XML_ROLE_ATTLIST_NONE;
pub use self::xmlrole_h::XML_ROLE_ATTRIBUTE_ENUM_VALUE;
pub use self::xmlrole_h::XML_ROLE_ATTRIBUTE_NAME;
pub use self::xmlrole_h::XML_ROLE_ATTRIBUTE_NOTATION_VALUE;
pub use self::xmlrole_h::XML_ROLE_ATTRIBUTE_TYPE_CDATA;
pub use self::xmlrole_h::XML_ROLE_ATTRIBUTE_TYPE_ENTITIES;
pub use self::xmlrole_h::XML_ROLE_ATTRIBUTE_TYPE_ENTITY;
pub use self::xmlrole_h::XML_ROLE_ATTRIBUTE_TYPE_ID;
pub use self::xmlrole_h::XML_ROLE_ATTRIBUTE_TYPE_IDREF;
pub use self::xmlrole_h::XML_ROLE_ATTRIBUTE_TYPE_IDREFS;
pub use self::xmlrole_h::XML_ROLE_ATTRIBUTE_TYPE_NMTOKEN;
pub use self::xmlrole_h::XML_ROLE_ATTRIBUTE_TYPE_NMTOKENS;
pub use self::xmlrole_h::XML_ROLE_COMMENT;
pub use self::xmlrole_h::XML_ROLE_CONTENT_ANY;
pub use self::xmlrole_h::XML_ROLE_CONTENT_ELEMENT;
pub use self::xmlrole_h::XML_ROLE_CONTENT_ELEMENT_OPT;
pub use self::xmlrole_h::XML_ROLE_CONTENT_ELEMENT_PLUS;
pub use self::xmlrole_h::XML_ROLE_CONTENT_ELEMENT_REP;
pub use self::xmlrole_h::XML_ROLE_CONTENT_EMPTY;
pub use self::xmlrole_h::XML_ROLE_CONTENT_PCDATA;
pub use self::xmlrole_h::XML_ROLE_DEFAULT_ATTRIBUTE_VALUE;
pub use self::xmlrole_h::XML_ROLE_DOCTYPE_CLOSE;
pub use self::xmlrole_h::XML_ROLE_DOCTYPE_INTERNAL_SUBSET;
pub use self::xmlrole_h::XML_ROLE_DOCTYPE_NAME;
pub use self::xmlrole_h::XML_ROLE_DOCTYPE_NONE;
pub use self::xmlrole_h::XML_ROLE_DOCTYPE_PUBLIC_ID;
pub use self::xmlrole_h::XML_ROLE_DOCTYPE_SYSTEM_ID;
pub use self::xmlrole_h::XML_ROLE_ELEMENT_NAME;
pub use self::xmlrole_h::XML_ROLE_ELEMENT_NONE;
pub use self::xmlrole_h::XML_ROLE_ENTITY_COMPLETE;
pub use self::xmlrole_h::XML_ROLE_ENTITY_NONE;
pub use self::xmlrole_h::XML_ROLE_ENTITY_NOTATION_NAME;
pub use self::xmlrole_h::XML_ROLE_ENTITY_PUBLIC_ID;
pub use self::xmlrole_h::XML_ROLE_ENTITY_SYSTEM_ID;
pub use self::xmlrole_h::XML_ROLE_ENTITY_VALUE;
pub use self::xmlrole_h::XML_ROLE_ERROR;
pub use self::xmlrole_h::XML_ROLE_FIXED_ATTRIBUTE_VALUE;
pub use self::xmlrole_h::XML_ROLE_GENERAL_ENTITY_NAME;
pub use self::xmlrole_h::XML_ROLE_GROUP_CHOICE;
pub use self::xmlrole_h::XML_ROLE_GROUP_CLOSE;
pub use self::xmlrole_h::XML_ROLE_GROUP_CLOSE_OPT;
pub use self::xmlrole_h::XML_ROLE_GROUP_CLOSE_PLUS;
pub use self::xmlrole_h::XML_ROLE_GROUP_CLOSE_REP;
pub use self::xmlrole_h::XML_ROLE_GROUP_OPEN;
pub use self::xmlrole_h::XML_ROLE_GROUP_SEQUENCE;
pub use self::xmlrole_h::XML_ROLE_IGNORE_SECT;
pub use self::xmlrole_h::XML_ROLE_IMPLIED_ATTRIBUTE_VALUE;
pub use self::xmlrole_h::XML_ROLE_INNER_PARAM_ENTITY_REF;
pub use self::xmlrole_h::XML_ROLE_INSTANCE_START;
pub use self::xmlrole_h::XML_ROLE_NONE;
pub use self::xmlrole_h::XML_ROLE_NOTATION_NAME;
pub use self::xmlrole_h::XML_ROLE_NOTATION_NONE;
pub use self::xmlrole_h::XML_ROLE_NOTATION_NO_SYSTEM_ID;
pub use self::xmlrole_h::XML_ROLE_NOTATION_PUBLIC_ID;
pub use self::xmlrole_h::XML_ROLE_NOTATION_SYSTEM_ID;
pub use self::xmlrole_h::XML_ROLE_PARAM_ENTITY_NAME;
pub use self::xmlrole_h::XML_ROLE_PARAM_ENTITY_REF;
pub use self::xmlrole_h::XML_ROLE_PI;
pub use self::xmlrole_h::XML_ROLE_REQUIRED_ATTRIBUTE_VALUE;
pub use self::xmlrole_h::XML_ROLE_TEXT_DECL;
pub use self::xmlrole_h::XML_ROLE_XML_DECL;
pub use self::xmltok_h::encoding;
pub use self::xmltok_h::position;
pub use self::xmltok_h::XML_Convert_Result;
pub use self::xmltok_h::ATTRIBUTE;
pub use self::xmltok_h::ENCODING;
pub use self::xmltok_h::POSITION;
pub use self::xmltok_h::SCANNER;
pub use self::xmltok_h::XML_CONVERT_COMPLETED;
pub use self::xmltok_h::XML_CONVERT_INPUT_INCOMPLETE;
pub use self::xmltok_h::XML_CONVERT_OUTPUT_EXHAUSTED;
pub use self::xmltok_h::XML_TOK_BOM;
pub use self::xmltok_h::XML_TOK_CLOSE_BRACKET;
pub use self::xmltok_h::XML_TOK_CLOSE_PAREN;
pub use self::xmltok_h::XML_TOK_CLOSE_PAREN_ASTERISK;
pub use self::xmltok_h::XML_TOK_CLOSE_PAREN_PLUS;
pub use self::xmltok_h::XML_TOK_CLOSE_PAREN_QUESTION;
pub use self::xmltok_h::XML_TOK_COMMA;
pub use self::xmltok_h::XML_TOK_COMMENT;
pub use self::xmltok_h::XML_TOK_COND_SECT_CLOSE;
pub use self::xmltok_h::XML_TOK_COND_SECT_OPEN;
pub use self::xmltok_h::XML_TOK_DECL_CLOSE;
pub use self::xmltok_h::XML_TOK_DECL_OPEN;
pub use self::xmltok_h::XML_TOK_INSTANCE_START;
pub use self::xmltok_h::XML_TOK_LITERAL;
pub use self::xmltok_h::XML_TOK_NAME;
pub use self::xmltok_h::XML_TOK_NAME_ASTERISK;
pub use self::xmltok_h::XML_TOK_NAME_PLUS;
pub use self::xmltok_h::XML_TOK_NAME_QUESTION;
pub use self::xmltok_h::XML_TOK_NMTOKEN;
pub use self::xmltok_h::XML_TOK_NONE;
pub use self::xmltok_h::XML_TOK_OPEN_BRACKET;
pub use self::xmltok_h::XML_TOK_OPEN_PAREN;
pub use self::xmltok_h::XML_TOK_OR;
pub use self::xmltok_h::XML_TOK_PARAM_ENTITY_REF;
pub use self::xmltok_h::XML_TOK_PERCENT;
pub use self::xmltok_h::XML_TOK_PI;
pub use self::xmltok_h::XML_TOK_POUND_NAME;
pub use self::xmltok_h::XML_TOK_PREFIXED_NAME;
pub use self::xmltok_h::XML_TOK_PROLOG_S;
pub use self::xmltok_h::XML_TOK_XML_DECL;
#[c2rust::src_loc = "122:1"]
pub type PROLOG_HANDLER = unsafe extern "C" fn(
    *mut PROLOG_STATE,
    ::core::ffi::c_int,
    *const ::core::ffi::c_char,
    *const ::core::ffi::c_char,
    *const ENCODING,
) -> ::core::ffi::c_int;
#[c2rust::src_loc = "61:1"]
static mut KW_ANY: [::core::ffi::c_char; 4] = [
    ASCII_A as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    ASCII_Y as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "62:1"]
static mut KW_ATTLIST: [::core::ffi::c_char; 8] = [
    ASCII_A as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_L as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_S as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "64:1"]
static mut KW_CDATA: [::core::ffi::c_char; 6] = [
    ASCII_C as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    ASCII_A as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_A as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "66:1"]
static mut KW_DOCTYPE: [::core::ffi::c_char; 8] = [
    ASCII_D as ::core::ffi::c_char,
    ASCII_O as ::core::ffi::c_char,
    ASCII_C as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_Y as ::core::ffi::c_char,
    ASCII_P as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "68:1"]
static mut KW_ELEMENT: [::core::ffi::c_char; 8] = [
    ASCII_E as ::core::ffi::c_char,
    ASCII_L as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_M as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "70:1"]
static mut KW_EMPTY: [::core::ffi::c_char; 6] = [
    ASCII_E as ::core::ffi::c_char,
    ASCII_M as ::core::ffi::c_char,
    ASCII_P as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_Y as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "72:1"]
static mut KW_ENTITIES: [::core::ffi::c_char; 9] = [
    ASCII_E as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_S as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "74:1"]
static mut KW_ENTITY: [::core::ffi::c_char; 7] = [
    ASCII_E as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_Y as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "76:1"]
static mut KW_FIXED: [::core::ffi::c_char; 6] = [
    ASCII_F as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_X as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "78:1"]
static mut KW_ID: [::core::ffi::c_char; 3] = [
    ASCII_I as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "79:1"]
static mut KW_IDREF: [::core::ffi::c_char; 6] = [
    ASCII_I as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    ASCII_R as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_F as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "81:1"]
static mut KW_IDREFS: [::core::ffi::c_char; 7] = [
    ASCII_I as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    ASCII_R as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_F as ::core::ffi::c_char,
    ASCII_S as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "84:1"]
static mut KW_IGNORE: [::core::ffi::c_char; 7] = [
    ASCII_I as ::core::ffi::c_char,
    ASCII_G as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    ASCII_O as ::core::ffi::c_char,
    ASCII_R as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "87:1"]
static mut KW_IMPLIED: [::core::ffi::c_char; 8] = [
    ASCII_I as ::core::ffi::c_char,
    ASCII_M as ::core::ffi::c_char,
    ASCII_P as ::core::ffi::c_char,
    ASCII_L as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "90:1"]
static mut KW_INCLUDE: [::core::ffi::c_char; 8] = [
    ASCII_I as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    ASCII_C as ::core::ffi::c_char,
    ASCII_L as ::core::ffi::c_char,
    ASCII_U as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "93:1"]
static mut KW_NDATA: [::core::ffi::c_char; 6] = [
    ASCII_N as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    ASCII_A as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_A as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "95:1"]
static mut KW_NMTOKEN: [::core::ffi::c_char; 8] = [
    ASCII_N as ::core::ffi::c_char,
    ASCII_M as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_O as ::core::ffi::c_char,
    ASCII_K as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "97:1"]
static mut KW_NMTOKENS: [::core::ffi::c_char; 9] = [
    ASCII_N as ::core::ffi::c_char,
    ASCII_M as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_O as ::core::ffi::c_char,
    ASCII_K as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    ASCII_S as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "99:1"]
static mut KW_NOTATION: [::core::ffi::c_char; 9] = [
    ASCII_N as ::core::ffi::c_char,
    ASCII_O as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_A as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_O as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "101:1"]
static mut KW_PCDATA: [::core::ffi::c_char; 7] = [
    ASCII_P as ::core::ffi::c_char,
    ASCII_C as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    ASCII_A as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_A as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "103:1"]
static mut KW_PUBLIC: [::core::ffi::c_char; 7] = [
    ASCII_P as ::core::ffi::c_char,
    ASCII_U as ::core::ffi::c_char,
    ASCII_B as ::core::ffi::c_char,
    ASCII_L as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_C as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "105:1"]
static mut KW_REQUIRED: [::core::ffi::c_char; 9] = [
    ASCII_R as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_Q as ::core::ffi::c_char,
    ASCII_U as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_R as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "107:1"]
static mut KW_SYSTEM: [::core::ffi::c_char; 7] = [
    ASCII_S as ::core::ffi::c_char,
    ASCII_Y as ::core::ffi::c_char,
    ASCII_S as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_M as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
#[c2rust::src_loc = "140:1"]
unsafe extern "C" fn prolog0(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => {
            (*state).handler = Some(
                prolog1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_NONE as ::core::ffi::c_int;
        }
        XML_TOK_XML_DECL => {
            (*state).handler = Some(
                prolog1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_XML_DECL as ::core::ffi::c_int;
        }
        XML_TOK_PI => {
            (*state).handler = Some(
                prolog1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_PI as ::core::ffi::c_int;
        }
        XML_TOK_COMMENT => {
            (*state).handler = Some(
                prolog1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_COMMENT as ::core::ffi::c_int;
        }
        XML_TOK_BOM => return XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_DECL_OPEN => {
            if !((*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((2 as ::core::ffi::c_int * (*enc).minBytesPerChar) as isize),
                end,
                &raw const KW_DOCTYPE as *const ::core::ffi::c_char,
            ) == 0)
            {
                (*state).handler = Some(
                    doctype0
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int;
            }
        }
        XML_TOK_INSTANCE_START => {
            (*state).handler = Some(
                error
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_INSTANCE_START as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "171:1"]
unsafe extern "C" fn prolog1(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_PI => return XML_ROLE_PI as ::core::ffi::c_int,
        XML_TOK_COMMENT => return XML_ROLE_COMMENT as ::core::ffi::c_int,
        XML_TOK_BOM => return XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_DECL_OPEN => {
            if !((*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((2 as ::core::ffi::c_int * (*enc).minBytesPerChar) as isize),
                end,
                &raw const KW_DOCTYPE as *const ::core::ffi::c_char,
            ) == 0)
            {
                (*state).handler = Some(
                    doctype0
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int;
            }
        }
        XML_TOK_INSTANCE_START => {
            (*state).handler = Some(
                error
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_INSTANCE_START as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "203:1"]
unsafe extern "C" fn prolog2(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_PI => return XML_ROLE_PI as ::core::ffi::c_int,
        XML_TOK_COMMENT => return XML_ROLE_COMMENT as ::core::ffi::c_int,
        XML_TOK_INSTANCE_START => {
            (*state).handler = Some(
                error
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_INSTANCE_START as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "223:1"]
unsafe extern "C" fn doctype0(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int,
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                doctype1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_DOCTYPE_NAME as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "240:1"]
unsafe extern "C" fn doctype1(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int,
        XML_TOK_OPEN_BRACKET => {
            (*state).handler = Some(
                internalSubset
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_DOCTYPE_INTERNAL_SUBSET as ::core::ffi::c_int;
        }
        XML_TOK_DECL_CLOSE => {
            (*state).handler = Some(
                prolog2
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_DOCTYPE_CLOSE as ::core::ffi::c_int;
        }
        XML_TOK_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_SYSTEM as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    doctype3
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_PUBLIC as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    doctype2
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int;
            }
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "266:1"]
unsafe extern "C" fn doctype2(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                doctype3
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_DOCTYPE_PUBLIC_ID as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "282:1"]
unsafe extern "C" fn doctype3(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                doctype4
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_DOCTYPE_SYSTEM_ID as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "298:1"]
unsafe extern "C" fn doctype4(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int,
        XML_TOK_OPEN_BRACKET => {
            (*state).handler = Some(
                internalSubset
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_DOCTYPE_INTERNAL_SUBSET as ::core::ffi::c_int;
        }
        XML_TOK_DECL_CLOSE => {
            (*state).handler = Some(
                prolog2
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_DOCTYPE_CLOSE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "317:1"]
unsafe extern "C" fn doctype5(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int,
        XML_TOK_DECL_CLOSE => {
            (*state).handler = Some(
                prolog2
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_DOCTYPE_CLOSE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "333:1"]
unsafe extern "C" fn internalSubset(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_DECL_OPEN => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((2 as ::core::ffi::c_int * (*enc).minBytesPerChar) as isize),
                end,
                &raw const KW_ENTITY as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    entity0
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((2 as ::core::ffi::c_int * (*enc).minBytesPerChar) as isize),
                end,
                &raw const KW_ATTLIST as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    attlist0
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((2 as ::core::ffi::c_int * (*enc).minBytesPerChar) as isize),
                end,
                &raw const KW_ELEMENT as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    element0
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((2 as ::core::ffi::c_int * (*enc).minBytesPerChar) as isize),
                end,
                &raw const KW_NOTATION as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    notation0
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_NOTATION_NONE as ::core::ffi::c_int;
            }
        }
        XML_TOK_PI => return XML_ROLE_PI as ::core::ffi::c_int,
        XML_TOK_COMMENT => return XML_ROLE_COMMENT as ::core::ffi::c_int,
        XML_TOK_PARAM_ENTITY_REF => {
            return XML_ROLE_PARAM_ENTITY_REF as ::core::ffi::c_int;
        }
        XML_TOK_CLOSE_BRACKET => {
            (*state).handler = Some(
                doctype5
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int;
        }
        XML_TOK_NONE => return XML_ROLE_NONE as ::core::ffi::c_int,
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "378:1"]
unsafe extern "C" fn externalSubset0(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    (*state).handler = Some(
        externalSubset1
            as unsafe extern "C" fn(
                *mut PROLOG_STATE,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ENCODING,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut prolog_state,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ENCODING,
            ) -> ::core::ffi::c_int,
        >;
    if tok == XML_TOK_XML_DECL {
        return XML_ROLE_TEXT_DECL as ::core::ffi::c_int;
    }
    return externalSubset1(state, tok, ptr, end, enc);
}
#[c2rust::src_loc = "387:1"]
unsafe extern "C" fn externalSubset1(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_COND_SECT_OPEN => {
            (*state).handler = Some(
                condSect0
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_NONE as ::core::ffi::c_int;
        }
        XML_TOK_COND_SECT_CLOSE => {
            if !((*state).includeLevel == 0 as ::core::ffi::c_uint) {
                (*state).includeLevel =
                    (*state).includeLevel.wrapping_sub(1 as ::core::ffi::c_uint);
                return XML_ROLE_NONE as ::core::ffi::c_int;
            }
        }
        XML_TOK_PROLOG_S => return XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_CLOSE_BRACKET => {}
        XML_TOK_NONE => {
            if !((*state).includeLevel != 0) {
                return XML_ROLE_NONE as ::core::ffi::c_int;
            }
        }
        _ => return internalSubset(state, tok, ptr, end, enc),
    }
    return common(state, tok);
}
#[c2rust::src_loc = "415:1"]
unsafe extern "C" fn entity0(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int,
        XML_TOK_PERCENT => {
            (*state).handler = Some(
                entity1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int;
        }
        XML_TOK_NAME => {
            (*state).handler = Some(
                entity2
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_GENERAL_ENTITY_NAME as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "434:1"]
unsafe extern "C" fn entity1(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            (*state).handler = Some(
                entity7
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_PARAM_ENTITY_NAME as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "450:1"]
unsafe extern "C" fn entity2(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_SYSTEM as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    entity4
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_PUBLIC as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    entity3
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int;
            }
        }
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                declClose
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            (*state).role_none = XML_ROLE_ENTITY_NONE as ::core::ffi::c_int;
            return XML_ROLE_ENTITY_VALUE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "474:1"]
unsafe extern "C" fn entity3(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                entity4
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ENTITY_PUBLIC_ID as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "490:1"]
unsafe extern "C" fn entity4(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                entity5
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ENTITY_SYSTEM_ID as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "506:1"]
unsafe extern "C" fn entity5(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int,
        XML_TOK_DECL_CLOSE => {
            (*state).handler = (if (*state).documentEntity != 0 {
                Some(
                    internalSubset
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
            } else {
                Some(
                    externalSubset1
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
            })
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ENTITY_COMPLETE as ::core::ffi::c_int;
        }
        XML_TOK_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_NDATA as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    entity6
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int;
            }
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "525:1"]
unsafe extern "C" fn entity6(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            (*state).handler = Some(
                declClose
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            (*state).role_none = XML_ROLE_ENTITY_NONE as ::core::ffi::c_int;
            return XML_ROLE_ENTITY_NOTATION_NAME as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "542:1"]
unsafe extern "C" fn entity7(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_SYSTEM as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    entity9
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_PUBLIC as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    entity8
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int;
            }
        }
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                declClose
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            (*state).role_none = XML_ROLE_ENTITY_NONE as ::core::ffi::c_int;
            return XML_ROLE_ENTITY_VALUE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "566:1"]
unsafe extern "C" fn entity8(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                entity9
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ENTITY_PUBLIC_ID as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "582:1"]
unsafe extern "C" fn entity9(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                entity10
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ENTITY_SYSTEM_ID as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "598:1"]
unsafe extern "C" fn entity10(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int,
        XML_TOK_DECL_CLOSE => {
            (*state).handler = (if (*state).documentEntity != 0 {
                Some(
                    internalSubset
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
            } else {
                Some(
                    externalSubset1
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
            })
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ENTITY_COMPLETE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "614:1"]
unsafe extern "C" fn notation0(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            (*state).handler = Some(
                notation1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_NOTATION_NAME as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "630:1"]
unsafe extern "C" fn notation1(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_SYSTEM as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    notation3
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_NOTATION_NONE as ::core::ffi::c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_PUBLIC as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    notation2
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_NOTATION_NONE as ::core::ffi::c_int;
            }
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "650:1"]
unsafe extern "C" fn notation2(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                notation4
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_NOTATION_PUBLIC_ID as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "666:1"]
unsafe extern "C" fn notation3(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                declClose
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            (*state).role_none = XML_ROLE_NOTATION_NONE as ::core::ffi::c_int;
            return XML_ROLE_NOTATION_SYSTEM_ID as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "683:1"]
unsafe extern "C" fn notation4(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                declClose
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            (*state).role_none = XML_ROLE_NOTATION_NONE as ::core::ffi::c_int;
            return XML_ROLE_NOTATION_SYSTEM_ID as ::core::ffi::c_int;
        }
        XML_TOK_DECL_CLOSE => {
            (*state).handler = (if (*state).documentEntity != 0 {
                Some(
                    internalSubset
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
            } else {
                Some(
                    externalSubset1
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
            })
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_NOTATION_NO_SYSTEM_ID as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "703:1"]
unsafe extern "C" fn attlist0(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                attlist1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ATTLIST_ELEMENT_NAME as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "720:1"]
unsafe extern "C" fn attlist1(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_DECL_CLOSE => {
            (*state).handler = (if (*state).documentEntity != 0 {
                Some(
                    internalSubset
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
            } else {
                Some(
                    externalSubset1
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
            })
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int;
        }
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                attlist2
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ATTRIBUTE_NAME as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "740:1"]
unsafe extern "C" fn attlist2(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            static mut types: [*const ::core::ffi::c_char; 8] = unsafe {
                [
                    &raw const KW_CDATA as *const ::core::ffi::c_char,
                    &raw const KW_ID as *const ::core::ffi::c_char,
                    &raw const KW_IDREF as *const ::core::ffi::c_char,
                    &raw const KW_IDREFS as *const ::core::ffi::c_char,
                    &raw const KW_ENTITY as *const ::core::ffi::c_char,
                    &raw const KW_ENTITIES as *const ::core::ffi::c_char,
                    &raw const KW_NMTOKEN as *const ::core::ffi::c_char,
                    &raw const KW_NMTOKENS as *const ::core::ffi::c_char,
                ]
            };
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i
                < (::core::mem::size_of::<[*const ::core::ffi::c_char; 8]>() as usize)
                    .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
                    as ::core::ffi::c_int
            {
                if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                    enc,
                    ptr,
                    end,
                    types[i as usize],
                ) != 0
                {
                    (*state).handler = Some(
                        attlist8
                            as unsafe extern "C" fn(
                                *mut PROLOG_STATE,
                                ::core::ffi::c_int,
                                *const ::core::ffi::c_char,
                                *const ::core::ffi::c_char,
                                *const ENCODING,
                            )
                                -> ::core::ffi::c_int,
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut prolog_state,
                                ::core::ffi::c_int,
                                *const ::core::ffi::c_char,
                                *const ::core::ffi::c_char,
                                *const ENCODING,
                            ) -> ::core::ffi::c_int,
                        >;
                    return XML_ROLE_ATTRIBUTE_TYPE_CDATA as ::core::ffi::c_int + i;
                }
                i += 1;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_NOTATION as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    attlist5
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int;
            }
        }
        XML_TOK_OPEN_PAREN => {
            (*state).handler = Some(
                attlist3
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "770:1"]
unsafe extern "C" fn attlist3(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_NMTOKEN | XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                attlist4
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ATTRIBUTE_ENUM_VALUE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "788:1"]
unsafe extern "C" fn attlist4(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_CLOSE_PAREN => {
            (*state).handler = Some(
                attlist8
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int;
        }
        XML_TOK_OR => {
            (*state).handler = Some(
                attlist3
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "807:1"]
unsafe extern "C" fn attlist5(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_OPEN_PAREN => {
            (*state).handler = Some(
                attlist6
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "823:1"]
unsafe extern "C" fn attlist6(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            (*state).handler = Some(
                attlist7
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ATTRIBUTE_NOTATION_VALUE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "839:1"]
unsafe extern "C" fn attlist7(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_CLOSE_PAREN => {
            (*state).handler = Some(
                attlist8
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int;
        }
        XML_TOK_OR => {
            (*state).handler = Some(
                attlist6
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "859:1"]
unsafe extern "C" fn attlist8(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_POUND_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((*enc).minBytesPerChar as isize),
                end,
                &raw const KW_IMPLIED as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    attlist1
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_IMPLIED_ATTRIBUTE_VALUE as ::core::ffi::c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((*enc).minBytesPerChar as isize),
                end,
                &raw const KW_REQUIRED as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    attlist1
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_REQUIRED_ATTRIBUTE_VALUE as ::core::ffi::c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((*enc).minBytesPerChar as isize),
                end,
                &raw const KW_FIXED as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    attlist9
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int;
            }
        }
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                attlist1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_DEFAULT_ATTRIBUTE_VALUE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "889:1"]
unsafe extern "C" fn attlist9(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                attlist1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_FIXED_ATTRIBUTE_VALUE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "905:1"]
unsafe extern "C" fn element0(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int,
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                element1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ELEMENT_NAME as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "922:1"]
unsafe extern "C" fn element1(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_EMPTY as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    declClose
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                (*state).role_none = XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
                return XML_ROLE_CONTENT_EMPTY as ::core::ffi::c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_ANY as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    declClose
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                (*state).role_none = XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
                return XML_ROLE_CONTENT_ANY as ::core::ffi::c_int;
            }
        }
        XML_TOK_OPEN_PAREN => {
            (*state).handler = Some(
                element2
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            (*state).level = 1 as ::core::ffi::c_uint;
            return XML_ROLE_GROUP_OPEN as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "948:1"]
unsafe extern "C" fn element2(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int,
        XML_TOK_POUND_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((*enc).minBytesPerChar as isize),
                end,
                &raw const KW_PCDATA as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    element3
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_CONTENT_PCDATA as ::core::ffi::c_int;
            }
        }
        XML_TOK_OPEN_PAREN => {
            (*state).level = 2 as ::core::ffi::c_uint;
            (*state).handler = Some(
                element6
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_GROUP_OPEN as ::core::ffi::c_int;
        }
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                element7
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_CONTENT_ELEMENT as ::core::ffi::c_int;
        }
        XML_TOK_NAME_QUESTION => {
            (*state).handler = Some(
                element7
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_CONTENT_ELEMENT_OPT as ::core::ffi::c_int;
        }
        XML_TOK_NAME_ASTERISK => {
            (*state).handler = Some(
                element7
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_CONTENT_ELEMENT_REP as ::core::ffi::c_int;
        }
        XML_TOK_NAME_PLUS => {
            (*state).handler = Some(
                element7
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_CONTENT_ELEMENT_PLUS as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "982:1"]
unsafe extern "C" fn element3(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int,
        XML_TOK_CLOSE_PAREN => {
            (*state).handler = Some(
                declClose
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            (*state).role_none = XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
            return XML_ROLE_GROUP_CLOSE as ::core::ffi::c_int;
        }
        XML_TOK_CLOSE_PAREN_ASTERISK => {
            (*state).handler = Some(
                declClose
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            (*state).role_none = XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
            return XML_ROLE_GROUP_CLOSE_REP as ::core::ffi::c_int;
        }
        XML_TOK_OR => {
            (*state).handler = Some(
                element4
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "1006:1"]
unsafe extern "C" fn element4(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int,
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                element5
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_CONTENT_ELEMENT as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "1023:1"]
unsafe extern "C" fn element5(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int,
        XML_TOK_CLOSE_PAREN_ASTERISK => {
            (*state).handler = Some(
                declClose
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            (*state).role_none = XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
            return XML_ROLE_GROUP_CLOSE_REP as ::core::ffi::c_int;
        }
        XML_TOK_OR => {
            (*state).handler = Some(
                element4
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "1043:1"]
unsafe extern "C" fn element6(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int,
        XML_TOK_OPEN_PAREN => {
            (*state).level = (*state).level.wrapping_add(1 as ::core::ffi::c_uint);
            return XML_ROLE_GROUP_OPEN as ::core::ffi::c_int;
        }
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                element7
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_CONTENT_ELEMENT as ::core::ffi::c_int;
        }
        XML_TOK_NAME_QUESTION => {
            (*state).handler = Some(
                element7
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_CONTENT_ELEMENT_OPT as ::core::ffi::c_int;
        }
        XML_TOK_NAME_ASTERISK => {
            (*state).handler = Some(
                element7
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_CONTENT_ELEMENT_REP as ::core::ffi::c_int;
        }
        XML_TOK_NAME_PLUS => {
            (*state).handler = Some(
                element7
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_CONTENT_ELEMENT_PLUS as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "1072:1"]
unsafe extern "C" fn element7(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int,
        XML_TOK_CLOSE_PAREN => {
            (*state).level = (*state).level.wrapping_sub(1 as ::core::ffi::c_uint);
            if (*state).level == 0 as ::core::ffi::c_uint {
                (*state).handler = Some(
                    declClose
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                (*state).role_none = XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
            }
            return XML_ROLE_GROUP_CLOSE as ::core::ffi::c_int;
        }
        XML_TOK_CLOSE_PAREN_ASTERISK => {
            (*state).level = (*state).level.wrapping_sub(1 as ::core::ffi::c_uint);
            if (*state).level == 0 as ::core::ffi::c_uint {
                (*state).handler = Some(
                    declClose
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                (*state).role_none = XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
            }
            return XML_ROLE_GROUP_CLOSE_REP as ::core::ffi::c_int;
        }
        XML_TOK_CLOSE_PAREN_QUESTION => {
            (*state).level = (*state).level.wrapping_sub(1 as ::core::ffi::c_uint);
            if (*state).level == 0 as ::core::ffi::c_uint {
                (*state).handler = Some(
                    declClose
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                (*state).role_none = XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
            }
            return XML_ROLE_GROUP_CLOSE_OPT as ::core::ffi::c_int;
        }
        XML_TOK_CLOSE_PAREN_PLUS => {
            (*state).level = (*state).level.wrapping_sub(1 as ::core::ffi::c_uint);
            if (*state).level == 0 as ::core::ffi::c_uint {
                (*state).handler = Some(
                    declClose
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                (*state).role_none = XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
            }
            return XML_ROLE_GROUP_CLOSE_PLUS as ::core::ffi::c_int;
        }
        XML_TOK_COMMA => {
            (*state).handler = Some(
                element6
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_GROUP_SEQUENCE as ::core::ffi::c_int;
        }
        XML_TOK_OR => {
            (*state).handler = Some(
                element6
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_GROUP_CHOICE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "1121:1"]
unsafe extern "C" fn condSect0(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_INCLUDE as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    condSect1
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_NONE as ::core::ffi::c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_IGNORE as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    condSect2
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_NONE as ::core::ffi::c_int;
            }
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "1141:1"]
unsafe extern "C" fn condSect1(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_OPEN_BRACKET => {
            (*state).handler = Some(
                externalSubset1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            (*state).includeLevel = (*state).includeLevel.wrapping_add(1 as ::core::ffi::c_uint);
            return XML_ROLE_NONE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "1158:1"]
unsafe extern "C" fn condSect2(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_OPEN_BRACKET => {
            (*state).handler = Some(
                externalSubset1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_IGNORE_SECT as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "1176:1"]
unsafe extern "C" fn declClose(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return (*state).role_none,
        XML_TOK_DECL_CLOSE => {
            (*state).handler = (if (*state).documentEntity != 0 {
                Some(
                    internalSubset
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
            } else {
                Some(
                    externalSubset1
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
            })
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return (*state).role_none;
        }
        _ => {}
    }
    return common(state, tok);
}
#[c2rust::src_loc = "1212:1"]
unsafe extern "C" fn error(
    mut _state: *mut PROLOG_STATE,
    mut _tok: ::core::ffi::c_int,
    mut _ptr: *const ::core::ffi::c_char,
    mut _end: *const ::core::ffi::c_char,
    mut _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    return XML_ROLE_NONE as ::core::ffi::c_int;
}
#[c2rust::src_loc = "1224:1"]
unsafe extern "C" fn common(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*state).documentEntity == 0 && tok == XML_TOK_PARAM_ENTITY_REF {
        return XML_ROLE_INNER_PARAM_ENTITY_REF as ::core::ffi::c_int;
    }
    (*state).handler = Some(
        error
            as unsafe extern "C" fn(
                *mut PROLOG_STATE,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ENCODING,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut prolog_state,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ENCODING,
            ) -> ::core::ffi::c_int,
        >;
    return XML_ROLE_ERROR as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1236:1"]
pub unsafe extern "C" fn XmlPrologStateInit(mut state: *mut PROLOG_STATE) {
    (*state).handler = Some(
        prolog0
            as unsafe extern "C" fn(
                *mut PROLOG_STATE,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ENCODING,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut prolog_state,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ENCODING,
            ) -> ::core::ffi::c_int,
        >;
    (*state).documentEntity = 1 as ::core::ffi::c_int;
    (*state).includeLevel = 0 as ::core::ffi::c_uint;
    (*state).inEntityValue = 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1248:1"]
pub unsafe extern "C" fn XmlPrologStateInitExternalEntity(mut state: *mut PROLOG_STATE) {
    (*state).handler = Some(
        externalSubset0
            as unsafe extern "C" fn(
                *mut PROLOG_STATE,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ENCODING,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut prolog_state,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ENCODING,
            ) -> ::core::ffi::c_int,
        >;
    (*state).documentEntity = 0 as ::core::ffi::c_int;
    (*state).includeLevel = 0 as ::core::ffi::c_uint;
}
