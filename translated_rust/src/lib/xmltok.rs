pub type ptrdiff_t = isize;
pub type size_t = usize;
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
        extern "C" fn(
            *const ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub nameLength:
        Option<extern "C" fn(*const ENCODING, *const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub skipS: Option<
        extern "C" fn(*const ENCODING, *const ::core::ffi::c_char) -> *const ::core::ffi::c_char,
    >,
    pub getAtts: Option<
        extern "C" fn(
            *const ENCODING,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut ATTRIBUTE,
        ) -> ::core::ffi::c_int,
    >,
    pub charRefNumber:
        Option<extern "C" fn(*const ENCODING, *const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub predefinedEntityName: Option<
        extern "C" fn(
            *const ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub updatePosition: Option<
        extern "C" fn(
            *const ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut POSITION,
        ) -> (),
    >,
    pub isPublicId: Option<
        extern "C" fn(
            *const ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub utf8Convert: Option<
        extern "C" fn(
            *const ENCODING,
            *mut *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut *mut ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> XML_Convert_Result,
    >,
    pub utf16Convert: Option<
        extern "C" fn(
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
    extern "C" fn(
        *const ENCODING,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INIT_ENCODING {
    pub initEnc: ENCODING,
    pub encPtr: *mut *const ENCODING,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct normal_encoding {
    pub enc: ENCODING,
    pub type_0: [::core::ffi::c_uchar; 256],
    pub isName2:
        Option<extern "C" fn(*const ENCODING, *const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub isName3:
        Option<extern "C" fn(*const ENCODING, *const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub isName4:
        Option<extern "C" fn(*const ENCODING, *const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub isNmstrt2:
        Option<extern "C" fn(*const ENCODING, *const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub isNmstrt3:
        Option<extern "C" fn(*const ENCODING, *const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub isNmstrt4:
        Option<extern "C" fn(*const ENCODING, *const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub isInvalid2:
        Option<extern "C" fn(*const ENCODING, *const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub isInvalid3:
        Option<extern "C" fn(*const ENCODING, *const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub isInvalid4:
        Option<extern "C" fn(*const ENCODING, *const ::core::ffi::c_char) -> ::core::ffi::c_int>,
}
pub const BT_MALFORM: C2Rust_Unnamed_3 = 1;
pub const BT_NONXML: C2Rust_Unnamed_3 = 0;
pub const BT_LEAD4: C2Rust_Unnamed_3 = 7;
pub const BT_LEAD3: C2Rust_Unnamed_3 = 6;
pub const BT_LEAD2: C2Rust_Unnamed_3 = 5;
pub const BT_TRAIL: C2Rust_Unnamed_3 = 8;
pub const BT_OTHER: C2Rust_Unnamed_3 = 28;
pub const BT_VERBAR: C2Rust_Unnamed_3 = 36;
pub const BT_NMSTRT: C2Rust_Unnamed_3 = 22;
pub const BT_HEX: C2Rust_Unnamed_3 = 24;
pub const BT_RSQB: C2Rust_Unnamed_3 = 4;
pub const BT_LSQB: C2Rust_Unnamed_3 = 20;
pub const BT_QUEST: C2Rust_Unnamed_3 = 15;
pub const BT_GT: C2Rust_Unnamed_3 = 11;
pub const BT_EQUALS: C2Rust_Unnamed_3 = 14;
pub const BT_LT: C2Rust_Unnamed_3 = 2;
pub const BT_SEMI: C2Rust_Unnamed_3 = 18;
pub const BT_DIGIT: C2Rust_Unnamed_3 = 25;
pub const BT_SOL: C2Rust_Unnamed_3 = 17;
pub const BT_NAME: C2Rust_Unnamed_3 = 26;
pub const BT_MINUS: C2Rust_Unnamed_3 = 27;
pub const BT_COMMA: C2Rust_Unnamed_3 = 35;
pub const BT_PLUS: C2Rust_Unnamed_3 = 34;
pub const BT_AST: C2Rust_Unnamed_3 = 33;
pub const BT_RPAR: C2Rust_Unnamed_3 = 32;
pub const BT_LPAR: C2Rust_Unnamed_3 = 31;
pub const BT_APOS: C2Rust_Unnamed_3 = 13;
pub const BT_AMP: C2Rust_Unnamed_3 = 3;
pub const BT_PERCNT: C2Rust_Unnamed_3 = 30;
pub const BT_NUM: C2Rust_Unnamed_3 = 19;
pub const BT_QUOT: C2Rust_Unnamed_3 = 12;
pub const BT_EXCL: C2Rust_Unnamed_3 = 16;
pub const BT_S: C2Rust_Unnamed_3 = 21;
pub const BT_CR: C2Rust_Unnamed_3 = 9;
pub const BT_LF: C2Rust_Unnamed_3 = 10;
pub const BT_COLON_0: C2Rust_Unnamed_3 = 23;
pub const UTF8_cval2: C2Rust_Unnamed_4 = 192;
pub const inValue: C2Rust_Unnamed = 2;
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const inName: C2Rust_Unnamed = 1;
pub const other: C2Rust_Unnamed = 0;
pub const BT_NONASCII: C2Rust_Unnamed_3 = 29;
pub const UTF8_cval4: C2Rust_Unnamed_4 = 240;
pub const UTF8_cval3: C2Rust_Unnamed_4 = 224;
pub const inValue_0: C2Rust_Unnamed_0 = 2;
pub type C2Rust_Unnamed_0 = ::core::ffi::c_uint;
pub const inName_0: C2Rust_Unnamed_0 = 1;
pub const other_0: C2Rust_Unnamed_0 = 0;
pub const inValue_1: C2Rust_Unnamed_1 = 2;
pub type C2Rust_Unnamed_1 = ::core::ffi::c_uint;
pub const inName_1: C2Rust_Unnamed_1 = 1;
pub const other_1: C2Rust_Unnamed_1 = 0;
pub const UNKNOWN_ENC: C2Rust_Unnamed_5 = -1;
pub const NO_ENC: C2Rust_Unnamed_5 = 6;
pub const UTF_16LE_ENC: C2Rust_Unnamed_5 = 5;
pub const UTF_16BE_ENC: C2Rust_Unnamed_5 = 4;
pub const UTF_8_ENC: C2Rust_Unnamed_5 = 2;
pub const UTF_16_ENC: C2Rust_Unnamed_5 = 3;
pub const ISO_8859_1_ENC: C2Rust_Unnamed_5 = 0;
pub const min4: C2Rust_Unnamed_2 = 65536;
pub const min3: C2Rust_Unnamed_2 = 2048;
pub const UTF8_cval1: C2Rust_Unnamed_4 = 0;
pub const min2: C2Rust_Unnamed_2 = 128;
pub type C2Rust_Unnamed_2 = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unknown_encoding {
    pub normal: normal_encoding,
    pub convert: CONVERTER,
    pub userData: *mut ::core::ffi::c_void,
    pub utf16: [::core::ffi::c_ushort; 256],
    pub utf8: [[::core::ffi::c_char; 4]; 256],
}
pub type CONVERTER = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
>;
pub type C2Rust_Unnamed_3 = ::core::ffi::c_uint;
pub type C2Rust_Unnamed_4 = ::core::ffi::c_uint;
pub type C2Rust_Unnamed_5 = ::core::ffi::c_int;
pub const US_ASCII_ENC: C2Rust_Unnamed_5 = 1;
pub const XML_TOK_TRAILING_RSQB: ::core::ffi::c_int = -(5 as ::core::ffi::c_int);
pub const XML_TOK_NONE: ::core::ffi::c_int = -(4 as ::core::ffi::c_int);
pub const XML_TOK_TRAILING_CR: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const XML_TOK_PARTIAL_CHAR: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const XML_TOK_PARTIAL: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const XML_TOK_INVALID: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XML_TOK_START_TAG_WITH_ATTS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const XML_TOK_START_TAG_NO_ATTS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const XML_TOK_EMPTY_ELEMENT_WITH_ATTS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const XML_TOK_EMPTY_ELEMENT_NO_ATTS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const XML_TOK_END_TAG: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const XML_TOK_DATA_CHARS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const XML_TOK_DATA_NEWLINE: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const XML_TOK_CDATA_SECT_OPEN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const XML_TOK_ENTITY_REF: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const XML_TOK_CHAR_REF: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const XML_TOK_PI: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const XML_TOK_XML_DECL: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const XML_TOK_COMMENT: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const XML_TOK_BOM: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const XML_TOK_PROLOG_S: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const XML_TOK_DECL_OPEN: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const XML_TOK_DECL_CLOSE: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const XML_TOK_NAME: ::core::ffi::c_int = 18;
pub const XML_TOK_NMTOKEN: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const XML_TOK_POUND_NAME: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const XML_TOK_OR: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const XML_TOK_PERCENT: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const XML_TOK_OPEN_PAREN: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const XML_TOK_CLOSE_PAREN: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const XML_TOK_OPEN_BRACKET: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const XML_TOK_CLOSE_BRACKET: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const XML_TOK_LITERAL: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const XML_TOK_PARAM_ENTITY_REF: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const XML_TOK_INSTANCE_START: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const XML_TOK_NAME_QUESTION: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const XML_TOK_NAME_ASTERISK: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const XML_TOK_NAME_PLUS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const XML_TOK_COND_SECT_OPEN: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const XML_TOK_COND_SECT_CLOSE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const XML_TOK_CLOSE_PAREN_QUESTION: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
pub const XML_TOK_CLOSE_PAREN_ASTERISK: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const XML_TOK_CLOSE_PAREN_PLUS: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const XML_TOK_COMMA: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const XML_TOK_ATTRIBUTE_VALUE_S: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
pub const XML_TOK_CDATA_SECT_CLOSE: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const XML_TOK_PREFIXED_NAME: ::core::ffi::c_int = 41;
pub const XML_TOK_IGNORE_SECT: ::core::ffi::c_int = 42 as ::core::ffi::c_int;
pub const XML_PROLOG_STATE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XML_CONTENT_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
fn read_c_char_bytes<const N: usize>(p: *const ::core::ffi::c_char) -> [u8; N] {
    ::core::array::from_fn(|index| read_copy(p.cast::<u8>().wrapping_add(index)))
}

fn with_ref<T, R>(ptr: *const T, f: impl FnOnce(&T) -> R) -> R {
    unsafe { f(&*ptr) }
}

fn with_mut<T, R>(ptr: *mut T, f: impl FnOnce(&mut T) -> R) -> R {
    unsafe { f(&mut *ptr) }
}

fn read_copy<T: Copy>(ptr: *const T) -> T {
    with_ref(ptr, |value| *value)
}

fn write_copy<T>(ptr: *mut T, value: T) {
    with_mut(ptr, |slot| *slot = value)
}

fn add_const_c_char(ptr: *const ::core::ffi::c_char, offset: isize) -> *const ::core::ffi::c_char {
    ptr.wrapping_offset(offset)
}

fn add_mut_c_char(ptr: *mut ::core::ffi::c_char, offset: isize) -> *mut ::core::ffi::c_char {
    ptr.wrapping_offset(offset)
}

fn add_mut_c_ushort(ptr: *mut ::core::ffi::c_ushort, offset: isize) -> *mut ::core::ffi::c_ushort {
    ptr.wrapping_offset(offset)
}

fn available_c_chars(ptr: *const ::core::ffi::c_char, end: *const ::core::ffi::c_char) -> usize {
    if end >= ptr {
        end as usize - ptr as usize
    } else {
        0
    }
}

fn has_c_chars(
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    len: usize,
) -> bool {
    available_c_chars(ptr, end) >= len
}

fn set_const_c_char_ptr(slot: *mut *const ::core::ffi::c_char, value: *const ::core::ffi::c_char) {
    write_copy(slot, value)
}

fn normal_is_invalid(
    enc: *const ENCODING,
    width: usize,
    ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    with_ref(enc.cast::<normal_encoding>(), |normal| match width {
        2 => normal.isInvalid2.expect("non-null function pointer")(enc, ptr),
        3 => normal.isInvalid3.expect("non-null function pointer")(enc, ptr),
        4 => normal.isInvalid4.expect("non-null function pointer")(enc, ptr),
        _ => unreachable!("unsupported character width: {width}"),
    })
}

fn normal_is_name_start(
    enc: *const ENCODING,
    width: usize,
    ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    with_ref(enc.cast::<normal_encoding>(), |normal| match width {
        2 => normal.isNmstrt2.expect("non-null function pointer")(enc, ptr),
        3 => normal.isNmstrt3.expect("non-null function pointer")(enc, ptr),
        4 => normal.isNmstrt4.expect("non-null function pointer")(enc, ptr),
        _ => unreachable!("unsupported character width: {width}"),
    })
}

fn normal_is_name_char(
    enc: *const ENCODING,
    width: usize,
    ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    with_ref(enc.cast::<normal_encoding>(), |normal| match width {
        2 => normal.isName2.expect("non-null function pointer")(enc, ptr),
        3 => normal.isName3.expect("non-null function pointer")(enc, ptr),
        4 => normal.isName4.expect("non-null function pointer")(enc, ptr),
        _ => unreachable!("unsupported character width: {width}"),
    })
}

fn little2_code_unit(ptr: *const ::core::ffi::c_char) -> u16 {
    u16::from_le_bytes(read_c_char_bytes::<2>(ptr))
}

fn big2_code_unit(ptr: *const ::core::ffi::c_char) -> u16 {
    u16::from_be_bytes(read_c_char_bytes::<2>(ptr))
}

fn name_bitmap_contains(page_table: &[::core::ffi::c_uchar], code_unit: u16) -> bool {
    let page = page_table[(code_unit >> 8) as usize] as usize;
    let bitmap_index = (page << 3) + (((code_unit & 0xff) >> 5) as usize);
    (namingBitmap[bitmap_index] & (1_u32 << (code_unit & 0x1f))) != 0
}

fn little2_is_name_start(ptr: *const ::core::ffi::c_char) -> bool {
    name_bitmap_contains(&nmstrtPages, little2_code_unit(ptr))
}

fn little2_is_name_char(ptr: *const ::core::ffi::c_char) -> bool {
    name_bitmap_contains(&namePages, little2_code_unit(ptr))
}

fn big2_is_name_start(ptr: *const ::core::ffi::c_char) -> bool {
    name_bitmap_contains(&nmstrtPages, big2_code_unit(ptr))
}

fn big2_is_name_char(ptr: *const ::core::ffi::c_char) -> bool {
    name_bitmap_contains(&namePages, big2_code_unit(ptr))
}

fn normal_name_start_seq_valid(
    enc: *const ENCODING,
    width: usize,
    ptr: *const ::core::ffi::c_char,
) -> bool {
    normal_is_invalid(enc, width, ptr) == 0 && normal_is_name_start(enc, width, ptr) != 0
}

fn normal_name_char_seq_valid(
    enc: *const ENCODING,
    width: usize,
    ptr: *const ::core::ffi::c_char,
) -> bool {
    normal_is_invalid(enc, width, ptr) == 0 && normal_is_name_char(enc, width, ptr) != 0
}

fn reject_multibyte_name_sequence(
    _enc: *const ENCODING,
    _width: usize,
    _ptr: *const ::core::ffi::c_char,
) -> bool {
    false
}

fn read_c_uchar(ptr: *const ::core::ffi::c_char) -> ::core::ffi::c_uchar {
    read_c_char(ptr) as ::core::ffi::c_uchar
}

type AsciiUnitReader = fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int;
type ByteTypeReader = fn(*const ENCODING, *const ::core::ffi::c_char) -> ::core::ffi::c_int;
type AsciiMatcher = fn(*const ::core::ffi::c_char, u8) -> bool;
type PiTargetChecker = extern "C" fn(
    *const ENCODING,
    *const ::core::ffi::c_char,
    *const ::core::ffi::c_char,
    *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int;

static CDATA_LSQB_UNITS: [::core::ffi::c_int; 6] =
    [ASCII_C, ASCII_D, ASCII_A, ASCII_T, ASCII_A, ASCII_LSQB];

#[derive(Copy, Clone, Eq, PartialEq)]
enum AttributeParseState {
    Other,
    InName,
    InValue,
}

fn with_attribute_mut<R>(
    atts: *mut ATTRIBUTE,
    index: ::core::ffi::c_int,
    f: impl FnOnce(&mut ATTRIBUTE) -> R,
) -> R {
    with_mut(atts.wrapping_offset(index as isize), f)
}

fn attribute_value_ptr(
    atts: *mut ATTRIBUTE,
    index: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    with_ref(atts.wrapping_offset(index as isize).cast_const(), |att| {
        att.valuePtr
    })
}

fn attribute_is_normalized(atts: *mut ATTRIBUTE, index: ::core::ffi::c_int) -> bool {
    with_ref(atts.wrapping_offset(index as isize).cast_const(), |att| {
        att.normalized as ::core::ffi::c_int != 0
    })
}

fn set_attribute_name(
    atts: *mut ATTRIBUTE,
    index: ::core::ffi::c_int,
    value: *const ::core::ffi::c_char,
) {
    with_attribute_mut(atts, index, |att| att.name = value)
}

fn set_attribute_value_ptr(
    atts: *mut ATTRIBUTE,
    index: ::core::ffi::c_int,
    value: *const ::core::ffi::c_char,
) {
    with_attribute_mut(atts, index, |att| att.valuePtr = value)
}

fn set_attribute_value_end(
    atts: *mut ATTRIBUTE,
    index: ::core::ffi::c_int,
    value: *const ::core::ffi::c_char,
) {
    with_attribute_mut(atts, index, |att| att.valueEnd = value)
}

fn set_attribute_normalized(
    atts: *mut ATTRIBUTE,
    index: ::core::ffi::c_int,
    value: ::core::ffi::c_char,
) {
    with_attribute_mut(atts, index, |att| att.normalized = value)
}

fn note_attribute_name_start(
    atts: *mut ATTRIBUTE,
    atts_max: ::core::ffi::c_int,
    n_atts: ::core::ffi::c_int,
    ptr: *const ::core::ffi::c_char,
    state: &mut AttributeParseState,
) {
    if *state == AttributeParseState::Other {
        if n_atts < atts_max {
            set_attribute_name(atts, n_atts, ptr);
            set_attribute_normalized(atts, n_atts, 1 as ::core::ffi::c_char);
        }
        *state = AttributeParseState::InName;
    }
}

fn get_atts_with(
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    atts_max: ::core::ffi::c_int,
    atts: *mut ATTRIBUTE,
    unit_size: isize,
    byte_type: ByteTypeReader,
    read_ascii_unit: AsciiUnitReader,
) -> ::core::ffi::c_int {
    let mut state = AttributeParseState::InName;
    let mut n_atts = 0 as ::core::ffi::c_int;
    let mut open = 0 as ::core::ffi::c_int;
    ptr = add_const_c_char(ptr, unit_size);

    loop {
        match byte_type(enc, ptr) {
            5 => {
                note_attribute_name_start(atts, atts_max, n_atts, ptr, &mut state);
                ptr = add_const_c_char(ptr, 2 - unit_size);
            }
            6 => {
                note_attribute_name_start(atts, atts_max, n_atts, ptr, &mut state);
                ptr = add_const_c_char(ptr, 3 - unit_size);
            }
            7 => {
                note_attribute_name_start(atts, atts_max, n_atts, ptr, &mut state);
                ptr = add_const_c_char(ptr, 4 - unit_size);
            }
            29 | 22 | 24 => {
                note_attribute_name_start(atts, atts_max, n_atts, ptr, &mut state);
            }
            12 => {
                if state != AttributeParseState::InValue {
                    if n_atts < atts_max {
                        set_attribute_value_ptr(atts, n_atts, add_const_c_char(ptr, unit_size));
                    }
                    state = AttributeParseState::InValue;
                    open = BT_QUOT as ::core::ffi::c_int;
                } else if open == BT_QUOT as ::core::ffi::c_int {
                    state = AttributeParseState::Other;
                    if n_atts < atts_max {
                        set_attribute_value_end(atts, n_atts, ptr);
                    }
                    n_atts += 1;
                }
            }
            13 => {
                if state != AttributeParseState::InValue {
                    if n_atts < atts_max {
                        set_attribute_value_ptr(atts, n_atts, add_const_c_char(ptr, unit_size));
                    }
                    state = AttributeParseState::InValue;
                    open = BT_APOS as ::core::ffi::c_int;
                } else if open == BT_APOS as ::core::ffi::c_int {
                    state = AttributeParseState::Other;
                    if n_atts < atts_max {
                        set_attribute_value_end(atts, n_atts, ptr);
                    }
                    n_atts += 1;
                }
            }
            3 => {
                if n_atts < atts_max {
                    set_attribute_normalized(atts, n_atts, 0 as ::core::ffi::c_char);
                }
            }
            21 => {
                if state == AttributeParseState::InName {
                    state = AttributeParseState::Other;
                } else if state == AttributeParseState::InValue
                    && n_atts < atts_max
                    && attribute_is_normalized(atts, n_atts)
                    && (ptr == attribute_value_ptr(atts, n_atts)
                        || read_ascii_unit(ptr) != ASCII_SPACE
                        || read_ascii_unit(add_const_c_char(ptr, unit_size)) == ASCII_SPACE
                        || byte_type(enc, add_const_c_char(ptr, unit_size)) == open)
                {
                    set_attribute_normalized(atts, n_atts, 0 as ::core::ffi::c_char);
                }
            }
            9 | 10 => {
                if state == AttributeParseState::InName {
                    state = AttributeParseState::Other;
                } else if state == AttributeParseState::InValue && n_atts < atts_max {
                    set_attribute_normalized(atts, n_atts, 0 as ::core::ffi::c_char);
                }
            }
            11 | 17 => {
                if state != AttributeParseState::InValue {
                    return n_atts;
                }
            }
            _ => {}
        }

        ptr = add_const_c_char(ptr, unit_size);
    }
}

fn read_normal_ascii_unit(ptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    read_c_char(ptr) as ::core::ffi::c_int
}

fn read_little2_ascii_unit(ptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if read_c_char(add_const_c_char(ptr, 1)) as ::core::ffi::c_int == 0 {
        read_c_char(ptr) as ::core::ffi::c_int
    } else {
        -(1 as ::core::ffi::c_int)
    }
}

fn read_big2_ascii_unit(ptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if read_c_char(ptr) as ::core::ffi::c_int == 0 {
        read_c_char(add_const_c_char(ptr, 1)) as ::core::ffi::c_int
    } else {
        -(1 as ::core::ffi::c_int)
    }
}

fn matches_ascii_units(
    mut ptr: *const ::core::ffi::c_char,
    unit_size: isize,
    read_unit: AsciiUnitReader,
    expected: &[::core::ffi::c_int],
) -> bool {
    for &expected_unit in expected {
        if read_unit(ptr) != expected_unit {
            return false;
        }
        ptr = add_const_c_char(ptr, unit_size);
    }
    true
}

fn scan_cdata_section_open_with(
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    unit_size: isize,
    read_unit: AsciiUnitReader,
) -> ::core::ffi::c_int {
    if remaining_const_c_chars(ptr, end) < CDATA_LSQB_UNITS.len() * unit_size as usize {
        return XML_TOK_PARTIAL;
    }
    if !matches_ascii_units(ptr, unit_size, read_unit, &CDATA_LSQB_UNITS) {
        set_next_token_ptr(next_tok_ptr, ptr);
        return XML_TOK_INVALID;
    }
    set_next_token_ptr(
        next_tok_ptr,
        add_const_c_char(ptr, (CDATA_LSQB_UNITS.len() as isize) * unit_size),
    );
    XML_TOK_CDATA_SECT_OPEN
}

fn cdata_section_tok_normal(
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if ptr >= end {
        return XML_TOK_NONE;
    }

    match normal_byte_type(enc, ptr) {
        value if value == BT_RSQB as ::core::ffi::c_int => {
            let next = add_const_c_char(ptr, 1);
            if !has_c_chars(next, end, 1) {
                return XML_TOK_PARTIAL;
            }
            if c_char_is_ascii(next, b']') {
                let close = add_const_c_char(next, 1);
                if !has_c_chars(close, end, 1) {
                    return XML_TOK_PARTIAL;
                }
                if c_char_is_ascii(close, b'>') {
                    set_next_token_ptr(next_tok_ptr, add_const_c_char(close, 1));
                    return XML_TOK_CDATA_SECT_CLOSE;
                }
            }
            ptr = next;
        }
        value if value == BT_CR as ::core::ffi::c_int => {
            let next = add_const_c_char(ptr, 1);
            if !has_c_chars(next, end, 1) {
                return XML_TOK_PARTIAL;
            }
            ptr = if normal_byte_type(enc, next) == BT_LF as ::core::ffi::c_int {
                add_const_c_char(next, 1)
            } else {
                next
            };
            set_next_token_ptr(next_tok_ptr, ptr);
            return XML_TOK_DATA_NEWLINE;
        }
        value if value == BT_LF as ::core::ffi::c_int => {
            set_next_token_ptr(next_tok_ptr, add_const_c_char(ptr, 1));
            return XML_TOK_DATA_NEWLINE;
        }
        value if value == BT_LEAD2 as ::core::ffi::c_int => {
            if !has_c_chars(ptr, end, 2) {
                return XML_TOK_PARTIAL_CHAR;
            }
            if normal_is_invalid(enc, 2, ptr) != 0 {
                set_next_token_ptr(next_tok_ptr, ptr);
                return XML_TOK_INVALID;
            }
            ptr = add_const_c_char(ptr, 2);
        }
        value if value == BT_LEAD3 as ::core::ffi::c_int => {
            if !has_c_chars(ptr, end, 3) {
                return XML_TOK_PARTIAL_CHAR;
            }
            if normal_is_invalid(enc, 3, ptr) != 0 {
                set_next_token_ptr(next_tok_ptr, ptr);
                return XML_TOK_INVALID;
            }
            ptr = add_const_c_char(ptr, 3);
        }
        value if value == BT_LEAD4 as ::core::ffi::c_int => {
            if !has_c_chars(ptr, end, 4) {
                return XML_TOK_PARTIAL_CHAR;
            }
            if normal_is_invalid(enc, 4, ptr) != 0 {
                set_next_token_ptr(next_tok_ptr, ptr);
                return XML_TOK_INVALID;
            }
            ptr = add_const_c_char(ptr, 4);
        }
        value
            if matches!(
                value,
                x if x == BT_NONXML as ::core::ffi::c_int
                    || x == BT_MALFORM as ::core::ffi::c_int
                    || x == BT_TRAIL as ::core::ffi::c_int
            ) =>
        {
            set_next_token_ptr(next_tok_ptr, ptr);
            return XML_TOK_INVALID;
        }
        _ => {
            ptr = add_const_c_char(ptr, 1);
        }
    }

    while has_c_chars(ptr, end, 1) {
        match normal_byte_type(enc, ptr) {
            value if value == BT_LEAD2 as ::core::ffi::c_int => {
                if !has_c_chars(ptr, end, 2) || normal_is_invalid(enc, 2, ptr) != 0 {
                    set_next_token_ptr(next_tok_ptr, ptr);
                    return XML_TOK_DATA_CHARS;
                }
                ptr = add_const_c_char(ptr, 2);
            }
            value if value == BT_LEAD3 as ::core::ffi::c_int => {
                if !has_c_chars(ptr, end, 3) || normal_is_invalid(enc, 3, ptr) != 0 {
                    set_next_token_ptr(next_tok_ptr, ptr);
                    return XML_TOK_DATA_CHARS;
                }
                ptr = add_const_c_char(ptr, 3);
            }
            value if value == BT_LEAD4 as ::core::ffi::c_int => {
                if !has_c_chars(ptr, end, 4) || normal_is_invalid(enc, 4, ptr) != 0 {
                    set_next_token_ptr(next_tok_ptr, ptr);
                    return XML_TOK_DATA_CHARS;
                }
                ptr = add_const_c_char(ptr, 4);
            }
            value
                if matches!(
                    value,
                    x if x == BT_NONXML as ::core::ffi::c_int
                        || x == BT_MALFORM as ::core::ffi::c_int
                        || x == BT_TRAIL as ::core::ffi::c_int
                        || x == BT_CR as ::core::ffi::c_int
                        || x == BT_LF as ::core::ffi::c_int
                        || x == BT_RSQB as ::core::ffi::c_int
                ) =>
            {
                set_next_token_ptr(next_tok_ptr, ptr);
                return XML_TOK_DATA_CHARS;
            }
            _ => {
                ptr = add_const_c_char(ptr, 1);
            }
        }
    }

    set_next_token_ptr(next_tok_ptr, ptr);
    XML_TOK_DATA_CHARS
}

fn cdata_section_tok_wide(
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    byte_type: ByteTypeReader,
    is_ascii: AsciiMatcher,
) -> ::core::ffi::c_int {
    if ptr >= end {
        return XML_TOK_NONE;
    }

    let end = even_c_char_boundary(ptr, end);
    if ptr >= end {
        return XML_TOK_PARTIAL;
    }

    match byte_type(enc, ptr) {
        value if value == BT_RSQB as ::core::ffi::c_int => {
            let next = add_const_c_char(ptr, 2);
            if !has_c_chars(next, end, 2) {
                return XML_TOK_PARTIAL;
            }
            if is_ascii(next, b']') {
                let close = add_const_c_char(next, 2);
                if !has_c_chars(close, end, 2) {
                    return XML_TOK_PARTIAL;
                }
                if is_ascii(close, b'>') {
                    set_next_token_ptr(next_tok_ptr, add_const_c_char(close, 2));
                    return XML_TOK_CDATA_SECT_CLOSE;
                }
            }
            ptr = next;
        }
        value if value == BT_CR as ::core::ffi::c_int => {
            let next = add_const_c_char(ptr, 2);
            if !has_c_chars(next, end, 2) {
                return XML_TOK_PARTIAL;
            }
            ptr = if byte_type(enc, next) == BT_LF as ::core::ffi::c_int {
                add_const_c_char(next, 2)
            } else {
                next
            };
            set_next_token_ptr(next_tok_ptr, ptr);
            return XML_TOK_DATA_NEWLINE;
        }
        value if value == BT_LF as ::core::ffi::c_int => {
            set_next_token_ptr(next_tok_ptr, add_const_c_char(ptr, 2));
            return XML_TOK_DATA_NEWLINE;
        }
        value if value == BT_LEAD2 as ::core::ffi::c_int => {
            if !has_c_chars(ptr, end, 2) {
                return XML_TOK_PARTIAL_CHAR;
            }
            ptr = add_const_c_char(ptr, 2);
        }
        value if value == BT_LEAD3 as ::core::ffi::c_int => {
            if !has_c_chars(ptr, end, 3) {
                return XML_TOK_PARTIAL_CHAR;
            }
            ptr = add_const_c_char(ptr, 3);
        }
        value if value == BT_LEAD4 as ::core::ffi::c_int => {
            if !has_c_chars(ptr, end, 4) {
                return XML_TOK_PARTIAL_CHAR;
            }
            ptr = add_const_c_char(ptr, 4);
        }
        value
            if matches!(
                value,
                x if x == BT_NONXML as ::core::ffi::c_int
                    || x == BT_MALFORM as ::core::ffi::c_int
                    || x == BT_TRAIL as ::core::ffi::c_int
            ) =>
        {
            set_next_token_ptr(next_tok_ptr, ptr);
            return XML_TOK_INVALID;
        }
        _ => {
            ptr = add_const_c_char(ptr, 2);
        }
    }

    while has_c_chars(ptr, end, 2) {
        match byte_type(enc, ptr) {
            value if value == BT_LEAD2 as ::core::ffi::c_int => {
                ptr = add_const_c_char(ptr, 2);
            }
            value if value == BT_LEAD3 as ::core::ffi::c_int => {
                if !has_c_chars(ptr, end, 3) {
                    set_next_token_ptr(next_tok_ptr, ptr);
                    return XML_TOK_DATA_CHARS;
                }
                ptr = add_const_c_char(ptr, 3);
            }
            value if value == BT_LEAD4 as ::core::ffi::c_int => {
                if !has_c_chars(ptr, end, 4) {
                    set_next_token_ptr(next_tok_ptr, ptr);
                    return XML_TOK_DATA_CHARS;
                }
                ptr = add_const_c_char(ptr, 4);
            }
            value
                if matches!(
                    value,
                    x if x == BT_NONXML as ::core::ffi::c_int
                        || x == BT_MALFORM as ::core::ffi::c_int
                        || x == BT_TRAIL as ::core::ffi::c_int
                        || x == BT_CR as ::core::ffi::c_int
                        || x == BT_LF as ::core::ffi::c_int
                        || x == BT_RSQB as ::core::ffi::c_int
                ) =>
            {
                set_next_token_ptr(next_tok_ptr, ptr);
                return XML_TOK_DATA_CHARS;
            }
            _ => {
                ptr = add_const_c_char(ptr, 2);
            }
        }
    }

    set_next_token_ptr(next_tok_ptr, ptr);
    XML_TOK_DATA_CHARS
}

fn check_pi_target_with(
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    tok_ptr: *mut ::core::ffi::c_int,
    unit_size: isize,
    read_unit: AsciiUnitReader,
) -> ::core::ffi::c_int {
    let mut upper = 0 as ::core::ffi::c_int;
    write_copy(tok_ptr, XML_TOK_PI);

    if remaining_const_c_chars(ptr, end) != unit_size as usize * 3 {
        return 1 as ::core::ffi::c_int;
    }

    for &(lower, upper_variant) in &[(ASCII_x, ASCII_X), (ASCII_m, ASCII_M), (ASCII_l, ASCII_L)] {
        match read_unit(ptr) {
            value if value == lower => {}
            value if value == upper_variant => {
                upper = 1 as ::core::ffi::c_int;
            }
            _ => return 1 as ::core::ffi::c_int,
        }
        ptr = add_const_c_char(ptr, unit_size);
    }

    if upper != 0 {
        return 0 as ::core::ffi::c_int;
    }

    write_copy(tok_ptr, XML_TOK_XML_DECL);
    1 as ::core::ffi::c_int
}

fn matches_byte_type(token_type: ::core::ffi::c_int, expected: &[::core::ffi::c_int]) -> bool {
    expected.contains(&token_type)
}

fn scan_utf16_name_token_with(
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    byte_type: ByteTypeReader,
    is_name_start: fn(*const ::core::ffi::c_char) -> bool,
    is_name_char: fn(*const ::core::ffi::c_char) -> bool,
    initial_terminators: &[::core::ffi::c_int],
    loop_terminators: &[::core::ffi::c_int],
    initial_result: ::core::ffi::c_int,
    loop_result: ::core::ffi::c_int,
    loop_result_ptr_offset: isize,
    partial_result: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !has_c_chars(ptr, end, 2) {
        return XML_TOK_PARTIAL;
    }

    let token_type = byte_type(enc, ptr);
    ptr = match token_type {
        value if value == BT_NONASCII as ::core::ffi::c_int => {
            if !is_name_start(ptr) {
                set_next_token_ptr(next_tok_ptr, ptr);
                return XML_TOK_INVALID;
            }
            add_const_c_char(ptr, 2)
        }
        value
            if value == BT_NMSTRT as ::core::ffi::c_int
                || value == BT_HEX as ::core::ffi::c_int =>
        {
            add_const_c_char(ptr, 2)
        }
        value
            if value == BT_LEAD2 as ::core::ffi::c_int
                || value == BT_LEAD3 as ::core::ffi::c_int
                || value == BT_LEAD4 as ::core::ffi::c_int =>
        {
            let width = multibyte_sequence_len(token_type);
            if !has_c_chars(ptr, end, width) {
                return XML_TOK_PARTIAL_CHAR;
            }
            set_next_token_ptr(next_tok_ptr, ptr);
            return XML_TOK_INVALID;
        }
        _ if matches_byte_type(token_type, initial_terminators) => {
            set_next_token_ptr(next_tok_ptr, ptr);
            return initial_result;
        }
        _ => {
            set_next_token_ptr(next_tok_ptr, ptr);
            return XML_TOK_INVALID;
        }
    };

    while has_c_chars(ptr, end, 2) {
        let token_type = byte_type(enc, ptr);
        ptr = match token_type {
            value if value == BT_NONASCII as ::core::ffi::c_int => {
                if !is_name_char(ptr) {
                    set_next_token_ptr(next_tok_ptr, ptr);
                    return XML_TOK_INVALID;
                }
                add_const_c_char(ptr, 2)
            }
            value
                if value == BT_NMSTRT as ::core::ffi::c_int
                    || value == BT_HEX as ::core::ffi::c_int
                    || value == BT_DIGIT as ::core::ffi::c_int
                    || value == BT_NAME as ::core::ffi::c_int
                    || value == BT_MINUS as ::core::ffi::c_int =>
            {
                add_const_c_char(ptr, 2)
            }
            value
                if value == BT_LEAD2 as ::core::ffi::c_int
                    || value == BT_LEAD3 as ::core::ffi::c_int
                    || value == BT_LEAD4 as ::core::ffi::c_int =>
            {
                let width = multibyte_sequence_len(token_type);
                if !has_c_chars(ptr, end, width) {
                    return XML_TOK_PARTIAL_CHAR;
                }
                set_next_token_ptr(next_tok_ptr, ptr);
                return XML_TOK_INVALID;
            }
            _ if matches_byte_type(token_type, loop_terminators) => {
                set_next_token_ptr(next_tok_ptr, add_const_c_char(ptr, loop_result_ptr_offset));
                return loop_result;
            }
            _ => {
                set_next_token_ptr(next_tok_ptr, ptr);
                return XML_TOK_INVALID;
            }
        };
    }

    partial_result
}

fn scan_utf16_pi_body_with(
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    tok: ::core::ffi::c_int,
    byte_type: ByteTypeReader,
    is_ascii: AsciiMatcher,
) -> ::core::ffi::c_int {
    while has_c_chars(ptr, end, 2) {
        match byte_type(enc, ptr) {
            value
                if value == BT_LEAD2 as ::core::ffi::c_int
                    || value == BT_LEAD3 as ::core::ffi::c_int
                    || value == BT_LEAD4 as ::core::ffi::c_int =>
            {
                let width = multibyte_sequence_len(value);
                if !has_c_chars(ptr, end, width) {
                    return XML_TOK_PARTIAL_CHAR;
                }
                ptr = add_const_c_char(ptr, width as isize);
            }
            value
                if value == BT_NONXML as ::core::ffi::c_int
                    || value == BT_MALFORM as ::core::ffi::c_int
                    || value == BT_TRAIL as ::core::ffi::c_int =>
            {
                set_next_token_ptr(next_tok_ptr, ptr);
                return XML_TOK_INVALID;
            }
            value if value == BT_QUEST as ::core::ffi::c_int => {
                ptr = add_const_c_char(ptr, 2);
                if !has_c_chars(ptr, end, 2) {
                    return XML_TOK_PARTIAL;
                }
                if is_ascii(ptr, b'>') {
                    set_next_token_ptr(next_tok_ptr, add_const_c_char(ptr, 2));
                    return tok;
                }
            }
            _ => {
                ptr = add_const_c_char(ptr, 2);
            }
        }
    }

    XML_TOK_PARTIAL
}

fn scan_utf16_pi_with(
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    byte_type: ByteTypeReader,
    is_name_start: fn(*const ::core::ffi::c_char) -> bool,
    is_name_char: fn(*const ::core::ffi::c_char) -> bool,
    check_pi_target: PiTargetChecker,
    is_ascii: AsciiMatcher,
) -> ::core::ffi::c_int {
    let mut tok = 0;
    let target = ptr;

    if !has_c_chars(ptr, end, 2) {
        return XML_TOK_PARTIAL;
    }

    let token_type = byte_type(enc, ptr);
    ptr = match token_type {
        value if value == BT_NONASCII as ::core::ffi::c_int => {
            if !is_name_start(ptr) {
                set_next_token_ptr(next_tok_ptr, ptr);
                return XML_TOK_INVALID;
            }
            add_const_c_char(ptr, 2)
        }
        value
            if value == BT_NMSTRT as ::core::ffi::c_int
                || value == BT_HEX as ::core::ffi::c_int =>
        {
            add_const_c_char(ptr, 2)
        }
        value
            if value == BT_LEAD2 as ::core::ffi::c_int
                || value == BT_LEAD3 as ::core::ffi::c_int
                || value == BT_LEAD4 as ::core::ffi::c_int =>
        {
            let width = multibyte_sequence_len(token_type);
            if !has_c_chars(ptr, end, width) {
                return XML_TOK_PARTIAL_CHAR;
            }
            set_next_token_ptr(next_tok_ptr, ptr);
            return XML_TOK_INVALID;
        }
        _ => {
            set_next_token_ptr(next_tok_ptr, ptr);
            return XML_TOK_INVALID;
        }
    };

    while has_c_chars(ptr, end, 2) {
        match byte_type(enc, ptr) {
            value if value == BT_NONASCII as ::core::ffi::c_int => {
                if !is_name_char(ptr) {
                    set_next_token_ptr(next_tok_ptr, ptr);
                    return XML_TOK_INVALID;
                }
                ptr = add_const_c_char(ptr, 2);
            }
            value
                if value == BT_NMSTRT as ::core::ffi::c_int
                    || value == BT_HEX as ::core::ffi::c_int
                    || value == BT_DIGIT as ::core::ffi::c_int
                    || value == BT_NAME as ::core::ffi::c_int
                    || value == BT_MINUS as ::core::ffi::c_int =>
            {
                ptr = add_const_c_char(ptr, 2);
            }
            value
                if value == BT_LEAD2 as ::core::ffi::c_int
                    || value == BT_LEAD3 as ::core::ffi::c_int
                    || value == BT_LEAD4 as ::core::ffi::c_int =>
            {
                let width = multibyte_sequence_len(value);
                if !has_c_chars(ptr, end, width) {
                    return XML_TOK_PARTIAL_CHAR;
                }
                set_next_token_ptr(next_tok_ptr, ptr);
                return XML_TOK_INVALID;
            }
            value
                if value == BT_S as ::core::ffi::c_int
                    || value == BT_CR as ::core::ffi::c_int
                    || value == BT_LF as ::core::ffi::c_int =>
            {
                if check_pi_target(enc, target, ptr, &raw mut tok) == 0 {
                    set_next_token_ptr(next_tok_ptr, ptr);
                    return XML_TOK_INVALID;
                }
                ptr = add_const_c_char(ptr, 2);
                return scan_utf16_pi_body_with(
                    enc,
                    ptr,
                    end,
                    next_tok_ptr,
                    tok,
                    byte_type,
                    is_ascii,
                );
            }
            value if value == BT_QUEST as ::core::ffi::c_int => {
                if check_pi_target(enc, target, ptr, &raw mut tok) == 0 {
                    set_next_token_ptr(next_tok_ptr, ptr);
                    return XML_TOK_INVALID;
                }
                ptr = add_const_c_char(ptr, 2);
                if !has_c_chars(ptr, end, 2) {
                    return XML_TOK_PARTIAL;
                }
                if is_ascii(ptr, b'>') {
                    set_next_token_ptr(next_tok_ptr, add_const_c_char(ptr, 2));
                    return tok;
                }
                set_next_token_ptr(next_tok_ptr, ptr);
                return XML_TOK_INVALID;
            }
            _ => {
                set_next_token_ptr(next_tok_ptr, ptr);
                return XML_TOK_INVALID;
            }
        }
    }

    XML_TOK_PARTIAL
}

fn predefined_entity_name_with(
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    unit_size: isize,
    read_unit: AsciiUnitReader,
) -> ::core::ffi::c_int {
    match remaining_const_c_chars(ptr, end) / unit_size as usize {
        2 => {
            if matches_ascii_units(ptr, unit_size, read_unit, &[ASCII_l, ASCII_t]) {
                ASCII_LT
            } else if matches_ascii_units(ptr, unit_size, read_unit, &[ASCII_g, ASCII_t]) {
                ASCII_GT
            } else {
                0 as ::core::ffi::c_int
            }
        }
        3 => {
            if matches_ascii_units(
                ptr,
                unit_size,
                read_unit,
                &[ASCII_a, ASCII_m, 0x70 as ::core::ffi::c_int],
            ) {
                ASCII_AMP
            } else {
                0 as ::core::ffi::c_int
            }
        }
        4 => {
            if matches_ascii_units(
                ptr,
                unit_size,
                read_unit,
                &[ASCII_q, 0x75 as ::core::ffi::c_int, ASCII_o, ASCII_t],
            ) {
                ASCII_QUOT
            } else if matches_ascii_units(
                ptr,
                unit_size,
                read_unit,
                &[ASCII_a, 0x70 as ::core::ffi::c_int, ASCII_o, ASCII_s],
            ) {
                ASCII_APOS
            } else {
                0 as ::core::ffi::c_int
            }
        }
        _ => 0 as ::core::ffi::c_int,
    }
}

fn char_ref_number_with(
    mut ptr: *const ::core::ffi::c_char,
    unit_size: isize,
    read_unit: AsciiUnitReader,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    ptr = add_const_c_char(ptr, 2 * unit_size);

    if read_unit(ptr) == 0x78 as ::core::ffi::c_int {
        ptr = add_const_c_char(ptr, unit_size);
        while read_unit(ptr) != 0x3b as ::core::ffi::c_int {
            let c = read_unit(ptr);
            match c {
                ASCII_0 | ASCII_1 | ASCII_2 | ASCII_3 | ASCII_4 | ASCII_5 | ASCII_6 | ASCII_7
                | ASCII_8 | ASCII_9 => {
                    result <<= 4 as ::core::ffi::c_int;
                    result |= c - ASCII_0;
                }
                ASCII_A | ASCII_B | ASCII_C | ASCII_D | ASCII_E | ASCII_F => {
                    result <<= 4 as ::core::ffi::c_int;
                    result += 10 as ::core::ffi::c_int + (c - ASCII_A);
                }
                ASCII_a | ASCII_b | ASCII_c | ASCII_d | ASCII_e | ASCII_f => {
                    result <<= 4 as ::core::ffi::c_int;
                    result += 10 as ::core::ffi::c_int + (c - ASCII_a);
                }
                _ => {}
            }
            if result >= 0x110000 as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int);
            }
            ptr = add_const_c_char(ptr, unit_size);
        }
    } else {
        while read_unit(ptr) != 0x3b as ::core::ffi::c_int {
            let c = read_unit(ptr);
            result *= 10 as ::core::ffi::c_int;
            result += c - ASCII_0;
            if result >= 0x110000 as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int);
            }
            ptr = add_const_c_char(ptr, unit_size);
        }
    }

    checkCharRefNumber(result)
}

fn write_c_char_and_advance(ptr: &mut *mut ::core::ffi::c_char, value: ::core::ffi::c_char) {
    write_copy(*ptr, value);
    *ptr = add_mut_c_char(*ptr, 1);
}

fn write_c_ushort_and_advance(ptr: &mut *mut ::core::ffi::c_ushort, value: ::core::ffi::c_ushort) {
    write_copy(*ptr, value);
    *ptr = add_mut_c_ushort(*ptr, 1);
}

fn trim_to_complete_utf8_end(
    from: *const ::core::ffi::c_char,
    mut from_lim: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut walked = 0 as size_t;

    while from_lim > from {
        let prev = read_c_char(add_const_c_char(from_lim, -1)) as ::core::ffi::c_uchar;
        if prev as ::core::ffi::c_uint & 0xf8 as ::core::ffi::c_uint == 0xf0 as ::core::ffi::c_uint
        {
            if walked.wrapping_add(1) >= 4 {
                from_lim = add_const_c_char(from_lim, 3);
                break;
            }
            walked = 0;
        } else if prev as ::core::ffi::c_uint & 0xf0 as ::core::ffi::c_uint
            == 0xe0 as ::core::ffi::c_uint
        {
            if walked.wrapping_add(1) >= 3 {
                from_lim = add_const_c_char(from_lim, 2);
                break;
            }
            walked = 0;
        } else if prev as ::core::ffi::c_uint & 0xe0 as ::core::ffi::c_uint
            == 0xc0 as ::core::ffi::c_uint
        {
            if walked.wrapping_add(1) >= 2 {
                from_lim = add_const_c_char(from_lim, 1);
                break;
            }
            walked = 0;
        } else if prev as ::core::ffi::c_uint & 0x80 as ::core::ffi::c_uint == 0 {
            break;
        }

        from_lim = add_const_c_char(from_lim, -1);
        walked = walked.wrapping_add(1);
    }

    from_lim
}

fn even_c_char_boundary(
    start: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let even_len = remaining_const_c_chars(start, end) & !1;
    add_const_c_char(start, even_len as isize)
}

fn read_c_char(ptr: *const ::core::ffi::c_char) -> ::core::ffi::c_char {
    read_c_char_bytes::<1>(ptr)[0] as ::core::ffi::c_char
}

fn c_char_is_ascii(ptr: *const ::core::ffi::c_char, ascii: u8) -> bool {
    read_c_char(ptr) as ::core::ffi::c_uchar == ascii
}

fn little2_is_ascii(ptr: *const ::core::ffi::c_char, ascii: u8) -> bool {
    read_c_char(add_const_c_char(ptr, 1)) as ::core::ffi::c_int == 0 && c_char_is_ascii(ptr, ascii)
}

fn big2_is_ascii(ptr: *const ::core::ffi::c_char, ascii: u8) -> bool {
    read_c_char(ptr) as ::core::ffi::c_int == 0 && c_char_is_ascii(add_const_c_char(ptr, 1), ascii)
}

fn remaining_c_chars(ptr: *mut ::core::ffi::c_char, end: *const ::core::ffi::c_char) -> usize {
    (end as usize).wrapping_sub(ptr as usize)
}

fn remaining_c_ushorts(
    ptr: *mut ::core::ffi::c_ushort,
    end: *const ::core::ffi::c_ushort,
) -> usize {
    ((end as usize).wrapping_sub(ptr as usize)) / ::core::mem::size_of::<::core::ffi::c_ushort>()
}

fn remaining_const_c_chars(
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
) -> usize {
    (end as usize).wrapping_sub(ptr as usize)
}

fn c_char_distance(
    start: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    ::core::ffi::c_int::try_from((end as usize).wrapping_sub(start as usize))
        .expect("pointer distance should fit in c_int")
}

fn normal_byte_type(enc: *const ENCODING, ptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    with_ref(enc.cast::<normal_encoding>(), |normal| {
        normal.type_0[read_c_char(ptr) as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
    })
}

fn little2_byte_type(enc: *const ENCODING, ptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let high = read_c_char(add_const_c_char(ptr, 1));
    if high as ::core::ffi::c_int == 0 {
        normal_byte_type(enc, ptr)
    } else {
        unicode_byte_type(high, read_c_char(ptr))
    }
}

fn big2_byte_type(enc: *const ENCODING, ptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let high = read_c_char(ptr);
    let low = read_c_char(add_const_c_char(ptr, 1));
    if high as ::core::ffi::c_int == 0 {
        with_ref(enc.cast::<normal_encoding>(), |normal| {
            normal.type_0[low as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
        })
    } else {
        unicode_byte_type(high, low)
    }
}

fn line_number(pos: *mut POSITION) -> XML_Size {
    with_ref(pos.cast_const(), |position| position.lineNumber)
}

fn set_line_number(pos: *mut POSITION, value: XML_Size) {
    with_mut(pos, |position| position.lineNumber = value)
}

fn column_number(pos: *mut POSITION) -> XML_Size {
    with_ref(pos.cast_const(), |position| position.columnNumber)
}

fn set_column_number(pos: *mut POSITION, value: XML_Size) {
    with_mut(pos, |position| position.columnNumber = value)
}

fn increment_line_number(pos: *mut POSITION) {
    set_line_number(pos, line_number(pos).wrapping_add(1));
}

fn increment_column_number(pos: *mut POSITION) {
    set_column_number(pos, column_number(pos).wrapping_add(1));
}

fn encoding_min_bytes_per_char(enc: *const ENCODING) -> isize {
    with_ref(enc, |encoding| encoding.minBytesPerChar as isize)
}

fn encoding_name_matches_ascii(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    ascii_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let name_matches_ascii =
        with_ref(enc, |encoding| encoding.nameMatchesAscii).expect("non-null function pointer");
    name_matches_ascii(enc, ptr, end, ascii_name)
}

fn encoding_utf8_convert(
    enc: *const ENCODING,
    from_p: *mut *const ::core::ffi::c_char,
    from_lim: *const ::core::ffi::c_char,
    to_p: *mut *mut ::core::ffi::c_char,
    to_lim: *const ::core::ffi::c_char,
) -> XML_Convert_Result {
    let utf8_convert =
        with_ref(enc, |encoding| encoding.utf8Convert).expect("non-null function pointer");
    utf8_convert(enc, from_p, from_lim, to_p, to_lim)
}

fn encoding_table_ptr() -> *const *const ENCODING {
    encodings.0.as_ptr()
}

fn encoding_table_ns_ptr() -> *const *const ENCODING {
    encodingsNS.0.as_ptr()
}

fn update_position_with_utf8(
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    pos: *mut POSITION,
) {
    normal_updatePosition(
        read_encoding_table_entry(encoding_table_ptr(), UTF_8_ENC as usize),
        ptr,
        end,
        pos,
    );
}

fn call_unknown_converter(
    uenc: &unknown_encoding,
    ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe { uenc.convert.expect("non-null function pointer")(uenc.userData, ptr) }
}

fn encode_utf8_bytes(
    c: ::core::ffi::c_int,
    buf: &mut [::core::ffi::c_char; 4],
) -> ::core::ffi::c_int {
    if c < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if c < min2 as ::core::ffi::c_int {
        buf[0] = (c | UTF8_cval1 as ::core::ffi::c_int) as ::core::ffi::c_char;
        return 1 as ::core::ffi::c_int;
    }
    if c < min3 as ::core::ffi::c_int {
        buf[0] = (c >> 6 as ::core::ffi::c_int | UTF8_cval2 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        buf[1] =
            (c & 0x3f as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
        return 2 as ::core::ffi::c_int;
    }
    if c < min4 as ::core::ffi::c_int {
        buf[0] = (c >> 12 as ::core::ffi::c_int | UTF8_cval3 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        buf[1] = (c >> 6 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int
            | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
        buf[2] =
            (c & 0x3f as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
        return 3 as ::core::ffi::c_int;
    }
    if c < 0x110000 as ::core::ffi::c_int {
        buf[0] = (c >> 18 as ::core::ffi::c_int | UTF8_cval4 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        buf[1] = (c >> 12 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int
            | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
        buf[2] = (c >> 6 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int
            | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
        buf[3] =
            (c & 0x3f as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
        return 4 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}

fn call_xml_utf8_encode(
    c: ::core::ffi::c_int,
    buf: &mut [::core::ffi::c_char; 4],
) -> ::core::ffi::c_int {
    encode_utf8_bytes(c, buf)
}

fn call_encoding_finder(
    encoding_finder: extern "C" fn(
        *const ENCODING,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
    ) -> *const ENCODING,
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
) -> *const ENCODING {
    encoding_finder(enc, ptr, end)
}

fn copy_c_chars(dest: *mut ::core::ffi::c_char, src: *const ::core::ffi::c_char, len: size_t) {
    for index in 0..len {
        write_copy(dest.wrapping_add(index), read_copy(src.wrapping_add(index)));
    }
}

fn unknown_sequence_length(enc: *const ENCODING, ptr: *const ::core::ffi::c_char) -> isize {
    let byte = read_c_char(ptr) as ::core::ffi::c_uchar as usize;
    with_ref(enc.cast::<normal_encoding>(), |normal| {
        (normal.type_0[byte] as ::core::ffi::c_int - (BT_LEAD2 as ::core::ffi::c_int - 2)) as isize
    })
}

fn read_encoding_table_entry(table: *const *const ENCODING, index: usize) -> *const ENCODING {
    read_copy(table.wrapping_add(index))
}

fn init_encoding_kind(enc: *const INIT_ENCODING) -> ::core::ffi::c_int {
    with_ref(enc, |init| init.initEnc.isUtf16 as ::core::ffi::c_int)
}

fn init_encoding_ptr_slot(enc: *const INIT_ENCODING) -> *mut *const ENCODING {
    with_ref(enc, |init| init.encPtr)
}

fn set_encoding_ptr(enc_ptr: *mut *const ENCODING, value: *const ENCODING) {
    write_copy(enc_ptr, value)
}

fn set_next_token_ptr(
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    value: *const ::core::ffi::c_char,
) {
    write_copy(next_tok_ptr, value)
}

fn scan_hex_char_ref_with(
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    unit_size: usize,
    byte_type: ByteTypeReader,
) -> ::core::ffi::c_int {
    if remaining_const_c_chars(ptr, end) >= unit_size {
        match byte_type(enc, ptr) {
            25 | 24 => {}
            _ => {
                set_next_token_ptr(next_tok_ptr, ptr);
                return XML_TOK_INVALID;
            }
        }
        ptr = add_const_c_char(ptr, unit_size as isize);
        while remaining_const_c_chars(ptr, end) >= unit_size {
            match byte_type(enc, ptr) {
                25 | 24 => {}
                18 => {
                    set_next_token_ptr(next_tok_ptr, add_const_c_char(ptr, unit_size as isize));
                    return XML_TOK_CHAR_REF;
                }
                _ => {
                    set_next_token_ptr(next_tok_ptr, ptr);
                    return XML_TOK_INVALID;
                }
            }
            ptr = add_const_c_char(ptr, unit_size as isize);
        }
    }
    XML_TOK_PARTIAL
}

fn scan_char_ref_with(
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    unit_size: usize,
    read_unit: AsciiUnitReader,
    byte_type: ByteTypeReader,
) -> ::core::ffi::c_int {
    if remaining_const_c_chars(ptr, end) < unit_size {
        return XML_TOK_PARTIAL;
    }
    if read_unit(ptr) == ASCII_x {
        return scan_hex_char_ref_with(
            enc,
            add_const_c_char(ptr, unit_size as isize),
            end,
            next_tok_ptr,
            unit_size,
            byte_type,
        );
    }
    match byte_type(enc, ptr) {
        25 => {}
        _ => {
            set_next_token_ptr(next_tok_ptr, ptr);
            return XML_TOK_INVALID;
        }
    }
    ptr = add_const_c_char(ptr, unit_size as isize);
    while remaining_const_c_chars(ptr, end) >= unit_size {
        match byte_type(enc, ptr) {
            25 => {}
            18 => {
                set_next_token_ptr(next_tok_ptr, add_const_c_char(ptr, unit_size as isize));
                return XML_TOK_CHAR_REF;
            }
            _ => {
                set_next_token_ptr(next_tok_ptr, ptr);
                return XML_TOK_INVALID;
            }
        }
        ptr = add_const_c_char(ptr, unit_size as isize);
    }
    XML_TOK_PARTIAL
}

fn scan_ref_with(
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    unit_size: usize,
    byte_type: ByteTypeReader,
    nonascii_name_start: fn(*const ::core::ffi::c_char) -> bool,
    nonascii_name_char: fn(*const ::core::ffi::c_char) -> bool,
    multibyte_name_start: fn(*const ENCODING, usize, *const ::core::ffi::c_char) -> bool,
    multibyte_name_char: fn(*const ENCODING, usize, *const ::core::ffi::c_char) -> bool,
    scan_char_ref: extern "C" fn(
        *const ENCODING,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let unit_offset = unit_size as isize;

    if !has_c_chars(ptr, end, unit_size) {
        return XML_TOK_PARTIAL;
    }

    match byte_type(enc, ptr) {
        x if x == BT_NONASCII as ::core::ffi::c_int => {
            if !nonascii_name_start(ptr) {
                set_const_c_char_ptr(next_tok_ptr, ptr);
                return XML_TOK_INVALID;
            }
            ptr = add_const_c_char(ptr, unit_offset);
        }
        x if x == BT_NMSTRT as ::core::ffi::c_int || x == BT_HEX as ::core::ffi::c_int => {
            ptr = add_const_c_char(ptr, unit_offset);
        }
        x if x == BT_LEAD2 as ::core::ffi::c_int
            || x == BT_LEAD3 as ::core::ffi::c_int
            || x == BT_LEAD4 as ::core::ffi::c_int =>
        {
            let width = (x - BT_LEAD2 as ::core::ffi::c_int + 2) as usize;
            if !has_c_chars(ptr, end, width) {
                return XML_TOK_PARTIAL_CHAR;
            }
            if !multibyte_name_start(enc, width, ptr) {
                set_const_c_char_ptr(next_tok_ptr, ptr);
                return XML_TOK_INVALID;
            }
            ptr = add_const_c_char(ptr, width as isize);
        }
        x if x == BT_NUM as ::core::ffi::c_int => {
            return scan_char_ref(enc, add_const_c_char(ptr, unit_offset), end, next_tok_ptr);
        }
        _ => {
            set_const_c_char_ptr(next_tok_ptr, ptr);
            return XML_TOK_INVALID;
        }
    }

    while has_c_chars(ptr, end, unit_size) {
        match byte_type(enc, ptr) {
            x if x == BT_NONASCII as ::core::ffi::c_int => {
                if !nonascii_name_char(ptr) {
                    set_const_c_char_ptr(next_tok_ptr, ptr);
                    return XML_TOK_INVALID;
                }
                ptr = add_const_c_char(ptr, unit_offset);
            }
            x if x == BT_NMSTRT as ::core::ffi::c_int
                || x == BT_HEX as ::core::ffi::c_int
                || x == BT_DIGIT as ::core::ffi::c_int
                || x == BT_NAME as ::core::ffi::c_int
                || x == BT_MINUS as ::core::ffi::c_int =>
            {
                ptr = add_const_c_char(ptr, unit_offset);
            }
            x if x == BT_LEAD2 as ::core::ffi::c_int
                || x == BT_LEAD3 as ::core::ffi::c_int
                || x == BT_LEAD4 as ::core::ffi::c_int =>
            {
                let width = (x - BT_LEAD2 as ::core::ffi::c_int + 2) as usize;
                if !has_c_chars(ptr, end, width) {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if !multibyte_name_char(enc, width, ptr) {
                    set_const_c_char_ptr(next_tok_ptr, ptr);
                    return XML_TOK_INVALID;
                }
                ptr = add_const_c_char(ptr, width as isize);
            }
            x if x == BT_SEMI as ::core::ffi::c_int => {
                set_const_c_char_ptr(next_tok_ptr, add_const_c_char(ptr, unit_offset));
                return XML_TOK_ENTITY_REF;
            }
            _ => {
                set_const_c_char_ptr(next_tok_ptr, ptr);
                return XML_TOK_INVALID;
            }
        }
    }

    XML_TOK_PARTIAL
}

fn call_scanner(
    scanner: SCANNER,
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    scanner.expect("non-null function pointer")(enc, ptr, end, next_tok_ptr)
}

fn dispatch_scanner(
    enc_ptr: *mut *const ENCODING,
    state: ::core::ffi::c_int,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let enc = read_copy(enc_ptr);
    let scanner = with_ref(enc, |encoding| encoding.scanners[state as usize]);
    call_scanner(scanner, enc, ptr, end, next_tok_ptr)
}

extern "C" fn isNever(_enc: *const ENCODING, _p: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    0
}
extern "C" fn utf8_isName2(
    _enc: *const ENCODING,
    p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let [first, second] = read_c_char_bytes::<2>(p);
    let page = namePages[((first >> 2) & 7) as usize] as usize;
    let bitmap_index = (page << 3) + (((first as usize & 3) << 1) + (((second >> 5) & 1) as usize));
    ((namingBitmap[bitmap_index] & (1_u32 << (second & 0x1f))) != 0) as ::core::ffi::c_int
}
extern "C" fn utf8_isName3(
    _enc: *const ENCODING,
    p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let [first, second, third] = read_c_char_bytes::<3>(p);
    let page =
        namePages[((((first & 0xf) as usize) << 4) + (((second >> 2) & 0xf) as usize))] as usize;
    let bitmap_index =
        (page << 3) + ((((second & 3) as usize) << 1) + (((third >> 5) & 1) as usize));
    ((namingBitmap[bitmap_index] & (1_u32 << (third & 0x1f))) != 0) as ::core::ffi::c_int
}
extern "C" fn utf8_isNmstrt2(
    _enc: *const ENCODING,
    p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let [first, second] = read_c_char_bytes::<2>(p);
    let page = nmstrtPages[((first >> 2) & 7) as usize] as usize;
    let bitmap_index = (page << 3) + (((first as usize & 3) << 1) + (((second >> 5) & 1) as usize));
    ((namingBitmap[bitmap_index] & (1_u32 << (second & 0x1f))) != 0) as ::core::ffi::c_int
}
extern "C" fn utf8_isNmstrt3(
    _enc: *const ENCODING,
    p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let [first, second, third] = read_c_char_bytes::<3>(p);
    let page =
        nmstrtPages[((((first & 0xf) as usize) << 4) + (((second >> 2) & 0xf) as usize))] as usize;
    let bitmap_index =
        (page << 3) + ((((second & 3) as usize) << 1) + (((third >> 5) & 1) as usize));
    ((namingBitmap[bitmap_index] & (1_u32 << (third & 0x1f))) != 0) as ::core::ffi::c_int
}
extern "C" fn utf8_isInvalid2(
    _enc: *const ENCODING,
    p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let [first, second] = read_c_char_bytes::<2>(p);
    (first < 0xc2 || second & 0x80 == 0 || second & 0xc0 == 0xc0) as ::core::ffi::c_int
}
extern "C" fn utf8_isInvalid3(
    _enc: *const ENCODING,
    p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let [first, second, third] = read_c_char_bytes::<3>(p);
    let invalid_third = third & 0x80 == 0
        || if first == 0xef && second == 0xbf {
            third > 0xbd
        } else {
            third & 0xc0 == 0xc0
        };
    let invalid_second = if first == 0xe0 {
        second < 0xa0 || second & 0xc0 == 0xc0
    } else {
        second & 0x80 == 0
            || if first == 0xed {
                second > 0x9f
            } else {
                second & 0xc0 == 0xc0
            }
    };
    (invalid_third || invalid_second) as ::core::ffi::c_int
}

extern "C" fn utf8_isInvalid4(
    _enc: *const ENCODING,
    p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let [first, second, third, fourth] = read_c_char_bytes::<4>(p);
    let invalid_fourth = fourth & 0x80 == 0 || fourth & 0xc0 == 0xc0;
    let invalid_third = third & 0x80 == 0 || third & 0xc0 == 0xc0;
    let invalid_second = if first == 0xf0 {
        second < 0x90 || second & 0xc0 == 0xc0
    } else {
        second & 0x80 == 0
            || if first == 0xf4 {
                second > 0x8f
            } else {
                second & 0xc0 == 0xc0
            }
    };
    (invalid_fourth || invalid_third || invalid_second) as ::core::ffi::c_int
}
extern "C" fn normal_scanComment(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if has_c_chars(ptr, end, 1) {
        if !c_char_is_ascii(ptr, b'-') {
            set_const_c_char_ptr(nextTokPtr, ptr);
            return XML_TOK_INVALID;
        }
        ptr = add_const_c_char(ptr, 1);
        while has_c_chars(ptr, end, 1) {
            match normal_byte_type(enc, ptr) {
                value if value == BT_LEAD2 as ::core::ffi::c_int => {
                    if !has_c_chars(ptr, end, 2) {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if normal_is_invalid(enc, 2, ptr) != 0 {
                        set_const_c_char_ptr(nextTokPtr, ptr);
                        return XML_TOK_INVALID;
                    }
                    ptr = add_const_c_char(ptr, 2);
                }
                value if value == BT_LEAD3 as ::core::ffi::c_int => {
                    if !has_c_chars(ptr, end, 3) {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if normal_is_invalid(enc, 3, ptr) != 0 {
                        set_const_c_char_ptr(nextTokPtr, ptr);
                        return XML_TOK_INVALID;
                    }
                    ptr = add_const_c_char(ptr, 3);
                }
                value if value == BT_LEAD4 as ::core::ffi::c_int => {
                    if !has_c_chars(ptr, end, 4) {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if normal_is_invalid(enc, 4, ptr) != 0 {
                        set_const_c_char_ptr(nextTokPtr, ptr);
                        return XML_TOK_INVALID;
                    }
                    ptr = add_const_c_char(ptr, 4);
                }
                value
                    if value == BT_NONXML as ::core::ffi::c_int
                        || value == BT_MALFORM as ::core::ffi::c_int
                        || value == BT_TRAIL as ::core::ffi::c_int =>
                {
                    set_const_c_char_ptr(nextTokPtr, ptr);
                    return XML_TOK_INVALID;
                }
                value if value == BT_MINUS as ::core::ffi::c_int => {
                    ptr = add_const_c_char(ptr, 1);
                    if !has_c_chars(ptr, end, 1) {
                        return XML_TOK_PARTIAL;
                    }
                    if c_char_is_ascii(ptr, b'-') {
                        ptr = add_const_c_char(ptr, 1);
                        if !has_c_chars(ptr, end, 1) {
                            return XML_TOK_PARTIAL;
                        }
                        if !c_char_is_ascii(ptr, b'>') {
                            set_const_c_char_ptr(nextTokPtr, ptr);
                            return XML_TOK_INVALID;
                        }
                        set_const_c_char_ptr(nextTokPtr, add_const_c_char(ptr, 1));
                        return XML_TOK_COMMENT;
                    }
                }
                _ => {
                    ptr = add_const_c_char(ptr, 1);
                }
            }
        }
    }
    XML_TOK_PARTIAL
}
fn scan_decl_with(
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    unit_size: usize,
    byte_type: ByteTypeReader,
    scan_comment: extern "C" fn(
        *const ENCODING,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let unit_offset = unit_size as isize;

    if !has_c_chars(ptr, end, unit_size) {
        return XML_TOK_PARTIAL;
    }

    match byte_type(enc, ptr) {
        x if x == BT_MINUS as ::core::ffi::c_int => {
            return scan_comment(enc, add_const_c_char(ptr, unit_offset), end, next_tok_ptr);
        }
        x if x == BT_LSQB as ::core::ffi::c_int => {
            set_const_c_char_ptr(next_tok_ptr, add_const_c_char(ptr, unit_offset));
            return XML_TOK_COND_SECT_OPEN;
        }
        x if x == BT_NMSTRT as ::core::ffi::c_int || x == BT_HEX as ::core::ffi::c_int => {
            ptr = add_const_c_char(ptr, unit_offset);
        }
        _ => {
            set_const_c_char_ptr(next_tok_ptr, ptr);
            return XML_TOK_INVALID;
        }
    }

    while has_c_chars(ptr, end, unit_size) {
        match byte_type(enc, ptr) {
            x if x == BT_PERCNT as ::core::ffi::c_int => {
                if !has_c_chars(ptr, end, unit_size * 2) {
                    return XML_TOK_PARTIAL;
                }

                match byte_type(enc, add_const_c_char(ptr, unit_offset)) {
                    y if y == BT_S as ::core::ffi::c_int
                        || y == BT_CR as ::core::ffi::c_int
                        || y == BT_LF as ::core::ffi::c_int
                        || y == BT_PERCNT as ::core::ffi::c_int =>
                    {
                        set_const_c_char_ptr(next_tok_ptr, ptr);
                        return XML_TOK_INVALID;
                    }
                    _ => {}
                }
            }
            x if x == BT_S as ::core::ffi::c_int
                || x == BT_CR as ::core::ffi::c_int
                || x == BT_LF as ::core::ffi::c_int => {}
            x if x == BT_NMSTRT as ::core::ffi::c_int || x == BT_HEX as ::core::ffi::c_int => {
                ptr = add_const_c_char(ptr, unit_offset);
                continue;
            }
            _ => {
                set_const_c_char_ptr(next_tok_ptr, ptr);
                return XML_TOK_INVALID;
            }
        }

        set_const_c_char_ptr(next_tok_ptr, ptr);
        return XML_TOK_DECL_OPEN;
    }

    XML_TOK_PARTIAL
}

extern "C" fn normal_scanDecl(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    scan_decl_with(
        enc,
        ptr,
        end,
        nextTokPtr,
        1,
        normal_byte_type,
        normal_scanComment,
    )
}
extern "C" fn normal_checkPiTarget(
    _enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    tokPtr: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    check_pi_target_with(ptr, end, tokPtr, 1, read_normal_ascii_unit)
}
extern "C" fn normal_scanPi(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tok: ::core::ffi::c_int = 0;
    let target = ptr;
    if !has_c_chars(ptr, end, 1) {
        return XML_TOK_PARTIAL;
    }
    let mut c2rust_current_block_32: u64;
    match normal_byte_type(enc, ptr) {
        29 => {
            write_copy(nextTokPtr, ptr);
            return XML_TOK_INVALID;
        }
        22 | 24 => {
            c2rust_current_block_32 = 11470911313929454839;
        }
        5 | 6 | 7 => {
            let width = normal_byte_type(enc, ptr) as usize - 3;
            if !has_c_chars(ptr, end, width) {
                return XML_TOK_PARTIAL_CHAR;
            }
            if normal_is_invalid(enc, width, ptr) != 0 || normal_is_name_start(enc, width, ptr) == 0
            {
                write_copy(nextTokPtr, ptr);
                return XML_TOK_INVALID;
            }
            ptr = add_const_c_char(ptr, width as isize);
            c2rust_current_block_32 = 14763689060501151050;
        }
        _ => {
            write_copy(nextTokPtr, ptr);
            return XML_TOK_INVALID;
        }
    }
    if c2rust_current_block_32 == 11470911313929454839 {
        ptr = add_const_c_char(ptr, 1);
    }
    while has_c_chars(ptr, end, 1) {
        let mut c2rust_current_block_118: u64;
        match normal_byte_type(enc, ptr) {
            29 => {
                write_copy(nextTokPtr, ptr);
                return XML_TOK_INVALID;
            }
            22 | 24 | 25 | 26 | 27 => {
                c2rust_current_block_118 = 8485341570193076947;
            }
            5 | 6 | 7 => {
                let width = normal_byte_type(enc, ptr) as usize - 3;
                if !has_c_chars(ptr, end, width) {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if normal_is_invalid(enc, width, ptr) != 0
                    || normal_is_name_char(enc, width, ptr) == 0
                {
                    write_copy(nextTokPtr, ptr);
                    return XML_TOK_INVALID;
                }
                ptr = add_const_c_char(ptr, width as isize);
                c2rust_current_block_118 = 13349765058737954042;
            }
            21 | 9 | 10 => {
                if normal_checkPiTarget(enc, target, ptr, &raw mut tok) == 0 {
                    write_copy(nextTokPtr, ptr);
                    return XML_TOK_INVALID;
                }
                ptr = add_const_c_char(ptr, 1);
                while has_c_chars(ptr, end, 1) {
                    match normal_byte_type(enc, ptr) {
                        5 | 6 | 7 => {
                            let width = normal_byte_type(enc, ptr) as usize - 3;
                            if !has_c_chars(ptr, end, width) {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if normal_is_invalid(enc, width, ptr) != 0 {
                                write_copy(nextTokPtr, ptr);
                                return XML_TOK_INVALID;
                            }
                            ptr = add_const_c_char(ptr, width as isize);
                        }
                        0 | 1 | 8 => {
                            write_copy(nextTokPtr, ptr);
                            return XML_TOK_INVALID;
                        }
                        15 => {
                            ptr = add_const_c_char(ptr, 1);
                            if !has_c_chars(ptr, end, 1) {
                                return XML_TOK_PARTIAL;
                            }
                            if c_char_is_ascii(ptr, b'>') {
                                write_copy(nextTokPtr, add_const_c_char(ptr, 1));
                                return tok;
                            }
                        }
                        _ => {
                            ptr = add_const_c_char(ptr, 1);
                        }
                    }
                }
                return XML_TOK_PARTIAL;
            }
            15 => {
                if normal_checkPiTarget(enc, target, ptr, &raw mut tok) == 0 {
                    write_copy(nextTokPtr, ptr);
                    return XML_TOK_INVALID;
                }
                ptr = add_const_c_char(ptr, 1);
                if !has_c_chars(ptr, end, 1) {
                    return XML_TOK_PARTIAL;
                }
                if c_char_is_ascii(ptr, b'>') {
                    write_copy(nextTokPtr, add_const_c_char(ptr, 1));
                    return tok;
                }
                c2rust_current_block_118 = 11310415194689177606;
            }
            _ => {
                c2rust_current_block_118 = 11310415194689177606;
            }
        }
        match c2rust_current_block_118 {
            11310415194689177606 => {
                write_copy(nextTokPtr, ptr);
                return XML_TOK_INVALID;
            }
            8485341570193076947 => {
                ptr = add_const_c_char(ptr, 1);
            }
            _ => {}
        }
    }
    XML_TOK_PARTIAL
}
extern "C" fn normal_scanCdataSection(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let _ = enc;
    scan_cdata_section_open_with(ptr, end, nextTokPtr, 1, read_normal_ascii_unit)
}
extern "C" fn normal_cdataSectionTok(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    cdata_section_tok_normal(enc, ptr, end, nextTokPtr)
}
extern "C" fn normal_scanEndTag(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !has_c_chars(ptr, end, 1) {
        return XML_TOK_PARTIAL;
    }
    let mut c2rust_current_block_32: u64;
    match normal_byte_type(enc, ptr) {
        29 => {
            write_copy(nextTokPtr, ptr);
            return XML_TOK_INVALID;
        }
        22 | 24 => {
            c2rust_current_block_32 = 4324628675098861213;
        }
        5 | 6 | 7 => {
            let width = normal_byte_type(enc, ptr) as usize - 3;
            if !has_c_chars(ptr, end, width) {
                return XML_TOK_PARTIAL_CHAR;
            }
            if normal_is_invalid(enc, width, ptr) != 0 || normal_is_name_start(enc, width, ptr) == 0
            {
                write_copy(nextTokPtr, ptr);
                return XML_TOK_INVALID;
            }
            ptr = add_const_c_char(ptr, width as isize);
            c2rust_current_block_32 = 7056779235015430508;
        }
        _ => {
            write_copy(nextTokPtr, ptr);
            return XML_TOK_INVALID;
        }
    }
    if c2rust_current_block_32 == 4324628675098861213 {
        ptr = add_const_c_char(ptr, 1);
    }
    while has_c_chars(ptr, end, 1) {
        let mut c2rust_current_block_73: u64;
        match normal_byte_type(enc, ptr) {
            29 => {
                write_copy(nextTokPtr, ptr);
                return XML_TOK_INVALID;
            }
            22 | 24 | 25 | 26 | 27 => {
                c2rust_current_block_73 = 14883924698754021420;
            }
            5 | 6 | 7 => {
                let width = normal_byte_type(enc, ptr) as usize - 3;
                if !has_c_chars(ptr, end, width) {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if normal_is_invalid(enc, width, ptr) != 0
                    || normal_is_name_char(enc, width, ptr) == 0
                {
                    write_copy(nextTokPtr, ptr);
                    return XML_TOK_INVALID;
                }
                ptr = add_const_c_char(ptr, width as isize);
                c2rust_current_block_73 = 981995395831942902;
            }
            21 | 9 | 10 => {
                ptr = add_const_c_char(ptr, 1);
                while has_c_chars(ptr, end, 1) {
                    match normal_byte_type(enc, ptr) {
                        21 | 9 | 10 => {}
                        11 => {
                            write_copy(nextTokPtr, add_const_c_char(ptr, 1));
                            return XML_TOK_END_TAG;
                        }
                        _ => {
                            write_copy(nextTokPtr, ptr);
                            return XML_TOK_INVALID;
                        }
                    }
                    ptr = add_const_c_char(ptr, 1);
                }
                return XML_TOK_PARTIAL;
            }
            23 => {
                ptr = add_const_c_char(ptr, 1);
                c2rust_current_block_73 = 981995395831942902;
            }
            11 => {
                write_copy(nextTokPtr, add_const_c_char(ptr, 1));
                return XML_TOK_END_TAG;
            }
            _ => {
                write_copy(nextTokPtr, ptr);
                return XML_TOK_INVALID;
            }
        }
        if c2rust_current_block_73 == 14883924698754021420 {
            ptr = add_const_c_char(ptr, 1);
        }
    }
    XML_TOK_PARTIAL
}
extern "C" fn normal_scanHexCharRef(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    scan_hex_char_ref_with(enc, ptr, end, nextTokPtr, 1, normal_byte_type)
}
extern "C" fn normal_scanCharRef(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    scan_char_ref_with(
        enc,
        ptr,
        end,
        nextTokPtr,
        1,
        read_normal_ascii_unit,
        normal_byte_type,
    )
}
extern "C" fn normal_scanRef(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    scan_ref_with(
        enc,
        ptr,
        end,
        nextTokPtr,
        1,
        normal_byte_type,
        |_| false,
        |_| false,
        normal_name_start_seq_valid,
        normal_name_char_seq_valid,
        normal_scanCharRef,
    )
}
extern "C" fn normal_scanAtts(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut hadColon: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_186: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                29 => {
                    if 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    c2rust_current_block_186 = 3818392175876617014;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_186 = 3818392175876617014;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid2
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName2
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    c2rust_current_block_186 = 1634947208139838470;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid3
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName3
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                    c2rust_current_block_186 = 1634947208139838470;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid4
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName4
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    c2rust_current_block_186 = 1634947208139838470;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    hadColon = 1 as ::core::ffi::c_int;
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    let mut c2rust_current_block_64: u64;
                    match (*(enc as *const normal_encoding)).type_0
                        [*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                    {
                        29 => {
                            if 0 as ::core::ffi::c_int == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            c2rust_current_block_64 = 7083593080606520045;
                        }
                        22 | 24 => {
                            c2rust_current_block_64 = 7083593080606520045;
                        }
                        5 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 2 as ::core::ffi::c_long
                            {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if (*(enc as *const normal_encoding))
                                .isInvalid2
                                .expect("non-null function pointer")(
                                enc, ptr
                            ) != 0
                                || (*(enc as *const normal_encoding))
                                    .isNmstrt2
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            c2rust_current_block_64 = 10930818133215224067;
                        }
                        6 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 3 as ::core::ffi::c_long
                            {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if (*(enc as *const normal_encoding))
                                .isInvalid3
                                .expect("non-null function pointer")(
                                enc, ptr
                            ) != 0
                                || (*(enc as *const normal_encoding))
                                    .isNmstrt3
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                            c2rust_current_block_64 = 10930818133215224067;
                        }
                        7 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 4 as ::core::ffi::c_long
                            {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if (*(enc as *const normal_encoding))
                                .isInvalid4
                                .expect("non-null function pointer")(
                                enc, ptr
                            ) != 0
                                || (*(enc as *const normal_encoding))
                                    .isNmstrt4
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                            c2rust_current_block_64 = 10930818133215224067;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                    match c2rust_current_block_64 {
                        7083593080606520045 => {
                            ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        }
                        _ => {}
                    }
                    c2rust_current_block_186 = 1634947208139838470;
                }
                21 | 9 | 10 => {
                    loop {
                        let mut t: ::core::ffi::c_int = 0;
                        ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        t = (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int;
                        if t == BT_EQUALS as ::core::ffi::c_int {
                            break;
                        }
                        match t {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                    }
                    c2rust_current_block_186 = 10853015579903106591;
                }
                14 => {
                    c2rust_current_block_186 = 10853015579903106591;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match c2rust_current_block_186 {
                10853015579903106591 => {
                    let mut open: ::core::ffi::c_int = 0;
                    hadColon = 0 as ::core::ffi::c_int;
                    loop {
                        ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        open = (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int;
                        if open == BT_QUOT as ::core::ffi::c_int
                            || open == BT_APOS as ::core::ffi::c_int
                        {
                            break;
                        }
                        match open {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                    }
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                    loop {
                        let mut t_0: ::core::ffi::c_int = 0;
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        t_0 = (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int;
                        if t_0 == open {
                            break;
                        }
                        match t_0 {
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 2 as ::core::ffi::c_long
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid2
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 3 as ::core::ffi::c_long
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid3
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 4 as ::core::ffi::c_long
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid4
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                            }
                            0 | 1 | 8 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            3 => {
                                let mut tok: ::core::ffi::c_int = normal_scanRef(
                                    enc,
                                    ptr.offset(1 as ::core::ffi::c_int as isize),
                                    end,
                                    &raw mut ptr,
                                );
                                if tok <= 0 as ::core::ffi::c_int {
                                    if tok == XML_TOK_INVALID {
                                        *nextTokPtr = ptr;
                                    }
                                    return tok;
                                }
                            }
                            2 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            _ => {
                                ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                            }
                        }
                    }
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    match (*(enc as *const normal_encoding)).type_0
                        [*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                    {
                        21 | 9 | 10 => {
                            loop {
                                ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                                        as ::core::ffi::c_long)
                                {
                                    return XML_TOK_PARTIAL;
                                }
                                match (*(enc as *const normal_encoding)).type_0
                                    [*ptr as ::core::ffi::c_uchar as usize]
                                    as ::core::ffi::c_int
                                {
                                    29 => {
                                        if 0 as ::core::ffi::c_int == 0 {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        c2rust_current_block_186 = 11210999262882855128;
                                        break;
                                    }
                                    22 | 24 => {
                                        c2rust_current_block_186 = 11210999262882855128;
                                        break;
                                    }
                                    5 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long)
                                            < 2 as ::core::ffi::c_long
                                        {
                                            return XML_TOK_PARTIAL_CHAR;
                                        }
                                        if (*(enc as *const normal_encoding))
                                            .isInvalid2
                                            .expect("non-null function pointer")(
                                            enc, ptr
                                        ) != 0
                                            || (*(enc as *const normal_encoding))
                                                .isNmstrt2
                                                .expect("non-null function pointer")(
                                                enc, ptr
                                            ) == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                        c2rust_current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    6 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long)
                                            < 3 as ::core::ffi::c_long
                                        {
                                            return XML_TOK_PARTIAL_CHAR;
                                        }
                                        if (*(enc as *const normal_encoding))
                                            .isInvalid3
                                            .expect("non-null function pointer")(
                                            enc, ptr
                                        ) != 0
                                            || (*(enc as *const normal_encoding))
                                                .isNmstrt3
                                                .expect("non-null function pointer")(
                                                enc, ptr
                                            ) == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                                        c2rust_current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    7 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long)
                                            < 4 as ::core::ffi::c_long
                                        {
                                            return XML_TOK_PARTIAL_CHAR;
                                        }
                                        if (*(enc as *const normal_encoding))
                                            .isInvalid4
                                            .expect("non-null function pointer")(
                                            enc, ptr
                                        ) != 0
                                            || (*(enc as *const normal_encoding))
                                                .isNmstrt4
                                                .expect("non-null function pointer")(
                                                enc, ptr
                                            ) == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                                        c2rust_current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    21 | 9 | 10 => {}
                                    11 => {
                                        c2rust_current_block_186 = 2944436519209994553;
                                        break;
                                    }
                                    17 => {
                                        c2rust_current_block_186 = 398073151373002430;
                                        break;
                                    }
                                    _ => {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                }
                            }
                            match c2rust_current_block_186 {
                                2944436519209994553 => {}
                                398073151373002430 => {}
                                1634947208139838470 => {}
                                _ => {
                                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                                    c2rust_current_block_186 = 1634947208139838470;
                                }
                            }
                        }
                        17 => {
                            c2rust_current_block_186 = 398073151373002430;
                        }
                        11 => {
                            c2rust_current_block_186 = 2944436519209994553;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                    match c2rust_current_block_186 {
                        1634947208139838470 => {}
                        _ => match c2rust_current_block_186 {
                            398073151373002430 => {
                                ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                                        as ::core::ffi::c_long)
                                {
                                    return XML_TOK_PARTIAL;
                                }
                                if !(*ptr as ::core::ffi::c_int == 0x3e as ::core::ffi::c_int) {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                                return XML_TOK_EMPTY_ELEMENT_WITH_ATTS;
                            }
                            _ => {
                                *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                                return XML_TOK_START_TAG_WITH_ATTS;
                            }
                        },
                    }
                }
                3818392175876617014 => {
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }
}
extern "C" fn normal_scanLt(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut hadColon: ::core::ffi::c_int = 0;
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL;
        }
        let mut c2rust_current_block_45: u64;
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            29 => {
                if 0 as ::core::ffi::c_int == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                c2rust_current_block_45 = 2165477741955893522;
            }
            22 | 24 => {
                c2rust_current_block_45 = 2165477741955893522;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                    || (*(enc as *const normal_encoding))
                        .isNmstrt2
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                c2rust_current_block_45 = 8180496224585318153;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                    || (*(enc as *const normal_encoding))
                        .isNmstrt3
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                c2rust_current_block_45 = 8180496224585318153;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                    || (*(enc as *const normal_encoding))
                        .isNmstrt4
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                c2rust_current_block_45 = 8180496224585318153;
            }
            16 => {
                ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as ::core::ffi::c_long)
                {
                    return XML_TOK_PARTIAL;
                }
                match (*(enc as *const normal_encoding)).type_0
                    [*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                {
                    27 => {
                        return normal_scanComment(
                            enc,
                            ptr.offset(1 as ::core::ffi::c_int as isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    20 => {
                        return normal_scanCdataSection(
                            enc,
                            ptr.offset(1 as ::core::ffi::c_int as isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            15 => {
                return normal_scanPi(
                    enc,
                    ptr.offset(1 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            17 => {
                return normal_scanEndTag(
                    enc,
                    ptr.offset(1 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match c2rust_current_block_45 {
            2165477741955893522 => {
                ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
            }
            _ => {}
        }
        hadColon = 0 as ::core::ffi::c_int;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_161: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                29 => {
                    if 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    c2rust_current_block_161 = 6701753098489376273;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_161 = 6701753098489376273;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid2
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName2
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    c2rust_current_block_161 = 14714495436747744489;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid3
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName3
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                    c2rust_current_block_161 = 14714495436747744489;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid4
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName4
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    c2rust_current_block_161 = 14714495436747744489;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    hadColon = 1 as ::core::ffi::c_int;
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    let mut c2rust_current_block_112: u64;
                    match (*(enc as *const normal_encoding)).type_0
                        [*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                    {
                        29 => {
                            if 0 as ::core::ffi::c_int == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            c2rust_current_block_112 = 9169466483824547789;
                        }
                        22 | 24 => {
                            c2rust_current_block_112 = 9169466483824547789;
                        }
                        5 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 2 as ::core::ffi::c_long
                            {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if (*(enc as *const normal_encoding))
                                .isInvalid2
                                .expect("non-null function pointer")(
                                enc, ptr
                            ) != 0
                                || (*(enc as *const normal_encoding))
                                    .isNmstrt2
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            c2rust_current_block_112 = 2616667235040759262;
                        }
                        6 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 3 as ::core::ffi::c_long
                            {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if (*(enc as *const normal_encoding))
                                .isInvalid3
                                .expect("non-null function pointer")(
                                enc, ptr
                            ) != 0
                                || (*(enc as *const normal_encoding))
                                    .isNmstrt3
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                            c2rust_current_block_112 = 2616667235040759262;
                        }
                        7 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 4 as ::core::ffi::c_long
                            {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if (*(enc as *const normal_encoding))
                                .isInvalid4
                                .expect("non-null function pointer")(
                                enc, ptr
                            ) != 0
                                || (*(enc as *const normal_encoding))
                                    .isNmstrt4
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                            c2rust_current_block_112 = 2616667235040759262;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                    match c2rust_current_block_112 {
                        9169466483824547789 => {
                            ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        }
                        _ => {}
                    }
                    c2rust_current_block_161 = 14714495436747744489;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                    loop {
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            c2rust_current_block_161 = 13215501469961642988;
                            break;
                        }
                        match (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                        {
                            29 => {
                                if 0 as ::core::ffi::c_int == 0 {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                c2rust_current_block_161 = 7939927167482451446;
                            }
                            22 | 24 => {
                                c2rust_current_block_161 = 7939927167482451446;
                            }
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 2 as ::core::ffi::c_long
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid2
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                    || (*(enc as *const normal_encoding))
                                        .isNmstrt2
                                        .expect("non-null function pointer")(
                                        enc, ptr
                                    ) == 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                c2rust_current_block_161 = 16314074004867283505;
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 3 as ::core::ffi::c_long
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid3
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                    || (*(enc as *const normal_encoding))
                                        .isNmstrt3
                                        .expect("non-null function pointer")(
                                        enc, ptr
                                    ) == 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                                c2rust_current_block_161 = 16314074004867283505;
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 4 as ::core::ffi::c_long
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid4
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                    || (*(enc as *const normal_encoding))
                                        .isNmstrt4
                                        .expect("non-null function pointer")(
                                        enc, ptr
                                    ) == 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                                c2rust_current_block_161 = 16314074004867283505;
                            }
                            11 => {
                                c2rust_current_block_161 = 5640065479517572396;
                                break;
                            }
                            17 => {
                                c2rust_current_block_161 = 12549409781983877175;
                                break;
                            }
                            21 | 9 | 10 => {
                                ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                                continue;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                        match c2rust_current_block_161 {
                            7939927167482451446 => {
                                ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                            }
                            _ => {}
                        }
                        return normal_scanAtts(enc, ptr, end, nextTokPtr);
                    }
                    match c2rust_current_block_161 {
                        5640065479517572396 => {}
                        12549409781983877175 => {}
                        _ => return XML_TOK_PARTIAL,
                    }
                }
                11 => {
                    c2rust_current_block_161 = 5640065479517572396;
                }
                17 => {
                    c2rust_current_block_161 = 12549409781983877175;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match c2rust_current_block_161 {
                12549409781983877175 => {
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if !(*ptr as ::core::ffi::c_int == 0x3e as ::core::ffi::c_int) {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                    return XML_TOK_EMPTY_ELEMENT_NO_ATTS;
                }
                5640065479517572396 => {
                    *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                    return XML_TOK_START_TAG_NO_ATTS;
                }
                6701753098489376273 => {
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }
}
extern "C" fn normal_contentTok(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if ptr >= end {
        return XML_TOK_NONE;
    }

    ptr = match normal_byte_type(enc, ptr) {
        2 => {
            return normal_scanLt(enc, add_const_c_char(ptr, 1), end, nextTokPtr);
        }
        3 => {
            return normal_scanRef(enc, add_const_c_char(ptr, 1), end, nextTokPtr);
        }
        9 => {
            ptr = add_const_c_char(ptr, 1);
            if !has_c_chars(ptr, end, 1) {
                return XML_TOK_TRAILING_CR;
            }
            if normal_byte_type(enc, ptr) == BT_LF as ::core::ffi::c_int {
                ptr = add_const_c_char(ptr, 1);
            }
            set_const_c_char_ptr(nextTokPtr, ptr);
            return XML_TOK_DATA_NEWLINE;
        }
        10 => {
            set_const_c_char_ptr(nextTokPtr, add_const_c_char(ptr, 1));
            return XML_TOK_DATA_NEWLINE;
        }
        4 => {
            ptr = add_const_c_char(ptr, 1);
            if !has_c_chars(ptr, end, 1) {
                return XML_TOK_TRAILING_RSQB;
            }
            if read_c_char(ptr) as ::core::ffi::c_int == 0x5d as ::core::ffi::c_int {
                ptr = add_const_c_char(ptr, 1);
                if !has_c_chars(ptr, end, 1) {
                    return XML_TOK_TRAILING_RSQB;
                }
                if read_c_char(ptr) as ::core::ffi::c_int == 0x3e as ::core::ffi::c_int {
                    set_const_c_char_ptr(nextTokPtr, ptr);
                    return XML_TOK_INVALID;
                }
                add_const_c_char(ptr, -1)
            } else {
                ptr
            }
        }
        5 => {
            if !has_c_chars(ptr, end, 2) {
                return XML_TOK_PARTIAL_CHAR;
            }
            if normal_is_invalid(enc, 2, ptr) != 0 {
                set_const_c_char_ptr(nextTokPtr, ptr);
                return XML_TOK_INVALID;
            }
            add_const_c_char(ptr, 2)
        }
        6 => {
            if !has_c_chars(ptr, end, 3) {
                return XML_TOK_PARTIAL_CHAR;
            }
            if normal_is_invalid(enc, 3, ptr) != 0 {
                set_const_c_char_ptr(nextTokPtr, ptr);
                return XML_TOK_INVALID;
            }
            add_const_c_char(ptr, 3)
        }
        7 => {
            if !has_c_chars(ptr, end, 4) {
                return XML_TOK_PARTIAL_CHAR;
            }
            if normal_is_invalid(enc, 4, ptr) != 0 {
                set_const_c_char_ptr(nextTokPtr, ptr);
                return XML_TOK_INVALID;
            }
            add_const_c_char(ptr, 4)
        }
        0 | 1 | 8 => {
            set_const_c_char_ptr(nextTokPtr, ptr);
            return XML_TOK_INVALID;
        }
        _ => add_const_c_char(ptr, 1),
    };

    while has_c_chars(ptr, end, 1) {
        ptr = match normal_byte_type(enc, ptr) {
            5 => {
                if !has_c_chars(ptr, end, 2) || normal_is_invalid(enc, 2, ptr) != 0 {
                    set_const_c_char_ptr(nextTokPtr, ptr);
                    return XML_TOK_DATA_CHARS;
                }
                add_const_c_char(ptr, 2)
            }
            6 => {
                if !has_c_chars(ptr, end, 3) || normal_is_invalid(enc, 3, ptr) != 0 {
                    set_const_c_char_ptr(nextTokPtr, ptr);
                    return XML_TOK_DATA_CHARS;
                }
                add_const_c_char(ptr, 3)
            }
            7 => {
                if !has_c_chars(ptr, end, 4) || normal_is_invalid(enc, 4, ptr) != 0 {
                    set_const_c_char_ptr(nextTokPtr, ptr);
                    return XML_TOK_DATA_CHARS;
                }
                add_const_c_char(ptr, 4)
            }
            4 => {
                if has_c_chars(ptr, end, 2) {
                    if read_c_char(add_const_c_char(ptr, 1)) as ::core::ffi::c_int
                        != 0x5d as ::core::ffi::c_int
                    {
                        add_const_c_char(ptr, 1)
                    } else if has_c_chars(ptr, end, 3) {
                        if read_c_char(add_const_c_char(ptr, 2)) as ::core::ffi::c_int
                            == 0x3e as ::core::ffi::c_int
                        {
                            set_const_c_char_ptr(nextTokPtr, add_const_c_char(ptr, 2));
                            return XML_TOK_INVALID;
                        }
                        add_const_c_char(ptr, 1)
                    } else {
                        set_const_c_char_ptr(nextTokPtr, ptr);
                        return XML_TOK_DATA_CHARS;
                    }
                } else {
                    set_const_c_char_ptr(nextTokPtr, ptr);
                    return XML_TOK_DATA_CHARS;
                }
            }
            3 | 2 | 0 | 1 | 8 | 9 | 10 => {
                set_const_c_char_ptr(nextTokPtr, ptr);
                return XML_TOK_DATA_CHARS;
            }
            _ => add_const_c_char(ptr, 1),
        };
    }

    set_const_c_char_ptr(nextTokPtr, ptr);
    XML_TOK_DATA_CHARS
}
extern "C" fn normal_scanPercent(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !has_c_chars(ptr, end, 1) {
        return XML_TOK_PARTIAL;
    }

    ptr = match normal_byte_type(enc, ptr) {
        29 => {
            set_const_c_char_ptr(nextTokPtr, ptr);
            return XML_TOK_INVALID;
        }
        22 | 24 => add_const_c_char(ptr, 1),
        5 => {
            if !has_c_chars(ptr, end, 2) {
                return XML_TOK_PARTIAL_CHAR;
            }
            if normal_is_invalid(enc, 2, ptr) != 0 || normal_is_name_start(enc, 2, ptr) == 0 {
                set_const_c_char_ptr(nextTokPtr, ptr);
                return XML_TOK_INVALID;
            }
            add_const_c_char(ptr, 2)
        }
        6 => {
            if !has_c_chars(ptr, end, 3) {
                return XML_TOK_PARTIAL_CHAR;
            }
            if normal_is_invalid(enc, 3, ptr) != 0 || normal_is_name_start(enc, 3, ptr) == 0 {
                set_const_c_char_ptr(nextTokPtr, ptr);
                return XML_TOK_INVALID;
            }
            add_const_c_char(ptr, 3)
        }
        7 => {
            if !has_c_chars(ptr, end, 4) {
                return XML_TOK_PARTIAL_CHAR;
            }
            if normal_is_invalid(enc, 4, ptr) != 0 || normal_is_name_start(enc, 4, ptr) == 0 {
                set_const_c_char_ptr(nextTokPtr, ptr);
                return XML_TOK_INVALID;
            }
            add_const_c_char(ptr, 4)
        }
        21 | 10 | 9 | 30 => {
            set_const_c_char_ptr(nextTokPtr, ptr);
            return XML_TOK_PERCENT;
        }
        _ => {
            set_const_c_char_ptr(nextTokPtr, ptr);
            return XML_TOK_INVALID;
        }
    };

    while has_c_chars(ptr, end, 1) {
        ptr = match normal_byte_type(enc, ptr) {
            29 => {
                set_const_c_char_ptr(nextTokPtr, ptr);
                return XML_TOK_INVALID;
            }
            22 | 24 | 25 | 26 | 27 => add_const_c_char(ptr, 1),
            5 => {
                if !has_c_chars(ptr, end, 2) {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if normal_is_invalid(enc, 2, ptr) != 0 || normal_is_name_char(enc, 2, ptr) == 0 {
                    set_const_c_char_ptr(nextTokPtr, ptr);
                    return XML_TOK_INVALID;
                }
                add_const_c_char(ptr, 2)
            }
            6 => {
                if !has_c_chars(ptr, end, 3) {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if normal_is_invalid(enc, 3, ptr) != 0 || normal_is_name_char(enc, 3, ptr) == 0 {
                    set_const_c_char_ptr(nextTokPtr, ptr);
                    return XML_TOK_INVALID;
                }
                add_const_c_char(ptr, 3)
            }
            7 => {
                if !has_c_chars(ptr, end, 4) {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if normal_is_invalid(enc, 4, ptr) != 0 || normal_is_name_char(enc, 4, ptr) == 0 {
                    set_const_c_char_ptr(nextTokPtr, ptr);
                    return XML_TOK_INVALID;
                }
                add_const_c_char(ptr, 4)
            }
            18 => {
                set_const_c_char_ptr(nextTokPtr, add_const_c_char(ptr, 1));
                return XML_TOK_PARAM_ENTITY_REF;
            }
            _ => {
                set_const_c_char_ptr(nextTokPtr, ptr);
                return XML_TOK_INVALID;
            }
        };
    }

    XML_TOK_PARTIAL
}
extern "C" fn normal_scanPoundName(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !has_c_chars(ptr, end, 1) {
        return XML_TOK_PARTIAL;
    }

    ptr = match normal_byte_type(enc, ptr) {
        29 => {
            set_const_c_char_ptr(nextTokPtr, ptr);
            return XML_TOK_INVALID;
        }
        22 | 24 => add_const_c_char(ptr, 1),
        5 => {
            if !has_c_chars(ptr, end, 2) {
                return XML_TOK_PARTIAL_CHAR;
            }
            if normal_is_invalid(enc, 2, ptr) != 0 || normal_is_name_start(enc, 2, ptr) == 0 {
                set_const_c_char_ptr(nextTokPtr, ptr);
                return XML_TOK_INVALID;
            }
            add_const_c_char(ptr, 2)
        }
        6 => {
            if !has_c_chars(ptr, end, 3) {
                return XML_TOK_PARTIAL_CHAR;
            }
            if normal_is_invalid(enc, 3, ptr) != 0 || normal_is_name_start(enc, 3, ptr) == 0 {
                set_const_c_char_ptr(nextTokPtr, ptr);
                return XML_TOK_INVALID;
            }
            add_const_c_char(ptr, 3)
        }
        7 => {
            if !has_c_chars(ptr, end, 4) {
                return XML_TOK_PARTIAL_CHAR;
            }
            if normal_is_invalid(enc, 4, ptr) != 0 || normal_is_name_start(enc, 4, ptr) == 0 {
                set_const_c_char_ptr(nextTokPtr, ptr);
                return XML_TOK_INVALID;
            }
            add_const_c_char(ptr, 4)
        }
        _ => {
            set_const_c_char_ptr(nextTokPtr, ptr);
            return XML_TOK_INVALID;
        }
    };

    while has_c_chars(ptr, end, 1) {
        ptr = match normal_byte_type(enc, ptr) {
            29 => {
                set_const_c_char_ptr(nextTokPtr, ptr);
                return XML_TOK_INVALID;
            }
            22 | 24 | 25 | 26 | 27 => add_const_c_char(ptr, 1),
            5 => {
                if !has_c_chars(ptr, end, 2) {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if normal_is_invalid(enc, 2, ptr) != 0 || normal_is_name_char(enc, 2, ptr) == 0 {
                    set_const_c_char_ptr(nextTokPtr, ptr);
                    return XML_TOK_INVALID;
                }
                add_const_c_char(ptr, 2)
            }
            6 => {
                if !has_c_chars(ptr, end, 3) {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if normal_is_invalid(enc, 3, ptr) != 0 || normal_is_name_char(enc, 3, ptr) == 0 {
                    set_const_c_char_ptr(nextTokPtr, ptr);
                    return XML_TOK_INVALID;
                }
                add_const_c_char(ptr, 3)
            }
            7 => {
                if !has_c_chars(ptr, end, 4) {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if normal_is_invalid(enc, 4, ptr) != 0 || normal_is_name_char(enc, 4, ptr) == 0 {
                    set_const_c_char_ptr(nextTokPtr, ptr);
                    return XML_TOK_INVALID;
                }
                add_const_c_char(ptr, 4)
            }
            9 | 10 | 21 | 32 | 11 | 30 | 36 => {
                set_const_c_char_ptr(nextTokPtr, ptr);
                return XML_TOK_POUND_NAME;
            }
            _ => {
                set_const_c_char_ptr(nextTokPtr, ptr);
                return XML_TOK_INVALID;
            }
        };
    }

    -XML_TOK_POUND_NAME
}
fn allow_literal_multibyte_sequence(
    _enc: *const ENCODING,
    _width: usize,
    _ptr: *const ::core::ffi::c_char,
) -> bool {
    true
}

fn normal_literal_multibyte_sequence_valid(
    enc: *const ENCODING,
    width: usize,
    ptr: *const ::core::ffi::c_char,
) -> bool {
    normal_is_invalid(enc, width, ptr) == 0
}

fn multibyte_sequence_len(byte_type: ::core::ffi::c_int) -> usize {
    match byte_type {
        value if value == BT_LEAD2 as ::core::ffi::c_int => 2,
        value if value == BT_LEAD3 as ::core::ffi::c_int => 3,
        value if value == BT_LEAD4 as ::core::ffi::c_int => 4,
        _ => unreachable!("unexpected multibyte type: {byte_type}"),
    }
}

fn scan_lit_with(
    open: ::core::ffi::c_int,
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    unit_size: usize,
    byte_type: ByteTypeReader,
    validate_multibyte: fn(*const ENCODING, usize, *const ::core::ffi::c_char) -> bool,
) -> ::core::ffi::c_int {
    let unit_offset = unit_size as isize;

    while remaining_const_c_chars(ptr, end) >= unit_size {
        let token_type = byte_type(enc, ptr);

        match token_type {
            value
                if value == BT_LEAD2 as ::core::ffi::c_int
                    || value == BT_LEAD3 as ::core::ffi::c_int
                    || value == BT_LEAD4 as ::core::ffi::c_int =>
            {
                let width = multibyte_sequence_len(token_type);
                if remaining_const_c_chars(ptr, end) < width {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if !validate_multibyte(enc, width, ptr) {
                    set_next_token_ptr(next_tok_ptr, ptr);
                    return XML_TOK_INVALID;
                }
                ptr = add_const_c_char(ptr, width as isize);
            }
            value
                if value == BT_NONXML as ::core::ffi::c_int
                    || value == BT_MALFORM as ::core::ffi::c_int
                    || value == BT_TRAIL as ::core::ffi::c_int =>
            {
                set_next_token_ptr(next_tok_ptr, ptr);
                return XML_TOK_INVALID;
            }
            value
                if value == BT_QUOT as ::core::ffi::c_int
                    || value == BT_APOS as ::core::ffi::c_int =>
            {
                ptr = add_const_c_char(ptr, unit_offset);
                if token_type == open {
                    if remaining_const_c_chars(ptr, end) < unit_size {
                        return -XML_TOK_LITERAL;
                    }
                    set_next_token_ptr(next_tok_ptr, ptr);
                    return match byte_type(enc, ptr) {
                        value
                            if value == BT_S as ::core::ffi::c_int
                                || value == BT_CR as ::core::ffi::c_int
                                || value == BT_LF as ::core::ffi::c_int
                                || value == BT_GT as ::core::ffi::c_int
                                || value == BT_PERCNT as ::core::ffi::c_int
                                || value == BT_LSQB as ::core::ffi::c_int =>
                        {
                            XML_TOK_LITERAL
                        }
                        _ => XML_TOK_INVALID,
                    };
                }
            }
            _ => {
                ptr = add_const_c_char(ptr, unit_offset);
            }
        }
    }

    XML_TOK_PARTIAL
}

extern "C" fn normal_scanLit(
    mut open: ::core::ffi::c_int,
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    scan_lit_with(
        open,
        enc,
        ptr,
        end,
        nextTokPtr,
        1,
        normal_byte_type,
        normal_literal_multibyte_sequence_valid,
    )
}
extern "C" fn normal_prologTok(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut tok: ::core::ffi::c_int = 0;
        if ptr >= end {
            return XML_TOK_NONE;
        }
        if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            let mut n: size_t = end.offset_from(ptr) as ::core::ffi::c_long as size_t;
            if n & (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t != 0 {
                n &= !(1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t;
                if n == 0 as size_t {
                    return XML_TOK_PARTIAL;
                }
                end = ptr.offset(n as isize);
            }
        }
        let mut c2rust_current_block_124: u64;
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            12 => {
                return normal_scanLit(
                    BT_QUOT as ::core::ffi::c_int,
                    enc,
                    ptr.offset(1 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            13 => {
                return normal_scanLit(
                    BT_APOS as ::core::ffi::c_int,
                    enc,
                    ptr.offset(1 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            2 => {
                ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as ::core::ffi::c_long)
                {
                    return XML_TOK_PARTIAL;
                }
                match (*(enc as *const normal_encoding)).type_0
                    [*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                {
                    16 => {
                        return normal_scanDecl(
                            enc,
                            ptr.offset(1 as ::core::ffi::c_int as isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    15 => {
                        return normal_scanPi(
                            enc,
                            ptr.offset(1 as ::core::ffi::c_int as isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    22 | 24 | 29 | 5 | 6 | 7 => {
                        *nextTokPtr = ptr.offset(-(1 as ::core::ffi::c_int as isize));
                        return XML_TOK_INSTANCE_START;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            9 => {
                if ptr.offset(1 as ::core::ffi::c_int as isize) == end {
                    *nextTokPtr = end;
                    return -XML_TOK_PROLOG_S;
                }
                c2rust_current_block_124 = 6405334113228567422;
            }
            21 | 10 => {
                c2rust_current_block_124 = 6405334113228567422;
            }
            30 => {
                return normal_scanPercent(
                    enc,
                    ptr.offset(1 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            35 => {
                *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                return XML_TOK_COMMA;
            }
            20 => {
                *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                return XML_TOK_OPEN_BRACKET;
            }
            4 => {
                ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as ::core::ffi::c_long)
                {
                    return -XML_TOK_CLOSE_BRACKET;
                }
                if *ptr as ::core::ffi::c_int == 0x5d as ::core::ffi::c_int {
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0x3e as ::core::ffi::c_int
                    {
                        *nextTokPtr = ptr
                            .offset((2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize);
                        return XML_TOK_COND_SECT_CLOSE;
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_CLOSE_BRACKET;
            }
            31 => {
                *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                return XML_TOK_OPEN_PAREN;
            }
            32 => {
                ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as ::core::ffi::c_long)
                {
                    return -XML_TOK_CLOSE_PAREN;
                }
                match (*(enc as *const normal_encoding)).type_0
                    [*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                {
                    33 => {
                        *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        return XML_TOK_CLOSE_PAREN_ASTERISK;
                    }
                    15 => {
                        *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        return XML_TOK_CLOSE_PAREN_QUESTION;
                    }
                    34 => {
                        *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        return XML_TOK_CLOSE_PAREN_PLUS;
                    }
                    9 | 10 | 21 | 11 | 35 | 36 | 32 => {
                        *nextTokPtr = ptr;
                        return XML_TOK_CLOSE_PAREN;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            36 => {
                *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                return XML_TOK_OR;
            }
            11 => {
                *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                return XML_TOK_DECL_CLOSE;
            }
            19 => {
                return normal_scanPoundName(
                    enc,
                    ptr.offset(1 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    tok = XML_TOK_NAME;
                } else if (*(enc as *const normal_encoding))
                    .isName2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    tok = XML_TOK_NMTOKEN;
                } else {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                c2rust_current_block_124 = 2956972668325154207;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                    tok = XML_TOK_NAME;
                } else if (*(enc as *const normal_encoding))
                    .isName3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                    tok = XML_TOK_NMTOKEN;
                } else {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                c2rust_current_block_124 = 2956972668325154207;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    tok = XML_TOK_NAME;
                } else if (*(enc as *const normal_encoding))
                    .isName4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    tok = XML_TOK_NMTOKEN;
                } else {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                c2rust_current_block_124 = 2956972668325154207;
            }
            22 | 24 => {
                tok = XML_TOK_NAME;
                ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                c2rust_current_block_124 = 2956972668325154207;
            }
            25 | 26 | 27 | 23 => {
                tok = XML_TOK_NMTOKEN;
                ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                c2rust_current_block_124 = 2956972668325154207;
            }
            29 | _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match c2rust_current_block_124 {
            2956972668325154207 => {}
            _ => {
                loop {
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        break;
                    }
                    let mut c2rust_current_block_32: u64;
                    match (*(enc as *const normal_encoding)).type_0
                        [*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                    {
                        21 | 10 => {
                            c2rust_current_block_32 = 17500079516916021833;
                        }
                        9 => {
                            if ptr.offset(1 as ::core::ffi::c_int as isize) != end {
                                c2rust_current_block_32 = 17500079516916021833;
                            } else {
                                c2rust_current_block_32 = 3687018382384043009;
                            }
                        }
                        _ => {
                            c2rust_current_block_32 = 3687018382384043009;
                        }
                    }
                    match c2rust_current_block_32 {
                        17500079516916021833 => {}
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_PROLOG_S;
                        }
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_PROLOG_S;
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_210: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                29 => {
                    if 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    c2rust_current_block_210 = 17210391895989911948;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_210 = 17210391895989911948;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid2
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName2
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    c2rust_current_block_210 = 14244298717249035578;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid3
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName3
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                    c2rust_current_block_210 = 14244298717249035578;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid4
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName4
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    c2rust_current_block_210 = 14244298717249035578;
                }
                11 | 32 | 35 | 36 | 20 | 30 | 21 | 9 | 10 => {
                    *nextTokPtr = ptr;
                    return tok;
                }
                23 => {
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                    match tok {
                        XML_TOK_NAME => {
                            if !(end.offset_from(ptr) as ::core::ffi::c_long
                                >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                                    as ::core::ffi::c_long)
                            {
                                return XML_TOK_PARTIAL;
                            }
                            tok = XML_TOK_PREFIXED_NAME;
                            let mut c2rust_current_block_187: u64;
                            match (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                            {
                                29 => {
                                    if 0 as ::core::ffi::c_int == 0 {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    c2rust_current_block_187 = 2692573546887820791;
                                }
                                22 | 24 | 25 | 26 | 27 => {
                                    c2rust_current_block_187 = 2692573546887820791;
                                }
                                5 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long)
                                        < 2 as ::core::ffi::c_long
                                    {
                                        return XML_TOK_PARTIAL_CHAR;
                                    }
                                    if (*(enc as *const normal_encoding))
                                        .isInvalid2
                                        .expect("non-null function pointer")(
                                        enc, ptr
                                    ) != 0
                                        || (*(enc as *const normal_encoding))
                                            .isName2
                                            .expect("non-null function pointer")(
                                            enc, ptr
                                        ) == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                                6 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long)
                                        < 3 as ::core::ffi::c_long
                                    {
                                        return XML_TOK_PARTIAL_CHAR;
                                    }
                                    if (*(enc as *const normal_encoding))
                                        .isInvalid3
                                        .expect("non-null function pointer")(
                                        enc, ptr
                                    ) != 0
                                        || (*(enc as *const normal_encoding))
                                            .isName3
                                            .expect("non-null function pointer")(
                                            enc, ptr
                                        ) == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                                7 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long)
                                        < 4 as ::core::ffi::c_long
                                    {
                                        return XML_TOK_PARTIAL_CHAR;
                                    }
                                    if (*(enc as *const normal_encoding))
                                        .isInvalid4
                                        .expect("non-null function pointer")(
                                        enc, ptr
                                    ) != 0
                                        || (*(enc as *const normal_encoding))
                                            .isName4
                                            .expect("non-null function pointer")(
                                            enc, ptr
                                        ) == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                                _ => {
                                    tok = XML_TOK_NMTOKEN;
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                            }
                            match c2rust_current_block_187 {
                                2692573546887820791 => {
                                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                                }
                                _ => {}
                            }
                        }
                        XML_TOK_PREFIXED_NAME => {
                            tok = XML_TOK_NMTOKEN;
                        }
                        _ => {}
                    }
                    c2rust_current_block_210 = 14244298717249035578;
                }
                34 => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                    return XML_TOK_NAME_PLUS;
                }
                33 => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                    return XML_TOK_NAME_ASTERISK;
                }
                15 => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                    return XML_TOK_NAME_QUESTION;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match c2rust_current_block_210 {
                17210391895989911948 => {
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                }
                _ => {}
            }
        }
        return -tok;
    }
}
extern "C" fn normal_attributeValueTok(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if ptr >= end {
            return XML_TOK_NONE;
        } else if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL;
        }
        start = ptr;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                5 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                6 => {
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                }
                7 => {
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                }
                3 => {
                    if ptr == start {
                        return normal_scanRef(
                            enc,
                            ptr.offset(1 as ::core::ffi::c_int as isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                2 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_TRAILING_CR;
                        }
                        if (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                            == BT_LF as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        }
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                21 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        return XML_TOK_ATTRIBUTE_VALUE_S;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                _ => {
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }
}
extern "C" fn normal_entityValueTok(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if ptr >= end {
            return XML_TOK_NONE;
        } else if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL;
        }
        start = ptr;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                5 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                6 => {
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                }
                7 => {
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                }
                3 => {
                    if ptr == start {
                        return normal_scanRef(
                            enc,
                            ptr.offset(1 as ::core::ffi::c_int as isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                30 => {
                    if ptr == start {
                        let mut tok: ::core::ffi::c_int = normal_scanPercent(
                            enc,
                            ptr.offset(1 as ::core::ffi::c_int as isize),
                            end,
                            nextTokPtr,
                        );
                        return if tok == XML_TOK_PERCENT {
                            XML_TOK_INVALID
                        } else {
                            tok
                        };
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_TRAILING_CR;
                        }
                        if (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                            == BT_LF as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        }
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                _ => {
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }
}
extern "C" fn normal_ignoreSectionTok(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut level: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            let mut n: size_t = end.offset_from(ptr) as ::core::ffi::c_long as size_t;
            if n & (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t != 0 {
                n &= !(1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t;
                end = ptr.offset(n as isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid2
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid3
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid4
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                2 => {
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if *ptr as ::core::ffi::c_int == 0x21 as ::core::ffi::c_int {
                        ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        if *ptr as ::core::ffi::c_int == 0x5b as ::core::ffi::c_int {
                            level += 1;
                            ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        }
                    }
                }
                4 => {
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if *ptr as ::core::ffi::c_int == 0x5d as ::core::ffi::c_int {
                        ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        if *ptr as ::core::ffi::c_int == 0x3e as ::core::ffi::c_int {
                            ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                            if level == 0 as ::core::ffi::c_int {
                                *nextTokPtr = ptr;
                                return XML_TOK_IGNORE_SECT;
                            }
                            level -= 1;
                        }
                    }
                }
                _ => {
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                }
            }
        }
        return XML_TOK_PARTIAL;
    }
}
fn is_public_id_type(byte_type: ::core::ffi::c_int) -> bool {
    matches!(
        byte_type,
        XML_TOK_OPEN_BRACKET
            | XML_TOK_CLOSE_PAREN
            | XML_TOK_LITERAL
            | XML_TOK_COMMENT
            | XML_TOK_NAME_ASTERISK
            | XML_TOK_NAME_PLUS
            | XML_TOK_COND_SECT_CLOSE
            | XML_TOK_CLOSE_PAREN_QUESTION
            | XML_TOK_DECL_CLOSE
            | XML_TOK_BOM
            | XML_TOK_PROLOG_S
            | XML_TOK_ENTITY_REF
            | XML_TOK_CHAR_REF
            | XML_TOK_NAME
            | XML_TOK_DECL_OPEN
            | XML_TOK_COND_SECT_OPEN
            | XML_TOK_NAME_QUESTION
            | XML_TOK_NMTOKEN
            | XML_TOK_OPEN_PAREN
    )
}

fn validate_public_id(
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    unit_len: usize,
    bad_ptr: *mut *const ::core::ffi::c_char,
    mut byte_type_at: impl FnMut(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
    mut ascii_at: impl FnMut(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current = ptr;
    while remaining_const_c_chars(current, end) >= unit_len {
        let byte_type = byte_type_at(current);
        let ascii = ascii_at(current);
        let allowed = if is_public_id_type(byte_type) {
            true
        } else if byte_type == XML_TOK_OR {
            if ascii == 0x9 as ::core::ffi::c_int {
                write_copy(bad_ptr, current);
                return 0 as ::core::ffi::c_int;
            }
            true
        } else if matches!(byte_type, XML_TOK_CLOSE_BRACKET | XML_TOK_PERCENT) {
            ascii & !(0x7f as ::core::ffi::c_int) == 0
        } else {
            false
        };

        if !allowed && !matches!(ascii, 36 | 64) {
            write_copy(bad_ptr, current);
            return 0 as ::core::ffi::c_int;
        }

        current = add_const_c_char(current, unit_len as isize);
    }
    1 as ::core::ffi::c_int
}

extern "C" fn normal_isPublicId(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    badPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let ptr = add_const_c_char(ptr, 1);
    let end = add_const_c_char(end, -1);
    validate_public_id(
        ptr,
        end,
        1,
        badPtr,
        |current| {
            let byte = read_c_char(current) as ::core::ffi::c_uchar as usize;
            with_ref(enc.cast::<normal_encoding>(), |normal| {
                normal.type_0[byte] as ::core::ffi::c_int
            })
        },
        |current| read_c_char(current) as ::core::ffi::c_int,
    )
}
extern "C" fn normal_getAtts(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut attsMax: ::core::ffi::c_int,
    mut atts: *mut ATTRIBUTE,
) -> ::core::ffi::c_int {
    get_atts_with(
        enc,
        ptr,
        attsMax,
        atts,
        1,
        normal_byte_type,
        read_normal_ascii_unit,
    )
}
extern "C" fn normal_charRefNumber(
    _enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    char_ref_number_with(ptr, 1, read_normal_ascii_unit)
}
extern "C" fn normal_predefinedEntityName(
    _enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    predefined_entity_name_with(ptr, end, 1, read_normal_ascii_unit)
}
extern "C" fn normal_nameMatchesAscii(
    _enc: *const ENCODING,
    mut ptr1: *const ::core::ffi::c_char,
    end1: *const ::core::ffi::c_char,
    mut ptr2: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    while read_c_char_bytes::<1>(ptr2)[0] != 0 {
        if ptr1 == end1 {
            return 0;
        }
        if read_c_char_bytes::<1>(ptr1)[0] as ::core::ffi::c_int
            != read_c_char_bytes::<1>(ptr2)[0] as ::core::ffi::c_int
        {
            return 0;
        }
        ptr1 = ptr1.wrapping_add(1);
        ptr2 = ptr2.wrapping_add(1);
    }
    (ptr1 == end1) as ::core::ffi::c_int
}
extern "C" fn normal_nameLength(
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let start = ptr;
    loop {
        match normal_byte_type(enc, ptr) {
            5 => ptr = ptr.wrapping_add(2),
            6 => ptr = ptr.wrapping_add(3),
            7 => ptr = ptr.wrapping_add(4),
            29 | 22 | 23 | 24 | 25 | 26 | 27 => ptr = ptr.wrapping_add(1),
            _ => return c_char_distance(start, ptr),
        }
    }
}
extern "C" fn normal_skipS(
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    loop {
        match normal_byte_type(enc, ptr) {
            10 | 9 | 21 => ptr = ptr.wrapping_add(1),
            _ => return ptr,
        }
    }
}
extern "C" fn normal_updatePosition(
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    pos: *mut POSITION,
) {
    while remaining_const_c_chars(ptr, end) >= 1 {
        match normal_byte_type(enc, ptr) {
            5 => {
                ptr = ptr.wrapping_add(2);
                increment_column_number(pos);
            }
            6 => {
                ptr = ptr.wrapping_add(3);
                increment_column_number(pos);
            }
            7 => {
                ptr = ptr.wrapping_add(4);
                increment_column_number(pos);
            }
            10 => {
                set_column_number(pos, 0 as XML_Size);
                increment_line_number(pos);
                ptr = ptr.wrapping_add(1);
            }
            9 => {
                increment_line_number(pos);
                ptr = ptr.wrapping_add(1);
                if remaining_const_c_chars(ptr, end) >= 1
                    && normal_byte_type(enc, ptr) == BT_LF as ::core::ffi::c_int
                {
                    ptr = ptr.wrapping_add(1);
                }
                set_column_number(pos, 0 as XML_Size);
            }
            _ => {
                ptr = ptr.wrapping_add(1);
                increment_column_number(pos);
            }
        }
    }
}
extern "C" fn little2_scanComment(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if has_c_chars(ptr, end, 2) {
        if !little2_is_ascii(ptr, b'-') {
            set_const_c_char_ptr(nextTokPtr, ptr);
            return XML_TOK_INVALID;
        }
        ptr = add_const_c_char(ptr, 2);
        while has_c_chars(ptr, end, 2) {
            match little2_byte_type(enc, ptr) {
                value if value == BT_LEAD2 as ::core::ffi::c_int => {
                    if !has_c_chars(ptr, end, 2) {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = add_const_c_char(ptr, 2);
                }
                value if value == BT_LEAD3 as ::core::ffi::c_int => {
                    if !has_c_chars(ptr, end, 3) {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = add_const_c_char(ptr, 3);
                }
                value if value == BT_LEAD4 as ::core::ffi::c_int => {
                    if !has_c_chars(ptr, end, 4) {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = add_const_c_char(ptr, 4);
                }
                value
                    if value == BT_NONXML as ::core::ffi::c_int
                        || value == BT_MALFORM as ::core::ffi::c_int
                        || value == BT_TRAIL as ::core::ffi::c_int =>
                {
                    set_const_c_char_ptr(nextTokPtr, ptr);
                    return XML_TOK_INVALID;
                }
                value if value == BT_MINUS as ::core::ffi::c_int => {
                    ptr = add_const_c_char(ptr, 2);
                    if !has_c_chars(ptr, end, 2) {
                        return XML_TOK_PARTIAL;
                    }
                    if little2_is_ascii(ptr, b'-') {
                        ptr = add_const_c_char(ptr, 2);
                        if !has_c_chars(ptr, end, 2) {
                            return XML_TOK_PARTIAL;
                        }
                        if !little2_is_ascii(ptr, b'>') {
                            set_const_c_char_ptr(nextTokPtr, ptr);
                            return XML_TOK_INVALID;
                        }
                        set_const_c_char_ptr(nextTokPtr, add_const_c_char(ptr, 2));
                        return XML_TOK_COMMENT;
                    }
                }
                _ => {
                    ptr = add_const_c_char(ptr, 2);
                }
            }
        }
    }
    XML_TOK_PARTIAL
}
extern "C" fn little2_scanDecl(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    scan_decl_with(
        enc,
        ptr,
        end,
        nextTokPtr,
        2,
        little2_byte_type,
        little2_scanComment,
    )
}
extern "C" fn little2_checkPiTarget(
    _enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    tokPtr: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    check_pi_target_with(ptr, end, tokPtr, 2, read_little2_ascii_unit)
}
extern "C" fn little2_scanPi(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    scan_utf16_pi_with(
        enc,
        ptr,
        end,
        nextTokPtr,
        little2_byte_type,
        little2_is_name_start,
        little2_is_name_char,
        little2_checkPiTarget,
        little2_is_ascii,
    )
}

extern "C" fn little2_scanCdataSection(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let _ = enc;
    scan_cdata_section_open_with(ptr, end, nextTokPtr, 2, read_little2_ascii_unit)
}
extern "C" fn little2_cdataSectionTok(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    cdata_section_tok_wide(
        enc,
        ptr,
        end,
        nextTokPtr,
        little2_byte_type,
        little2_is_ascii,
    )
}
extern "C" fn little2_scanEndTag(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL;
        }
        let mut c2rust_current_block_32: u64;
        match if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1 as ::core::ffi::c_int as isize),
                *ptr.offset(0 as ::core::ffi::c_int as isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages
                    [*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3 as ::core::ffi::c_int)
                    + (*ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5 as ::core::ffi::c_int)) as usize]
                    & (1 as ::core::ffi::c_uint)
                        << (*ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1f as ::core::ffi::c_int)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                c2rust_current_block_32 = 8654814784450400207;
            }
            22 | 24 => {
                c2rust_current_block_32 = 8654814784450400207;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match c2rust_current_block_32 {
            8654814784450400207 => {
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_73: u64;
            match if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1 as ::core::ffi::c_int as isize),
                    *ptr.offset(0 as ::core::ffi::c_int as isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3 as ::core::ffi::c_int)
                        + (*ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5 as ::core::ffi::c_int))
                        as usize]
                        & (1 as ::core::ffi::c_uint)
                            << (*ptr.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1f as ::core::ffi::c_int)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    c2rust_current_block_73 = 16411184819389759620;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_73 = 16411184819389759620;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    c2rust_current_block_73 = 981995395831942902;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                    c2rust_current_block_73 = 981995395831942902;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    c2rust_current_block_73 = 981995395831942902;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    while end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long
                    {
                        match if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1 as ::core::ffi::c_int as isize),
                                *ptr.offset(0 as ::core::ffi::c_int as isize),
                            )
                        } {
                            21 | 9 | 10 => {}
                            11 => {
                                *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                return XML_TOK_END_TAG;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    }
                    return XML_TOK_PARTIAL;
                }
                23 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    c2rust_current_block_73 = 981995395831942902;
                }
                11 => {
                    *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    return XML_TOK_END_TAG;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match c2rust_current_block_73 {
                16411184819389759620 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }
}
extern "C" fn little2_scanHexCharRef(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    scan_hex_char_ref_with(enc, ptr, end, nextTokPtr, 2, little2_byte_type)
}
extern "C" fn little2_scanCharRef(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    scan_char_ref_with(
        enc,
        ptr,
        end,
        nextTokPtr,
        2,
        read_little2_ascii_unit,
        little2_byte_type,
    )
}
extern "C" fn little2_scanRef(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    scan_ref_with(
        enc,
        ptr,
        end,
        nextTokPtr,
        2,
        little2_byte_type,
        little2_is_name_start,
        little2_is_name_char,
        reject_multibyte_name_sequence,
        reject_multibyte_name_sequence,
        little2_scanCharRef,
    )
}
extern "C" fn little2_scanAtts(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut hadColon: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_186: u64;
            match if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1 as ::core::ffi::c_int as isize),
                    *ptr.offset(0 as ::core::ffi::c_int as isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3 as ::core::ffi::c_int)
                        + (*ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5 as ::core::ffi::c_int))
                        as usize]
                        & (1 as ::core::ffi::c_uint)
                            << (*ptr.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1f as ::core::ffi::c_int)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    c2rust_current_block_186 = 17747718632989559416;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_186 = 17747718632989559416;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    c2rust_current_block_186 = 1634947208139838470;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                    c2rust_current_block_186 = 1634947208139838470;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    c2rust_current_block_186 = 1634947208139838470;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    hadColon = 1 as ::core::ffi::c_int;
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    let mut c2rust_current_block_64: u64;
                    match if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(1 as ::core::ffi::c_int as isize),
                            *ptr.offset(0 as ::core::ffi::c_int as isize),
                        )
                    } {
                        29 => {
                            if namingBitmap[(((nmstrtPages[*ptr
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar
                                as usize]
                                as ::core::ffi::c_int)
                                << 3 as ::core::ffi::c_int)
                                + (*ptr.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uchar
                                    as ::core::ffi::c_int
                                    >> 5 as ::core::ffi::c_int))
                                as usize]
                                & (1 as ::core::ffi::c_uint)
                                    << (*ptr.offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uchar
                                        as ::core::ffi::c_int
                                        & 0x1f as ::core::ffi::c_int)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            c2rust_current_block_64 = 12531724302225488581;
                        }
                        22 | 24 => {
                            c2rust_current_block_64 = 12531724302225488581;
                        }
                        5 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 2 as ::core::ffi::c_long
                            {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            c2rust_current_block_64 = 10930818133215224067;
                        }
                        6 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 3 as ::core::ffi::c_long
                            {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                            c2rust_current_block_64 = 10930818133215224067;
                        }
                        7 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 4 as ::core::ffi::c_long
                            {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                            c2rust_current_block_64 = 10930818133215224067;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                    match c2rust_current_block_64 {
                        12531724302225488581 => {
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        }
                        _ => {}
                    }
                    c2rust_current_block_186 = 1634947208139838470;
                }
                21 | 9 | 10 => {
                    loop {
                        let mut t: ::core::ffi::c_int = 0;
                        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        t = if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1 as ::core::ffi::c_int as isize),
                                *ptr.offset(0 as ::core::ffi::c_int as isize),
                            )
                        };
                        if t == BT_EQUALS as ::core::ffi::c_int {
                            break;
                        }
                        match t {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                    }
                    c2rust_current_block_186 = 10853015579903106591;
                }
                14 => {
                    c2rust_current_block_186 = 10853015579903106591;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match c2rust_current_block_186 {
                10853015579903106591 => {
                    let mut open: ::core::ffi::c_int = 0;
                    hadColon = 0 as ::core::ffi::c_int;
                    loop {
                        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        open = if *ptr.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1 as ::core::ffi::c_int as isize),
                                *ptr.offset(0 as ::core::ffi::c_int as isize),
                            )
                        };
                        if open == BT_QUOT as ::core::ffi::c_int
                            || open == BT_APOS as ::core::ffi::c_int
                        {
                            break;
                        }
                        match open {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    loop {
                        let mut t_0: ::core::ffi::c_int = 0;
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        t_0 = if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1 as ::core::ffi::c_int as isize),
                                *ptr.offset(0 as ::core::ffi::c_int as isize),
                            )
                        };
                        if t_0 == open {
                            break;
                        }
                        match t_0 {
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 2 as ::core::ffi::c_long
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 3 as ::core::ffi::c_long
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 4 as ::core::ffi::c_long
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                            }
                            0 | 1 | 8 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            3 => {
                                let mut tok: ::core::ffi::c_int = little2_scanRef(
                                    enc,
                                    ptr.offset(2 as ::core::ffi::c_int as isize),
                                    end,
                                    &raw mut ptr,
                                );
                                if tok <= 0 as ::core::ffi::c_int {
                                    if tok == XML_TOK_INVALID {
                                        *nextTokPtr = ptr;
                                    }
                                    return tok;
                                }
                            }
                            2 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            _ => {
                                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            }
                        }
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    match if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(1 as ::core::ffi::c_int as isize),
                            *ptr.offset(0 as ::core::ffi::c_int as isize),
                        )
                    } {
                        21 | 9 | 10 => {
                            loop {
                                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                        as ::core::ffi::c_long)
                                {
                                    return XML_TOK_PARTIAL;
                                }
                                match if *ptr.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                {
                                    (*(enc as *const normal_encoding)).type_0
                                        [*ptr as ::core::ffi::c_uchar as usize]
                                        as ::core::ffi::c_int
                                } else {
                                    unicode_byte_type(
                                        *ptr.offset(1 as ::core::ffi::c_int as isize),
                                        *ptr.offset(0 as ::core::ffi::c_int as isize),
                                    )
                                } {
                                    29 => {
                                        if namingBitmap[(((nmstrtPages[*ptr
                                            .offset(1 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uchar
                                            as usize]
                                            as ::core::ffi::c_int)
                                            << 3 as ::core::ffi::c_int)
                                            + (*ptr.offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_uchar
                                                as ::core::ffi::c_int
                                                >> 5 as ::core::ffi::c_int))
                                            as usize]
                                            & (1 as ::core::ffi::c_uint)
                                                << (*ptr.offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uchar
                                                    as ::core::ffi::c_int
                                                    & 0x1f as ::core::ffi::c_int)
                                            == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        c2rust_current_block_186 = 923465642386550266;
                                        break;
                                    }
                                    22 | 24 => {
                                        c2rust_current_block_186 = 923465642386550266;
                                        break;
                                    }
                                    5 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long)
                                            < 2 as ::core::ffi::c_long
                                        {
                                            return XML_TOK_PARTIAL_CHAR;
                                        }
                                        if 0 as ::core::ffi::c_int != 0
                                            || 0 as ::core::ffi::c_int == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                        c2rust_current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    6 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long)
                                            < 3 as ::core::ffi::c_long
                                        {
                                            return XML_TOK_PARTIAL_CHAR;
                                        }
                                        if 0 as ::core::ffi::c_int != 0
                                            || 0 as ::core::ffi::c_int == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                                        c2rust_current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    7 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long)
                                            < 4 as ::core::ffi::c_long
                                        {
                                            return XML_TOK_PARTIAL_CHAR;
                                        }
                                        if 0 as ::core::ffi::c_int != 0
                                            || 0 as ::core::ffi::c_int == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                                        c2rust_current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    21 | 9 | 10 => {}
                                    11 => {
                                        c2rust_current_block_186 = 15103464935601583148;
                                        break;
                                    }
                                    17 => {
                                        c2rust_current_block_186 = 619033562305054167;
                                        break;
                                    }
                                    _ => {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                }
                            }
                            match c2rust_current_block_186 {
                                15103464935601583148 => {}
                                619033562305054167 => {}
                                1634947208139838470 => {}
                                _ => {
                                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                    c2rust_current_block_186 = 1634947208139838470;
                                }
                            }
                        }
                        17 => {
                            c2rust_current_block_186 = 619033562305054167;
                        }
                        11 => {
                            c2rust_current_block_186 = 15103464935601583148;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                    match c2rust_current_block_186 {
                        1634947208139838470 => {}
                        _ => match c2rust_current_block_186 {
                            619033562305054167 => {
                                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                        as ::core::ffi::c_long)
                                {
                                    return XML_TOK_PARTIAL;
                                }
                                if !(*ptr.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                    && *ptr.offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == 0x3e as ::core::ffi::c_int)
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                return XML_TOK_EMPTY_ELEMENT_WITH_ATTS;
                            }
                            _ => {
                                *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                return XML_TOK_START_TAG_WITH_ATTS;
                            }
                        },
                    }
                }
                17747718632989559416 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }
}
extern "C" fn little2_scanLt(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut hadColon: ::core::ffi::c_int = 0;
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL;
        }
        let mut c2rust_current_block_45: u64;
        match if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1 as ::core::ffi::c_int as isize),
                *ptr.offset(0 as ::core::ffi::c_int as isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages
                    [*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3 as ::core::ffi::c_int)
                    + (*ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5 as ::core::ffi::c_int)) as usize]
                    & (1 as ::core::ffi::c_uint)
                        << (*ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1f as ::core::ffi::c_int)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                c2rust_current_block_45 = 18046087305847344724;
            }
            22 | 24 => {
                c2rust_current_block_45 = 18046087305847344724;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                c2rust_current_block_45 = 8180496224585318153;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                c2rust_current_block_45 = 8180496224585318153;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                c2rust_current_block_45 = 8180496224585318153;
            }
            16 => {
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long)
                {
                    return XML_TOK_PARTIAL;
                }
                match if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(1 as ::core::ffi::c_int as isize),
                        *ptr.offset(0 as ::core::ffi::c_int as isize),
                    )
                } {
                    27 => {
                        return little2_scanComment(
                            enc,
                            ptr.offset(2 as ::core::ffi::c_int as isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    20 => {
                        return little2_scanCdataSection(
                            enc,
                            ptr.offset(2 as ::core::ffi::c_int as isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            15 => {
                return little2_scanPi(
                    enc,
                    ptr.offset(2 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            17 => {
                return little2_scanEndTag(
                    enc,
                    ptr.offset(2 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match c2rust_current_block_45 {
            18046087305847344724 => {
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            }
            _ => {}
        }
        hadColon = 0 as ::core::ffi::c_int;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_161: u64;
            match if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1 as ::core::ffi::c_int as isize),
                    *ptr.offset(0 as ::core::ffi::c_int as isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3 as ::core::ffi::c_int)
                        + (*ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5 as ::core::ffi::c_int))
                        as usize]
                        & (1 as ::core::ffi::c_uint)
                            << (*ptr.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1f as ::core::ffi::c_int)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    c2rust_current_block_161 = 8998928240368606981;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_161 = 8998928240368606981;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    c2rust_current_block_161 = 14714495436747744489;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                    c2rust_current_block_161 = 14714495436747744489;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    c2rust_current_block_161 = 14714495436747744489;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    hadColon = 1 as ::core::ffi::c_int;
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    let mut c2rust_current_block_112: u64;
                    match if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(1 as ::core::ffi::c_int as isize),
                            *ptr.offset(0 as ::core::ffi::c_int as isize),
                        )
                    } {
                        29 => {
                            if namingBitmap[(((nmstrtPages[*ptr
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar
                                as usize]
                                as ::core::ffi::c_int)
                                << 3 as ::core::ffi::c_int)
                                + (*ptr.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uchar
                                    as ::core::ffi::c_int
                                    >> 5 as ::core::ffi::c_int))
                                as usize]
                                & (1 as ::core::ffi::c_uint)
                                    << (*ptr.offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uchar
                                        as ::core::ffi::c_int
                                        & 0x1f as ::core::ffi::c_int)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            c2rust_current_block_112 = 14391208795021697965;
                        }
                        22 | 24 => {
                            c2rust_current_block_112 = 14391208795021697965;
                        }
                        5 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 2 as ::core::ffi::c_long
                            {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            c2rust_current_block_112 = 2616667235040759262;
                        }
                        6 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 3 as ::core::ffi::c_long
                            {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                            c2rust_current_block_112 = 2616667235040759262;
                        }
                        7 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 4 as ::core::ffi::c_long
                            {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                            c2rust_current_block_112 = 2616667235040759262;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                    match c2rust_current_block_112 {
                        14391208795021697965 => {
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        }
                        _ => {}
                    }
                    c2rust_current_block_161 = 14714495436747744489;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    loop {
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            c2rust_current_block_161 = 13215501469961642988;
                            break;
                        }
                        match if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1 as ::core::ffi::c_int as isize),
                                *ptr.offset(0 as ::core::ffi::c_int as isize),
                            )
                        } {
                            29 => {
                                if namingBitmap[(((nmstrtPages[*ptr
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uchar
                                    as usize]
                                    as ::core::ffi::c_int)
                                    << 3 as ::core::ffi::c_int)
                                    + (*ptr.offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uchar
                                        as ::core::ffi::c_int
                                        >> 5 as ::core::ffi::c_int))
                                    as usize]
                                    & (1 as ::core::ffi::c_uint)
                                        << (*ptr.offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uchar
                                            as ::core::ffi::c_int
                                            & 0x1f as ::core::ffi::c_int)
                                    == 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                c2rust_current_block_161 = 2369392326157537288;
                            }
                            22 | 24 => {
                                c2rust_current_block_161 = 2369392326157537288;
                            }
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 2 as ::core::ffi::c_long
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                c2rust_current_block_161 = 16314074004867283505;
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 3 as ::core::ffi::c_long
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                                c2rust_current_block_161 = 16314074004867283505;
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 4 as ::core::ffi::c_long
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                                c2rust_current_block_161 = 16314074004867283505;
                            }
                            11 => {
                                c2rust_current_block_161 = 1918622160084604696;
                                break;
                            }
                            17 => {
                                c2rust_current_block_161 = 1114269873380682160;
                                break;
                            }
                            21 | 9 | 10 => {
                                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                continue;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                        match c2rust_current_block_161 {
                            2369392326157537288 => {
                                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            }
                            _ => {}
                        }
                        return little2_scanAtts(enc, ptr, end, nextTokPtr);
                    }
                    match c2rust_current_block_161 {
                        1918622160084604696 => {}
                        1114269873380682160 => {}
                        _ => return XML_TOK_PARTIAL,
                    }
                }
                11 => {
                    c2rust_current_block_161 = 1918622160084604696;
                }
                17 => {
                    c2rust_current_block_161 = 1114269873380682160;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match c2rust_current_block_161 {
                1114269873380682160 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if !(*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        && *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0x3e as ::core::ffi::c_int)
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    return XML_TOK_EMPTY_ELEMENT_NO_ATTS;
                }
                1918622160084604696 => {
                    *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    return XML_TOK_START_TAG_NO_ATTS;
                }
                8998928240368606981 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }
}
extern "C" fn little2_contentTok(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if ptr >= end {
            return XML_TOK_NONE;
        }
        if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            let mut n: size_t = end.offset_from(ptr) as ::core::ffi::c_long as size_t;
            if n & (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t != 0 {
                n &= !(2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t;
                if n == 0 as size_t {
                    return XML_TOK_PARTIAL;
                }
                end = ptr.offset(n as isize);
            }
        }
        match if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1 as ::core::ffi::c_int as isize),
                *ptr.offset(0 as ::core::ffi::c_int as isize),
            )
        } {
            2 => {
                return little2_scanLt(
                    enc,
                    ptr.offset(2 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            3 => {
                return little2_scanRef(
                    enc,
                    ptr.offset(2 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            9 => {
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long)
                {
                    return XML_TOK_TRAILING_CR;
                }
                if (if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(1 as ::core::ffi::c_int as isize),
                        *ptr.offset(0 as ::core::ffi::c_int as isize),
                    )
                }) == BT_LF as ::core::ffi::c_int
                {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                *nextTokPtr = ptr;
                return XML_TOK_DATA_NEWLINE;
            }
            10 => {
                *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                return XML_TOK_DATA_NEWLINE;
            }
            4 => {
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long)
                {
                    return XML_TOK_TRAILING_RSQB;
                }
                if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                    && *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0x5d as ::core::ffi::c_int
                {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_TRAILING_RSQB;
                    }
                    if !(*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        && *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0x3e as ::core::ffi::c_int)
                    {
                        ptr = ptr.offset(-(2 as ::core::ffi::c_int as isize));
                    } else {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
            }
            0 | 1 | 8 => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            _ => {
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_76: u64;
            match if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1 as ::core::ffi::c_int as isize),
                    *ptr.offset(0 as ::core::ffi::c_int as isize),
                )
            } {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long
                        || 0 as ::core::ffi::c_int != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    c2rust_current_block_76 = 7158658067966855297;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long
                        || 0 as ::core::ffi::c_int != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                    c2rust_current_block_76 = 7158658067966855297;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long
                        || 0 as ::core::ffi::c_int != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    c2rust_current_block_76 = 7158658067966855297;
                }
                4 => {
                    if end.offset_from(ptr) as ::core::ffi::c_long
                        >= (2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long
                    {
                        if !(*ptr
                            .offset(2 as ::core::ffi::c_int as isize)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                            && *ptr
                                .offset(2 as ::core::ffi::c_int as isize)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == 0x5d as ::core::ffi::c_int)
                        {
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            c2rust_current_block_76 = 7158658067966855297;
                        } else if end.offset_from(ptr) as ::core::ffi::c_long
                            >= (3 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as ::core::ffi::c_long
                        {
                            if !(*ptr
                                .offset(
                                    (2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                )
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                                && *ptr
                                    .offset(
                                        (2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    )
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == 0x3e as ::core::ffi::c_int)
                            {
                                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            } else {
                                *nextTokPtr = ptr.offset(
                                    (2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                );
                                return XML_TOK_INVALID;
                            }
                            c2rust_current_block_76 = 7158658067966855297;
                        } else {
                            c2rust_current_block_76 = 17804070343020517427;
                        }
                    } else {
                        c2rust_current_block_76 = 17804070343020517427;
                    }
                }
                3 | 2 | 0 | 1 | 8 | 9 | 10 => {
                    c2rust_current_block_76 = 17804070343020517427;
                }
                _ => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    c2rust_current_block_76 = 7158658067966855297;
                }
            }
            match c2rust_current_block_76 {
                7158658067966855297 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }
}
extern "C" fn little2_scanPercent(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    scan_utf16_name_token_with(
        enc,
        ptr,
        end,
        nextTokPtr,
        little2_byte_type,
        little2_is_name_start,
        little2_is_name_char,
        &[
            BT_S as ::core::ffi::c_int,
            BT_LF as ::core::ffi::c_int,
            BT_CR as ::core::ffi::c_int,
            BT_PERCNT as ::core::ffi::c_int,
        ],
        &[BT_SEMI as ::core::ffi::c_int],
        XML_TOK_PERCENT,
        XML_TOK_PARAM_ENTITY_REF,
        2,
        XML_TOK_PARTIAL,
    )
}

extern "C" fn little2_scanPoundName(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    scan_utf16_name_token_with(
        enc,
        ptr,
        end,
        nextTokPtr,
        little2_byte_type,
        little2_is_name_start,
        little2_is_name_char,
        &[],
        &[
            BT_CR as ::core::ffi::c_int,
            BT_LF as ::core::ffi::c_int,
            BT_S as ::core::ffi::c_int,
            BT_RPAR as ::core::ffi::c_int,
            BT_GT as ::core::ffi::c_int,
            BT_PERCNT as ::core::ffi::c_int,
            BT_VERBAR as ::core::ffi::c_int,
        ],
        XML_TOK_POUND_NAME,
        XML_TOK_POUND_NAME,
        0,
        -XML_TOK_POUND_NAME,
    )
}

extern "C" fn little2_scanLit(
    mut open: ::core::ffi::c_int,
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    scan_lit_with(
        open,
        enc,
        ptr,
        end,
        nextTokPtr,
        2,
        little2_byte_type,
        allow_literal_multibyte_sequence,
    )
}
extern "C" fn little2_prologTok(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut tok: ::core::ffi::c_int = 0;
        if ptr >= end {
            return XML_TOK_NONE;
        }
        if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            let mut n: size_t = end.offset_from(ptr) as ::core::ffi::c_long as size_t;
            if n & (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t != 0 {
                n &= !(2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t;
                if n == 0 as size_t {
                    return XML_TOK_PARTIAL;
                }
                end = ptr.offset(n as isize);
            }
        }
        let mut c2rust_current_block_124: u64;
        match if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1 as ::core::ffi::c_int as isize),
                *ptr.offset(0 as ::core::ffi::c_int as isize),
            )
        } {
            12 => {
                return little2_scanLit(
                    BT_QUOT as ::core::ffi::c_int,
                    enc,
                    ptr.offset(2 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            13 => {
                return little2_scanLit(
                    BT_APOS as ::core::ffi::c_int,
                    enc,
                    ptr.offset(2 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            2 => {
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long)
                {
                    return XML_TOK_PARTIAL;
                }
                match if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(1 as ::core::ffi::c_int as isize),
                        *ptr.offset(0 as ::core::ffi::c_int as isize),
                    )
                } {
                    16 => {
                        return little2_scanDecl(
                            enc,
                            ptr.offset(2 as ::core::ffi::c_int as isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    15 => {
                        return little2_scanPi(
                            enc,
                            ptr.offset(2 as ::core::ffi::c_int as isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    22 | 24 | 29 | 5 | 6 | 7 => {
                        *nextTokPtr = ptr.offset(-(2 as ::core::ffi::c_int as isize));
                        return XML_TOK_INSTANCE_START;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            9 => {
                if ptr.offset(2 as ::core::ffi::c_int as isize) == end {
                    *nextTokPtr = end;
                    return -XML_TOK_PROLOG_S;
                }
                c2rust_current_block_124 = 17513858719706519675;
            }
            21 | 10 => {
                c2rust_current_block_124 = 17513858719706519675;
            }
            30 => {
                return little2_scanPercent(
                    enc,
                    ptr.offset(2 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            35 => {
                *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                return XML_TOK_COMMA;
            }
            20 => {
                *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                return XML_TOK_OPEN_BRACKET;
            }
            4 => {
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long)
                {
                    return -XML_TOK_CLOSE_BRACKET;
                }
                if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                    && *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0x5d as ::core::ffi::c_int
                {
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if *ptr
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        && *ptr
                            .offset(2 as ::core::ffi::c_int as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == 0x3e as ::core::ffi::c_int
                    {
                        *nextTokPtr = ptr
                            .offset((2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize);
                        return XML_TOK_COND_SECT_CLOSE;
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_CLOSE_BRACKET;
            }
            31 => {
                *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                return XML_TOK_OPEN_PAREN;
            }
            32 => {
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long)
                {
                    return -XML_TOK_CLOSE_PAREN;
                }
                match if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(1 as ::core::ffi::c_int as isize),
                        *ptr.offset(0 as ::core::ffi::c_int as isize),
                    )
                } {
                    33 => {
                        *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        return XML_TOK_CLOSE_PAREN_ASTERISK;
                    }
                    15 => {
                        *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        return XML_TOK_CLOSE_PAREN_QUESTION;
                    }
                    34 => {
                        *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        return XML_TOK_CLOSE_PAREN_PLUS;
                    }
                    9 | 10 | 21 | 11 | 35 | 36 | 32 => {
                        *nextTokPtr = ptr;
                        return XML_TOK_CLOSE_PAREN;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            36 => {
                *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                return XML_TOK_OR;
            }
            11 => {
                *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                return XML_TOK_DECL_CLOSE;
            }
            19 => {
                return little2_scanPoundName(
                    enc,
                    ptr.offset(2 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            22 | 24 => {
                tok = XML_TOK_NAME;
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                c2rust_current_block_124 = 2956972668325154207;
            }
            25 | 26 | 27 | 23 => {
                tok = XML_TOK_NMTOKEN;
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                c2rust_current_block_124 = 2956972668325154207;
            }
            29 => {
                if namingBitmap[(((nmstrtPages
                    [*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3 as ::core::ffi::c_int)
                    + (*ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5 as ::core::ffi::c_int)) as usize]
                    & (1 as ::core::ffi::c_uint)
                        << (*ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1f as ::core::ffi::c_int)
                    != 0
                {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    tok = XML_TOK_NAME;
                    c2rust_current_block_124 = 2956972668325154207;
                } else if namingBitmap[(((namePages
                    [*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3 as ::core::ffi::c_int)
                    + (*ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5 as ::core::ffi::c_int))
                    as usize]
                    & (1 as ::core::ffi::c_uint)
                        << (*ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1f as ::core::ffi::c_int)
                    != 0
                {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    tok = XML_TOK_NMTOKEN;
                    c2rust_current_block_124 = 2956972668325154207;
                } else {
                    c2rust_current_block_124 = 2543942683527618915;
                }
            }
            _ => {
                c2rust_current_block_124 = 2543942683527618915;
            }
        }
        match c2rust_current_block_124 {
            2956972668325154207 => {}
            17513858719706519675 => {
                loop {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        break;
                    }
                    let mut c2rust_current_block_32: u64;
                    match if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(1 as ::core::ffi::c_int as isize),
                            *ptr.offset(0 as ::core::ffi::c_int as isize),
                        )
                    } {
                        21 | 10 => {
                            c2rust_current_block_32 = 17500079516916021833;
                        }
                        9 => {
                            if ptr.offset(2 as ::core::ffi::c_int as isize) != end {
                                c2rust_current_block_32 = 17500079516916021833;
                            } else {
                                c2rust_current_block_32 = 17471725448347076649;
                            }
                        }
                        _ => {
                            c2rust_current_block_32 = 17471725448347076649;
                        }
                    }
                    match c2rust_current_block_32 {
                        17500079516916021833 => {}
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_PROLOG_S;
                        }
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_PROLOG_S;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_210: u64;
            match if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1 as ::core::ffi::c_int as isize),
                    *ptr.offset(0 as ::core::ffi::c_int as isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3 as ::core::ffi::c_int)
                        + (*ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5 as ::core::ffi::c_int))
                        as usize]
                        & (1 as ::core::ffi::c_uint)
                            << (*ptr.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1f as ::core::ffi::c_int)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    c2rust_current_block_210 = 786388639404123072;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_210 = 786388639404123072;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    c2rust_current_block_210 = 14244298717249035578;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                    c2rust_current_block_210 = 14244298717249035578;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    c2rust_current_block_210 = 14244298717249035578;
                }
                11 | 32 | 35 | 36 | 20 | 30 | 21 | 9 | 10 => {
                    *nextTokPtr = ptr;
                    return tok;
                }
                23 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    match tok {
                        XML_TOK_NAME => {
                            if !(end.offset_from(ptr) as ::core::ffi::c_long
                                >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                    as ::core::ffi::c_long)
                            {
                                return XML_TOK_PARTIAL;
                            }
                            tok = XML_TOK_PREFIXED_NAME;
                            let mut c2rust_current_block_187: u64;
                            match if *ptr.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                (*(enc as *const normal_encoding)).type_0
                                    [*ptr as ::core::ffi::c_uchar as usize]
                                    as ::core::ffi::c_int
                            } else {
                                unicode_byte_type(
                                    *ptr.offset(1 as ::core::ffi::c_int as isize),
                                    *ptr.offset(0 as ::core::ffi::c_int as isize),
                                )
                            } {
                                29 => {
                                    if namingBitmap[(((namePages[*ptr
                                        .offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uchar
                                        as usize]
                                        as ::core::ffi::c_int)
                                        << 3 as ::core::ffi::c_int)
                                        + (*ptr.offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uchar
                                            as ::core::ffi::c_int
                                            >> 5 as ::core::ffi::c_int))
                                        as usize]
                                        & (1 as ::core::ffi::c_uint)
                                            << (*ptr.offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_uchar
                                                as ::core::ffi::c_int
                                                & 0x1f as ::core::ffi::c_int)
                                        == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    c2rust_current_block_187 = 16869951820887225088;
                                }
                                22 | 24 | 25 | 26 | 27 => {
                                    c2rust_current_block_187 = 16869951820887225088;
                                }
                                5 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long)
                                        < 2 as ::core::ffi::c_long
                                    {
                                        return XML_TOK_PARTIAL_CHAR;
                                    }
                                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                                6 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long)
                                        < 3 as ::core::ffi::c_long
                                    {
                                        return XML_TOK_PARTIAL_CHAR;
                                    }
                                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                                7 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long)
                                        < 4 as ::core::ffi::c_long
                                    {
                                        return XML_TOK_PARTIAL_CHAR;
                                    }
                                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                                _ => {
                                    tok = XML_TOK_NMTOKEN;
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                            }
                            match c2rust_current_block_187 {
                                16869951820887225088 => {
                                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                }
                                _ => {}
                            }
                        }
                        XML_TOK_PREFIXED_NAME => {
                            tok = XML_TOK_NMTOKEN;
                        }
                        _ => {}
                    }
                    c2rust_current_block_210 = 14244298717249035578;
                }
                34 => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    return XML_TOK_NAME_PLUS;
                }
                33 => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    return XML_TOK_NAME_ASTERISK;
                }
                15 => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    return XML_TOK_NAME_QUESTION;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match c2rust_current_block_210 {
                786388639404123072 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                _ => {}
            }
        }
        return -tok;
    }
}
extern "C" fn little2_attributeValueTok(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if ptr >= end {
            return XML_TOK_NONE;
        } else if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL;
        }
        start = ptr;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            match if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1 as ::core::ffi::c_int as isize),
                    *ptr.offset(0 as ::core::ffi::c_int as isize),
                )
            } {
                5 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                6 => {
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                }
                7 => {
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                }
                3 => {
                    if ptr == start {
                        return little2_scanRef(
                            enc,
                            ptr.offset(2 as ::core::ffi::c_int as isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                2 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_TRAILING_CR;
                        }
                        if (if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1 as ::core::ffi::c_int as isize),
                                *ptr.offset(0 as ::core::ffi::c_int as isize),
                            )
                        }) == BT_LF as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        }
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                21 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        return XML_TOK_ATTRIBUTE_VALUE_S;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                _ => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }
}
extern "C" fn little2_entityValueTok(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if ptr >= end {
            return XML_TOK_NONE;
        } else if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL;
        }
        start = ptr;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            match if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1 as ::core::ffi::c_int as isize),
                    *ptr.offset(0 as ::core::ffi::c_int as isize),
                )
            } {
                5 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                6 => {
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                }
                7 => {
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                }
                3 => {
                    if ptr == start {
                        return little2_scanRef(
                            enc,
                            ptr.offset(2 as ::core::ffi::c_int as isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                30 => {
                    if ptr == start {
                        let mut tok: ::core::ffi::c_int = little2_scanPercent(
                            enc,
                            ptr.offset(2 as ::core::ffi::c_int as isize),
                            end,
                            nextTokPtr,
                        );
                        return if tok == XML_TOK_PERCENT {
                            XML_TOK_INVALID
                        } else {
                            tok
                        };
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_TRAILING_CR;
                        }
                        if (if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1 as ::core::ffi::c_int as isize),
                                *ptr.offset(0 as ::core::ffi::c_int as isize),
                            )
                        }) == BT_LF as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        }
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                _ => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }
}
extern "C" fn little2_ignoreSectionTok(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut level: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            let mut n: size_t = end.offset_from(ptr) as ::core::ffi::c_long as size_t;
            if n & (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t != 0 {
                n &= !(2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t;
                end = ptr.offset(n as isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            match if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1 as ::core::ffi::c_int as isize),
                    *ptr.offset(0 as ::core::ffi::c_int as isize),
                )
            } {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                2 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        && *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0x21 as ::core::ffi::c_int
                    {
                        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                            && *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                == 0x5b as ::core::ffi::c_int
                        {
                            level += 1;
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        }
                    }
                }
                4 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        && *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0x5d as ::core::ffi::c_int
                    {
                        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        if *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                            && *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                == 0x3e as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            if level == 0 as ::core::ffi::c_int {
                                *nextTokPtr = ptr;
                                return XML_TOK_IGNORE_SECT;
                            }
                            level -= 1;
                        }
                    }
                }
                _ => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
            }
        }
        return XML_TOK_PARTIAL;
    }
}
extern "C" fn little2_isPublicId(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    badPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let ptr = add_const_c_char(ptr, 2);
    let end = add_const_c_char(end, -2);
    validate_public_id(
        ptr,
        end,
        2,
        badPtr,
        |current| {
            let low = read_c_char(current);
            let high = read_c_char(add_const_c_char(current, 1));
            if high == 0 {
                with_ref(enc.cast::<normal_encoding>(), |normal| {
                    normal.type_0[low as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
                })
            } else {
                unicode_byte_type(high, low)
            }
        },
        |current| {
            let low = read_c_char(current) as ::core::ffi::c_int;
            let high = read_c_char(add_const_c_char(current, 1)) as ::core::ffi::c_int;
            if high == 0 {
                low
            } else {
                -(1 as ::core::ffi::c_int)
            }
        },
    )
}
extern "C" fn little2_getAtts(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut attsMax: ::core::ffi::c_int,
    mut atts: *mut ATTRIBUTE,
) -> ::core::ffi::c_int {
    get_atts_with(
        enc,
        ptr,
        attsMax,
        atts,
        2,
        little2_byte_type,
        read_little2_ascii_unit,
    )
}
extern "C" fn little2_charRefNumber(
    _enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    char_ref_number_with(ptr, 2, read_little2_ascii_unit)
}
extern "C" fn little2_predefinedEntityName(
    _enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    predefined_entity_name_with(ptr, end, 2, read_little2_ascii_unit)
}
extern "C" fn little2_nameMatchesAscii(
    _enc: *const ENCODING,
    mut ptr1: *const ::core::ffi::c_char,
    end1: *const ::core::ffi::c_char,
    mut ptr2: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    while read_c_char_bytes::<1>(ptr2)[0] != 0 {
        if ptr1.wrapping_add(1) >= end1 {
            return 0;
        }
        let [first, second] = read_c_char_bytes::<2>(ptr1);
        if second as ::core::ffi::c_int != 0
            || first as ::core::ffi::c_int != read_c_char_bytes::<1>(ptr2)[0] as ::core::ffi::c_int
        {
            return 0;
        }
        ptr1 = ptr1.wrapping_add(2);
        ptr2 = ptr2.wrapping_add(1);
    }
    (ptr1 == end1) as ::core::ffi::c_int
}
extern "C" fn little2_nameLength(
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let start = ptr;
    loop {
        match little2_byte_type(enc, ptr) {
            5 => ptr = ptr.wrapping_add(2),
            6 => ptr = ptr.wrapping_add(3),
            7 => ptr = ptr.wrapping_add(4),
            29 | 22 | 23 | 24 | 25 | 26 | 27 => ptr = ptr.wrapping_add(2),
            _ => return c_char_distance(start, ptr),
        }
    }
}
extern "C" fn little2_skipS(
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    loop {
        match little2_byte_type(enc, ptr) {
            10 | 9 | 21 => ptr = ptr.wrapping_add(2),
            _ => return ptr,
        }
    }
}
extern "C" fn little2_updatePosition(
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    pos: *mut POSITION,
) {
    while remaining_const_c_chars(ptr, end) >= 2 {
        match little2_byte_type(enc, ptr) {
            5 => {
                ptr = ptr.wrapping_add(2);
                increment_column_number(pos);
            }
            6 => {
                ptr = ptr.wrapping_add(3);
                increment_column_number(pos);
            }
            7 => {
                ptr = ptr.wrapping_add(4);
                increment_column_number(pos);
            }
            10 => {
                set_column_number(pos, 0 as XML_Size);
                increment_line_number(pos);
                ptr = ptr.wrapping_add(2);
            }
            9 => {
                increment_line_number(pos);
                ptr = ptr.wrapping_add(2);
                if remaining_const_c_chars(ptr, end) >= 2
                    && little2_byte_type(enc, ptr) == BT_LF as ::core::ffi::c_int
                {
                    ptr = ptr.wrapping_add(2);
                }
                set_column_number(pos, 0 as XML_Size);
            }
            _ => {
                ptr = ptr.wrapping_add(2);
                increment_column_number(pos);
            }
        }
    }
}
extern "C" fn big2_scanComment(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if has_c_chars(ptr, end, 2) {
        if !big2_is_ascii(ptr, b'-') {
            set_const_c_char_ptr(nextTokPtr, ptr);
            return XML_TOK_INVALID;
        }
        ptr = add_const_c_char(ptr, 2);
        while has_c_chars(ptr, end, 2) {
            match big2_byte_type(enc, ptr) {
                value if value == BT_LEAD2 as ::core::ffi::c_int => {
                    if !has_c_chars(ptr, end, 2) {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = add_const_c_char(ptr, 2);
                }
                value if value == BT_LEAD3 as ::core::ffi::c_int => {
                    if !has_c_chars(ptr, end, 3) {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = add_const_c_char(ptr, 3);
                }
                value if value == BT_LEAD4 as ::core::ffi::c_int => {
                    if !has_c_chars(ptr, end, 4) {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = add_const_c_char(ptr, 4);
                }
                value
                    if value == BT_NONXML as ::core::ffi::c_int
                        || value == BT_MALFORM as ::core::ffi::c_int
                        || value == BT_TRAIL as ::core::ffi::c_int =>
                {
                    set_const_c_char_ptr(nextTokPtr, ptr);
                    return XML_TOK_INVALID;
                }
                value if value == BT_MINUS as ::core::ffi::c_int => {
                    ptr = add_const_c_char(ptr, 2);
                    if !has_c_chars(ptr, end, 2) {
                        return XML_TOK_PARTIAL;
                    }
                    if big2_is_ascii(ptr, b'-') {
                        ptr = add_const_c_char(ptr, 2);
                        if !has_c_chars(ptr, end, 2) {
                            return XML_TOK_PARTIAL;
                        }
                        if !big2_is_ascii(ptr, b'>') {
                            set_const_c_char_ptr(nextTokPtr, ptr);
                            return XML_TOK_INVALID;
                        }
                        set_const_c_char_ptr(nextTokPtr, add_const_c_char(ptr, 2));
                        return XML_TOK_COMMENT;
                    }
                }
                _ => {
                    ptr = add_const_c_char(ptr, 2);
                }
            }
        }
    }
    XML_TOK_PARTIAL
}
extern "C" fn big2_scanDecl(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    scan_decl_with(
        enc,
        ptr,
        end,
        nextTokPtr,
        2,
        big2_byte_type,
        big2_scanComment,
    )
}
extern "C" fn big2_checkPiTarget(
    _enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    tokPtr: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    check_pi_target_with(ptr, end, tokPtr, 2, read_big2_ascii_unit)
}
extern "C" fn big2_scanPi(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    scan_utf16_pi_with(
        enc,
        ptr,
        end,
        nextTokPtr,
        big2_byte_type,
        big2_is_name_start,
        big2_is_name_char,
        big2_checkPiTarget,
        big2_is_ascii,
    )
}

extern "C" fn big2_scanCdataSection(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let _ = enc;
    scan_cdata_section_open_with(ptr, end, nextTokPtr, 2, read_big2_ascii_unit)
}
extern "C" fn big2_cdataSectionTok(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    cdata_section_tok_wide(enc, ptr, end, nextTokPtr, big2_byte_type, big2_is_ascii)
}
extern "C" fn big2_scanEndTag(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL;
        }
        let mut c2rust_current_block_32: u64;
        match if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0 as ::core::ffi::c_int as isize),
                *ptr.offset(1 as ::core::ffi::c_int as isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages
                    [*ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3 as ::core::ffi::c_int)
                    + (*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5 as ::core::ffi::c_int)) as usize]
                    & (1 as ::core::ffi::c_uint)
                        << (*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1f as ::core::ffi::c_int)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                c2rust_current_block_32 = 12738221189273011712;
            }
            22 | 24 => {
                c2rust_current_block_32 = 12738221189273011712;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match c2rust_current_block_32 {
            12738221189273011712 => {
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_73: u64;
            match if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0 as ::core::ffi::c_int as isize),
                    *ptr.offset(1 as ::core::ffi::c_int as isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3 as ::core::ffi::c_int)
                        + (*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5 as ::core::ffi::c_int))
                        as usize]
                        & (1 as ::core::ffi::c_uint)
                            << (*ptr.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1f as ::core::ffi::c_int)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    c2rust_current_block_73 = 1281007054303163758;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_73 = 1281007054303163758;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    c2rust_current_block_73 = 981995395831942902;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                    c2rust_current_block_73 = 981995395831942902;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    c2rust_current_block_73 = 981995395831942902;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    while end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long
                    {
                        match if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0 as ::core::ffi::c_int as isize),
                                *ptr.offset(1 as ::core::ffi::c_int as isize),
                            )
                        } {
                            21 | 9 | 10 => {}
                            11 => {
                                *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                return XML_TOK_END_TAG;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    }
                    return XML_TOK_PARTIAL;
                }
                23 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    c2rust_current_block_73 = 981995395831942902;
                }
                11 => {
                    *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    return XML_TOK_END_TAG;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match c2rust_current_block_73 {
                1281007054303163758 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }
}
extern "C" fn big2_scanHexCharRef(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    scan_hex_char_ref_with(enc, ptr, end, nextTokPtr, 2, big2_byte_type)
}
extern "C" fn big2_scanCharRef(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    scan_char_ref_with(
        enc,
        ptr,
        end,
        nextTokPtr,
        2,
        read_big2_ascii_unit,
        big2_byte_type,
    )
}
extern "C" fn big2_scanRef(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    scan_ref_with(
        enc,
        ptr,
        end,
        nextTokPtr,
        2,
        big2_byte_type,
        big2_is_name_start,
        big2_is_name_char,
        reject_multibyte_name_sequence,
        reject_multibyte_name_sequence,
        big2_scanCharRef,
    )
}
extern "C" fn big2_scanAtts(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut hadColon: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_186: u64;
            match if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0 as ::core::ffi::c_int as isize),
                    *ptr.offset(1 as ::core::ffi::c_int as isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3 as ::core::ffi::c_int)
                        + (*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5 as ::core::ffi::c_int))
                        as usize]
                        & (1 as ::core::ffi::c_uint)
                            << (*ptr.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1f as ::core::ffi::c_int)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    c2rust_current_block_186 = 6092917267242331817;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_186 = 6092917267242331817;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    c2rust_current_block_186 = 1634947208139838470;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                    c2rust_current_block_186 = 1634947208139838470;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    c2rust_current_block_186 = 1634947208139838470;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    hadColon = 1 as ::core::ffi::c_int;
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    let mut c2rust_current_block_64: u64;
                    match if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        (*(enc as *const normal_encoding)).type_0[*ptr
                            .offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uchar
                            as usize] as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(0 as ::core::ffi::c_int as isize),
                            *ptr.offset(1 as ::core::ffi::c_int as isize),
                        )
                    } {
                        29 => {
                            if namingBitmap[(((nmstrtPages[*ptr
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar
                                as usize]
                                as ::core::ffi::c_int)
                                << 3 as ::core::ffi::c_int)
                                + (*ptr.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uchar
                                    as ::core::ffi::c_int
                                    >> 5 as ::core::ffi::c_int))
                                as usize]
                                & (1 as ::core::ffi::c_uint)
                                    << (*ptr.offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uchar
                                        as ::core::ffi::c_int
                                        & 0x1f as ::core::ffi::c_int)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            c2rust_current_block_64 = 6604085902723260545;
                        }
                        22 | 24 => {
                            c2rust_current_block_64 = 6604085902723260545;
                        }
                        5 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 2 as ::core::ffi::c_long
                            {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            c2rust_current_block_64 = 10930818133215224067;
                        }
                        6 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 3 as ::core::ffi::c_long
                            {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                            c2rust_current_block_64 = 10930818133215224067;
                        }
                        7 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 4 as ::core::ffi::c_long
                            {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                            c2rust_current_block_64 = 10930818133215224067;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                    match c2rust_current_block_64 {
                        6604085902723260545 => {
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        }
                        _ => {}
                    }
                    c2rust_current_block_186 = 1634947208139838470;
                }
                21 | 9 | 10 => {
                    loop {
                        let mut t: ::core::ffi::c_int = 0;
                        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        t = if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0 as ::core::ffi::c_int as isize),
                                *ptr.offset(1 as ::core::ffi::c_int as isize),
                            )
                        };
                        if t == BT_EQUALS as ::core::ffi::c_int {
                            break;
                        }
                        match t {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                    }
                    c2rust_current_block_186 = 10853015579903106591;
                }
                14 => {
                    c2rust_current_block_186 = 10853015579903106591;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match c2rust_current_block_186 {
                10853015579903106591 => {
                    let mut open: ::core::ffi::c_int = 0;
                    hadColon = 0 as ::core::ffi::c_int;
                    loop {
                        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        open = if *ptr.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0 as ::core::ffi::c_int as isize),
                                *ptr.offset(1 as ::core::ffi::c_int as isize),
                            )
                        };
                        if open == BT_QUOT as ::core::ffi::c_int
                            || open == BT_APOS as ::core::ffi::c_int
                        {
                            break;
                        }
                        match open {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    loop {
                        let mut t_0: ::core::ffi::c_int = 0;
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        t_0 = if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0 as ::core::ffi::c_int as isize),
                                *ptr.offset(1 as ::core::ffi::c_int as isize),
                            )
                        };
                        if t_0 == open {
                            break;
                        }
                        match t_0 {
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 2 as ::core::ffi::c_long
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 3 as ::core::ffi::c_long
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 4 as ::core::ffi::c_long
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                            }
                            0 | 1 | 8 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            3 => {
                                let mut tok: ::core::ffi::c_int = big2_scanRef(
                                    enc,
                                    ptr.offset(2 as ::core::ffi::c_int as isize),
                                    end,
                                    &raw mut ptr,
                                );
                                if tok <= 0 as ::core::ffi::c_int {
                                    if tok == XML_TOK_INVALID {
                                        *nextTokPtr = ptr;
                                    }
                                    return tok;
                                }
                            }
                            2 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            _ => {
                                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            }
                        }
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    match if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        (*(enc as *const normal_encoding)).type_0[*ptr
                            .offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uchar
                            as usize] as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(0 as ::core::ffi::c_int as isize),
                            *ptr.offset(1 as ::core::ffi::c_int as isize),
                        )
                    } {
                        21 | 9 | 10 => {
                            loop {
                                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                        as ::core::ffi::c_long)
                                {
                                    return XML_TOK_PARTIAL;
                                }
                                match if *ptr.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                {
                                    (*(enc as *const normal_encoding)).type_0[*ptr
                                        .offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uchar
                                        as usize]
                                        as ::core::ffi::c_int
                                } else {
                                    unicode_byte_type(
                                        *ptr.offset(0 as ::core::ffi::c_int as isize),
                                        *ptr.offset(1 as ::core::ffi::c_int as isize),
                                    )
                                } {
                                    29 => {
                                        if namingBitmap[(((nmstrtPages[*ptr
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uchar
                                            as usize]
                                            as ::core::ffi::c_int)
                                            << 3 as ::core::ffi::c_int)
                                            + (*ptr.offset(1 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_uchar
                                                as ::core::ffi::c_int
                                                >> 5 as ::core::ffi::c_int))
                                            as usize]
                                            & (1 as ::core::ffi::c_uint)
                                                << (*ptr.offset(1 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uchar
                                                    as ::core::ffi::c_int
                                                    & 0x1f as ::core::ffi::c_int)
                                            == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        c2rust_current_block_186 = 7794494472231011433;
                                        break;
                                    }
                                    22 | 24 => {
                                        c2rust_current_block_186 = 7794494472231011433;
                                        break;
                                    }
                                    5 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long)
                                            < 2 as ::core::ffi::c_long
                                        {
                                            return XML_TOK_PARTIAL_CHAR;
                                        }
                                        if 0 as ::core::ffi::c_int != 0
                                            || 0 as ::core::ffi::c_int == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                        c2rust_current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    6 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long)
                                            < 3 as ::core::ffi::c_long
                                        {
                                            return XML_TOK_PARTIAL_CHAR;
                                        }
                                        if 0 as ::core::ffi::c_int != 0
                                            || 0 as ::core::ffi::c_int == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                                        c2rust_current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    7 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long)
                                            < 4 as ::core::ffi::c_long
                                        {
                                            return XML_TOK_PARTIAL_CHAR;
                                        }
                                        if 0 as ::core::ffi::c_int != 0
                                            || 0 as ::core::ffi::c_int == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID;
                                        }
                                        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                                        c2rust_current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    21 | 9 | 10 => {}
                                    11 => {
                                        c2rust_current_block_186 = 1783713129665224809;
                                        break;
                                    }
                                    17 => {
                                        c2rust_current_block_186 = 18153789983347219713;
                                        break;
                                    }
                                    _ => {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                }
                            }
                            match c2rust_current_block_186 {
                                1783713129665224809 => {}
                                18153789983347219713 => {}
                                1634947208139838470 => {}
                                _ => {
                                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                    c2rust_current_block_186 = 1634947208139838470;
                                }
                            }
                        }
                        17 => {
                            c2rust_current_block_186 = 18153789983347219713;
                        }
                        11 => {
                            c2rust_current_block_186 = 1783713129665224809;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                    match c2rust_current_block_186 {
                        1634947208139838470 => {}
                        _ => match c2rust_current_block_186 {
                            18153789983347219713 => {
                                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                        as ::core::ffi::c_long)
                                {
                                    return XML_TOK_PARTIAL;
                                }
                                if !(*ptr.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                    && *ptr.offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == 0x3e as ::core::ffi::c_int)
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                return XML_TOK_EMPTY_ELEMENT_WITH_ATTS;
                            }
                            _ => {
                                *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                return XML_TOK_START_TAG_WITH_ATTS;
                            }
                        },
                    }
                }
                6092917267242331817 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }
}
extern "C" fn big2_scanLt(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut hadColon: ::core::ffi::c_int = 0;
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL;
        }
        let mut c2rust_current_block_45: u64;
        match if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0 as ::core::ffi::c_int as isize),
                *ptr.offset(1 as ::core::ffi::c_int as isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages
                    [*ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3 as ::core::ffi::c_int)
                    + (*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5 as ::core::ffi::c_int)) as usize]
                    & (1 as ::core::ffi::c_uint)
                        << (*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1f as ::core::ffi::c_int)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                c2rust_current_block_45 = 6477200489819026004;
            }
            22 | 24 => {
                c2rust_current_block_45 = 6477200489819026004;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                c2rust_current_block_45 = 8180496224585318153;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                c2rust_current_block_45 = 8180496224585318153;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                c2rust_current_block_45 = 8180496224585318153;
            }
            16 => {
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long)
                {
                    return XML_TOK_PARTIAL;
                }
                match if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(0 as ::core::ffi::c_int as isize),
                        *ptr.offset(1 as ::core::ffi::c_int as isize),
                    )
                } {
                    27 => {
                        return big2_scanComment(
                            enc,
                            ptr.offset(2 as ::core::ffi::c_int as isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    20 => {
                        return big2_scanCdataSection(
                            enc,
                            ptr.offset(2 as ::core::ffi::c_int as isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            15 => {
                return big2_scanPi(
                    enc,
                    ptr.offset(2 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            17 => {
                return big2_scanEndTag(
                    enc,
                    ptr.offset(2 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        match c2rust_current_block_45 {
            6477200489819026004 => {
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            }
            _ => {}
        }
        hadColon = 0 as ::core::ffi::c_int;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_161: u64;
            match if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0 as ::core::ffi::c_int as isize),
                    *ptr.offset(1 as ::core::ffi::c_int as isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3 as ::core::ffi::c_int)
                        + (*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5 as ::core::ffi::c_int))
                        as usize]
                        & (1 as ::core::ffi::c_uint)
                            << (*ptr.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1f as ::core::ffi::c_int)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    c2rust_current_block_161 = 18151815167355992796;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_161 = 18151815167355992796;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    c2rust_current_block_161 = 14714495436747744489;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                    c2rust_current_block_161 = 14714495436747744489;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    c2rust_current_block_161 = 14714495436747744489;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    hadColon = 1 as ::core::ffi::c_int;
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    let mut c2rust_current_block_112: u64;
                    match if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        (*(enc as *const normal_encoding)).type_0[*ptr
                            .offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uchar
                            as usize] as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(0 as ::core::ffi::c_int as isize),
                            *ptr.offset(1 as ::core::ffi::c_int as isize),
                        )
                    } {
                        29 => {
                            if namingBitmap[(((nmstrtPages[*ptr
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar
                                as usize]
                                as ::core::ffi::c_int)
                                << 3 as ::core::ffi::c_int)
                                + (*ptr.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uchar
                                    as ::core::ffi::c_int
                                    >> 5 as ::core::ffi::c_int))
                                as usize]
                                & (1 as ::core::ffi::c_uint)
                                    << (*ptr.offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uchar
                                        as ::core::ffi::c_int
                                        & 0x1f as ::core::ffi::c_int)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            c2rust_current_block_112 = 16337619596932156899;
                        }
                        22 | 24 => {
                            c2rust_current_block_112 = 16337619596932156899;
                        }
                        5 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 2 as ::core::ffi::c_long
                            {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            c2rust_current_block_112 = 2616667235040759262;
                        }
                        6 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 3 as ::core::ffi::c_long
                            {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                            c2rust_current_block_112 = 2616667235040759262;
                        }
                        7 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 4 as ::core::ffi::c_long
                            {
                                return XML_TOK_PARTIAL_CHAR;
                            }
                            if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                            ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                            c2rust_current_block_112 = 2616667235040759262;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID;
                        }
                    }
                    match c2rust_current_block_112 {
                        16337619596932156899 => {
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        }
                        _ => {}
                    }
                    c2rust_current_block_161 = 14714495436747744489;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    loop {
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            c2rust_current_block_161 = 13215501469961642988;
                            break;
                        }
                        match if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0 as ::core::ffi::c_int as isize),
                                *ptr.offset(1 as ::core::ffi::c_int as isize),
                            )
                        } {
                            29 => {
                                if namingBitmap[(((nmstrtPages[*ptr
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uchar
                                    as usize]
                                    as ::core::ffi::c_int)
                                    << 3 as ::core::ffi::c_int)
                                    + (*ptr.offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uchar
                                        as ::core::ffi::c_int
                                        >> 5 as ::core::ffi::c_int))
                                    as usize]
                                    & (1 as ::core::ffi::c_uint)
                                        << (*ptr.offset(1 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uchar
                                            as ::core::ffi::c_int
                                            & 0x1f as ::core::ffi::c_int)
                                    == 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                c2rust_current_block_161 = 11066148936714919733;
                            }
                            22 | 24 => {
                                c2rust_current_block_161 = 11066148936714919733;
                            }
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 2 as ::core::ffi::c_long
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                c2rust_current_block_161 = 16314074004867283505;
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 3 as ::core::ffi::c_long
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                                c2rust_current_block_161 = 16314074004867283505;
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 4 as ::core::ffi::c_long
                                {
                                    return XML_TOK_PARTIAL_CHAR;
                                }
                                if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID;
                                }
                                ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                                c2rust_current_block_161 = 16314074004867283505;
                            }
                            11 => {
                                c2rust_current_block_161 = 13089361350718158941;
                                break;
                            }
                            17 => {
                                c2rust_current_block_161 = 11384015785330443424;
                                break;
                            }
                            21 | 9 | 10 => {
                                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                continue;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID;
                            }
                        }
                        match c2rust_current_block_161 {
                            11066148936714919733 => {
                                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            }
                            _ => {}
                        }
                        return big2_scanAtts(enc, ptr, end, nextTokPtr);
                    }
                    match c2rust_current_block_161 {
                        13089361350718158941 => {}
                        11384015785330443424 => {}
                        _ => return XML_TOK_PARTIAL,
                    }
                }
                11 => {
                    c2rust_current_block_161 = 13089361350718158941;
                }
                17 => {
                    c2rust_current_block_161 = 11384015785330443424;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match c2rust_current_block_161 {
                11384015785330443424 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if !(*ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        && *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0x3e as ::core::ffi::c_int)
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    return XML_TOK_EMPTY_ELEMENT_NO_ATTS;
                }
                13089361350718158941 => {
                    *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    return XML_TOK_START_TAG_NO_ATTS;
                }
                18151815167355992796 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL;
    }
}
extern "C" fn big2_contentTok(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if ptr >= end {
            return XML_TOK_NONE;
        }
        if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            let mut n: size_t = end.offset_from(ptr) as ::core::ffi::c_long as size_t;
            if n & (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t != 0 {
                n &= !(2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t;
                if n == 0 as size_t {
                    return XML_TOK_PARTIAL;
                }
                end = ptr.offset(n as isize);
            }
        }
        match if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0 as ::core::ffi::c_int as isize),
                *ptr.offset(1 as ::core::ffi::c_int as isize),
            )
        } {
            2 => {
                return big2_scanLt(
                    enc,
                    ptr.offset(2 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            3 => {
                return big2_scanRef(
                    enc,
                    ptr.offset(2 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            9 => {
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long)
                {
                    return XML_TOK_TRAILING_CR;
                }
                if (if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(0 as ::core::ffi::c_int as isize),
                        *ptr.offset(1 as ::core::ffi::c_int as isize),
                    )
                }) == BT_LF as ::core::ffi::c_int
                {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                *nextTokPtr = ptr;
                return XML_TOK_DATA_NEWLINE;
            }
            10 => {
                *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                return XML_TOK_DATA_NEWLINE;
            }
            4 => {
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long)
                {
                    return XML_TOK_TRAILING_RSQB;
                }
                if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                    && *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0x5d as ::core::ffi::c_int
                {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_TRAILING_RSQB;
                    }
                    if !(*ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        && *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0x3e as ::core::ffi::c_int)
                    {
                        ptr = ptr.offset(-(2 as ::core::ffi::c_int as isize));
                    } else {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                }
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
            }
            0 | 1 | 8 => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            _ => {
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_76: u64;
            match if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0 as ::core::ffi::c_int as isize),
                    *ptr.offset(1 as ::core::ffi::c_int as isize),
                )
            } {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long
                        || 0 as ::core::ffi::c_int != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    c2rust_current_block_76 = 7158658067966855297;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long
                        || 0 as ::core::ffi::c_int != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                    c2rust_current_block_76 = 7158658067966855297;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long
                        || 0 as ::core::ffi::c_int != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS;
                    }
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    c2rust_current_block_76 = 7158658067966855297;
                }
                4 => {
                    if end.offset_from(ptr) as ::core::ffi::c_long
                        >= (2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long
                    {
                        if !(*ptr
                            .offset(2 as ::core::ffi::c_int as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                            && *ptr
                                .offset(2 as ::core::ffi::c_int as isize)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == 0x5d as ::core::ffi::c_int)
                        {
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            c2rust_current_block_76 = 7158658067966855297;
                        } else if end.offset_from(ptr) as ::core::ffi::c_long
                            >= (3 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as ::core::ffi::c_long
                        {
                            if !(*ptr
                                .offset(
                                    (2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                )
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                                && *ptr
                                    .offset(
                                        (2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    )
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == 0x3e as ::core::ffi::c_int)
                            {
                                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            } else {
                                *nextTokPtr = ptr.offset(
                                    (2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                );
                                return XML_TOK_INVALID;
                            }
                            c2rust_current_block_76 = 7158658067966855297;
                        } else {
                            c2rust_current_block_76 = 11890188771060868767;
                        }
                    } else {
                        c2rust_current_block_76 = 11890188771060868767;
                    }
                }
                3 | 2 | 0 | 1 | 8 | 9 | 10 => {
                    c2rust_current_block_76 = 11890188771060868767;
                }
                _ => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    c2rust_current_block_76 = 7158658067966855297;
                }
            }
            match c2rust_current_block_76 {
                7158658067966855297 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }
}
extern "C" fn big2_scanPercent(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    scan_utf16_name_token_with(
        enc,
        ptr,
        end,
        nextTokPtr,
        big2_byte_type,
        big2_is_name_start,
        big2_is_name_char,
        &[
            BT_S as ::core::ffi::c_int,
            BT_LF as ::core::ffi::c_int,
            BT_CR as ::core::ffi::c_int,
            BT_PERCNT as ::core::ffi::c_int,
        ],
        &[BT_SEMI as ::core::ffi::c_int],
        XML_TOK_PERCENT,
        XML_TOK_PARAM_ENTITY_REF,
        2,
        XML_TOK_PARTIAL,
    )
}

extern "C" fn big2_scanPoundName(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    scan_utf16_name_token_with(
        enc,
        ptr,
        end,
        nextTokPtr,
        big2_byte_type,
        big2_is_name_start,
        big2_is_name_char,
        &[],
        &[
            BT_CR as ::core::ffi::c_int,
            BT_LF as ::core::ffi::c_int,
            BT_S as ::core::ffi::c_int,
            BT_RPAR as ::core::ffi::c_int,
            BT_GT as ::core::ffi::c_int,
            BT_PERCNT as ::core::ffi::c_int,
            BT_VERBAR as ::core::ffi::c_int,
        ],
        XML_TOK_POUND_NAME,
        XML_TOK_POUND_NAME,
        0,
        -XML_TOK_POUND_NAME,
    )
}

extern "C" fn big2_scanLit(
    mut open: ::core::ffi::c_int,
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    scan_lit_with(
        open,
        enc,
        ptr,
        end,
        nextTokPtr,
        2,
        big2_byte_type,
        allow_literal_multibyte_sequence,
    )
}
extern "C" fn big2_prologTok(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut tok: ::core::ffi::c_int = 0;
        if ptr >= end {
            return XML_TOK_NONE;
        }
        if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            let mut n: size_t = end.offset_from(ptr) as ::core::ffi::c_long as size_t;
            if n & (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t != 0 {
                n &= !(2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t;
                if n == 0 as size_t {
                    return XML_TOK_PARTIAL;
                }
                end = ptr.offset(n as isize);
            }
        }
        let mut c2rust_current_block_124: u64;
        match if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0 as ::core::ffi::c_int as isize),
                *ptr.offset(1 as ::core::ffi::c_int as isize),
            )
        } {
            12 => {
                return big2_scanLit(
                    BT_QUOT as ::core::ffi::c_int,
                    enc,
                    ptr.offset(2 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            13 => {
                return big2_scanLit(
                    BT_APOS as ::core::ffi::c_int,
                    enc,
                    ptr.offset(2 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            2 => {
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long)
                {
                    return XML_TOK_PARTIAL;
                }
                match if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(0 as ::core::ffi::c_int as isize),
                        *ptr.offset(1 as ::core::ffi::c_int as isize),
                    )
                } {
                    16 => {
                        return big2_scanDecl(
                            enc,
                            ptr.offset(2 as ::core::ffi::c_int as isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    15 => {
                        return big2_scanPi(
                            enc,
                            ptr.offset(2 as ::core::ffi::c_int as isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    22 | 24 | 29 | 5 | 6 | 7 => {
                        *nextTokPtr = ptr.offset(-(2 as ::core::ffi::c_int as isize));
                        return XML_TOK_INSTANCE_START;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            9 => {
                if ptr.offset(2 as ::core::ffi::c_int as isize) == end {
                    *nextTokPtr = end;
                    return -XML_TOK_PROLOG_S;
                }
                c2rust_current_block_124 = 16869865525854146339;
            }
            21 | 10 => {
                c2rust_current_block_124 = 16869865525854146339;
            }
            30 => {
                return big2_scanPercent(
                    enc,
                    ptr.offset(2 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            35 => {
                *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                return XML_TOK_COMMA;
            }
            20 => {
                *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                return XML_TOK_OPEN_BRACKET;
            }
            4 => {
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long)
                {
                    return -XML_TOK_CLOSE_BRACKET;
                }
                if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                    && *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0x5d as ::core::ffi::c_int
                {
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if *ptr
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        && *ptr
                            .offset(2 as ::core::ffi::c_int as isize)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == 0x3e as ::core::ffi::c_int
                    {
                        *nextTokPtr = ptr
                            .offset((2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize);
                        return XML_TOK_COND_SECT_CLOSE;
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_CLOSE_BRACKET;
            }
            31 => {
                *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                return XML_TOK_OPEN_PAREN;
            }
            32 => {
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long)
                {
                    return -XML_TOK_CLOSE_PAREN;
                }
                match if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(0 as ::core::ffi::c_int as isize),
                        *ptr.offset(1 as ::core::ffi::c_int as isize),
                    )
                } {
                    33 => {
                        *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        return XML_TOK_CLOSE_PAREN_ASTERISK;
                    }
                    15 => {
                        *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        return XML_TOK_CLOSE_PAREN_QUESTION;
                    }
                    34 => {
                        *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        return XML_TOK_CLOSE_PAREN_PLUS;
                    }
                    9 | 10 | 21 | 11 | 35 | 36 | 32 => {
                        *nextTokPtr = ptr;
                        return XML_TOK_CLOSE_PAREN;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            36 => {
                *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                return XML_TOK_OR;
            }
            11 => {
                *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                return XML_TOK_DECL_CLOSE;
            }
            19 => {
                return big2_scanPoundName(
                    enc,
                    ptr.offset(2 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                    return XML_TOK_PARTIAL_CHAR;
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
            22 | 24 => {
                tok = XML_TOK_NAME;
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                c2rust_current_block_124 = 2956972668325154207;
            }
            25 | 26 | 27 | 23 => {
                tok = XML_TOK_NMTOKEN;
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                c2rust_current_block_124 = 2956972668325154207;
            }
            29 => {
                if namingBitmap[(((nmstrtPages
                    [*ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3 as ::core::ffi::c_int)
                    + (*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5 as ::core::ffi::c_int)) as usize]
                    & (1 as ::core::ffi::c_uint)
                        << (*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1f as ::core::ffi::c_int)
                    != 0
                {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    tok = XML_TOK_NAME;
                    c2rust_current_block_124 = 2956972668325154207;
                } else if namingBitmap[(((namePages
                    [*ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3 as ::core::ffi::c_int)
                    + (*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5 as ::core::ffi::c_int))
                    as usize]
                    & (1 as ::core::ffi::c_uint)
                        << (*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1f as ::core::ffi::c_int)
                    != 0
                {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    tok = XML_TOK_NMTOKEN;
                    c2rust_current_block_124 = 2956972668325154207;
                } else {
                    c2rust_current_block_124 = 6428058487030868344;
                }
            }
            _ => {
                c2rust_current_block_124 = 6428058487030868344;
            }
        }
        match c2rust_current_block_124 {
            2956972668325154207 => {}
            16869865525854146339 => {
                loop {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        break;
                    }
                    let mut c2rust_current_block_32: u64;
                    match if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        (*(enc as *const normal_encoding)).type_0[*ptr
                            .offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uchar
                            as usize] as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(0 as ::core::ffi::c_int as isize),
                            *ptr.offset(1 as ::core::ffi::c_int as isize),
                        )
                    } {
                        21 | 10 => {
                            c2rust_current_block_32 = 17500079516916021833;
                        }
                        9 => {
                            if ptr.offset(2 as ::core::ffi::c_int as isize) != end {
                                c2rust_current_block_32 = 17500079516916021833;
                            } else {
                                c2rust_current_block_32 = 11299462660638600073;
                            }
                        }
                        _ => {
                            c2rust_current_block_32 = 11299462660638600073;
                        }
                    }
                    match c2rust_current_block_32 {
                        17500079516916021833 => {}
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_PROLOG_S;
                        }
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_PROLOG_S;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID;
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_210: u64;
            match if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0 as ::core::ffi::c_int as isize),
                    *ptr.offset(1 as ::core::ffi::c_int as isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3 as ::core::ffi::c_int)
                        + (*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5 as ::core::ffi::c_int))
                        as usize]
                        & (1 as ::core::ffi::c_uint)
                            << (*ptr.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1f as ::core::ffi::c_int)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    c2rust_current_block_210 = 9794574411605359176;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_210 = 9794574411605359176;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    c2rust_current_block_210 = 14244298717249035578;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                    c2rust_current_block_210 = 14244298717249035578;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    c2rust_current_block_210 = 14244298717249035578;
                }
                11 | 32 | 35 | 36 | 20 | 30 | 21 | 9 | 10 => {
                    *nextTokPtr = ptr;
                    return tok;
                }
                23 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    match tok {
                        XML_TOK_NAME => {
                            if !(end.offset_from(ptr) as ::core::ffi::c_long
                                >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                    as ::core::ffi::c_long)
                            {
                                return XML_TOK_PARTIAL;
                            }
                            tok = XML_TOK_PREFIXED_NAME;
                            let mut c2rust_current_block_187: u64;
                            match if *ptr.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                (*(enc as *const normal_encoding)).type_0[*ptr
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uchar
                                    as usize] as ::core::ffi::c_int
                            } else {
                                unicode_byte_type(
                                    *ptr.offset(0 as ::core::ffi::c_int as isize),
                                    *ptr.offset(1 as ::core::ffi::c_int as isize),
                                )
                            } {
                                29 => {
                                    if namingBitmap[(((namePages[*ptr
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uchar
                                        as usize]
                                        as ::core::ffi::c_int)
                                        << 3 as ::core::ffi::c_int)
                                        + (*ptr.offset(1 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uchar
                                            as ::core::ffi::c_int
                                            >> 5 as ::core::ffi::c_int))
                                        as usize]
                                        & (1 as ::core::ffi::c_uint)
                                            << (*ptr.offset(1 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_uchar
                                                as ::core::ffi::c_int
                                                & 0x1f as ::core::ffi::c_int)
                                        == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    c2rust_current_block_187 = 17275381528970576968;
                                }
                                22 | 24 | 25 | 26 | 27 => {
                                    c2rust_current_block_187 = 17275381528970576968;
                                }
                                5 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long)
                                        < 2 as ::core::ffi::c_long
                                    {
                                        return XML_TOK_PARTIAL_CHAR;
                                    }
                                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                                6 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long)
                                        < 3 as ::core::ffi::c_long
                                    {
                                        return XML_TOK_PARTIAL_CHAR;
                                    }
                                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                                7 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long)
                                        < 4 as ::core::ffi::c_long
                                    {
                                        return XML_TOK_PARTIAL_CHAR;
                                    }
                                    if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID;
                                    }
                                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                                _ => {
                                    tok = XML_TOK_NMTOKEN;
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                            }
                            match c2rust_current_block_187 {
                                17275381528970576968 => {
                                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                }
                                _ => {}
                            }
                        }
                        XML_TOK_PREFIXED_NAME => {
                            tok = XML_TOK_NMTOKEN;
                        }
                        _ => {}
                    }
                    c2rust_current_block_210 = 14244298717249035578;
                }
                34 => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    return XML_TOK_NAME_PLUS;
                }
                33 => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    return XML_TOK_NAME_ASTERISK;
                }
                15 => {
                    if tok == XML_TOK_NMTOKEN {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID;
                    }
                    *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    return XML_TOK_NAME_QUESTION;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
            }
            match c2rust_current_block_210 {
                9794574411605359176 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                _ => {}
            }
        }
        return -tok;
    }
}
extern "C" fn big2_attributeValueTok(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if ptr >= end {
            return XML_TOK_NONE;
        } else if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL;
        }
        start = ptr;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            match if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0 as ::core::ffi::c_int as isize),
                    *ptr.offset(1 as ::core::ffi::c_int as isize),
                )
            } {
                5 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                6 => {
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                }
                7 => {
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                }
                3 => {
                    if ptr == start {
                        return big2_scanRef(
                            enc,
                            ptr.offset(2 as ::core::ffi::c_int as isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                2 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_TRAILING_CR;
                        }
                        if (if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0 as ::core::ffi::c_int as isize),
                                *ptr.offset(1 as ::core::ffi::c_int as isize),
                            )
                        }) == BT_LF as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        }
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                21 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        return XML_TOK_ATTRIBUTE_VALUE_S;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                _ => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }
}
extern "C" fn big2_entityValueTok(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if ptr >= end {
            return XML_TOK_NONE;
        } else if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL;
        }
        start = ptr;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            match if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0 as ::core::ffi::c_int as isize),
                    *ptr.offset(1 as ::core::ffi::c_int as isize),
                )
            } {
                5 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                6 => {
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                }
                7 => {
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                }
                3 => {
                    if ptr == start {
                        return big2_scanRef(
                            enc,
                            ptr.offset(2 as ::core::ffi::c_int as isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                30 => {
                    if ptr == start {
                        let mut tok: ::core::ffi::c_int = big2_scanPercent(
                            enc,
                            ptr.offset(2 as ::core::ffi::c_int as isize),
                            end,
                            nextTokPtr,
                        );
                        return if tok == XML_TOK_PERCENT {
                            XML_TOK_INVALID
                        } else {
                            tok
                        };
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_TRAILING_CR;
                        }
                        if (if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0 as ::core::ffi::c_int as isize),
                                *ptr.offset(1 as ::core::ffi::c_int as isize),
                            )
                        }) == BT_LF as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        }
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_NEWLINE;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS;
                }
                _ => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS;
    }
}
extern "C" fn big2_ignoreSectionTok(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut level: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            let mut n: size_t = end.offset_from(ptr) as ::core::ffi::c_long as size_t;
            if n & (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t != 0 {
                n &= !(2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t;
                end = ptr.offset(n as isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            match if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0 as ::core::ffi::c_int as isize),
                    *ptr.offset(1 as ::core::ffi::c_int as isize),
                )
            } {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                        return XML_TOK_PARTIAL_CHAR;
                    }
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID;
                }
                2 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        && *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0x21 as ::core::ffi::c_int
                    {
                        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                            && *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                == 0x5b as ::core::ffi::c_int
                        {
                            level += 1;
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        }
                    }
                }
                4 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL;
                    }
                    if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        && *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0x5d as ::core::ffi::c_int
                    {
                        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL;
                        }
                        if *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                            && *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                == 0x3e as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            if level == 0 as ::core::ffi::c_int {
                                *nextTokPtr = ptr;
                                return XML_TOK_IGNORE_SECT;
                            }
                            level -= 1;
                        }
                    }
                }
                _ => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
            }
        }
        return XML_TOK_PARTIAL;
    }
}
extern "C" fn big2_isPublicId(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    badPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let ptr = add_const_c_char(ptr, 2);
    let end = add_const_c_char(end, -2);
    validate_public_id(
        ptr,
        end,
        2,
        badPtr,
        |current| {
            let high = read_c_char(current);
            let low = read_c_char(add_const_c_char(current, 1));
            if high == 0 {
                with_ref(enc.cast::<normal_encoding>(), |normal| {
                    normal.type_0[low as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
                })
            } else {
                unicode_byte_type(high, low)
            }
        },
        |current| {
            let high = read_c_char(current) as ::core::ffi::c_int;
            let low = read_c_char(add_const_c_char(current, 1)) as ::core::ffi::c_int;
            if high == 0 {
                low
            } else {
                -(1 as ::core::ffi::c_int)
            }
        },
    )
}
extern "C" fn big2_getAtts(
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut attsMax: ::core::ffi::c_int,
    mut atts: *mut ATTRIBUTE,
) -> ::core::ffi::c_int {
    get_atts_with(
        enc,
        ptr,
        attsMax,
        atts,
        2,
        big2_byte_type,
        read_big2_ascii_unit,
    )
}
extern "C" fn big2_charRefNumber(
    _enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    char_ref_number_with(ptr, 2, read_big2_ascii_unit)
}
extern "C" fn big2_predefinedEntityName(
    _enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    predefined_entity_name_with(ptr, end, 2, read_big2_ascii_unit)
}
extern "C" fn big2_nameMatchesAscii(
    _enc: *const ENCODING,
    mut ptr1: *const ::core::ffi::c_char,
    end1: *const ::core::ffi::c_char,
    mut ptr2: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    while read_c_char_bytes::<1>(ptr2)[0] != 0 {
        if ptr1.wrapping_add(1) >= end1 {
            return 0;
        }
        let [first, second] = read_c_char_bytes::<2>(ptr1);
        if first as ::core::ffi::c_int != 0
            || second as ::core::ffi::c_int != read_c_char_bytes::<1>(ptr2)[0] as ::core::ffi::c_int
        {
            return 0;
        }
        ptr1 = ptr1.wrapping_add(2);
        ptr2 = ptr2.wrapping_add(1);
    }
    (ptr1 == end1) as ::core::ffi::c_int
}
extern "C" fn big2_nameLength(
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let start = ptr;
    loop {
        match big2_byte_type(enc, ptr) {
            5 => ptr = ptr.wrapping_add(2),
            6 => ptr = ptr.wrapping_add(3),
            7 => ptr = ptr.wrapping_add(4),
            29 | 22 | 23 | 24 | 25 | 26 | 27 => ptr = ptr.wrapping_add(2),
            _ => return c_char_distance(start, ptr),
        }
    }
}
extern "C" fn big2_skipS(
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    loop {
        match big2_byte_type(enc, ptr) {
            10 | 9 | 21 => ptr = ptr.wrapping_add(2),
            _ => return ptr,
        }
    }
}
extern "C" fn big2_updatePosition(
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    pos: *mut POSITION,
) {
    while remaining_const_c_chars(ptr, end) >= 2 {
        match big2_byte_type(enc, ptr) {
            5 => {
                ptr = ptr.wrapping_add(2);
                increment_column_number(pos);
            }
            6 => {
                ptr = ptr.wrapping_add(3);
                increment_column_number(pos);
            }
            7 => {
                ptr = ptr.wrapping_add(4);
                increment_column_number(pos);
            }
            10 => {
                set_column_number(pos, 0 as XML_Size);
                increment_line_number(pos);
                ptr = ptr.wrapping_add(2);
            }
            9 => {
                increment_line_number(pos);
                ptr = ptr.wrapping_add(2);
                if remaining_const_c_chars(ptr, end) >= 2
                    && big2_byte_type(enc, ptr) == BT_LF as ::core::ffi::c_int
                {
                    ptr = ptr.wrapping_add(2);
                }
                set_column_number(pos, 0 as XML_Size);
            }
            _ => {
                ptr = ptr.wrapping_add(2);
                increment_column_number(pos);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn _INTERNAL_trim_to_complete_utf8_characters(
    mut from: *const ::core::ffi::c_char,
    mut fromLimRef: *mut *const ::core::ffi::c_char,
) {
    write_copy(
        fromLimRef,
        trim_to_complete_utf8_end(from, read_copy(fromLimRef)),
    );
}
extern "C" fn utf8_toUtf8(
    _enc: *const ENCODING,
    fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    toP: *mut *mut ::core::ffi::c_char,
    toLim: *const ::core::ffi::c_char,
) -> XML_Convert_Result {
    let from = read_copy(fromP);
    let to = read_copy(toP);
    let mut input_incomplete = false_0 != 0;
    let mut output_exhausted = false_0 != 0;
    let bytes_available = remaining_const_c_chars(from, fromLim);
    let bytes_storable = remaining_c_chars(to, toLim);

    if bytes_available > bytes_storable {
        fromLim = add_const_c_char(from, bytes_storable as isize);
        output_exhausted = true_0 != 0;
    }

    let from_lim_before = fromLim;
    fromLim = trim_to_complete_utf8_end(from, fromLim);
    if fromLim < from_lim_before {
        input_incomplete = true_0 != 0;
    }

    let bytes_to_copy = remaining_const_c_chars(from, fromLim);
    copy_c_chars(to, from, bytes_to_copy);
    write_copy(fromP, add_const_c_char(from, bytes_to_copy as isize));
    write_copy(toP, add_mut_c_char(to, bytes_to_copy as isize));

    if output_exhausted {
        XML_CONVERT_OUTPUT_EXHAUSTED
    } else if input_incomplete {
        XML_CONVERT_INPUT_INCOMPLETE
    } else {
        XML_CONVERT_COMPLETED
    }
}
extern "C" fn utf8_toUtf16(
    enc: *const ENCODING,
    fromP: *mut *const ::core::ffi::c_char,
    fromLim: *const ::core::ffi::c_char,
    toP: *mut *mut ::core::ffi::c_ushort,
    toLim: *const ::core::ffi::c_ushort,
) -> XML_Convert_Result {
    let mut res = XML_CONVERT_COMPLETED;
    let mut to = read_copy(toP);
    let mut from = read_copy(fromP);

    while from < fromLim && to < toLim as *mut ::core::ffi::c_ushort {
        match normal_byte_type(enc, from) {
            5 => {
                if remaining_const_c_chars(from, fromLim) < 2 {
                    res = XML_CONVERT_INPUT_INCOMPLETE;
                    break;
                }

                let first = read_c_uchar(from) as ::core::ffi::c_int;
                let second = read_c_uchar(add_const_c_char(from, 1)) as ::core::ffi::c_int;
                write_c_ushort_and_advance(
                    &mut to,
                    (((first & 0x1f) << 6) | (second & 0x3f)) as ::core::ffi::c_ushort,
                );
                from = add_const_c_char(from, 2);
            }
            6 => {
                if remaining_const_c_chars(from, fromLim) < 3 {
                    res = XML_CONVERT_INPUT_INCOMPLETE;
                    break;
                }

                let first = read_c_uchar(from) as ::core::ffi::c_int;
                let second = read_c_uchar(add_const_c_char(from, 1)) as ::core::ffi::c_int;
                let third = read_c_uchar(add_const_c_char(from, 2)) as ::core::ffi::c_int;
                write_c_ushort_and_advance(
                    &mut to,
                    (((first & 0xf) << 12) | ((second & 0x3f) << 6) | (third & 0x3f))
                        as ::core::ffi::c_ushort,
                );
                from = add_const_c_char(from, 3);
            }
            7 => {
                if remaining_c_ushorts(to, toLim) < 2 {
                    res = XML_CONVERT_OUTPUT_EXHAUSTED;
                    break;
                }
                if remaining_const_c_chars(from, fromLim) < 4 {
                    res = XML_CONVERT_INPUT_INCOMPLETE;
                    break;
                }

                let first = read_c_uchar(from) as ::core::ffi::c_ulong;
                let second = read_c_uchar(add_const_c_char(from, 1)) as ::core::ffi::c_ulong;
                let third = read_c_uchar(add_const_c_char(from, 2)) as ::core::ffi::c_ulong;
                let fourth = read_c_uchar(add_const_c_char(from, 3)) as ::core::ffi::c_ulong;
                let n = (((first & 0x7) << 18)
                    | ((second & 0x3f) << 12)
                    | ((third & 0x3f) << 6)
                    | (fourth & 0x3f))
                    .wrapping_sub(0x10000);
                write_copy(to, ((n >> 10) | 0xd800) as ::core::ffi::c_ushort);
                write_copy(
                    add_mut_c_ushort(to, 1),
                    ((n & 0x3ff) | 0xdc00) as ::core::ffi::c_ushort,
                );
                to = add_mut_c_ushort(to, 2);
                from = add_const_c_char(from, 4);
            }
            _ => {
                write_c_ushort_and_advance(&mut to, read_c_char(from) as ::core::ffi::c_ushort);
                from = add_const_c_char(from, 1);
            }
        }
    }

    if to == toLim as *mut ::core::ffi::c_ushort && from < fromLim {
        res = XML_CONVERT_OUTPUT_EXHAUSTED;
    }

    write_copy(fromP, from);
    write_copy(toP, to);
    res
}
static mut utf8_encoding_ns: normal_encoding = unsafe {
    normal_encoding {
        enc: encoding {
            scanners: [
                Some(
                    normal_prologTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_contentTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_cdataSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_ignoreSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            literalScanners: [
                Some(
                    normal_attributeValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_entityValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                normal_nameMatchesAscii
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                normal_nameLength
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                normal_skipS
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                normal_getAtts
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                normal_charRefNumber
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                normal_predefinedEntityName
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                normal_updatePosition
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                normal_isPublicId
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(utf8_toUtf8),
            utf16Convert: Some(utf8_toUtf16),
            minBytesPerChar: 1 as ::core::ffi::c_int,
            isUtf8: 1 as ::core::ffi::c_char,
            isUtf16: 0 as ::core::ffi::c_char,
        },
        type_0: [
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        isName2: Some(utf8_isName2),
        isName3: Some(utf8_isName3),
        isName4: Some(isNever),
        isNmstrt2: Some(utf8_isNmstrt2),
        isNmstrt3: Some(utf8_isNmstrt3),
        isNmstrt4: Some(isNever),
        isInvalid2: Some(utf8_isInvalid2),
        isInvalid3: Some(utf8_isInvalid3),
        isInvalid4: Some(utf8_isInvalid4),
    }
};
static mut utf8_encoding: normal_encoding = unsafe {
    normal_encoding {
        enc: encoding {
            scanners: [
                Some(
                    normal_prologTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_contentTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_cdataSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_ignoreSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            literalScanners: [
                Some(
                    normal_attributeValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_entityValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                normal_nameMatchesAscii
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                normal_nameLength
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                normal_skipS
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                normal_getAtts
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                normal_charRefNumber
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                normal_predefinedEntityName
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                normal_updatePosition
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                normal_isPublicId
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(utf8_toUtf8),
            utf16Convert: Some(utf8_toUtf16),
            minBytesPerChar: 1 as ::core::ffi::c_int,
            isUtf8: 1 as ::core::ffi::c_char,
            isUtf16: 0 as ::core::ffi::c_char,
        },
        type_0: [
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        isName2: Some(utf8_isName2),
        isName3: Some(utf8_isName3),
        isName4: Some(isNever),
        isNmstrt2: Some(utf8_isNmstrt2),
        isNmstrt3: Some(utf8_isNmstrt3),
        isNmstrt4: Some(isNever),
        isInvalid2: Some(utf8_isInvalid2),
        isInvalid3: Some(utf8_isInvalid3),
        isInvalid4: Some(utf8_isInvalid4),
    }
};
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut internal_utf8_encoding_ns: normal_encoding = unsafe {
    normal_encoding {
        enc: encoding {
            scanners: [
                Some(
                    normal_prologTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_contentTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_cdataSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_ignoreSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            literalScanners: [
                Some(
                    normal_attributeValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_entityValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                normal_nameMatchesAscii
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                normal_nameLength
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                normal_skipS
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                normal_getAtts
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                normal_charRefNumber
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                normal_predefinedEntityName
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                normal_updatePosition
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                normal_isPublicId
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(utf8_toUtf8),
            utf16Convert: Some(utf8_toUtf16),
            minBytesPerChar: 1 as ::core::ffi::c_int,
            isUtf8: 1 as ::core::ffi::c_char,
            isUtf16: 0 as ::core::ffi::c_char,
        },
        type_0: [
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        isName2: Some(utf8_isName2),
        isName3: Some(utf8_isName3),
        isName4: Some(isNever),
        isNmstrt2: Some(utf8_isNmstrt2),
        isNmstrt3: Some(utf8_isNmstrt3),
        isNmstrt4: Some(isNever),
        isInvalid2: Some(utf8_isInvalid2),
        isInvalid3: Some(utf8_isInvalid3),
        isInvalid4: Some(utf8_isInvalid4),
    }
};
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const ASCII_A: ::core::ffi::c_int = 0x41 as ::core::ffi::c_int;
pub const ASCII_B: ::core::ffi::c_int = 66;
pub const ASCII_C: ::core::ffi::c_int = 0x43 as ::core::ffi::c_int;
pub const ASCII_D: ::core::ffi::c_int = 0x44 as ::core::ffi::c_int;
pub const ASCII_E: ::core::ffi::c_int = 69;
pub const ASCII_F: ::core::ffi::c_int = 70;
pub const ASCII_I: ::core::ffi::c_int = 0x49 as ::core::ffi::c_int;
pub const ASCII_L: ::core::ffi::c_int = 76;
pub const ASCII_M: ::core::ffi::c_int = 77;
pub const ASCII_O: ::core::ffi::c_int = 0x4f as ::core::ffi::c_int;
pub const ASCII_S: ::core::ffi::c_int = 0x53 as ::core::ffi::c_int;
pub const ASCII_T: ::core::ffi::c_int = 0x54 as ::core::ffi::c_int;
pub const ASCII_U: ::core::ffi::c_int = 0x55 as ::core::ffi::c_int;
pub const ASCII_X: ::core::ffi::c_int = 88;
pub const ASCII_Z: ::core::ffi::c_int = 0x5a as ::core::ffi::c_int;
pub const ASCII_a: ::core::ffi::c_int = 97;
pub const ASCII_b: ::core::ffi::c_int = 98;
pub const ASCII_c: ::core::ffi::c_int = 99;
pub const ASCII_d: ::core::ffi::c_int = 100;
pub const ASCII_e: ::core::ffi::c_int = 101;
pub const ASCII_f: ::core::ffi::c_int = 102;
pub const ASCII_g: ::core::ffi::c_int = 103;
pub const ASCII_i: ::core::ffi::c_int = 0x69 as ::core::ffi::c_int;
pub const ASCII_l: ::core::ffi::c_int = 108;
pub const ASCII_m: ::core::ffi::c_int = 109;
pub const ASCII_n: ::core::ffi::c_int = 0x6e as ::core::ffi::c_int;
pub const ASCII_o: ::core::ffi::c_int = 0x6f as ::core::ffi::c_int;
pub const ASCII_q: ::core::ffi::c_int = 113;
pub const ASCII_r: ::core::ffi::c_int = 0x72 as ::core::ffi::c_int;
pub const ASCII_s: ::core::ffi::c_int = 0x73 as ::core::ffi::c_int;
pub const ASCII_t: ::core::ffi::c_int = 0x74 as ::core::ffi::c_int;
pub const ASCII_v: ::core::ffi::c_int = 0x76 as ::core::ffi::c_int;
pub const ASCII_x: ::core::ffi::c_int = 120;
pub const ASCII_y: ::core::ffi::c_int = 0x79 as ::core::ffi::c_int;
pub const ASCII_z: ::core::ffi::c_int = 0x7a as ::core::ffi::c_int;
pub const ASCII_0: ::core::ffi::c_int = 0x30 as ::core::ffi::c_int;
pub const ASCII_1: ::core::ffi::c_int = 49;
pub const ASCII_2: ::core::ffi::c_int = 50;
pub const ASCII_3: ::core::ffi::c_int = 51;
pub const ASCII_4: ::core::ffi::c_int = 52;
pub const ASCII_5: ::core::ffi::c_int = 53;
pub const ASCII_6: ::core::ffi::c_int = 54;
pub const ASCII_7: ::core::ffi::c_int = 55;
pub const ASCII_8: ::core::ffi::c_int = 56;
pub const ASCII_9: ::core::ffi::c_int = 57;
pub const ASCII_SPACE: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
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
static mut internal_utf8_encoding: normal_encoding = unsafe {
    normal_encoding {
        enc: encoding {
            scanners: [
                Some(
                    normal_prologTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_contentTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_cdataSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_ignoreSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            literalScanners: [
                Some(
                    normal_attributeValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_entityValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                normal_nameMatchesAscii
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                normal_nameLength
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                normal_skipS
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                normal_getAtts
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                normal_charRefNumber
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                normal_predefinedEntityName
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                normal_updatePosition
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                normal_isPublicId
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(utf8_toUtf8),
            utf16Convert: Some(utf8_toUtf16),
            minBytesPerChar: 1 as ::core::ffi::c_int,
            isUtf8: 1 as ::core::ffi::c_char,
            isUtf16: 0 as ::core::ffi::c_char,
        },
        type_0: [
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        isName2: Some(utf8_isName2),
        isName3: Some(utf8_isName3),
        isName4: Some(isNever),
        isNmstrt2: Some(utf8_isNmstrt2),
        isNmstrt3: Some(utf8_isNmstrt3),
        isNmstrt4: Some(isNever),
        isInvalid2: Some(utf8_isInvalid2),
        isInvalid3: Some(utf8_isInvalid3),
        isInvalid4: Some(utf8_isInvalid4),
    }
};
extern "C" fn latin1_toUtf8(
    _enc: *const ENCODING,
    fromP: *mut *const ::core::ffi::c_char,
    fromLim: *const ::core::ffi::c_char,
    toP: *mut *mut ::core::ffi::c_char,
    toLim: *const ::core::ffi::c_char,
) -> XML_Convert_Result {
    loop {
        let from = read_copy(fromP);
        if from == fromLim {
            return XML_CONVERT_COMPLETED;
        }

        let c = read_c_char(from) as ::core::ffi::c_uchar;
        let to = read_copy(toP);
        if c as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
            if remaining_c_chars(to, toLim) < 2 {
                return XML_CONVERT_OUTPUT_EXHAUSTED;
            }

            write_copy(
                to,
                (c as ::core::ffi::c_int >> 6 as ::core::ffi::c_int
                    | UTF8_cval2 as ::core::ffi::c_int) as ::core::ffi::c_char,
            );
            write_copy(
                add_mut_c_char(to, 1),
                (c as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int)
                    as ::core::ffi::c_char,
            );
            write_copy(fromP, add_const_c_char(from, 1));
            write_copy(toP, add_mut_c_char(to, 2));
        } else {
            if to == toLim as *mut ::core::ffi::c_char {
                return XML_CONVERT_OUTPUT_EXHAUSTED;
            }

            write_copy(to, read_c_char(from));
            write_copy(fromP, add_const_c_char(from, 1));
            write_copy(toP, add_mut_c_char(to, 1));
        }
    }
}
extern "C" fn latin1_toUtf16(
    _enc: *const ENCODING,
    fromP: *mut *const ::core::ffi::c_char,
    fromLim: *const ::core::ffi::c_char,
    toP: *mut *mut ::core::ffi::c_ushort,
    toLim: *const ::core::ffi::c_ushort,
) -> XML_Convert_Result {
    let mut from = read_copy(fromP);
    let mut to = read_copy(toP);

    while from < fromLim && to < toLim as *mut ::core::ffi::c_ushort {
        write_copy(
            to,
            read_c_char(from) as ::core::ffi::c_uchar as ::core::ffi::c_ushort,
        );
        from = add_const_c_char(from, 1);
        to = add_mut_c_ushort(to, 1);
    }

    write_copy(fromP, from);
    write_copy(toP, to);
    if to == toLim as *mut ::core::ffi::c_ushort && from < fromLim {
        XML_CONVERT_OUTPUT_EXHAUSTED
    } else {
        XML_CONVERT_COMPLETED
    }
}
static mut latin1_encoding_ns: normal_encoding = unsafe {
    normal_encoding {
        enc: encoding {
            scanners: [
                Some(
                    normal_prologTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_contentTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_cdataSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_ignoreSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            literalScanners: [
                Some(
                    normal_attributeValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_entityValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                normal_nameMatchesAscii
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                normal_nameLength
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                normal_skipS
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                normal_getAtts
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                normal_charRefNumber
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                normal_predefinedEntityName
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                normal_updatePosition
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                normal_isPublicId
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(latin1_toUtf8),
            utf16Convert: Some(latin1_toUtf16),
            minBytesPerChar: 1 as ::core::ffi::c_int,
            isUtf8: 0 as ::core::ffi::c_char,
            isUtf16: 0 as ::core::ffi::c_char,
        },
        type_0: [
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        isName2: None,
        isName3: None,
        isName4: None,
        isNmstrt2: None,
        isNmstrt3: None,
        isNmstrt4: None,
        isInvalid2: None,
        isInvalid3: None,
        isInvalid4: None,
    }
};
static mut latin1_encoding: normal_encoding = unsafe {
    normal_encoding {
        enc: encoding {
            scanners: [
                Some(
                    normal_prologTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_contentTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_cdataSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_ignoreSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            literalScanners: [
                Some(
                    normal_attributeValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_entityValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                normal_nameMatchesAscii
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                normal_nameLength
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                normal_skipS
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                normal_getAtts
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                normal_charRefNumber
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                normal_predefinedEntityName
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                normal_updatePosition
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                normal_isPublicId
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(latin1_toUtf8),
            utf16Convert: Some(latin1_toUtf16),
            minBytesPerChar: 1 as ::core::ffi::c_int,
            isUtf8: 0 as ::core::ffi::c_char,
            isUtf16: 0 as ::core::ffi::c_char,
        },
        type_0: [
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        isName2: None,
        isName3: None,
        isName4: None,
        isNmstrt2: None,
        isNmstrt3: None,
        isNmstrt4: None,
        isInvalid2: None,
        isInvalid3: None,
        isInvalid4: None,
    }
};
extern "C" fn ascii_toUtf8(
    _enc: *const ENCODING,
    fromP: *mut *const ::core::ffi::c_char,
    fromLim: *const ::core::ffi::c_char,
    toP: *mut *mut ::core::ffi::c_char,
    toLim: *const ::core::ffi::c_char,
) -> XML_Convert_Result {
    let mut from = read_copy(fromP);
    let mut to = read_copy(toP);

    while from < fromLim && to < toLim as *mut ::core::ffi::c_char {
        write_copy(to, read_c_char(from));
        from = add_const_c_char(from, 1);
        to = add_mut_c_char(to, 1);
    }

    write_copy(fromP, from);
    write_copy(toP, to);
    if to == toLim as *mut ::core::ffi::c_char && from < fromLim {
        XML_CONVERT_OUTPUT_EXHAUSTED
    } else {
        XML_CONVERT_COMPLETED
    }
}
static mut ascii_encoding_ns: normal_encoding = unsafe {
    normal_encoding {
        enc: encoding {
            scanners: [
                Some(
                    normal_prologTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_contentTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_cdataSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_ignoreSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            literalScanners: [
                Some(
                    normal_attributeValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_entityValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                normal_nameMatchesAscii
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                normal_nameLength
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                normal_skipS
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                normal_getAtts
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                normal_charRefNumber
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                normal_predefinedEntityName
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                normal_updatePosition
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                normal_isPublicId
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(ascii_toUtf8),
            utf16Convert: Some(latin1_toUtf16),
            minBytesPerChar: 1 as ::core::ffi::c_int,
            isUtf8: 1 as ::core::ffi::c_char,
            isUtf16: 0 as ::core::ffi::c_char,
        },
        type_0: [
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        isName2: None,
        isName3: None,
        isName4: None,
        isNmstrt2: None,
        isNmstrt3: None,
        isNmstrt4: None,
        isInvalid2: None,
        isInvalid3: None,
        isInvalid4: None,
    }
};
static mut ascii_encoding: normal_encoding = unsafe {
    normal_encoding {
        enc: encoding {
            scanners: [
                Some(
                    normal_prologTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_contentTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_cdataSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_ignoreSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            literalScanners: [
                Some(
                    normal_attributeValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_entityValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                normal_nameMatchesAscii
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                normal_nameLength
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                normal_skipS
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                normal_getAtts
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                normal_charRefNumber
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                normal_predefinedEntityName
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                normal_updatePosition
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                normal_isPublicId
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(ascii_toUtf8),
            utf16Convert: Some(latin1_toUtf16),
            minBytesPerChar: 1 as ::core::ffi::c_int,
            isUtf8: 1 as ::core::ffi::c_char,
            isUtf16: 0 as ::core::ffi::c_char,
        },
        type_0: [
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        isName2: None,
        isName3: None,
        isName4: None,
        isNmstrt2: None,
        isNmstrt3: None,
        isNmstrt4: None,
        isInvalid2: None,
        isInvalid3: None,
        isInvalid4: None,
    }
};
fn unicode_byte_type(hi: ::core::ffi::c_char, lo: ::core::ffi::c_char) -> ::core::ffi::c_int {
    match hi as ::core::ffi::c_uchar as ::core::ffi::c_int {
        216 | 217 | 218 | 219 => BT_LEAD4 as ::core::ffi::c_int,
        220 | 221 | 222 | 223 => BT_TRAIL as ::core::ffi::c_int,
        255 => match lo as ::core::ffi::c_uchar as ::core::ffi::c_int {
            255 | 254 => BT_NONXML as ::core::ffi::c_int,
            _ => BT_NONASCII as ::core::ffi::c_int,
        },
        _ => BT_NONASCII as ::core::ffi::c_int,
    }
}
extern "C" fn little2_toUtf8(
    _enc: *const ENCODING,
    fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    toP: *mut *mut ::core::ffi::c_char,
    toLim: *const ::core::ffi::c_char,
) -> XML_Convert_Result {
    let mut from = read_copy(fromP);
    let mut to = read_copy(toP);
    fromLim = even_c_char_boundary(from, fromLim);

    while from < fromLim {
        let lo = read_c_uchar(from) as ::core::ffi::c_int;
        let hi = read_c_uchar(add_const_c_char(from, 1)) as ::core::ffi::c_int;

        match hi {
            0 if lo < 0x80 => {
                if to == toLim as *mut ::core::ffi::c_char {
                    write_copy(fromP, from);
                    write_copy(toP, to);
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }

                write_c_char_and_advance(&mut to, lo as ::core::ffi::c_char);
                from = add_const_c_char(from, 2);
            }
            1..=7 => {
                if remaining_c_chars(to, toLim) < 2 {
                    write_copy(fromP, from);
                    write_copy(toP, to);
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }

                write_c_char_and_advance(
                    &mut to,
                    (lo >> 6 | (hi << 2) | UTF8_cval2 as ::core::ffi::c_int) as ::core::ffi::c_char,
                );
                write_c_char_and_advance(&mut to, ((lo & 0x3f) | 0x80) as ::core::ffi::c_char);
                from = add_const_c_char(from, 2);
            }
            216..=219 => {
                if remaining_c_chars(to, toLim) < 4 {
                    write_copy(fromP, from);
                    write_copy(toP, to);
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                if remaining_const_c_chars(from, fromLim) < 4 {
                    write_copy(fromP, from);
                    write_copy(toP, to);
                    return XML_CONVERT_INPUT_INCOMPLETE;
                }

                let plane = ((hi & 0x3) << 2 | ((lo >> 6) & 0x3)) + 1;
                write_c_char_and_advance(
                    &mut to,
                    ((plane >> 2) | UTF8_cval4 as ::core::ffi::c_int) as ::core::ffi::c_char,
                );
                write_c_char_and_advance(
                    &mut to,
                    (((lo >> 2) & 0xf) | ((plane & 0x3) << 4) | 0x80) as ::core::ffi::c_char,
                );

                let next = add_const_c_char(from, 2);
                let lo2 = read_c_uchar(next) as ::core::ffi::c_int;
                let hi2 = read_c_uchar(add_const_c_char(next, 1)) as ::core::ffi::c_int;
                write_c_char_and_advance(
                    &mut to,
                    (((lo & 0x3) << 4) | ((hi2 & 0x3) << 2) | (lo2 >> 6) | 0x80)
                        as ::core::ffi::c_char,
                );
                write_c_char_and_advance(&mut to, ((lo2 & 0x3f) | 0x80) as ::core::ffi::c_char);
                from = add_const_c_char(from, 4);
            }
            _ => {
                if remaining_c_chars(to, toLim) < 3 {
                    write_copy(fromP, from);
                    write_copy(toP, to);
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }

                write_c_char_and_advance(
                    &mut to,
                    ((hi >> 4) | UTF8_cval3 as ::core::ffi::c_int) as ::core::ffi::c_char,
                );
                write_c_char_and_advance(
                    &mut to,
                    (((hi & 0xf) << 2) | (lo >> 6) | 0x80) as ::core::ffi::c_char,
                );
                write_c_char_and_advance(&mut to, ((lo & 0x3f) | 0x80) as ::core::ffi::c_char);
                from = add_const_c_char(from, 2);
            }
        }
    }

    write_copy(fromP, from);
    write_copy(toP, to);
    if from < fromLim {
        XML_CONVERT_INPUT_INCOMPLETE
    } else {
        XML_CONVERT_COMPLETED
    }
}
extern "C" fn little2_toUtf16(
    _enc: *const ENCODING,
    fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    toP: *mut *mut ::core::ffi::c_ushort,
    toLim: *const ::core::ffi::c_ushort,
) -> XML_Convert_Result {
    let mut res = XML_CONVERT_COMPLETED;
    let mut from = read_copy(fromP);
    let mut to = read_copy(toP);

    fromLim = even_c_char_boundary(from, fromLim);
    if remaining_const_c_chars(from, fromLim) > remaining_c_ushorts(to, toLim) << 1
        && read_c_char(add_const_c_char(fromLim, -1)) as ::core::ffi::c_uchar as ::core::ffi::c_int
            & 0xf8 as ::core::ffi::c_int
            == 0xd8 as ::core::ffi::c_int
    {
        fromLim = add_const_c_char(fromLim, -2);
        res = XML_CONVERT_INPUT_INCOMPLETE;
    }

    while from < fromLim && to < toLim as *mut ::core::ffi::c_ushort {
        let low = read_c_char(from) as ::core::ffi::c_uchar as ::core::ffi::c_int;
        let high =
            read_c_char(add_const_c_char(from, 1)) as ::core::ffi::c_uchar as ::core::ffi::c_int;
        write_copy(to, ((high << 8) | low) as ::core::ffi::c_ushort);
        from = add_const_c_char(from, 2);
        to = add_mut_c_ushort(to, 1);
    }

    write_copy(fromP, from);
    write_copy(toP, to);
    if to == toLim as *mut ::core::ffi::c_ushort && from < fromLim {
        XML_CONVERT_OUTPUT_EXHAUSTED
    } else {
        res
    }
}
extern "C" fn big2_toUtf8(
    _enc: *const ENCODING,
    fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    toP: *mut *mut ::core::ffi::c_char,
    toLim: *const ::core::ffi::c_char,
) -> XML_Convert_Result {
    let mut from = read_copy(fromP);
    let mut to = read_copy(toP);
    fromLim = even_c_char_boundary(from, fromLim);

    while from < fromLim {
        let hi = read_c_uchar(from) as ::core::ffi::c_int;
        let lo = read_c_uchar(add_const_c_char(from, 1)) as ::core::ffi::c_int;

        match hi {
            0 if lo < 0x80 => {
                if to == toLim as *mut ::core::ffi::c_char {
                    write_copy(fromP, from);
                    write_copy(toP, to);
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }

                write_c_char_and_advance(&mut to, lo as ::core::ffi::c_char);
                from = add_const_c_char(from, 2);
            }
            1..=7 => {
                if remaining_c_chars(to, toLim) < 2 {
                    write_copy(fromP, from);
                    write_copy(toP, to);
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }

                write_c_char_and_advance(
                    &mut to,
                    (lo >> 6 | (hi << 2) | UTF8_cval2 as ::core::ffi::c_int) as ::core::ffi::c_char,
                );
                write_c_char_and_advance(&mut to, ((lo & 0x3f) | 0x80) as ::core::ffi::c_char);
                from = add_const_c_char(from, 2);
            }
            216..=219 => {
                if remaining_c_chars(to, toLim) < 4 {
                    write_copy(fromP, from);
                    write_copy(toP, to);
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                if remaining_const_c_chars(from, fromLim) < 4 {
                    write_copy(fromP, from);
                    write_copy(toP, to);
                    return XML_CONVERT_INPUT_INCOMPLETE;
                }

                let plane = ((hi & 0x3) << 2 | ((lo >> 6) & 0x3)) + 1;
                write_c_char_and_advance(
                    &mut to,
                    ((plane >> 2) | UTF8_cval4 as ::core::ffi::c_int) as ::core::ffi::c_char,
                );
                write_c_char_and_advance(
                    &mut to,
                    (((lo >> 2) & 0xf) | ((plane & 0x3) << 4) | 0x80) as ::core::ffi::c_char,
                );

                let next = add_const_c_char(from, 2);
                let hi2 = read_c_uchar(next) as ::core::ffi::c_int;
                let lo2 = read_c_uchar(add_const_c_char(next, 1)) as ::core::ffi::c_int;
                write_c_char_and_advance(
                    &mut to,
                    (((lo & 0x3) << 4) | ((hi2 & 0x3) << 2) | (lo2 >> 6) | 0x80)
                        as ::core::ffi::c_char,
                );
                write_c_char_and_advance(&mut to, ((lo2 & 0x3f) | 0x80) as ::core::ffi::c_char);
                from = add_const_c_char(from, 4);
            }
            _ => {
                if remaining_c_chars(to, toLim) < 3 {
                    write_copy(fromP, from);
                    write_copy(toP, to);
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }

                write_c_char_and_advance(
                    &mut to,
                    ((hi >> 4) | UTF8_cval3 as ::core::ffi::c_int) as ::core::ffi::c_char,
                );
                write_c_char_and_advance(
                    &mut to,
                    (((hi & 0xf) << 2) | (lo >> 6) | 0x80) as ::core::ffi::c_char,
                );
                write_c_char_and_advance(&mut to, ((lo & 0x3f) | 0x80) as ::core::ffi::c_char);
                from = add_const_c_char(from, 2);
            }
        }
    }

    write_copy(fromP, from);
    write_copy(toP, to);
    if from < fromLim {
        XML_CONVERT_INPUT_INCOMPLETE
    } else {
        XML_CONVERT_COMPLETED
    }
}
extern "C" fn big2_toUtf16(
    _enc: *const ENCODING,
    fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    toP: *mut *mut ::core::ffi::c_ushort,
    toLim: *const ::core::ffi::c_ushort,
) -> XML_Convert_Result {
    let mut res = XML_CONVERT_COMPLETED;
    let mut from = read_copy(fromP);
    let mut to = read_copy(toP);

    fromLim = even_c_char_boundary(from, fromLim);
    if remaining_const_c_chars(from, fromLim) > remaining_c_ushorts(to, toLim) << 1
        && read_c_char(add_const_c_char(fromLim, -2)) as ::core::ffi::c_uchar as ::core::ffi::c_int
            & 0xf8 as ::core::ffi::c_int
            == 0xd8 as ::core::ffi::c_int
    {
        fromLim = add_const_c_char(fromLim, -2);
        res = XML_CONVERT_INPUT_INCOMPLETE;
    }

    while from < fromLim && to < toLim as *mut ::core::ffi::c_ushort {
        let high = read_c_char(from) as ::core::ffi::c_uchar as ::core::ffi::c_int;
        let low =
            read_c_char(add_const_c_char(from, 1)) as ::core::ffi::c_uchar as ::core::ffi::c_int;
        write_copy(to, ((high << 8) | low) as ::core::ffi::c_ushort);
        from = add_const_c_char(from, 2);
        to = add_mut_c_ushort(to, 1);
    }

    write_copy(fromP, from);
    write_copy(toP, to);
    if to == toLim as *mut ::core::ffi::c_ushort && from < fromLim {
        XML_CONVERT_OUTPUT_EXHAUSTED
    } else {
        res
    }
}
static mut little2_encoding_ns: normal_encoding = unsafe {
    normal_encoding {
        enc: encoding {
            scanners: [
                Some(
                    little2_prologTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_contentTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_cdataSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_ignoreSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            literalScanners: [
                Some(
                    little2_attributeValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_entityValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                little2_nameMatchesAscii
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                little2_nameLength
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                little2_skipS
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                little2_getAtts
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                little2_charRefNumber
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                little2_predefinedEntityName
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                little2_updatePosition
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                little2_isPublicId
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(little2_toUtf8),
            utf16Convert: Some(little2_toUtf16),
            minBytesPerChar: 2 as ::core::ffi::c_int,
            isUtf8: 0 as ::core::ffi::c_char,
            isUtf16: 1 as ::core::ffi::c_char,
        },
        type_0: [
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        isName2: None,
        isName3: None,
        isName4: None,
        isNmstrt2: None,
        isNmstrt3: None,
        isNmstrt4: None,
        isInvalid2: None,
        isInvalid3: None,
        isInvalid4: None,
    }
};
static mut little2_encoding: normal_encoding = unsafe {
    normal_encoding {
        enc: encoding {
            scanners: [
                Some(
                    little2_prologTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_contentTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_cdataSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_ignoreSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            literalScanners: [
                Some(
                    little2_attributeValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_entityValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                little2_nameMatchesAscii
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                little2_nameLength
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                little2_skipS
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                little2_getAtts
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                little2_charRefNumber
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                little2_predefinedEntityName
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                little2_updatePosition
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                little2_isPublicId
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(little2_toUtf8),
            utf16Convert: Some(little2_toUtf16),
            minBytesPerChar: 2 as ::core::ffi::c_int,
            isUtf8: 0 as ::core::ffi::c_char,
            isUtf16: 1 as ::core::ffi::c_char,
        },
        type_0: [
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        isName2: None,
        isName3: None,
        isName4: None,
        isNmstrt2: None,
        isNmstrt3: None,
        isNmstrt4: None,
        isInvalid2: None,
        isInvalid3: None,
        isInvalid4: None,
    }
};
static mut internal_little2_encoding_ns: normal_encoding = unsafe {
    normal_encoding {
        enc: encoding {
            scanners: [
                Some(
                    little2_prologTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_contentTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_cdataSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_ignoreSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            literalScanners: [
                Some(
                    little2_attributeValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_entityValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                little2_nameMatchesAscii
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                little2_nameLength
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                little2_skipS
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                little2_getAtts
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                little2_charRefNumber
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                little2_predefinedEntityName
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                little2_updatePosition
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                little2_isPublicId
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(little2_toUtf8),
            utf16Convert: Some(little2_toUtf16),
            minBytesPerChar: 2 as ::core::ffi::c_int,
            isUtf8: 0 as ::core::ffi::c_char,
            isUtf16: 1 as ::core::ffi::c_char,
        },
        type_0: [
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        isName2: None,
        isName3: None,
        isName4: None,
        isNmstrt2: None,
        isNmstrt3: None,
        isNmstrt4: None,
        isInvalid2: None,
        isInvalid3: None,
        isInvalid4: None,
    }
};
static mut internal_little2_encoding: normal_encoding = unsafe {
    normal_encoding {
        enc: encoding {
            scanners: [
                Some(
                    little2_prologTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_contentTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_cdataSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_ignoreSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            literalScanners: [
                Some(
                    little2_attributeValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_entityValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                little2_nameMatchesAscii
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                little2_nameLength
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                little2_skipS
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                little2_getAtts
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                little2_charRefNumber
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                little2_predefinedEntityName
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                little2_updatePosition
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                little2_isPublicId
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(little2_toUtf8),
            utf16Convert: Some(little2_toUtf16),
            minBytesPerChar: 2 as ::core::ffi::c_int,
            isUtf8: 0 as ::core::ffi::c_char,
            isUtf16: 1 as ::core::ffi::c_char,
        },
        type_0: [
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        isName2: None,
        isName3: None,
        isName4: None,
        isNmstrt2: None,
        isNmstrt3: None,
        isNmstrt4: None,
        isInvalid2: None,
        isInvalid3: None,
        isInvalid4: None,
    }
};
static mut big2_encoding_ns: normal_encoding = unsafe {
    normal_encoding {
        enc: encoding {
            scanners: [
                Some(
                    big2_prologTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    big2_contentTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    big2_cdataSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    big2_ignoreSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            literalScanners: [
                Some(
                    big2_attributeValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    big2_entityValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                big2_nameMatchesAscii
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                big2_nameLength
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                big2_skipS
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                big2_getAtts
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                big2_charRefNumber
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                big2_predefinedEntityName
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                big2_updatePosition
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                big2_isPublicId
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(big2_toUtf8),
            utf16Convert: Some(big2_toUtf16),
            minBytesPerChar: 2 as ::core::ffi::c_int,
            isUtf8: 0 as ::core::ffi::c_char,
            isUtf16: 0 as ::core::ffi::c_char,
        },
        type_0: [
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        isName2: None,
        isName3: None,
        isName4: None,
        isNmstrt2: None,
        isNmstrt3: None,
        isNmstrt4: None,
        isInvalid2: None,
        isInvalid3: None,
        isInvalid4: None,
    }
};
static mut big2_encoding: normal_encoding = unsafe {
    normal_encoding {
        enc: encoding {
            scanners: [
                Some(
                    big2_prologTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    big2_contentTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    big2_cdataSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    big2_ignoreSectionTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            literalScanners: [
                Some(
                    big2_attributeValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    big2_entityValueTok
                        as extern "C" fn(
                            *const ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                big2_nameMatchesAscii
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                big2_nameLength
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                big2_skipS
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                big2_getAtts
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                big2_charRefNumber
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                big2_predefinedEntityName
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                big2_updatePosition
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                big2_isPublicId
                    as extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(big2_toUtf8),
            utf16Convert: Some(big2_toUtf16),
            minBytesPerChar: 2 as ::core::ffi::c_int,
            isUtf8: 0 as ::core::ffi::c_char,
            isUtf16: 0 as ::core::ffi::c_char,
        },
        type_0: [
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
            BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        isName2: None,
        isName3: None,
        isName4: None,
        isNmstrt2: None,
        isNmstrt3: None,
        isNmstrt4: None,
        isInvalid2: None,
        isInvalid3: None,
        isInvalid4: None,
    }
};
fn streqci(
    mut s1: *const ::core::ffi::c_char,
    mut s2: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    loop {
        let mut c1 = read_c_char(s1);
        s1 = add_const_c_char(s1, 1);
        let mut c2 = read_c_char(s2);
        s2 = add_const_c_char(s2, 1);
        if ASCII_a <= c1 as ::core::ffi::c_int && c1 as ::core::ffi::c_int <= ASCII_z {
            c1 = (c1 as ::core::ffi::c_int + (ASCII_A - ASCII_a)) as ::core::ffi::c_char;
        }
        if ASCII_a <= c2 as ::core::ffi::c_int && c2 as ::core::ffi::c_int <= ASCII_z {
            c2 = (c2 as ::core::ffi::c_int + (ASCII_A - ASCII_a)) as ::core::ffi::c_char;
        }
        if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if c1 == 0 {
            return 1 as ::core::ffi::c_int;
        }
    }
}

extern "C" fn initUpdatePosition(
    _enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    pos: *mut POSITION,
) {
    update_position_with_utf8(ptr, end, pos);
}

fn toAscii(
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_char; 1] = [0; 1];
    let mut out = buf.as_mut_ptr();
    encoding_utf8_convert(
        enc,
        &mut ptr,
        end,
        &mut out,
        add_const_c_char(out.cast_const(), 1),
    );
    if out == buf.as_mut_ptr() {
        -(1 as ::core::ffi::c_int)
    } else {
        buf[0] as ::core::ffi::c_int
    }
}
fn isSpace(c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    match c {
        32 | 13 | 10 | 9 => 1 as ::core::ffi::c_int,
        _ => 0 as ::core::ffi::c_int,
    }
}
fn parsePseudoAttribute(
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    namePtr: *mut *const ::core::ffi::c_char,
    nameEndPtr: *mut *const ::core::ffi::c_char,
    valPtr: *mut *const ::core::ffi::c_char,
    nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let min_bytes_per_char = encoding_min_bytes_per_char(enc);
    let mut c: ::core::ffi::c_int = 0;
    let mut open: ::core::ffi::c_char = 0;
    if ptr == end {
        write_copy(namePtr, ::core::ptr::null::<::core::ffi::c_char>());
        return 1 as ::core::ffi::c_int;
    }
    if isSpace(toAscii(enc, ptr, end)) == 0 {
        write_copy(nextTokPtr, ptr);
        return 0 as ::core::ffi::c_int;
    }
    loop {
        ptr = add_const_c_char(ptr, min_bytes_per_char);
        if isSpace(toAscii(enc, ptr, end)) == 0 {
            break;
        }
    }
    if ptr == end {
        write_copy(namePtr, ::core::ptr::null::<::core::ffi::c_char>());
        return 1 as ::core::ffi::c_int;
    }
    write_copy(namePtr, ptr);
    loop {
        c = toAscii(enc, ptr, end);
        if c == -(1 as ::core::ffi::c_int) {
            write_copy(nextTokPtr, ptr);
            return 0 as ::core::ffi::c_int;
        }
        if c == ASCII_EQUALS {
            write_copy(nameEndPtr, ptr);
            break;
        } else if isSpace(c) != 0 {
            write_copy(nameEndPtr, ptr);
            loop {
                ptr = add_const_c_char(ptr, min_bytes_per_char);
                c = toAscii(enc, ptr, end);
                if isSpace(c) == 0 {
                    break;
                }
            }
            if c != ASCII_EQUALS {
                write_copy(nextTokPtr, ptr);
                return 0 as ::core::ffi::c_int;
            }
            break;
        } else {
            ptr = add_const_c_char(ptr, min_bytes_per_char);
        }
    }
    if ptr == read_copy(namePtr.cast_const()) {
        write_copy(nextTokPtr, ptr);
        return 0 as ::core::ffi::c_int;
    }
    ptr = add_const_c_char(ptr, min_bytes_per_char);
    c = toAscii(enc, ptr, end);
    while isSpace(c) != 0 {
        ptr = add_const_c_char(ptr, min_bytes_per_char);
        c = toAscii(enc, ptr, end);
    }
    if c != ASCII_QUOT && c != ASCII_APOS {
        write_copy(nextTokPtr, ptr);
        return 0 as ::core::ffi::c_int;
    }
    open = c as ::core::ffi::c_char;
    ptr = add_const_c_char(ptr, min_bytes_per_char);
    write_copy(valPtr, ptr);
    loop {
        c = toAscii(enc, ptr, end);
        if c == open as ::core::ffi::c_int {
            break;
        }
        if !(ASCII_a <= c && c <= ASCII_z)
            && !(ASCII_A <= c && c <= ASCII_Z)
            && !(ASCII_0 <= c && c <= ASCII_9)
            && c != ASCII_PERIOD
            && c != ASCII_MINUS
            && c != ASCII_UNDERSCORE
        {
            write_copy(nextTokPtr, ptr);
            return 0 as ::core::ffi::c_int;
        }
        ptr = add_const_c_char(ptr, min_bytes_per_char);
    }
    write_copy(nextTokPtr, add_const_c_char(ptr, min_bytes_per_char));
    1 as ::core::ffi::c_int
}
static KW_version: [::core::ffi::c_char; 8] = [
    ASCII_v as ::core::ffi::c_char,
    ASCII_e as ::core::ffi::c_char,
    ASCII_r as ::core::ffi::c_char,
    ASCII_s as ::core::ffi::c_char,
    ASCII_i as ::core::ffi::c_char,
    ASCII_o as ::core::ffi::c_char,
    ASCII_n as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_encoding: [::core::ffi::c_char; 9] = [
    ASCII_e as ::core::ffi::c_char,
    ASCII_n as ::core::ffi::c_char,
    ASCII_c as ::core::ffi::c_char,
    ASCII_o as ::core::ffi::c_char,
    ASCII_d as ::core::ffi::c_char,
    ASCII_i as ::core::ffi::c_char,
    ASCII_n as ::core::ffi::c_char,
    ASCII_g as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_standalone: [::core::ffi::c_char; 11] = [
    ASCII_s as ::core::ffi::c_char,
    ASCII_t as ::core::ffi::c_char,
    ASCII_a as ::core::ffi::c_char,
    ASCII_n as ::core::ffi::c_char,
    ASCII_d as ::core::ffi::c_char,
    ASCII_a as ::core::ffi::c_char,
    ASCII_l as ::core::ffi::c_char,
    ASCII_o as ::core::ffi::c_char,
    ASCII_n as ::core::ffi::c_char,
    ASCII_e as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_yes: [::core::ffi::c_char; 4] = [
    ASCII_y as ::core::ffi::c_char,
    ASCII_e as ::core::ffi::c_char,
    ASCII_s as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_no: [::core::ffi::c_char; 3] = [
    ASCII_n as ::core::ffi::c_char,
    ASCII_o as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
fn doParseXmlDecl(
    encodingFinder: Option<
        extern "C" fn(
            *const ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> *const ENCODING,
    >,
    isGeneralTextEntity: ::core::ffi::c_int,
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    badPtr: *mut *const ::core::ffi::c_char,
    versionPtr: *mut *const ::core::ffi::c_char,
    versionEndPtr: *mut *const ::core::ffi::c_char,
    encodingName: *mut *const ::core::ffi::c_char,
    encoding: *mut *const ENCODING,
    standalone: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let min_bytes_per_char = encoding_min_bytes_per_char(enc);
    let mut val: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut nameEnd: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    ptr = add_const_c_char(ptr, 5 * min_bytes_per_char);
    end = add_const_c_char(end, -(2 * min_bytes_per_char));
    if parsePseudoAttribute(
        enc,
        ptr,
        end,
        &raw mut name,
        &raw mut nameEnd,
        &raw mut val,
        &raw mut ptr,
    ) == 0
        || name.is_null()
    {
        write_copy(badPtr, ptr);
        return 0 as ::core::ffi::c_int;
    }
    if encoding_name_matches_ascii(enc, name, nameEnd, KW_version.as_ptr()) == 0 {
        if isGeneralTextEntity == 0 {
            write_copy(badPtr, name);
            return 0 as ::core::ffi::c_int;
        }
    } else {
        if !versionPtr.is_null() {
            write_copy(versionPtr, val);
        }
        if !versionEndPtr.is_null() {
            write_copy(versionEndPtr, ptr);
        }
        if parsePseudoAttribute(
            enc,
            ptr,
            end,
            &raw mut name,
            &raw mut nameEnd,
            &raw mut val,
            &raw mut ptr,
        ) == 0
        {
            write_copy(badPtr, ptr);
            return 0 as ::core::ffi::c_int;
        }
        if name.is_null() {
            if isGeneralTextEntity != 0 {
                write_copy(badPtr, ptr);
                return 0 as ::core::ffi::c_int;
            }
            return 1 as ::core::ffi::c_int;
        }
    }
    if encoding_name_matches_ascii(enc, name, nameEnd, KW_encoding.as_ptr()) != 0 {
        let c = toAscii(enc, val, end);
        if !(ASCII_a <= c && c <= ASCII_z) && !(ASCII_A <= c && c <= ASCII_Z) {
            write_copy(badPtr, val);
            return 0 as ::core::ffi::c_int;
        }
        if !encodingName.is_null() {
            write_copy(encodingName, val);
        }
        if !encoding.is_null() {
            write_copy(
                encoding,
                call_encoding_finder(
                    encodingFinder.expect("non-null function pointer"),
                    enc,
                    val,
                    add_const_c_char(ptr, -min_bytes_per_char),
                ),
            );
        }
        if parsePseudoAttribute(
            enc,
            ptr,
            end,
            &raw mut name,
            &raw mut nameEnd,
            &raw mut val,
            &raw mut ptr,
        ) == 0
        {
            write_copy(badPtr, ptr);
            return 0 as ::core::ffi::c_int;
        }
        if name.is_null() {
            return 1 as ::core::ffi::c_int;
        }
    }
    if encoding_name_matches_ascii(enc, name, nameEnd, KW_standalone.as_ptr()) == 0
        || isGeneralTextEntity != 0
    {
        write_copy(badPtr, name);
        return 0 as ::core::ffi::c_int;
    }
    let value_end = add_const_c_char(ptr, -min_bytes_per_char);
    if encoding_name_matches_ascii(enc, val, value_end, KW_yes.as_ptr()) != 0 {
        if !standalone.is_null() {
            write_copy(standalone, 1 as ::core::ffi::c_int);
        }
    } else if encoding_name_matches_ascii(enc, val, value_end, KW_no.as_ptr()) != 0 {
        if !standalone.is_null() {
            write_copy(standalone, 0 as ::core::ffi::c_int);
        }
    } else {
        write_copy(badPtr, val);
        return 0 as ::core::ffi::c_int;
    }
    while isSpace(toAscii(enc, ptr, end)) != 0 {
        ptr = add_const_c_char(ptr, min_bytes_per_char);
    }
    if ptr != end {
        write_copy(badPtr, ptr);
        return 0 as ::core::ffi::c_int;
    }
    1 as ::core::ffi::c_int
}
fn checkCharRefNumber(result: ::core::ffi::c_int) -> ::core::ffi::c_int {
    match result >> 8 as ::core::ffi::c_int {
        216 | 217 | 218 | 219 | 220 | 221 | 222 | 223 => -(1 as ::core::ffi::c_int),
        0 => {
            if matches!(
                result,
                0..=0x8 | 0xB | 0xC | 0xE..=0x1F
            ) {
                -(1 as ::core::ffi::c_int)
            } else {
                result
            }
        }
        255 => {
            if result == 0xfffe as ::core::ffi::c_int || result == 0xffff as ::core::ffi::c_int {
                -(1 as ::core::ffi::c_int)
            } else {
                result
            }
        }
        _ => result,
    }
}
#[no_mangle]
pub unsafe extern "C" fn XmlUtf8Encode(
    mut c: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    encode_utf8_bytes(c, unsafe { &mut *(buf as *mut [::core::ffi::c_char; 4]) })
}
#[no_mangle]
pub unsafe extern "C" fn XmlUtf16Encode(
    mut charNum: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_ushort,
) -> ::core::ffi::c_int {
    unsafe {
        if charNum < 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if charNum < 0x10000 as ::core::ffi::c_int {
            *buf.offset(0 as ::core::ffi::c_int as isize) = charNum as ::core::ffi::c_ushort;
            return 1 as ::core::ffi::c_int;
        }
        if charNum < 0x110000 as ::core::ffi::c_int {
            charNum -= 0x10000 as ::core::ffi::c_int;
            *buf.offset(0 as ::core::ffi::c_int as isize) = ((charNum >> 10 as ::core::ffi::c_int)
                + 0xd800 as ::core::ffi::c_int)
                as ::core::ffi::c_ushort;
            *buf.offset(1 as ::core::ffi::c_int as isize) =
                ((charNum & 0x3ff as ::core::ffi::c_int) + 0xdc00 as ::core::ffi::c_int)
                    as ::core::ffi::c_ushort;
            return 2 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XmlSizeOfUnknownEncoding() -> ::core::ffi::c_int {
    ::core::mem::size_of::<unknown_encoding>() as ::core::ffi::c_int
}
fn unknown_code_point(enc: *const ENCODING, p: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    with_ref(enc.cast::<unknown_encoding>(), |uenc| {
        call_unknown_converter(uenc, p)
    })
}
extern "C" fn unknown_isName(
    enc: *const ENCODING,
    p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let code_point = unknown_code_point(enc, p);
    if code_point & !(0xffff as ::core::ffi::c_int) != 0 {
        return 0;
    }
    let page = namePages[(code_point >> 8) as usize] as usize;
    let bitmap_index = (page << 3) + (((code_point & 0xff) >> 5) as usize);
    ((namingBitmap[bitmap_index] & (1 as ::core::ffi::c_uint) << (code_point & 0x1f)) != 0)
        as ::core::ffi::c_int
}
extern "C" fn unknown_isNmstrt(
    enc: *const ENCODING,
    p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let code_point = unknown_code_point(enc, p);
    if code_point & !(0xffff as ::core::ffi::c_int) != 0 {
        return 0;
    }
    let page = nmstrtPages[(code_point >> 8) as usize] as usize;
    let bitmap_index = (page << 3) + (((code_point & 0xff) >> 5) as usize);
    ((namingBitmap[bitmap_index] & (1 as ::core::ffi::c_uint) << (code_point & 0x1f)) != 0)
        as ::core::ffi::c_int
}

extern "C" fn unknown_isInvalid(
    enc: *const ENCODING,
    p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let code_point = unknown_code_point(enc, p);
    (code_point & !(0xffff as ::core::ffi::c_int) != 0
        || checkCharRefNumber(code_point) < 0 as ::core::ffi::c_int) as ::core::ffi::c_int
}
extern "C" fn unknown_toUtf8(
    enc: *const ENCODING,
    fromP: *mut *const ::core::ffi::c_char,
    fromLim: *const ::core::ffi::c_char,
    toP: *mut *mut ::core::ffi::c_char,
    toLim: *const ::core::ffi::c_char,
) -> XML_Convert_Result {
    with_ref(enc.cast::<unknown_encoding>(), |uenc| {
        let mut buf: [::core::ffi::c_char; 4] = [0; 4];
        loop {
            let from = read_copy(fromP.cast_const());
            if from == fromLim {
                break XML_CONVERT_COMPLETED;
            }
            let byte = read_c_char(from) as ::core::ffi::c_uchar as usize;
            let utf8_entry = read_copy(uenc.utf8.as_ptr().wrapping_add(byte));
            let mut n = utf8_entry[0] as ::core::ffi::c_int;
            let mut utf8 = utf8_entry.as_ptr().wrapping_add(1);
            let to = read_copy(toP.cast_const());
            if n == 0 as ::core::ffi::c_int {
                let c = call_unknown_converter(uenc, from);
                n = call_xml_utf8_encode(c, &mut buf);
                if n as usize > remaining_c_chars(to, toLim) {
                    break XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                utf8 = buf.as_ptr();
                write_copy(
                    fromP,
                    add_const_c_char(from, unknown_sequence_length(enc, from)),
                );
            } else {
                if n as usize > remaining_c_chars(to, toLim) {
                    break XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                write_copy(fromP, add_const_c_char(from, 1));
            }
            copy_c_chars(to, utf8, n as size_t);
            write_copy(toP, add_mut_c_char(to, n as isize));
        }
    })
}
extern "C" fn unknown_toUtf16(
    enc: *const ENCODING,
    fromP: *mut *const ::core::ffi::c_char,
    fromLim: *const ::core::ffi::c_char,
    toP: *mut *mut ::core::ffi::c_ushort,
    toLim: *const ::core::ffi::c_ushort,
) -> XML_Convert_Result {
    with_ref(enc.cast::<unknown_encoding>(), |uenc| {
        while read_copy(fromP.cast_const()) < fromLim
            && read_copy(toP.cast_const()) < toLim.cast_mut()
        {
            let from = read_copy(fromP.cast_const());
            let byte = read_c_char(from) as ::core::ffi::c_uchar as usize;
            let mut c = uenc.utf16[byte];
            if c as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                c = call_unknown_converter(uenc, from) as ::core::ffi::c_ushort;
                write_copy(
                    fromP,
                    add_const_c_char(from, unknown_sequence_length(enc, from)),
                );
            } else {
                write_copy(fromP, add_const_c_char(from, 1));
            }
            let to = read_copy(toP.cast_const());
            write_copy(to, c);
            write_copy(toP, add_mut_c_ushort(to, 1));
        }
        if read_copy(toP.cast_const()) == toLim.cast_mut()
            && read_copy(fromP.cast_const()) < fromLim
        {
            XML_CONVERT_OUTPUT_EXHAUSTED
        } else {
            XML_CONVERT_COMPLETED
        }
    })
}
#[no_mangle]
pub unsafe extern "C" fn XmlInitUnknownEncoding(
    mut mem: *mut ::core::ffi::c_void,
    mut table: *const ::core::ffi::c_int,
    mut convert: CONVERTER,
    mut userData: *mut ::core::ffi::c_void,
) -> *mut ENCODING {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut e: *mut unknown_encoding = mem as *mut unknown_encoding;
        write_copy(e.cast::<normal_encoding>(), latin1_encoding);
        i = 0 as ::core::ffi::c_int;
        while i < 128 as ::core::ffi::c_int {
            if latin1_encoding.type_0[i as usize] as ::core::ffi::c_int
                != BT_OTHER as ::core::ffi::c_int
                && latin1_encoding.type_0[i as usize] as ::core::ffi::c_int
                    != BT_NONXML as ::core::ffi::c_int
                && *table.offset(i as isize) != i
            {
                return ::core::ptr::null_mut::<ENCODING>();
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < 256 as ::core::ffi::c_int {
            let mut c: ::core::ffi::c_int = *table.offset(i as isize);
            if c == -(1 as ::core::ffi::c_int) {
                (*e).normal.type_0[i as usize] =
                    BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar;
                (*e).utf16[i as usize] = 0xffff as ::core::ffi::c_ushort;
                (*e).utf8[i as usize][0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_char;
                (*e).utf8[i as usize][1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
            } else if c < 0 as ::core::ffi::c_int {
                if c < -(4 as ::core::ffi::c_int) {
                    return ::core::ptr::null_mut::<ENCODING>();
                }
                if convert.is_none() {
                    return ::core::ptr::null_mut::<ENCODING>();
                }
                (*e).normal.type_0[i as usize] = (BT_LEAD2 as ::core::ffi::c_int
                    - (c + 2 as ::core::ffi::c_int))
                    as ::core::ffi::c_uchar;
                (*e).utf8[i as usize][0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
                (*e).utf16[i as usize] = 0 as ::core::ffi::c_ushort;
            } else if c < 0x80 as ::core::ffi::c_int {
                if latin1_encoding.type_0[c as usize] as ::core::ffi::c_int
                    != BT_OTHER as ::core::ffi::c_int
                    && latin1_encoding.type_0[c as usize] as ::core::ffi::c_int
                        != BT_NONXML as ::core::ffi::c_int
                    && c != i
                {
                    return ::core::ptr::null_mut::<ENCODING>();
                }
                (*e).normal.type_0[i as usize] = latin1_encoding.type_0[c as usize];
                (*e).utf8[i as usize][0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_char;
                (*e).utf8[i as usize][1 as ::core::ffi::c_int as usize] = c as ::core::ffi::c_char;
                (*e).utf16[i as usize] = (if c == 0 as ::core::ffi::c_int {
                    0xffff as ::core::ffi::c_int
                } else {
                    c
                }) as ::core::ffi::c_ushort;
            } else if checkCharRefNumber(c) < 0 as ::core::ffi::c_int {
                (*e).normal.type_0[i as usize] =
                    BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar;
                (*e).utf16[i as usize] = 0xffff as ::core::ffi::c_ushort;
                (*e).utf8[i as usize][0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_char;
                (*e).utf8[i as usize][1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
            } else {
                if c > 0xffff as ::core::ffi::c_int {
                    return ::core::ptr::null_mut::<ENCODING>();
                }
                if namingBitmap[(((nmstrtPages[(c >> 8 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int)
                    << 3 as ::core::ffi::c_int)
                    + ((c & 0xff as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int))
                    as usize]
                    & (1 as ::core::ffi::c_uint)
                        << (c & 0xff as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int)
                    != 0
                {
                    (*e).normal.type_0[i as usize] =
                        BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar;
                } else if namingBitmap[(((namePages[(c >> 8 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int)
                    << 3 as ::core::ffi::c_int)
                    + ((c & 0xff as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int))
                    as usize]
                    & (1 as ::core::ffi::c_uint)
                        << (c & 0xff as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int)
                    != 0
                {
                    (*e).normal.type_0[i as usize] =
                        BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar;
                } else {
                    (*e).normal.type_0[i as usize] =
                        BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar;
                }
                (*e).utf8[i as usize][0 as ::core::ffi::c_int as usize] = XmlUtf8Encode(
                    c,
                    (&raw mut *(&raw mut (*e).utf8 as *mut [::core::ffi::c_char; 4])
                        .offset(i as isize) as *mut ::core::ffi::c_char)
                        .offset(1 as ::core::ffi::c_int as isize),
                )
                    as ::core::ffi::c_char;
                (*e).utf16[i as usize] = c as ::core::ffi::c_ushort;
            }
            i += 1;
        }
        (*e).userData = userData;
        (*e).convert = convert;
        if convert.is_some() {
            (*e).normal.isName2 = Some(unknown_isName);
            (*e).normal.isName3 = Some(unknown_isName);
            (*e).normal.isName4 = Some(unknown_isName);
            (*e).normal.isNmstrt2 = Some(unknown_isNmstrt);
            (*e).normal.isNmstrt3 = Some(unknown_isNmstrt);
            (*e).normal.isNmstrt4 = Some(unknown_isNmstrt);
            (*e).normal.isInvalid2 = Some(unknown_isInvalid);
            (*e).normal.isInvalid3 = Some(unknown_isInvalid);
            (*e).normal.isInvalid4 = Some(unknown_isInvalid);
        }
        (*e).normal.enc.utf8Convert = Some(unknown_toUtf8);
        (*e).normal.enc.utf16Convert = Some(unknown_toUtf16);
        return &raw mut (*e).normal.enc;
    }
}
static KW_ISO_8859_1: [::core::ffi::c_char; 11] = [
    ASCII_I as ::core::ffi::c_char,
    ASCII_S as ::core::ffi::c_char,
    ASCII_O as ::core::ffi::c_char,
    ASCII_MINUS as ::core::ffi::c_char,
    ASCII_8 as ::core::ffi::c_char,
    ASCII_8 as ::core::ffi::c_char,
    ASCII_5 as ::core::ffi::c_char,
    ASCII_9 as ::core::ffi::c_char,
    ASCII_MINUS as ::core::ffi::c_char,
    ASCII_1 as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_US_ASCII: [::core::ffi::c_char; 9] = [
    ASCII_U as ::core::ffi::c_char,
    ASCII_S as ::core::ffi::c_char,
    ASCII_MINUS as ::core::ffi::c_char,
    ASCII_A as ::core::ffi::c_char,
    ASCII_S as ::core::ffi::c_char,
    ASCII_C as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_UTF_8: [::core::ffi::c_char; 6] = [
    ASCII_U as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_F as ::core::ffi::c_char,
    ASCII_MINUS as ::core::ffi::c_char,
    ASCII_8 as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_UTF_16: [::core::ffi::c_char; 7] = [
    ASCII_U as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_F as ::core::ffi::c_char,
    ASCII_MINUS as ::core::ffi::c_char,
    ASCII_1 as ::core::ffi::c_char,
    ASCII_6 as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_UTF_16BE: [::core::ffi::c_char; 9] = [
    ASCII_U as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_F as ::core::ffi::c_char,
    ASCII_MINUS as ::core::ffi::c_char,
    ASCII_1 as ::core::ffi::c_char,
    ASCII_6 as ::core::ffi::c_char,
    ASCII_B as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static KW_UTF_16LE: [::core::ffi::c_char; 9] = [
    ASCII_U as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_F as ::core::ffi::c_char,
    ASCII_MINUS as ::core::ffi::c_char,
    ASCII_1 as ::core::ffi::c_char,
    ASCII_6 as ::core::ffi::c_char,
    ASCII_L as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
fn getEncodingIndex(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let encoding_names = [
        KW_ISO_8859_1.as_ptr(),
        KW_US_ASCII.as_ptr(),
        KW_UTF_8.as_ptr(),
        KW_UTF_16.as_ptr(),
        KW_UTF_16BE.as_ptr(),
        KW_UTF_16LE.as_ptr(),
    ];
    if name.is_null() {
        return NO_ENC as ::core::ffi::c_int;
    }
    for (index, encoding_name) in encoding_names.iter().enumerate() {
        if streqci(name, *encoding_name) != 0 {
            return index as ::core::ffi::c_int;
        }
    }
    UNKNOWN_ENC as ::core::ffi::c_int
}
fn initScan(
    encoding_table: *const *const ENCODING,
    enc: *const INIT_ENCODING,
    state: ::core::ffi::c_int,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if ptr >= end {
        return XML_TOK_NONE;
    }

    let enc_ptr = init_encoding_ptr_slot(enc);
    let initial_encoding = init_encoding_kind(enc);

    if add_const_c_char(ptr, 1) == end {
        match initial_encoding {
            UTF_16_ENC | UTF_16LE_ENC | UTF_16BE_ENC => return XML_TOK_PARTIAL,
            _ => {}
        }

        let byte = read_c_char(ptr) as ::core::ffi::c_uchar as ::core::ffi::c_int;
        let requires_second_byte = matches!(byte, 0xfe | 0xff | 0xef | 0x00 | 0x3c);
        if requires_second_byte
            && !(initial_encoding == ISO_8859_1_ENC && state == XML_CONTENT_STATE)
        {
            return XML_TOK_PARTIAL;
        }
    } else {
        let first = read_c_char(ptr) as ::core::ffi::c_uchar as ::core::ffi::c_int;
        let second =
            read_c_char(add_const_c_char(ptr, 1)) as ::core::ffi::c_uchar as ::core::ffi::c_int;

        match (first << 8) | second {
            65279 => {
                if !(initial_encoding == ISO_8859_1_ENC && state == XML_CONTENT_STATE) {
                    set_next_token_ptr(next_tok_ptr, add_const_c_char(ptr, 2));
                    set_encoding_ptr(
                        enc_ptr,
                        read_encoding_table_entry(encoding_table, UTF_16BE_ENC as usize),
                    );
                    return XML_TOK_BOM;
                }
            }
            15360 => {
                if !((initial_encoding == UTF_16BE_ENC || initial_encoding == UTF_16_ENC)
                    && state == XML_CONTENT_STATE)
                {
                    set_encoding_ptr(
                        enc_ptr,
                        read_encoding_table_entry(encoding_table, UTF_16LE_ENC as usize),
                    );
                    return dispatch_scanner(enc_ptr, state, ptr, end, next_tok_ptr);
                }
            }
            65534 => {
                if !(initial_encoding == ISO_8859_1_ENC && state == XML_CONTENT_STATE) {
                    set_next_token_ptr(next_tok_ptr, add_const_c_char(ptr, 2));
                    set_encoding_ptr(
                        enc_ptr,
                        read_encoding_table_entry(encoding_table, UTF_16LE_ENC as usize),
                    );
                    return XML_TOK_BOM;
                }
            }
            61371 => {
                let known_utf16_content = state == XML_CONTENT_STATE
                    && matches!(
                        initial_encoding,
                        ISO_8859_1_ENC | UTF_16BE_ENC | UTF_16LE_ENC | UTF_16_ENC
                    );
                if !known_utf16_content {
                    if add_const_c_char(ptr, 2) == end {
                        return XML_TOK_PARTIAL;
                    }
                    let third = read_c_char(add_const_c_char(ptr, 2)) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int;
                    if third == 0xbf {
                        set_next_token_ptr(next_tok_ptr, add_const_c_char(ptr, 3));
                        set_encoding_ptr(
                            enc_ptr,
                            read_encoding_table_entry(encoding_table, UTF_8_ENC as usize),
                        );
                        return XML_TOK_BOM;
                    }
                }
            }
            _ => {
                if first == 0 {
                    if !(state == XML_CONTENT_STATE && initial_encoding == UTF_16LE_ENC) {
                        set_encoding_ptr(
                            enc_ptr,
                            read_encoding_table_entry(encoding_table, UTF_16BE_ENC as usize),
                        );
                        return dispatch_scanner(enc_ptr, state, ptr, end, next_tok_ptr);
                    }
                } else if second == 0 && state != XML_CONTENT_STATE {
                    set_encoding_ptr(
                        enc_ptr,
                        read_encoding_table_entry(encoding_table, UTF_16LE_ENC as usize),
                    );
                    return dispatch_scanner(enc_ptr, state, ptr, end, next_tok_ptr);
                }
            }
        }
    }

    set_encoding_ptr(
        enc_ptr,
        read_encoding_table_entry(encoding_table, initial_encoding as usize),
    );
    dispatch_scanner(enc_ptr, state, ptr, end, next_tok_ptr)
}
#[no_mangle]
pub unsafe extern "C" fn XmlGetUtf8InternalEncoding() -> *const ENCODING {
    unsafe {
        return &raw const internal_utf8_encoding.enc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XmlGetUtf16InternalEncoding() -> *const ENCODING {
    unsafe {
        return &raw const internal_little2_encoding.enc;
    }
}
struct EncodingTable([*const ENCODING; 7]);

unsafe impl Sync for EncodingTable {}

static encodings: EncodingTable = unsafe {
    EncodingTable([
        ::core::ptr::addr_of!(latin1_encoding.enc),
        ::core::ptr::addr_of!(ascii_encoding.enc),
        ::core::ptr::addr_of!(utf8_encoding.enc),
        ::core::ptr::addr_of!(big2_encoding.enc),
        ::core::ptr::addr_of!(big2_encoding.enc),
        ::core::ptr::addr_of!(little2_encoding.enc),
        ::core::ptr::addr_of!(utf8_encoding.enc),
    ])
};
extern "C" fn initScanProlog(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    initScan(
        encoding_table_ptr(),
        enc.cast::<INIT_ENCODING>(),
        XML_PROLOG_STATE,
        ptr,
        end,
        next_tok_ptr,
    )
}
extern "C" fn initScanContent(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    initScan(
        encoding_table_ptr(),
        enc.cast::<INIT_ENCODING>(),
        XML_CONTENT_STATE,
        ptr,
        end,
        next_tok_ptr,
    )
}

pub(crate) fn init_encoding_state(
    init: &mut INIT_ENCODING,
    enc_ptr: &mut *const ENCODING,
    name: *const ::core::ffi::c_char,
    namespace_aware: bool,
) -> ::core::ffi::c_int {
    let i = getEncodingIndex(name);
    if i == UNKNOWN_ENC as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }

    init.initEnc.isUtf16 = i as ::core::ffi::c_char;
    init.initEnc.scanners[XML_PROLOG_STATE as usize] = Some(if namespace_aware {
        initScanPrologNS
    } else {
        initScanProlog
    });
    init.initEnc.scanners[XML_CONTENT_STATE as usize] = Some(if namespace_aware {
        initScanContentNS
    } else {
        initScanContent
    });
    init.initEnc.updatePosition = Some(initUpdatePosition);
    init.encPtr = enc_ptr;
    *enc_ptr = ::core::ptr::addr_of_mut!(init.initEnc).cast_const();
    1 as ::core::ffi::c_int
}

#[no_mangle]
pub unsafe extern "C" fn XmlInitEncoding(
    mut p: *mut INIT_ENCODING,
    mut encPtr: *mut *const ENCODING,
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe { init_encoding_state(&mut *p, &mut *encPtr, name, false) }
}
fn find_encoding(
    encoding_table: *const *const ENCODING,
    enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
) -> *const ENCODING {
    let mut buf: [::core::ffi::c_char; 128] = [0; 128];
    let mut out = buf.as_mut_ptr();
    encoding_utf8_convert(
        enc,
        &mut ptr,
        end,
        &mut out,
        add_const_c_char(out.cast_const(), 127),
    );
    if ptr != end {
        return ::core::ptr::null::<ENCODING>();
    }
    write_copy(out, 0 as ::core::ffi::c_char);
    if streqci(buf.as_ptr(), KW_UTF_16.as_ptr()) != 0 && encoding_min_bytes_per_char(enc) == 2 {
        return enc;
    }
    let index = getEncodingIndex(buf.as_ptr());
    if index == UNKNOWN_ENC as ::core::ffi::c_int {
        return ::core::ptr::null::<ENCODING>();
    }
    read_encoding_table_entry(encoding_table, index as usize)
}

extern "C" fn findEncoding(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
) -> *const ENCODING {
    find_encoding(encoding_table_ptr(), enc, ptr, end)
}
#[no_mangle]
pub unsafe extern "C" fn XmlParseXmlDecl(
    mut isGeneralTextEntity: ::core::ffi::c_int,
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut badPtr: *mut *const ::core::ffi::c_char,
    mut versionPtr: *mut *const ::core::ffi::c_char,
    mut versionEndPtr: *mut *const ::core::ffi::c_char,
    mut encodingName: *mut *const ::core::ffi::c_char,
    mut encoding: *mut *const ENCODING,
    mut standalone: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    doParseXmlDecl(
        Some(
            findEncoding
                as extern "C" fn(
                    *const ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> *const ENCODING,
        ),
        isGeneralTextEntity,
        enc,
        ptr,
        end,
        badPtr,
        versionPtr,
        versionEndPtr,
        encodingName,
        encoding,
        standalone,
    )
}
#[no_mangle]
pub unsafe extern "C" fn XmlGetUtf8InternalEncodingNS() -> *const ENCODING {
    unsafe {
        return &raw const internal_utf8_encoding_ns.enc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XmlGetUtf16InternalEncodingNS() -> *const ENCODING {
    unsafe {
        return &raw const internal_little2_encoding_ns.enc;
    }
}
static encodingsNS: EncodingTable = unsafe {
    EncodingTable([
        ::core::ptr::addr_of!(latin1_encoding_ns.enc),
        ::core::ptr::addr_of!(ascii_encoding_ns.enc),
        ::core::ptr::addr_of!(utf8_encoding_ns.enc),
        ::core::ptr::addr_of!(big2_encoding_ns.enc),
        ::core::ptr::addr_of!(big2_encoding_ns.enc),
        ::core::ptr::addr_of!(little2_encoding_ns.enc),
        ::core::ptr::addr_of!(utf8_encoding_ns.enc),
    ])
};
extern "C" fn initScanPrologNS(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    initScan(
        encoding_table_ns_ptr(),
        enc.cast::<INIT_ENCODING>(),
        XML_PROLOG_STATE,
        ptr,
        end,
        next_tok_ptr,
    )
}
extern "C" fn initScanContentNS(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    initScan(
        encoding_table_ns_ptr(),
        enc.cast::<INIT_ENCODING>(),
        XML_CONTENT_STATE,
        ptr,
        end,
        next_tok_ptr,
    )
}
#[no_mangle]
pub unsafe extern "C" fn XmlInitEncodingNS(
    mut p: *mut INIT_ENCODING,
    mut encPtr: *mut *const ENCODING,
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe { init_encoding_state(&mut *p, &mut *encPtr, name, true) }
}
extern "C" fn findEncodingNS(
    enc: *const ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
) -> *const ENCODING {
    find_encoding(encoding_table_ns_ptr(), enc, ptr, end)
}
#[no_mangle]
pub unsafe extern "C" fn XmlParseXmlDeclNS(
    mut isGeneralTextEntity: ::core::ffi::c_int,
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut badPtr: *mut *const ::core::ffi::c_char,
    mut versionPtr: *mut *const ::core::ffi::c_char,
    mut versionEndPtr: *mut *const ::core::ffi::c_char,
    mut encodingName: *mut *const ::core::ffi::c_char,
    mut encoding: *mut *const ENCODING,
    mut standalone: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    doParseXmlDecl(
        Some(
            findEncodingNS
                as extern "C" fn(
                    *const ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> *const ENCODING,
        ),
        isGeneralTextEntity,
        enc,
        ptr,
        end,
        badPtr,
        versionPtr,
        versionEndPtr,
        encodingName,
        encoding,
        standalone,
    )
}
static namingBitmap: [::core::ffi::c_uint; 320] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x4000000 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x87fffffe as ::core::ffi::c_uint,
    0x7fffffe as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xff7fffff as ::core::ffi::c_uint,
    0xff7fffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0x7ff3ffff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfffffdfe as ::core::ffi::c_uint,
    0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffe00f as ::core::ffi::c_uint,
    0xfc31ffff as ::core::ffi::c_uint,
    0xffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffff0000 as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xf80001ff as ::core::ffi::c_uint,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffffd740 as ::core::ffi::c_uint,
    0xfffffffb as ::core::ffi::c_uint,
    0x547f7fff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffffd as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffffdffe as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xdffeffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffff0003 as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffff199f as ::core::ffi::c_uint,
    0x33fcfff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfffe0000 as ::core::ffi::c_uint,
    0x27fffff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfffffffe as ::core::ffi::c_uint,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffff0000 as ::core::ffi::c_uint,
    0x707ff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x7fffffe as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x7fe as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfffe0000 as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0x7cffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x2f7fff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x60 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffffffe0 as ::core::ffi::c_uint,
    0x23ffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xff000000 as ::core::ffi::c_uint,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfff99fe0 as ::core::ffi::c_uint,
    0x3c5fdff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xb0000000 as ::core::ffi::c_uint,
    0x30003 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfff987e0 as ::core::ffi::c_uint,
    0x36dfdff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x5e000000 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x1c0000 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfffbafe0 as ::core::ffi::c_uint,
    0x23edfdff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfff99fe0 as ::core::ffi::c_uint,
    0x23cdfdff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xb0000000 as ::core::ffi::c_uint,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xd63dc7e0 as ::core::ffi::c_uint,
    0x3bfc718 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfffddfe0 as ::core::ffi::c_uint,
    0x3effdff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfffddfe0 as ::core::ffi::c_uint,
    0x3effdff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x40000000 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfffddfe0 as ::core::ffi::c_uint,
    0x3fffdff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfffffffe as ::core::ffi::c_uint,
    0xd7fff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfef02596 as ::core::ffi::c_uint,
    0x200d6cae as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfffffeff as ::core::ffi::c_uint,
    0x3ff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffff003f as ::core::ffi::c_uint,
    0x7fffff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x7daed as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x50000000 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x82315001 as ::core::ffi::c_uint,
    0x2c62ab as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x40000000 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xf580c900 as ::core::ffi::c_uint,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x2010800 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xfffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0x3ffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x3f3fffff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xaaff3f3f as ::core::ffi::c_uint,
    0x3fffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0x5fdfffff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfcf1fdc as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x1fdc1fff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x4c40 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x3fe as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfffffffe as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0x1fffff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfffffffe as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0x7ffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffffffe0 as ::core::ffi::c_uint,
    0x1fff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xf as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x7ff6000 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x87fffffe as ::core::ffi::c_uint,
    0x7fffffe as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x800000 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xff7fffff as ::core::ffi::c_uint,
    0xff7fffff as ::core::ffi::c_uint,
    0xffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffff0000 as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xf80001ff as ::core::ffi::c_uint,
    0x30003 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffffd7c0 as ::core::ffi::c_uint,
    0xfffffffb as ::core::ffi::c_uint,
    0x547f7fff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffffd as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffffdffe as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xdffeffff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffff007b as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xffff199f as ::core::ffi::c_uint,
    0x33fcfff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfffe0000 as ::core::ffi::c_uint,
    0x27fffff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfffffffe as ::core::ffi::c_uint,
    0xfffe007f as ::core::ffi::c_uint,
    0xbbfffffb as ::core::ffi::c_uint,
    0xffff0016 as ::core::ffi::c_uint,
    0x707ff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x7fffffe as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x7ffff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffff03ff as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0x7cffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffef7fff as ::core::ffi::c_uint,
    0x3ff3dff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffffffee as ::core::ffi::c_uint,
    0xf3ffffff as ::core::ffi::c_uint,
    0xff1e3fff as ::core::ffi::c_uint,
    0xffcf as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfff99fee as ::core::ffi::c_uint,
    0xd3c5fdff as ::core::ffi::c_uint,
    0xb080399f as ::core::ffi::c_uint,
    0x3ffcf as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfff987e4 as ::core::ffi::c_uint,
    0xd36dfdff as ::core::ffi::c_uint,
    0x5e003987 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x1fffc0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfffbafee as ::core::ffi::c_uint,
    0xf3edfdff as ::core::ffi::c_uint,
    0x3bbf as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffc1 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfff99fee as ::core::ffi::c_uint,
    0xf3cdfdff as ::core::ffi::c_uint,
    0xb0c0398f as ::core::ffi::c_uint,
    0xffc3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xd63dc7ec as ::core::ffi::c_uint,
    0xc3bfc718 as ::core::ffi::c_uint,
    0x803dc7 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xff80 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfffddfee as ::core::ffi::c_uint,
    0xc3effdff as ::core::ffi::c_uint,
    0x603ddf as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffc3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfffddfec as ::core::ffi::c_uint,
    0xc3effdff as ::core::ffi::c_uint,
    0x40603ddf as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffc3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfffddfec as ::core::ffi::c_uint,
    0xc3fffdff as ::core::ffi::c_uint,
    0x803dcf as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xffc3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfffffffe as ::core::ffi::c_uint,
    0x7ff7fff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x3ff7fff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfef02596 as ::core::ffi::c_uint,
    0x3bff6cae as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x3ff3f5f as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x3000000 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xc2a003ff as ::core::ffi::c_uint,
    0xfffffeff as ::core::ffi::c_uint,
    0xfffe03ff as ::core::ffi::c_uint,
    0xfebf0fdf as ::core::ffi::c_uint,
    0x2fe3fff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x1fff0000 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xa0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x3efffe as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfffffffe as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0x661fffff as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xfffffffe as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0x77ffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
];
static nmstrtPages: [::core::ffi::c_uchar; 256] = [
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xb as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
static namePages: [::core::ffi::c_uchar; 256] = [
    0x19 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1a as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1b as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1c as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1d as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x21 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x23 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x25 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x26 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
#[no_mangle]
pub unsafe extern "C" fn XmlInitUnknownEncodingNS(
    mut mem: *mut ::core::ffi::c_void,
    mut table: *const ::core::ffi::c_int,
    mut convert: CONVERTER,
    mut userData: *mut ::core::ffi::c_void,
) -> *mut ENCODING {
    unsafe {
        let mut enc: *mut ENCODING = XmlInitUnknownEncoding(mem, table, convert, userData);
        if !enc.is_null() {
            (*(enc as *mut normal_encoding)).type_0[ASCII_COLON as usize] =
                BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar;
        }
        return enc;
    }
}
