pub type XML_Size = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct position {
    pub lineNumber: XML_Size,
    pub columnNumber: XML_Size,
}
pub type POSITION = position;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ATTRIBUTE {
    pub name: *const ::core::ffi::c_char,
    pub valuePtr: *const ::core::ffi::c_char,
    pub valueEnd: *const ::core::ffi::c_char,
    pub normalized: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
pub type ENCODING = encoding;
pub type XML_Convert_Result = ::core::ffi::c_uint;
pub const XML_CONVERT_OUTPUT_EXHAUSTED: XML_Convert_Result = 2;
pub const XML_CONVERT_INPUT_INCOMPLETE: XML_Convert_Result = 1;
pub const XML_CONVERT_COMPLETED: XML_Convert_Result = 0;
pub type SCANNER = Option<
    unsafe extern "C" fn(
        *const ENCODING,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
>;
pub type C2Rust_Unnamed = ::core::ffi::c_int;
pub const XML_ROLE_PARAM_ENTITY_REF: C2Rust_Unnamed = 60;
pub const XML_ROLE_INNER_PARAM_ENTITY_REF: C2Rust_Unnamed = 59;
pub const XML_ROLE_IGNORE_SECT: C2Rust_Unnamed = 58;
pub const XML_ROLE_TEXT_DECL: C2Rust_Unnamed = 57;
pub const XML_ROLE_COMMENT: C2Rust_Unnamed = 56;
pub const XML_ROLE_PI: C2Rust_Unnamed = 55;
pub const XML_ROLE_CONTENT_ELEMENT_PLUS: C2Rust_Unnamed = 54;
pub const XML_ROLE_CONTENT_ELEMENT_OPT: C2Rust_Unnamed = 53;
pub const XML_ROLE_CONTENT_ELEMENT_REP: C2Rust_Unnamed = 52;
pub const XML_ROLE_CONTENT_ELEMENT: C2Rust_Unnamed = 51;
pub const XML_ROLE_GROUP_SEQUENCE: C2Rust_Unnamed = 50;
pub const XML_ROLE_GROUP_CHOICE: C2Rust_Unnamed = 49;
pub const XML_ROLE_GROUP_CLOSE_PLUS: C2Rust_Unnamed = 48;
pub const XML_ROLE_GROUP_CLOSE_OPT: C2Rust_Unnamed = 47;
pub const XML_ROLE_GROUP_CLOSE_REP: C2Rust_Unnamed = 46;
pub const XML_ROLE_GROUP_CLOSE: C2Rust_Unnamed = 45;
pub const XML_ROLE_GROUP_OPEN: C2Rust_Unnamed = 44;
pub const XML_ROLE_CONTENT_PCDATA: C2Rust_Unnamed = 43;
pub const XML_ROLE_CONTENT_EMPTY: C2Rust_Unnamed = 42;
pub const XML_ROLE_CONTENT_ANY: C2Rust_Unnamed = 41;
pub const XML_ROLE_ELEMENT_NAME: C2Rust_Unnamed = 40;
pub const XML_ROLE_ELEMENT_NONE: C2Rust_Unnamed = 39;
pub const XML_ROLE_FIXED_ATTRIBUTE_VALUE: C2Rust_Unnamed = 38;
pub const XML_ROLE_DEFAULT_ATTRIBUTE_VALUE: C2Rust_Unnamed = 37;
pub const XML_ROLE_REQUIRED_ATTRIBUTE_VALUE: C2Rust_Unnamed = 36;
pub const XML_ROLE_IMPLIED_ATTRIBUTE_VALUE: C2Rust_Unnamed = 35;
pub const XML_ROLE_ATTLIST_ELEMENT_NAME: C2Rust_Unnamed = 34;
pub const XML_ROLE_ATTLIST_NONE: C2Rust_Unnamed = 33;
pub const XML_ROLE_ATTRIBUTE_NOTATION_VALUE: C2Rust_Unnamed = 32;
pub const XML_ROLE_ATTRIBUTE_ENUM_VALUE: C2Rust_Unnamed = 31;
pub const XML_ROLE_ATTRIBUTE_TYPE_NMTOKENS: C2Rust_Unnamed = 30;
pub const XML_ROLE_ATTRIBUTE_TYPE_NMTOKEN: C2Rust_Unnamed = 29;
pub const XML_ROLE_ATTRIBUTE_TYPE_ENTITIES: C2Rust_Unnamed = 28;
pub const XML_ROLE_ATTRIBUTE_TYPE_ENTITY: C2Rust_Unnamed = 27;
pub const XML_ROLE_ATTRIBUTE_TYPE_IDREFS: C2Rust_Unnamed = 26;
pub const XML_ROLE_ATTRIBUTE_TYPE_IDREF: C2Rust_Unnamed = 25;
pub const XML_ROLE_ATTRIBUTE_TYPE_ID: C2Rust_Unnamed = 24;
pub const XML_ROLE_ATTRIBUTE_TYPE_CDATA: C2Rust_Unnamed = 23;
pub const XML_ROLE_ATTRIBUTE_NAME: C2Rust_Unnamed = 22;
pub const XML_ROLE_NOTATION_PUBLIC_ID: C2Rust_Unnamed = 21;
pub const XML_ROLE_NOTATION_NO_SYSTEM_ID: C2Rust_Unnamed = 20;
pub const XML_ROLE_NOTATION_SYSTEM_ID: C2Rust_Unnamed = 19;
pub const XML_ROLE_NOTATION_NAME: C2Rust_Unnamed = 18;
pub const XML_ROLE_NOTATION_NONE: C2Rust_Unnamed = 17;
pub const XML_ROLE_ENTITY_NOTATION_NAME: C2Rust_Unnamed = 16;
pub const XML_ROLE_ENTITY_COMPLETE: C2Rust_Unnamed = 15;
pub const XML_ROLE_ENTITY_PUBLIC_ID: C2Rust_Unnamed = 14;
pub const XML_ROLE_ENTITY_SYSTEM_ID: C2Rust_Unnamed = 13;
pub const XML_ROLE_ENTITY_VALUE: C2Rust_Unnamed = 12;
pub const XML_ROLE_ENTITY_NONE: C2Rust_Unnamed = 11;
pub const XML_ROLE_PARAM_ENTITY_NAME: C2Rust_Unnamed = 10;
pub const XML_ROLE_GENERAL_ENTITY_NAME: C2Rust_Unnamed = 9;
pub const XML_ROLE_DOCTYPE_CLOSE: C2Rust_Unnamed = 8;
pub const XML_ROLE_DOCTYPE_INTERNAL_SUBSET: C2Rust_Unnamed = 7;
pub const XML_ROLE_DOCTYPE_PUBLIC_ID: C2Rust_Unnamed = 6;
pub const XML_ROLE_DOCTYPE_SYSTEM_ID: C2Rust_Unnamed = 5;
pub const XML_ROLE_DOCTYPE_NAME: C2Rust_Unnamed = 4;
pub const XML_ROLE_DOCTYPE_NONE: C2Rust_Unnamed = 3;
pub const XML_ROLE_INSTANCE_START: C2Rust_Unnamed = 2;
pub const XML_ROLE_XML_DECL: C2Rust_Unnamed = 1;
pub const XML_ROLE_NONE: C2Rust_Unnamed = 0;
pub const XML_ROLE_ERROR: C2Rust_Unnamed = -1;
#[derive(Copy, Clone)]
#[repr(C)]
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
pub type PROLOG_STATE = prolog_state;
pub type PROLOG_HANDLER = unsafe extern "C" fn(
    *mut PROLOG_STATE,
    ::core::ffi::c_int,
    *const ::core::ffi::c_char,
    *const ::core::ffi::c_char,
    *const ENCODING,
) -> ::core::ffi::c_int;
pub const XML_TOK_NONE: ::core::ffi::c_int = -4;
pub const XML_TOK_PI: ::core::ffi::c_int = 11;
pub const XML_TOK_XML_DECL: ::core::ffi::c_int = 12;
pub const XML_TOK_COMMENT: ::core::ffi::c_int = 13;
pub const XML_TOK_BOM: ::core::ffi::c_int = 14;
pub const XML_TOK_PROLOG_S: ::core::ffi::c_int = 15;
pub const XML_TOK_DECL_OPEN: ::core::ffi::c_int = 16;
pub const XML_TOK_DECL_CLOSE: ::core::ffi::c_int = 17;
pub const XML_TOK_NAME: ::core::ffi::c_int = 18;
pub const XML_TOK_NMTOKEN: ::core::ffi::c_int = 19;
pub const XML_TOK_POUND_NAME: ::core::ffi::c_int = 20;
pub const XML_TOK_OR: ::core::ffi::c_int = 21;
pub const XML_TOK_PERCENT: ::core::ffi::c_int = 22;
pub const XML_TOK_OPEN_PAREN: ::core::ffi::c_int = 23;
pub const XML_TOK_CLOSE_PAREN: ::core::ffi::c_int = 24;
pub const XML_TOK_OPEN_BRACKET: ::core::ffi::c_int = 25;
pub const XML_TOK_CLOSE_BRACKET: ::core::ffi::c_int = 26;
pub const XML_TOK_LITERAL: ::core::ffi::c_int = 27;
pub const XML_TOK_PARAM_ENTITY_REF: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const XML_TOK_INSTANCE_START: ::core::ffi::c_int = 29;
pub const XML_TOK_NAME_QUESTION: ::core::ffi::c_int = 30;
pub const XML_TOK_NAME_ASTERISK: ::core::ffi::c_int = 31;
pub const XML_TOK_NAME_PLUS: ::core::ffi::c_int = 32;
pub const XML_TOK_COND_SECT_OPEN: ::core::ffi::c_int = 33;
pub const XML_TOK_COND_SECT_CLOSE: ::core::ffi::c_int = 34;
pub const XML_TOK_CLOSE_PAREN_QUESTION: ::core::ffi::c_int = 35;
pub const XML_TOK_CLOSE_PAREN_ASTERISK: ::core::ffi::c_int = 36;
pub const XML_TOK_CLOSE_PAREN_PLUS: ::core::ffi::c_int = 37;
pub const XML_TOK_COMMA: ::core::ffi::c_int = 38;
pub const XML_TOK_PREFIXED_NAME: ::core::ffi::c_int = 41;
pub const ASCII_A: ::core::ffi::c_int = 0x41 as ::core::ffi::c_int;
pub const ASCII_B: ::core::ffi::c_int = 0x42 as ::core::ffi::c_int;
pub const ASCII_C: ::core::ffi::c_int = 0x43 as ::core::ffi::c_int;
pub const ASCII_D: ::core::ffi::c_int = 0x44 as ::core::ffi::c_int;
pub const ASCII_E: ::core::ffi::c_int = 0x45 as ::core::ffi::c_int;
pub const ASCII_F: ::core::ffi::c_int = 0x46 as ::core::ffi::c_int;
pub const ASCII_G: ::core::ffi::c_int = 0x47 as ::core::ffi::c_int;
pub const ASCII_I: ::core::ffi::c_int = 0x49 as ::core::ffi::c_int;
pub const ASCII_K: ::core::ffi::c_int = 0x4b as ::core::ffi::c_int;
pub const ASCII_L: ::core::ffi::c_int = 0x4c as ::core::ffi::c_int;
pub const ASCII_M: ::core::ffi::c_int = 0x4d as ::core::ffi::c_int;
pub const ASCII_N: ::core::ffi::c_int = 0x4e as ::core::ffi::c_int;
pub const ASCII_O: ::core::ffi::c_int = 0x4f as ::core::ffi::c_int;
pub const ASCII_P: ::core::ffi::c_int = 0x50 as ::core::ffi::c_int;
pub const ASCII_Q: ::core::ffi::c_int = 0x51 as ::core::ffi::c_int;
pub const ASCII_R: ::core::ffi::c_int = 0x52 as ::core::ffi::c_int;
pub const ASCII_S: ::core::ffi::c_int = 0x53 as ::core::ffi::c_int;
pub const ASCII_T: ::core::ffi::c_int = 0x54 as ::core::ffi::c_int;
pub const ASCII_U: ::core::ffi::c_int = 0x55 as ::core::ffi::c_int;
pub const ASCII_X: ::core::ffi::c_int = 0x58 as ::core::ffi::c_int;
pub const ASCII_Y: ::core::ffi::c_int = 0x59 as ::core::ffi::c_int;
static KW_ANY: [::core::ffi::c_char; 4] = [
    ASCII_A as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    ASCII_Y as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_ATTLIST: [::core::ffi::c_char; 8] = [
    ASCII_A as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_L as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_S as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_CDATA: [::core::ffi::c_char; 6] = [
    ASCII_C as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    ASCII_A as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_A as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_DOCTYPE: [::core::ffi::c_char; 8] = [
    ASCII_D as ::core::ffi::c_char,
    ASCII_O as ::core::ffi::c_char,
    ASCII_C as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_Y as ::core::ffi::c_char,
    ASCII_P as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_ELEMENT: [::core::ffi::c_char; 8] = [
    ASCII_E as ::core::ffi::c_char,
    ASCII_L as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_M as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_EMPTY: [::core::ffi::c_char; 6] = [
    ASCII_E as ::core::ffi::c_char,
    ASCII_M as ::core::ffi::c_char,
    ASCII_P as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_Y as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_ENTITIES: [::core::ffi::c_char; 9] = [
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
static KW_ENTITY: [::core::ffi::c_char; 7] = [
    ASCII_E as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_Y as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_FIXED: [::core::ffi::c_char; 6] = [
    ASCII_F as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_X as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_ID: [::core::ffi::c_char; 3] = [
    ASCII_I as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_IDREF: [::core::ffi::c_char; 6] = [
    ASCII_I as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    ASCII_R as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_F as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_IDREFS: [::core::ffi::c_char; 7] = [
    ASCII_I as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    ASCII_R as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_F as ::core::ffi::c_char,
    ASCII_S as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_IGNORE: [::core::ffi::c_char; 7] = [
    ASCII_I as ::core::ffi::c_char,
    ASCII_G as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    ASCII_O as ::core::ffi::c_char,
    ASCII_R as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_IMPLIED: [::core::ffi::c_char; 8] = [
    ASCII_I as ::core::ffi::c_char,
    ASCII_M as ::core::ffi::c_char,
    ASCII_P as ::core::ffi::c_char,
    ASCII_L as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_INCLUDE: [::core::ffi::c_char; 8] = [
    ASCII_I as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    ASCII_C as ::core::ffi::c_char,
    ASCII_L as ::core::ffi::c_char,
    ASCII_U as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_NDATA: [::core::ffi::c_char; 6] = [
    ASCII_N as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    ASCII_A as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_A as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_NMTOKEN: [::core::ffi::c_char; 8] = [
    ASCII_N as ::core::ffi::c_char,
    ASCII_M as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_O as ::core::ffi::c_char,
    ASCII_K as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_NMTOKENS: [::core::ffi::c_char; 9] = [
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
static KW_NOTATION: [::core::ffi::c_char; 9] = [
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
static KW_PCDATA: [::core::ffi::c_char; 7] = [
    ASCII_P as ::core::ffi::c_char,
    ASCII_C as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    ASCII_A as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_A as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_PUBLIC: [::core::ffi::c_char; 7] = [
    ASCII_P as ::core::ffi::c_char,
    ASCII_U as ::core::ffi::c_char,
    ASCII_B as ::core::ffi::c_char,
    ASCII_L as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_C as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_REQUIRED: [::core::ffi::c_char; 9] = [
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
static KW_SYSTEM: [::core::ffi::c_char; 7] = [
    ASCII_S as ::core::ffi::c_char,
    ASCII_Y as ::core::ffi::c_char,
    ASCII_S as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_M as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
extern "C" fn prolog0(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => {
            set_handler(state, prolog1);
            XML_ROLE_NONE as ::core::ffi::c_int
        }
        XML_TOK_XML_DECL => {
            set_handler(state, prolog1);
            XML_ROLE_XML_DECL as ::core::ffi::c_int
        }
        XML_TOK_PI => {
            set_handler(state, prolog1);
            XML_ROLE_PI as ::core::ffi::c_int
        }
        XML_TOK_COMMENT => {
            set_handler(state, prolog1);
            XML_ROLE_COMMENT as ::core::ffi::c_int
        }
        XML_TOK_BOM => XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_DECL_OPEN
            if decl_matches_ascii(
                enc,
                ptr,
                end,
                &raw const KW_DOCTYPE as *const ::core::ffi::c_char,
            ) =>
        {
            set_handler(state, doctype0);
            XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int
        }
        XML_TOK_INSTANCE_START => {
            set_handler(state, error);
            XML_ROLE_INSTANCE_START as ::core::ffi::c_int
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn prolog1(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S | XML_TOK_BOM => XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_PI => XML_ROLE_PI as ::core::ffi::c_int,
        XML_TOK_COMMENT => XML_ROLE_COMMENT as ::core::ffi::c_int,
        XML_TOK_DECL_OPEN
            if decl_matches_ascii(
                enc,
                ptr,
                end,
                &raw const KW_DOCTYPE as *const ::core::ffi::c_char,
            ) =>
        {
            set_handler(state, doctype0);
            XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int
        }
        XML_TOK_INSTANCE_START => {
            set_handler(state, error);
            XML_ROLE_INSTANCE_START as ::core::ffi::c_int
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn prolog2(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    _ptr: *const ::core::ffi::c_char,
    _end: *const ::core::ffi::c_char,
    _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_PI => XML_ROLE_PI as ::core::ffi::c_int,
        XML_TOK_COMMENT => XML_ROLE_COMMENT as ::core::ffi::c_int,
        XML_TOK_INSTANCE_START => {
            set_handler(state, error);
            XML_ROLE_INSTANCE_START as ::core::ffi::c_int
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn doctype0(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    _ptr: *const ::core::ffi::c_char,
    _end: *const ::core::ffi::c_char,
    _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int,
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            set_handler(state, doctype1);
            XML_ROLE_DOCTYPE_NAME as ::core::ffi::c_int
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn doctype1(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int,
        XML_TOK_OPEN_BRACKET => {
            set_handler(state, internalSubset);
            XML_ROLE_DOCTYPE_INTERNAL_SUBSET as ::core::ffi::c_int
        }
        XML_TOK_DECL_CLOSE => {
            set_handler(state, prolog2);
            XML_ROLE_DOCTYPE_CLOSE as ::core::ffi::c_int
        }
        XML_TOK_NAME
            if matches_ascii(
                enc,
                ptr,
                end,
                &raw const KW_SYSTEM as *const ::core::ffi::c_char,
            ) =>
        {
            set_handler(state, doctype3);
            XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int
        }
        XML_TOK_NAME
            if matches_ascii(
                enc,
                ptr,
                end,
                &raw const KW_PUBLIC as *const ::core::ffi::c_char,
            ) =>
        {
            set_handler(state, doctype2);
            XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn doctype2(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    _ptr: *const ::core::ffi::c_char,
    _end: *const ::core::ffi::c_char,
    _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            set_handler(state, doctype3);
            XML_ROLE_DOCTYPE_PUBLIC_ID as ::core::ffi::c_int
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn doctype3(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    _ptr: *const ::core::ffi::c_char,
    _end: *const ::core::ffi::c_char,
    _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            set_handler(state, doctype4);
            XML_ROLE_DOCTYPE_SYSTEM_ID as ::core::ffi::c_int
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn doctype4(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    _ptr: *const ::core::ffi::c_char,
    _end: *const ::core::ffi::c_char,
    _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int,
        XML_TOK_OPEN_BRACKET => {
            set_handler(state, internalSubset);
            XML_ROLE_DOCTYPE_INTERNAL_SUBSET as ::core::ffi::c_int
        }
        XML_TOK_DECL_CLOSE => {
            set_handler(state, prolog2);
            XML_ROLE_DOCTYPE_CLOSE as ::core::ffi::c_int
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn doctype5(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    _ptr: *const ::core::ffi::c_char,
    _end: *const ::core::ffi::c_char,
    _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int,
        XML_TOK_DECL_CLOSE => {
            set_handler(state, prolog2);
            XML_ROLE_DOCTYPE_CLOSE as ::core::ffi::c_int
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn internalSubset(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_DECL_OPEN
            if decl_matches_ascii(
                enc,
                ptr,
                end,
                &raw const KW_ENTITY as *const ::core::ffi::c_char,
            ) =>
        {
            set_handler(state, entity0);
            XML_ROLE_ENTITY_NONE as ::core::ffi::c_int
        }
        XML_TOK_DECL_OPEN
            if decl_matches_ascii(
                enc,
                ptr,
                end,
                &raw const KW_ATTLIST as *const ::core::ffi::c_char,
            ) =>
        {
            set_handler(state, attlist0);
            XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int
        }
        XML_TOK_DECL_OPEN
            if decl_matches_ascii(
                enc,
                ptr,
                end,
                &raw const KW_ELEMENT as *const ::core::ffi::c_char,
            ) =>
        {
            set_handler(state, element0);
            XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int
        }
        XML_TOK_DECL_OPEN
            if decl_matches_ascii(
                enc,
                ptr,
                end,
                &raw const KW_NOTATION as *const ::core::ffi::c_char,
            ) =>
        {
            set_handler(state, notation0);
            XML_ROLE_NOTATION_NONE as ::core::ffi::c_int
        }
        XML_TOK_PI => XML_ROLE_PI as ::core::ffi::c_int,
        XML_TOK_COMMENT => XML_ROLE_COMMENT as ::core::ffi::c_int,
        XML_TOK_PARAM_ENTITY_REF => XML_ROLE_PARAM_ENTITY_REF as ::core::ffi::c_int,
        XML_TOK_CLOSE_BRACKET => {
            set_handler(state, doctype5);
            XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int
        }
        XML_TOK_NONE => XML_ROLE_NONE as ::core::ffi::c_int,
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn externalSubset0(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);
    set_handler(state, externalSubset1);

    if tok == XML_TOK_XML_DECL {
        XML_ROLE_TEXT_DECL as ::core::ffi::c_int
    } else {
        externalSubset1(state as *mut PROLOG_STATE, tok, ptr, end, enc)
    }
}

extern "C" fn externalSubset1(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_COND_SECT_OPEN => {
            set_handler(state, condSect0);
            XML_ROLE_NONE as ::core::ffi::c_int
        }
        XML_TOK_COND_SECT_CLOSE if state.includeLevel != 0 => {
            state.includeLevel = state.includeLevel.wrapping_sub(1 as ::core::ffi::c_uint);
            XML_ROLE_NONE as ::core::ffi::c_int
        }
        XML_TOK_PROLOG_S => XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_CLOSE_BRACKET => common(state as *mut PROLOG_STATE, tok),
        XML_TOK_NONE if state.includeLevel == 0 => XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_NONE => common(state as *mut PROLOG_STATE, tok),
        _ => internalSubset(state as *mut PROLOG_STATE, tok, ptr, end, enc),
    }
}
unsafe extern "C" fn entity0(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
}
unsafe extern "C" fn entity1(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
}
unsafe extern "C" fn entity2(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
}
unsafe extern "C" fn entity3(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
}
unsafe extern "C" fn entity4(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
}
unsafe extern "C" fn entity5(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
                            )
                                -> ::core::ffi::c_int,
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
                            )
                                -> ::core::ffi::c_int,
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
                    return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int;
                }
            }
            _ => {}
        }
        return common(state, tok);
    }
}
unsafe extern "C" fn entity6(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
}
unsafe extern "C" fn entity7(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
}
unsafe extern "C" fn entity8(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
}
unsafe extern "C" fn entity9(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
}
unsafe extern "C" fn entity10(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
                            )
                                -> ::core::ffi::c_int,
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
                            )
                                -> ::core::ffi::c_int,
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
}
unsafe extern "C" fn notation0(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
}
unsafe extern "C" fn notation1(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
                    return XML_ROLE_NOTATION_NONE as ::core::ffi::c_int;
                }
            }
            _ => {}
        }
        return common(state, tok);
    }
}
unsafe extern "C" fn notation2(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
}
unsafe extern "C" fn notation3(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
}
unsafe extern "C" fn notation4(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
                            )
                                -> ::core::ffi::c_int,
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
                            )
                                -> ::core::ffi::c_int,
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
}
extern "C" fn attlist0(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    _ptr: *const ::core::ffi::c_char,
    _end: *const ::core::ffi::c_char,
    _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            set_handler(state, attlist1);
            XML_ROLE_ATTLIST_ELEMENT_NAME as ::core::ffi::c_int
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn attlist1(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    _ptr: *const ::core::ffi::c_char,
    _end: *const ::core::ffi::c_char,
    _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_DECL_CLOSE => {
            state.handler = Some(next_subset_handler(state.documentEntity));
            XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int
        }
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            set_handler(state, attlist2);
            XML_ROLE_ATTRIBUTE_NAME as ::core::ffi::c_int
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn attlist2(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            let types = [
                &raw const KW_CDATA as *const ::core::ffi::c_char,
                &raw const KW_ID as *const ::core::ffi::c_char,
                &raw const KW_IDREF as *const ::core::ffi::c_char,
                &raw const KW_IDREFS as *const ::core::ffi::c_char,
                &raw const KW_ENTITY as *const ::core::ffi::c_char,
                &raw const KW_ENTITIES as *const ::core::ffi::c_char,
                &raw const KW_NMTOKEN as *const ::core::ffi::c_char,
                &raw const KW_NMTOKENS as *const ::core::ffi::c_char,
            ];

            if let Some(index) = types
                .iter()
                .position(|&keyword| matches_ascii(enc, ptr, end, keyword))
            {
                set_handler(state, attlist8);
                return XML_ROLE_ATTRIBUTE_TYPE_CDATA as ::core::ffi::c_int
                    + index as ::core::ffi::c_int;
            }

            if matches_ascii(
                enc,
                ptr,
                end,
                &raw const KW_NOTATION as *const ::core::ffi::c_char,
            ) {
                set_handler(state, attlist5);
                return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int;
            }

            common(state as *mut PROLOG_STATE, tok)
        }
        XML_TOK_OPEN_PAREN => {
            set_handler(state, attlist3);
            XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn attlist3(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    _ptr: *const ::core::ffi::c_char,
    _end: *const ::core::ffi::c_char,
    _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_NMTOKEN | XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            set_handler(state, attlist4);
            XML_ROLE_ATTRIBUTE_ENUM_VALUE as ::core::ffi::c_int
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn attlist4(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    _ptr: *const ::core::ffi::c_char,
    _end: *const ::core::ffi::c_char,
    _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_CLOSE_PAREN => {
            set_handler(state, attlist8);
            XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int
        }
        XML_TOK_OR => {
            set_handler(state, attlist3);
            XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn attlist5(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    _ptr: *const ::core::ffi::c_char,
    _end: *const ::core::ffi::c_char,
    _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_OPEN_PAREN => {
            set_handler(state, attlist6);
            XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn attlist6(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    _ptr: *const ::core::ffi::c_char,
    _end: *const ::core::ffi::c_char,
    _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            set_handler(state, attlist7);
            XML_ROLE_ATTRIBUTE_NOTATION_VALUE as ::core::ffi::c_int
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn attlist7(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    _ptr: *const ::core::ffi::c_char,
    _end: *const ::core::ffi::c_char,
    _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_CLOSE_PAREN => {
            set_handler(state, attlist8);
            XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int
        }
        XML_TOK_OR => {
            set_handler(state, attlist6);
            XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn attlist8(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_POUND_NAME => {
            if pound_name_matches_ascii(
                enc,
                ptr,
                end,
                &raw const KW_IMPLIED as *const ::core::ffi::c_char,
            ) {
                set_handler(state, attlist1);
                return XML_ROLE_IMPLIED_ATTRIBUTE_VALUE as ::core::ffi::c_int;
            }

            if pound_name_matches_ascii(
                enc,
                ptr,
                end,
                &raw const KW_REQUIRED as *const ::core::ffi::c_char,
            ) {
                set_handler(state, attlist1);
                return XML_ROLE_REQUIRED_ATTRIBUTE_VALUE as ::core::ffi::c_int;
            }

            if pound_name_matches_ascii(
                enc,
                ptr,
                end,
                &raw const KW_FIXED as *const ::core::ffi::c_char,
            ) {
                set_handler(state, attlist9);
                return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int;
            }

            common(state as *mut PROLOG_STATE, tok)
        }
        XML_TOK_LITERAL => {
            set_handler(state, attlist1);
            XML_ROLE_DEFAULT_ATTRIBUTE_VALUE as ::core::ffi::c_int
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn attlist9(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    _ptr: *const ::core::ffi::c_char,
    _end: *const ::core::ffi::c_char,
    _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            set_handler(state, attlist1);
            XML_ROLE_FIXED_ATTRIBUTE_VALUE as ::core::ffi::c_int
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}
unsafe extern "C" fn element0(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
}
unsafe extern "C" fn element1(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
}
unsafe extern "C" fn element2(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
}
unsafe extern "C" fn element3(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
}
unsafe extern "C" fn element4(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
}
unsafe extern "C" fn element5(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
}
unsafe extern "C" fn element6(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
}
unsafe extern "C" fn element7(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    unsafe {
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
}
fn state_mut<'a>(state: *mut PROLOG_STATE) -> &'a mut PROLOG_STATE {
    unsafe { &mut *state }
}

fn encoding_ref<'a>(enc: *const ENCODING) -> &'a ENCODING {
    unsafe { &*enc }
}

fn set_handler(state: &mut PROLOG_STATE, handler: PROLOG_HANDLER) {
    state.handler = Some(handler);
}

fn decl_name_ptr(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let prefix_len = (2 as ::core::ffi::c_int * encoding_ref(enc).minBytesPerChar) as usize;
    ptr.wrapping_add(prefix_len)
}

fn matches_ascii(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    keyword: *const ::core::ffi::c_char,
) -> bool {
    unsafe {
        encoding_ref(enc)
            .nameMatchesAscii
            .expect("non-null function pointer")(enc, ptr, end, keyword)
            != 0
    }
}

fn decl_matches_ascii(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    keyword: *const ::core::ffi::c_char,
) -> bool {
    matches_ascii(enc, decl_name_ptr(enc, ptr), end, keyword)
}

fn ptr_after_char(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    ptr.wrapping_add(encoding_ref(enc).minBytesPerChar as usize)
}

fn pound_name_matches_ascii(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    keyword: *const ::core::ffi::c_char,
) -> bool {
    matches_ascii(enc, ptr_after_char(enc, ptr), end, keyword)
}

fn next_subset_handler(document_entity: ::core::ffi::c_int) -> PROLOG_HANDLER {
    if document_entity != 0 {
        internalSubset as PROLOG_HANDLER
    } else {
        externalSubset1 as PROLOG_HANDLER
    }
}

enum ConditionalKeyword {
    Ignore,
    Include,
}

fn matches_conditional_keyword(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    keyword: ConditionalKeyword,
) -> bool {
    let keyword = match keyword {
        ConditionalKeyword::Ignore => &raw const KW_IGNORE as *const ::core::ffi::c_char,
        ConditionalKeyword::Include => &raw const KW_INCLUDE as *const ::core::ffi::c_char,
    };

    matches_ascii(enc, ptr, end, keyword)
}

extern "C" fn condSect0(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            if matches_conditional_keyword(enc, ptr, end, ConditionalKeyword::Include) {
                state.handler = Some(condSect1);
                return XML_ROLE_NONE as ::core::ffi::c_int;
            }

            if matches_conditional_keyword(enc, ptr, end, ConditionalKeyword::Ignore) {
                state.handler = Some(condSect2);
                return XML_ROLE_NONE as ::core::ffi::c_int;
            }

            common(state as *mut PROLOG_STATE, tok)
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn condSect1(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    _ptr: *const ::core::ffi::c_char,
    _end: *const ::core::ffi::c_char,
    _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_OPEN_BRACKET => {
            state.handler = Some(externalSubset1);
            state.includeLevel = state.includeLevel.wrapping_add(1 as ::core::ffi::c_uint);
            XML_ROLE_NONE as ::core::ffi::c_int
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn condSect2(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    _ptr: *const ::core::ffi::c_char,
    _end: *const ::core::ffi::c_char,
    _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_OPEN_BRACKET => {
            state.handler = Some(externalSubset1);
            XML_ROLE_IGNORE_SECT as ::core::ffi::c_int
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn declClose(
    state: *mut PROLOG_STATE,
    tok: ::core::ffi::c_int,
    _ptr: *const ::core::ffi::c_char,
    _end: *const ::core::ffi::c_char,
    _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    let state = state_mut(state);

    match tok {
        XML_TOK_PROLOG_S => state.role_none,
        XML_TOK_DECL_CLOSE => {
            state.handler = Some(if state.documentEntity != 0 {
                internalSubset
            } else {
                externalSubset1
            });
            state.role_none
        }
        _ => common(state as *mut PROLOG_STATE, tok),
    }
}

extern "C" fn error(
    _state: *mut PROLOG_STATE,
    _tok: ::core::ffi::c_int,
    _ptr: *const ::core::ffi::c_char,
    _end: *const ::core::ffi::c_char,
    _enc: *const ENCODING,
) -> ::core::ffi::c_int {
    XML_ROLE_NONE as ::core::ffi::c_int
}

extern "C" fn common(state: *mut PROLOG_STATE, tok: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let state = state_mut(state);

    if state.documentEntity == 0 && tok == XML_TOK_PARAM_ENTITY_REF {
        return XML_ROLE_INNER_PARAM_ENTITY_REF as ::core::ffi::c_int;
    }

    state.handler = Some(error);
    XML_ROLE_ERROR as ::core::ffi::c_int
}
#[no_mangle]
pub unsafe extern "C" fn XmlPrologStateInit(mut state: *mut PROLOG_STATE) {
    let state = state_mut(state);
    state.handler = Some(prolog0);
    state.documentEntity = 1 as ::core::ffi::c_int;
    state.includeLevel = 0 as ::core::ffi::c_uint;
    state.inEntityValue = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn XmlPrologStateInitExternalEntity(mut state: *mut PROLOG_STATE) {
    let state = state_mut(state);
    state.handler = Some(externalSubset0);
    state.documentEntity = 0 as ::core::ffi::c_int;
    state.includeLevel = 0 as ::core::ffi::c_uint;
}
