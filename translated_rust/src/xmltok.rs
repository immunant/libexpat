// =============== BEGIN xmltok_h ================
pub const XML_TOK_TRAILING_RSQB: ::core::ffi::c_int = -5;

pub const XML_TOK_TRAILING_RSQB_1: ::core::ffi::c_int = -5 as ::core::ffi::c_int;

pub const XML_TOK_NONE: ::core::ffi::c_int = -4;

pub const XML_TOK_NONE_1: ::core::ffi::c_int = -4 as ::core::ffi::c_int;

pub const XML_TOK_TRAILING_CR: ::core::ffi::c_int = -3;

pub const XML_TOK_TRAILING_CR_1: ::core::ffi::c_int = -3 as ::core::ffi::c_int;

pub const XML_TOK_PARTIAL_CHAR: ::core::ffi::c_int = -2;

pub const XML_TOK_PARTIAL_CHAR_1: ::core::ffi::c_int = -2 as ::core::ffi::c_int;

pub const XML_TOK_PARTIAL: ::core::ffi::c_int = -1;

pub const XML_TOK_PARTIAL_1: ::core::ffi::c_int = -1 as ::core::ffi::c_int;

pub const XML_TOK_INVALID: ::core::ffi::c_int = 0;

pub const XML_TOK_INVALID_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const XML_TOK_START_TAG_WITH_ATTS: ::core::ffi::c_int = 1;

pub const XML_TOK_START_TAG_WITH_ATTS_1: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const XML_TOK_START_TAG_NO_ATTS: ::core::ffi::c_int = 2;

pub const XML_TOK_START_TAG_NO_ATTS_1: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const XML_TOK_EMPTY_ELEMENT_WITH_ATTS: ::core::ffi::c_int = 3;

pub const XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const XML_TOK_EMPTY_ELEMENT_NO_ATTS: ::core::ffi::c_int = 4;

pub const XML_TOK_EMPTY_ELEMENT_NO_ATTS_1: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const XML_TOK_END_TAG: ::core::ffi::c_int = 5;

pub const XML_TOK_END_TAG_1: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const XML_TOK_DATA_CHARS: ::core::ffi::c_int = 6;

pub const XML_TOK_DATA_CHARS_1: ::core::ffi::c_int = 6 as ::core::ffi::c_int;

pub const XML_TOK_DATA_NEWLINE: ::core::ffi::c_int = 7;

pub const XML_TOK_DATA_NEWLINE_1: ::core::ffi::c_int = 7 as ::core::ffi::c_int;

pub const XML_TOK_CDATA_SECT_OPEN: ::core::ffi::c_int = 8;

pub const XML_TOK_CDATA_SECT_OPEN_1: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

pub const XML_TOK_ENTITY_REF: ::core::ffi::c_int = 9;

pub const XML_TOK_ENTITY_REF_1: ::core::ffi::c_int = 9 as ::core::ffi::c_int;

pub const XML_TOK_CHAR_REF: ::core::ffi::c_int = 10;

pub const XML_TOK_CHAR_REF_1: ::core::ffi::c_int = 10 as ::core::ffi::c_int;

pub const XML_TOK_PI: ::core::ffi::c_int = 11;

pub const XML_TOK_PI_1: ::core::ffi::c_int = 11 as ::core::ffi::c_int;

pub const XML_TOK_XML_DECL: ::core::ffi::c_int = 12;

pub const XML_TOK_XML_DECL_1: ::core::ffi::c_int = 12 as ::core::ffi::c_int;

pub const XML_TOK_COMMENT: ::core::ffi::c_int = 13;

pub const XML_TOK_COMMENT_1: ::core::ffi::c_int = 13 as ::core::ffi::c_int;

pub const XML_TOK_BOM: ::core::ffi::c_int = 14;

pub const XML_TOK_BOM_1: ::core::ffi::c_int = 14 as ::core::ffi::c_int;

pub const XML_TOK_PROLOG_S: ::core::ffi::c_int = 15;

pub const XML_TOK_PROLOG_S_1: ::core::ffi::c_int = 15 as ::core::ffi::c_int;

pub const XML_TOK_DECL_OPEN: ::core::ffi::c_int = 16;

pub const XML_TOK_DECL_OPEN_1: ::core::ffi::c_int = 16 as ::core::ffi::c_int;

pub const XML_TOK_DECL_CLOSE: ::core::ffi::c_int = 17;

pub const XML_TOK_DECL_CLOSE_1: ::core::ffi::c_int = 17 as ::core::ffi::c_int;

pub const XML_TOK_NAME: ::core::ffi::c_int = 18;

pub const XML_TOK_NMTOKEN: ::core::ffi::c_int = 19;

pub const XML_TOK_NMTOKEN_1: ::core::ffi::c_int = 19 as ::core::ffi::c_int;

pub const XML_TOK_POUND_NAME: ::core::ffi::c_int = 20;

pub const XML_TOK_POUND_NAME_1: ::core::ffi::c_int = 20 as ::core::ffi::c_int;

pub const XML_TOK_OR: ::core::ffi::c_int = 21;

pub const XML_TOK_OR_1: ::core::ffi::c_int = 21 as ::core::ffi::c_int;

pub const XML_TOK_PERCENT: ::core::ffi::c_int = 22;

pub const XML_TOK_PERCENT_1: ::core::ffi::c_int = 22 as ::core::ffi::c_int;

pub const XML_TOK_OPEN_PAREN: ::core::ffi::c_int = 23;

pub const XML_TOK_OPEN_PAREN_1: ::core::ffi::c_int = 23 as ::core::ffi::c_int;

pub const XML_TOK_CLOSE_PAREN: ::core::ffi::c_int = 24;

pub const XML_TOK_CLOSE_PAREN_1: ::core::ffi::c_int = 24 as ::core::ffi::c_int;

pub const XML_TOK_OPEN_BRACKET: ::core::ffi::c_int = 25;

pub const XML_TOK_OPEN_BRACKET_1: ::core::ffi::c_int = 25 as ::core::ffi::c_int;

pub const XML_TOK_CLOSE_BRACKET: ::core::ffi::c_int = 26;

pub const XML_TOK_CLOSE_BRACKET_1: ::core::ffi::c_int = 26 as ::core::ffi::c_int;

pub const XML_TOK_LITERAL: ::core::ffi::c_int = 27;

pub const XML_TOK_LITERAL_1: ::core::ffi::c_int = 27 as ::core::ffi::c_int;

pub const XML_TOK_PARAM_ENTITY_REF: ::core::ffi::c_int = 28;

pub const XML_TOK_PARAM_ENTITY_REF_1: ::core::ffi::c_int = 28 as ::core::ffi::c_int;

pub const XML_TOK_INSTANCE_START: ::core::ffi::c_int = 29 as ::core::ffi::c_int;

pub const XML_TOK_INSTANCE_START_1: ::core::ffi::c_int = 29;

pub const XML_TOK_NAME_QUESTION: ::core::ffi::c_int = 30;

pub const XML_TOK_NAME_QUESTION_1: ::core::ffi::c_int = 30 as ::core::ffi::c_int;

pub const XML_TOK_NAME_ASTERISK: ::core::ffi::c_int = 31;

pub const XML_TOK_NAME_ASTERISK_1: ::core::ffi::c_int = 31 as ::core::ffi::c_int;

pub const XML_TOK_NAME_PLUS: ::core::ffi::c_int = 32;

pub const XML_TOK_NAME_PLUS_1: ::core::ffi::c_int = 32 as ::core::ffi::c_int;

pub const XML_TOK_COND_SECT_OPEN: ::core::ffi::c_int = 33;

pub const XML_TOK_COND_SECT_OPEN_1: ::core::ffi::c_int = 33 as ::core::ffi::c_int;

pub const XML_TOK_COND_SECT_CLOSE: ::core::ffi::c_int = 34;

pub const XML_TOK_COND_SECT_CLOSE_1: ::core::ffi::c_int = 34 as ::core::ffi::c_int;

pub const XML_TOK_CLOSE_PAREN_QUESTION: ::core::ffi::c_int = 35;

pub const XML_TOK_CLOSE_PAREN_QUESTION_1: ::core::ffi::c_int = 35 as ::core::ffi::c_int;

pub const XML_TOK_CLOSE_PAREN_ASTERISK: ::core::ffi::c_int = 36;

pub const XML_TOK_CLOSE_PAREN_ASTERISK_1: ::core::ffi::c_int = 36 as ::core::ffi::c_int;

pub const XML_TOK_CLOSE_PAREN_PLUS: ::core::ffi::c_int = 37;

pub const XML_TOK_CLOSE_PAREN_PLUS_1: ::core::ffi::c_int = 37 as ::core::ffi::c_int;

pub const XML_TOK_COMMA: ::core::ffi::c_int = 38;

pub const XML_TOK_COMMA_1: ::core::ffi::c_int = 38 as ::core::ffi::c_int;

pub const XML_TOK_ATTRIBUTE_VALUE_S: ::core::ffi::c_int = 39;

pub const XML_TOK_ATTRIBUTE_VALUE_S_1: ::core::ffi::c_int = 39 as ::core::ffi::c_int;

pub const XML_TOK_CDATA_SECT_CLOSE: ::core::ffi::c_int = 40;

pub const XML_TOK_CDATA_SECT_CLOSE_1: ::core::ffi::c_int = 40 as ::core::ffi::c_int;

pub const XML_TOK_PREFIXED_NAME: ::core::ffi::c_int = 41;

pub const XML_TOK_IGNORE_SECT: ::core::ffi::c_int = 42;

pub const XML_TOK_IGNORE_SECT_1: ::core::ffi::c_int = 42 as ::core::ffi::c_int;

pub const XML_PROLOG_STATE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const XML_CONTENT_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub type POSITION = crate::src::xmltok::position;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct position {
    pub lineNumber: crate::expat_external_h::XML_Size,
    pub columnNumber: crate::expat_external_h::XML_Size,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ATTRIBUTE {
    pub name: *const ::core::ffi::c_char,
    pub valuePtr: *const ::core::ffi::c_char,
    pub valueEnd: *const ::core::ffi::c_char,
    pub normalized: ::core::ffi::c_char,
}

pub type ENCODING = crate::src::xmltok::encoding;

pub type SCANNER = Option<
    unsafe extern "C" fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
>;

pub type XML_Convert_Result = ::core::ffi::c_uint;

pub const XML_CONVERT_COMPLETED: crate::src::xmltok::XML_Convert_Result = 0;

pub const XML_CONVERT_INPUT_INCOMPLETE: crate::src::xmltok::XML_Convert_Result = 1;

pub const XML_CONVERT_OUTPUT_EXHAUSTED: crate::src::xmltok::XML_Convert_Result = 2;

#[derive(Copy, Clone)]
pub enum Utf8Converter {
    Utf8,
    Latin1,
    Ascii,
    Little2,
    Big2,
    Unknown,
}

#[derive(Copy, Clone)]
pub enum Utf16Converter {
    Utf8,
    Latin1,
    Little2,
    Big2,
    Unknown,
}

/// Chooses the fixed position updater without retaining an internal C callback.
#[derive(Copy, Clone)]
pub enum PositionUpdater {
    Init,
    Normal,
    Little2,
    Big2,
}

/// Chooses the literal scanner without retaining an internal C callback.
#[derive(Copy, Clone)]
pub enum LiteralScanner {
    NormalAttributeValue,
    NormalEntityValue,
    Little2AttributeValue,
    Little2EntityValue,
    Big2AttributeValue,
    Big2EntityValue,
}

/// Selects one of the fixed tokenizer scanners without retaining an internal
/// C callback in every encoding table.
#[derive(Copy, Clone)]
pub enum Scanner {
    NormalProlog,
    NormalContent,
    NormalCdataSection,
    NormalIgnoreSection,
    Little2Prolog,
    Little2Content,
    Little2CdataSection,
    Little2IgnoreSection,
    Big2Prolog,
    Big2Content,
    Big2CdataSection,
    Big2IgnoreSection,
    InitProlog,
    InitContent,
    InitPrologNS,
    InitContentNS,
}

impl Scanner {
    pub unsafe fn scan(
        self,
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        next_tok_ptr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let scanner: unsafe extern "C" fn(
            *const crate::src::xmltok::ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int = match self {
            Self::NormalProlog => xmltok_impl_c::normal_prologTok,
            Self::NormalContent => xmltok_impl_c::normal_contentTok,
            Self::NormalCdataSection => xmltok_impl_c::normal_cdataSectionTok,
            Self::NormalIgnoreSection => xmltok_impl_c::normal_ignoreSectionTok,
            Self::Little2Prolog => xmltok_impl_c::little2_prologTok,
            Self::Little2Content => xmltok_impl_c::little2_contentTok,
            Self::Little2CdataSection => xmltok_impl_c::little2_cdataSectionTok,
            Self::Little2IgnoreSection => xmltok_impl_c::little2_ignoreSectionTok,
            Self::Big2Prolog => xmltok_impl_c::big2_prologTok,
            Self::Big2Content => xmltok_impl_c::big2_contentTok,
            Self::Big2CdataSection => xmltok_impl_c::big2_cdataSectionTok,
            Self::Big2IgnoreSection => xmltok_impl_c::big2_ignoreSectionTok,
            Self::InitProlog => xmltok_ns_c::initScanProlog,
            Self::InitContent => xmltok_ns_c::initScanContent,
            Self::InitPrologNS => xmltok_ns_c::initScanPrologNS,
            Self::InitContentNS => xmltok_ns_c::initScanContentNS,
        };
        scanner(enc, ptr, end, next_tok_ptr)
    }
}

/// The tokenizer has exactly three public-identifier scanners.  Keeping the
/// selection as data avoids retaining an internal C callback in every
/// encoding table.
#[derive(Copy, Clone)]
pub enum PublicIdChecker {
    Normal,
    Little2,
    Big2,
}

fn public_id_bad_offset(
    bytes: &[u8],
    byte_types: &[::core::ffi::c_uchar; 256],
    checker: PublicIdChecker,
) -> Option<usize> {
    let width = match checker {
        PublicIdChecker::Normal => 1,
        PublicIdChecker::Little2 | PublicIdChecker::Big2 => 2,
    };
    for (offset, code_unit) in bytes.chunks_exact(width).enumerate() {
        let (character, byte_type) = match checker {
            PublicIdChecker::Normal => (Some(code_unit[0]), byte_types[code_unit[0] as usize]),
            PublicIdChecker::Little2 => {
                let (low, high) = (code_unit[0], code_unit[1]);
                (
                    (high == 0).then_some(low),
                    if high == 0 {
                        byte_types[low as usize]
                    } else {
                        unicode_byte_type(high as ::core::ffi::c_char, low as ::core::ffi::c_char)
                            as ::core::ffi::c_uchar
                    },
                )
            }
            PublicIdChecker::Big2 => {
                let (high, low) = (code_unit[0], code_unit[1]);
                (
                    (high == 0).then_some(low),
                    if high == 0 {
                        byte_types[low as usize]
                    } else {
                        unicode_byte_type(high as ::core::ffi::c_char, low as ::core::ffi::c_char)
                            as ::core::ffi::c_uchar
                    },
                )
            }
        };
        let valid = match byte_type as ::core::ffi::c_int {
            25 | 24 | 27 | 13 | 31 | 32 | 34 | 35 | 17 | 14 | 15 | 9 | 10 | 18 | 16 | 33 | 30
            | 19 | 23 => true,
            21 => character != Some(b'\t'),
            26 | 22 if character.is_some_and(|byte| byte & !0x7f == 0) => true,
            _ => matches!(character, Some(b'$' | b'@')),
        };
        if !valid {
            return Some(offset * width);
        }
    }
    None
}

pub unsafe fn check_public_id(
    checker: PublicIdChecker,
    enc: *const crate::src::xmltok::ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    bad_ptr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    match checker {
        PublicIdChecker::Normal => xmltok_impl_c::normal_isPublicId(enc, ptr, end, bad_ptr),
        PublicIdChecker::Little2 => xmltok_impl_c::little2_isPublicId(enc, ptr, end, bad_ptr),
        PublicIdChecker::Big2 => xmltok_impl_c::big2_isPublicId(enc, ptr, end, bad_ptr),
    }
}
#[derive(Copy, Clone)]
pub enum NameMatcher {
    Normal,
    Little2,
    Big2,
}

/// Selects the fixed encoded-name length scanner without retaining a raw
/// callback in each encoding table.
#[derive(Copy, Clone)]
pub enum NameLength {
    Normal,
    Little2,
    Big2,
}

pub unsafe fn name_length(
    enc: *const crate::src::xmltok::ENCODING,
    ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    match (*enc).nameLength {
        NameLength::Normal => normal_nameLength(enc, ptr),
        NameLength::Little2 => little2_nameLength(enc, ptr),
        NameLength::Big2 => big2_nameLength(enc, ptr),
    }
}

impl NameMatcher {
    pub unsafe fn matches_ascii(
        self,
        mut ptr1: *const ::core::ffi::c_char,
        end1: *const ::core::ffi::c_char,
        mut ptr2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let width = match self {
            Self::Normal => 1,
            Self::Little2 | Self::Big2 => 2,
        };
        while *ptr2 != 0 {
            if end1.offset_from(ptr1) < width {
                return 0;
            }
            let matches = match self {
                Self::Normal => *ptr1 == *ptr2,
                Self::Little2 => *ptr1.offset(1) == 0 && *ptr1 == *ptr2,
                Self::Big2 => *ptr1 == 0 && *ptr1.offset(1) == *ptr2,
            };
            if !matches {
                return 0;
            }
            ptr1 = ptr1.offset(width);
            ptr2 = ptr2.offset(1);
        }
        (ptr1 == end1) as ::core::ffi::c_int
    }
}

#[derive(Copy, Clone)]
pub enum AttributeScanner {
    Normal,
    Little2,
    Big2,
}

/// Selects the fixed whitespace scanner without retaining a raw callback.
#[derive(Copy, Clone)]
pub enum WhitespaceSkipper {
    Normal,
    Little2,
    Big2,
}

/// Selects the fixed entity-name matcher without retaining a raw callback.
#[derive(Copy, Clone)]
pub enum PredefinedEntityNameMatcher {
    Normal,
    Little2,
    Big2,
}

/// Selects the fixed decoder for numeric character references.
#[derive(Copy, Clone)]
pub enum CharRefNumberDecoder {
    Normal,
    Little2,
    Big2,
}

impl CharRefNumberDecoder {
    pub unsafe fn decode(
        self,
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        match self {
            Self::Normal => crate::src::xmltok::normal_charRefNumber(enc, ptr),
            Self::Little2 => crate::src::xmltok::little2_charRefNumber(enc, ptr),
            Self::Big2 => crate::src::xmltok::big2_charRefNumber(enc, ptr),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct encoding {
    pub scanners: [crate::src::xmltok::Scanner; 4],
    pub literalScanners: [crate::src::xmltok::LiteralScanner; 2],
    pub nameMatchesAscii: crate::src::xmltok::NameMatcher,
    pub nameLength: crate::src::xmltok::NameLength,
    pub skipS: crate::src::xmltok::WhitespaceSkipper,
    pub getAtts: crate::src::xmltok::AttributeScanner,
    pub charRefNumber: crate::src::xmltok::CharRefNumberDecoder,
    pub predefinedEntityName: crate::src::xmltok::PredefinedEntityNameMatcher,
    pub updatePosition: crate::src::xmltok::PositionUpdater,
    pub isPublicId: crate::src::xmltok::PublicIdChecker,
    pub utf8Convert: crate::src::xmltok::Utf8Converter,
    /// Selects the fixed UTF-16 converter without retaining a raw callback.
    pub utf16Convert: crate::src::xmltok::Utf16Converter,
    pub minBytesPerChar: ::core::ffi::c_int,
    pub isUtf8: ::core::ffi::c_char,
    pub isUtf16: ::core::ffi::c_char,
}

/// Matches a tokenizer-bounded encoded entity name using the selected fixed encoding.
pub unsafe fn predefined_entity_name(
    enc: *const crate::src::xmltok::ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    const PREDEFINED: [(&[u8], ::core::ffi::c_int); 5] = [
        (b"lt", crate::ascii_h::ASCII_LT),
        (b"gt", crate::ascii_h::ASCII_GT),
        (b"amp", crate::ascii_h::ASCII_AMP),
        (b"quot", crate::ascii_h::ASCII_QUOT),
        (b"apos", crate::ascii_h::ASCII_APOS),
    ];

    let matcher = (*enc).predefinedEntityName;
    let encoded_name_len = end.offset_from(ptr);
    for &(name, value) in &PREDEFINED {
        let width = match matcher {
            PredefinedEntityNameMatcher::Normal => 1,
            PredefinedEntityNameMatcher::Little2 | PredefinedEntityNameMatcher::Big2 => 2,
        };
        if encoded_name_len / width as isize != name.len() as isize {
            continue;
        }
        let matches = match matcher {
            PredefinedEntityNameMatcher::Normal => name
                .iter()
                .enumerate()
                .all(|(index, &byte)| *ptr.add(index) as u8 == byte),
            PredefinedEntityNameMatcher::Little2 => {
                name.iter().enumerate().all(|(index, &byte)| {
                    *ptr.add(index * 2) as u8 == byte && *ptr.add(index * 2 + 1) == 0
                })
            }
            PredefinedEntityNameMatcher::Big2 => name.iter().enumerate().all(|(index, &byte)| {
                *ptr.add(index * 2) == 0 && *ptr.add(index * 2 + 1) as u8 == byte
            }),
        };
        if matches {
            return value;
        }
    }
    0
}

pub unsafe fn convert_to_utf8(
    enc: *const crate::src::xmltok::ENCODING,
    from: *mut *const ::core::ffi::c_char,
    from_lim: *const ::core::ffi::c_char,
    to: *mut *mut ::core::ffi::c_char,
    to_lim: *const ::core::ffi::c_char,
) -> crate::src::xmltok::XML_Convert_Result {
    match (*enc).utf8Convert {
        Utf8Converter::Utf8 => utf8_toUtf8(enc, from, from_lim, to, to_lim),
        Utf8Converter::Latin1 => latin1_toUtf8(enc, from, from_lim, to, to_lim),
        Utf8Converter::Ascii => ascii_toUtf8(enc, from, from_lim, to, to_lim),
        Utf8Converter::Little2 => little2_toUtf8(enc, from, from_lim, to, to_lim),
        Utf8Converter::Big2 => big2_toUtf8(enc, from, from_lim, to, to_lim),
        Utf8Converter::Unknown => unknown_toUtf8(enc, from, from_lim, to, to_lim),
    }
}

#[derive(Copy, Clone)]
#[repr(C)]

pub struct INIT_ENCODING {
    pub initEnc: crate::src::xmltok::ENCODING,
    pub(crate) selected_encoding: Option<usize>,
}

pub type CONVERTER = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
>;

trait UnknownEncodingConverter: Send + Sync {
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        input: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}

impl UnknownEncodingConverter
    for unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char) -> ::core::ffi::c_int
{
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        input: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        self(user_data, input)
    }
}

// Unknown encodings are initialized in caller-provided storage.  Keep the
// foreign callback in this boundary adapter instead of retaining it in that
// internal tokenizer object; the storage address is a stable key until the
// parser resets or is freed.
static UNKNOWN_ENCODING_CONVERTERS: std::sync::OnceLock<
    std::sync::Mutex<
        std::collections::HashMap<usize, std::sync::Arc<dyn UnknownEncodingConverter>>,
    >,
> = std::sync::OnceLock::new();

fn register_unknown_encoding_converter(
    storage_id: usize,
    converter: Option<std::sync::Arc<dyn UnknownEncodingConverter>>,
) {
    let mut converters = UNKNOWN_ENCODING_CONVERTERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(converter) = converter {
        converters.insert(storage_id, converter);
    } else {
        converters.remove(&storage_id);
    }
}

fn unknown_encoding_converter(
    storage_id: usize,
) -> Option<std::sync::Arc<dyn UnknownEncodingConverter>> {
    UNKNOWN_ENCODING_CONVERTERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&storage_id)
        .cloned()
}

pub fn unregister_unknown_encoding_converter(storage_id: usize) {
    UNKNOWN_ENCODING_CONVERTERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&storage_id);
}

pub mod xmltok_impl_c {

    enum NormalCharCheck {
        Invalid,
        NameStart,
        Name,
    }

    unsafe fn normal_char_check(
        normal: &normal_encoding,
        kind: NormalCharCheck,
        width: usize,
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        input: &[u8],
    ) -> bool {
        let function = match kind {
            NormalCharCheck::Invalid => match width {
                2 => match normal.invalid2 {
                    Invalid2Checker::Never => Some(isNever as unsafe extern "C" fn(_, _) -> _),
                    Invalid2Checker::Utf8 => return utf8_invalid2(input),
                    Invalid2Checker::Unknown => {
                        Some(unknown_isInvalid as unsafe extern "C" fn(_, _) -> _)
                    }
                },
                3 => match normal.invalid3 {
                    Invalid3Checker::Never => Some(isNever as unsafe extern "C" fn(_, _) -> _),
                    Invalid3Checker::Utf8 => return utf8_invalid3(input),
                    Invalid3Checker::Unknown => {
                        Some(unknown_isInvalid as unsafe extern "C" fn(_, _) -> _)
                    }
                },
                4 => match normal.invalid4 {
                    Invalid4Checker::Never => return false,
                    Invalid4Checker::Utf8 => return utf8_invalid4(input),
                    Invalid4Checker::Unknown => {
                        Some(unknown_isInvalid as unsafe extern "C" fn(_, _) -> _)
                    }
                },
                _ => unreachable!(),
            },
            NormalCharCheck::NameStart => match width {
                2 => {
                    match normal.isNmstrt2 {
                        crate::src::xmltok::NameStart2Checker::Never => {
                            Some(isNever as unsafe extern "C" fn(_, _) -> _)
                        }
                        crate::src::xmltok::NameStart2Checker::Utf8 => {
                            return crate::src::xmltok::utf8_is_name_start2(input);
                        }
                        crate::src::xmltok::NameStart2Checker::Unknown => {
                            Some(
                                crate::src::xmltok::unknown_isNmstrt
                                    as unsafe extern "C" fn(_, _) -> _,
                            )
                        }
                    }
                }
                3 => match normal.isNmstrt3 {
                    NameStart3Checker::Never => {
                        Some(isNever as unsafe extern "C" fn(_, _) -> _)
                    }
                    NameStart3Checker::Utf8 => return utf8_is_name_start3(input),
                    NameStart3Checker::Unknown => Some(
                        unknown_isNmstrt as unsafe extern "C" fn(_, _) -> _,
                    ),
                },
                4 => match normal.isNmstrt4 {
                    NameStart4Checker::Never => return false,
                    NameStart4Checker::Unknown => Some(
                        unknown_isNmstrt as unsafe extern "C" fn(_, _) -> _,
                    ),
                },
                _ => unreachable!(),
            },
            NormalCharCheck::Name => match width {
                2 => {
                    return match normal.isName2 {
                        Name2Checker::Never => false,
                        Name2Checker::Utf8 => utf8_is_name2(input),
                        Name2Checker::Unknown => unknown_isName(enc, ptr) != 0,
                    };
                }
                3 => {
                    return match normal.isName3 {
                        Name3Checker::Never => false,
                        Name3Checker::Utf8 => utf8_is_name3(input),
                        Name3Checker::Unknown => unknown_isName(enc, ptr) != 0,
                    };
                }
                4 => match normal.isName4 {
                    Name4Checker::Never => Some(isNever as unsafe extern "C" fn(_, _) -> _),
                    Name4Checker::Unknown => {
                        Some(unknown_isName as unsafe extern "C" fn(_, _) -> _)
                    }
                },
                _ => unreachable!(),
            },
        };
        function.expect("non-null function pointer")(enc, ptr) != 0
    }

    fn normal_scan_comment_impl(
        byte_types: &[::core::ffi::c_uchar; 256],
        input: &[u8],
        is_invalid: impl Fn(usize, usize) -> bool,
    ) -> (::core::ffi::c_int, Option<usize>) {
        if input.first() != Some(&b'-') {
            return if input.is_empty() {
                (crate::src::xmltok::XML_TOK_PARTIAL_1, None)
            } else {
                (crate::src::xmltok::XML_TOK_INVALID_1, Some(0))
            };
        }

        let mut offset = 1;
        while offset < input.len() {
            match byte_types[input[offset] as usize] as ::core::ffi::c_int {
                kind @ (5 | 6 | 7) => {
                    let width = kind as usize - 3;
                    if input.len() - offset < width {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                    }
                    if is_invalid(offset, width) {
                        return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                    }
                    offset += width;
                }
                0 | 1 | 8 => {
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                }
                27 => {
                    offset += 1;
                    if offset == input.len() {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                    }
                    if input[offset] == b'-' {
                        offset += 1;
                        if offset == input.len() {
                            return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                        }
                        return if input[offset] == b'>' {
                            (crate::src::xmltok::XML_TOK_COMMENT_1, Some(offset + 1))
                        } else {
                            (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset))
                        };
                    }
                }
                _ => offset += 1,
            }
        }
        (crate::src::xmltok::XML_TOK_PARTIAL_1, None)
    }

    pub unsafe extern "C" fn normal_scanComment(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = end.offset_from(ptr);
        if input_len <= 0 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = ::core::slice::from_raw_parts(ptr.cast::<u8>(), input_len as usize);
        let normal = &*(enc as *const normal_encoding);
        let (token, next) = normal_scan_comment_impl(&normal.type_0, input, |offset, width| {
            normal_char_check(
                normal,
                NormalCharCheck::Invalid,
                width,
                enc,
                ptr.add(offset),
                &input[offset..],
            )
        });
        if let Some(offset) = next {
            *nextTokPtr = ptr.add(offset);
        }
        token
    }

    pub unsafe extern "C" fn normal_scanDecl(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize) {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
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
                *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                return crate::src::xmltok::XML_TOK_COND_SECT_OPEN_1;
            }
            22 | 24 => {
                ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
        }
        while end.offset_from(ptr) >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize {
            's_129: {
                match (*(enc as *const normal_encoding)).type_0
                    [*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                {
                    30 => {
                        if !(end.offset_from(ptr)
                            >= (2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                        {
                            return crate::src::xmltok::XML_TOK_PARTIAL_1;
                        }
                        match (*(enc as *const normal_encoding)).type_0[*ptr
                            .offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uchar
                            as usize] as ::core::ffi::c_int
                        {
                            21 | 9 | 10 | 30 => {
                                *nextTokPtr = ptr;
                                return crate::src::xmltok::XML_TOK_INVALID_1;
                            }
                            _ => {}
                        }
                    }
                    21 | 9 | 10 => {}
                    22 | 24 => {
                        ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        break 's_129;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                }
                *nextTokPtr = ptr;
                return crate::src::xmltok::XML_TOK_DECL_OPEN_1;
            }
        }
        return crate::src::xmltok::XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn normal_checkPiTarget(
        _enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut tokPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        let mut upper: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        *tokPtr = crate::src::xmltok::XML_TOK_PI_1;
        if end.offset_from(ptr) != (1 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize {
            return 1 as ::core::ffi::c_int;
        }
        match *ptr as ::core::ffi::c_int {
            crate::ascii_h::ASCII_x_1 => {}
            crate::ascii_h::ASCII_X_1 => {
                upper = 1 as ::core::ffi::c_int;
            }
            _ => return 1 as ::core::ffi::c_int,
        }
        ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
        match *ptr as ::core::ffi::c_int {
            crate::ascii_h::ASCII_m_1 => {}
            crate::ascii_h::ASCII_M_1 => {
                upper = 1 as ::core::ffi::c_int;
            }
            _ => return 1 as ::core::ffi::c_int,
        }
        ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
        match *ptr as ::core::ffi::c_int {
            crate::ascii_h::ASCII_l_1 => {}
            crate::ascii_h::ASCII_L_1 => {
                upper = 1 as ::core::ffi::c_int;
            }
            _ => return 1 as ::core::ffi::c_int,
        }
        if upper != 0 {
            return 0 as ::core::ffi::c_int;
        }
        *tokPtr = crate::src::xmltok::XML_TOK_XML_DECL_1;
        return 1 as ::core::ffi::c_int;
    }

    fn normal_byte_type(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
        offset: usize,
    ) -> ::core::ffi::c_int {
        enc.type_0[input[offset] as u8 as usize] as ::core::ffi::c_int
    }

    fn normal_utf8_invalid(input: &[::core::ffi::c_char], offset: usize, width: usize) -> bool {
        let bytes = &input[offset..offset + width];
        let b0 = bytes[0] as u8;
        let b1 = bytes[1] as u8;
        match width {
            2 => b0 < 0xc2 || b1 & 0xc0 != 0x80,
            3 => {
                let b2 = bytes[2] as u8;
                b2 & 0xc0 != 0x80
                    || (b0 == 0xef && b1 == 0xbf && b2 > 0xbd)
                    || (b0 == 0xe0 && (b1 < 0xa0 || b1 & 0xc0 == 0xc0))
                    || (b0 != 0xe0 && (b1 & 0xc0 != 0x80 || (b0 == 0xed && b1 > 0x9f)))
            }
            4 => {
                let b2 = bytes[2] as u8;
                let b3 = bytes[3] as u8;
                b2 & 0xc0 != 0x80
                    || b3 & 0xc0 != 0x80
                    || (b0 == 0xf0 && (b1 < 0x90 || b1 & 0xc0 == 0xc0))
                    || (b0 != 0xf0 && (b1 & 0xc0 != 0x80 || (b0 == 0xf4 && b1 > 0x8f)))
            }
            _ => true,
        }
    }

    fn normal_utf8_name_char(
        input: &[::core::ffi::c_char],
        offset: usize,
        width: usize,
        pages: &[::core::ffi::c_uchar; 256],
    ) -> bool {
        if width == 4 {
            return false;
        }
        let b0 = input[offset] as u8;
        let b1 = input[offset + 1] as u8;
        let bitmap_index = if width == 2 {
            pages[((b0 >> 2) & 7) as usize] as usize * 8
                + ((b0 & 3) as usize) * 2
                + ((b1 >> 5) & 1) as usize
        } else {
            pages[(((b0 & 0x0f) << 4) + (b1 >> 2 & 0x0f)) as usize] as usize * 8
                + ((b1 & 3) as usize) * 2
                + ((input[offset + 2] as u8 >> 5) & 1) as usize
        };
        namingBitmap[bitmap_index] & (1 << ((input[offset + width - 1] as u8) & 0x1f)) != 0
    }

    fn normal_pi_target_token(
        input: &[::core::ffi::c_char],
        target: usize,
        terminator: usize,
    ) -> Option<::core::ffi::c_int> {
        if terminator - target != 3 {
            return Some(crate::src::xmltok::XML_TOK_PI_1);
        }
        let mut upper = false;
        for (offset, lower, upper_case) in [
            (
                0,
                crate::ascii_h::ASCII_x_1 as u8,
                crate::ascii_h::ASCII_X_1 as u8,
            ),
            (
                1,
                crate::ascii_h::ASCII_m_1 as u8,
                crate::ascii_h::ASCII_M_1 as u8,
            ),
            (
                2,
                crate::ascii_h::ASCII_l_1 as u8,
                crate::ascii_h::ASCII_L_1 as u8,
            ),
        ] {
            match input[target + offset] as u8 {
                value if value == lower => {}
                value if value == upper_case => upper = true,
                _ => return Some(crate::src::xmltok::XML_TOK_PI_1),
            }
        }
        if upper {
            None
        } else {
            Some(crate::src::xmltok::XML_TOK_XML_DECL_1)
        }
    }

    fn normal_scan_pi_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> (::core::ffi::c_int, Option<usize>) {
        if input.is_empty() {
            return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }
        let target = 0;
        let mut offset = 0;
        match normal_byte_type(enc, input, offset) {
            22 | 24 => offset += 1,
            29 => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
            kind @ (5 | 6 | 7) => {
                let width = kind as usize - 3;
                if input.len() < width {
                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                }
                if enc.enc.isUtf8 == 0
                    || normal_utf8_invalid(input, offset, width)
                    || !normal_utf8_name_char(input, offset, width, &nmstrtPages)
                {
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                }
                offset += width;
            }
            _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
        }

        while offset < input.len() {
            match normal_byte_type(enc, input, offset) {
                22 | 24 | 25 | 26 | 27 => offset += 1,
                29 => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
                kind @ (5 | 6 | 7) => {
                    let width = kind as usize - 3;
                    if input.len() - offset < width {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                    }
                    if enc.enc.isUtf8 == 0
                        || normal_utf8_invalid(input, offset, width)
                        || !normal_utf8_name_char(input, offset, width, &namePages)
                    {
                        return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                    }
                    offset += width;
                }
                21 | 9 | 10 => {
                    let Some(token) = normal_pi_target_token(input, target, offset) else {
                        return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                    };
                    offset += 1;
                    while offset < input.len() {
                        match normal_byte_type(enc, input, offset) {
                            kind @ (5 | 6 | 7) => {
                                let width = kind as usize - 3;
                                if input.len() - offset < width {
                                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                                }
                                if enc.enc.isUtf8 != 0 && normal_utf8_invalid(input, offset, width)
                                {
                                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                                }
                                offset += width;
                            }
                            0 | 1 | 8 => {
                                return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset))
                            }
                            15 => {
                                offset += 1;
                                if offset == input.len() {
                                    return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                                }
                                if input[offset] as u8 == 0x3e {
                                    return (token, Some(offset + 1));
                                }
                            }
                            _ => offset += 1,
                        }
                    }
                    return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                }
                15 => {
                    let Some(token) = normal_pi_target_token(input, target, offset) else {
                        return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                    };
                    offset += 1;
                    if offset == input.len() {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                    }
                    return if input[offset] as u8 == 0x3e {
                        (token, Some(offset + 1))
                    } else {
                        (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset))
                    };
                }
                _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
            }
        }
        (crate::src::xmltok::XML_TOK_PARTIAL_1, None)
    }

    pub unsafe extern "C" fn normal_scanPi(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = unsafe { end.offset_from(ptr) };
        if input_len <= 0 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = unsafe { ::core::slice::from_raw_parts(ptr, input_len as usize) };
        let enc = unsafe { &*(enc as *const normal_encoding) };
        let (token, next) = normal_scan_pi_impl(enc, input);
        if let Some(offset) = next {
            unsafe { *nextTokPtr = ptr.add(offset) };
        }
        token
    }

    pub unsafe extern "C" fn normal_scanCdataSection(
        _enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        pub static mut CDATA_LSQB: [::core::ffi::c_char; 6] = [
            crate::ascii_h::ASCII_C as ::core::ffi::c_char,
            crate::ascii_h::ASCII_D as ::core::ffi::c_char,
            crate::ascii_h::ASCII_A as ::core::ffi::c_char,
            crate::ascii_h::ASCII_T as ::core::ffi::c_char,
            crate::ascii_h::ASCII_A as ::core::ffi::c_char,
            crate::ascii_h::ASCII_LSQB as ::core::ffi::c_char,
        ];
        let mut i: ::core::ffi::c_int = 0;
        if !(end.offset_from(ptr) >= (6 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize) {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < 6 as ::core::ffi::c_int {
            if !(*ptr as ::core::ffi::c_int == CDATA_LSQB[i as usize] as ::core::ffi::c_int) {
                *nextTokPtr = ptr;
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            i += 1;
            ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
        }
        *nextTokPtr = ptr;
        return crate::src::xmltok::XML_TOK_CDATA_SECT_OPEN_1;
    }

    fn utf8_sequence_is_invalid(bytes: &[u8]) -> bool {
        match bytes {
            [first, second] => *first < 0xc2 || second & 0x80 == 0 || second & 0xc0 == 0xc0,
            [first, second, third] => {
                third & 0x80 == 0
                    || if *first == 0xef && *second == 0xbf {
                        *third > 0xbd
                    } else {
                        third & 0xc0 == 0xc0
                    }
                    || if *first == 0xe0 {
                        *second < 0xa0 || second & 0xc0 == 0xc0
                    } else {
                        second & 0x80 == 0
                            || if *first == 0xed {
                                *second > 0x9f
                            } else {
                                second & 0xc0 == 0xc0
                            }
                    }
            }
            [first, second, third, fourth] => {
                fourth & 0x80 == 0
                    || fourth & 0xc0 == 0xc0
                    || third & 0x80 == 0
                    || third & 0xc0 == 0xc0
                    || if *first == 0xf0 {
                        *second < 0x90 || second & 0xc0 == 0xc0
                    } else {
                        second & 0x80 == 0
                            || if *first == 0xf4 {
                                *second > 0x8f
                            } else {
                                second & 0xc0 == 0xc0
                            }
                    }
            }
            _ => false,
        }
    }

    fn normal_cdata_section_tok(
        input: &[u8],
        byte_types: &[::core::ffi::c_uchar; 256],
        is_utf8: bool,
    ) -> (::core::ffi::c_int, Option<usize>) {
        if input.is_empty() {
            return (crate::src::xmltok::XML_TOK_NONE_1, None);
        }

        let byte_type = |offset: usize| byte_types[input[offset] as usize] as u32;
        let invalid_sequence = |offset: usize, width: usize| {
            is_utf8 && utf8_sequence_is_invalid(&input[offset..offset + width])
        };
        let mut offset = 0;
        match byte_type(offset) {
            crate::xmltok_impl_h::BT_RSQB => {
                offset += 1;
                if offset == input.len() {
                    return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                }
                if input[offset] == b']' {
                    offset += 1;
                    if offset == input.len() {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                    }
                    if input[offset] == b'>' {
                        return (
                            crate::src::xmltok::XML_TOK_CDATA_SECT_CLOSE_1,
                            Some(offset + 1),
                        );
                    }
                    offset -= 1;
                }
            }
            crate::xmltok_impl_h::BT_CR => {
                offset += 1;
                if offset == input.len() {
                    return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                }
                if byte_type(offset) == crate::xmltok_impl_h::BT_LF {
                    offset += 1;
                }
                return (crate::src::xmltok::XML_TOK_DATA_NEWLINE_1, Some(offset));
            }
            crate::xmltok_impl_h::BT_LF => {
                return (crate::src::xmltok::XML_TOK_DATA_NEWLINE_1, Some(1));
            }
            crate::xmltok_impl_h::BT_LEAD2
            | crate::xmltok_impl_h::BT_LEAD3
            | crate::xmltok_impl_h::BT_LEAD4 => {
                let width = match byte_type(offset) {
                    crate::xmltok_impl_h::BT_LEAD2 => 2,
                    crate::xmltok_impl_h::BT_LEAD3 => 3,
                    _ => 4,
                };
                if input.len() < width {
                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                }
                if invalid_sequence(offset, width) {
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                }
                offset += width;
            }
            crate::xmltok_impl_h::BT_NONXML
            | crate::xmltok_impl_h::BT_MALFORM
            | crate::xmltok_impl_h::BT_TRAIL => {
                return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
            }
            _ => offset += 1,
        }

        while offset < input.len() {
            match byte_type(offset) {
                crate::xmltok_impl_h::BT_LEAD2
                | crate::xmltok_impl_h::BT_LEAD3
                | crate::xmltok_impl_h::BT_LEAD4 => {
                    let width = match byte_type(offset) {
                        crate::xmltok_impl_h::BT_LEAD2 => 2,
                        crate::xmltok_impl_h::BT_LEAD3 => 3,
                        _ => 4,
                    };
                    if input.len() - offset < width || invalid_sequence(offset, width) {
                        return (crate::src::xmltok::XML_TOK_DATA_CHARS_1, Some(offset));
                    }
                    offset += width;
                }
                crate::xmltok_impl_h::BT_NONXML
                | crate::xmltok_impl_h::BT_MALFORM
                | crate::xmltok_impl_h::BT_TRAIL
                | crate::xmltok_impl_h::BT_CR
                | crate::xmltok_impl_h::BT_LF
                | crate::xmltok_impl_h::BT_RSQB => {
                    return (crate::src::xmltok::XML_TOK_DATA_CHARS_1, Some(offset));
                }
                _ => offset += 1,
            }
        }
        (crate::src::xmltok::XML_TOK_DATA_CHARS_1, Some(offset))
    }

    pub unsafe extern "C" fn normal_cdataSectionTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if ptr >= end {
            return crate::src::xmltok::XML_TOK_NONE_1;
        }
        let encoding = &*(enc as *const normal_encoding);
        let input = ::core::slice::from_raw_parts(ptr.cast::<u8>(), end.offset_from(ptr) as usize);
        let (token, next_offset) = normal_cdata_section_tok(
            input,
            &encoding.type_0,
            encoding.enc.isUtf8 != 0,
        );
        if let Some(next_offset) = next_offset {
            *nextTokPtr = ptr.add(next_offset);
        }
        token
    }

    enum NormalScanEndTagCharCheck {
        Invalid,
        NameStart,
        Name,
    }

    struct NormalScanEndTagResult {
        token: ::core::ffi::c_int,
        next: Option<usize>,
    }

    fn normal_scan_end_tag_result(
        token: ::core::ffi::c_int,
        next: Option<usize>,
    ) -> NormalScanEndTagResult {
        NormalScanEndTagResult { token, next }
    }

    fn normal_scan_end_tag_multibyte<F>(
        byte_type: ::core::ffi::c_int,
        offset: usize,
        input: &[u8],
        check: &F,
        name_start: bool,
    ) -> Result<usize, NormalScanEndTagResult>
    where
        F: Fn(NormalScanEndTagCharCheck, usize, usize) -> bool,
    {
        let width = match byte_type {
            5 => 2,
            6 => 3,
            7 => 4,
            _ => unreachable!("only UTF-8 lead byte types are multibyte"),
        };
        if input.len() - offset < width {
            return Err(normal_scan_end_tag_result(
                crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                None,
            ));
        }
        if check(NormalScanEndTagCharCheck::Invalid, offset, width)
            || !check(
                if name_start {
                    NormalScanEndTagCharCheck::NameStart
                } else {
                    NormalScanEndTagCharCheck::Name
                },
                offset,
                width,
            )
        {
            return Err(normal_scan_end_tag_result(
                crate::src::xmltok::XML_TOK_INVALID_1,
                Some(offset),
            ));
        }
        Ok(width)
    }

    fn normal_scan_end_tag_impl<F>(
        enc: &normal_encoding,
        input: &[u8],
        check: F,
    ) -> NormalScanEndTagResult
    where
        F: Fn(NormalScanEndTagCharCheck, usize, usize) -> bool,
    {
        let byte_type = |offset: usize| enc.type_0[input[offset] as usize] as ::core::ffi::c_int;
        if input.is_empty() {
            return normal_scan_end_tag_result(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }

        let mut ptr = match byte_type(0) {
            22 | 24 => 1,
            5 | 6 | 7 => {
                match normal_scan_end_tag_multibyte(byte_type(0), 0, input, &check, true) {
                    Ok(width) => width,
                    Err(result) => return result,
                }
            }
            _ => return normal_scan_end_tag_result(crate::src::xmltok::XML_TOK_INVALID_1, Some(0)),
        };

        while ptr < input.len() {
            match byte_type(ptr) {
                22 | 24 | 25 | 26 | 27 => ptr += 1,
                5 | 6 | 7 => {
                    match normal_scan_end_tag_multibyte(byte_type(ptr), ptr, input, &check, false) {
                        Ok(width) => ptr += width,
                        Err(result) => return result,
                    }
                }
                21 | 9 | 10 => {
                    ptr += 1;
                    while ptr < input.len() {
                        match byte_type(ptr) {
                            21 | 9 | 10 => ptr += 1,
                            11 => {
                                return normal_scan_end_tag_result(
                                    crate::src::xmltok::XML_TOK_END_TAG_1,
                                    Some(ptr + 1),
                                )
                            }
                            _ => {
                                return normal_scan_end_tag_result(
                                    crate::src::xmltok::XML_TOK_INVALID_1,
                                    Some(ptr),
                                )
                            }
                        }
                    }
                    return normal_scan_end_tag_result(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                }
                23 => ptr += 1,
                11 => {
                    return normal_scan_end_tag_result(
                        crate::src::xmltok::XML_TOK_END_TAG_1,
                        Some(ptr + 1),
                    )
                }
                _ => {
                    return normal_scan_end_tag_result(
                        crate::src::xmltok::XML_TOK_INVALID_1,
                        Some(ptr),
                    )
                }
            }
        }
        normal_scan_end_tag_result(crate::src::xmltok::XML_TOK_PARTIAL_1, None)
    }

    fn normal_scan_hex_char_ref_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> (::core::ffi::c_int, Option<usize>) {
        let Some((&first, rest)) = input.split_first() else {
            return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        };

        match enc.type_0[first as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int {
            25 | 24 => {}
            _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(0)),
        }

        for (offset, byte) in rest.iter().enumerate() {
            match enc.type_0[*byte as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int {
                25 | 24 => {}
                18 => return (crate::src::xmltok::XML_TOK_CHAR_REF_1, Some(offset + 2)),
                _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset + 1)),
            }
        }
        (crate::src::xmltok::XML_TOK_PARTIAL_1, None)
    }

    pub unsafe extern "C" fn normal_scanHexCharRef(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = unsafe { end.offset_from(ptr) };
        if input_len <= 0 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = unsafe { ::core::slice::from_raw_parts(ptr, input_len as usize) };
        let normal = unsafe { &*(enc as *const normal_encoding) };
        let (token, next) = normal_scan_hex_char_ref_impl(normal, input);
        if let Some(offset) = next {
            unsafe { *nextTokPtr = ptr.add(offset) };
        }
        token
    }

    pub unsafe extern "C" fn normal_scanCharRef(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if end.offset_from(ptr) >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize {
            if *ptr as ::core::ffi::c_int == 0x78 as ::core::ffi::c_int {
                return normal_scanHexCharRef(
                    enc,
                    ptr.offset(1 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                25 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
            }
            ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
            while end.offset_from(ptr)
                >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize
            {
                match (*(enc as *const normal_encoding)).type_0
                    [*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                {
                    25 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        return crate::src::xmltok::XML_TOK_CHAR_REF_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                }
                ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
            }
        }
        return crate::src::xmltok::XML_TOK_PARTIAL_1;
    }

    fn normal_scan_char_ref_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> (::core::ffi::c_int, Option<usize>) {
        if input.is_empty() {
            return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }

        let hex = input[0] as u8 == b'x';
        let mut offset = if hex { 1 } else { 0 };
        if offset == input.len() {
            return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }
        let first_kind = normal_byte_type(enc, input, offset);
        if !(first_kind == 25 || (hex && first_kind == 24)) {
            return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
        }
        offset += 1;

        while offset < input.len() {
            match normal_byte_type(enc, input, offset) {
                25 | 24 if hex => offset += 1,
                25 if !hex => offset += 1,
                18 => return (crate::src::xmltok::XML_TOK_CHAR_REF_1, Some(offset + 1)),
                _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
            }
        }
        (crate::src::xmltok::XML_TOK_PARTIAL_1, None)
    }

    fn normal_scan_ref_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> (::core::ffi::c_int, Option<usize>) {
        if input.is_empty() {
            return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }

        let mut offset = match normal_byte_type(enc, input, 0) {
            22 | 24 => 1,
            29 => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(0)),
            kind @ (5 | 6 | 7) => {
                let width = kind as usize - 3;
                if input.len() < width {
                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                }
                if enc.enc.isUtf8 == 0
                    || normal_utf8_invalid(input, 0, width)
                    || !normal_utf8_name_char(input, 0, width, &nmstrtPages)
                {
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(0));
                }
                width
            }
            19 => {
                let (token, next) = normal_scan_char_ref_impl(enc, &input[1..]);
                return (token, next.map(|offset| offset + 1));
            }
            _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(0)),
        };

        while offset < input.len() {
            match normal_byte_type(enc, input, offset) {
                22 | 24 | 25 | 26 | 27 => offset += 1,
                29 => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
                kind @ (5 | 6 | 7) => {
                    let width = kind as usize - 3;
                    if input.len() - offset < width {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                    }
                    if enc.enc.isUtf8 == 0
                        || normal_utf8_invalid(input, offset, width)
                        || !normal_utf8_name_char(input, offset, width, &namePages)
                    {
                        return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                    }
                    offset += width;
                }
                18 => return (crate::src::xmltok::XML_TOK_ENTITY_REF_1, Some(offset + 1)),
                _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
            }
        }
        (crate::src::xmltok::XML_TOK_PARTIAL_1, None)
    }

    pub unsafe extern "C" fn normal_scanRef(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = unsafe { end.offset_from(ptr) };
        if input_len <= 0 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = unsafe { ::core::slice::from_raw_parts(ptr, input_len as usize) };
        let normal = unsafe { &*(enc as *const normal_encoding) };
        let (token, next) = normal_scan_ref_impl(normal, input);
        if let Some(offset) = next {
            unsafe { *nextTokPtr = ptr.add(offset) };
        }
        token
    }

    enum NormalScanAttsCharCheck {
        Invalid,
        Name,
        NameStart,
    }

    struct NormalScanAttsResult {
        token: ::core::ffi::c_int,
        next: Option<usize>,
    }

    fn normal_scan_atts_result(
        token: ::core::ffi::c_int,
        next: Option<usize>,
    ) -> NormalScanAttsResult {
        NormalScanAttsResult { token, next }
    }

    fn normal_scan_atts_multibyte<F>(
        byte_type: ::core::ffi::c_int,
        input: &[u8],
        offset: usize,
        check: &F,
        required: Option<NormalScanAttsCharCheck>,
    ) -> Result<usize, NormalScanAttsResult>
    where
        F: Fn(NormalScanAttsCharCheck, usize, usize) -> bool,
    {
        let width = match byte_type {
            5 => 2,
            6 => 3,
            7 => 4,
            _ => unreachable!("only UTF-8 lead byte types are multibyte"),
        };
        if input.len().saturating_sub(offset) < width {
            return Err(normal_scan_atts_result(
                crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                None,
            ));
        }
        if check(NormalScanAttsCharCheck::Invalid, offset, width)
            || required.is_some_and(|kind| !check(kind, offset, width))
        {
            return Err(normal_scan_atts_result(
                crate::src::xmltok::XML_TOK_INVALID_1,
                Some(offset),
            ));
        }
        Ok(width)
    }

    fn normal_scan_atts_impl<F, R>(
        normal: &normal_encoding,
        input: &[u8],
        check: F,
        mut scan_ref: R,
    ) -> NormalScanAttsResult
    where
        F: Fn(NormalScanAttsCharCheck, usize, usize) -> bool,
        R: FnMut(usize) -> (::core::ffi::c_int, usize),
    {
        let invalid = crate::src::xmltok::XML_TOK_INVALID_1;
        let partial = crate::src::xmltok::XML_TOK_PARTIAL_1;
        let byte_type = |offset: usize| normal.type_0[input[offset] as usize] as ::core::ffi::c_int;
        let mut ptr = 0;
        let mut had_colon = false;

        'attribute: while ptr < input.len() {
            match byte_type(ptr) {
                29 => return normal_scan_atts_result(invalid, Some(ptr)),
                22 | 24 | 25 | 26 | 27 => {
                    ptr += 1;
                    continue;
                }
                5 | 6 | 7 => match normal_scan_atts_multibyte(
                    byte_type(ptr),
                    input,
                    ptr,
                    &check,
                    Some(NormalScanAttsCharCheck::Name),
                ) {
                    Ok(width) => {
                        ptr += width;
                        continue;
                    }
                    Err(result) => return result,
                },
                23 => {
                    if had_colon {
                        return normal_scan_atts_result(invalid, Some(ptr));
                    }
                    had_colon = true;
                    ptr += 1;
                    if ptr == input.len() {
                        return normal_scan_atts_result(partial, None);
                    }
                    match byte_type(ptr) {
                        22 | 24 => ptr += 1,
                        5 | 6 | 7 => match normal_scan_atts_multibyte(
                            byte_type(ptr),
                            input,
                            ptr,
                            &check,
                            Some(NormalScanAttsCharCheck::NameStart),
                        ) {
                            Ok(width) => ptr += width,
                            Err(result) => return result,
                        },
                        _ => return normal_scan_atts_result(invalid, Some(ptr)),
                    }
                    continue;
                }
                21 | 9 | 10 => loop {
                    ptr += 1;
                    if ptr == input.len() {
                        return normal_scan_atts_result(partial, None);
                    }
                    let ty = byte_type(ptr);
                    if ty == crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int {
                        break;
                    }
                    if !matches!(ty, 21 | 10 | 9) {
                        return normal_scan_atts_result(invalid, Some(ptr));
                    }
                },
                14 => {}
                _ => return normal_scan_atts_result(invalid, Some(ptr)),
            }

            had_colon = false;
            let open = loop {
                ptr += 1;
                if ptr == input.len() {
                    return normal_scan_atts_result(partial, None);
                }
                let ty = byte_type(ptr);
                if ty == crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int
                    || ty == crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int
                {
                    break ty;
                }
                if !matches!(ty, 21 | 10 | 9) {
                    return normal_scan_atts_result(invalid, Some(ptr));
                }
            };
            ptr += 1;
            loop {
                if ptr == input.len() {
                    return normal_scan_atts_result(partial, None);
                }
                let ty = byte_type(ptr);
                if ty == open {
                    break;
                }
                match ty {
                    5 | 6 | 7 => match normal_scan_atts_multibyte(ty, input, ptr, &check, None) {
                        Ok(width) => ptr += width,
                        Err(result) => return result,
                    },
                    0 | 1 | 8 | 2 => return normal_scan_atts_result(invalid, Some(ptr)),
                    3 => {
                        let (token, next) = scan_ref(ptr + 1);
                        ptr = next;
                        if token <= 0 {
                            return normal_scan_atts_result(
                                token,
                                (token == invalid).then_some(ptr),
                            );
                        }
                    }
                    _ => ptr += 1,
                }
            }

            ptr += 1;
            if ptr == input.len() {
                return normal_scan_atts_result(partial, None);
            }
            match byte_type(ptr) {
                21 | 9 | 10 => loop {
                    ptr += 1;
                    if ptr == input.len() {
                        return normal_scan_atts_result(partial, None);
                    }
                    match byte_type(ptr) {
                        21 | 9 | 10 => continue,
                        11 => {
                            return normal_scan_atts_result(
                                crate::src::xmltok::XML_TOK_START_TAG_WITH_ATTS_1,
                                Some(ptr + 1),
                            )
                        }
                        17 => {
                            ptr += 1;
                            if ptr == input.len() {
                                return normal_scan_atts_result(partial, None);
                            }
                            return if input[ptr] == b'>' {
                                normal_scan_atts_result(
                                    crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1,
                                    Some(ptr + 1),
                                )
                            } else {
                                normal_scan_atts_result(invalid, Some(ptr))
                            };
                        }
                        22 | 24 => {
                            ptr += 1;
                            continue 'attribute;
                        }
                        5 | 6 | 7 => match normal_scan_atts_multibyte(
                            byte_type(ptr),
                            input,
                            ptr,
                            &check,
                            Some(NormalScanAttsCharCheck::NameStart),
                        ) {
                            Ok(width) => {
                                ptr += width;
                                continue 'attribute;
                            }
                            Err(result) => return result,
                        },
                        _ => return normal_scan_atts_result(invalid, Some(ptr)),
                    }
                },
                11 => {
                    return normal_scan_atts_result(
                        crate::src::xmltok::XML_TOK_START_TAG_WITH_ATTS_1,
                        Some(ptr + 1),
                    )
                }
                17 => {
                    ptr += 1;
                    if ptr == input.len() {
                        return normal_scan_atts_result(partial, None);
                    }
                    return if input[ptr] == b'>' {
                        normal_scan_atts_result(
                            crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1,
                            Some(ptr + 1),
                        )
                    } else {
                        normal_scan_atts_result(invalid, Some(ptr))
                    };
                }
                _ => return normal_scan_atts_result(invalid, Some(ptr)),
            }
        }
        normal_scan_atts_result(partial, None)
    }

    enum NormalScanLtCharCheck {
        Invalid,
        NameStart,
        Name,
    }

    enum NormalScanLtAction {
        Token(::core::ffi::c_int, Option<usize>),
        Comment(usize),
        CdataSection(usize),
        ProcessingInstruction(usize),
        EndTag(usize),
        Attributes(usize),
    }

    fn normal_scan_lt_multibyte<F>(
        byte_type: ::core::ffi::c_int,
        offset: usize,
        input_len: usize,
        check: &F,
        name_start: bool,
    ) -> Result<usize, NormalScanLtAction>
    where
        F: Fn(NormalScanLtCharCheck, usize, usize) -> bool,
    {
        let width = match byte_type {
            5 => 2,
            6 => 3,
            7 => 4,
            _ => unreachable!("only UTF-8 lead byte types are multibyte"),
        };
        if input_len - offset < width {
            return Err(NormalScanLtAction::Token(
                crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                None,
            ));
        }
        if check(NormalScanLtCharCheck::Invalid, offset, width) {
            return Err(NormalScanLtAction::Token(
                crate::src::xmltok::XML_TOK_INVALID_1,
                Some(offset),
            ));
        }
        let character_class = if name_start {
            NormalScanLtCharCheck::NameStart
        } else {
            NormalScanLtCharCheck::Name
        };
        if check(character_class, offset, width) {
            Ok(width)
        } else {
            Err(NormalScanLtAction::Token(
                crate::src::xmltok::XML_TOK_INVALID_1,
                Some(offset),
            ))
        }
    }

    fn normal_scan_lt_impl<F>(enc: &normal_encoding, input: &[u8], check: F) -> NormalScanLtAction
    where
        F: Fn(NormalScanLtCharCheck, usize, usize) -> bool,
    {
        let byte_type = |offset: usize| enc.type_0[input[offset] as usize] as ::core::ffi::c_int;
        let input_len = input.len();
        if input.is_empty() {
            return NormalScanLtAction::Token(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }

        let mut ptr = match byte_type(0) {
            29 => return NormalScanLtAction::Token(crate::src::xmltok::XML_TOK_INVALID_1, Some(0)),
            22 | 24 => 1,
            5 | 6 | 7 => match normal_scan_lt_multibyte(byte_type(0), 0, input_len, &check, true) {
                Ok(width) => width,
                Err(action) => return action,
            },
            16 => {
                if input_len == 1 {
                    return NormalScanLtAction::Token(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                }
                return match byte_type(1) {
                    27 => NormalScanLtAction::Comment(2),
                    20 => NormalScanLtAction::CdataSection(2),
                    _ => NormalScanLtAction::Token(crate::src::xmltok::XML_TOK_INVALID_1, Some(1)),
                };
            }
            15 => return NormalScanLtAction::ProcessingInstruction(1),
            17 => return NormalScanLtAction::EndTag(1),
            _ => return NormalScanLtAction::Token(crate::src::xmltok::XML_TOK_INVALID_1, Some(0)),
        };
        let mut had_colon = false;

        while ptr < input_len {
            match byte_type(ptr) {
                29 => {
                    return NormalScanLtAction::Token(
                        crate::src::xmltok::XML_TOK_INVALID_1,
                        Some(ptr),
                    )
                }
                22 | 24 | 25 | 26 | 27 => ptr += 1,
                5 | 6 | 7 => {
                    match normal_scan_lt_multibyte(byte_type(ptr), ptr, input_len, &check, false) {
                        Ok(width) => ptr += width,
                        Err(action) => return action,
                    }
                }
                23 => {
                    if had_colon {
                        return NormalScanLtAction::Token(
                            crate::src::xmltok::XML_TOK_INVALID_1,
                            Some(ptr),
                        );
                    }
                    had_colon = true;
                    ptr += 1;
                    if ptr == input_len {
                        return NormalScanLtAction::Token(
                            crate::src::xmltok::XML_TOK_PARTIAL_1,
                            None,
                        );
                    }
                    match byte_type(ptr) {
                        22 | 24 => ptr += 1,
                        5 | 6 | 7 => match normal_scan_lt_multibyte(
                            byte_type(ptr),
                            ptr,
                            input_len,
                            &check,
                            true,
                        ) {
                            Ok(width) => ptr += width,
                            Err(action) => return action,
                        },
                        _ => {
                            return NormalScanLtAction::Token(
                                crate::src::xmltok::XML_TOK_INVALID_1,
                                Some(ptr),
                            )
                        }
                    }
                }
                21 | 9 | 10 => {
                    ptr += 1;
                    while ptr < input_len {
                        match byte_type(ptr) {
                            29 => {
                                return NormalScanLtAction::Token(
                                    crate::src::xmltok::XML_TOK_INVALID_1,
                                    Some(ptr),
                                )
                            }
                            22 | 24 => return NormalScanLtAction::Attributes(ptr + 1),
                            5 | 6 | 7 => match normal_scan_lt_multibyte(
                                byte_type(ptr),
                                ptr,
                                input_len,
                                &check,
                                true,
                            ) {
                                Ok(width) => return NormalScanLtAction::Attributes(ptr + width),
                                Err(action) => return action,
                            },
                            21 | 9 | 10 => ptr += 1,
                            11 => {
                                return NormalScanLtAction::Token(
                                    crate::src::xmltok::XML_TOK_START_TAG_NO_ATTS_1,
                                    Some(ptr + 1),
                                )
                            }
                            17 => {
                                ptr += 1;
                                if ptr == input_len {
                                    return NormalScanLtAction::Token(
                                        crate::src::xmltok::XML_TOK_PARTIAL_1,
                                        None,
                                    );
                                }
                                if input[ptr] != b'>' {
                                    return NormalScanLtAction::Token(
                                        crate::src::xmltok::XML_TOK_INVALID_1,
                                        Some(ptr),
                                    );
                                }
                                return NormalScanLtAction::Token(
                                    crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_NO_ATTS_1,
                                    Some(ptr + 1),
                                );
                            }
                            _ => {
                                return NormalScanLtAction::Token(
                                    crate::src::xmltok::XML_TOK_INVALID_1,
                                    Some(ptr),
                                )
                            }
                        }
                    }
                    return NormalScanLtAction::Token(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                }
                11 => {
                    return NormalScanLtAction::Token(
                        crate::src::xmltok::XML_TOK_START_TAG_NO_ATTS_1,
                        Some(ptr + 1),
                    )
                }
                17 => {
                    ptr += 1;
                    if ptr == input_len {
                        return NormalScanLtAction::Token(
                            crate::src::xmltok::XML_TOK_PARTIAL_1,
                            None,
                        );
                    }
                    if input[ptr] != b'>' {
                        return NormalScanLtAction::Token(
                            crate::src::xmltok::XML_TOK_INVALID_1,
                            Some(ptr),
                        );
                    }
                    return NormalScanLtAction::Token(
                        crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_NO_ATTS_1,
                        Some(ptr + 1),
                    );
                }
                _ => {
                    return NormalScanLtAction::Token(
                        crate::src::xmltok::XML_TOK_INVALID_1,
                        Some(ptr),
                    )
                }
            }
        }
        NormalScanLtAction::Token(crate::src::xmltok::XML_TOK_PARTIAL_1, None)
    }

    pub unsafe extern "C" fn normal_scanLt(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if ptr >= end {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input_len = end.offset_from(ptr) as usize;
        let input = ::core::slice::from_raw_parts(ptr.cast::<u8>(), input_len);
        let normal = &*(enc as *const normal_encoding);
        let action = normal_scan_lt_impl(normal, input, |kind, offset, width| {
            let kind = match kind {
                NormalScanLtCharCheck::Invalid => NormalCharCheck::Invalid,
                NormalScanLtCharCheck::NameStart => NormalCharCheck::NameStart,
                NormalScanLtCharCheck::Name => NormalCharCheck::Name,
            };
            normal_char_check(normal, kind, width, enc, ptr.add(offset), &input[offset..])
        });
        match action {
            NormalScanLtAction::Token(token, next) => {
                if let Some(next) = next {
                    *nextTokPtr = ptr.add(next);
                }
                token
            }
            NormalScanLtAction::Comment(start) => {
                normal_scanComment(enc, ptr.add(start), end, nextTokPtr)
            }
            NormalScanLtAction::CdataSection(start) => {
                normal_scanCdataSection(enc, ptr.add(start), end, nextTokPtr)
            }
            NormalScanLtAction::ProcessingInstruction(start) => {
                normal_scanPi(enc, ptr.add(start), end, nextTokPtr)
            }
            NormalScanLtAction::EndTag(start) => {
                let end_tag_input = &input[start..];
                let result = normal_scan_end_tag_impl(normal, end_tag_input, |kind, offset, width| {
                    let kind = match kind {
                        NormalScanEndTagCharCheck::Invalid => NormalCharCheck::Invalid,
                        NormalScanEndTagCharCheck::NameStart => NormalCharCheck::NameStart,
                        NormalScanEndTagCharCheck::Name => NormalCharCheck::Name,
                    };
                    normal_char_check(
                        normal,
                        kind,
                        width,
                        enc,
                        ptr.add(start + offset),
                        &end_tag_input[offset..],
                    )
                });
                if let Some(next) = result.next {
                    *nextTokPtr = ptr.add(start + next);
                }
                result.token
            }
            NormalScanLtAction::Attributes(start) => {
                let attributes = &input[start..];
                // `c_char` is a one-byte integer type, so this keeps the
                // checked bounds of `attributes` while matching the shared
                // entity-reference scanner's input representation.
                let attributes_as_chars = ::core::slice::from_raw_parts(
                    attributes.as_ptr().cast::<::core::ffi::c_char>(),
                    attributes.len(),
                );
                let result = normal_scan_atts_impl(
                    normal,
                    attributes,
                    |kind, offset, width| {
                        let kind = match kind {
                            NormalScanAttsCharCheck::Invalid => NormalCharCheck::Invalid,
                            NormalScanAttsCharCheck::Name => NormalCharCheck::Name,
                            NormalScanAttsCharCheck::NameStart => NormalCharCheck::NameStart,
                        };
                        normal_char_check(
                            normal,
                            kind,
                            width,
                            enc,
                            ptr.add(start + offset),
                            &attributes[offset..],
                        )
                    },
                    |ref_start| {
                        let (token, next) =
                            normal_scan_ref_impl(normal, &attributes_as_chars[ref_start..]);
                        (token, next.map_or(0, |offset| ref_start + offset))
                    },
                );
                if let Some(next) = result.next {
                    *nextTokPtr = ptr.add(start + next);
                }
                result.token
            }
        }
    }

    enum NormalContentAction {
        Token(::core::ffi::c_int, Option<usize>),
        ScanLt(usize),
        ScanRef(usize),
    }

    fn normal_content_tok_impl(
        byte_types: &[::core::ffi::c_uchar; 256],
        input: &[u8],
        is_invalid: impl Fn(usize, usize) -> bool,
    ) -> NormalContentAction {
        if input.is_empty() {
            return NormalContentAction::Token(crate::src::xmltok::XML_TOK_NONE_1, None);
        }

        let mut offset = 0usize;
        match byte_types[input[offset] as usize] as ::core::ffi::c_int {
            2 => return NormalContentAction::ScanLt(1),
            3 => return NormalContentAction::ScanRef(1),
            9 => {
                offset = 1;
                if offset == input.len() {
                    return NormalContentAction::Token(
                        crate::src::xmltok::XML_TOK_TRAILING_CR_1,
                        None,
                    );
                }
                if byte_types[input[offset] as usize] as ::core::ffi::c_int
                    == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                {
                    offset += 1;
                }
                return NormalContentAction::Token(
                    crate::src::xmltok::XML_TOK_DATA_NEWLINE_1,
                    Some(offset),
                );
            }
            10 => {
                return NormalContentAction::Token(
                    crate::src::xmltok::XML_TOK_DATA_NEWLINE_1,
                    Some(1),
                );
            }
            4 => {
                offset = 1;
                if offset == input.len() {
                    return NormalContentAction::Token(
                        crate::src::xmltok::XML_TOK_TRAILING_RSQB_1,
                        None,
                    );
                }
                if input[offset] == b']' {
                    offset += 1;
                    if offset == input.len() {
                        return NormalContentAction::Token(
                            crate::src::xmltok::XML_TOK_TRAILING_RSQB_1,
                            None,
                        );
                    }
                    if input[offset] != b'>' {
                        offset -= 1;
                    } else {
                        return NormalContentAction::Token(
                            crate::src::xmltok::XML_TOK_INVALID_1,
                            Some(offset),
                        );
                    }
                }
            }
            5 | 6 | 7 => {
                let width = match byte_types[input[offset] as usize] as ::core::ffi::c_int {
                    5 => 2,
                    6 => 3,
                    7 => 4,
                    _ => unreachable!(),
                };
                if input.len() < width {
                    return NormalContentAction::Token(
                        crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        None,
                    );
                }
                if is_invalid(offset, width) {
                    return NormalContentAction::Token(
                        crate::src::xmltok::XML_TOK_INVALID_1,
                        Some(offset),
                    );
                }
                offset += width;
            }
            0 | 1 | 8 => {
                return NormalContentAction::Token(
                    crate::src::xmltok::XML_TOK_INVALID_1,
                    Some(offset),
                );
            }
            _ => offset = 1,
        }

        while offset < input.len() {
            match byte_types[input[offset] as usize] as ::core::ffi::c_int {
                5 | 6 | 7 => {
                    let width = match byte_types[input[offset] as usize] as ::core::ffi::c_int {
                        5 => 2,
                        6 => 3,
                        7 => 4,
                        _ => unreachable!(),
                    };
                    if input.len() - offset < width || is_invalid(offset, width) {
                        return NormalContentAction::Token(
                            crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                            Some(offset),
                        );
                    }
                    offset += width;
                }
                4 => {
                    if input.len() - offset >= 2 && input[offset + 1] != b']' {
                        offset += 1;
                    } else if input.len() - offset >= 3 && input[offset + 2] != b'>' {
                        offset += 1;
                    } else if input.len() - offset >= 3 {
                        return NormalContentAction::Token(
                            crate::src::xmltok::XML_TOK_INVALID_1,
                            Some(offset + 2),
                        );
                    } else {
                        return NormalContentAction::Token(
                            crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                            Some(offset),
                        );
                    }
                }
                3 | 2 | 0 | 1 | 8 | 9 | 10 => {
                    return NormalContentAction::Token(
                        crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        Some(offset),
                    );
                }
                _ => offset += 1,
            }
        }
        NormalContentAction::Token(crate::src::xmltok::XML_TOK_DATA_CHARS_1, Some(offset))
    }

    pub unsafe extern "C" fn normal_contentTok(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if ptr >= end {
            return crate::src::xmltok::XML_TOK_NONE_1;
        }
        let input_len = end.offset_from(ptr) as usize;
        let input = ::core::slice::from_raw_parts(ptr.cast::<u8>(), input_len);
        let normal = &*(enc as *const normal_encoding);
        match normal_content_tok_impl(&normal.type_0, input, |offset, width| {
            normal_char_check(
                normal,
                NormalCharCheck::Invalid,
                width,
                enc,
                ptr.add(offset),
                &input[offset..],
            )
        }) {
            NormalContentAction::Token(token, next) => {
                if let Some(next) = next {
                    *nextTokPtr = ptr.add(next);
                }
                token
            }
            NormalContentAction::ScanLt(start) => {
                normal_scanLt(enc, ptr.add(start), end, nextTokPtr)
            }
            NormalContentAction::ScanRef(start) => {
                normal_scanRef(enc, ptr.add(start), end, nextTokPtr)
            }
        }
    }

    fn normal_scan_percent_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> (::core::ffi::c_int, Option<usize>) {
        if input.is_empty() {
            return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }

        let mut offset = match normal_byte_type(enc, input, 0) {
            22 | 24 => 1,
            29 => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(0)),
            kind @ (5 | 6 | 7) => {
                let width = kind as usize - 3;
                if input.len() < width {
                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                }
                if enc.enc.isUtf8 == 0
                    || normal_utf8_invalid(input, 0, width)
                    || !normal_utf8_name_char(input, 0, width, &nmstrtPages)
                {
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(0));
                }
                width
            }
            21 | 10 | 9 | 30 => return (crate::src::xmltok::XML_TOK_PERCENT_1, Some(0)),
            _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(0)),
        };

        while offset < input.len() {
            match normal_byte_type(enc, input, offset) {
                22 | 24 | 25 | 26 | 27 => offset += 1,
                29 => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
                kind @ (5 | 6 | 7) => {
                    let width = kind as usize - 3;
                    if input.len() - offset < width {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                    }
                    if enc.enc.isUtf8 == 0
                        || normal_utf8_invalid(input, offset, width)
                        || !normal_utf8_name_char(input, offset, width, &namePages)
                    {
                        return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                    }
                    offset += width;
                }
                18 => {
                    return (
                        crate::src::xmltok::XML_TOK_PARAM_ENTITY_REF_1,
                        Some(offset + 1),
                    )
                }
                _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
            }
        }
        (crate::src::xmltok::XML_TOK_PARTIAL_1, None)
    }

    pub unsafe extern "C" fn normal_scanPercent(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = unsafe { end.offset_from(ptr) };
        if input_len <= 0 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = unsafe { ::core::slice::from_raw_parts(ptr, input_len as usize) };
        let normal = unsafe { &*(enc as *const normal_encoding) };
        let (token, next) = normal_scan_percent_impl(normal, input);
        if let Some(offset) = next {
            unsafe { *nextTokPtr = ptr.add(offset) };
        }
        token
    }

    fn normal_scan_pound_name_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> (::core::ffi::c_int, Option<usize>) {
        if input.is_empty() {
            return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }

        let mut offset = match normal_byte_type(enc, input, 0) {
            22 | 24 => 1,
            29 => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(0)),
            kind @ (5 | 6 | 7) => {
                let width = kind as usize - 3;
                if input.len() < width {
                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                }
                if enc.enc.isUtf8 == 0
                    || normal_utf8_invalid(input, 0, width)
                    || !normal_utf8_name_char(input, 0, width, &nmstrtPages)
                {
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(0));
                }
                width
            }
            _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(0)),
        };

        while offset < input.len() {
            match normal_byte_type(enc, input, offset) {
                22 | 24 | 25 | 26 | 27 => offset += 1,
                29 => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
                kind @ (5 | 6 | 7) => {
                    let width = kind as usize - 3;
                    if input.len() - offset < width {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                    }
                    if enc.enc.isUtf8 == 0
                        || normal_utf8_invalid(input, offset, width)
                        || !normal_utf8_name_char(input, offset, width, &namePages)
                    {
                        return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                    }
                    offset += width;
                }
                9 | 10 | 21 | 32 | 11 | 30 | 36 => {
                    return (crate::src::xmltok::XML_TOK_POUND_NAME_1, Some(offset));
                }
                _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
            }
        }
        (-crate::src::xmltok::XML_TOK_POUND_NAME_1, None)
    }

    pub unsafe extern "C" fn normal_scanPoundName(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = unsafe { end.offset_from(ptr) };
        if input_len <= 0 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = unsafe { ::core::slice::from_raw_parts(ptr, input_len as usize) };
        let normal = unsafe { &*(enc as *const normal_encoding) };
        let (token, next) = normal_scan_pound_name_impl(normal, input);
        if let Some(offset) = next {
            unsafe { *nextTokPtr = ptr.add(offset) };
        }
        token
    }

    pub unsafe extern "C" fn normal_scanLit(
        mut open: ::core::ffi::c_int,
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        while end.offset_from(ptr) >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize {
            let mut t: ::core::ffi::c_int = (*(enc as *const normal_encoding)).type_0
                [*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int;
            match t {
                5 => {
                    if end.offset_from(ptr) < 2 as isize {
                        return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    let normal = &*(enc as *const normal_encoding);
                    let invalid = match normal.invalid2 {
                        Invalid2Checker::Never => false,
                        Invalid2Checker::Utf8 => {
                            utf8_invalid2(::core::slice::from_raw_parts(ptr.cast::<u8>(), 2))
                        }
                        Invalid2Checker::Unknown => unknown_isInvalid(enc, ptr) != 0,
                    };
                    if invalid {
                        *nextTokPtr = ptr;
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                6 => {
                    if end.offset_from(ptr) < 3 as isize {
                        return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    let normal = &*(enc as *const normal_encoding);
                    let predicate = match normal.invalid3 {
                        Invalid3Checker::Never => isNever,
                        Invalid3Checker::Utf8 => utf8_isInvalid3,
                        Invalid3Checker::Unknown => unknown_isInvalid,
                    };
                    if predicate(enc, ptr) != 0 {
                        *nextTokPtr = ptr;
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                }
                7 => {
                    if end.offset_from(ptr) < 4 as isize {
                        return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    let normal = &*(enc as *const normal_encoding);
                    let invalid = match normal.invalid4 {
                        Invalid4Checker::Never => false,
                        Invalid4Checker::Utf8 => {
                            utf8_invalid4(::core::slice::from_raw_parts(ptr.cast::<u8>(), 4))
                        }
                        Invalid4Checker::Unknown => {
                            let predicate = unknown_isInvalid as unsafe extern "C" fn(_, _) -> _;
                            predicate(enc, ptr) != 0
                        }
                    };
                    if invalid {
                        *nextTokPtr = ptr;
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
                12 | 13 => {
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                    if t == open {
                        if !(end.offset_from(ptr)
                            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                        {
                            return -crate::src::xmltok::XML_TOK_LITERAL_1;
                        }
                        *nextTokPtr = ptr;
                        match (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                        {
                            21 | 9 | 10 | 11 | 30 | 20 => {
                                return crate::src::xmltok::XML_TOK_LITERAL_1
                            }
                            _ => return crate::src::xmltok::XML_TOK_INVALID_1,
                        }
                    }
                }
                _ => {
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                }
            }
        }
        return crate::src::xmltok::XML_TOK_PARTIAL_1;
    }

    enum NormalPrologCharCheck {
        Invalid,
        NameStart,
        Name,
    }

    enum NormalPrologAction {
        Token(::core::ffi::c_int, usize),
        Literal(::core::ffi::c_int, usize),
        Declaration(usize),
        ProcessingInstruction(usize),
        Percent(usize),
        PoundName(usize),
    }

    fn normal_prolog_multibyte<F>(
        byte_type: ::core::ffi::c_int,
        offset: usize,
        input_len: usize,
        check: &F,
        name_start: bool,
    ) -> Result<usize, NormalPrologAction>
    where
        F: Fn(NormalPrologCharCheck, usize, usize) -> bool,
    {
        let width = match byte_type {
            5 => 2,
            6 => 3,
            7 => 4,
            _ => unreachable!("only UTF-8 lead byte types are multibyte"),
        };
        if input_len - offset < width {
            return Err(NormalPrologAction::Token(
                crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                offset,
            ));
        }
        if check(NormalPrologCharCheck::Invalid, offset, width) {
            return Err(NormalPrologAction::Token(
                crate::src::xmltok::XML_TOK_INVALID_1,
                offset,
            ));
        }
        let accepted = if name_start {
            check(NormalPrologCharCheck::NameStart, offset, width)
        } else {
            check(NormalPrologCharCheck::Name, offset, width)
        };
        if accepted {
            Ok(width)
        } else {
            Err(NormalPrologAction::Token(
                crate::src::xmltok::XML_TOK_INVALID_1,
                offset,
            ))
        }
    }

    fn normal_prolog_tok_impl<F>(
        enc: &normal_encoding,
        input: &[u8],
        check: F,
    ) -> NormalPrologAction
    where
        F: Fn(NormalPrologCharCheck, usize, usize) -> bool,
    {
        let byte_type = |offset: usize| enc.type_0[input[offset] as usize] as ::core::ffi::c_int;
        let input_len = input.len();
        if input.is_empty() {
            return NormalPrologAction::Token(crate::src::xmltok::XML_TOK_NONE_1, 0);
        }

        let mut ptr = 0;
        let mut tok = 0;
        match byte_type(ptr) {
            12 => {
                return NormalPrologAction::Literal(
                    crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int,
                    1,
                )
            }
            13 => {
                return NormalPrologAction::Literal(
                    crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int,
                    1,
                )
            }
            2 => {
                ptr += 1;
                if ptr == input_len {
                    return NormalPrologAction::Token(crate::src::xmltok::XML_TOK_PARTIAL_1, ptr);
                }
                return match byte_type(ptr) {
                    16 => NormalPrologAction::Declaration(ptr + 1),
                    15 => NormalPrologAction::ProcessingInstruction(ptr + 1),
                    22 | 24 | 29 | 5 | 6 | 7 => NormalPrologAction::Token(
                        crate::src::xmltok::XML_TOK_INSTANCE_START,
                        ptr - 1,
                    ),
                    _ => NormalPrologAction::Token(crate::src::xmltok::XML_TOK_INVALID_1, ptr),
                };
            }
            9 if ptr + 1 == input_len => {
                return NormalPrologAction::Token(
                    -crate::src::xmltok::XML_TOK_PROLOG_S_1,
                    input_len,
                )
            }
            9 | 21 | 10 => loop {
                ptr += 1;
                if ptr == input_len {
                    return NormalPrologAction::Token(crate::src::xmltok::XML_TOK_PROLOG_S_1, ptr);
                }
                match byte_type(ptr) {
                    21 | 10 => {}
                    9 if ptr + 1 != input_len => {}
                    _ => {
                        return NormalPrologAction::Token(
                            crate::src::xmltok::XML_TOK_PROLOG_S_1,
                            ptr,
                        )
                    }
                }
            },
            30 => return NormalPrologAction::Percent(1),
            35 => return NormalPrologAction::Token(crate::src::xmltok::XML_TOK_COMMA_1, 1),
            20 => return NormalPrologAction::Token(crate::src::xmltok::XML_TOK_OPEN_BRACKET_1, 1),
            4 => {
                ptr += 1;
                if ptr == input_len {
                    return NormalPrologAction::Token(
                        -crate::src::xmltok::XML_TOK_CLOSE_BRACKET_1,
                        ptr,
                    );
                }
                if input[ptr] == b']' {
                    if input_len - ptr < 2 {
                        return NormalPrologAction::Token(
                            crate::src::xmltok::XML_TOK_PARTIAL_1,
                            ptr,
                        );
                    }
                    if input[ptr + 1] == b'>' {
                        return NormalPrologAction::Token(
                            crate::src::xmltok::XML_TOK_COND_SECT_CLOSE_1,
                            ptr + 2,
                        );
                    }
                }
                return NormalPrologAction::Token(crate::src::xmltok::XML_TOK_CLOSE_BRACKET_1, ptr);
            }
            31 => return NormalPrologAction::Token(crate::src::xmltok::XML_TOK_OPEN_PAREN_1, 1),
            32 => {
                ptr += 1;
                if ptr == input_len {
                    return NormalPrologAction::Token(
                        -crate::src::xmltok::XML_TOK_CLOSE_PAREN_1,
                        ptr,
                    );
                }
                return match byte_type(ptr) {
                    33 => NormalPrologAction::Token(
                        crate::src::xmltok::XML_TOK_CLOSE_PAREN_ASTERISK_1,
                        ptr + 1,
                    ),
                    15 => NormalPrologAction::Token(
                        crate::src::xmltok::XML_TOK_CLOSE_PAREN_QUESTION_1,
                        ptr + 1,
                    ),
                    34 => NormalPrologAction::Token(
                        crate::src::xmltok::XML_TOK_CLOSE_PAREN_PLUS_1,
                        ptr + 1,
                    ),
                    9 | 10 | 21 | 11 | 35 | 36 | 32 => {
                        NormalPrologAction::Token(crate::src::xmltok::XML_TOK_CLOSE_PAREN_1, ptr)
                    }
                    _ => NormalPrologAction::Token(crate::src::xmltok::XML_TOK_INVALID_1, ptr),
                };
            }
            36 => return NormalPrologAction::Token(crate::src::xmltok::XML_TOK_OR_1, 1),
            11 => return NormalPrologAction::Token(crate::src::xmltok::XML_TOK_DECL_CLOSE_1, 1),
            19 => return NormalPrologAction::PoundName(1),
            5 | 6 | 7 => {
                let width = match byte_type(ptr) {
                    5 => 2,
                    6 => 3,
                    7 => 4,
                    _ => unreachable!(),
                };
                if input_len - ptr < width {
                    return NormalPrologAction::Token(
                        crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        ptr,
                    );
                }
                if check(NormalPrologCharCheck::Invalid, ptr, width) {
                    return NormalPrologAction::Token(crate::src::xmltok::XML_TOK_INVALID_1, ptr);
                }
                ptr += width;
                if check(NormalPrologCharCheck::NameStart, ptr - width, width) {
                    tok = crate::src::xmltok::XML_TOK_NAME;
                } else if check(NormalPrologCharCheck::Name, ptr - width, width) {
                    tok = crate::src::xmltok::XML_TOK_NMTOKEN_1;
                } else {
                    return NormalPrologAction::Token(
                        crate::src::xmltok::XML_TOK_INVALID_1,
                        ptr - width,
                    );
                }
            }
            22 | 24 => {
                tok = crate::src::xmltok::XML_TOK_NAME;
                ptr += 1;
            }
            25 | 26 | 27 | 23 => {
                tok = crate::src::xmltok::XML_TOK_NMTOKEN_1;
                ptr += 1;
            }
            _ => return NormalPrologAction::Token(crate::src::xmltok::XML_TOK_INVALID_1, ptr),
        }

        while ptr < input_len {
            match byte_type(ptr) {
                29 => return NormalPrologAction::Token(crate::src::xmltok::XML_TOK_INVALID_1, ptr),
                22 | 24 | 25 | 26 | 27 => ptr += 1,
                5 | 6 | 7 => {
                    match normal_prolog_multibyte(byte_type(ptr), ptr, input_len, &check, false) {
                        Ok(width) => ptr += width,
                        Err(action) => return action,
                    }
                }
                11 | 32 | 35 | 36 | 20 | 30 | 21 | 9 | 10 => {
                    return NormalPrologAction::Token(tok, ptr)
                }
                23 => {
                    ptr += 1;
                    let mut consume_name_char = false;
                    match tok {
                        crate::src::xmltok::XML_TOK_NAME => {
                            if ptr == input_len {
                                return NormalPrologAction::Token(
                                    crate::src::xmltok::XML_TOK_PARTIAL_1,
                                    ptr,
                                );
                            }
                            tok = crate::src::xmltok::XML_TOK_PREFIXED_NAME;
                            match byte_type(ptr) {
                                29 => {
                                    return NormalPrologAction::Token(
                                        crate::src::xmltok::XML_TOK_INVALID_1,
                                        ptr,
                                    )
                                }
                                22 | 24 | 25 | 26 | 27 => consume_name_char = true,
                                5 | 6 | 7 => match normal_prolog_multibyte(
                                    byte_type(ptr),
                                    ptr,
                                    input_len,
                                    &check,
                                    false,
                                ) {
                                    Ok(width) => {
                                        ptr += width;
                                        continue;
                                    }
                                    Err(action) => return action,
                                },
                                _ => tok = crate::src::xmltok::XML_TOK_NMTOKEN_1,
                            }
                        }
                        crate::src::xmltok::XML_TOK_PREFIXED_NAME => {
                            tok = crate::src::xmltok::XML_TOK_NMTOKEN_1
                        }
                        _ => {}
                    }
                    if consume_name_char {
                        ptr += 1;
                    }
                }
                34 => {
                    if tok == crate::src::xmltok::XML_TOK_NMTOKEN_1 {
                        return NormalPrologAction::Token(
                            crate::src::xmltok::XML_TOK_INVALID_1,
                            ptr,
                        );
                    }
                    return NormalPrologAction::Token(
                        crate::src::xmltok::XML_TOK_NAME_PLUS_1,
                        ptr + 1,
                    );
                }
                33 => {
                    if tok == crate::src::xmltok::XML_TOK_NMTOKEN_1 {
                        return NormalPrologAction::Token(
                            crate::src::xmltok::XML_TOK_INVALID_1,
                            ptr,
                        );
                    }
                    return NormalPrologAction::Token(
                        crate::src::xmltok::XML_TOK_NAME_ASTERISK_1,
                        ptr + 1,
                    );
                }
                15 => {
                    if tok == crate::src::xmltok::XML_TOK_NMTOKEN_1 {
                        return NormalPrologAction::Token(
                            crate::src::xmltok::XML_TOK_INVALID_1,
                            ptr,
                        );
                    }
                    return NormalPrologAction::Token(
                        crate::src::xmltok::XML_TOK_NAME_QUESTION_1,
                        ptr + 1,
                    );
                }
                _ => return NormalPrologAction::Token(crate::src::xmltok::XML_TOK_INVALID_1, ptr),
            }
        }
        NormalPrologAction::Token(-tok, ptr)
    }

    pub unsafe extern "C" fn normal_prologTok(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = if ptr >= end {
            0
        } else {
            end.offset_from(ptr) as usize
        };
        let input = ::core::slice::from_raw_parts(ptr.cast::<u8>(), input_len);
        let normal = &*(enc as *const normal_encoding);
        let action = normal_prolog_tok_impl(normal, input, |kind, offset, width| {
            let kind = match kind {
                NormalPrologCharCheck::Invalid => NormalCharCheck::Invalid,
                NormalPrologCharCheck::NameStart => NormalCharCheck::NameStart,
                NormalPrologCharCheck::Name => NormalCharCheck::Name,
            };
            normal_char_check(normal, kind, width, enc, ptr.add(offset), &input[offset..])
        });
        match action {
            NormalPrologAction::Token(token, next) => {
                if token >= crate::src::xmltok::XML_TOK_INVALID_1
                    || token == -crate::src::xmltok::XML_TOK_PROLOG_S_1
                {
                    *nextTokPtr = ptr.add(next);
                }
                token
            }
            NormalPrologAction::Literal(open, start) => {
                normal_scanLit(open, enc, ptr.add(start), end, nextTokPtr)
            }
            NormalPrologAction::Declaration(start) => {
                normal_scanDecl(enc, ptr.add(start), end, nextTokPtr)
            }
            NormalPrologAction::ProcessingInstruction(start) => {
                normal_scanPi(enc, ptr.add(start), end, nextTokPtr)
            }
            NormalPrologAction::Percent(start) => {
                normal_scanPercent(enc, ptr.add(start), end, nextTokPtr)
            }
            NormalPrologAction::PoundName(start) => {
                normal_scanPoundName(enc, ptr.add(start), end, nextTokPtr)
            }
        }
    }

    pub unsafe extern "C" fn normal_attributeValueTok(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if ptr >= end {
            return crate::src::xmltok::XML_TOK_NONE_1;
        } else if !(end.offset_from(ptr)
            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
        {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        start = ptr;
        while end.offset_from(ptr) >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize {
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
                    return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
                }
                2 => {
                    *nextTokPtr = ptr;
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        return crate::src::xmltok::XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        if !(end.offset_from(ptr)
                            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                        {
                            return crate::src::xmltok::XML_TOK_TRAILING_CR_1;
                        }
                        if (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                            == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        }
                        *nextTokPtr = ptr;
                        return crate::src::xmltok::XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
                }
                21 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        return crate::src::xmltok::XML_TOK_ATTRIBUTE_VALUE_S_1;
                    }
                    *nextTokPtr = ptr;
                    return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
                }
                _ => {
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
    }

    pub unsafe extern "C" fn normal_entityValueTok(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if ptr >= end {
            return crate::src::xmltok::XML_TOK_NONE_1;
        } else if !(end.offset_from(ptr)
            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
        {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        start = ptr;
        while end.offset_from(ptr) >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize {
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
                    return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
                }
                30 => {
                    if ptr == start {
                        let mut tok: ::core::ffi::c_int = normal_scanPercent(
                            enc,
                            ptr.offset(1 as ::core::ffi::c_int as isize),
                            end,
                            nextTokPtr,
                        );
                        return if tok == crate::src::xmltok::XML_TOK_PERCENT_1 {
                            crate::src::xmltok::XML_TOK_INVALID_1
                        } else {
                            tok
                        };
                    }
                    *nextTokPtr = ptr;
                    return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        return crate::src::xmltok::XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        if !(end.offset_from(ptr)
                            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                        {
                            return crate::src::xmltok::XML_TOK_TRAILING_CR_1;
                        }
                        if (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                            == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                        }
                        *nextTokPtr = ptr;
                        return crate::src::xmltok::XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
                }
                _ => {
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
    }

    enum NormalIgnoreSectionOutcome {
        Token(::core::ffi::c_int, usize),
        Partial(::core::ffi::c_int),
        Invalid(usize),
        UnknownInvalid {
            at: usize,
            width: usize,
            level: ::core::ffi::c_int,
        },
    }

    /// Scans an ignored conditional section using bounded indices.  Unknown
    /// encodings leave their callback-backed validity check to the boundary
    /// adapter, which resumes this scanner after a valid character.
    fn normal_ignore_section_tok_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
        mut pos: usize,
        mut level: ::core::ffi::c_int,
    ) -> NormalIgnoreSectionOutcome {
        while pos < input.len() {
            match enc.type_0[input[pos] as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int {
                5 => {
                    if input.len() - pos < 2 {
                        return NormalIgnoreSectionOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    let invalid = match enc.invalid2 {
                        Invalid2Checker::Never => false,
                        Invalid2Checker::Utf8 => utf8_invalid2(&[
                            input[pos] as u8,
                            input[pos + 1] as u8,
                        ]),
                        Invalid2Checker::Unknown => {
                            return NormalIgnoreSectionOutcome::UnknownInvalid {
                                at: pos,
                                width: 2,
                                level,
                            };
                        }
                    };
                    if invalid {
                        return NormalIgnoreSectionOutcome::Invalid(pos);
                    }
                    pos += 2;
                }
                6 => {
                    if input.len() - pos < 3 {
                        return NormalIgnoreSectionOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    let invalid = match enc.invalid3 {
                        Invalid3Checker::Never => false,
                        Invalid3Checker::Utf8 => utf8_invalid3(&[
                            input[pos] as u8,
                            input[pos + 1] as u8,
                            input[pos + 2] as u8,
                        ]),
                        Invalid3Checker::Unknown => {
                            return NormalIgnoreSectionOutcome::UnknownInvalid {
                                at: pos,
                                width: 3,
                                level,
                            };
                        }
                    };
                    if invalid {
                        return NormalIgnoreSectionOutcome::Invalid(pos);
                    }
                    pos += 3;
                }
                7 => {
                    if input.len() - pos < 4 {
                        return NormalIgnoreSectionOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    let invalid = match enc.invalid4 {
                        Invalid4Checker::Never => false,
                        Invalid4Checker::Utf8 => utf8_invalid4(&[
                            input[pos] as u8,
                            input[pos + 1] as u8,
                            input[pos + 2] as u8,
                            input[pos + 3] as u8,
                        ]),
                        Invalid4Checker::Unknown => {
                            return NormalIgnoreSectionOutcome::UnknownInvalid {
                                at: pos,
                                width: 4,
                                level,
                            };
                        }
                    };
                    if invalid {
                        return NormalIgnoreSectionOutcome::Invalid(pos);
                    }
                    pos += 4;
                }
                0 | 1 | 8 => return NormalIgnoreSectionOutcome::Invalid(pos),
                2 => {
                    pos += 1;
                    if pos == input.len() {
                        return NormalIgnoreSectionOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_1,
                        );
                    }
                    if input[pos] as ::core::ffi::c_int == 0x21 {
                        pos += 1;
                        if pos == input.len() {
                            return NormalIgnoreSectionOutcome::Partial(
                                crate::src::xmltok::XML_TOK_PARTIAL_1,
                            );
                        }
                        if input[pos] as ::core::ffi::c_int == 0x5b {
                            level += 1;
                            pos += 1;
                        }
                    }
                }
                4 => {
                    pos += 1;
                    if pos == input.len() {
                        return NormalIgnoreSectionOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_1,
                        );
                    }
                    if input[pos] as ::core::ffi::c_int == 0x5d {
                        pos += 1;
                        if pos == input.len() {
                            return NormalIgnoreSectionOutcome::Partial(
                                crate::src::xmltok::XML_TOK_PARTIAL_1,
                            );
                        }
                        if input[pos] as ::core::ffi::c_int == 0x3e {
                            pos += 1;
                            if level == 0 {
                                return NormalIgnoreSectionOutcome::Token(
                                    crate::src::xmltok::XML_TOK_IGNORE_SECT_1,
                                    pos,
                                );
                            }
                            level -= 1;
                        }
                    }
                }
                _ => pos += 1,
            }
        }
        NormalIgnoreSectionOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1)
    }

    pub unsafe extern "C" fn normal_ignoreSectionTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = end.offset_from(ptr);
        if input_len <= 0 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = ::core::slice::from_raw_parts(ptr, input_len as usize);
        let encoding = &*(enc as *const normal_encoding);
        let mut start = 0;
        let mut level = 0;

        loop {
            match normal_ignore_section_tok_impl(encoding, input, start, level) {
                NormalIgnoreSectionOutcome::Token(token, next) => {
                    *nextTokPtr = ptr.add(next);
                    return token;
                }
                NormalIgnoreSectionOutcome::Partial(token) => return token,
                NormalIgnoreSectionOutcome::Invalid(at) => {
                    *nextTokPtr = ptr.add(at);
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
                NormalIgnoreSectionOutcome::UnknownInvalid {
                    at,
                    width,
                    level: saved_level,
                } => {
                    if unknown_isInvalid(enc, ptr.add(at)) != 0 {
                        *nextTokPtr = ptr.add(at);
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                    start = at + width;
                    level = saved_level;
                }
            }
        }
    }

    pub unsafe extern "C" fn normal_isPublicId(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        badPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let span_len = end.offset_from(ptr);
        if span_len < 2 {
            return 1;
        }
        let contents =
            ::core::slice::from_raw_parts(ptr.add(1).cast::<u8>(), (span_len - 2) as usize);
        let byte_types = &(*(enc as *const normal_encoding)).type_0;
        if let Some(offset) = crate::src::xmltok::public_id_bad_offset(
            contents,
            byte_types,
            crate::src::xmltok::PublicIdChecker::Normal,
        ) {
            *badPtr = ptr.add(1 + offset);
            return 0;
        }
        1
    }

    #[derive(Copy, Clone)]
    enum NormalAttributeAction {
        Name { attribute: ::core::ffi::c_int, offset: usize },
        ValueStart { attribute: ::core::ffi::c_int, offset: usize },
        ValueEnd { attribute: ::core::ffi::c_int, offset: usize },
        Normalized { attribute: ::core::ffi::c_int, value: ::core::ffi::c_char },
    }

    /// Scans a complete single-byte start-tag token using only bounded slice
    /// indices, reporting offsets for the boundary adapter to translate.
    fn scan_normal_atts(
        byte_types: &[::core::ffi::c_uchar; 256],
        source: &[u8],
        mut report: impl FnMut(NormalAttributeAction),
    ) -> ::core::ffi::c_int {
        #[derive(Copy, Clone, Eq, PartialEq)]
        enum State { InName, InValue, Other }

        let mut state = State::InName;
        let mut n_atts = 0;
        let mut open = 0;
        let mut value_start = None;
        let mut normalized = true;
        let mut index = 1;

        while let Some(&byte) = source.get(index) {
            let kind = byte_types[byte as usize] as ::core::ffi::c_int;
            match kind {
                5 | 6 | 7 | 29 | 22 | 24 => {
                    if state == State::Other {
                        report(NormalAttributeAction::Name { attribute: n_atts, offset: index });
                        normalized = true;
                        report(NormalAttributeAction::Normalized { attribute: n_atts, value: 1 });
                        state = State::InName;
                    }
                    index += match kind { 5 => 2, 6 => 3, 7 => 4, _ => 1 };
                    continue;
                }
                12 | 13 => {
                    let quote_kind = if kind == 12 {
                        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int
                    } else {
                        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int
                    };
                    if state != State::InValue {
                        report(NormalAttributeAction::ValueStart { attribute: n_atts, offset: index + 1 });
                        value_start = Some(index + 1);
                        state = State::InValue;
                        open = quote_kind;
                    } else if open == quote_kind {
                        state = State::Other;
                        report(NormalAttributeAction::ValueEnd { attribute: n_atts, offset: index });
                        n_atts += 1;
                        value_start = None;
                    }
                }
                3 => {
                    normalized = false;
                    report(NormalAttributeAction::Normalized { attribute: n_atts, value: 0 });
                }
                21 => {
                    if state == State::InName {
                        state = State::Other;
                    } else if state == State::InValue && normalized
                        && (value_start == Some(index)
                            || byte != crate::ascii_h::ASCII_SPACE as u8
                            || source.get(index + 1) == Some(&(crate::ascii_h::ASCII_SPACE as u8))
                            || source.get(index + 1)
                                .map(|next| byte_types[*next as usize] as ::core::ffi::c_int)
                                == Some(open))
                    {
                        normalized = false;
                        report(NormalAttributeAction::Normalized { attribute: n_atts, value: 0 });
                    }
                }
                9 | 10 => {
                    if state == State::InName {
                        state = State::Other;
                    } else if state == State::InValue {
                        normalized = false;
                        report(NormalAttributeAction::Normalized { attribute: n_atts, value: 0 });
                    }
                }
                11 | 17 if state != State::InValue => return n_atts,
                _ => {}
            }
            index += 1;
        }
        n_atts
    }

    /// Boundary adapter for the parser's bounded start-tag token.
    pub unsafe extern "C" fn normal_getAtts(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        attsMax: ::core::ffi::c_int,
        atts: *mut crate::src::xmltok::ATTRIBUTE,
    ) -> ::core::ffi::c_int {
        let source_len = end.offset_from(ptr);
        if source_len < 0 {
            return 0;
        }
        let source = ::core::slice::from_raw_parts(ptr.cast::<u8>(), source_len as usize);
        let byte_types = &(*(enc as *const normal_encoding)).type_0;
        scan_normal_atts(byte_types, source, |action| {
            let attribute = match action {
                NormalAttributeAction::Name { attribute, .. }
                | NormalAttributeAction::ValueStart { attribute, .. }
                | NormalAttributeAction::ValueEnd { attribute, .. }
                | NormalAttributeAction::Normalized { attribute, .. } => attribute,
            };
            if attribute < 0 || attribute >= attsMax {
                return;
            }
            let slot = atts.add(attribute as usize);
            match action {
                NormalAttributeAction::Name { offset, .. } => (*slot).name = ptr.add(offset),
                NormalAttributeAction::ValueStart { offset, .. } => (*slot).valuePtr = ptr.add(offset),
                NormalAttributeAction::ValueEnd { offset, .. } => (*slot).valueEnd = ptr.add(offset),
                NormalAttributeAction::Normalized { value, .. } => (*slot).normalized = value,
            }
        })
    }

    pub unsafe extern "C" fn normal_charRefNumber(
        _enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        ptr = ptr.wrapping_add((2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as usize);
        if *ptr as ::core::ffi::c_int == 0x78 as ::core::ffi::c_int {
            ptr = ptr.wrapping_add(1 as ::core::ffi::c_int as usize);
            while *ptr as ::core::ffi::c_int != 0x3b as ::core::ffi::c_int {
                let mut c: ::core::ffi::c_int = *ptr as ::core::ffi::c_int;
                match c {
                    crate::ascii_h::ASCII_0
                    | crate::ascii_h::ASCII_1_1
                    | crate::ascii_h::ASCII_2_1
                    | crate::ascii_h::ASCII_3_1
                    | crate::ascii_h::ASCII_4
                    | crate::ascii_h::ASCII_5
                    | crate::ascii_h::ASCII_6
                    | crate::ascii_h::ASCII_7
                    | crate::ascii_h::ASCII_8_1
                    | crate::ascii_h::ASCII_9_1 => {
                        result <<= 4 as ::core::ffi::c_int;
                        result |= c - crate::ascii_h::ASCII_0;
                    }
                    crate::ascii_h::ASCII_A
                    | crate::ascii_h::ASCII_B_1
                    | crate::ascii_h::ASCII_C
                    | crate::ascii_h::ASCII_D
                    | crate::ascii_h::ASCII_E_1
                    | crate::ascii_h::ASCII_F_1 => {
                        result <<= 4 as ::core::ffi::c_int;
                        result += 10 as ::core::ffi::c_int + (c - crate::ascii_h::ASCII_A);
                    }
                    crate::ascii_h::ASCII_a_1
                    | crate::ascii_h::ASCII_b
                    | crate::ascii_h::ASCII_c_1
                    | crate::ascii_h::ASCII_d
                    | crate::ascii_h::ASCII_e_1
                    | crate::ascii_h::ASCII_f => {
                        result <<= 4 as ::core::ffi::c_int;
                        result += 10 as ::core::ffi::c_int + (c - crate::ascii_h::ASCII_a_1);
                    }
                    _ => {}
                }
                if result >= 0x110000 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.wrapping_add(1 as ::core::ffi::c_int as usize);
            }
        } else {
            while *ptr as ::core::ffi::c_int != 0x3b as ::core::ffi::c_int {
                let mut c_0: ::core::ffi::c_int = *ptr as ::core::ffi::c_int;
                result *= 10 as ::core::ffi::c_int;
                result += c_0 - crate::ascii_h::ASCII_0;
                if result >= 0x110000 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.wrapping_add(1 as ::core::ffi::c_int as usize);
            }
        }
        return checkCharRefNumber(result);
    }

    pub unsafe extern "C" fn normal_nameLength(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ptr;
        loop {
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
                29 | 22 | 23 | 24 | 25 | 26 | 27 => {
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                }
                _ => return ptr.offset_from(start) as ::core::ffi::c_int,
            }
        }
    }

    pub unsafe fn skip_s(
        enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        skipper: crate::src::xmltok::WhitespaceSkipper,
    ) -> *const ::core::ffi::c_char {
        match skipper {
            crate::src::xmltok::WhitespaceSkipper::Normal => loop {
                match (*(enc as *const normal_encoding)).type_0
                    [*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                {
                    10 | 9 | 21 => ptr = ptr.offset(1),
                    _ => return ptr,
                }
            },
            crate::src::xmltok::WhitespaceSkipper::Little2 => loop {
                match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr)
                } {
                    10 | 9 | 21 => ptr = ptr.offset(2),
                    _ => return ptr,
                }
            },
            crate::src::xmltok::WhitespaceSkipper::Big2 => loop {
                match if *ptr as ::core::ffi::c_int == 0 {
                    (*(enc as *const normal_encoding)).type_0
                        [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr, *ptr.offset(1))
                } {
                    10 | 9 | 21 => ptr = ptr.offset(2),
                    _ => return ptr,
                }
            },
        }
    }

    pub unsafe extern "C" fn normal_updatePosition(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut pos: *mut crate::src::xmltok::POSITION,
    ) {
        let pos = &mut *pos;
        while end.offset_from(ptr) >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize {
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                5 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                6 => {
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                7 => {
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                10 => {
                    pos.columnNumber = 0 as crate::expat_external_h::XML_Size;
                    pos.lineNumber = pos.lineNumber.wrapping_add(1);
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                }
                9 => {
                    pos.lineNumber = pos.lineNumber.wrapping_add(1);
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                    if end.offset_from(ptr)
                        >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize
                        && (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                            == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                    {
                        ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                    }
                    pos.columnNumber = 0 as crate::expat_external_h::XML_Size;
                }
                _ => {
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
            }
        }
    }

    enum Little2ScanOutcome {
        Token(::core::ffi::c_int, usize),
        Partial(::core::ffi::c_int),
        Invalid(usize),
    }

    /// Scan a UTF-16LE comment body using offsets into the bounded input.
    /// The caller translates the result back to the ABI cursor pointer.
    fn little2_scan_comment_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> Little2ScanOutcome {
        if input.len() < 2 {
            return Little2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
        }
        if input[0] != b'-' as ::core::ffi::c_char || input[1] != 0 {
            return Little2ScanOutcome::Invalid(0);
        }

        let mut pos = 2;
        while pos + 2 <= input.len() {
            match little2_byte_type(&enc.type_0, input, pos) {
                5 => {
                    if input.len() - pos < 2 {
                        return Little2ScanOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    pos += 2;
                }
                6 => {
                    if input.len() - pos < 3 {
                        return Little2ScanOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    pos += 3;
                }
                7 => {
                    if input.len() - pos < 4 {
                        return Little2ScanOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    pos += 4;
                }
                0 | 1 | 8 => return Little2ScanOutcome::Invalid(pos),
                27 => {
                    pos += 2;
                    if pos + 2 > input.len() {
                        return Little2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
                    }
                    if input[pos] != b'-' as ::core::ffi::c_char || input[pos + 1] != 0 {
                        return Little2ScanOutcome::Invalid(pos);
                    }
                    pos += 2;
                    if pos + 2 > input.len() {
                        return Little2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
                    }
                    return if input[pos] == b'>' as ::core::ffi::c_char && input[pos + 1] == 0 {
                        Little2ScanOutcome::Token(crate::src::xmltok::XML_TOK_COMMENT_1, pos + 2)
                    } else {
                        Little2ScanOutcome::Invalid(pos)
                    };
                }
                _ => pos += 2,
            }
        }
        Little2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1)
    }

    pub unsafe extern "C" fn little2_scanComment(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = end.offset_from(ptr);
        if input_len < 0 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = ::core::slice::from_raw_parts(ptr, input_len as usize);
        let normal = &*(enc as *const normal_encoding);
        match little2_scan_comment_impl(normal, input) {
            Little2ScanOutcome::Token(token, next) => {
                *nextTokPtr = ptr.add(next);
                token
            }
            Little2ScanOutcome::Partial(token) => token,
            Little2ScanOutcome::Invalid(at) => {
                *nextTokPtr = ptr.add(at);
                crate::src::xmltok::XML_TOK_INVALID_1
            }
        }
    }

    enum Little2ScanDeclAction {
        ScanComment,
        Return { token: ::core::ffi::c_int, next: Option<usize> },
    }

    /// Scan the declaration prefix using bounded UTF-16LE code units.  The
    /// boundary adapter owns converting the returned cursor offset to a C
    /// pointer (and the comment scanner still has its own boundary adapter).
    fn little2_scan_decl_impl(
        byte_types: &[::core::ffi::c_uchar; 256],
        input: &[::core::ffi::c_char],
    ) -> Little2ScanDeclAction {
        let partial = crate::src::xmltok::XML_TOK_PARTIAL_1;
        let invalid = crate::src::xmltok::XML_TOK_INVALID_1;
        if input.len() < 2 {
            return Little2ScanDeclAction::Return { token: partial, next: None };
        }

        match little2_byte_type(byte_types, input, 0) {
            27 => return Little2ScanDeclAction::ScanComment,
            20 => {
                return Little2ScanDeclAction::Return {
                    token: crate::src::xmltok::XML_TOK_COND_SECT_OPEN_1,
                    next: Some(2),
                };
            }
            22 | 24 => {}
            _ => return Little2ScanDeclAction::Return { token: invalid, next: Some(0) },
        }

        let mut ptr = 2;
        while input.len().saturating_sub(ptr) >= 2 {
            match little2_byte_type(byte_types, input, ptr) {
                30 => {
                    if input.len().saturating_sub(ptr) < 4 {
                        return Little2ScanDeclAction::Return { token: partial, next: None };
                    }
                    if matches!(little2_byte_type(byte_types, input, ptr + 2), 21 | 9 | 10 | 30) {
                        return Little2ScanDeclAction::Return { token: invalid, next: Some(ptr) };
                    }
                    return Little2ScanDeclAction::Return {
                        token: crate::src::xmltok::XML_TOK_DECL_OPEN_1,
                        next: Some(ptr),
                    };
                }
                21 | 9 | 10 => {
                    return Little2ScanDeclAction::Return {
                        token: crate::src::xmltok::XML_TOK_DECL_OPEN_1,
                        next: Some(ptr),
                    };
                }
                22 | 24 => ptr += 2,
                _ => return Little2ScanDeclAction::Return { token: invalid, next: Some(ptr) },
            }
        }

        Little2ScanDeclAction::Return { token: partial, next: None }
    }

    pub unsafe extern "C" fn little2_scanDecl(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = end.offset_from(ptr);
        if input_len < 0 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = ::core::slice::from_raw_parts(ptr, input_len as usize);
        let normal = &*(enc as *const normal_encoding);
        match little2_scan_decl_impl(&normal.type_0, input) {
            Little2ScanDeclAction::ScanComment => little2_scanComment(enc, ptr.add(2), end, nextTokPtr),
            Little2ScanDeclAction::Return { token, next } => {
                if let Some(offset) = next {
                    *nextTokPtr = ptr.add(offset);
                }
                token
            }
        }
    }

    pub unsafe extern "C" fn little2_checkPiTarget(
        _enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut tokPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        let mut upper: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        *tokPtr = crate::src::xmltok::XML_TOK_PI_1;
        if end.offset_from(ptr) != (2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize {
            return 1 as ::core::ffi::c_int;
        }
        match if *ptr.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            *ptr.offset(0 as isize) as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        } {
            crate::ascii_h::ASCII_x_1 => {}
            crate::ascii_h::ASCII_X_1 => {
                upper = 1 as ::core::ffi::c_int;
            }
            _ => return 1 as ::core::ffi::c_int,
        }
        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
        match if *ptr.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            *ptr.offset(0 as isize) as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        } {
            crate::ascii_h::ASCII_m_1 => {}
            crate::ascii_h::ASCII_M_1 => {
                upper = 1 as ::core::ffi::c_int;
            }
            _ => return 1 as ::core::ffi::c_int,
        }
        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
        match if *ptr.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            *ptr.offset(0 as isize) as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        } {
            crate::ascii_h::ASCII_l_1 => {}
            crate::ascii_h::ASCII_L_1 => {
                upper = 1 as ::core::ffi::c_int;
            }
            _ => return 1 as ::core::ffi::c_int,
        }
        if upper != 0 {
            return 0 as ::core::ffi::c_int;
        }
        *tokPtr = crate::src::xmltok::XML_TOK_XML_DECL_1;
        return 1 as ::core::ffi::c_int;
    }

    fn little2_pi_target_token(
        input: &[::core::ffi::c_char],
        target: usize,
        terminator: usize,
    ) -> Option<::core::ffi::c_int> {
        if terminator - target != 6 {
            return Some(crate::src::xmltok::XML_TOK_PI_1);
        }

        let mut upper = false;
        for (offset, lower, upper_case) in [
            (0, crate::ascii_h::ASCII_x_1, crate::ascii_h::ASCII_X_1),
            (2, crate::ascii_h::ASCII_m_1, crate::ascii_h::ASCII_M_1),
            (4, crate::ascii_h::ASCII_l_1, crate::ascii_h::ASCII_L_1),
        ] {
            if input[target + offset + 1] != 0 {
                return Some(crate::src::xmltok::XML_TOK_PI_1);
            }
            match input[target + offset] as ::core::ffi::c_int {
                value if value == lower => {}
                value if value == upper_case => upper = true,
                _ => return Some(crate::src::xmltok::XML_TOK_PI_1),
            }
        }

        if upper {
            None
        } else {
            Some(crate::src::xmltok::XML_TOK_XML_DECL_1)
        }
    }

    fn little2_scan_pi_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> (::core::ffi::c_int, Option<usize>) {
        if input.len() < 2 {
            return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }

        let target = 0;
        let mut offset = 0;
        match little2_byte_type(&enc.type_0, input, offset) {
            29 => {
                if !little2_in_name_bitmap(input, offset, &nmstrtPages) {
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                }
            }
            22 | 24 => {}
            5 => {
                if input.len() < 2 {
                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                }
                return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
            }
            6 => {
                if input.len() < 3 {
                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                }
                return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
            }
            7 => {
                if input.len() < 4 {
                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                }
                return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
            }
            _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
        }
        offset += 2;

        while input.len() - offset >= 2 {
            match little2_byte_type(&enc.type_0, input, offset) {
                29 => {
                    if !little2_in_name_bitmap(input, offset, &namePages) {
                        return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                    }
                    offset += 2;
                }
                22 | 24 | 25 | 26 | 27 => offset += 2,
                5 => {
                    if input.len() - offset < 2 {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                    }
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                }
                6 => {
                    if input.len() - offset < 3 {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                    }
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                }
                7 => {
                    if input.len() - offset < 4 {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                    }
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                }
                21 | 9 | 10 => {
                    let Some(token) = little2_pi_target_token(input, target, offset) else {
                        return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                    };
                    offset += 2;
                    while input.len() - offset >= 2 {
                        match little2_byte_type(&enc.type_0, input, offset) {
                            5 => {
                                if input.len() - offset < 2 {
                                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                                }
                                offset += 2;
                            }
                            6 => {
                                if input.len() - offset < 3 {
                                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                                }
                                offset += 3;
                            }
                            7 => {
                                if input.len() - offset < 4 {
                                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                                }
                                offset += 4;
                            }
                            0 | 1 | 8 => {
                                return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                            }
                            15 => {
                                offset += 2;
                                if input.len() - offset < 2 {
                                    return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                                }
                                if input[offset] as u8 == 0x3e && input[offset + 1] == 0 {
                                    return (token, Some(offset + 2));
                                }
                            }
                            _ => offset += 2,
                        }
                    }
                    return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                }
                15 => {
                    let Some(token) = little2_pi_target_token(input, target, offset) else {
                        return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                    };
                    offset += 2;
                    if input.len() - offset < 2 {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                    }
                    if input[offset] as u8 == 0x3e && input[offset + 1] == 0 {
                        return (token, Some(offset + 2));
                    }
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                }
                _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
            }
        }

        (crate::src::xmltok::XML_TOK_PARTIAL_1, None)
    }

    pub unsafe extern "C" fn little2_scanPi(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = unsafe { end.offset_from(ptr) };
        if input_len <= 0 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = unsafe { ::core::slice::from_raw_parts(ptr, input_len as usize) };
        if input.len() < 2 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let enc = unsafe { &*(enc as *const normal_encoding) };
        let (token, next) = little2_scan_pi_impl(enc, input);
        if let Some(offset) = next {
            unsafe { *nextTokPtr = ptr.add(offset) };
        }
        token
    }

    pub unsafe extern "C" fn little2_scanCdataSection(
        _enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        pub static mut CDATA_LSQB: [::core::ffi::c_char; 6] = [
            crate::ascii_h::ASCII_C as ::core::ffi::c_char,
            crate::ascii_h::ASCII_D as ::core::ffi::c_char,
            crate::ascii_h::ASCII_A as ::core::ffi::c_char,
            crate::ascii_h::ASCII_T as ::core::ffi::c_char,
            crate::ascii_h::ASCII_A as ::core::ffi::c_char,
            crate::ascii_h::ASCII_LSQB as ::core::ffi::c_char,
        ];
        let mut i: ::core::ffi::c_int = 0;
        if !(end.offset_from(ptr) >= (6 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize) {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < 6 as ::core::ffi::c_int {
            if !(*ptr.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && *ptr.offset(0 as isize) as ::core::ffi::c_int
                    == CDATA_LSQB[i as usize] as ::core::ffi::c_int)
            {
                *nextTokPtr = ptr;
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            i += 1;
            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
        }
        *nextTokPtr = ptr;
        return crate::src::xmltok::XML_TOK_CDATA_SECT_OPEN_1;
    }

    struct Little2CdataSectionResult {
        token: ::core::ffi::c_int,
        next: Option<usize>,
    }

    fn little2_cdata_section_result(
        token: ::core::ffi::c_int,
        next: Option<usize>,
    ) -> Little2CdataSectionResult {
        Little2CdataSectionResult { token, next }
    }

    fn little2_cdata_section_tok_impl(
        enc: &normal_encoding,
        input: &[u8],
    ) -> Little2CdataSectionResult {
        if input.is_empty() {
            return little2_cdata_section_result(crate::src::xmltok::XML_TOK_NONE_1, None);
        }

        let input = &input[..input.len() & !1];
        if input.is_empty() {
            return little2_cdata_section_result(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }

        let byte_type = |offset: usize| {
            if input[offset + 1] == 0 {
                enc.type_0[input[offset] as usize] as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    input[offset + 1] as ::core::ffi::c_char,
                    input[offset] as ::core::ffi::c_char,
                )
            }
        };

        let mut offset = match byte_type(0) {
            4 => {
                if input.len() < 4 {
                    return little2_cdata_section_result(
                        crate::src::xmltok::XML_TOK_PARTIAL_1,
                        None,
                    );
                }
                if input[3] == 0 && input[2] == b']' {
                    if input.len() < 6 {
                        return little2_cdata_section_result(
                            crate::src::xmltok::XML_TOK_PARTIAL_1,
                            None,
                        );
                    }
                    if input[5] == 0 && input[4] == b'>' {
                        return little2_cdata_section_result(
                            crate::src::xmltok::XML_TOK_CDATA_SECT_CLOSE_1,
                            Some(6),
                        );
                    }
                }
                2
            }
            9 => {
                if input.len() < 4 {
                    return little2_cdata_section_result(
                        crate::src::xmltok::XML_TOK_PARTIAL_1,
                        None,
                    );
                }
                let next = if byte_type(2) == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int {
                    4
                } else {
                    2
                };
                return little2_cdata_section_result(
                    crate::src::xmltok::XML_TOK_DATA_NEWLINE_1,
                    Some(next),
                );
            }
            10 => {
                return little2_cdata_section_result(
                    crate::src::xmltok::XML_TOK_DATA_NEWLINE_1,
                    Some(2),
                );
            }
            5 => {
                if input.len() < 2 {
                    return little2_cdata_section_result(
                        crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        None,
                    );
                }
                2
            }
            6 => {
                if input.len() < 3 {
                    return little2_cdata_section_result(
                        crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        None,
                    );
                }
                3
            }
            7 => {
                if input.len() < 4 {
                    return little2_cdata_section_result(
                        crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        None,
                    );
                }
                4
            }
            0 | 1 | 8 => {
                return little2_cdata_section_result(
                    crate::src::xmltok::XML_TOK_INVALID_1,
                    Some(0),
                );
            }
            _ => 2,
        };

        while input.len().saturating_sub(offset) >= 2 {
            match byte_type(offset) {
                5 => {
                    if input.len() - offset < 2 {
                        return little2_cdata_section_result(
                            crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                            Some(offset),
                        );
                    }
                    offset += 2;
                }
                6 => {
                    if input.len() - offset < 3 {
                        return little2_cdata_section_result(
                            crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                            Some(offset),
                        );
                    }
                    offset += 3;
                }
                7 => {
                    if input.len() - offset < 4 {
                        return little2_cdata_section_result(
                            crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                            Some(offset),
                        );
                    }
                    offset += 4;
                }
                0 | 1 | 8 | 9 | 10 | 4 => {
                    return little2_cdata_section_result(
                        crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        Some(offset),
                    );
                }
                _ => offset += 2,
            }
        }

        little2_cdata_section_result(crate::src::xmltok::XML_TOK_DATA_CHARS_1, Some(offset))
    }

    pub unsafe extern "C" fn little2_cdataSectionTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if ptr >= end {
            return crate::src::xmltok::XML_TOK_NONE_1;
        }
        let input_len = end.offset_from(ptr) as usize;
        let input = ::core::slice::from_raw_parts(ptr.cast::<u8>(), input_len);
        let normal = &*(enc as *const normal_encoding);
        let result = little2_cdata_section_tok_impl(normal, input);
        if let Some(next) = result.next {
            *nextTokPtr = ptr.add(next);
        }
        result.token
    }

    fn little2_scan_end_tag_impl(enc: &normal_encoding, input: &[u8]) -> Little2ScanLtAction {
        if input.len() < 2 {
            return Little2ScanLtAction::Token(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }

        let mut ptr = match little2_scan_lt_name(enc, input, 0, true) {
            Ok(next) => next,
            Err(action) => return action,
        };

        while input.len() - ptr >= 2 {
            match little2_scan_lt_name(enc, input, ptr, false) {
                Ok(next) => {
                    ptr = next;
                    continue;
                }
                Err(Little2ScanLtAction::Token(crate::src::xmltok::XML_TOK_INVALID_1, Some(_))) => {
                }
                Err(action) => return action,
            }

            match little2_scan_lt_type(enc, input, ptr) {
                23 => ptr += 2,
                21 | 9 | 10 => {
                    ptr += 2;
                    while input.len() - ptr >= 2
                        && matches!(little2_scan_lt_type(enc, input, ptr), 21 | 9 | 10)
                    {
                        ptr += 2;
                    }
                    if input.len() - ptr < 2 {
                        return Little2ScanLtAction::Token(
                            crate::src::xmltok::XML_TOK_PARTIAL_1,
                            None,
                        );
                    }
                    return if little2_scan_lt_type(enc, input, ptr) == 11 {
                        Little2ScanLtAction::Token(
                            crate::src::xmltok::XML_TOK_END_TAG_1,
                            Some(ptr + 2),
                        )
                    } else {
                        Little2ScanLtAction::Token(crate::src::xmltok::XML_TOK_INVALID_1, Some(ptr))
                    };
                }
                11 => {
                    return Little2ScanLtAction::Token(
                        crate::src::xmltok::XML_TOK_END_TAG_1,
                        Some(ptr + 2),
                    )
                }
                _ => {
                    return Little2ScanLtAction::Token(
                        crate::src::xmltok::XML_TOK_INVALID_1,
                        Some(ptr),
                    )
                }
            }
        }

        Little2ScanLtAction::Token(crate::src::xmltok::XML_TOK_PARTIAL_1, None)
    }

    pub unsafe extern "C" fn little2_scanEndTag(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = end.offset_from(ptr);
        if input_len < 0 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = ::core::slice::from_raw_parts(ptr.cast::<u8>(), input_len as usize);
        let normal = &*(enc as *const normal_encoding);
        match little2_scan_end_tag_impl(normal, input) {
            Little2ScanLtAction::Token(token, next) => {
                if let Some(next) = next {
                    *nextTokPtr = ptr.add(next);
                }
                token
            }
            _ => unreachable!("end-tag scanning produces only token outcomes"),
        }
    }

    pub unsafe extern "C" fn little2_scanHexCharRef(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if end.offset_from(ptr) >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize {
            match if *ptr.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1 as isize), *ptr.offset(0 as isize))
            } {
                25 | 24 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
            }
            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            while end.offset_from(ptr)
                >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize
            {
                match if *ptr.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(1 as isize), *ptr.offset(0 as isize))
                } {
                    25 | 24 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        return crate::src::xmltok::XML_TOK_CHAR_REF_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                }
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            }
        }
        return crate::src::xmltok::XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn little2_scanCharRef(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if end.offset_from(ptr) >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize {
            if *ptr.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && *ptr.offset(0 as isize) as ::core::ffi::c_int == 0x78 as ::core::ffi::c_int
            {
                return little2_scanHexCharRef(
                    enc,
                    ptr.offset(2 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            match if *ptr.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1 as isize), *ptr.offset(0 as isize))
            } {
                25 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
            }
            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            while end.offset_from(ptr)
                >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize
            {
                match if *ptr.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(1 as isize), *ptr.offset(0 as isize))
                } {
                    25 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        return crate::src::xmltok::XML_TOK_CHAR_REF_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                }
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            }
        }
        return crate::src::xmltok::XML_TOK_PARTIAL_1;
    }

    // Retained as a disabled translation reference.  The active scanner below
    // performs the same state transitions over a checked input slice.
    #[cfg(any())]
    pub unsafe extern "C" fn little2_scanRef_legacy(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize) {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        's_138: {
            match if *ptr.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1 as isize), *ptr.offset(0 as isize))
            } {
                29 => {
                    if namingBitmap[(((nmstrtPages
                        [*ptr.offset(1 as isize) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int)
                        << 3 as ::core::ffi::c_int)
                        + (*ptr.offset(0 as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int
                            >> 5 as ::core::ffi::c_int))
                        as usize]
                        & (1 as ::core::ffi::c_uint)
                            << (*ptr.offset(0 as isize) as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1f as ::core::ffi::c_int)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                }
                22 | 24 => {}
                5 => {
                    if end.offset_from(ptr) < 2 as isize {
                        return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    break 's_138;
                }
                6 => {
                    if end.offset_from(ptr) < 3 as isize {
                        return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                    break 's_138;
                }
                7 => {
                    if end.offset_from(ptr) < 4 as isize {
                        return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    break 's_138;
                }
                19 => {
                    return little2_scanCharRef(
                        enc,
                        ptr.offset(2 as ::core::ffi::c_int as isize),
                        end,
                        nextTokPtr,
                    );
                }
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
            }
            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
        }
        while end.offset_from(ptr) >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize {
            's_275: {
                match if *ptr.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(1 as isize), *ptr.offset(0 as isize))
                } {
                    29 => {
                        if namingBitmap[(((namePages
                            [*ptr.offset(1 as isize) as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int)
                            << 3 as ::core::ffi::c_int)
                            + (*ptr.offset(0 as isize) as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                >> 5 as ::core::ffi::c_int))
                            as usize]
                            & (1 as ::core::ffi::c_uint)
                                << (*ptr.offset(0 as isize) as ::core::ffi::c_uchar
                                    as ::core::ffi::c_int
                                    & 0x1f as ::core::ffi::c_int)
                            == 0
                        {
                            *nextTokPtr = ptr;
                            return crate::src::xmltok::XML_TOK_INVALID_1;
                        }
                    }
                    22 | 24 | 25 | 26 | 27 => {}
                    5 => {
                        if end.offset_from(ptr) < 2 as isize {
                            return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                        }
                        if false || true {
                            *nextTokPtr = ptr;
                            return crate::src::xmltok::XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        break 's_275;
                    }
                    6 => {
                        if end.offset_from(ptr) < 3 as isize {
                            return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                        }
                        if false || true {
                            *nextTokPtr = ptr;
                            return crate::src::xmltok::XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                        break 's_275;
                    }
                    7 => {
                        if end.offset_from(ptr) < 4 as isize {
                            return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                        }
                        if false || true {
                            *nextTokPtr = ptr;
                            return crate::src::xmltok::XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                        break 's_275;
                    }
                    18 => {
                        *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        return crate::src::xmltok::XML_TOK_ENTITY_REF_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                }
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            }
        }
        return crate::src::xmltok::XML_TOK_PARTIAL_1;
    }

    struct Little2ScanResult {
        token: ::core::ffi::c_int,
        next: Option<usize>,
    }

    enum Little2NameCheck {
        Advance(usize),
        PartialChar,
        Invalid,
    }

    fn little2_check_name(
        byte_types: &[::core::ffi::c_uchar; 256],
        input: &[::core::ffi::c_char],
        pos: usize,
        start: bool,
    ) -> Little2NameCheck {
        match little2_byte_type(byte_types, input, pos) {
            29 => {
                let pages = if start { &nmstrtPages } else { &namePages };
                if little2_in_name_bitmap(input, pos, pages) {
                    Little2NameCheck::Advance(pos + 2)
                } else {
                    Little2NameCheck::Invalid
                }
            }
            22 | 24 if start => Little2NameCheck::Advance(pos + 2),
            22 | 24 | 25 | 26 | 27 if !start => Little2NameCheck::Advance(pos + 2),
            5 => {
                if input.len() - pos < 2 {
                    Little2NameCheck::PartialChar
                } else {
                    Little2NameCheck::Invalid
                }
            }
            6 => {
                if input.len() - pos < 3 {
                    Little2NameCheck::PartialChar
                } else {
                    Little2NameCheck::Invalid
                }
            }
            7 => {
                if input.len() - pos < 4 {
                    Little2NameCheck::PartialChar
                } else {
                    Little2NameCheck::Invalid
                }
            }
            _ => Little2NameCheck::Invalid,
        }
    }

    fn little2_scan_char_ref_impl(
        byte_types: &[::core::ffi::c_uchar; 256],
        input: &[::core::ffi::c_char],
        mut pos: usize,
    ) -> Little2ScanResult {
        if pos + 2 > input.len() {
            return Little2ScanResult {
                token: crate::src::xmltok::XML_TOK_PARTIAL_1,
                next: None,
            };
        }

        let hex = input[pos + 1] == 0 && input[pos] == b'x' as ::core::ffi::c_char;
        if hex {
            pos += 2;
            if pos + 2 > input.len() {
                return Little2ScanResult {
                    token: crate::src::xmltok::XML_TOK_PARTIAL_1,
                    next: None,
                };
            }
        }

        match little2_byte_type(byte_types, input, pos) {
            25 | 24 if hex => {}
            25 if !hex => {}
            _ => {
                return Little2ScanResult {
                    token: crate::src::xmltok::XML_TOK_INVALID_1,
                    next: Some(pos),
                }
            }
        }
        pos += 2;

        while pos + 2 <= input.len() {
            match little2_byte_type(byte_types, input, pos) {
                25 | 24 if hex => pos += 2,
                25 if !hex => pos += 2,
                18 => {
                    return Little2ScanResult {
                        token: crate::src::xmltok::XML_TOK_CHAR_REF_1,
                        next: Some(pos + 2),
                    }
                }
                _ => {
                    return Little2ScanResult {
                        token: crate::src::xmltok::XML_TOK_INVALID_1,
                        next: Some(pos),
                    }
                }
            }
        }

        Little2ScanResult {
            token: crate::src::xmltok::XML_TOK_PARTIAL_1,
            next: None,
        }
    }

    fn little2_scan_ref_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> Little2ScanResult {
        if input.len() < 2 {
            return Little2ScanResult {
                token: crate::src::xmltok::XML_TOK_PARTIAL_1,
                next: None,
            };
        }

        let byte_types = &enc.type_0;
        let mut pos = match little2_check_name(byte_types, input, 0, true) {
            Little2NameCheck::Advance(next) => next,
            Little2NameCheck::PartialChar => {
                return Little2ScanResult {
                    token: crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                    next: None,
                }
            }
            Little2NameCheck::Invalid => {
                if little2_byte_type(byte_types, input, 0) == 19 {
                    return little2_scan_char_ref_impl(byte_types, input, 2);
                }
                return Little2ScanResult {
                    token: crate::src::xmltok::XML_TOK_INVALID_1,
                    next: Some(0),
                };
            }
        };

        while pos + 2 <= input.len() {
            match little2_check_name(byte_types, input, pos, false) {
                Little2NameCheck::Advance(next) => pos = next,
                Little2NameCheck::PartialChar => {
                    return Little2ScanResult {
                        token: crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        next: None,
                    }
                }
                Little2NameCheck::Invalid => {
                    if little2_byte_type(byte_types, input, pos) == 18 {
                        return Little2ScanResult {
                            token: crate::src::xmltok::XML_TOK_ENTITY_REF_1,
                            next: Some(pos + 2),
                        };
                    }
                    return Little2ScanResult {
                        token: crate::src::xmltok::XML_TOK_INVALID_1,
                        next: Some(pos),
                    };
                }
            }
        }

        Little2ScanResult {
            token: crate::src::xmltok::XML_TOK_PARTIAL_1,
            next: None,
        }
    }

    pub unsafe extern "C" fn little2_scanRef(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = end.offset_from(ptr);
        if input_len < 0 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = ::core::slice::from_raw_parts(ptr, input_len as usize);
        let normal = &*(enc as *const normal_encoding);
        let result = little2_scan_ref_impl(normal, input);
        if let Some(next) = result.next {
            *nextTokPtr = ptr.add(next);
        }
        result.token
    }

    fn little2_byte_type(
        byte_types: &[::core::ffi::c_uchar; 256],
        input: &[::core::ffi::c_char],
        ptr: usize,
    ) -> ::core::ffi::c_int {
        let lo = input[ptr] as ::core::ffi::c_uchar;
        let hi = input[ptr + 1] as ::core::ffi::c_uchar;
        if hi == 0 {
            byte_types[lo as usize] as ::core::ffi::c_int
        } else {
            unicode_byte_type(hi as ::core::ffi::c_char, lo as ::core::ffi::c_char)
        }
    }

    fn little2_in_name_bitmap(
        input: &[::core::ffi::c_char],
        ptr: usize,
        pages: &[::core::ffi::c_uchar; 256],
    ) -> bool {
        let lo = input[ptr] as ::core::ffi::c_uchar;
        let hi = input[ptr + 1] as ::core::ffi::c_uchar;
        let word = (pages[hi as usize] as usize) * 8 + (lo as usize >> 5);
        namingBitmap[word] & (1 << (lo & 0x1f)) != 0
    }

    fn little2_scan_atts_impl<F>(
        byte_types: &[::core::ffi::c_uchar; 256],
        input: &[::core::ffi::c_char],
        mut scan_ref: F,
    ) -> Little2ScanResult
    where
        F: FnMut(usize) -> (::core::ffi::c_int, usize),
    {
        let partial = crate::src::xmltok::XML_TOK_PARTIAL_1;
        let partial_char = crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
        let invalid = crate::src::xmltok::XML_TOK_INVALID_1;
        let mut ptr = 0;
        let mut had_colon = false;

        'attribute: while input.len().saturating_sub(ptr) >= 2 {
            match little2_byte_type(byte_types, input, ptr) {
                29 => {
                    if !little2_in_name_bitmap(input, ptr, &namePages) {
                        return Little2ScanResult {
                            token: invalid,
                            next: Some(ptr),
                        };
                    }
                    ptr += 2;
                    continue 'attribute;
                }
                22 | 24 | 25 | 26 | 27 => {
                    ptr += 2;
                    continue 'attribute;
                }
                5 => {
                    return Little2ScanResult {
                        token: if input.len().saturating_sub(ptr) < 2 {
                            partial_char
                        } else {
                            invalid
                        },
                        next: (input.len().saturating_sub(ptr) >= 2).then_some(ptr),
                    };
                }
                6 => {
                    return Little2ScanResult {
                        token: if input.len().saturating_sub(ptr) < 3 {
                            partial_char
                        } else {
                            invalid
                        },
                        next: (input.len().saturating_sub(ptr) >= 3).then_some(ptr),
                    };
                }
                7 => {
                    return Little2ScanResult {
                        token: if input.len().saturating_sub(ptr) < 4 {
                            partial_char
                        } else {
                            invalid
                        },
                        next: (input.len().saturating_sub(ptr) >= 4).then_some(ptr),
                    };
                }
                23 => {
                    if had_colon {
                        return Little2ScanResult {
                            token: invalid,
                            next: Some(ptr),
                        };
                    }
                    had_colon = true;
                    ptr += 2;
                    if input.len().saturating_sub(ptr) < 2 {
                        return Little2ScanResult {
                            token: partial,
                            next: None,
                        };
                    }
                    match little2_byte_type(byte_types, input, ptr) {
                        29 => {
                            if !little2_in_name_bitmap(input, ptr, &nmstrtPages) {
                                return Little2ScanResult {
                                    token: invalid,
                                    next: Some(ptr),
                                };
                            }
                            ptr += 2;
                        }
                        22 | 24 => ptr += 2,
                        5 => {
                            return Little2ScanResult {
                                token: if input.len().saturating_sub(ptr) < 2 {
                                    partial_char
                                } else {
                                    invalid
                                },
                                next: (input.len().saturating_sub(ptr) >= 2).then_some(ptr),
                            };
                        }
                        6 => {
                            return Little2ScanResult {
                                token: if input.len().saturating_sub(ptr) < 3 {
                                    partial_char
                                } else {
                                    invalid
                                },
                                next: (input.len().saturating_sub(ptr) >= 3).then_some(ptr),
                            };
                        }
                        7 => {
                            return Little2ScanResult {
                                token: if input.len().saturating_sub(ptr) < 4 {
                                    partial_char
                                } else {
                                    invalid
                                },
                                next: (input.len().saturating_sub(ptr) >= 4).then_some(ptr),
                            };
                        }
                        _ => {
                            return Little2ScanResult {
                                token: invalid,
                                next: Some(ptr),
                            };
                        }
                    }
                    continue 'attribute;
                }
                21 | 9 | 10 => loop {
                    ptr += 2;
                    if input.len().saturating_sub(ptr) < 2 {
                        return Little2ScanResult {
                            token: partial,
                            next: None,
                        };
                    }
                    let byte_type = little2_byte_type(byte_types, input, ptr);
                    if byte_type == crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int {
                        break;
                    }
                    if !matches!(byte_type, 21 | 10 | 9) {
                        return Little2ScanResult {
                            token: invalid,
                            next: Some(ptr),
                        };
                    }
                },
                14 => {}
                _ => {
                    return Little2ScanResult {
                        token: invalid,
                        next: Some(ptr),
                    };
                }
            }

            had_colon = false;
            let open = loop {
                ptr += 2;
                if input.len().saturating_sub(ptr) < 2 {
                    return Little2ScanResult {
                        token: partial,
                        next: None,
                    };
                }
                let byte_type = little2_byte_type(byte_types, input, ptr);
                if matches!(
                    byte_type,
                    x if x == crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int
                        || x == crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int
                ) {
                    break byte_type;
                }
                if !matches!(byte_type, 21 | 10 | 9) {
                    return Little2ScanResult {
                        token: invalid,
                        next: Some(ptr),
                    };
                }
            };

            ptr += 2;
            loop {
                if input.len().saturating_sub(ptr) < 2 {
                    return Little2ScanResult {
                        token: partial,
                        next: None,
                    };
                }
                let byte_type = little2_byte_type(byte_types, input, ptr);
                if byte_type == open {
                    break;
                }
                match byte_type {
                    5 => {
                        if input.len().saturating_sub(ptr) < 2 {
                            return Little2ScanResult {
                                token: partial_char,
                                next: None,
                            };
                        }
                        ptr += 2;
                    }
                    6 => {
                        if input.len().saturating_sub(ptr) < 3 {
                            return Little2ScanResult {
                                token: partial_char,
                                next: None,
                            };
                        }
                        ptr += 3;
                    }
                    7 => {
                        if input.len().saturating_sub(ptr) < 4 {
                            return Little2ScanResult {
                                token: partial_char,
                                next: None,
                            };
                        }
                        ptr += 4;
                    }
                    0 | 1 | 8 => {
                        return Little2ScanResult {
                            token: invalid,
                            next: Some(ptr),
                        };
                    }
                    3 => {
                        let (token, next) = scan_ref(ptr + 2);
                        ptr = next;
                        if token <= 0 {
                            return Little2ScanResult {
                                token,
                                next: (token == invalid).then_some(ptr),
                            };
                        }
                    }
                    2 => {
                        return Little2ScanResult {
                            token: invalid,
                            next: Some(ptr),
                        };
                    }
                    _ => ptr += 2,
                }
            }

            ptr += 2;
            if input.len().saturating_sub(ptr) < 2 {
                return Little2ScanResult {
                    token: partial,
                    next: None,
                };
            }
            match little2_byte_type(byte_types, input, ptr) {
                21 | 9 | 10 => loop {
                    ptr += 2;
                    if input.len().saturating_sub(ptr) < 2 {
                        return Little2ScanResult {
                            token: partial,
                            next: None,
                        };
                    }
                    match little2_byte_type(byte_types, input, ptr) {
                        29 => {
                            if !little2_in_name_bitmap(input, ptr, &nmstrtPages) {
                                return Little2ScanResult {
                                    token: invalid,
                                    next: Some(ptr),
                                };
                            }
                            ptr += 2;
                            continue 'attribute;
                        }
                        22 | 24 => {
                            ptr += 2;
                            continue 'attribute;
                        }
                        5 => {
                            return Little2ScanResult {
                                token: if input.len().saturating_sub(ptr) < 2 {
                                    partial_char
                                } else {
                                    invalid
                                },
                                next: (input.len().saturating_sub(ptr) >= 2).then_some(ptr),
                            };
                        }
                        6 => {
                            return Little2ScanResult {
                                token: if input.len().saturating_sub(ptr) < 3 {
                                    partial_char
                                } else {
                                    invalid
                                },
                                next: (input.len().saturating_sub(ptr) >= 3).then_some(ptr),
                            };
                        }
                        7 => {
                            return Little2ScanResult {
                                token: if input.len().saturating_sub(ptr) < 4 {
                                    partial_char
                                } else {
                                    invalid
                                },
                                next: (input.len().saturating_sub(ptr) >= 4).then_some(ptr),
                            };
                        }
                        21 | 9 | 10 => continue,
                        11 => {
                            return Little2ScanResult {
                                token: crate::src::xmltok::XML_TOK_START_TAG_WITH_ATTS_1,
                                next: Some(ptr + 2),
                            };
                        }
                        17 => {
                            ptr += 2;
                            if input.len().saturating_sub(ptr) < 2 {
                                return Little2ScanResult {
                                    token: partial,
                                    next: None,
                                };
                            }
                            if input[ptr + 1] as ::core::ffi::c_uchar != 0
                                || input[ptr] as ::core::ffi::c_uchar != b'>'
                            {
                                return Little2ScanResult {
                                    token: invalid,
                                    next: Some(ptr),
                                };
                            }
                            return Little2ScanResult {
                                token: crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1,
                                next: Some(ptr + 2),
                            };
                        }
                        _ => {
                            return Little2ScanResult {
                                token: invalid,
                                next: Some(ptr),
                            };
                        }
                    }
                },
                17 => {
                    ptr += 2;
                    if input.len().saturating_sub(ptr) < 2 {
                        return Little2ScanResult {
                            token: partial,
                            next: None,
                        };
                    }
                    if input[ptr + 1] as ::core::ffi::c_uchar != 0
                        || input[ptr] as ::core::ffi::c_uchar != b'>'
                    {
                        return Little2ScanResult {
                            token: invalid,
                            next: Some(ptr),
                        };
                    }
                    return Little2ScanResult {
                        token: crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1,
                        next: Some(ptr + 2),
                    };
                }
                11 => {
                    return Little2ScanResult {
                        token: crate::src::xmltok::XML_TOK_START_TAG_WITH_ATTS_1,
                        next: Some(ptr + 2),
                    };
                }
                _ => {
                    return Little2ScanResult {
                        token: invalid,
                        next: Some(ptr),
                    };
                }
            }
        }

        Little2ScanResult {
            token: partial,
            next: None,
        }
    }

    pub unsafe extern "C" fn little2_scanAtts(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = end.offset_from(ptr);
        if input_len < 0 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = ::core::slice::from_raw_parts(ptr, input_len as usize);
        let normal = &*(enc as *const normal_encoding);
        let result = little2_scan_atts_impl(&normal.type_0, input, |ref_start| {
            let mut ref_end = ptr;
            let token = little2_scanRef(enc, ptr.add(ref_start), end, &mut ref_end);
            let next = ref_end.offset_from(ptr);
            if next < 0 || next as usize > input.len() {
                (crate::src::xmltok::XML_TOK_INVALID_1, ref_start)
            } else {
                (token, next as usize)
            }
        });
        if let Some(next) = result.next {
            *nextTokPtr = ptr.add(next);
        }
        result.token
    }
    enum Little2ScanLtAction {
        Token(::core::ffi::c_int, Option<usize>),
        Comment(usize),
        CdataSection(usize),
        ProcessingInstruction(usize),
        EndTag(usize),
        Attributes(usize),
    }

    fn little2_scan_lt_type(
        enc: &normal_encoding,
        input: &[u8],
        offset: usize,
    ) -> ::core::ffi::c_int {
        let lo = input[offset];
        let hi = input[offset + 1];
        if hi == 0 {
            enc.type_0[lo as usize] as ::core::ffi::c_int
        } else {
            unicode_byte_type(hi as ::core::ffi::c_char, lo as ::core::ffi::c_char)
        }
    }

    fn little2_scan_lt_name(
        enc: &normal_encoding,
        input: &[u8],
        offset: usize,
        name_start: bool,
    ) -> Result<usize, Little2ScanLtAction> {
        let remaining = input.len() - offset;
        match little2_scan_lt_type(enc, input, offset) {
            29 => {
                let pages = if name_start { &nmstrtPages } else { &namePages };
                let lo = input[offset];
                let hi = input[offset + 1];
                let word = (pages[hi as usize] as usize) * 8 + (lo as usize >> 5);
                if namingBitmap[word] & (1 << (lo & 0x1f)) != 0 {
                    Ok(offset + 2)
                } else {
                    Err(Little2ScanLtAction::Token(
                        crate::src::xmltok::XML_TOK_INVALID_1,
                        Some(offset),
                    ))
                }
            }
            22 | 24 if name_start => Ok(offset + 2),
            22 | 24 | 25 | 26 | 27 if !name_start => Ok(offset + 2),
            5 => Err(Little2ScanLtAction::Token(
                if remaining < 2 {
                    crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1
                } else {
                    crate::src::xmltok::XML_TOK_INVALID_1
                },
                (remaining >= 2).then_some(offset),
            )),
            6 => Err(Little2ScanLtAction::Token(
                if remaining < 3 {
                    crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1
                } else {
                    crate::src::xmltok::XML_TOK_INVALID_1
                },
                (remaining >= 3).then_some(offset),
            )),
            7 => Err(Little2ScanLtAction::Token(
                if remaining < 4 {
                    crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1
                } else {
                    crate::src::xmltok::XML_TOK_INVALID_1
                },
                (remaining >= 4).then_some(offset),
            )),
            _ => Err(Little2ScanLtAction::Token(
                crate::src::xmltok::XML_TOK_INVALID_1,
                Some(offset),
            )),
        }
    }

    fn little2_scan_lt_impl(enc: &normal_encoding, input: &[u8]) -> Little2ScanLtAction {
        if input.len() < 2 {
            return Little2ScanLtAction::Token(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }

        let mut ptr = match little2_scan_lt_name(enc, input, 0, true) {
            Ok(next) => next,
            Err(Little2ScanLtAction::Token(crate::src::xmltok::XML_TOK_INVALID_1, Some(0))) => {
                match little2_scan_lt_type(enc, input, 0) {
                    16 => {
                        if input.len() < 4 {
                            return Little2ScanLtAction::Token(
                                crate::src::xmltok::XML_TOK_PARTIAL_1,
                                None,
                            );
                        }
                        return match little2_scan_lt_type(enc, input, 2) {
                            27 => Little2ScanLtAction::Comment(4),
                            20 => Little2ScanLtAction::CdataSection(4),
                            _ => Little2ScanLtAction::Token(
                                crate::src::xmltok::XML_TOK_INVALID_1,
                                Some(2),
                            ),
                        };
                    }
                    15 => return Little2ScanLtAction::ProcessingInstruction(2),
                    17 => return Little2ScanLtAction::EndTag(2),
                    _ => {
                        return Little2ScanLtAction::Token(
                            crate::src::xmltok::XML_TOK_INVALID_1,
                            Some(0),
                        )
                    }
                }
            }
            Err(action) => return action,
        };
        let mut had_colon = false;

        while input.len() - ptr >= 2 {
            match little2_scan_lt_name(enc, input, ptr, false) {
                Ok(next) => {
                    ptr = next;
                    continue;
                }
                Err(Little2ScanLtAction::Token(crate::src::xmltok::XML_TOK_INVALID_1, Some(_))) => {
                }
                Err(action) => return action,
            }

            match little2_scan_lt_type(enc, input, ptr) {
                23 => {
                    if had_colon {
                        return Little2ScanLtAction::Token(
                            crate::src::xmltok::XML_TOK_INVALID_1,
                            Some(ptr),
                        );
                    }
                    had_colon = true;
                    ptr += 2;
                    if input.len() - ptr < 2 {
                        return Little2ScanLtAction::Token(
                            crate::src::xmltok::XML_TOK_PARTIAL_1,
                            None,
                        );
                    }
                    ptr = match little2_scan_lt_name(enc, input, ptr, true) {
                        Ok(next) => next,
                        Err(action) => return action,
                    };
                }
                21 | 9 | 10 => {
                    ptr += 2;
                    while input.len() - ptr >= 2
                        && matches!(little2_scan_lt_type(enc, input, ptr), 21 | 9 | 10)
                    {
                        ptr += 2;
                    }
                    if input.len() - ptr < 2 {
                        return Little2ScanLtAction::Token(
                            crate::src::xmltok::XML_TOK_PARTIAL_1,
                            None,
                        );
                    }
                    return Little2ScanLtAction::Attributes(ptr);
                }
                11 => {
                    return Little2ScanLtAction::Token(
                        crate::src::xmltok::XML_TOK_START_TAG_NO_ATTS_1,
                        Some(ptr + 2),
                    )
                }
                17 => {
                    ptr += 2;
                    if input.len() - ptr < 2 {
                        return Little2ScanLtAction::Token(
                            crate::src::xmltok::XML_TOK_PARTIAL_1,
                            None,
                        );
                    }
                    return if input[ptr] == b'>' && input[ptr + 1] == 0 {
                        Little2ScanLtAction::Token(
                            crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_NO_ATTS_1,
                            Some(ptr + 2),
                        )
                    } else {
                        Little2ScanLtAction::Token(crate::src::xmltok::XML_TOK_INVALID_1, Some(ptr))
                    };
                }
                _ => {
                    return Little2ScanLtAction::Token(
                        crate::src::xmltok::XML_TOK_INVALID_1,
                        Some(ptr),
                    )
                }
            }
        }
        Little2ScanLtAction::Token(crate::src::xmltok::XML_TOK_PARTIAL_1, None)
    }

    pub unsafe extern "C" fn little2_scanLt(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = end.offset_from(ptr);
        if input_len < 2 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = ::core::slice::from_raw_parts(ptr.cast::<u8>(), input_len as usize);
        let normal = &*(enc as *const normal_encoding);
        match little2_scan_lt_impl(normal, input) {
            Little2ScanLtAction::Token(token, next) => {
                if let Some(next) = next {
                    *nextTokPtr = ptr.add(next);
                }
                token
            }
            Little2ScanLtAction::Comment(start) => {
                little2_scanComment(enc, ptr.add(start), end, nextTokPtr)
            }
            Little2ScanLtAction::CdataSection(start) => {
                little2_scanCdataSection(enc, ptr.add(start), end, nextTokPtr)
            }
            Little2ScanLtAction::ProcessingInstruction(start) => {
                little2_scanPi(enc, ptr.add(start), end, nextTokPtr)
            }
            Little2ScanLtAction::EndTag(start) => {
                little2_scanEndTag(enc, ptr.add(start), end, nextTokPtr)
            }
            Little2ScanLtAction::Attributes(start) => {
                little2_scanAtts(enc, ptr.add(start), end, nextTokPtr)
            }
        }
    }

    enum Little2ContentToken {
        Result(::core::ffi::c_int, Option<usize>),
        ScanLt,
        ScanRef,
    }

    fn little2_char_type(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
        at: usize,
    ) -> ::core::ffi::c_int {
        if input[at + 1] == 0 {
            enc.type_0[input[at] as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
        } else {
            unicode_byte_type(input[at + 1], input[at])
        }
    }

    fn little2_content_tok_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> Little2ContentToken {
        if input.is_empty() {
            return Little2ContentToken::Result(crate::src::xmltok::XML_TOK_NONE_1, None);
        }
        let end = input.len() & !1;
        if end == 0 {
            return Little2ContentToken::Result(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }
        let mut ptr = 0;
        match little2_char_type(enc, input, ptr) {
            2 => return Little2ContentToken::ScanLt,
            3 => return Little2ContentToken::ScanRef,
            9 => {
                ptr += 2;
                if end - ptr < 2 {
                    return Little2ContentToken::Result(
                        crate::src::xmltok::XML_TOK_TRAILING_CR_1,
                        None,
                    );
                }
                if little2_char_type(enc, input, ptr)
                    == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                {
                    ptr += 2;
                }
                return Little2ContentToken::Result(
                    crate::src::xmltok::XML_TOK_DATA_NEWLINE_1,
                    Some(ptr),
                );
            }
            10 => {
                return Little2ContentToken::Result(
                    crate::src::xmltok::XML_TOK_DATA_NEWLINE_1,
                    Some(2),
                )
            }
            4 => {
                ptr += 2;
                if end - ptr < 2 {
                    return Little2ContentToken::Result(
                        crate::src::xmltok::XML_TOK_TRAILING_RSQB_1,
                        None,
                    );
                }
                if input[ptr + 1] == 0 && input[ptr] == 0x5d {
                    ptr += 2;
                    if end - ptr < 2 {
                        return Little2ContentToken::Result(
                            crate::src::xmltok::XML_TOK_TRAILING_RSQB_1,
                            None,
                        );
                    }
                    if input[ptr + 1] == 0 && input[ptr] == 0x3e {
                        return Little2ContentToken::Result(
                            crate::src::xmltok::XML_TOK_INVALID_1,
                            Some(ptr),
                        );
                    }
                    ptr -= 2;
                }
            }
            5 => ptr += 2,
            6 => {
                if end < 3 {
                    return Little2ContentToken::Result(
                        crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        None,
                    );
                }
                ptr += 3;
            }
            7 => {
                if end < 4 {
                    return Little2ContentToken::Result(
                        crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        None,
                    );
                }
                ptr += 4;
            }
            0 | 1 | 8 => {
                return Little2ContentToken::Result(crate::src::xmltok::XML_TOK_INVALID_1, Some(0))
            }
            _ => ptr += 2,
        }
        while end - ptr >= 2 {
            match little2_char_type(enc, input, ptr) {
                5 => {
                    ptr += 2;
                    break;
                }
                6 => {
                    if end - ptr < 3 {
                        return Little2ContentToken::Result(
                            crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                            Some(ptr),
                        );
                    }
                    ptr += 3;
                    break;
                }
                7 => {
                    if end - ptr < 4 {
                        return Little2ContentToken::Result(
                            crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                            Some(ptr),
                        );
                    }
                    ptr += 4;
                    break;
                }
                4 if end - ptr >= 4 => {
                    if input[ptr + 3] != 0 || input[ptr + 2] != 0x5d {
                        ptr += 2;
                        break;
                    }
                    if end - ptr >= 6 {
                        if input[ptr + 5] != 0 || input[ptr + 4] != 0x3e {
                            ptr += 2;
                            break;
                        }
                        return Little2ContentToken::Result(
                            crate::src::xmltok::XML_TOK_INVALID_1,
                            Some(ptr + 4),
                        );
                    }
                }
                3 | 2 | 0 | 1 | 8 | 9 | 10 | 4 => {}
                _ => {
                    ptr += 2;
                    break;
                }
            }
            return Little2ContentToken::Result(
                crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                Some(ptr),
            );
        }
        Little2ContentToken::Result(crate::src::xmltok::XML_TOK_DATA_CHARS_1, Some(ptr))
    }

    pub unsafe extern "C" fn little2_contentTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if ptr >= end {
            return crate::src::xmltok::XML_TOK_NONE_1;
        }
        let byte_len = end.offset_from(ptr) as usize;
        let input = ::core::slice::from_raw_parts(ptr, byte_len);
        let end = ptr.add(byte_len & !1);
        let encoding = &*(enc as *const normal_encoding);
        match little2_content_tok_impl(encoding, input) {
            Little2ContentToken::ScanLt => little2_scanLt(enc, ptr.add(2), end, nextTokPtr),
            Little2ContentToken::ScanRef => little2_scanRef(enc, ptr.add(2), end, nextTokPtr),
            Little2ContentToken::Result(token, Some(next)) => {
                *nextTokPtr = ptr.add(next);
                token
            }
            Little2ContentToken::Result(token, None) => token,
        }
    }

    fn little2_scan_percent_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> (::core::ffi::c_int, Option<usize>) {
        if input.len() < 2 {
            return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }

        let mut offset = match little2_byte_type(&enc.type_0, input, 0) {
            29 if !little2_in_name_bitmap(input, 0, &nmstrtPages) => {
                return (crate::src::xmltok::XML_TOK_INVALID_1, Some(0))
            }
            29 | 22 | 24 => 2,
            kind @ (5 | 6 | 7) => {
                let width = kind as usize - 3;
                if input.len() < width {
                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                }
                return (crate::src::xmltok::XML_TOK_INVALID_1, Some(0));
            }
            21 | 10 | 9 | 30 => return (crate::src::xmltok::XML_TOK_PERCENT_1, Some(0)),
            _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(0)),
        };

        while input.len() - offset >= 2 {
            match little2_byte_type(&enc.type_0, input, offset) {
                29 if !little2_in_name_bitmap(input, offset, &namePages) => {
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset))
                }
                29 | 22 | 24 | 25 | 26 | 27 => offset += 2,
                kind @ (5 | 6 | 7) => {
                    let width = kind as usize - 3;
                    if input.len() - offset < width {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                    }
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                }
                18 => {
                    return (
                        crate::src::xmltok::XML_TOK_PARAM_ENTITY_REF_1,
                        Some(offset + 2),
                    )
                }
                _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
            }
        }

        (crate::src::xmltok::XML_TOK_PARTIAL_1, None)
    }

    pub unsafe extern "C" fn little2_scanPercent(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = end.offset_from(ptr);
        if input_len < 2 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = ::core::slice::from_raw_parts(ptr, input_len as usize);
        let normal = &*(enc as *const normal_encoding);
        let (token, next) = little2_scan_percent_impl(normal, input);
        if let Some(offset) = next {
            *nextTokPtr = ptr.add(offset);
        }
        token
    }

    fn little2_scan_pound_name_impl(
        enc: &normal_encoding,
        input: &[u8],
    ) -> (::core::ffi::c_int, Option<usize>) {
        if input.len() < 2 {
            return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }

        let mut offset = match little2_type(enc, input, 0) {
            29 if !little2_bitmap_contains(&nmstrtPages, input[0], input[1]) => {
                return (crate::src::xmltok::XML_TOK_INVALID_1, Some(0))
            }
            29 | 22 | 24 => 2,
            kind @ (5 | 6 | 7) => {
                let width = kind as usize - 3;
                if input.len() < width {
                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                }
                return (crate::src::xmltok::XML_TOK_INVALID_1, Some(0));
            }
            _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(0)),
        };

        while input.len() - offset >= 2 {
            match little2_type(enc, input, offset) {
                29 if !little2_bitmap_contains(&namePages, input[offset], input[offset + 1]) => {
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset))
                }
                29 | 22 | 24 | 25 | 26 | 27 => offset += 2,
                kind @ (5 | 6 | 7) => {
                    let width = kind as usize - 3;
                    if input.len() - offset < width {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                    }
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                }
                9 | 10 | 11 | 21 | 30 | 32 | 36 => {
                    return (crate::src::xmltok::XML_TOK_POUND_NAME_1, Some(offset))
                }
                _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
            }
        }

        (-crate::src::xmltok::XML_TOK_POUND_NAME_1, None)
    }

    pub unsafe extern "C" fn little2_scanPoundName(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = end.offset_from(ptr);
        if input_len < 2 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = ::core::slice::from_raw_parts(ptr.cast::<u8>(), input_len as usize);
        let normal = &*(enc as *const normal_encoding);
        let (token, next) = little2_scan_pound_name_impl(normal, input);
        if let Some(offset) = next {
            *nextTokPtr = ptr.add(offset);
        }
        token
    }

    pub unsafe extern "C" fn little2_scanLit(
        mut open: ::core::ffi::c_int,
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        while end.offset_from(ptr) >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize {
            let mut t: ::core::ffi::c_int =
                if *ptr.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(1 as isize), *ptr.offset(0 as isize))
                };
            match t {
                5 => {
                    if end.offset_from(ptr) < 2 as isize {
                        return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                6 => {
                    if end.offset_from(ptr) < 3 as isize {
                        return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                }
                7 => {
                    if end.offset_from(ptr) < 4 as isize {
                        return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
                12 | 13 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    if t == open {
                        if !(end.offset_from(ptr)
                            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                        {
                            return -crate::src::xmltok::XML_TOK_LITERAL_1;
                        }
                        *nextTokPtr = ptr;
                        match if *ptr.offset(1 as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(1 as isize), *ptr.offset(0 as isize))
                        } {
                            21 | 9 | 10 | 11 | 30 | 20 => {
                                return crate::src::xmltok::XML_TOK_LITERAL_1
                            }
                            _ => return crate::src::xmltok::XML_TOK_INVALID_1,
                        }
                    }
                }
                _ => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
            }
        }
        return crate::src::xmltok::XML_TOK_PARTIAL_1;
    }

    enum Little2PrologAction {
        Return {
            token: ::core::ffi::c_int,
            next: Option<usize>,
        },
        ScanLit {
            open: ::core::ffi::c_int,
            start: usize,
        },
        ScanDecl {
            start: usize,
        },
        ScanPi {
            start: usize,
        },
        ScanPercent {
            start: usize,
        },
        ScanPoundName {
            start: usize,
        },
    }

    fn little2_type(enc: &normal_encoding, input: &[u8], offset: usize) -> ::core::ffi::c_int {
        let lo = input[offset];
        let hi = input[offset + 1];
        if hi == 0 {
            enc.type_0[lo as usize] as ::core::ffi::c_int
        } else {
            unicode_byte_type(hi as ::core::ffi::c_char, lo as ::core::ffi::c_char)
        }
    }

    fn little2_bitmap_contains(pages: &[::core::ffi::c_uchar; 256], lo: u8, hi: u8) -> bool {
        let bit = ((pages[hi as usize] as usize) << 3) + (lo as usize >> 5);
        namingBitmap[bit] & (1 << (lo & 0x1f)) != 0
    }

    fn little2_prolog_tok_impl(enc: &normal_encoding, input: &[u8]) -> Little2PrologAction {
        let len = input.len();
        let result = |token, next| Little2PrologAction::Return { token, next };
        let mut ptr = 0usize;
        let mut tok: ::core::ffi::c_int;
        let first = little2_type(enc, input, ptr);
        match first {
            12 => {
                return Little2PrologAction::ScanLit {
                    open: crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int,
                    start: 2,
                }
            }
            13 => {
                return Little2PrologAction::ScanLit {
                    open: crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int,
                    start: 2,
                }
            }
            2 => {
                ptr = 2;
                if len - ptr < 2 {
                    return result(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                }
                return match little2_type(enc, input, ptr) {
                    16 => Little2PrologAction::ScanDecl { start: ptr + 2 },
                    15 => Little2PrologAction::ScanPi { start: ptr + 2 },
                    22 | 24 | 29 | 5 | 6 | 7 => {
                        result(crate::src::xmltok::XML_TOK_INSTANCE_START, Some(0))
                    }
                    _ => result(crate::src::xmltok::XML_TOK_INVALID_1, Some(ptr)),
                };
            }
            9 | 21 | 10 => {
                if first == 9 && ptr + 2 == len {
                    return result(-crate::src::xmltok::XML_TOK_PROLOG_S_1, Some(len));
                }
                loop {
                    ptr += 2;
                    if len - ptr < 2 {
                        break;
                    }
                    match little2_type(enc, input, ptr) {
                        21 | 10 => continue,
                        9 if ptr + 2 != len => continue,
                        _ => return result(crate::src::xmltok::XML_TOK_PROLOG_S_1, Some(ptr)),
                    }
                }
                return result(crate::src::xmltok::XML_TOK_PROLOG_S_1, Some(ptr));
            }
            30 => return Little2PrologAction::ScanPercent { start: 2 },
            35 => return result(crate::src::xmltok::XML_TOK_COMMA_1, Some(2)),
            20 => return result(crate::src::xmltok::XML_TOK_OPEN_BRACKET_1, Some(2)),
            4 => {
                ptr = 2;
                if len - ptr < 2 {
                    return result(-crate::src::xmltok::XML_TOK_CLOSE_BRACKET_1, None);
                }
                if input[ptr + 1] == 0 && input[ptr] == b']' {
                    if len - ptr < 4 {
                        return result(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                    }
                    if input[ptr + 3] == 0 && input[ptr + 2] == b'>' {
                        return result(
                            crate::src::xmltok::XML_TOK_COND_SECT_CLOSE_1,
                            Some(ptr + 4),
                        );
                    }
                }
                return result(crate::src::xmltok::XML_TOK_CLOSE_BRACKET_1, Some(ptr));
            }
            31 => return result(crate::src::xmltok::XML_TOK_OPEN_PAREN_1, Some(2)),
            32 => {
                ptr = 2;
                if len - ptr < 2 {
                    return result(-crate::src::xmltok::XML_TOK_CLOSE_PAREN_1, None);
                }
                return match little2_type(enc, input, ptr) {
                    33 => result(
                        crate::src::xmltok::XML_TOK_CLOSE_PAREN_ASTERISK_1,
                        Some(ptr + 2),
                    ),
                    15 => result(
                        crate::src::xmltok::XML_TOK_CLOSE_PAREN_QUESTION_1,
                        Some(ptr + 2),
                    ),
                    34 => result(
                        crate::src::xmltok::XML_TOK_CLOSE_PAREN_PLUS_1,
                        Some(ptr + 2),
                    ),
                    9 | 10 | 21 | 11 | 35 | 36 | 32 => {
                        result(crate::src::xmltok::XML_TOK_CLOSE_PAREN_1, Some(ptr))
                    }
                    _ => result(crate::src::xmltok::XML_TOK_INVALID_1, Some(ptr)),
                };
            }
            36 => return result(crate::src::xmltok::XML_TOK_OR_1, Some(2)),
            11 => return result(crate::src::xmltok::XML_TOK_DECL_CLOSE_1, Some(2)),
            19 => return Little2PrologAction::ScanPoundName { start: 2 },
            5 => {
                return result(
                    if len < 2 {
                        crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1
                    } else {
                        crate::src::xmltok::XML_TOK_INVALID_1
                    },
                    Some(0),
                )
            }
            6 => {
                return result(
                    if len < 3 {
                        crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1
                    } else {
                        crate::src::xmltok::XML_TOK_INVALID_1
                    },
                    Some(0),
                )
            }
            7 => {
                return result(
                    if len < 4 {
                        crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1
                    } else {
                        crate::src::xmltok::XML_TOK_INVALID_1
                    },
                    Some(0),
                )
            }
            22 | 24 => {
                tok = crate::src::xmltok::XML_TOK_NAME;
                ptr = 2;
            }
            25 | 26 | 27 | 23 => {
                tok = crate::src::xmltok::XML_TOK_NMTOKEN_1;
                ptr = 2;
            }
            29 => {
                if little2_bitmap_contains(&nmstrtPages, input[0], input[1]) {
                    tok = crate::src::xmltok::XML_TOK_NAME;
                    ptr = 2;
                } else if little2_bitmap_contains(&namePages, input[0], input[1]) {
                    tok = crate::src::xmltok::XML_TOK_NMTOKEN_1;
                    ptr = 2;
                } else {
                    return result(crate::src::xmltok::XML_TOK_INVALID_1, Some(0));
                }
            }
            _ => return result(crate::src::xmltok::XML_TOK_INVALID_1, Some(0)),
        }

        while len - ptr >= 2 {
            match little2_type(enc, input, ptr) {
                29 if !little2_bitmap_contains(&namePages, input[ptr], input[ptr + 1]) => {
                    return result(crate::src::xmltok::XML_TOK_INVALID_1, Some(ptr));
                }
                29 | 22 | 24 | 25 | 26 | 27 => {}
                5 => {
                    return result(
                        if len - ptr < 2 {
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1
                        } else {
                            crate::src::xmltok::XML_TOK_INVALID_1
                        },
                        Some(ptr),
                    )
                }
                6 => {
                    return result(
                        if len - ptr < 3 {
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1
                        } else {
                            crate::src::xmltok::XML_TOK_INVALID_1
                        },
                        Some(ptr),
                    )
                }
                7 => {
                    return result(
                        if len - ptr < 4 {
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1
                        } else {
                            crate::src::xmltok::XML_TOK_INVALID_1
                        },
                        Some(ptr),
                    )
                }
                11 | 32 | 35 | 36 | 20 | 30 | 21 | 9 | 10 => return result(tok, Some(ptr)),
                23 => {
                    ptr += 2;
                    match tok {
                        crate::src::xmltok::XML_TOK_NAME => {
                            if len - ptr < 2 {
                                return result(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                            }
                            tok = crate::src::xmltok::XML_TOK_PREFIXED_NAME;
                            match little2_type(enc, input, ptr) {
                                29 if !little2_bitmap_contains(
                                    &namePages,
                                    input[ptr],
                                    input[ptr + 1],
                                ) =>
                                {
                                    return result(crate::src::xmltok::XML_TOK_INVALID_1, Some(ptr))
                                }
                                29 | 22 | 24 | 25 | 26 | 27 => ptr += 2,
                                5 => {
                                    return result(
                                        if len - ptr < 2 {
                                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1
                                        } else {
                                            crate::src::xmltok::XML_TOK_INVALID_1
                                        },
                                        Some(ptr),
                                    )
                                }
                                6 => {
                                    return result(
                                        if len - ptr < 3 {
                                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1
                                        } else {
                                            crate::src::xmltok::XML_TOK_INVALID_1
                                        },
                                        Some(ptr),
                                    )
                                }
                                7 => {
                                    return result(
                                        if len - ptr < 4 {
                                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1
                                        } else {
                                            crate::src::xmltok::XML_TOK_INVALID_1
                                        },
                                        Some(ptr),
                                    )
                                }
                                _ => tok = crate::src::xmltok::XML_TOK_NMTOKEN_1,
                            }
                        }
                        crate::src::xmltok::XML_TOK_PREFIXED_NAME => {
                            tok = crate::src::xmltok::XML_TOK_NMTOKEN_1
                        }
                        _ => {}
                    }
                }
                34 => {
                    return result(
                        if tok == crate::src::xmltok::XML_TOK_NMTOKEN_1 {
                            crate::src::xmltok::XML_TOK_INVALID_1
                        } else {
                            crate::src::xmltok::XML_TOK_NAME_PLUS_1
                        },
                        Some(if tok == crate::src::xmltok::XML_TOK_NMTOKEN_1 {
                            ptr
                        } else {
                            ptr + 2
                        }),
                    )
                }
                33 => {
                    return result(
                        if tok == crate::src::xmltok::XML_TOK_NMTOKEN_1 {
                            crate::src::xmltok::XML_TOK_INVALID_1
                        } else {
                            crate::src::xmltok::XML_TOK_NAME_ASTERISK_1
                        },
                        Some(if tok == crate::src::xmltok::XML_TOK_NMTOKEN_1 {
                            ptr
                        } else {
                            ptr + 2
                        }),
                    )
                }
                15 => {
                    return result(
                        if tok == crate::src::xmltok::XML_TOK_NMTOKEN_1 {
                            crate::src::xmltok::XML_TOK_INVALID_1
                        } else {
                            crate::src::xmltok::XML_TOK_NAME_QUESTION_1
                        },
                        Some(if tok == crate::src::xmltok::XML_TOK_NMTOKEN_1 {
                            ptr
                        } else {
                            ptr + 2
                        }),
                    )
                }
                _ => return result(crate::src::xmltok::XML_TOK_INVALID_1, Some(ptr)),
            }
            ptr += 2;
        }
        result(-tok, None)
    }

    pub unsafe extern "C" fn little2_prologTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let span = unsafe { end.offset_from(ptr) };
        if span <= 0 {
            return crate::src::xmltok::XML_TOK_NONE_1;
        }
        let len = (span as usize) & !1;
        if len == 0 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = unsafe { ::core::slice::from_raw_parts(ptr.cast::<u8>(), len) };
        let normal = unsafe { &*(enc as *const normal_encoding) };
        let action = little2_prolog_tok_impl(normal, input);
        unsafe {
            match action {
                Little2PrologAction::Return { token, next } => {
                    if let Some(offset) = next {
                        *nextTokPtr = ptr.add(offset);
                    }
                    token
                }
                Little2PrologAction::ScanLit { open, start } => {
                    little2_scanLit(open, enc, ptr.add(start), ptr.add(len), nextTokPtr)
                }
                Little2PrologAction::ScanDecl { start } => {
                    little2_scanDecl(enc, ptr.add(start), ptr.add(len), nextTokPtr)
                }
                Little2PrologAction::ScanPi { start } => {
                    little2_scanPi(enc, ptr.add(start), ptr.add(len), nextTokPtr)
                }
                Little2PrologAction::ScanPercent { start } => {
                    little2_scanPercent(enc, ptr.add(start), ptr.add(len), nextTokPtr)
                }
                Little2PrologAction::ScanPoundName { start } => {
                    little2_scanPoundName(enc, ptr.add(start), ptr.add(len), nextTokPtr)
                }
            }
        }
    }

    struct Little2AttributeValueToken {
        token: ::core::ffi::c_int,
        next: Option<usize>,
    }

    fn little2_attribute_value_tok_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> Little2AttributeValueToken {
        if input.is_empty() {
            return Little2AttributeValueToken {
                token: crate::src::xmltok::XML_TOK_NONE_1,
                next: None,
            };
        }
        if input.len() < 2 {
            return Little2AttributeValueToken {
                token: crate::src::xmltok::XML_TOK_PARTIAL_1,
                next: None,
            };
        }

        let mut pos = 0;
        while pos + 2 <= input.len() {
            match little2_byte_type(&enc.type_0, input, pos) {
                5 => pos += 2,
                6 => {
                    if input.len() - pos < 3 {
                        return Little2AttributeValueToken {
                            token: crate::src::xmltok::XML_TOK_PARTIAL_1,
                            next: None,
                        };
                    }
                    pos += 3;
                }
                7 => {
                    if input.len() - pos < 4 {
                        return Little2AttributeValueToken {
                            token: crate::src::xmltok::XML_TOK_PARTIAL_1,
                            next: None,
                        };
                    }
                    pos += 4;
                }
                3 if pos == 0 => {
                    return match little2_scan_ref_impl(enc, &input[2..]) {
                        Little2ScanResult {
                            token,
                            next: Some(next),
                        } => Little2AttributeValueToken {
                            token,
                            next: Some(next + 2),
                        },
                        Little2ScanResult { token, next: None } => Little2AttributeValueToken {
                            token,
                            next: None,
                        },
                    };
                }
                3 => {
                    return Little2AttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(pos),
                    };
                }
                2 => {
                    return Little2AttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_INVALID_1,
                        next: Some(pos),
                    };
                }
                10 if pos == 0 => {
                    return Little2AttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_NEWLINE_1,
                        next: Some(2),
                    };
                }
                10 => {
                    return Little2AttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(pos),
                    };
                }
                9 if pos == 0 => {
                    pos += 2;
                    if pos + 2 > input.len() {
                        return Little2AttributeValueToken {
                            token: crate::src::xmltok::XML_TOK_TRAILING_CR_1,
                            next: None,
                        };
                    }
                    if little2_byte_type(&enc.type_0, input, pos)
                        == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                    {
                        pos += 2;
                    }
                    return Little2AttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_NEWLINE_1,
                        next: Some(pos),
                    };
                }
                9 => {
                    return Little2AttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(pos),
                    };
                }
                21 if pos == 0 => {
                    return Little2AttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_ATTRIBUTE_VALUE_S_1,
                        next: Some(2),
                    };
                }
                21 => {
                    return Little2AttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(pos),
                    };
                }
                _ => pos += 2,
            }
        }

        Little2AttributeValueToken {
            token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
            next: Some(pos),
        }
    }

    pub unsafe extern "C" fn little2_attributeValueTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = end.offset_from(ptr);
        if input_len <= 0 {
            return crate::src::xmltok::XML_TOK_NONE_1;
        }
        let input = ::core::slice::from_raw_parts(ptr, input_len as usize);
        let normal = &*(enc as *const normal_encoding);
        let result = little2_attribute_value_tok_impl(normal, input);
        if let Some(offset) = result.next {
            *nextTokPtr = ptr.add(offset);
        }
        result.token
    }

    struct Little2EntityValueToken {
        token: ::core::ffi::c_int,
        next: Option<usize>,
    }

    fn little2_entity_value_tok_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> Little2EntityValueToken {
        if input.is_empty() {
            return Little2EntityValueToken {
                token: crate::src::xmltok::XML_TOK_NONE_1,
                next: None,
            };
        }
        if input.len() < 2 {
            return Little2EntityValueToken {
                token: crate::src::xmltok::XML_TOK_PARTIAL_1,
                next: None,
            };
        }

        let mut pos = 0;
        while pos + 2 <= input.len() {
            match little2_byte_type(&enc.type_0, input, pos) {
                5 => pos += 2,
                6 => {
                    if input.len() - pos < 3 {
                        return Little2EntityValueToken {
                            token: crate::src::xmltok::XML_TOK_PARTIAL_1,
                            next: None,
                        };
                    }
                    pos += 3;
                }
                7 => {
                    if input.len() - pos < 4 {
                        return Little2EntityValueToken {
                            token: crate::src::xmltok::XML_TOK_PARTIAL_1,
                            next: None,
                        };
                    }
                    pos += 4;
                }
                3 if pos == 0 => {
                    return match little2_scan_ref_impl(enc, &input[2..]) {
                        Little2ScanResult {
                            token,
                            next: Some(next),
                        } => Little2EntityValueToken {
                            token,
                            next: Some(next + 2),
                        },
                        Little2ScanResult { token, next: None } => Little2EntityValueToken {
                            token,
                            next: None,
                        },
                    };
                }
                3 => {
                    return Little2EntityValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(pos),
                    };
                }
                30 if pos == 0 => {
                    return match little2_scan_percent_impl(enc, &input[2..]) {
                        (token, Some(next)) => Little2EntityValueToken {
                            token: if token == crate::src::xmltok::XML_TOK_PERCENT_1 {
                                crate::src::xmltok::XML_TOK_INVALID_1
                            } else {
                                token
                            },
                            next: Some(next + 2),
                        },
                        (token, None) => Little2EntityValueToken { token, next: None },
                    };
                }
                30 => {
                    return Little2EntityValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(pos),
                    };
                }
                10 if pos == 0 => {
                    return Little2EntityValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_NEWLINE_1,
                        next: Some(2),
                    };
                }
                10 => {
                    return Little2EntityValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(pos),
                    };
                }
                9 if pos == 0 => {
                    pos += 2;
                    if pos + 2 > input.len() {
                        return Little2EntityValueToken {
                            token: crate::src::xmltok::XML_TOK_TRAILING_CR_1,
                            next: None,
                        };
                    }
                    if little2_byte_type(&enc.type_0, input, pos)
                        == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                    {
                        pos += 2;
                    }
                    return Little2EntityValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_NEWLINE_1,
                        next: Some(pos),
                    };
                }
                9 => {
                    return Little2EntityValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(pos),
                    };
                }
                _ => pos += 2,
            }
        }

        Little2EntityValueToken {
            token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
            next: Some(pos),
        }
    }

    pub unsafe extern "C" fn little2_entityValueTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = end.offset_from(ptr);
        if input_len <= 0 {
            return crate::src::xmltok::XML_TOK_NONE_1;
        }
        let input = ::core::slice::from_raw_parts(ptr, input_len as usize);
        let normal = &*(enc as *const normal_encoding);
        let result = little2_entity_value_tok_impl(normal, input);
        if let Some(offset) = result.next {
            *nextTokPtr = ptr.add(offset);
        }
        result.token
    }

    enum Little2IgnoreSectionOutcome {
        Token(::core::ffi::c_int, usize),
        Partial(::core::ffi::c_int),
        Invalid(usize),
    }

    fn little2_ignore_section_tok_impl(
        enc: &normal_encoding,
        raw_input: &[::core::ffi::c_char],
    ) -> Little2IgnoreSectionOutcome {
        // The tokenizer processes complete UTF-16 code units only; an odd
        // trailing byte remains for the next buffer just as in the C scanner.
        let input = &raw_input[..raw_input.len() & !1];
        let mut level: ::core::ffi::c_int = 0;
        let mut pos = 0;

        while input.len() - pos >= 2 {
            match little2_byte_type(&enc.type_0, input, pos) {
                5 => {
                    if input.len() - pos < 2 {
                        return Little2IgnoreSectionOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    pos += 2;
                }
                6 => {
                    if input.len() - pos < 3 {
                        return Little2IgnoreSectionOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    pos += 3;
                }
                7 => {
                    if input.len() - pos < 4 {
                        return Little2IgnoreSectionOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    pos += 4;
                }
                0 | 1 | 8 => return Little2IgnoreSectionOutcome::Invalid(pos),
                2 => {
                    pos += 2;
                    if input.len() - pos < 2 {
                        return Little2IgnoreSectionOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_1,
                        );
                    }
                    if input[pos + 1] == 0 && input[pos] == b'!' as ::core::ffi::c_char {
                        pos += 2;
                        if input.len() - pos < 2 {
                            return Little2IgnoreSectionOutcome::Partial(
                                crate::src::xmltok::XML_TOK_PARTIAL_1,
                            );
                        }
                        if input[pos + 1] == 0 && input[pos] == b'[' as ::core::ffi::c_char {
                            level += 1;
                            pos += 2;
                        }
                    }
                }
                4 => {
                    pos += 2;
                    if input.len() - pos < 2 {
                        return Little2IgnoreSectionOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_1,
                        );
                    }
                    if input[pos + 1] == 0 && input[pos] == b']' as ::core::ffi::c_char {
                        pos += 2;
                        if input.len() - pos < 2 {
                            return Little2IgnoreSectionOutcome::Partial(
                                crate::src::xmltok::XML_TOK_PARTIAL_1,
                            );
                        }
                        if input[pos + 1] == 0 && input[pos] == b'>' as ::core::ffi::c_char {
                            pos += 2;
                            if level == 0 {
                                return Little2IgnoreSectionOutcome::Token(
                                    crate::src::xmltok::XML_TOK_IGNORE_SECT_1,
                                    pos,
                                );
                            }
                            level -= 1;
                        }
                    }
                }
                _ => pos += 2,
            }
        }
        Little2IgnoreSectionOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1)
    }

    pub unsafe extern "C" fn little2_ignoreSectionTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = end.offset_from(ptr);
        if input_len <= 0 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = ::core::slice::from_raw_parts(ptr, input_len as usize);
        let encoding = &*(enc as *const normal_encoding);
        match little2_ignore_section_tok_impl(encoding, input) {
            Little2IgnoreSectionOutcome::Token(token, next) => {
                *nextTokPtr = ptr.wrapping_add(next);
                token
            }
            Little2IgnoreSectionOutcome::Partial(token) => token,
            Little2IgnoreSectionOutcome::Invalid(at) => {
                *nextTokPtr = ptr.wrapping_add(at);
                crate::src::xmltok::XML_TOK_INVALID_1
            }
        }
    }

    pub unsafe extern "C" fn little2_isPublicId(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        badPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let span_len = end.offset_from(ptr);
        if span_len < 4 {
            return 1;
        }
        let contents =
            ::core::slice::from_raw_parts(ptr.add(2).cast::<u8>(), (span_len - 4) as usize);
        let byte_types = &(*(enc as *const normal_encoding)).type_0;
        if let Some(offset) = crate::src::xmltok::public_id_bad_offset(
            contents,
            byte_types,
            crate::src::xmltok::PublicIdChecker::Little2,
        ) {
            *badPtr = ptr.add(2 + offset);
            return 0;
        }
        1
    }

    #[derive(Copy, Clone)]
    enum Little2AttributeAction {
        Name {
            attribute: ::core::ffi::c_int,
            offset: usize,
        },
        ValueStart {
            attribute: ::core::ffi::c_int,
            offset: usize,
        },
        ValueEnd {
            attribute: ::core::ffi::c_int,
            offset: usize,
        },
        Normalized {
            attribute: ::core::ffi::c_int,
            value: ::core::ffi::c_char,
        },
    }

    /// Scans a complete little-endian UTF-16 start-tag token with slice
    /// indices, reporting only safe offsets to its boundary adapter.
    fn scan_little2_atts(
        byte_types: &[::core::ffi::c_uchar; 256],
        source: &[u8],
        mut report: impl FnMut(Little2AttributeAction),
    ) -> ::core::ffi::c_int {
        #[derive(Copy, Clone, Eq, PartialEq)]
        enum State {
            InName,
            InValue,
            Other,
        }

        fn byte_type(
            byte_types: &[::core::ffi::c_uchar; 256],
            source: &[u8],
            index: usize,
        ) -> Option<::core::ffi::c_int> {
            let lo = *source.get(index)?;
            let hi = *source.get(index + 1)?;
            Some(if hi == 0 {
                byte_types[lo as usize] as ::core::ffi::c_int
            } else {
                unicode_byte_type(hi as ::core::ffi::c_char, lo as ::core::ffi::c_char)
            })
        }

        let mut state = State::InName;
        let mut n_atts: ::core::ffi::c_int = 0;
        let mut open = 0;
        let mut value_start = None;
        let mut normalized = true;
        let mut index = 2;

        while let Some(kind) = byte_type(byte_types, source, index) {
            match kind {
                5 | 6 | 7 | 29 | 22 | 24 => {
                    if state == State::Other {
                        report(Little2AttributeAction::Name {
                            attribute: n_atts,
                            offset: index,
                        });
                        normalized = true;
                        report(Little2AttributeAction::Normalized {
                            attribute: n_atts,
                            value: 1,
                        });
                        state = State::InName;
                    }
                    index += match kind {
                        5 => 2,
                        6 => 3,
                        7 => 4,
                        _ => 2,
                    };
                    continue;
                }
                12 | 13 => {
                    let quote_kind = if kind == 12 {
                        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int
                    } else {
                        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int
                    };
                    if state != State::InValue {
                        report(Little2AttributeAction::ValueStart {
                            attribute: n_atts,
                            offset: index + 2,
                        });
                        value_start = Some(index + 2);
                        state = State::InValue;
                        open = quote_kind;
                    } else if open == quote_kind {
                        state = State::Other;
                        report(Little2AttributeAction::ValueEnd {
                            attribute: n_atts,
                            offset: index,
                        });
                        n_atts += 1;
                        value_start = None;
                    }
                }
                3 => {
                    normalized = false;
                    report(Little2AttributeAction::Normalized {
                        attribute: n_atts,
                        value: 0,
                    });
                }
                21 => {
                    if state == State::InName {
                        state = State::Other;
                    } else if state == State::InValue && normalized {
                        let current_char = if source[index + 1] == 0 {
                            source[index] as ::core::ffi::c_int
                        } else {
                            -1
                        };
                        let next_char = source
                            .get(index + 3)
                            .copied()
                            .filter(|_| source.get(index + 2) == Some(&0))
                            .map(::core::ffi::c_int::from)
                            .unwrap_or(-1);
                        if value_start == Some(index)
                            || current_char != crate::ascii_h::ASCII_SPACE
                            || next_char == crate::ascii_h::ASCII_SPACE
                            || byte_type(byte_types, source, index + 2) == Some(open)
                        {
                            normalized = false;
                            report(Little2AttributeAction::Normalized {
                                attribute: n_atts,
                                value: 0,
                            });
                        }
                    }
                }
                9 | 10 => {
                    if state == State::InName {
                        state = State::Other;
                    } else if state == State::InValue {
                        normalized = false;
                        report(Little2AttributeAction::Normalized {
                            attribute: n_atts,
                            value: 0,
                        });
                    }
                }
                11 | 17 if state != State::InValue => return n_atts,
                _ => {}
            }
            index += 2;
        }
        n_atts
    }

    /// Boundary adapter for the legacy pointer-based tokenizer call sites.
    /// `end` is the token end returned by the tokenizer scanner.
    pub unsafe extern "C" fn little2_getAtts(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        attsMax: ::core::ffi::c_int,
        atts: *mut crate::src::xmltok::ATTRIBUTE,
    ) -> ::core::ffi::c_int {
        let source_len = end.offset_from(ptr);
        if source_len < 0 {
            return 0;
        }
        let source = ::core::slice::from_raw_parts(ptr.cast::<u8>(), source_len as usize);
        let byte_types = &(*(enc as *const normal_encoding)).type_0;
        scan_little2_atts(byte_types, source, |action| {
            let attribute = match action {
                Little2AttributeAction::Name { attribute, .. }
                | Little2AttributeAction::ValueStart { attribute, .. }
                | Little2AttributeAction::ValueEnd { attribute, .. }
                | Little2AttributeAction::Normalized { attribute, .. } => attribute,
            };
            if attribute < 0 || attribute >= attsMax {
                return;
            }
            let slot = atts.add(attribute as usize);
            match action {
                Little2AttributeAction::Name { offset, .. } => {
                    (*slot).name = ptr.add(offset);
                }
                Little2AttributeAction::ValueStart { offset, .. } => {
                    (*slot).valuePtr = ptr.add(offset);
                }
                Little2AttributeAction::ValueEnd { offset, .. } => {
                    (*slot).valueEnd = ptr.add(offset);
                }
                Little2AttributeAction::Normalized { value, .. } => {
                    (*slot).normalized = value;
                }
            }
        })
    }

    pub unsafe extern "C" fn little2_charRefNumber(
        _enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        ptr = ptr.offset((2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize);
        if *ptr.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && *ptr.offset(0 as isize) as ::core::ffi::c_int == 0x78 as ::core::ffi::c_int
        {
            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            while !(*ptr.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && *ptr.offset(0 as isize) as ::core::ffi::c_int == 0x3b as ::core::ffi::c_int)
            {
                let mut c: ::core::ffi::c_int =
                    if *ptr.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        *ptr.offset(0 as isize) as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    };
                match c {
                    crate::ascii_h::ASCII_0
                    | crate::ascii_h::ASCII_1_1
                    | crate::ascii_h::ASCII_2_1
                    | crate::ascii_h::ASCII_3_1
                    | crate::ascii_h::ASCII_4
                    | crate::ascii_h::ASCII_5
                    | crate::ascii_h::ASCII_6
                    | crate::ascii_h::ASCII_7
                    | crate::ascii_h::ASCII_8_1
                    | crate::ascii_h::ASCII_9_1 => {
                        result <<= 4 as ::core::ffi::c_int;
                        result |= c - crate::ascii_h::ASCII_0;
                    }
                    crate::ascii_h::ASCII_A
                    | crate::ascii_h::ASCII_B_1
                    | crate::ascii_h::ASCII_C
                    | crate::ascii_h::ASCII_D
                    | crate::ascii_h::ASCII_E_1
                    | crate::ascii_h::ASCII_F_1 => {
                        result <<= 4 as ::core::ffi::c_int;
                        result += 10 as ::core::ffi::c_int + (c - crate::ascii_h::ASCII_A);
                    }
                    crate::ascii_h::ASCII_a_1
                    | crate::ascii_h::ASCII_b
                    | crate::ascii_h::ASCII_c_1
                    | crate::ascii_h::ASCII_d
                    | crate::ascii_h::ASCII_e_1
                    | crate::ascii_h::ASCII_f => {
                        result <<= 4 as ::core::ffi::c_int;
                        result += 10 as ::core::ffi::c_int + (c - crate::ascii_h::ASCII_a_1);
                    }
                    _ => {}
                }
                if result >= 0x110000 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            }
        } else {
            while !(*ptr.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && *ptr.offset(0 as isize) as ::core::ffi::c_int == 0x3b as ::core::ffi::c_int)
            {
                let mut c_0: ::core::ffi::c_int =
                    if *ptr.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        *ptr.offset(0 as isize) as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    };
                result *= 10 as ::core::ffi::c_int;
                result += c_0 - crate::ascii_h::ASCII_0;
                if result >= 0x110000 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            }
        }
        return checkCharRefNumber(result);
    }

    pub unsafe extern "C" fn little2_nameLength(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ptr;
        loop {
            match if *ptr.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1 as isize), *ptr.offset(0 as isize))
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
                29 | 22 | 23 | 24 | 25 | 26 | 27 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                _ => return ptr.offset_from(start) as ::core::ffi::c_int,
            }
        }
    }

    fn little2_update_position(
        encoding: &normal_encoding,
        bytes: &[::core::ffi::c_char],
        pos: &mut crate::src::xmltok::POSITION,
    ) {
        let mut offset = 0;
        while bytes.len().saturating_sub(offset) >= 2 {
            match little2_byte_type(&encoding.type_0, bytes, offset) {
                5 => {
                    offset = (offset + 2).min(bytes.len());
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                6 => {
                    offset = (offset + 3).min(bytes.len());
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                7 => {
                    offset = (offset + 4).min(bytes.len());
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                10 => {
                    pos.columnNumber = 0 as crate::expat_external_h::XML_Size;
                    pos.lineNumber = pos.lineNumber.wrapping_add(1);
                    offset = (offset + 2).min(bytes.len());
                }
                9 => {
                    pos.lineNumber = pos.lineNumber.wrapping_add(1);
                    offset = (offset + 2).min(bytes.len());
                    if bytes.len().saturating_sub(offset) >= 2
                        && little2_byte_type(&encoding.type_0, bytes, offset)
                            == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                    {
                        offset = (offset + 2).min(bytes.len());
                    }
                    pos.columnNumber = 0 as crate::expat_external_h::XML_Size;
                }
                _ => {
                    offset = (offset + 2).min(bytes.len());
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
            }
        }
    }

    pub unsafe extern "C" fn little2_updatePosition(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        pos: *mut crate::src::xmltok::POSITION,
    ) {
        let byte_len = unsafe { end.offset_from(ptr) };
        if byte_len <= 0 {
            return;
        }
        let encoding = unsafe { &*(enc as *const normal_encoding) };
        let bytes = unsafe { ::core::slice::from_raw_parts(ptr, byte_len as usize) };
        let pos = unsafe { &mut *pos };
        little2_update_position(encoding, bytes, pos);
    }

    pub unsafe extern "C" fn big2_scanComment(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = end.offset_from(ptr);
        if input_len < 0 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = ::core::slice::from_raw_parts(ptr, input_len as usize);
        let normal = &*(enc as *const normal_encoding);
        match big2_scan_comment_impl(normal, input) {
            Big2ScanOutcome::Token(token, next) => {
                *nextTokPtr = ptr.add(next);
                token
            }
            Big2ScanOutcome::Partial(token) => token,
            Big2ScanOutcome::Invalid(at) => {
                *nextTokPtr = ptr.add(at);
                crate::src::xmltok::XML_TOK_INVALID_1
            }
        }
    }

    pub unsafe extern "C" fn big2_scanDecl(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = end.offset_from(ptr);
        if input_len < (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = ::core::slice::from_raw_parts(ptr, input_len as usize);
        let normal = &*(enc as *const normal_encoding);
        let mut pos = 0usize;
        match big2_byte_type(normal, input, pos) {
            27 => {
                return big2_scanComment(
                    enc,
                    ptr.offset(2 as ::core::ffi::c_int as isize),
                    end,
                    nextTokPtr,
                );
            }
            20 => {
                *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                return crate::src::xmltok::XML_TOK_COND_SECT_OPEN_1;
            }
            22 | 24 => {
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                pos += 2;
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
        }
        while input.len() - pos >= 2 {
            's_129: {
                match big2_byte_type(normal, input, pos) {
                    30 => {
                        if input.len() - pos < 4 {
                            return crate::src::xmltok::XML_TOK_PARTIAL_1;
                        }
                        match big2_byte_type(normal, input, pos + 2) {
                            21 | 9 | 10 | 30 => {
                                *nextTokPtr = ptr;
                                return crate::src::xmltok::XML_TOK_INVALID_1;
                            }
                            _ => {}
                        }
                    }
                    21 | 9 | 10 => {}
                    22 | 24 => {
                        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        pos += 2;
                        break 's_129;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                }
                *nextTokPtr = ptr;
                return crate::src::xmltok::XML_TOK_DECL_OPEN_1;
            }
        }
        return crate::src::xmltok::XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn big2_checkPiTarget(
        _enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut tokPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        let mut upper: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        *tokPtr = crate::src::xmltok::XML_TOK_PI_1;
        if end.offset_from(ptr) != (2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize {
            return 1 as ::core::ffi::c_int;
        }
        match if *ptr.offset(0 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            *ptr.offset(1 as isize) as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        } {
            crate::ascii_h::ASCII_x_1 => {}
            crate::ascii_h::ASCII_X_1 => {
                upper = 1 as ::core::ffi::c_int;
            }
            _ => return 1 as ::core::ffi::c_int,
        }
        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
        match if *ptr.offset(0 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            *ptr.offset(1 as isize) as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        } {
            crate::ascii_h::ASCII_m_1 => {}
            crate::ascii_h::ASCII_M_1 => {
                upper = 1 as ::core::ffi::c_int;
            }
            _ => return 1 as ::core::ffi::c_int,
        }
        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
        match if *ptr.offset(0 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            *ptr.offset(1 as isize) as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        } {
            crate::ascii_h::ASCII_l_1 => {}
            crate::ascii_h::ASCII_L_1 => {
                upper = 1 as ::core::ffi::c_int;
            }
            _ => return 1 as ::core::ffi::c_int,
        }
        if upper != 0 {
            return 0 as ::core::ffi::c_int;
        }
        *tokPtr = crate::src::xmltok::XML_TOK_XML_DECL_1;
        return 1 as ::core::ffi::c_int;
    }

    fn big2_byte_type(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
        offset: usize,
    ) -> ::core::ffi::c_int {
        let hi = input[offset];
        let lo = input[offset + 1];
        if hi == 0 {
            enc.type_0[lo as u8 as usize] as ::core::ffi::c_int
        } else {
            unicode_byte_type(hi, lo)
        }
    }

    fn big2_name_char_is_valid(
        input: &[::core::ffi::c_char],
        offset: usize,
        pages: &[::core::ffi::c_uchar; 256],
    ) -> bool {
        let hi = input[offset] as u8 as usize;
        let lo = input[offset + 1] as u8;
        let bitmap_index = pages[hi] as usize * 8 + (lo >> 5) as usize;
        namingBitmap[bitmap_index] & (1 << (lo & 0x1f)) != 0
    }

    fn big2_pi_target_token(
        input: &[::core::ffi::c_char],
        target: usize,
        terminator: usize,
    ) -> Option<::core::ffi::c_int> {
        if terminator - target != 6 {
            return Some(crate::src::xmltok::XML_TOK_PI_1);
        }

        let mut upper = false;
        for (offset, lower, upper_case) in [
            (0, crate::ascii_h::ASCII_x_1, crate::ascii_h::ASCII_X_1),
            (2, crate::ascii_h::ASCII_m_1, crate::ascii_h::ASCII_M_1),
            (4, crate::ascii_h::ASCII_l_1, crate::ascii_h::ASCII_L_1),
        ] {
            if input[target + offset] != 0 {
                return Some(crate::src::xmltok::XML_TOK_PI_1);
            }
            match input[target + offset + 1] as ::core::ffi::c_int {
                value if value == lower => {}
                value if value == upper_case => upper = true,
                _ => return Some(crate::src::xmltok::XML_TOK_PI_1),
            }
        }

        if upper {
            None
        } else {
            Some(crate::src::xmltok::XML_TOK_XML_DECL_1)
        }
    }

    fn big2_scan_pi_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> (::core::ffi::c_int, Option<usize>) {
        if input.len() < 2 {
            return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }

        let target = 0;
        let mut offset = 0;
        match big2_byte_type(enc, input, offset) {
            29 => {
                if !big2_name_char_is_valid(input, offset, &nmstrtPages) {
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                }
            }
            22 | 24 => {}
            5 => {
                if input.len() < 2 {
                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                }
                return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
            }
            6 => {
                if input.len() < 3 {
                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                }
                return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
            }
            7 => {
                if input.len() < 4 {
                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                }
                return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
            }
            _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
        }
        offset += 2;

        while input.len() - offset >= 2 {
            match big2_byte_type(enc, input, offset) {
                29 => {
                    if !big2_name_char_is_valid(input, offset, &namePages) {
                        return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                    }
                    offset += 2;
                }
                22 | 24 | 25 | 26 | 27 => offset += 2,
                5 => {
                    if input.len() - offset < 2 {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                    }
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                }
                6 => {
                    if input.len() - offset < 3 {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                    }
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                }
                7 => {
                    if input.len() - offset < 4 {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                    }
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                }
                21 | 9 | 10 => {
                    let Some(token) = big2_pi_target_token(input, target, offset) else {
                        return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                    };
                    offset += 2;
                    while input.len() - offset >= 2 {
                        match big2_byte_type(enc, input, offset) {
                            5 => {
                                if input.len() - offset < 2 {
                                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                                }
                                offset += 2;
                            }
                            6 => {
                                if input.len() - offset < 3 {
                                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                                }
                                offset += 3;
                            }
                            7 => {
                                if input.len() - offset < 4 {
                                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                                }
                                offset += 4;
                            }
                            0 | 1 | 8 => {
                                return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                            }
                            15 => {
                                offset += 2;
                                if input.len() - offset < 2 {
                                    return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                                }
                                if input[offset] == 0 && input[offset + 1] as u8 == 0x3e {
                                    return (token, Some(offset + 2));
                                }
                            }
                            _ => offset += 2,
                        }
                    }
                    return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                }
                15 => {
                    let Some(token) = big2_pi_target_token(input, target, offset) else {
                        return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                    };
                    offset += 2;
                    if input.len() - offset < 2 {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                    }
                    if input[offset] == 0 && input[offset + 1] as u8 == 0x3e {
                        return (token, Some(offset + 2));
                    }
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                }
                _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
            }
        }

        (crate::src::xmltok::XML_TOK_PARTIAL_1, None)
    }

    pub unsafe extern "C" fn big2_scanPi(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = unsafe { end.offset_from(ptr) };
        if input_len <= 0 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = unsafe { ::core::slice::from_raw_parts(ptr, input_len as usize) };
        if input.len() < 2 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let enc = unsafe { &*(enc as *const normal_encoding) };
        let (token, next) = big2_scan_pi_impl(enc, input);
        if let Some(offset) = next {
            unsafe {
                *nextTokPtr = ptr.add(offset);
            }
        }
        token
    }
    pub unsafe extern "C" fn big2_scanCdataSection(
        _enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        pub static mut CDATA_LSQB: [::core::ffi::c_char; 6] = [
            crate::ascii_h::ASCII_C as ::core::ffi::c_char,
            crate::ascii_h::ASCII_D as ::core::ffi::c_char,
            crate::ascii_h::ASCII_A as ::core::ffi::c_char,
            crate::ascii_h::ASCII_T as ::core::ffi::c_char,
            crate::ascii_h::ASCII_A as ::core::ffi::c_char,
            crate::ascii_h::ASCII_LSQB as ::core::ffi::c_char,
        ];
        let mut i: ::core::ffi::c_int = 0;
        if !(end.offset_from(ptr) >= (6 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize) {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < 6 as ::core::ffi::c_int {
            if !(*ptr.offset(0 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && *ptr.offset(1 as isize) as ::core::ffi::c_int
                    == CDATA_LSQB[i as usize] as ::core::ffi::c_int)
            {
                *nextTokPtr = ptr;
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            i += 1;
            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
        }
        *nextTokPtr = ptr;
        return crate::src::xmltok::XML_TOK_CDATA_SECT_OPEN_1;
    }

    struct Big2CdataSectionResult {
        token: ::core::ffi::c_int,
        next: Option<usize>,
    }

    fn big2_cdata_section_result(
        token: ::core::ffi::c_int,
        next: Option<usize>,
    ) -> Big2CdataSectionResult {
        Big2CdataSectionResult { token, next }
    }

    fn big2_cdata_section_tok_impl(enc: &normal_encoding, input: &[u8]) -> Big2CdataSectionResult {
        if input.is_empty() {
            return big2_cdata_section_result(crate::src::xmltok::XML_TOK_NONE_1, None);
        }

        let input = &input[..input.len() & !1];
        if input.is_empty() {
            return big2_cdata_section_result(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }

        let byte_type = |offset: usize| {
            if input[offset] == 0 {
                enc.type_0[input[offset + 1] as usize] as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    input[offset] as ::core::ffi::c_char,
                    input[offset + 1] as ::core::ffi::c_char,
                )
            }
        };

        let mut offset = match byte_type(0) {
            4 => {
                if input.len() < 4 {
                    return big2_cdata_section_result(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                }
                if input[2] == 0 && input[3] == b']' {
                    if input.len() < 6 {
                        return big2_cdata_section_result(
                            crate::src::xmltok::XML_TOK_PARTIAL_1,
                            None,
                        );
                    }
                    if input[4] == 0 && input[5] == b'>' {
                        return big2_cdata_section_result(
                            crate::src::xmltok::XML_TOK_CDATA_SECT_CLOSE_1,
                            Some(6),
                        );
                    }
                }
                2
            }
            9 => {
                if input.len() < 4 {
                    return big2_cdata_section_result(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                }
                let next = if byte_type(2) == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int {
                    4
                } else {
                    2
                };
                return big2_cdata_section_result(
                    crate::src::xmltok::XML_TOK_DATA_NEWLINE_1,
                    Some(next),
                );
            }
            10 => {
                return big2_cdata_section_result(
                    crate::src::xmltok::XML_TOK_DATA_NEWLINE_1,
                    Some(2),
                );
            }
            5 => {
                if input.len() < 2 {
                    return big2_cdata_section_result(
                        crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        None,
                    );
                }
                2
            }
            6 => {
                if input.len() < 3 {
                    return big2_cdata_section_result(
                        crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        None,
                    );
                }
                3
            }
            7 => {
                if input.len() < 4 {
                    return big2_cdata_section_result(
                        crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        None,
                    );
                }
                4
            }
            0 | 1 | 8 => {
                return big2_cdata_section_result(crate::src::xmltok::XML_TOK_INVALID_1, Some(0));
            }
            _ => 2,
        };

        while input.len().saturating_sub(offset) >= 2 {
            match byte_type(offset) {
                5 => {
                    if input.len() - offset < 2 {
                        return big2_cdata_section_result(
                            crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                            Some(offset),
                        );
                    }
                    offset += 2;
                }
                6 => {
                    if input.len() - offset < 3 {
                        return big2_cdata_section_result(
                            crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                            Some(offset),
                        );
                    }
                    offset += 3;
                }
                7 => {
                    if input.len() - offset < 4 {
                        return big2_cdata_section_result(
                            crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                            Some(offset),
                        );
                    }
                    offset += 4;
                }
                0 | 1 | 8 | 9 | 10 | 4 => {
                    return big2_cdata_section_result(
                        crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        Some(offset),
                    );
                }
                _ => offset += 2,
            }
        }

        big2_cdata_section_result(crate::src::xmltok::XML_TOK_DATA_CHARS_1, Some(offset))
    }

    pub unsafe extern "C" fn big2_cdataSectionTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if ptr >= end {
            return crate::src::xmltok::XML_TOK_NONE_1;
        }
        let input_len = end.offset_from(ptr) as usize;
        let input = ::core::slice::from_raw_parts(ptr.cast::<u8>(), input_len);
        let normal = &*(enc as *const normal_encoding);
        let result = big2_cdata_section_tok_impl(normal, input);
        if let Some(next) = result.next {
            *nextTokPtr = ptr.add(next);
        }
        result.token
    }

    pub unsafe extern "C" fn big2_scanEndTag(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = end.offset_from(ptr);
        if input_len < 0 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = ::core::slice::from_raw_parts(ptr, input_len as usize);
        let normal = &*(enc as *const normal_encoding);
        match big2_scan_end_tag_impl(normal, input) {
            Big2ScanOutcome::Token(token, next) => {
                *nextTokPtr = ptr.add(next);
                token
            }
            Big2ScanOutcome::Partial(token) => token,
            Big2ScanOutcome::Invalid(at) => {
                *nextTokPtr = ptr.add(at);
                crate::src::xmltok::XML_TOK_INVALID_1
            }
        }
    }

    pub unsafe extern "C" fn big2_scanHexCharRef(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if end.offset_from(ptr) >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize {
            match if *ptr.offset(0 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0 as isize), *ptr.offset(1 as isize))
            } {
                25 | 24 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
            }
            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            while end.offset_from(ptr)
                >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize
            {
                match if *ptr.offset(0 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    (*(enc as *const normal_encoding)).type_0[*ptr
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(0 as isize), *ptr.offset(1 as isize))
                } {
                    25 | 24 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        return crate::src::xmltok::XML_TOK_CHAR_REF_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                }
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            }
        }
        return crate::src::xmltok::XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn big2_scanRef(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = end.offset_from(ptr);
        if input_len < 0 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = ::core::slice::from_raw_parts(ptr, input_len as usize);
        let normal = &*(enc as *const normal_encoding);
        match big2_scan_ref(normal, input, 0) {
            Big2ScanOutcome::Token(token, next) => {
                *nextTokPtr = ptr.add(next);
                token
            }
            Big2ScanOutcome::Partial(token) => token,
            Big2ScanOutcome::Invalid(at) => {
                *nextTokPtr = ptr.add(at);
                crate::src::xmltok::XML_TOK_INVALID_1
            }
        }
    }

    // Kept as a disabled translation reference while the slice implementation below
    // replaces it.  The callable scanner is `big2_scanAtts` after this block.
    #[cfg(any())]
    pub unsafe extern "C" fn big2_scanAtts_legacy(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut hadColon: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while end.offset_from(ptr) >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize {
            's_852: {
                'c_45267: {
                    match if *ptr.offset(0 as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        (*(enc as *const normal_encoding)).type_0[*ptr
                            .offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uchar
                            as usize] as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(*ptr.offset(0 as isize), *ptr.offset(1 as isize))
                    } {
                        29 => {
                            if namingBitmap[(((namePages
                                [*ptr.offset(0 as isize) as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int)
                                << 3 as ::core::ffi::c_int)
                                + (*ptr.offset(1 as isize) as ::core::ffi::c_uchar
                                    as ::core::ffi::c_int
                                    >> 5 as ::core::ffi::c_int))
                                as usize]
                                & (1 as ::core::ffi::c_uint)
                                    << (*ptr.offset(1 as isize) as ::core::ffi::c_uchar
                                        as ::core::ffi::c_int
                                        & 0x1f as ::core::ffi::c_int)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                return crate::src::xmltok::XML_TOK_INVALID_1;
                            }
                            break 'c_45267;
                        }
                        22 | 24 | 25 | 26 | 27 => {
                            break 'c_45267;
                        }
                        5 => {
                            if end.offset_from(ptr) < 2 as isize {
                                return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                            }
                            if false || true {
                                *nextTokPtr = ptr;
                                return crate::src::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            break 's_852;
                        }
                        6 => {
                            if end.offset_from(ptr) < 3 as isize {
                                return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                            }
                            if false || true {
                                *nextTokPtr = ptr;
                                return crate::src::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                            break 's_852;
                        }
                        7 => {
                            if end.offset_from(ptr) < 4 as isize {
                                return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                            }
                            if false || true {
                                *nextTokPtr = ptr;
                                return crate::src::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                            break 's_852;
                        }
                        23 => {
                            if hadColon != 0 {
                                *nextTokPtr = ptr;
                                return crate::src::xmltok::XML_TOK_INVALID_1;
                            }
                            hadColon = 1 as ::core::ffi::c_int;
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            if !(end.offset_from(ptr)
                                >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                            {
                                return crate::src::xmltok::XML_TOK_PARTIAL_1;
                            }
                            's_275: {
                                match if *ptr.offset(0 as isize) as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                {
                                    (*(enc as *const normal_encoding)).type_0[*ptr
                                        .offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uchar
                                        as usize]
                                        as ::core::ffi::c_int
                                } else {
                                    unicode_byte_type(
                                        *ptr.offset(0 as isize),
                                        *ptr.offset(1 as isize),
                                    )
                                } {
                                    29 => {
                                        if namingBitmap[(((nmstrtPages[*ptr.offset(0 as isize)
                                            as ::core::ffi::c_uchar
                                            as usize]
                                            as ::core::ffi::c_int)
                                            << 3 as ::core::ffi::c_int)
                                            + (*ptr.offset(1 as isize) as ::core::ffi::c_uchar
                                                as ::core::ffi::c_int
                                                >> 5 as ::core::ffi::c_int))
                                            as usize]
                                            & (1 as ::core::ffi::c_uint)
                                                << (*ptr.offset(1 as isize) as ::core::ffi::c_uchar
                                                    as ::core::ffi::c_int
                                                    & 0x1f as ::core::ffi::c_int)
                                            == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return crate::src::xmltok::XML_TOK_INVALID_1;
                                        }
                                    }
                                    22 | 24 => {}
                                    5 => {
                                        if end.offset_from(ptr) < 2 as isize {
                                            return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if false || true {
                                            *nextTokPtr = ptr;
                                            return crate::src::xmltok::XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                        break 's_275;
                                    }
                                    6 => {
                                        if end.offset_from(ptr) < 3 as isize {
                                            return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if false || true {
                                            *nextTokPtr = ptr;
                                            return crate::src::xmltok::XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                                        break 's_275;
                                    }
                                    7 => {
                                        if end.offset_from(ptr) < 4 as isize {
                                            return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if false || true {
                                            *nextTokPtr = ptr;
                                            return crate::src::xmltok::XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                                        break 's_275;
                                    }
                                    _ => {
                                        *nextTokPtr = ptr;
                                        return crate::src::xmltok::XML_TOK_INVALID_1;
                                    }
                                }
                                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            }
                            break 's_852;
                        }
                        21 | 9 | 10 => loop {
                            let mut t: ::core::ffi::c_int = 0;
                            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            if !(end.offset_from(ptr)
                                >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                            {
                                return crate::src::xmltok::XML_TOK_PARTIAL_1;
                            }
                            t = if *ptr.offset(0 as isize) as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                (*(enc as *const normal_encoding)).type_0[*ptr
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uchar
                                    as usize] as ::core::ffi::c_int
                            } else {
                                unicode_byte_type(*ptr.offset(0 as isize), *ptr.offset(1 as isize))
                            };
                            if t == crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int {
                                break;
                            }
                            match t {
                                21 | 10 | 9 => {}
                                _ => {
                                    *nextTokPtr = ptr;
                                    return crate::src::xmltok::XML_TOK_INVALID_1;
                                }
                            }
                        },
                        14 => {}
                        _ => {
                            *nextTokPtr = ptr;
                            return crate::src::xmltok::XML_TOK_INVALID_1;
                        }
                    }
                    let mut open: ::core::ffi::c_int = 0;
                    hadColon = 0 as ::core::ffi::c_int;
                    loop {
                        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        if !(end.offset_from(ptr)
                            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                        {
                            return crate::src::xmltok::XML_TOK_PARTIAL_1;
                        }
                        open = if *ptr.offset(0 as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(0 as isize), *ptr.offset(1 as isize))
                        };
                        if open == crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int
                            || open == crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int
                        {
                            break;
                        }
                        match open {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return crate::src::xmltok::XML_TOK_INVALID_1;
                            }
                        }
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    loop {
                        let mut t_0: ::core::ffi::c_int = 0;
                        if !(end.offset_from(ptr)
                            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                        {
                            return crate::src::xmltok::XML_TOK_PARTIAL_1;
                        }
                        t_0 = if *ptr.offset(0 as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(0 as isize), *ptr.offset(1 as isize))
                        };
                        if t_0 == open {
                            break;
                        }
                        match t_0 {
                            5 => {
                                if end.offset_from(ptr) < 2 as isize {
                                    return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            }
                            6 => {
                                if end.offset_from(ptr) < 3 as isize {
                                    return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                            }
                            7 => {
                                if end.offset_from(ptr) < 4 as isize {
                                    return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                            }
                            0 | 1 | 8 => {
                                *nextTokPtr = ptr;
                                return crate::src::xmltok::XML_TOK_INVALID_1;
                            }
                            3 => {
                                let mut tok: ::core::ffi::c_int = big2_scanRef(
                                    enc,
                                    ptr.offset(2 as ::core::ffi::c_int as isize),
                                    end,
                                    &raw mut ptr,
                                );
                                if tok <= 0 as ::core::ffi::c_int {
                                    if tok == crate::src::xmltok::XML_TOK_INVALID_1 {
                                        *nextTokPtr = ptr;
                                    }
                                    return tok;
                                }
                            }
                            2 => {
                                *nextTokPtr = ptr;
                                return crate::src::xmltok::XML_TOK_INVALID_1;
                            }
                            _ => {
                                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                            }
                        }
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr)
                        >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                    {
                        return crate::src::xmltok::XML_TOK_PARTIAL_1;
                    }
                    '_sol: {
                        '_gt: {
                            match if *ptr.offset(0 as isize) as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                (*(enc as *const normal_encoding)).type_0[*ptr
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uchar
                                    as usize] as ::core::ffi::c_int
                            } else {
                                unicode_byte_type(*ptr.offset(0 as isize), *ptr.offset(1 as isize))
                            } {
                                21 | 9 | 10 => {
                                    loop {
                                        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                        if !(end.offset_from(ptr)
                                            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                                as isize)
                                        {
                                            return crate::src::xmltok::XML_TOK_PARTIAL_1;
                                        }
                                        match if *ptr.offset(0 as isize) as ::core::ffi::c_int
                                            == 0 as ::core::ffi::c_int
                                        {
                                            (*(enc as *const normal_encoding)).type_0[*ptr
                                                .offset(1 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_uchar
                                                as usize]
                                                as ::core::ffi::c_int
                                        } else {
                                            unicode_byte_type(
                                                *ptr.offset(0 as isize),
                                                *ptr.offset(1 as isize),
                                            )
                                        } {
                                            29 => {
                                                if namingBitmap[(((nmstrtPages[*ptr
                                                    .offset(0 as isize)
                                                    as ::core::ffi::c_uchar
                                                    as usize]
                                                    as ::core::ffi::c_int)
                                                    << 3 as ::core::ffi::c_int)
                                                    + (*ptr.offset(1 as isize)
                                                        as ::core::ffi::c_uchar
                                                        as ::core::ffi::c_int
                                                        >> 5 as ::core::ffi::c_int))
                                                    as usize]
                                                    & (1 as ::core::ffi::c_uint)
                                                        << (*ptr.offset(1 as isize)
                                                            as ::core::ffi::c_uchar
                                                            as ::core::ffi::c_int
                                                            & 0x1f as ::core::ffi::c_int)
                                                    == 0
                                                {
                                                    *nextTokPtr = ptr;
                                                    return crate::src::xmltok::XML_TOK_INVALID_1;
                                                }
                                                break;
                                            }
                                            22 | 24 => {
                                                break;
                                            }
                                            5 => {
                                                if end.offset_from(ptr) < 2 as isize {
                                                    return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                                }
                                                if false || true {
                                                    *nextTokPtr = ptr;
                                                    return crate::src::xmltok::XML_TOK_INVALID_1;
                                                }
                                                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                                break 's_852;
                                            }
                                            6 => {
                                                if end.offset_from(ptr) < 3 as isize {
                                                    return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                                }
                                                if false || true {
                                                    *nextTokPtr = ptr;
                                                    return crate::src::xmltok::XML_TOK_INVALID_1;
                                                }
                                                ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                                                break 's_852;
                                            }
                                            7 => {
                                                if end.offset_from(ptr) < 4 as isize {
                                                    return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                                }
                                                if false || true {
                                                    *nextTokPtr = ptr;
                                                    return crate::src::xmltok::XML_TOK_INVALID_1;
                                                }
                                                ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                                                break 's_852;
                                            }
                                            21 | 9 | 10 => {}
                                            11 => {
                                                break '_gt;
                                            }
                                            17 => {
                                                break '_sol;
                                            }
                                            _ => {
                                                *nextTokPtr = ptr;
                                                return crate::src::xmltok::XML_TOK_INVALID_1;
                                            }
                                        }
                                    }
                                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                                    break 's_852;
                                }
                                17 => {
                                    break '_sol;
                                }
                                11 => {}
                                _ => {
                                    *nextTokPtr = ptr;
                                    return crate::src::xmltok::XML_TOK_INVALID_1;
                                }
                            }
                        }
                        *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                        return crate::src::xmltok::XML_TOK_START_TAG_WITH_ATTS_1;
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    if !(end.offset_from(ptr)
                        >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                    {
                        return crate::src::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if !(*ptr.offset(0 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        && *ptr.offset(1 as isize) as ::core::ffi::c_int
                            == 0x3e as ::core::ffi::c_int)
                    {
                        *nextTokPtr = ptr;
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    return crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1;
                }
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            }
        }
        return crate::src::xmltok::XML_TOK_PARTIAL_1;
    }

    enum Big2ScanOutcome {
        Token(::core::ffi::c_int, usize),
        Partial(::core::ffi::c_int),
        Invalid(usize),
    }

    enum Big2NameCheck {
        Advance(usize),
        PartialChar,
        Invalid,
    }

    fn big2_check_name(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
        pos: usize,
        start: bool,
    ) -> Big2NameCheck {
        match big2_byte_type(enc, input, pos) {
            29 => {
                let pages = if start { &nmstrtPages } else { &namePages };
                if big2_name_char_is_valid(input, pos, pages) {
                    Big2NameCheck::Advance(pos + 2)
                } else {
                    Big2NameCheck::Invalid
                }
            }
            22 | 24 if start => Big2NameCheck::Advance(pos + 2),
            22 | 24 | 25 | 26 | 27 if !start => Big2NameCheck::Advance(pos + 2),
            5 => {
                if input.len() - pos < 2 {
                    Big2NameCheck::PartialChar
                } else {
                    Big2NameCheck::Invalid
                }
            }
            6 => {
                if input.len() - pos < 3 {
                    Big2NameCheck::PartialChar
                } else {
                    Big2NameCheck::Invalid
                }
            }
            7 => {
                if input.len() - pos < 4 {
                    Big2NameCheck::PartialChar
                } else {
                    Big2NameCheck::Invalid
                }
            }
            _ => Big2NameCheck::Invalid,
        }
    }

    fn big2_scan_char_ref(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
        mut pos: usize,
    ) -> Big2ScanOutcome {
        if pos + 2 > input.len() {
            return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
        }
        if input[pos] == 0 && input[pos + 1] == b'x' as ::core::ffi::c_char {
            pos += 2;
            if pos + 2 > input.len() {
                return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
            }
            match big2_byte_type(enc, input, pos) {
                24 | 25 => {}
                _ => return Big2ScanOutcome::Invalid(pos),
            }
            pos += 2;
            while pos + 2 <= input.len() {
                match big2_byte_type(enc, input, pos) {
                    24 | 25 => pos += 2,
                    18 => {
                        return Big2ScanOutcome::Token(
                            crate::src::xmltok::XML_TOK_CHAR_REF_1,
                            pos + 2,
                        )
                    }
                    _ => return Big2ScanOutcome::Invalid(pos),
                }
            }
            return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
        }
        if big2_byte_type(enc, input, pos) != 25 {
            return Big2ScanOutcome::Invalid(pos);
        }
        pos += 2;
        while pos + 2 <= input.len() {
            match big2_byte_type(enc, input, pos) {
                25 => pos += 2,
                18 => {
                    return Big2ScanOutcome::Token(crate::src::xmltok::XML_TOK_CHAR_REF_1, pos + 2)
                }
                _ => return Big2ScanOutcome::Invalid(pos),
            }
        }
        Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1)
    }

    fn big2_scan_ref(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
        mut pos: usize,
    ) -> Big2ScanOutcome {
        if pos + 2 > input.len() {
            return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
        }
        match big2_check_name(enc, input, pos, true) {
            Big2NameCheck::Advance(next) => pos = next,
            Big2NameCheck::PartialChar => {
                return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1)
            }
            Big2NameCheck::Invalid => {
                if big2_byte_type(enc, input, pos) == 19 {
                    return big2_scan_char_ref(enc, input, pos + 2);
                }
                return Big2ScanOutcome::Invalid(pos);
            }
        }
        while pos + 2 <= input.len() {
            match big2_check_name(enc, input, pos, false) {
                Big2NameCheck::Advance(next) => pos = next,
                Big2NameCheck::PartialChar => {
                    return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1)
                }
                Big2NameCheck::Invalid => {
                    if big2_byte_type(enc, input, pos) == 18 {
                        return Big2ScanOutcome::Token(
                            crate::src::xmltok::XML_TOK_ENTITY_REF_1,
                            pos + 2,
                        );
                    }
                    return Big2ScanOutcome::Invalid(pos);
                }
            }
        }
        Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1)
    }

    fn big2_scan_atts_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> Big2ScanOutcome {
        let mut had_colon = false;
        let mut pos = 0;
        'attributes: while pos + 2 <= input.len() {
            match big2_check_name(enc, input, pos, false) {
                Big2NameCheck::Advance(next) => {
                    pos = next;
                    continue;
                }
                Big2NameCheck::PartialChar => {
                    return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1)
                }
                Big2NameCheck::Invalid => {}
            }
            match big2_byte_type(enc, input, pos) {
                23 => {
                    if had_colon {
                        return Big2ScanOutcome::Invalid(pos);
                    }
                    had_colon = true;
                    pos += 2;
                    if pos + 2 > input.len() {
                        return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
                    }
                    match big2_check_name(enc, input, pos, true) {
                        Big2NameCheck::Advance(next) => pos = next,
                        Big2NameCheck::PartialChar => {
                            return Big2ScanOutcome::Partial(
                                crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                            )
                        }
                        Big2NameCheck::Invalid => return Big2ScanOutcome::Invalid(pos),
                    }
                    continue;
                }
                21 | 9 | 10 => loop {
                    pos += 2;
                    if pos + 2 > input.len() {
                        return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
                    }
                    let ty = big2_byte_type(enc, input, pos);
                    if ty == crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int {
                        break;
                    }
                    if !matches!(ty, 21 | 9 | 10) {
                        return Big2ScanOutcome::Invalid(pos);
                    }
                },
                14 => {}
                _ => return Big2ScanOutcome::Invalid(pos),
            }

            had_colon = false;
            let open = loop {
                pos += 2;
                if pos + 2 > input.len() {
                    return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
                }
                let ty = big2_byte_type(enc, input, pos);
                if ty == crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int
                    || ty == crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int
                {
                    break ty;
                }
                if !matches!(ty, 21 | 9 | 10) {
                    return Big2ScanOutcome::Invalid(pos);
                }
            };
            pos += 2;
            loop {
                if pos + 2 > input.len() {
                    return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
                }
                let ty = big2_byte_type(enc, input, pos);
                if ty == open {
                    break;
                }
                match ty {
                    5 => {
                        if input.len() - pos < 2 {
                            return Big2ScanOutcome::Partial(
                                crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                            );
                        } else {
                            return Big2ScanOutcome::Invalid(pos);
                        }
                    }
                    6 => {
                        if input.len() - pos < 3 {
                            return Big2ScanOutcome::Partial(
                                crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                            );
                        } else {
                            return Big2ScanOutcome::Invalid(pos);
                        }
                    }
                    7 => {
                        if input.len() - pos < 4 {
                            return Big2ScanOutcome::Partial(
                                crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                            );
                        } else {
                            return Big2ScanOutcome::Invalid(pos);
                        }
                    }
                    0 | 1 | 8 | 2 => return Big2ScanOutcome::Invalid(pos),
                    3 => match big2_scan_ref(enc, input, pos + 2) {
                        Big2ScanOutcome::Token(_, next) => pos = next,
                        outcome => return outcome,
                    },
                    _ => pos += 2,
                }
            }
            pos += 2;
            if pos + 2 > input.len() {
                return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
            }
            match big2_byte_type(enc, input, pos) {
                21 | 9 | 10 => loop {
                    pos += 2;
                    if pos + 2 > input.len() {
                        return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
                    }
                    match big2_byte_type(enc, input, pos) {
                        21 | 9 | 10 => {}
                        11 => {
                            return Big2ScanOutcome::Token(
                                crate::src::xmltok::XML_TOK_START_TAG_WITH_ATTS_1,
                                pos + 2,
                            )
                        }
                        17 => {
                            pos += 2;
                            if pos + 2 > input.len() {
                                return Big2ScanOutcome::Partial(
                                    crate::src::xmltok::XML_TOK_PARTIAL_1,
                                );
                            }
                            return if input[pos] == 0
                                && input[pos + 1] == b'>' as ::core::ffi::c_char
                            {
                                Big2ScanOutcome::Token(
                                    crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1,
                                    pos + 2,
                                )
                            } else {
                                Big2ScanOutcome::Invalid(pos)
                            };
                        }
                        _ => match big2_check_name(enc, input, pos, true) {
                            Big2NameCheck::Advance(next) => {
                                pos = next;
                                continue 'attributes;
                            }
                            Big2NameCheck::PartialChar => {
                                return Big2ScanOutcome::Partial(
                                    crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                                )
                            }
                            Big2NameCheck::Invalid => return Big2ScanOutcome::Invalid(pos),
                        },
                    }
                },
                11 => {
                    return Big2ScanOutcome::Token(
                        crate::src::xmltok::XML_TOK_START_TAG_WITH_ATTS_1,
                        pos + 2,
                    )
                }
                17 => {
                    pos += 2;
                    if pos + 2 > input.len() {
                        return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
                    }
                    return if input[pos] == 0 && input[pos + 1] == b'>' as ::core::ffi::c_char {
                        Big2ScanOutcome::Token(
                            crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1,
                            pos + 2,
                        )
                    } else {
                        Big2ScanOutcome::Invalid(pos)
                    };
                }
                _ => return Big2ScanOutcome::Invalid(pos),
            }
        }
        Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1)
    }

    pub unsafe extern "C" fn big2_scanAtts(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let len = end.offset_from(ptr) as usize;
        let input = ::core::slice::from_raw_parts(ptr, len);
        let encoding = &*(enc as *const normal_encoding);
        match big2_scan_atts_impl(encoding, input) {
            Big2ScanOutcome::Token(token, next) => {
                *nextTokPtr = ptr.add(next);
                token
            }
            Big2ScanOutcome::Partial(token) => token,
            Big2ScanOutcome::Invalid(at) => {
                *nextTokPtr = ptr.add(at);
                crate::src::xmltok::XML_TOK_INVALID_1
            }
        }
    }

    fn big2_rebase_scan_outcome(outcome: Big2ScanOutcome, base: usize) -> Big2ScanOutcome {
        match outcome {
            Big2ScanOutcome::Token(token, next) => Big2ScanOutcome::Token(token, base + next),
            Big2ScanOutcome::Partial(token) => Big2ScanOutcome::Partial(token),
            Big2ScanOutcome::Invalid(at) => Big2ScanOutcome::Invalid(base + at),
        }
    }

    fn big2_scan_comment_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> Big2ScanOutcome {
        if input.len() < 2 {
            return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
        }
        if input[0] != 0 || input[1] != b'-' as ::core::ffi::c_char {
            return Big2ScanOutcome::Invalid(0);
        }

        let mut pos = 2;
        while pos + 2 <= input.len() {
            match big2_byte_type(enc, input, pos) {
                5 => {
                    if input.len() - pos < 2 {
                        return Big2ScanOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    pos += 2;
                }
                6 => {
                    if input.len() - pos < 3 {
                        return Big2ScanOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    pos += 3;
                }
                7 => {
                    if input.len() - pos < 4 {
                        return Big2ScanOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    pos += 4;
                }
                0 | 1 | 8 => return Big2ScanOutcome::Invalid(pos),
                27 => {
                    pos += 2;
                    if pos + 2 > input.len() {
                        return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
                    }
                    if input[pos] != 0 || input[pos + 1] != b'-' as ::core::ffi::c_char {
                        return Big2ScanOutcome::Invalid(pos);
                    }
                    pos += 2;
                    if pos + 2 > input.len() {
                        return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
                    }
                    return if input[pos] == 0 && input[pos + 1] == b'>' as ::core::ffi::c_char {
                        Big2ScanOutcome::Token(crate::src::xmltok::XML_TOK_COMMENT_1, pos + 2)
                    } else {
                        Big2ScanOutcome::Invalid(pos)
                    };
                }
                _ => pos += 2,
            }
        }
        Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1)
    }

    fn big2_scan_cdata_section_impl(input: &[::core::ffi::c_char]) -> Big2ScanOutcome {
        const CDATA_LSQB: &[u8; 6] = b"CDATA[";
        if input.len() < CDATA_LSQB.len() * 2 {
            return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
        }
        for (index, byte) in CDATA_LSQB.iter().enumerate() {
            let pos = index * 2;
            if input[pos] != 0 || input[pos + 1] != *byte as ::core::ffi::c_char {
                return Big2ScanOutcome::Invalid(pos);
            }
        }
        Big2ScanOutcome::Token(
            crate::src::xmltok::XML_TOK_CDATA_SECT_OPEN_1,
            CDATA_LSQB.len() * 2,
        )
    }

    fn big2_scan_end_tag_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> Big2ScanOutcome {
        if input.len() < 2 {
            return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
        }

        let mut pos = match big2_check_name(enc, input, 0, true) {
            Big2NameCheck::Advance(next) => next,
            Big2NameCheck::PartialChar => {
                return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1)
            }
            Big2NameCheck::Invalid => return Big2ScanOutcome::Invalid(0),
        };

        while pos + 2 <= input.len() {
            match big2_check_name(enc, input, pos, false) {
                Big2NameCheck::Advance(next) => {
                    pos = next;
                    continue;
                }
                Big2NameCheck::PartialChar => {
                    return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1)
                }
                Big2NameCheck::Invalid => {}
            }
            match big2_byte_type(enc, input, pos) {
                23 => pos += 2,
                21 | 9 | 10 => {
                    pos += 2;
                    while pos + 2 <= input.len()
                        && matches!(big2_byte_type(enc, input, pos), 21 | 9 | 10)
                    {
                        pos += 2;
                    }
                    if pos + 2 > input.len() {
                        return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
                    }
                    return if big2_byte_type(enc, input, pos) == 11 {
                        Big2ScanOutcome::Token(crate::src::xmltok::XML_TOK_END_TAG_1, pos + 2)
                    } else {
                        Big2ScanOutcome::Invalid(pos)
                    };
                }
                11 => {
                    return Big2ScanOutcome::Token(crate::src::xmltok::XML_TOK_END_TAG_1, pos + 2)
                }
                _ => return Big2ScanOutcome::Invalid(pos),
            }
        }
        Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1)
    }

    fn big2_scan_lt_impl(enc: &normal_encoding, input: &[::core::ffi::c_char]) -> Big2ScanOutcome {
        if input.len() < 2 {
            return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
        }

        let mut pos = match big2_check_name(enc, input, 0, true) {
            Big2NameCheck::Advance(next) => next,
            Big2NameCheck::PartialChar => {
                return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1)
            }
            Big2NameCheck::Invalid => match big2_byte_type(enc, input, 0) {
                16 => {
                    if input.len() < 4 {
                        return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
                    }
                    match big2_byte_type(enc, input, 2) {
                        27 => {
                            return big2_rebase_scan_outcome(
                                big2_scan_comment_impl(enc, &input[4..]),
                                4,
                            )
                        }
                        20 => {
                            return big2_rebase_scan_outcome(
                                big2_scan_cdata_section_impl(&input[4..]),
                                4,
                            )
                        }
                        _ => return Big2ScanOutcome::Invalid(2),
                    }
                }
                15 => {
                    let (token, next) = big2_scan_pi_impl(enc, &input[2..]);
                    return match next {
                        Some(offset) => Big2ScanOutcome::Token(token, offset + 2),
                        None if token == crate::src::xmltok::XML_TOK_INVALID_1 => {
                            Big2ScanOutcome::Invalid(2)
                        }
                        None => Big2ScanOutcome::Partial(token),
                    };
                }
                17 => return big2_rebase_scan_outcome(big2_scan_end_tag_impl(enc, &input[2..]), 2),
                _ => return Big2ScanOutcome::Invalid(0),
            },
        };
        let mut had_colon = false;

        while pos + 2 <= input.len() {
            match big2_check_name(enc, input, pos, false) {
                Big2NameCheck::Advance(next) => {
                    pos = next;
                    continue;
                }
                Big2NameCheck::PartialChar => {
                    return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1)
                }
                Big2NameCheck::Invalid => {}
            }

            match big2_byte_type(enc, input, pos) {
                23 => {
                    if had_colon {
                        return Big2ScanOutcome::Invalid(pos);
                    }
                    had_colon = true;
                    pos += 2;
                    if pos + 2 > input.len() {
                        return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
                    }
                    pos = match big2_check_name(enc, input, pos, true) {
                        Big2NameCheck::Advance(next) => next,
                        Big2NameCheck::PartialChar => {
                            return Big2ScanOutcome::Partial(
                                crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                            )
                        }
                        Big2NameCheck::Invalid => return Big2ScanOutcome::Invalid(pos),
                    };
                }
                21 | 9 | 10 => {
                    pos += 2;
                    while pos + 2 <= input.len()
                        && matches!(big2_byte_type(enc, input, pos), 21 | 9 | 10)
                    {
                        pos += 2;
                    }
                    if pos + 2 > input.len() {
                        return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
                    }
                    return big2_rebase_scan_outcome(big2_scan_atts_impl(enc, &input[pos..]), pos);
                }
                11 => {
                    return Big2ScanOutcome::Token(
                        crate::src::xmltok::XML_TOK_START_TAG_NO_ATTS_1,
                        pos + 2,
                    )
                }
                17 => {
                    pos += 2;
                    if pos + 2 > input.len() {
                        return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
                    }
                    return if input[pos] == 0 && input[pos + 1] == b'>' as ::core::ffi::c_char {
                        Big2ScanOutcome::Token(
                            crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_NO_ATTS_1,
                            pos + 2,
                        )
                    } else {
                        Big2ScanOutcome::Invalid(pos)
                    };
                }
                _ => return Big2ScanOutcome::Invalid(pos),
            }
        }
        Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1)
    }

    pub unsafe extern "C" fn big2_scanLt(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let len = end.offset_from(ptr);
        if len <= 0 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = ::core::slice::from_raw_parts(ptr, len as usize);
        let encoding = &*(enc as *const normal_encoding);
        match big2_scan_lt_impl(encoding, input) {
            Big2ScanOutcome::Token(token, next) => {
                *nextTokPtr = ptr.add(next);
                token
            }
            Big2ScanOutcome::Partial(token) => token,
            Big2ScanOutcome::Invalid(at) => {
                *nextTokPtr = ptr.add(at);
                crate::src::xmltok::XML_TOK_INVALID_1
            }
        }
    }

    enum Big2ContentToken {
        Result(::core::ffi::c_int, Option<usize>),
        ScanLt,
        ScanRef,
    }

    fn big2_char_type(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
        at: usize,
    ) -> ::core::ffi::c_int {
        if input[at] == 0 {
            enc.type_0[input[at + 1] as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
        } else {
            unicode_byte_type(input[at], input[at + 1])
        }
    }

    fn big2_content_tok_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> Big2ContentToken {
        if input.is_empty() {
            return Big2ContentToken::Result(crate::src::xmltok::XML_TOK_NONE_1, None);
        }
        let end = input.len() & !1;
        if end == 0 {
            return Big2ContentToken::Result(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }
        let mut ptr = 0;
        match big2_char_type(enc, input, ptr) {
            2 => return Big2ContentToken::ScanLt,
            3 => return Big2ContentToken::ScanRef,
            9 => {
                ptr += 2;
                if end - ptr < 2 {
                    return Big2ContentToken::Result(
                        crate::src::xmltok::XML_TOK_TRAILING_CR_1,
                        None,
                    );
                }
                if big2_char_type(enc, input, ptr)
                    == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                {
                    ptr += 2;
                }
                return Big2ContentToken::Result(
                    crate::src::xmltok::XML_TOK_DATA_NEWLINE_1,
                    Some(ptr),
                );
            }
            10 => {
                return Big2ContentToken::Result(
                    crate::src::xmltok::XML_TOK_DATA_NEWLINE_1,
                    Some(2),
                )
            }
            4 => {
                ptr += 2;
                if end - ptr < 2 {
                    return Big2ContentToken::Result(
                        crate::src::xmltok::XML_TOK_TRAILING_RSQB_1,
                        None,
                    );
                }
                if input[ptr] == 0 && input[ptr + 1] == 0x5d {
                    ptr += 2;
                    if end - ptr < 2 {
                        return Big2ContentToken::Result(
                            crate::src::xmltok::XML_TOK_TRAILING_RSQB_1,
                            None,
                        );
                    }
                    if input[ptr] == 0 && input[ptr + 1] == 0x3e {
                        return Big2ContentToken::Result(
                            crate::src::xmltok::XML_TOK_INVALID_1,
                            Some(ptr),
                        );
                    }
                    ptr -= 2;
                }
            }
            5 => ptr += 2,
            6 => {
                if end < 3 {
                    return Big2ContentToken::Result(
                        crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        None,
                    );
                }
                ptr += 3;
            }
            7 => {
                if end < 4 {
                    return Big2ContentToken::Result(
                        crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        None,
                    );
                }
                ptr += 4;
            }
            0 | 1 | 8 => {
                return Big2ContentToken::Result(crate::src::xmltok::XML_TOK_INVALID_1, Some(0))
            }
            _ => ptr += 2,
        }
        while end - ptr >= 2 {
            match big2_char_type(enc, input, ptr) {
                5 => {
                    ptr += 2;
                    break;
                }
                6 => {
                    if end - ptr < 3 {
                        return Big2ContentToken::Result(
                            crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                            Some(ptr),
                        );
                    }
                    ptr += 3;
                    break;
                }
                7 => {
                    if end - ptr < 4 {
                        return Big2ContentToken::Result(
                            crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                            Some(ptr),
                        );
                    }
                    ptr += 4;
                    break;
                }
                4 if end - ptr >= 4 => {
                    if input[ptr + 2] != 0 || input[ptr + 3] != 0x5d {
                        ptr += 2;
                        break;
                    }
                    if end - ptr >= 6 {
                        if input[ptr + 4] != 0 || input[ptr + 5] != 0x3e {
                            ptr += 2;
                            break;
                        }
                        return Big2ContentToken::Result(
                            crate::src::xmltok::XML_TOK_INVALID_1,
                            Some(ptr + 4),
                        );
                    }
                }
                3 | 2 | 0 | 1 | 8 | 9 | 10 | 4 => {}
                _ => {
                    ptr += 2;
                    break;
                }
            }
            return Big2ContentToken::Result(crate::src::xmltok::XML_TOK_DATA_CHARS_1, Some(ptr));
        }
        Big2ContentToken::Result(crate::src::xmltok::XML_TOK_DATA_CHARS_1, Some(ptr))
    }

    pub unsafe extern "C" fn big2_contentTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if ptr >= end {
            return crate::src::xmltok::XML_TOK_NONE_1;
        }
        let byte_len = end.offset_from(ptr) as usize;
        let input = ::core::slice::from_raw_parts(ptr, byte_len);
        let end = ptr.add(byte_len & !1);
        let encoding = &*(enc as *const normal_encoding);
        match big2_content_tok_impl(encoding, input) {
            Big2ContentToken::ScanLt => big2_scanLt(enc, ptr.add(2), end, nextTokPtr),
            Big2ContentToken::ScanRef => big2_scanRef(enc, ptr.add(2), end, nextTokPtr),
            Big2ContentToken::Result(token, Some(next)) => {
                *nextTokPtr = ptr.add(next);
                token
            }
            Big2ContentToken::Result(token, None) => token,
        }
    }

    fn big2_scan_percent_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> Big2ScanOutcome {
        if input.len() < 2 {
            return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
        }

        let mut offset = match big2_byte_type(enc, input, 0) {
            29 if !big2_name_char_is_valid(input, 0, &nmstrtPages) => {
                return Big2ScanOutcome::Invalid(0)
            }
            29 | 22 | 24 => 2,
            kind @ (5 | 6 | 7) => {
                if input.len() < kind as usize - 3 {
                    return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1);
                }
                return Big2ScanOutcome::Invalid(0);
            }
            21 | 10 | 9 | 30 => {
                return Big2ScanOutcome::Token(crate::src::xmltok::XML_TOK_PERCENT_1, 0)
            }
            _ => return Big2ScanOutcome::Invalid(0),
        };

        while offset + 2 <= input.len() {
            match big2_byte_type(enc, input, offset) {
                29 if !big2_name_char_is_valid(input, offset, &namePages) => {
                    return Big2ScanOutcome::Invalid(offset)
                }
                29 | 22 | 24 | 25 | 26 | 27 => offset += 2,
                kind @ (5 | 6 | 7) => {
                    if input.len() - offset < kind as usize - 3 {
                        return Big2ScanOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    return Big2ScanOutcome::Invalid(offset);
                }
                18 => {
                    return Big2ScanOutcome::Token(
                        crate::src::xmltok::XML_TOK_PARAM_ENTITY_REF_1,
                        offset + 2,
                    )
                }
                _ => return Big2ScanOutcome::Invalid(offset),
            }
        }
        Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1)
    }

    pub unsafe extern "C" fn big2_scanPercent(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let len = end.offset_from(ptr);
        if len < 2 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let len = len as usize;
        let input = ::core::slice::from_raw_parts(ptr, len);
        let encoding = &*(enc as *const normal_encoding);
        match big2_scan_percent_impl(encoding, input) {
            Big2ScanOutcome::Token(token, next) => {
                *nextTokPtr = ptr.add(next);
                token
            }
            Big2ScanOutcome::Partial(token) => token,
            Big2ScanOutcome::Invalid(at) => {
                *nextTokPtr = ptr.add(at);
                crate::src::xmltok::XML_TOK_INVALID_1
            }
        }
    }

    fn big2_scan_pound_name_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> Big2ScanOutcome {
        if input.len() < 2 {
            return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
        }

        let mut offset = match big2_check_name(enc, input, 0, true) {
            Big2NameCheck::Advance(next) => next,
            Big2NameCheck::PartialChar => {
                return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1)
            }
            Big2NameCheck::Invalid => return Big2ScanOutcome::Invalid(0),
        };

        while offset + 2 <= input.len() {
            match big2_byte_type(enc, input, offset) {
                9 | 10 | 11 | 21 | 30 | 32 | 36 => {
                    return Big2ScanOutcome::Token(
                        crate::src::xmltok::XML_TOK_POUND_NAME_1,
                        offset,
                    )
                }
                _ => match big2_check_name(enc, input, offset, false) {
                    Big2NameCheck::Advance(next) => offset = next,
                    Big2NameCheck::PartialChar => {
                        return Big2ScanOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        )
                    }
                    Big2NameCheck::Invalid => return Big2ScanOutcome::Invalid(offset),
                },
            }
        }

        Big2ScanOutcome::Partial(-crate::src::xmltok::XML_TOK_POUND_NAME_1)
    }

    pub unsafe extern "C" fn big2_scanPoundName(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = end.offset_from(ptr);
        if input_len < 2 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = ::core::slice::from_raw_parts(ptr, input_len as usize);
        let normal = &*(enc as *const normal_encoding);
        match big2_scan_pound_name_impl(normal, input) {
            Big2ScanOutcome::Token(token, next) => {
                *nextTokPtr = ptr.add(next);
                token
            }
            Big2ScanOutcome::Partial(token) => token,
            Big2ScanOutcome::Invalid(at) => {
                *nextTokPtr = ptr.add(at);
                crate::src::xmltok::XML_TOK_INVALID_1
            }
        }
    }

    pub unsafe extern "C" fn big2_scanLit(
        mut open: ::core::ffi::c_int,
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        while end.offset_from(ptr) >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize {
            let mut t: ::core::ffi::c_int = if *ptr.offset(0 as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0 as isize), *ptr.offset(1 as isize))
            };
            match t {
                5 => {
                    if end.offset_from(ptr) < 2 as isize {
                        return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                6 => {
                    if end.offset_from(ptr) < 3 as isize {
                        return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                }
                7 => {
                    if end.offset_from(ptr) < 4 as isize {
                        return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
                12 | 13 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    if t == open {
                        if !(end.offset_from(ptr)
                            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                        {
                            return -crate::src::xmltok::XML_TOK_LITERAL_1;
                        }
                        *nextTokPtr = ptr;
                        match if *ptr.offset(0 as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(0 as isize), *ptr.offset(1 as isize))
                        } {
                            21 | 9 | 10 | 11 | 30 | 20 => {
                                return crate::src::xmltok::XML_TOK_LITERAL_1
                            }
                            _ => return crate::src::xmltok::XML_TOK_INVALID_1,
                        }
                    }
                }
                _ => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
            }
        }
        return crate::src::xmltok::XML_TOK_PARTIAL_1;
    }

    enum Big2PrologToken {
        Result(::core::ffi::c_int, Option<usize>),
        ScanLit(::core::ffi::c_int, usize),
        ScanDecl(usize),
        ScanPi(usize),
        ScanPercent(usize),
        ScanPoundName(usize),
    }

    fn big2_prolog_tok_impl(
        enc: &normal_encoding,
        raw_input: &[::core::ffi::c_char],
    ) -> Big2PrologToken {
        if raw_input.is_empty() {
            return Big2PrologToken::Result(crate::src::xmltok::XML_TOK_NONE_1, None);
        }

        let input = &raw_input[..raw_input.len() & !1];
        if input.is_empty() {
            return Big2PrologToken::Result(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }

        let token_at = |offset| big2_byte_type(enc, input, offset);
        let invalid =
            |offset| Big2PrologToken::Result(crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));

        let mut offset = 0;
        let mut token;
        match token_at(offset) {
            12 => {
                return Big2PrologToken::ScanLit(
                    crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int,
                    2,
                );
            }
            13 => {
                return Big2PrologToken::ScanLit(
                    crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int,
                    2,
                );
            }
            2 => {
                offset = 2;
                if input.len() - offset < 2 {
                    return Big2PrologToken::Result(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                }
                match token_at(offset) {
                    16 => return Big2PrologToken::ScanDecl(4),
                    15 => return Big2PrologToken::ScanPi(4),
                    22 | 24 | 29 | 5 | 6 | 7 => {
                        return Big2PrologToken::Result(
                            crate::src::xmltok::XML_TOK_INSTANCE_START,
                            Some(0),
                        );
                    }
                    _ => return invalid(offset),
                }
            }
            9 => {
                if input.len() == 2 {
                    return Big2PrologToken::Result(
                        -crate::src::xmltok::XML_TOK_PROLOG_S_1,
                        Some(2),
                    );
                }
                offset = 2;
                loop {
                    if input.len() - offset < 2 {
                        break;
                    }
                    match token_at(offset) {
                        21 | 10 | 9 if offset + 2 != input.len() => offset += 2,
                        21 | 10 => offset += 2,
                        _ => {
                            return Big2PrologToken::Result(
                                crate::src::xmltok::XML_TOK_PROLOG_S_1,
                                Some(offset),
                            );
                        }
                    }
                }
                return Big2PrologToken::Result(
                    crate::src::xmltok::XML_TOK_PROLOG_S_1,
                    Some(offset),
                );
            }
            21 | 10 => {
                offset = 2;
                loop {
                    if input.len() - offset < 2 {
                        break;
                    }
                    match token_at(offset) {
                        21 | 10 | 9 if offset + 2 != input.len() => offset += 2,
                        21 | 10 => offset += 2,
                        _ => {
                            return Big2PrologToken::Result(
                                crate::src::xmltok::XML_TOK_PROLOG_S_1,
                                Some(offset),
                            );
                        }
                    }
                }
                return Big2PrologToken::Result(
                    crate::src::xmltok::XML_TOK_PROLOG_S_1,
                    Some(offset),
                );
            }
            30 => return Big2PrologToken::ScanPercent(2),
            35 => {
                return Big2PrologToken::Result(crate::src::xmltok::XML_TOK_COMMA_1, Some(2));
            }
            20 => {
                return Big2PrologToken::Result(
                    crate::src::xmltok::XML_TOK_OPEN_BRACKET_1,
                    Some(2),
                );
            }
            4 => {
                offset = 2;
                if input.len() - offset < 2 {
                    return Big2PrologToken::Result(
                        -crate::src::xmltok::XML_TOK_CLOSE_BRACKET_1,
                        None,
                    );
                }
                if input[offset] == 0 && input[offset + 1] as u8 == b']' {
                    if input.len() - offset < 4 {
                        return Big2PrologToken::Result(
                            crate::src::xmltok::XML_TOK_PARTIAL_1,
                            None,
                        );
                    }
                    if input[offset + 2] == 0 && input[offset + 3] as u8 == b'>' {
                        return Big2PrologToken::Result(
                            crate::src::xmltok::XML_TOK_COND_SECT_CLOSE_1,
                            Some(offset + 4),
                        );
                    }
                }
                return Big2PrologToken::Result(
                    crate::src::xmltok::XML_TOK_CLOSE_BRACKET_1,
                    Some(offset),
                );
            }
            31 => {
                return Big2PrologToken::Result(crate::src::xmltok::XML_TOK_OPEN_PAREN_1, Some(2));
            }
            32 => {
                offset = 2;
                if input.len() - offset < 2 {
                    return Big2PrologToken::Result(
                        -crate::src::xmltok::XML_TOK_CLOSE_PAREN_1,
                        None,
                    );
                }
                return match token_at(offset) {
                    33 => Big2PrologToken::Result(
                        crate::src::xmltok::XML_TOK_CLOSE_PAREN_ASTERISK_1,
                        Some(4),
                    ),
                    15 => Big2PrologToken::Result(
                        crate::src::xmltok::XML_TOK_CLOSE_PAREN_QUESTION_1,
                        Some(4),
                    ),
                    34 => Big2PrologToken::Result(
                        crate::src::xmltok::XML_TOK_CLOSE_PAREN_PLUS_1,
                        Some(4),
                    ),
                    9 | 10 | 21 | 11 | 35 | 36 | 32 => {
                        Big2PrologToken::Result(crate::src::xmltok::XML_TOK_CLOSE_PAREN_1, Some(2))
                    }
                    _ => invalid(offset),
                };
            }
            36 => {
                return Big2PrologToken::Result(crate::src::xmltok::XML_TOK_OR_1, Some(2));
            }
            11 => {
                return Big2PrologToken::Result(crate::src::xmltok::XML_TOK_DECL_CLOSE_1, Some(2));
            }
            19 => return Big2PrologToken::ScanPoundName(2),
            5 | 6 | 7 => return invalid(0),
            22 | 24 => {
                token = crate::src::xmltok::XML_TOK_NAME;
                offset = 2;
            }
            25 | 26 | 27 | 23 => {
                token = crate::src::xmltok::XML_TOK_NMTOKEN_1;
                offset = 2;
            }
            29 => {
                if big2_name_char_is_valid(input, 0, &nmstrtPages) {
                    token = crate::src::xmltok::XML_TOK_NAME;
                    offset = 2;
                } else if big2_name_char_is_valid(input, 0, &namePages) {
                    token = crate::src::xmltok::XML_TOK_NMTOKEN_1;
                    offset = 2;
                } else {
                    return invalid(0);
                }
            }
            _ => return invalid(0),
        }

        while input.len() - offset >= 2 {
            match token_at(offset) {
                29 => {
                    if !big2_name_char_is_valid(input, offset, &namePages) {
                        return invalid(offset);
                    }
                }
                22 | 24 | 25 | 26 | 27 => {}
                5 | 6 | 7 => return invalid(offset),
                11 | 32 | 35 | 36 | 20 | 30 | 21 | 9 | 10 => {
                    return Big2PrologToken::Result(token, Some(offset));
                }
                23 => {
                    offset += 2;
                    match token {
                        crate::src::xmltok::XML_TOK_NAME => {
                            if input.len() - offset < 2 {
                                return Big2PrologToken::Result(
                                    crate::src::xmltok::XML_TOK_PARTIAL_1,
                                    None,
                                );
                            }
                            token = crate::src::xmltok::XML_TOK_PREFIXED_NAME;
                            match token_at(offset) {
                                29 if !big2_name_char_is_valid(input, offset, &namePages) => {
                                    return invalid(offset);
                                }
                                29 | 22 | 24 | 25 | 26 | 27 => offset += 2,
                                5 | 6 | 7 => return invalid(offset),
                                _ => token = crate::src::xmltok::XML_TOK_NMTOKEN_1,
                            }
                        }
                        crate::src::xmltok::XML_TOK_PREFIXED_NAME => {
                            token = crate::src::xmltok::XML_TOK_NMTOKEN_1;
                        }
                        _ => {}
                    }
                    // The colon itself has been consumed.  A valid first
                    // local-name character is consumed by the nested match;
                    // otherwise the next loop iteration must examine it as
                    // an NMTOKEN character.  In both cases, do not publish a
                    // next-token position for the partial-token result above.
                    continue;
                }
                34 => {
                    if token == crate::src::xmltok::XML_TOK_NMTOKEN_1 {
                        return invalid(offset);
                    }
                    return Big2PrologToken::Result(
                        crate::src::xmltok::XML_TOK_NAME_PLUS_1,
                        Some(offset + 2),
                    );
                }
                33 => {
                    if token == crate::src::xmltok::XML_TOK_NMTOKEN_1 {
                        return invalid(offset);
                    }
                    return Big2PrologToken::Result(
                        crate::src::xmltok::XML_TOK_NAME_ASTERISK_1,
                        Some(offset + 2),
                    );
                }
                15 => {
                    if token == crate::src::xmltok::XML_TOK_NMTOKEN_1 {
                        return invalid(offset);
                    }
                    return Big2PrologToken::Result(
                        crate::src::xmltok::XML_TOK_NAME_QUESTION_1,
                        Some(offset + 2),
                    );
                }
                _ => return invalid(offset),
            }
            offset += 2;
        }

        Big2PrologToken::Result(-token, None)
    }

    pub unsafe extern "C" fn big2_prologTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if ptr >= end {
            return crate::src::xmltok::XML_TOK_NONE_1;
        }
        let input_len = end.offset_from(ptr) as usize;
        let input = ::core::slice::from_raw_parts(ptr, input_len);
        let input = &input[..input.len() & !1];
        let enc_ref = &*(enc as *const normal_encoding);
        match big2_prolog_tok_impl(enc_ref, input) {
            Big2PrologToken::Result(token, next) => {
                if let Some(offset) = next {
                    *nextTokPtr = ptr.add(offset);
                }
                token
            }
            Big2PrologToken::ScanLit(open, offset) => {
                big2_scanLit(open, enc, ptr.add(offset), ptr.add(input.len()), nextTokPtr)
            }
            Big2PrologToken::ScanDecl(offset) => {
                big2_scanDecl(enc, ptr.add(offset), ptr.add(input.len()), nextTokPtr)
            }
            Big2PrologToken::ScanPi(offset) => {
                big2_scanPi(enc, ptr.add(offset), ptr.add(input.len()), nextTokPtr)
            }
            Big2PrologToken::ScanPercent(offset) => {
                big2_scanPercent(enc, ptr.add(offset), ptr.add(input.len()), nextTokPtr)
            }
            Big2PrologToken::ScanPoundName(offset) => {
                big2_scanPoundName(enc, ptr.add(offset), ptr.add(input.len()), nextTokPtr)
            }
        }
    }

    struct Big2AttributeValueToken {
        token: ::core::ffi::c_int,
        next: Option<usize>,
    }

    fn big2_attribute_value_tok_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> Big2AttributeValueToken {
        if input.is_empty() {
            return Big2AttributeValueToken {
                token: crate::src::xmltok::XML_TOK_NONE_1,
                next: None,
            };
        }
        if input.len() < 2 {
            return Big2AttributeValueToken {
                token: crate::src::xmltok::XML_TOK_PARTIAL_1,
                next: None,
            };
        }

        let mut pos = 0;
        while pos + 2 <= input.len() {
            match big2_byte_type(enc, input, pos) {
                5 => pos += 2,
                6 => {
                    if input.len() - pos < 3 {
                        return Big2AttributeValueToken {
                            token: crate::src::xmltok::XML_TOK_PARTIAL_1,
                            next: None,
                        };
                    }
                    pos += 3;
                }
                7 => {
                    if input.len() - pos < 4 {
                        return Big2AttributeValueToken {
                            token: crate::src::xmltok::XML_TOK_PARTIAL_1,
                            next: None,
                        };
                    }
                    pos += 4;
                }
                3 if pos == 0 => {
                    return match big2_scan_ref(enc, input, 2) {
                        Big2ScanOutcome::Token(token, next) => Big2AttributeValueToken {
                            token,
                            next: Some(next),
                        },
                        Big2ScanOutcome::Partial(token) => Big2AttributeValueToken {
                            token,
                            next: None,
                        },
                        Big2ScanOutcome::Invalid(next) => Big2AttributeValueToken {
                            token: crate::src::xmltok::XML_TOK_INVALID_1,
                            next: Some(next),
                        },
                    };
                }
                3 => {
                    return Big2AttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(pos),
                    };
                }
                2 => {
                    return Big2AttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_INVALID_1,
                        next: Some(pos),
                    };
                }
                10 if pos == 0 => {
                    return Big2AttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_NEWLINE_1,
                        next: Some(2),
                    };
                }
                10 => {
                    return Big2AttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(pos),
                    };
                }
                9 if pos == 0 => {
                    pos += 2;
                    if pos + 2 > input.len() {
                        return Big2AttributeValueToken {
                            token: crate::src::xmltok::XML_TOK_TRAILING_CR_1,
                            next: None,
                        };
                    }
                    if big2_byte_type(enc, input, pos)
                        == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                    {
                        pos += 2;
                    }
                    return Big2AttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_NEWLINE_1,
                        next: Some(pos),
                    };
                }
                9 => {
                    return Big2AttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(pos),
                    };
                }
                21 if pos == 0 => {
                    return Big2AttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_ATTRIBUTE_VALUE_S_1,
                        next: Some(2),
                    };
                }
                21 => {
                    return Big2AttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(pos),
                    };
                }
                _ => pos += 2,
            }
        }

        Big2AttributeValueToken {
            token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
            next: Some(pos),
        }
    }

    pub unsafe extern "C" fn big2_attributeValueTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = end.offset_from(ptr);
        if input_len <= 0 {
            return crate::src::xmltok::XML_TOK_NONE_1;
        }
        let input = ::core::slice::from_raw_parts(ptr, input_len as usize);
        let normal = &*(enc as *const normal_encoding);
        let result = big2_attribute_value_tok_impl(normal, input);
        if let Some(offset) = result.next {
            *nextTokPtr = ptr.add(offset);
        }
        result.token
    }

    struct Big2EntityValueToken {
        token: ::core::ffi::c_int,
        next: Option<usize>,
    }

    fn big2_entity_value_tok_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> Big2EntityValueToken {
        if input.is_empty() {
            return Big2EntityValueToken {
                token: crate::src::xmltok::XML_TOK_NONE_1,
                next: None,
            };
        }
        if input.len() < 2 {
            return Big2EntityValueToken {
                token: crate::src::xmltok::XML_TOK_PARTIAL_1,
                next: None,
            };
        }

        let mut pos = 0;
        while pos + 2 <= input.len() {
            match big2_byte_type(enc, input, pos) {
                5 => pos += 2,
                6 => {
                    if input.len() - pos < 3 {
                        return Big2EntityValueToken {
                            token: crate::src::xmltok::XML_TOK_PARTIAL_1,
                            next: None,
                        };
                    }
                    pos += 3;
                }
                7 => {
                    if input.len() - pos < 4 {
                        return Big2EntityValueToken {
                            token: crate::src::xmltok::XML_TOK_PARTIAL_1,
                            next: None,
                        };
                    }
                    pos += 4;
                }
                3 if pos == 0 => {
                    return match big2_scan_ref(enc, input, 2) {
                        Big2ScanOutcome::Token(token, next) => Big2EntityValueToken {
                            token,
                            next: Some(next),
                        },
                        Big2ScanOutcome::Partial(token) => Big2EntityValueToken {
                            token,
                            next: None,
                        },
                        Big2ScanOutcome::Invalid(next) => Big2EntityValueToken {
                            token: crate::src::xmltok::XML_TOK_INVALID_1,
                            next: Some(next),
                        },
                    };
                }
                3 => {
                    return Big2EntityValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(pos),
                    };
                }
                30 if pos == 0 => {
                    return match big2_scan_percent_impl(enc, &input[2..]) {
                        Big2ScanOutcome::Token(token, next) => Big2EntityValueToken {
                            token: if token == crate::src::xmltok::XML_TOK_PERCENT_1 {
                                crate::src::xmltok::XML_TOK_INVALID_1
                            } else {
                                token
                            },
                            next: Some(next + 2),
                        },
                        Big2ScanOutcome::Partial(token) => Big2EntityValueToken {
                            token,
                            next: None,
                        },
                        Big2ScanOutcome::Invalid(next) => Big2EntityValueToken {
                            token: crate::src::xmltok::XML_TOK_INVALID_1,
                            next: Some(next + 2),
                        },
                    };
                }
                30 => {
                    return Big2EntityValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(pos),
                    };
                }
                10 if pos == 0 => {
                    return Big2EntityValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_NEWLINE_1,
                        next: Some(2),
                    };
                }
                10 => {
                    return Big2EntityValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(pos),
                    };
                }
                9 if pos == 0 => {
                    pos += 2;
                    if pos + 2 > input.len() {
                        return Big2EntityValueToken {
                            token: crate::src::xmltok::XML_TOK_TRAILING_CR_1,
                            next: None,
                        };
                    }
                    if big2_byte_type(enc, input, pos)
                        == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                    {
                        pos += 2;
                    }
                    return Big2EntityValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_NEWLINE_1,
                        next: Some(pos),
                    };
                }
                9 => {
                    return Big2EntityValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(pos),
                    };
                }
                _ => pos += 2,
            }
        }

        Big2EntityValueToken {
            token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
            next: Some(pos),
        }
    }

    pub unsafe extern "C" fn big2_entityValueTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = end.offset_from(ptr);
        if input_len <= 0 {
            return crate::src::xmltok::XML_TOK_NONE_1;
        }
        let input = ::core::slice::from_raw_parts(ptr, input_len as usize);
        let normal = &*(enc as *const normal_encoding);
        let result = big2_entity_value_tok_impl(normal, input);
        if let Some(offset) = result.next {
            *nextTokPtr = ptr.add(offset);
        }
        result.token
    }

    fn big2_ignore_section_tok_impl(
        enc: &normal_encoding,
        raw_input: &[::core::ffi::c_char],
    ) -> Big2ScanOutcome {
        let input = &raw_input[..raw_input.len() & !1];
        let mut level: ::core::ffi::c_int = 0;
        let mut pos = 0;

        while input.len() - pos >= 2 {
            match big2_byte_type(enc, input, pos) {
                5 => {
                    if input.len() - pos < 2 {
                        return Big2ScanOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    pos += 2;
                }
                6 => {
                    if input.len() - pos < 3 {
                        return Big2ScanOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    pos += 3;
                }
                7 => {
                    if input.len() - pos < 4 {
                        return Big2ScanOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    pos += 4;
                }
                0 | 1 | 8 => return Big2ScanOutcome::Invalid(pos),
                2 => {
                    pos += 2;
                    if input.len() - pos < 2 {
                        return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
                    }
                    if input[pos] == 0 && input[pos + 1] == b'!' as ::core::ffi::c_char {
                        pos += 2;
                        if input.len() - pos < 2 {
                            return Big2ScanOutcome::Partial(
                                crate::src::xmltok::XML_TOK_PARTIAL_1,
                            );
                        }
                        if input[pos] == 0 && input[pos + 1] == b'[' as ::core::ffi::c_char {
                            level += 1;
                            pos += 2;
                        }
                    }
                }
                4 => {
                    pos += 2;
                    if input.len() - pos < 2 {
                        return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
                    }
                    if input[pos] == 0 && input[pos + 1] == b']' as ::core::ffi::c_char {
                        pos += 2;
                        if input.len() - pos < 2 {
                            return Big2ScanOutcome::Partial(
                                crate::src::xmltok::XML_TOK_PARTIAL_1,
                            );
                        }
                        if input[pos] == 0 && input[pos + 1] == b'>' as ::core::ffi::c_char {
                            pos += 2;
                            if level == 0 {
                                return Big2ScanOutcome::Token(
                                    crate::src::xmltok::XML_TOK_IGNORE_SECT_1,
                                    pos,
                                );
                            }
                            level -= 1;
                        }
                    }
                }
                _ => pos += 2,
            }
        }
        Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1)
    }

    pub unsafe extern "C" fn big2_ignoreSectionTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = end.offset_from(ptr);
        if input_len <= 0 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = ::core::slice::from_raw_parts(ptr, input_len as usize);
        let encoding = &*(enc as *const normal_encoding);
        match big2_ignore_section_tok_impl(encoding, input) {
            Big2ScanOutcome::Token(token, next) => {
                *nextTokPtr = ptr.wrapping_add(next);
                token
            }
            Big2ScanOutcome::Partial(token) => token,
            Big2ScanOutcome::Invalid(at) => {
                *nextTokPtr = ptr.wrapping_add(at);
                crate::src::xmltok::XML_TOK_INVALID_1
            }
        }
    }

    pub unsafe extern "C" fn big2_isPublicId(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        badPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let span_len = end.offset_from(ptr);
        if span_len < 4 {
            return 1;
        }
        let contents =
            ::core::slice::from_raw_parts(ptr.add(2).cast::<u8>(), (span_len - 4) as usize);
        let byte_types = &(*(enc as *const normal_encoding)).type_0;
        if let Some(offset) = crate::src::xmltok::public_id_bad_offset(
            contents,
            byte_types,
            crate::src::xmltok::PublicIdChecker::Big2,
        ) {
            *badPtr = ptr.add(2 + offset);
            return 0;
        }
        1
    }

    #[derive(Copy, Clone)]
    pub enum Big2AttributeAction {
        Name {
            attribute: ::core::ffi::c_int,
            offset: usize,
        },
        ValueStart {
            attribute: ::core::ffi::c_int,
            offset: usize,
        },
        ValueEnd {
            attribute: ::core::ffi::c_int,
            offset: usize,
        },
        Normalized {
            attribute: ::core::ffi::c_int,
            value: ::core::ffi::c_char,
        },
    }

    /// Scans a complete big-endian UTF-16 start-tag token with slice indices,
    /// reporting only safe offsets to its boundary adapter.
    fn scan_big2_atts(
        byte_types: &[::core::ffi::c_uchar; 256],
        source: &[u8],
        mut report: impl FnMut(Big2AttributeAction),
    ) -> ::core::ffi::c_int {
        #[derive(Copy, Clone, Eq, PartialEq)]
        enum State {
            InName,
            InValue,
            Other,
        }

        fn byte_type(
            byte_types: &[::core::ffi::c_uchar; 256],
            source: &[u8],
            index: usize,
        ) -> Option<::core::ffi::c_int> {
            let hi = *source.get(index)?;
            let lo = *source.get(index + 1)?;
            Some(if hi == 0 {
                byte_types[lo as usize] as ::core::ffi::c_int
            } else {
                unicode_byte_type(hi as ::core::ffi::c_char, lo as ::core::ffi::c_char)
            })
        }

        let mut state = State::InName;
        let mut n_atts: ::core::ffi::c_int = 0;
        let mut open = 0;
        let mut value_start = None;
        let mut normalized = true;
        let mut index = 2;

        while let Some(kind) = byte_type(byte_types, source, index) {
            match kind {
                5 | 6 | 7 | 29 | 22 | 24 => {
                    if state == State::Other {
                        report(Big2AttributeAction::Name {
                            attribute: n_atts,
                            offset: index,
                        });
                        normalized = true;
                        report(Big2AttributeAction::Normalized {
                            attribute: n_atts,
                            value: 1,
                        });
                        state = State::InName;
                    }
                    index += match kind {
                        5 => 2,
                        6 => 3,
                        7 => 4,
                        _ => 2,
                    };
                    continue;
                }
                12 | 13 => {
                    let quote_kind = if kind == 12 {
                        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int
                    } else {
                        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int
                    };
                    if state != State::InValue {
                        report(Big2AttributeAction::ValueStart {
                            attribute: n_atts,
                            offset: index + 2,
                        });
                        value_start = Some(index + 2);
                        state = State::InValue;
                        open = quote_kind;
                    } else if open == quote_kind {
                        state = State::Other;
                        report(Big2AttributeAction::ValueEnd {
                            attribute: n_atts,
                            offset: index,
                        });
                        n_atts += 1;
                        value_start = None;
                    }
                }
                3 => {
                    normalized = false;
                    report(Big2AttributeAction::Normalized {
                        attribute: n_atts,
                        value: 0,
                    });
                }
                21 => {
                    if state == State::InName {
                        state = State::Other;
                    } else if state == State::InValue && normalized {
                        let current_char = if source[index] == 0 {
                            source[index + 1] as ::core::ffi::c_int
                        } else {
                            -1
                        };
                        let next_char = source
                            .get(index + 3)
                            .copied()
                            .filter(|_| source.get(index + 2) == Some(&0))
                            .map(::core::ffi::c_int::from)
                            .unwrap_or(-1);
                        if value_start == Some(index)
                            || current_char != crate::ascii_h::ASCII_SPACE
                            || next_char == crate::ascii_h::ASCII_SPACE
                            || byte_type(byte_types, source, index + 2) == Some(open)
                        {
                            normalized = false;
                            report(Big2AttributeAction::Normalized {
                                attribute: n_atts,
                                value: 0,
                            });
                        }
                    }
                }
                9 | 10 => {
                    if state == State::InName {
                        state = State::Other;
                    } else if state == State::InValue {
                        normalized = false;
                        report(Big2AttributeAction::Normalized {
                            attribute: n_atts,
                            value: 0,
                        });
                    }
                }
                11 | 17 if state != State::InValue => return n_atts,
                _ => {}
            }
            index += 2;
        }
        n_atts
    }

    /// Scans a bounded big-endian start-tag token and reports offsets to the
    /// parser-owned attribute storage adapter.
    pub fn big2_getAtts(
        byte_types: &[::core::ffi::c_uchar; 256],
        source: &[u8],
        report: impl FnMut(Big2AttributeAction),
    ) -> ::core::ffi::c_int {
        scan_big2_atts(byte_types, source, report)
    }

    pub unsafe extern "C" fn big2_charRefNumber(
        _enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        ptr = ptr.offset((2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize);
        if *ptr.offset(0 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && *ptr.offset(1 as isize) as ::core::ffi::c_int == 0x78 as ::core::ffi::c_int
        {
            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            while !(*ptr.offset(0 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && *ptr.offset(1 as isize) as ::core::ffi::c_int == 0x3b as ::core::ffi::c_int)
            {
                let mut c: ::core::ffi::c_int =
                    if *ptr.offset(0 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        *ptr.offset(1 as isize) as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    };
                match c {
                    crate::ascii_h::ASCII_0
                    | crate::ascii_h::ASCII_1_1
                    | crate::ascii_h::ASCII_2_1
                    | crate::ascii_h::ASCII_3_1
                    | crate::ascii_h::ASCII_4
                    | crate::ascii_h::ASCII_5
                    | crate::ascii_h::ASCII_6
                    | crate::ascii_h::ASCII_7
                    | crate::ascii_h::ASCII_8_1
                    | crate::ascii_h::ASCII_9_1 => {
                        result <<= 4 as ::core::ffi::c_int;
                        result |= c - crate::ascii_h::ASCII_0;
                    }
                    crate::ascii_h::ASCII_A
                    | crate::ascii_h::ASCII_B_1
                    | crate::ascii_h::ASCII_C
                    | crate::ascii_h::ASCII_D
                    | crate::ascii_h::ASCII_E_1
                    | crate::ascii_h::ASCII_F_1 => {
                        result <<= 4 as ::core::ffi::c_int;
                        result += 10 as ::core::ffi::c_int + (c - crate::ascii_h::ASCII_A);
                    }
                    crate::ascii_h::ASCII_a_1
                    | crate::ascii_h::ASCII_b
                    | crate::ascii_h::ASCII_c_1
                    | crate::ascii_h::ASCII_d
                    | crate::ascii_h::ASCII_e_1
                    | crate::ascii_h::ASCII_f => {
                        result <<= 4 as ::core::ffi::c_int;
                        result += 10 as ::core::ffi::c_int + (c - crate::ascii_h::ASCII_a_1);
                    }
                    _ => {}
                }
                if result >= 0x110000 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            }
        } else {
            while !(*ptr.offset(0 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && *ptr.offset(1 as isize) as ::core::ffi::c_int == 0x3b as ::core::ffi::c_int)
            {
                let mut c_0: ::core::ffi::c_int =
                    if *ptr.offset(0 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        *ptr.offset(1 as isize) as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    };
                result *= 10 as ::core::ffi::c_int;
                result += c_0 - crate::ascii_h::ASCII_0;
                if result >= 0x110000 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            }
        }
        return checkCharRefNumber(result);
    }

    pub unsafe extern "C" fn big2_nameLength(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ptr;
        loop {
            match if *ptr.offset(0 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0 as isize), *ptr.offset(1 as isize))
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
                29 | 22 | 23 | 24 | 25 | 26 | 27 => {
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                }
                _ => return ptr.offset_from(start) as ::core::ffi::c_int,
            }
        }
    }

    fn big2_update_position(
        encoding: &normal_encoding,
        bytes: &[::core::ffi::c_char],
        pos: &mut crate::src::xmltok::POSITION,
    ) {
        let mut offset = 0;
        while bytes.len().saturating_sub(offset) >= 2 {
            match big2_byte_type(encoding, bytes, offset) {
                5 => {
                    offset = (offset + 2).min(bytes.len());
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                6 => {
                    offset = (offset + 3).min(bytes.len());
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                7 => {
                    offset = (offset + 4).min(bytes.len());
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                10 => {
                    pos.columnNumber = 0 as crate::expat_external_h::XML_Size;
                    pos.lineNumber = pos.lineNumber.wrapping_add(1);
                    offset = (offset + 2).min(bytes.len());
                }
                9 => {
                    pos.lineNumber = pos.lineNumber.wrapping_add(1);
                    offset = (offset + 2).min(bytes.len());
                    if bytes.len().saturating_sub(offset) >= 2
                        && big2_byte_type(encoding, bytes, offset)
                            == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                    {
                        offset = (offset + 2).min(bytes.len());
                    }
                    pos.columnNumber = 0 as crate::expat_external_h::XML_Size;
                }
                _ => {
                    offset = (offset + 2).min(bytes.len());
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
            }
        }
    }

    pub unsafe extern "C" fn big2_updatePosition(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        pos: *mut crate::src::xmltok::POSITION,
    ) {
        let byte_len = unsafe { end.offset_from(ptr) };
        if byte_len <= 0 {
            return;
        }
        let encoding = unsafe { &*(enc as *const normal_encoding) };
        let bytes = unsafe { ::core::slice::from_raw_parts(ptr, byte_len as usize) };
        let pos = unsafe { &mut *pos };
        big2_update_position(encoding, bytes, pos);
    }

    use crate::src::xmltok::checkCharRefNumber;
    use crate::src::xmltok::isNever;
    use crate::src::xmltok::nametab_h::namePages;
    use crate::src::xmltok::nametab_h::namingBitmap;
    use crate::src::xmltok::nametab_h::nmstrtPages;
    use crate::src::xmltok::normal_encoding;
    use crate::src::xmltok::unicode_byte_type;
    use crate::src::xmltok::unknown_isName;
    use crate::src::xmltok::unknown_isInvalid;
    use crate::src::xmltok::unknown_isNmstrt;
    use crate::src::xmltok::utf8_invalid2;
    use crate::src::xmltok::utf8_invalid3;
    use crate::src::xmltok::utf8_invalid4;
    use crate::src::xmltok::utf8_is_name2;
    use crate::src::xmltok::utf8_is_name3;
    use crate::src::xmltok::utf8_is_name_start3;
    use crate::src::xmltok::utf8_isInvalid3;
    use crate::src::xmltok::Invalid2Checker;
    use crate::src::xmltok::Invalid3Checker;
    use crate::src::xmltok::Invalid4Checker;
    use crate::src::xmltok::Name2Checker;
    use crate::src::xmltok::Name3Checker;
    use crate::src::xmltok::Name4Checker;
    use crate::src::xmltok::NameStart3Checker;
    use crate::src::xmltok::NameStart4Checker;
}

pub mod xmltok_ns_c {
    pub unsafe extern "C" fn XmlGetUtf8InternalEncoding() -> *const crate::src::xmltok::ENCODING {
        return &raw const internal_utf8_encoding.enc;
    }
    #[export_name = "XmlGetUtf8InternalEncoding"]

    pub unsafe extern "C" fn XmlGetUtf8InternalEncoding_ffi() -> *const crate::src::xmltok::ENCODING
    {
        XmlGetUtf8InternalEncoding()
    }
    pub unsafe extern "C" fn XmlGetUtf16InternalEncoding() -> *const crate::src::xmltok::ENCODING {
        return &raw const internal_little2_encoding.enc;
    }
    #[export_name = "XmlGetUtf16InternalEncoding"]

    pub unsafe extern "C" fn XmlGetUtf16InternalEncoding_ffi() -> *const crate::src::xmltok::ENCODING
    {
        XmlGetUtf16InternalEncoding()
    }
    pub static mut encodings: [*const crate::src::xmltok::ENCODING; 7] =
        [::core::ptr::null::<crate::src::xmltok::ENCODING>(); 7];

    pub unsafe extern "C" fn initScanProlog(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        return initScan(
            &raw const encodings as *const *const crate::src::xmltok::ENCODING,
            enc as *const crate::src::xmltok::INIT_ENCODING,
            crate::src::xmltok::XML_PROLOG_STATE,
            ptr,
            end,
            nextTokPtr,
        );
    }

    pub unsafe extern "C" fn initScanContent(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        return initScan(
            &raw const encodings as *const *const crate::src::xmltok::ENCODING,
            enc as *const crate::src::xmltok::INIT_ENCODING,
            crate::src::xmltok::XML_CONTENT_STATE,
            ptr,
            end,
            nextTokPtr,
        );
    }
    pub unsafe extern "C" fn XmlInitEncoding(
        mut p: *mut crate::src::xmltok::INIT_ENCODING,
        mut encPtr: *mut *const crate::src::xmltok::ENCODING,
        mut name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = getEncodingIndex(name);
        if i == UNKNOWN_ENC as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        (*p).initEnc.isUtf16 = i as ::core::ffi::c_char;
        (*p).initEnc.scanners[crate::src::xmltok::XML_PROLOG_STATE as usize] =
            crate::src::xmltok::Scanner::InitProlog;
        (*p).initEnc.scanners[crate::src::xmltok::XML_CONTENT_STATE as usize] =
            crate::src::xmltok::Scanner::InitContent;
        (*p).initEnc.updatePosition = crate::src::xmltok::PositionUpdater::Init;
        (*p).selected_encoding = None;
        *encPtr = &raw mut (*p).initEnc;
        return 1 as ::core::ffi::c_int;
    }
    #[export_name = "XmlInitEncoding"]

    pub unsafe extern "C" fn XmlInitEncoding_ffi(
        mut p: *mut crate::src::xmltok::INIT_ENCODING,
        mut encPtr: *mut *const crate::src::xmltok::ENCODING,
        mut name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        XmlInitEncoding(p, encPtr, name)
    }
    pub unsafe extern "C" fn findEncoding(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
    ) -> *const crate::src::xmltok::ENCODING {
        let mut buf: [::core::ffi::c_char; 128] = ::core::mem::transmute:: <
            [u8; 128],
            [::core::ffi::c_char; 128],
        >(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
        let mut p: *mut ::core::ffi::c_char = &raw mut buf as *mut ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        crate::src::xmltok::convert_to_utf8(
            enc,
            &raw mut ptr,
            end,
            &raw mut p,
            p.offset(128 as ::core::ffi::c_int as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)),
        );
        if ptr != end {
            return ::core::ptr::null::<crate::src::xmltok::ENCODING>();
        }
        *p = 0 as ::core::ffi::c_char;
        if streqci(
            &raw mut buf as *mut ::core::ffi::c_char,
            &raw const KW_UTF_16 as *const ::core::ffi::c_char,
        ) != 0
            && (*enc).minBytesPerChar == 2 as ::core::ffi::c_int
        {
            return enc;
        }
        i = getEncodingIndex(&raw mut buf as *mut ::core::ffi::c_char);
        if i == UNKNOWN_ENC as ::core::ffi::c_int {
            return ::core::ptr::null::<crate::src::xmltok::ENCODING>();
        }
        return encodings[i as usize];
    }
    pub unsafe extern "C" fn XmlParseXmlDecl(
        mut isGeneralTextEntity: ::core::ffi::c_int,
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut badPtr: *mut *const ::core::ffi::c_char,
        mut versionPtr: *mut *const ::core::ffi::c_char,
        mut versionEndPtr: *mut *const ::core::ffi::c_char,
        mut encodingName: *mut *const ::core::ffi::c_char,
        mut encoding: *mut *const crate::src::xmltok::ENCODING,
        mut standalone: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        return doParseXmlDecl(
            Some(
                findEncoding
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    )
                        -> *const crate::src::xmltok::ENCODING,
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
        );
    }
    #[export_name = "XmlParseXmlDecl"]

    pub unsafe extern "C" fn XmlParseXmlDecl_ffi(
        mut isGeneralTextEntity: ::core::ffi::c_int,
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut badPtr: *mut *const ::core::ffi::c_char,
        mut versionPtr: *mut *const ::core::ffi::c_char,
        mut versionEndPtr: *mut *const ::core::ffi::c_char,
        mut encodingName: *mut *const ::core::ffi::c_char,
        mut encoding: *mut *const crate::src::xmltok::ENCODING,
        mut standalone: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        XmlParseXmlDecl(
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
    pub unsafe extern "C" fn XmlGetUtf8InternalEncodingNS() -> *const crate::src::xmltok::ENCODING {
        return &raw const internal_utf8_encoding_ns.enc;
    }
    #[export_name = "XmlGetUtf8InternalEncodingNS"]

    pub unsafe extern "C" fn XmlGetUtf8InternalEncodingNS_ffi(
    ) -> *const crate::src::xmltok::ENCODING {
        XmlGetUtf8InternalEncodingNS()
    }
    pub unsafe extern "C" fn XmlGetUtf16InternalEncodingNS() -> *const crate::src::xmltok::ENCODING
    {
        return &raw const internal_little2_encoding_ns.enc;
    }
    #[export_name = "XmlGetUtf16InternalEncodingNS"]

    pub unsafe extern "C" fn XmlGetUtf16InternalEncodingNS_ffi(
    ) -> *const crate::src::xmltok::ENCODING {
        XmlGetUtf16InternalEncodingNS()
    }
    pub static mut encodingsNS: [*const crate::src::xmltok::ENCODING; 7] =
        [::core::ptr::null::<crate::src::xmltok::ENCODING>(); 7];

    pub unsafe extern "C" fn initScanPrologNS(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        return initScan(
            &raw const encodingsNS as *const *const crate::src::xmltok::ENCODING,
            enc as *const crate::src::xmltok::INIT_ENCODING,
            crate::src::xmltok::XML_PROLOG_STATE,
            ptr,
            end,
            nextTokPtr,
        );
    }

    pub unsafe extern "C" fn initScanContentNS(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        return initScan(
            &raw const encodingsNS as *const *const crate::src::xmltok::ENCODING,
            enc as *const crate::src::xmltok::INIT_ENCODING,
            crate::src::xmltok::XML_CONTENT_STATE,
            ptr,
            end,
            nextTokPtr,
        );
    }
    pub unsafe extern "C" fn XmlInitEncodingNS(
        mut p: *mut crate::src::xmltok::INIT_ENCODING,
        mut encPtr: *mut *const crate::src::xmltok::ENCODING,
        mut name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = getEncodingIndex(name);
        if i == UNKNOWN_ENC as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        (*p).initEnc.isUtf16 = i as ::core::ffi::c_char;
        (*p).initEnc.scanners[crate::src::xmltok::XML_PROLOG_STATE as usize] =
            crate::src::xmltok::Scanner::InitPrologNS;
        (*p).initEnc.scanners[crate::src::xmltok::XML_CONTENT_STATE as usize] =
            crate::src::xmltok::Scanner::InitContentNS;
        (*p).initEnc.updatePosition = crate::src::xmltok::PositionUpdater::Init;
        (*p).selected_encoding = None;
        *encPtr = &raw mut (*p).initEnc;
        return 1 as ::core::ffi::c_int;
    }
    #[export_name = "XmlInitEncodingNS"]

    pub unsafe extern "C" fn XmlInitEncodingNS_ffi(
        mut p: *mut crate::src::xmltok::INIT_ENCODING,
        mut encPtr: *mut *const crate::src::xmltok::ENCODING,
        mut name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        XmlInitEncodingNS(p, encPtr, name)
    }
    pub unsafe extern "C" fn findEncodingNS(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
    ) -> *const crate::src::xmltok::ENCODING {
        let mut buf: [::core::ffi::c_char; 128] = ::core::mem::transmute:: <
            [u8; 128],
            [::core::ffi::c_char; 128],
        >(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
        let mut p: *mut ::core::ffi::c_char = &raw mut buf as *mut ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        crate::src::xmltok::convert_to_utf8(
            enc,
            &raw mut ptr,
            end,
            &raw mut p,
            p.offset(128 as ::core::ffi::c_int as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)),
        );
        if ptr != end {
            return ::core::ptr::null::<crate::src::xmltok::ENCODING>();
        }
        *p = 0 as ::core::ffi::c_char;
        if streqci(
            &raw mut buf as *mut ::core::ffi::c_char,
            &raw const KW_UTF_16 as *const ::core::ffi::c_char,
        ) != 0
            && (*enc).minBytesPerChar == 2 as ::core::ffi::c_int
        {
            return enc;
        }
        i = getEncodingIndex(&raw mut buf as *mut ::core::ffi::c_char);
        if i == UNKNOWN_ENC as ::core::ffi::c_int {
            return ::core::ptr::null::<crate::src::xmltok::ENCODING>();
        }
        return encodingsNS[i as usize];
    }
    pub unsafe extern "C" fn XmlParseXmlDeclNS(
        mut isGeneralTextEntity: ::core::ffi::c_int,
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut badPtr: *mut *const ::core::ffi::c_char,
        mut versionPtr: *mut *const ::core::ffi::c_char,
        mut versionEndPtr: *mut *const ::core::ffi::c_char,
        mut encodingName: *mut *const ::core::ffi::c_char,
        mut encoding: *mut *const crate::src::xmltok::ENCODING,
        mut standalone: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        return doParseXmlDecl(
            Some(
                findEncodingNS
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    )
                        -> *const crate::src::xmltok::ENCODING,
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
        );
    }
    #[export_name = "XmlParseXmlDeclNS"]

    pub unsafe extern "C" fn XmlParseXmlDeclNS_ffi(
        mut isGeneralTextEntity: ::core::ffi::c_int,
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut badPtr: *mut *const ::core::ffi::c_char,
        mut versionPtr: *mut *const ::core::ffi::c_char,
        mut versionEndPtr: *mut *const ::core::ffi::c_char,
        mut encodingName: *mut *const ::core::ffi::c_char,
        mut encoding: *mut *const crate::src::xmltok::ENCODING,
        mut standalone: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        XmlParseXmlDeclNS(
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
    use crate::src::xmltok::doParseXmlDecl;
    use crate::src::xmltok::getEncodingIndex;
    use crate::src::xmltok::initScan;
    use crate::src::xmltok::internal_little2_encoding;
    use crate::src::xmltok::internal_little2_encoding_ns;
    use crate::src::xmltok::internal_utf8_encoding;
    use crate::src::xmltok::internal_utf8_encoding_ns;
    use crate::src::xmltok::streqci;

    use crate::src::xmltok::KW_UTF_16;

    use crate::src::xmltok::UNKNOWN_ENC;
}

pub mod nametab_h {

    pub static namingBitmap: [::core::ffi::c_uint; 320] = [
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0x4000000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x87fffffe as ::core::ffi::c_uint,
        0x7fffffe as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
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
        0 as ::core::ffi::c_uint,
        0xffff0000 as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xf80001ff as ::core::ffi::c_uint,
        0x3 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
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
        0 as ::core::ffi::c_uint,
        0xfffe0000 as ::core::ffi::c_uint,
        0x27fffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffffffe as ::core::ffi::c_uint,
        0x7f as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0xffff0000 as ::core::ffi::c_uint,
        0x707ff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0x7fffffe as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x7fe as ::core::ffi::c_uint,
        0xfffe0000 as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0x7cffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x2f7fff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x60 as ::core::ffi::c_uint,
        0xffffffe0 as ::core::ffi::c_uint,
        0x23ffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xff000000 as ::core::ffi::c_uint,
        0x3 as ::core::ffi::c_uint,
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
        0 as ::core::ffi::c_uint,
        0x1 as ::core::ffi::c_uint,
        0xfff99fe0 as ::core::ffi::c_uint,
        0x23cdfdff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xb0000000 as ::core::ffi::c_uint,
        0x3 as ::core::ffi::c_uint,
        0xd63dc7e0 as ::core::ffi::c_uint,
        0x3bfc718 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0xfffddfe0 as ::core::ffi::c_uint,
        0x3effdff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0x3 as ::core::ffi::c_uint,
        0xfffddfe0 as ::core::ffi::c_uint,
        0x3effdff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x40000000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x3 as ::core::ffi::c_uint,
        0xfffddfe0 as ::core::ffi::c_uint,
        0x3fffdff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0x3 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0xfffffffe as ::core::ffi::c_uint,
        0xd7fff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x3f as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0xfef02596 as ::core::ffi::c_uint,
        0x200d6cae as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x1f as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0xfffffeff as ::core::ffi::c_uint,
        0x3ff as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffff003f as ::core::ffi::c_uint,
        0x7fffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x7daed as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x50000000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x82315001 as ::core::ffi::c_uint,
        0x2c62ab as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x40000000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xf580c900 as ::core::ffi::c_uint,
        0x7 as ::core::ffi::c_uint,
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
        0 as ::core::ffi::c_uint,
        0x4c40 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0x7 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0x80 as ::core::ffi::c_uint,
        0x3fe as ::core::ffi::c_uint,
        0xfffffffe as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0x1fffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffffffe as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0x7ffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffffffe0 as ::core::ffi::c_uint,
        0x1fff as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0x3f as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xf as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0x7ff6000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x87fffffe as ::core::ffi::c_uint,
        0x7fffffe as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0x800000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xff7fffff as ::core::ffi::c_uint,
        0xff7fffff as ::core::ffi::c_uint,
        0xffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0xffff0000 as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xf80001ff as ::core::ffi::c_uint,
        0x30003 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0x3f as ::core::ffi::c_uint,
        0x3 as ::core::ffi::c_uint,
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
        0 as ::core::ffi::c_uint,
        0xfffe0000 as ::core::ffi::c_uint,
        0x27fffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffffffe as ::core::ffi::c_uint,
        0xfffe007f as ::core::ffi::c_uint,
        0xbbfffffb as ::core::ffi::c_uint,
        0xffff0016 as ::core::ffi::c_uint,
        0x707ff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
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
        0xffcf as ::core::ffi::c_uint,
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
        0x3bbf as ::core::ffi::c_uint,
        0xffc1 as ::core::ffi::c_uint,
        0xfff99fee as ::core::ffi::c_uint,
        0xf3cdfdff as ::core::ffi::c_uint,
        0xb0c0398f as ::core::ffi::c_uint,
        0xffc3 as ::core::ffi::c_uint,
        0xd63dc7ec as ::core::ffi::c_uint,
        0xc3bfc718 as ::core::ffi::c_uint,
        0x803dc7 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xff80 as ::core::ffi::c_uint,
        0xfffddfee as ::core::ffi::c_uint,
        0xc3effdff as ::core::ffi::c_uint,
        0x603ddf as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffc3 as ::core::ffi::c_uint,
        0xfffddfec as ::core::ffi::c_uint,
        0xc3effdff as ::core::ffi::c_uint,
        0x40603ddf as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffc3 as ::core::ffi::c_uint,
        0xfffddfec as ::core::ffi::c_uint,
        0xc3fffdff as ::core::ffi::c_uint,
        0x803dcf as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffc3 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0xfffffffe as ::core::ffi::c_uint,
        0x7ff7fff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x3ff7fff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0xfef02596 as ::core::ffi::c_uint,
        0x3bff6cae as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x3ff3f5f as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0x3000000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xc2a003ff as ::core::ffi::c_uint,
        0xfffffeff as ::core::ffi::c_uint,
        0xfffe03ff as ::core::ffi::c_uint,
        0xfebf0fdf as ::core::ffi::c_uint,
        0x2fe3fff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        0x1fff0000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x2 as ::core::ffi::c_uint,
        0xa0 as ::core::ffi::c_uint,
        0x3efffe as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffffffe as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0x661fffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffffffe as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0x77ffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
    ];

    pub static nmstrtPages: [::core::ffi::c_uchar; 256] = [
        0x2 as ::core::ffi::c_uchar,
        0x3 as ::core::ffi::c_uchar,
        0x4 as ::core::ffi::c_uchar,
        0x5 as ::core::ffi::c_uchar,
        0x6 as ::core::ffi::c_uchar,
        0x7 as ::core::ffi::c_uchar,
        0x8 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0x9 as ::core::ffi::c_uchar,
        0xa as ::core::ffi::c_uchar,
        0xb as ::core::ffi::c_uchar,
        0xc as ::core::ffi::c_uchar,
        0xd as ::core::ffi::c_uchar,
        0xe as ::core::ffi::c_uchar,
        0xf as ::core::ffi::c_uchar,
        0x10 as ::core::ffi::c_uchar,
        0x11 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0x12 as ::core::ffi::c_uchar,
        0x13 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0x14 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0x15 as ::core::ffi::c_uchar,
        0x16 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x17 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x18 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
    ];

    pub static namePages: [::core::ffi::c_uchar; 256] = [
        0x19 as ::core::ffi::c_uchar,
        0x3 as ::core::ffi::c_uchar,
        0x1a as ::core::ffi::c_uchar,
        0x1b as ::core::ffi::c_uchar,
        0x1c as ::core::ffi::c_uchar,
        0x1d as ::core::ffi::c_uchar,
        0x1e as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0x1f as ::core::ffi::c_uchar,
        0x20 as ::core::ffi::c_uchar,
        0x21 as ::core::ffi::c_uchar,
        0x22 as ::core::ffi::c_uchar,
        0x23 as ::core::ffi::c_uchar,
        0x24 as ::core::ffi::c_uchar,
        0x25 as ::core::ffi::c_uchar,
        0x10 as ::core::ffi::c_uchar,
        0x11 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0x12 as ::core::ffi::c_uchar,
        0x13 as ::core::ffi::c_uchar,
        0x26 as ::core::ffi::c_uchar,
        0x14 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0x27 as ::core::ffi::c_uchar,
        0x16 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x17 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_uchar,
        0x18 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_uchar,
    ];
}
pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_ptrdiff_t_h::ptrdiff_t;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::ascii_h::ASCII_a_1;
pub use crate::ascii_h::ASCII_b;
pub use crate::ascii_h::ASCII_c_1;
pub use crate::ascii_h::ASCII_d;
pub use crate::ascii_h::ASCII_e_1;
pub use crate::ascii_h::ASCII_f;
pub use crate::ascii_h::ASCII_g_1;
pub use crate::ascii_h::ASCII_i;
pub use crate::ascii_h::ASCII_l_1;
pub use crate::ascii_h::ASCII_m_1;
pub use crate::ascii_h::ASCII_n;
pub use crate::ascii_h::ASCII_o;
pub use crate::ascii_h::ASCII_q;
pub use crate::ascii_h::ASCII_r;
pub use crate::ascii_h::ASCII_s;
pub use crate::ascii_h::ASCII_t;
pub use crate::ascii_h::ASCII_v;
pub use crate::ascii_h::ASCII_x_1;
pub use crate::ascii_h::ASCII_y;
pub use crate::ascii_h::ASCII_z;
pub use crate::ascii_h::ASCII_0;
pub use crate::ascii_h::ASCII_1_1;
pub use crate::ascii_h::ASCII_2_1;
pub use crate::ascii_h::ASCII_3_1;
pub use crate::ascii_h::ASCII_4;
pub use crate::ascii_h::ASCII_5;
pub use crate::ascii_h::ASCII_6;
pub use crate::ascii_h::ASCII_7;
pub use crate::ascii_h::ASCII_8_1;
pub use crate::ascii_h::ASCII_9_1;
pub use crate::ascii_h::ASCII_A;
pub use crate::ascii_h::ASCII_AMP;
pub use crate::ascii_h::ASCII_APOS;
pub use crate::ascii_h::ASCII_B_1;
pub use crate::ascii_h::ASCII_C;
pub use crate::ascii_h::ASCII_COLON;
pub use crate::ascii_h::ASCII_D;
pub use crate::ascii_h::ASCII_EQUALS;
pub use crate::ascii_h::ASCII_E_1;
pub use crate::ascii_h::ASCII_F_1;
pub use crate::ascii_h::ASCII_GT;
pub use crate::ascii_h::ASCII_I;
pub use crate::ascii_h::ASCII_LSQB;
pub use crate::ascii_h::ASCII_LT;
pub use crate::ascii_h::ASCII_L_1;
pub use crate::ascii_h::ASCII_MINUS;
pub use crate::ascii_h::ASCII_M_1;
pub use crate::ascii_h::ASCII_O;
pub use crate::ascii_h::ASCII_PERIOD;
pub use crate::ascii_h::ASCII_QUOT;
pub use crate::ascii_h::ASCII_S;
pub use crate::ascii_h::ASCII_SPACE;
pub use crate::ascii_h::ASCII_T;
pub use crate::ascii_h::ASCII_U;
pub use crate::ascii_h::ASCII_UNDERSCORE;
pub use crate::ascii_h::ASCII_X_1;
pub use crate::ascii_h::ASCII_Z;
pub use crate::expat_external_h::XML_Size;

pub use crate::src::xmltok::nametab_h::namePages;
pub use crate::src::xmltok::nametab_h::namingBitmap;
pub use crate::src::xmltok::nametab_h::nmstrtPages;
pub use crate::stdbool_h::false_0;
pub use crate::stdbool_h::true_0;

pub use crate::src::xmltok::xmltok_impl_c::big2_attributeValueTok;
pub use crate::src::xmltok::xmltok_impl_c::Big2AttributeAction;
pub use crate::src::xmltok::xmltok_impl_c::big2_cdataSectionTok;
pub use crate::src::xmltok::xmltok_impl_c::big2_charRefNumber;
pub use crate::src::xmltok::xmltok_impl_c::big2_checkPiTarget;
pub use crate::src::xmltok::xmltok_impl_c::big2_contentTok;
pub use crate::src::xmltok::xmltok_impl_c::big2_entityValueTok;
pub use crate::src::xmltok::xmltok_impl_c::big2_getAtts;
pub use crate::src::xmltok::xmltok_impl_c::big2_ignoreSectionTok;
pub use crate::src::xmltok::xmltok_impl_c::big2_isPublicId;
pub use crate::src::xmltok::xmltok_impl_c::big2_nameLength;
pub use crate::src::xmltok::xmltok_impl_c::big2_prologTok;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanAtts;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanCdataSection;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanComment;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanDecl;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanEndTag;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanHexCharRef;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanLit;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanLt;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanPercent;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanPi;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanPoundName;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanRef;
pub use crate::src::xmltok::xmltok_impl_c::big2_updatePosition;
pub use crate::src::xmltok::xmltok_impl_c::little2_attributeValueTok;
pub use crate::src::xmltok::xmltok_impl_c::little2_cdataSectionTok;
pub use crate::src::xmltok::xmltok_impl_c::little2_charRefNumber;
pub use crate::src::xmltok::xmltok_impl_c::little2_checkPiTarget;
pub use crate::src::xmltok::xmltok_impl_c::little2_contentTok;
pub use crate::src::xmltok::xmltok_impl_c::little2_entityValueTok;
pub use crate::src::xmltok::xmltok_impl_c::little2_getAtts;
pub use crate::src::xmltok::xmltok_impl_c::little2_ignoreSectionTok;
pub use crate::src::xmltok::xmltok_impl_c::little2_isPublicId;
pub use crate::src::xmltok::xmltok_impl_c::little2_nameLength;
pub use crate::src::xmltok::xmltok_impl_c::little2_prologTok;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanAtts;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanCdataSection;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanCharRef;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanComment;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanDecl;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanEndTag;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanHexCharRef;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanLit;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanLt;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanPercent;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanPi;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanPoundName;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanRef;
pub use crate::src::xmltok::xmltok_impl_c::little2_updatePosition;
pub use crate::src::xmltok::xmltok_impl_c::normal_attributeValueTok;
pub use crate::src::xmltok::xmltok_impl_c::normal_cdataSectionTok;
pub use crate::src::xmltok::xmltok_impl_c::normal_charRefNumber;
pub use crate::src::xmltok::xmltok_impl_c::normal_checkPiTarget;
pub use crate::src::xmltok::xmltok_impl_c::normal_contentTok;
pub use crate::src::xmltok::xmltok_impl_c::normal_entityValueTok;
pub use crate::src::xmltok::xmltok_impl_c::normal_getAtts;
pub use crate::src::xmltok::xmltok_impl_c::normal_ignoreSectionTok;
pub use crate::src::xmltok::xmltok_impl_c::normal_isPublicId;
pub use crate::src::xmltok::xmltok_impl_c::normal_nameLength;
pub use crate::src::xmltok::xmltok_impl_c::normal_prologTok;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanCdataSection;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanCharRef;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanComment;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanDecl;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanHexCharRef;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanLit;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanLt;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanPercent;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanPi;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanPoundName;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanRef;
pub use crate::src::xmltok::xmltok_impl_c::normal_updatePosition;
pub use crate::src::xmltok::xmltok_impl_c::skip_s;
pub use crate::src::xmltok::xmltok_ns_c::encodings;
pub use crate::src::xmltok::xmltok_ns_c::encodingsNS;
pub use crate::src::xmltok::xmltok_ns_c::findEncoding;
pub use crate::src::xmltok::xmltok_ns_c::findEncodingNS;
pub use crate::src::xmltok::xmltok_ns_c::initScanContent;
pub use crate::src::xmltok::xmltok_ns_c::initScanContentNS;
pub use crate::src::xmltok::xmltok_ns_c::initScanProlog;
pub use crate::src::xmltok::xmltok_ns_c::initScanPrologNS;
pub use crate::src::xmltok::xmltok_ns_c::XmlGetUtf16InternalEncoding;
pub use crate::src::xmltok::xmltok_ns_c::XmlGetUtf16InternalEncodingNS;
pub use crate::src::xmltok::xmltok_ns_c::XmlGetUtf8InternalEncoding;
pub use crate::src::xmltok::xmltok_ns_c::XmlGetUtf8InternalEncodingNS;
pub use crate::src::xmltok::xmltok_ns_c::XmlInitEncoding;
pub use crate::src::xmltok::xmltok_ns_c::XmlInitEncodingNS;
pub use crate::src::xmltok::xmltok_ns_c::XmlParseXmlDecl;
pub use crate::src::xmltok::xmltok_ns_c::XmlParseXmlDeclNS;
pub use crate::xmltok_impl_c::inName;
pub use crate::xmltok_impl_c::inName_0;
pub use crate::xmltok_impl_c::inName_1;
pub use crate::xmltok_impl_c::inValue;
pub use crate::xmltok_impl_c::inValue_0;
pub use crate::xmltok_impl_c::inValue_1;
pub use crate::xmltok_impl_c::other;
pub use crate::xmltok_impl_c::other_0;
pub use crate::xmltok_impl_c::other_1;
pub use crate::xmltok_impl_h::C2Rust_Unnamed_3;
pub use crate::xmltok_impl_h::BT_AMP;
pub use crate::xmltok_impl_h::BT_APOS;
pub use crate::xmltok_impl_h::BT_AST;
pub use crate::xmltok_impl_h::BT_COLON_0;
pub use crate::xmltok_impl_h::BT_COMMA;
pub use crate::xmltok_impl_h::BT_CR;
pub use crate::xmltok_impl_h::BT_DIGIT;
pub use crate::xmltok_impl_h::BT_EQUALS;
pub use crate::xmltok_impl_h::BT_EXCL;
pub use crate::xmltok_impl_h::BT_GT;
pub use crate::xmltok_impl_h::BT_HEX;
pub use crate::xmltok_impl_h::BT_LEAD2;
pub use crate::xmltok_impl_h::BT_LEAD3;
pub use crate::xmltok_impl_h::BT_LEAD4;
pub use crate::xmltok_impl_h::BT_LF;
pub use crate::xmltok_impl_h::BT_LPAR;
pub use crate::xmltok_impl_h::BT_LSQB;
pub use crate::xmltok_impl_h::BT_LT;
pub use crate::xmltok_impl_h::BT_MALFORM;
pub use crate::xmltok_impl_h::BT_MINUS;
pub use crate::xmltok_impl_h::BT_NAME;
pub use crate::xmltok_impl_h::BT_NMSTRT;
pub use crate::xmltok_impl_h::BT_NONASCII;
pub use crate::xmltok_impl_h::BT_NONXML;
pub use crate::xmltok_impl_h::BT_NUM;
pub use crate::xmltok_impl_h::BT_OTHER;
pub use crate::xmltok_impl_h::BT_PERCNT;
pub use crate::xmltok_impl_h::BT_PLUS;
pub use crate::xmltok_impl_h::BT_QUEST;
pub use crate::xmltok_impl_h::BT_QUOT;
pub use crate::xmltok_impl_h::BT_RPAR;
pub use crate::xmltok_impl_h::BT_RSQB;
pub use crate::xmltok_impl_h::BT_S;
pub use crate::xmltok_impl_h::BT_SEMI;
pub use crate::xmltok_impl_h::BT_SOL;
pub use crate::xmltok_impl_h::BT_TRAIL;
pub use crate::xmltok_impl_h::BT_VERBAR;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct normal_encoding {
    pub enc: crate::src::xmltok::ENCODING,
    pub type_0: [::core::ffi::c_uchar; 256],
    pub isName2: Name2Checker,
    pub isName3: Name3Checker,
    pub isName4: Name4Checker,
    pub isNmstrt2: NameStart2Checker,
    pub isNmstrt3: NameStart3Checker,
    pub isNmstrt4: NameStart4Checker,
    pub invalid2: Invalid2Checker,
    pub invalid3: Invalid3Checker,
    pub invalid4: Invalid4Checker,
}

#[derive(Copy, Clone)]
pub enum Name2Checker {
    Never,
    Utf8,
    Unknown,
}

#[derive(Copy, Clone)]
pub enum Name3Checker {
    Never,
    Utf8,
    Unknown,
}

#[derive(Copy, Clone)]
pub enum Name4Checker {
    Never,
    Unknown,
}

#[derive(Copy, Clone)]
pub enum NameStart2Checker {
    Never,
    Utf8,
    Unknown,
}

#[derive(Copy, Clone)]
pub enum NameStart3Checker {
    Never,
    Utf8,
    Unknown,
}

#[derive(Copy, Clone)]
pub enum NameStart4Checker {
    Never,
    Unknown,
}

#[derive(Copy, Clone)]
pub enum Invalid2Checker {
    Never,
    Utf8,
    Unknown,
}

#[derive(Copy, Clone)]
pub enum Invalid3Checker {
    Never,
    Utf8,
    Unknown,
}

#[derive(Copy, Clone)]
pub enum Invalid4Checker {
    Never,
    Utf8,
    Unknown,
}

fn utf8_invalid2(input: &[u8]) -> bool {
    input[0] < 0xc2 || input[1] & 0x80 == 0 || input[1] & 0xc0 == 0xc0
}

pub const UTF8_cval2: C2Rust_Unnamed_8 = 192;

pub const UTF8_cval4: C2Rust_Unnamed_8 = 240;

pub const UTF8_cval3: C2Rust_Unnamed_8 = 224;

pub const UNKNOWN_ENC: C2Rust_Unnamed_9 = -1;

pub const NO_ENC: C2Rust_Unnamed_9 = 6;

pub const UTF_16LE_ENC: C2Rust_Unnamed_9 = 5;

pub const UTF_16BE_ENC: C2Rust_Unnamed_9 = 4;

pub const UTF_8_ENC: C2Rust_Unnamed_9 = 2;

pub const UTF_16_ENC: C2Rust_Unnamed_9 = 3;

pub const ISO_8859_1_ENC: C2Rust_Unnamed_9 = 0;

pub const min4: C2Rust_Unnamed_7 = 65536;

pub const min3: C2Rust_Unnamed_7 = 2048;

pub const UTF8_cval1: C2Rust_Unnamed_8 = 0;

pub const min2: C2Rust_Unnamed_7 = 128;

pub type C2Rust_Unnamed_7 = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct unknown_encoding {
    pub normal: normal_encoding,
    pub converter_id: usize,
    pub userData: *mut ::core::ffi::c_void,
    pub utf16: [::core::ffi::c_ushort; 256],
    pub utf8: [[::core::ffi::c_char; 4]; 256],
}

pub type C2Rust_Unnamed_8 = ::core::ffi::c_uint;

pub type C2Rust_Unnamed_9 = ::core::ffi::c_int;

pub const US_ASCII_ENC: C2Rust_Unnamed_9 = 1;

unsafe extern "C" fn isNever(
    _enc: *const crate::src::xmltok::ENCODING,
    _p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}

fn utf8_is_name2(input: &[u8]) -> bool {
    (namingBitmap[(((namePages[(input[0] as ::core::ffi::c_int >> 2 as ::core::ffi::c_int
        & 7 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int)
        << 3 as ::core::ffi::c_int)
        + ((input[0] as ::core::ffi::c_int & 3 as ::core::ffi::c_int)
            << 1 as ::core::ffi::c_int)
        + (input[1] as ::core::ffi::c_int
            >> 5 as ::core::ffi::c_int
            & 1 as ::core::ffi::c_int)) as usize]
        & (1 as ::core::ffi::c_uint)
            << (input[1] as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
        != 0
}

fn utf8_is_name3(input: &[u8]) -> bool {
    let first = input[0] as ::core::ffi::c_int;
    let second = input[1] as ::core::ffi::c_int;
    let third = input[2] as ::core::ffi::c_int;
    (namingBitmap[(((namePages[(((first & 0xf) << 4) + ((second >> 2) & 0xf)) as usize]
        as ::core::ffi::c_int)
        << 3)
        + ((second & 3) << 1)
        + ((third >> 5) & 1)) as usize]
        & (1 as ::core::ffi::c_uint) << (third & 0x1f))
        != 0
}

fn utf8_is_name_start2(input: &[u8]) -> bool {
    let Some((&first, rest)) = input.split_first() else {
        return false;
    };
    let Some((&second, _)) = rest.split_first() else {
        return false;
    };
    (namingBitmap[(((nmstrtPages[(first as ::core::ffi::c_int >> 2 as ::core::ffi::c_int
        & 7 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int)
        << 3 as ::core::ffi::c_int)
        + ((first as ::core::ffi::c_int & 3 as ::core::ffi::c_int)
            << 1 as ::core::ffi::c_int)
        + (second as ::core::ffi::c_int >> 5 as ::core::ffi::c_int & 1 as ::core::ffi::c_int))
        as usize]
        & (1 as ::core::ffi::c_uint)
            << (second as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
        != 0
}

fn utf8_is_name_start3(input: &[u8]) -> bool {
    let [first, second, third, ..] = input else {
        return false;
    };
    let first = *first as ::core::ffi::c_int;
    let second = *second as ::core::ffi::c_int;
    let third = *third as ::core::ffi::c_int;
    (namingBitmap[(((nmstrtPages[(((first & 0xf) << 4) + ((second >> 2) & 0xf)) as usize]
        as ::core::ffi::c_int)
        << 3)
        + ((second & 3) << 1)
        + (third >> 5 & 1)) as usize]
        & (1 as ::core::ffi::c_uint) << (third & 0x1f))
        != 0
}

fn utf8_invalid3(input: &[u8]) -> bool {
    let Some((&first, rest)) = input.split_first() else {
        return true;
    };
    let Some((&second, rest)) = rest.split_first() else {
        return true;
    };
    let Some((&third, _)) = rest.split_first() else {
        return true;
    };
    third & 0x80 == 0
        || if first == 0xef && second == 0xbf {
            third > 0xbd
        } else {
            third & 0xc0 == 0xc0
        }
        || if first == 0xe0 {
            second < 0xa0 || second & 0xc0 == 0xc0
        } else {
            second & 0x80 == 0
                || if first == 0xed {
                    second > 0x9f
                } else {
                    second & 0xc0 == 0xc0
                }
        }
}

unsafe extern "C" fn utf8_isInvalid3(
    _enc: *const crate::src::xmltok::ENCODING,
    p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    utf8_invalid3(::core::slice::from_raw_parts(p.cast::<u8>(), 3)) as ::core::ffi::c_int
}

fn utf8_invalid4(input: &[u8]) -> bool {
    let [first, second, third, fourth, ..] = input else {
        return true;
    };
    *fourth & 0xc0 != 0x80
        || *third & 0xc0 != 0x80
        || match *first {
            0xf0 => *second < 0x90 || *second & 0xc0 != 0x80,
            0xf4 => *second > 0x8f || *second & 0xc0 != 0x80,
            _ => *second & 0xc0 != 0x80,
        }
}
pub unsafe extern "C" fn _INTERNAL_trim_to_complete_utf8_characters(
    mut from: *const ::core::ffi::c_char,
    mut fromLimRef: *mut *const ::core::ffi::c_char,
) {
    let mut fromLim: *const ::core::ffi::c_char = *fromLimRef;
    let mut walked: crate::__stddef_size_t_h::size_t = 0 as crate::__stddef_size_t_h::size_t;
    while fromLim > from {
        let prev: ::core::ffi::c_uchar = *fromLim.offset(-1 as isize) as ::core::ffi::c_uchar;
        if prev as ::core::ffi::c_uint & 0xf8 as ::core::ffi::c_uint == 0xf0 as ::core::ffi::c_uint
        {
            if walked.wrapping_add(1 as crate::__stddef_size_t_h::size_t)
                >= 4 as crate::__stddef_size_t_h::size_t
            {
                fromLim =
                    fromLim.offset((4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
                break;
            } else {
                walked = 0 as crate::__stddef_size_t_h::size_t;
            }
        } else if prev as ::core::ffi::c_uint & 0xf0 as ::core::ffi::c_uint
            == 0xe0 as ::core::ffi::c_uint
        {
            if walked.wrapping_add(1 as crate::__stddef_size_t_h::size_t)
                >= 3 as crate::__stddef_size_t_h::size_t
            {
                fromLim =
                    fromLim.offset((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
                break;
            } else {
                walked = 0 as crate::__stddef_size_t_h::size_t;
            }
        } else if prev as ::core::ffi::c_uint & 0xe0 as ::core::ffi::c_uint
            == 0xc0 as ::core::ffi::c_uint
        {
            if walked.wrapping_add(1 as crate::__stddef_size_t_h::size_t)
                >= 2 as crate::__stddef_size_t_h::size_t
            {
                fromLim =
                    fromLim.offset((2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
                break;
            } else {
                walked = 0 as crate::__stddef_size_t_h::size_t;
            }
        } else if prev as ::core::ffi::c_uint & 0x80 as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        {
            break;
        }
        fromLim = fromLim.offset(-1);
        walked = walked.wrapping_add(1);
    }
    *fromLimRef = fromLim;
}
#[export_name = "_INTERNAL_trim_to_complete_utf8_characters"]

pub unsafe extern "C" fn _INTERNAL_trim_to_complete_utf8_characters_ffi(
    mut from: *const ::core::ffi::c_char,
    mut fromLimRef: *mut *const ::core::ffi::c_char,
) {
    _INTERNAL_trim_to_complete_utf8_characters(from, fromLimRef)
}
unsafe extern "C" fn utf8_toUtf8(
    _enc: *const crate::src::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_char,
    mut toLim: *const ::core::ffi::c_char,
) -> crate::src::xmltok::XML_Convert_Result {
    let mut input_incomplete: bool = crate::stdbool_h::false_0 != 0;
    let mut output_exhausted: bool = crate::stdbool_h::false_0 != 0;
    let bytesAvailable: crate::__stddef_ptrdiff_t_h::ptrdiff_t = fromLim.offset_from(*fromP);
    let bytesStorable: crate::__stddef_ptrdiff_t_h::ptrdiff_t = toLim.offset_from(*toP);
    if bytesAvailable > bytesStorable {
        fromLim = (*fromP).offset(bytesStorable as isize);
        output_exhausted = crate::stdbool_h::true_0 != 0;
    }
    let fromLimBefore: *const ::core::ffi::c_char = fromLim;
    _INTERNAL_trim_to_complete_utf8_characters(*fromP, &raw mut fromLim);
    if fromLim < fromLimBefore {
        input_incomplete = crate::stdbool_h::true_0 != 0;
    }
    let bytesToCopy: crate::__stddef_ptrdiff_t_h::ptrdiff_t = fromLim.offset_from(*fromP);
    crate::stdlib::memcpy(
        *toP as *mut ::core::ffi::c_void,
        *fromP as *const ::core::ffi::c_void,
        bytesToCopy as crate::__stddef_size_t_h::size_t,
    );
    *fromP = (*fromP).offset(bytesToCopy as isize);
    *toP = (*toP).offset(bytesToCopy as isize);
    if output_exhausted {
        return crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
    } else if input_incomplete {
        return crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
    } else {
        return crate::src::xmltok::XML_CONVERT_COMPLETED;
    };
}

fn utf8_to_utf16(
    byte_types: &[::core::ffi::c_uchar; 256],
    input: &[u8],
    output: &mut [::core::ffi::c_ushort],
) -> (crate::src::xmltok::XML_Convert_Result, usize, usize) {
    let mut input_offset = 0;
    let mut output_offset = 0;

    while input_offset < input.len() && output_offset < output.len() {
        match byte_types[input[input_offset] as usize] as u32 {
            crate::xmltok_impl_h::BT_LEAD2 => {
                if input.len() - input_offset < 2 {
                    return (
                        crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE,
                        input_offset,
                        output_offset,
                    );
                }
                output[output_offset] = (((input[input_offset] & 0x1f) as u16) << 6)
                    | (input[input_offset + 1] & 0x3f) as u16;
                input_offset += 2;
                output_offset += 1;
            }
            crate::xmltok_impl_h::BT_LEAD3 => {
                if input.len() - input_offset < 3 {
                    return (
                        crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE,
                        input_offset,
                        output_offset,
                    );
                }
                output[output_offset] = (((input[input_offset] & 0x0f) as u16) << 12)
                    | (((input[input_offset + 1] & 0x3f) as u16) << 6)
                    | (input[input_offset + 2] & 0x3f) as u16;
                input_offset += 3;
                output_offset += 1;
            }
            crate::xmltok_impl_h::BT_LEAD4 => {
                if output.len() - output_offset < 2 {
                    return (
                        crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED,
                        input_offset,
                        output_offset,
                    );
                }
                if input.len() - input_offset < 4 {
                    return (
                        crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE,
                        input_offset,
                        output_offset,
                    );
                }
                let scalar = (((input[input_offset] & 0x07) as u32) << 18)
                    | (((input[input_offset + 1] & 0x3f) as u32) << 12)
                    | (((input[input_offset + 2] & 0x3f) as u32) << 6)
                    | (input[input_offset + 3] & 0x3f) as u32;
                let surrogate = scalar.wrapping_sub(0x10000);
                output[output_offset] = ((surrogate >> 10) | 0xd800) as u16;
                output[output_offset + 1] = ((surrogate & 0x3ff) | 0xdc00) as u16;
                input_offset += 4;
                output_offset += 2;
            }
            _ => {
                output[output_offset] = input[input_offset] as u16;
                input_offset += 1;
                output_offset += 1;
            }
        }
    }

    let result = if input_offset < input.len() {
        crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED
    } else {
        crate::src::xmltok::XML_CONVERT_COMPLETED
    };
    (result, input_offset, output_offset)
}

unsafe extern "C" fn utf8_toUtf16(
    enc: *const crate::src::xmltok::ENCODING,
    fromP: *mut *const ::core::ffi::c_char,
    fromLim: *const ::core::ffi::c_char,
    toP: *mut *mut ::core::ffi::c_ushort,
    toLim: *const ::core::ffi::c_ushort,
) -> crate::src::xmltok::XML_Convert_Result {
    // The converter contract supplies two ordered, single-allocation windows.
    // Keep the borrowed slices local so their lifetimes cannot outlive the call.
    let from = *fromP;
    let to = *toP;
    let input_len: usize = fromLim.offset_from(from).try_into().unwrap();
    let output_len: usize = toLim.offset_from(to).try_into().unwrap();
    let input_start = if input_len == 0 {
        ::core::ptr::NonNull::<u8>::dangling().as_ptr()
    } else {
        from as *const u8
    };
    let output_start = if output_len == 0 {
        ::core::ptr::NonNull::<::core::ffi::c_ushort>::dangling().as_ptr()
    } else {
        to
    };
    let input = ::core::slice::from_raw_parts(input_start, input_len);
    let output = ::core::slice::from_raw_parts_mut(output_start, output_len);
    let byte_types = &(*(enc as *const normal_encoding)).type_0;
    let (result, input_offset, output_offset) = utf8_to_utf16(byte_types, input, output);
    *fromP = from.add(input_offset);
    *toP = to.add(output_offset);
    result
}

static mut utf8_encoding_ns: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            crate::src::xmltok::Scanner::NormalProlog,
            crate::src::xmltok::Scanner::NormalContent,
            crate::src::xmltok::Scanner::NormalCdataSection,
            crate::src::xmltok::Scanner::NormalIgnoreSection,
        ],
        literalScanners: [
            LiteralScanner::NormalAttributeValue,
            LiteralScanner::NormalEntityValue,
        ],
        nameMatchesAscii: NameMatcher::Normal,
        nameLength: NameLength::Normal,
        skipS: WhitespaceSkipper::Normal,
        getAtts: AttributeScanner::Normal,
        charRefNumber: CharRefNumberDecoder::Normal,
        predefinedEntityName: PredefinedEntityNameMatcher::Normal,
        updatePosition: PositionUpdater::Normal,
        isPublicId: PublicIdChecker::Normal,
        utf8Convert: Utf8Converter::Utf8,
        utf16Convert: Utf16Converter::Utf8,
        minBytesPerChar: 1 as ::core::ffi::c_int,
        isUtf8: 1 as ::core::ffi::c_char,
        isUtf16: 0 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: Name2Checker::Utf8,
    isName3: Name3Checker::Utf8,
    isName4: Name4Checker::Never,
    isNmstrt2: NameStart2Checker::Utf8,
    isNmstrt3: NameStart3Checker::Utf8,
    isNmstrt4: NameStart4Checker::Never,
    invalid2: Invalid2Checker::Utf8,
    invalid3: Invalid3Checker::Utf8,
    invalid4: Invalid4Checker::Utf8,
};

static mut utf8_encoding: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            crate::src::xmltok::Scanner::NormalProlog,
            crate::src::xmltok::Scanner::NormalContent,
            crate::src::xmltok::Scanner::NormalCdataSection,
            crate::src::xmltok::Scanner::NormalIgnoreSection,
        ],
        literalScanners: [
            LiteralScanner::NormalAttributeValue,
            LiteralScanner::NormalEntityValue,
        ],
        nameMatchesAscii: NameMatcher::Normal,
        nameLength: NameLength::Normal,
        skipS: WhitespaceSkipper::Normal,
        getAtts: AttributeScanner::Normal,
        charRefNumber: CharRefNumberDecoder::Normal,
        predefinedEntityName: PredefinedEntityNameMatcher::Normal,
        updatePosition: PositionUpdater::Normal,
        isPublicId: PublicIdChecker::Normal,
        utf8Convert: Utf8Converter::Utf8,
        utf16Convert: Utf16Converter::Utf8,
        minBytesPerChar: 1 as ::core::ffi::c_int,
        isUtf8: 1 as ::core::ffi::c_char,
        isUtf16: 0 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: Name2Checker::Utf8,
    isName3: Name3Checker::Utf8,
    isName4: Name4Checker::Never,
    isNmstrt2: NameStart2Checker::Utf8,
    isNmstrt3: NameStart3Checker::Utf8,
    isNmstrt4: NameStart4Checker::Never,
    invalid2: Invalid2Checker::Utf8,
    invalid3: Invalid3Checker::Utf8,
    invalid4: Invalid4Checker::Utf8,
};

static mut internal_utf8_encoding_ns: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            crate::src::xmltok::Scanner::NormalProlog,
            crate::src::xmltok::Scanner::NormalContent,
            crate::src::xmltok::Scanner::NormalCdataSection,
            crate::src::xmltok::Scanner::NormalIgnoreSection,
        ],
        literalScanners: [
            LiteralScanner::NormalAttributeValue,
            LiteralScanner::NormalEntityValue,
        ],
        nameMatchesAscii: NameMatcher::Normal,
        nameLength: NameLength::Normal,
        skipS: WhitespaceSkipper::Normal,
        getAtts: AttributeScanner::Normal,
        charRefNumber: CharRefNumberDecoder::Normal,
        predefinedEntityName: PredefinedEntityNameMatcher::Normal,
        updatePosition: PositionUpdater::Normal,
        isPublicId: PublicIdChecker::Normal,
        utf8Convert: Utf8Converter::Utf8,
        utf16Convert: Utf16Converter::Utf8,
        minBytesPerChar: 1 as ::core::ffi::c_int,
        isUtf8: 1 as ::core::ffi::c_char,
        isUtf16: 0 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: Name2Checker::Utf8,
    isName3: Name3Checker::Utf8,
    isName4: Name4Checker::Never,
    isNmstrt2: NameStart2Checker::Utf8,
    isNmstrt3: NameStart3Checker::Utf8,
    isNmstrt4: NameStart4Checker::Never,
    invalid2: Invalid2Checker::Utf8,
    invalid3: Invalid3Checker::Utf8,
    invalid4: Invalid4Checker::Utf8,
};

static mut internal_utf8_encoding: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            crate::src::xmltok::Scanner::NormalProlog,
            crate::src::xmltok::Scanner::NormalContent,
            crate::src::xmltok::Scanner::NormalCdataSection,
            crate::src::xmltok::Scanner::NormalIgnoreSection,
        ],
        literalScanners: [
            LiteralScanner::NormalAttributeValue,
            LiteralScanner::NormalEntityValue,
        ],
        nameMatchesAscii: NameMatcher::Normal,
        nameLength: NameLength::Normal,
        skipS: WhitespaceSkipper::Normal,
        getAtts: AttributeScanner::Normal,
        charRefNumber: CharRefNumberDecoder::Normal,
        predefinedEntityName: PredefinedEntityNameMatcher::Normal,
        updatePosition: PositionUpdater::Normal,
        isPublicId: PublicIdChecker::Normal,
        utf8Convert: Utf8Converter::Utf8,
        utf16Convert: Utf16Converter::Utf8,
        minBytesPerChar: 1 as ::core::ffi::c_int,
        isUtf8: 1 as ::core::ffi::c_char,
        isUtf16: 0 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: Name2Checker::Utf8,
    isName3: Name3Checker::Utf8,
    isName4: Name4Checker::Never,
    isNmstrt2: NameStart2Checker::Utf8,
    isNmstrt3: NameStart3Checker::Utf8,
    isNmstrt4: NameStart4Checker::Never,
    invalid2: Invalid2Checker::Utf8,
    invalid3: Invalid3Checker::Utf8,
    invalid4: Invalid4Checker::Utf8,
};

unsafe extern "C" fn latin1_toUtf8(
    _enc: *const crate::src::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_char,
    mut toLim: *const ::core::ffi::c_char,
) -> crate::src::xmltok::XML_Convert_Result {
    loop {
        let mut c: ::core::ffi::c_uchar = 0;
        if *fromP == fromLim {
            return crate::src::xmltok::XML_CONVERT_COMPLETED;
        }
        c = **fromP as ::core::ffi::c_uchar;
        if c as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
            if toLim.offset_from(*toP) < 2 as isize {
                return crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
            }
            let c2rust_fresh6 = *toP;
            *toP = (*toP).offset(1);
            *c2rust_fresh6 = (c as ::core::ffi::c_int >> 6 as ::core::ffi::c_int
                | UTF8_cval2 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            let c2rust_fresh7 = *toP;
            *toP = (*toP).offset(1);
            *c2rust_fresh7 = (c as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int
                | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
            *fromP = (*fromP).offset(1);
        } else {
            if *toP == toLim as *mut ::core::ffi::c_char {
                return crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
            }
            let c2rust_fresh8 = *fromP;
            *fromP = (*fromP).offset(1);
            let c2rust_fresh9 = *toP;
            *toP = (*toP).offset(1);
            *c2rust_fresh9 = *c2rust_fresh8;
        }
    }
}

unsafe extern "C" fn latin1_toUtf16(
    _enc: *const crate::src::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_ushort,
    mut toLim: *const ::core::ffi::c_ushort,
) -> crate::src::xmltok::XML_Convert_Result {
    while *fromP < fromLim && *toP < toLim as *mut ::core::ffi::c_ushort {
        let c2rust_fresh4 = *fromP;
        *fromP = (*fromP).offset(1);
        let c2rust_fresh5 = *toP;
        *toP = (*toP).offset(1);
        *c2rust_fresh5 = *c2rust_fresh4 as ::core::ffi::c_uchar as ::core::ffi::c_ushort;
    }
    if *toP == toLim as *mut ::core::ffi::c_ushort && *fromP < fromLim {
        return crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
    } else {
        return crate::src::xmltok::XML_CONVERT_COMPLETED;
    };
}

static mut latin1_encoding_ns: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            crate::src::xmltok::Scanner::NormalProlog,
            crate::src::xmltok::Scanner::NormalContent,
            crate::src::xmltok::Scanner::NormalCdataSection,
            crate::src::xmltok::Scanner::NormalIgnoreSection,
        ],
        literalScanners: [
            LiteralScanner::NormalAttributeValue,
            LiteralScanner::NormalEntityValue,
        ],
        nameMatchesAscii: NameMatcher::Normal,
        nameLength: NameLength::Normal,
        skipS: WhitespaceSkipper::Normal,
        getAtts: AttributeScanner::Normal,
        charRefNumber: CharRefNumberDecoder::Normal,
        predefinedEntityName: PredefinedEntityNameMatcher::Normal,
        updatePosition: PositionUpdater::Normal,
        isPublicId: PublicIdChecker::Normal,
        utf8Convert: Utf8Converter::Latin1,
        utf16Convert: Utf16Converter::Latin1,
        minBytesPerChar: 1 as ::core::ffi::c_int,
        isUtf8: 0 as ::core::ffi::c_char,
        isUtf16: 0 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: Name2Checker::Never,
    isName3: Name3Checker::Never,
    isName4: Name4Checker::Never,
    isNmstrt2: NameStart2Checker::Never,
    isNmstrt3: NameStart3Checker::Never,
    isNmstrt4: NameStart4Checker::Never,
    invalid2: Invalid2Checker::Never,
    invalid3: Invalid3Checker::Never,
    invalid4: Invalid4Checker::Never,
};

static latin1_encoding: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            crate::src::xmltok::Scanner::NormalProlog,
            crate::src::xmltok::Scanner::NormalContent,
            crate::src::xmltok::Scanner::NormalCdataSection,
            crate::src::xmltok::Scanner::NormalIgnoreSection,
        ],
        literalScanners: [
            LiteralScanner::NormalAttributeValue,
            LiteralScanner::NormalEntityValue,
        ],
        nameMatchesAscii: NameMatcher::Normal,
        nameLength: NameLength::Normal,
        skipS: WhitespaceSkipper::Normal,
        getAtts: AttributeScanner::Normal,
        charRefNumber: CharRefNumberDecoder::Normal,
        predefinedEntityName: PredefinedEntityNameMatcher::Normal,
        updatePosition: PositionUpdater::Normal,
        isPublicId: PublicIdChecker::Normal,
        utf8Convert: Utf8Converter::Latin1,
        utf16Convert: Utf16Converter::Latin1,
        minBytesPerChar: 1 as ::core::ffi::c_int,
        isUtf8: 0 as ::core::ffi::c_char,
        isUtf16: 0 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: Name2Checker::Never,
    isName3: Name3Checker::Never,
    isName4: Name4Checker::Never,
    isNmstrt2: NameStart2Checker::Never,
    isNmstrt3: NameStart3Checker::Never,
    isNmstrt4: NameStart4Checker::Never,
    invalid2: Invalid2Checker::Never,
    invalid3: Invalid3Checker::Never,
    invalid4: Invalid4Checker::Never,
};

unsafe extern "C" fn ascii_toUtf8(
    _enc: *const crate::src::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_char,
    mut toLim: *const ::core::ffi::c_char,
) -> crate::src::xmltok::XML_Convert_Result {
    while *fromP < fromLim && *toP < toLim as *mut ::core::ffi::c_char {
        let c2rust_fresh32 = *fromP;
        *fromP = (*fromP).offset(1);
        let c2rust_fresh33 = *toP;
        *toP = (*toP).offset(1);
        *c2rust_fresh33 = *c2rust_fresh32;
    }
    if *toP == toLim as *mut ::core::ffi::c_char && *fromP < fromLim {
        return crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
    } else {
        return crate::src::xmltok::XML_CONVERT_COMPLETED;
    };
}

static mut ascii_encoding_ns: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            crate::src::xmltok::Scanner::NormalProlog,
            crate::src::xmltok::Scanner::NormalContent,
            crate::src::xmltok::Scanner::NormalCdataSection,
            crate::src::xmltok::Scanner::NormalIgnoreSection,
        ],
        literalScanners: [
            LiteralScanner::NormalAttributeValue,
            LiteralScanner::NormalEntityValue,
        ],
        nameMatchesAscii: NameMatcher::Normal,
        nameLength: NameLength::Normal,
        skipS: WhitespaceSkipper::Normal,
        getAtts: AttributeScanner::Normal,
        charRefNumber: CharRefNumberDecoder::Normal,
        predefinedEntityName: PredefinedEntityNameMatcher::Normal,
        updatePosition: PositionUpdater::Normal,
        isPublicId: PublicIdChecker::Normal,
        utf8Convert: Utf8Converter::Ascii,
        utf16Convert: Utf16Converter::Latin1,
        minBytesPerChar: 1 as ::core::ffi::c_int,
        isUtf8: 1 as ::core::ffi::c_char,
        isUtf16: 0 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
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
    isName2: Name2Checker::Never,
    isName3: Name3Checker::Never,
    isName4: Name4Checker::Never,
    isNmstrt2: NameStart2Checker::Never,
    isNmstrt3: NameStart3Checker::Never,
    isNmstrt4: NameStart4Checker::Never,
    invalid2: Invalid2Checker::Never,
    invalid3: Invalid3Checker::Never,
    invalid4: Invalid4Checker::Never,
};

static mut ascii_encoding: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            crate::src::xmltok::Scanner::NormalProlog,
            crate::src::xmltok::Scanner::NormalContent,
            crate::src::xmltok::Scanner::NormalCdataSection,
            crate::src::xmltok::Scanner::NormalIgnoreSection,
        ],
        literalScanners: [
            LiteralScanner::NormalAttributeValue,
            LiteralScanner::NormalEntityValue,
        ],
        nameMatchesAscii: NameMatcher::Normal,
        nameLength: NameLength::Normal,
        skipS: WhitespaceSkipper::Normal,
        getAtts: AttributeScanner::Normal,
        charRefNumber: CharRefNumberDecoder::Normal,
        predefinedEntityName: PredefinedEntityNameMatcher::Normal,
        updatePosition: PositionUpdater::Normal,
        isPublicId: PublicIdChecker::Normal,
        utf8Convert: Utf8Converter::Ascii,
        utf16Convert: Utf16Converter::Latin1,
        minBytesPerChar: 1 as ::core::ffi::c_int,
        isUtf8: 1 as ::core::ffi::c_char,
        isUtf16: 0 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
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
    isName2: Name2Checker::Never,
    isName3: Name3Checker::Never,
    isName4: Name4Checker::Never,
    isNmstrt2: NameStart2Checker::Never,
    isNmstrt3: NameStart3Checker::Never,
    isNmstrt4: NameStart4Checker::Never,
    invalid2: Invalid2Checker::Never,
    invalid3: Invalid3Checker::Never,
    invalid4: Invalid4Checker::Never,
};

fn unicode_byte_type(hi: ::core::ffi::c_char, lo: ::core::ffi::c_char) -> ::core::ffi::c_int {
    match hi as ::core::ffi::c_uchar as ::core::ffi::c_int {
        216 | 217 | 218 | 219 => return crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int,
        220 | 221 | 222 | 223 => return crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int,
        255 => match lo as ::core::ffi::c_uchar as ::core::ffi::c_int {
            255 | 254 => return crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int,
            _ => {}
        },
        _ => {}
    }
    return crate::xmltok_impl_h::BT_NONASCII as ::core::ffi::c_int;
}

fn little2_to_utf8_window(
    input: &[u8],
    output: &mut [u8],
) -> (crate::src::xmltok::XML_Convert_Result, usize, usize) {
    let input = &input[..input.len() & !1];
    let mut input_offset = 0;
    let mut output_offset = 0;

    while input_offset < input.len() {
        let lo = input[input_offset];
        let hi = input[input_offset + 1];
        let output_len = match hi {
            0 if lo < 0x80 => 1,
            0xd8..=0xdb => 4,
            0..=7 => 2,
            _ => 3,
        };
        if output.len() - output_offset < output_len {
            return (
                crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED,
                input_offset,
                output_offset,
            );
        }

        match output_len {
            1 => output[output_offset] = lo,
            2 => {
                output[output_offset] = (lo >> 6) | (hi << 2) | UTF8_cval2 as u8;
                output[output_offset + 1] = (lo & 0x3f) | 0x80;
            }
            3 => {
                output[output_offset] = (hi >> 4) | UTF8_cval3 as u8;
                output[output_offset + 1] = ((hi & 0x0f) << 2) | (lo >> 6) | 0x80;
                output[output_offset + 2] = (lo & 0x3f) | 0x80;
            }
            4 => {
                if input.len() - input_offset < 4 {
                    return (
                        crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE,
                        input_offset,
                        output_offset,
                    );
                }
                let plane = (((hi & 3) << 2) | ((lo >> 6) & 3)) + 1;
                let next_lo = input[input_offset + 2];
                let next_hi = input[input_offset + 3];
                output[output_offset] = (plane >> 2) | UTF8_cval4 as u8;
                output[output_offset + 1] = ((lo >> 2) & 0x0f) | ((plane & 3) << 4) | 0x80;
                output[output_offset + 2] =
                    ((lo & 3) << 4) | ((next_hi & 3) << 2) | (next_lo >> 6) | 0x80;
                output[output_offset + 3] = (next_lo & 0x3f) | 0x80;
                input_offset += 2;
            }
            _ => unreachable!(),
        }
        input_offset += 2;
        output_offset += output_len;
    }

    (
        crate::src::xmltok::XML_CONVERT_COMPLETED,
        input_offset,
        output_offset,
    )
}

unsafe extern "C" fn little2_toUtf8(
    _enc: *const crate::src::xmltok::ENCODING,
    fromP: *mut *const ::core::ffi::c_char,
    fromLim: *const ::core::ffi::c_char,
    toP: *mut *mut ::core::ffi::c_char,
    toLim: *const ::core::ffi::c_char,
) -> crate::src::xmltok::XML_Convert_Result {
    let input_start = *fromP;
    let input_len = fromLim.offset_from(input_start) as usize;
    let output_start = *toP;
    let output_len = toLim.offset_from(output_start) as usize;
    // The converter's callers provide two disjoint, bounded C byte windows.
    let input = core::slice::from_raw_parts(input_start.cast::<u8>(), input_len);
    let output = core::slice::from_raw_parts_mut(output_start.cast::<u8>(), output_len);
    let (result, input_used, output_used) = little2_to_utf8_window(input, output);
    *fromP = input_start.add(input_used);
    *toP = output_start.add(output_used);
    result
}

unsafe extern "C" fn little2_toUtf16(
    _enc: *const crate::src::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_ushort,
    mut toLim: *const ::core::ffi::c_ushort,
) -> crate::src::xmltok::XML_Convert_Result {
    let mut res: crate::src::xmltok::XML_Convert_Result = crate::src::xmltok::XML_CONVERT_COMPLETED;
    fromLim = (*fromP).offset(
        ((fromLim.offset_from(*fromP) >> 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
            as isize,
    );
    if fromLim.offset_from(*fromP) > toLim.offset_from(*toP) << 1 as ::core::ffi::c_int
        && *fromLim
            .offset(-(2 as ::core::ffi::c_int as isize))
            .offset(1 as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int
            & 0xf8 as ::core::ffi::c_int
            == 0xd8 as ::core::ffi::c_int
    {
        fromLim = fromLim.offset(-(2 as ::core::ffi::c_int as isize));
        res = crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
    }
    while *fromP < fromLim && *toP < toLim as *mut ::core::ffi::c_ushort {
        let c2rust_fresh10 = *toP;
        *toP = (*toP).offset(1);
        *c2rust_fresh10 = ((*(*fromP).offset(1 as isize) as ::core::ffi::c_uchar
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *(*fromP).offset(0 as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int)
            as ::core::ffi::c_ushort;
        *fromP = (*fromP).offset(2 as ::core::ffi::c_int as isize);
    }
    if *toP == toLim as *mut ::core::ffi::c_ushort && *fromP < fromLim {
        return crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
    } else {
        return res;
    };
}

fn big2_to_utf8_window(
    input: &[u8],
    output: &mut [u8],
) -> (crate::src::xmltok::XML_Convert_Result, usize, usize) {
    let input = &input[..input.len() & !1];
    let mut input_offset = 0;
    let mut output_offset = 0;

    while input_offset < input.len() {
        let hi = input[input_offset];
        let lo = input[input_offset + 1];
        let output_len = match hi {
            0 if lo < 0x80 => 1,
            0xd8..=0xdb => 4,
            0..=7 => 2,
            _ => 3,
        };
        if output.len() - output_offset < output_len {
            return (
                crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED,
                input_offset,
                output_offset,
            );
        }

        match output_len {
            1 => output[output_offset] = lo,
            2 => {
                output[output_offset] = (lo >> 6) | (hi << 2) | UTF8_cval2 as u8;
                output[output_offset + 1] = (lo & 0x3f) | 0x80;
            }
            3 => {
                output[output_offset] = (hi >> 4) | UTF8_cval3 as u8;
                output[output_offset + 1] = ((hi & 0x0f) << 2) | (lo >> 6) | 0x80;
                output[output_offset + 2] = (lo & 0x3f) | 0x80;
            }
            4 => {
                if input.len() - input_offset < 4 {
                    return (
                        crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE,
                        input_offset,
                        output_offset,
                    );
                }
                let plane = (((hi & 3) << 2) | ((lo >> 6) & 3)) + 1;
                let next_hi = input[input_offset + 2];
                let next_lo = input[input_offset + 3];
                output[output_offset] = (plane >> 2) | UTF8_cval4 as u8;
                output[output_offset + 1] = ((lo >> 2) & 0x0f) | ((plane & 3) << 4) | 0x80;
                output[output_offset + 2] =
                    ((lo & 3) << 4) | ((next_hi & 3) << 2) | (next_lo >> 6) | 0x80;
                output[output_offset + 3] = (next_lo & 0x3f) | 0x80;
                input_offset += 2;
            }
            _ => unreachable!(),
        }
        input_offset += 2;
        output_offset += output_len;
    }

    (
        crate::src::xmltok::XML_CONVERT_COMPLETED,
        input_offset,
        output_offset,
    )
}

unsafe extern "C" fn big2_toUtf8(
    _enc: *const crate::src::xmltok::ENCODING,
    fromP: *mut *const ::core::ffi::c_char,
    fromLim: *const ::core::ffi::c_char,
    toP: *mut *mut ::core::ffi::c_char,
    toLim: *const ::core::ffi::c_char,
) -> crate::src::xmltok::XML_Convert_Result {
    let input_start = *fromP;
    let input_len = fromLim.offset_from(input_start) as usize;
    let output_start = *toP;
    let output_len = toLim.offset_from(output_start) as usize;
    // The converter's callers provide two disjoint, bounded C byte windows.
    let input = core::slice::from_raw_parts(input_start.cast::<u8>(), input_len);
    let output = core::slice::from_raw_parts_mut(output_start.cast::<u8>(), output_len);
    let (result, input_used, output_used) = big2_to_utf8_window(input, output);
    *fromP = input_start.add(input_used);
    *toP = output_start.add(output_used);
    result
}

unsafe extern "C" fn big2_toUtf16(
    _enc: *const crate::src::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_ushort,
    mut toLim: *const ::core::ffi::c_ushort,
) -> crate::src::xmltok::XML_Convert_Result {
    let mut res: crate::src::xmltok::XML_Convert_Result = crate::src::xmltok::XML_CONVERT_COMPLETED;
    fromLim = (*fromP).offset(
        ((fromLim.offset_from(*fromP) >> 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
            as isize,
    );
    if fromLim.offset_from(*fromP) > toLim.offset_from(*toP) << 1 as ::core::ffi::c_int
        && *fromLim
            .offset(-(2 as ::core::ffi::c_int as isize))
            .offset(0 as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int
            & 0xf8 as ::core::ffi::c_int
            == 0xd8 as ::core::ffi::c_int
    {
        fromLim = fromLim.offset(-(2 as ::core::ffi::c_int as isize));
        res = crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
    }
    while *fromP < fromLim && *toP < toLim as *mut ::core::ffi::c_ushort {
        let c2rust_fresh21 = *toP;
        *toP = (*toP).offset(1);
        *c2rust_fresh21 = ((*(*fromP).offset(0 as isize) as ::core::ffi::c_uchar
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *(*fromP).offset(1 as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int)
            as ::core::ffi::c_ushort;
        *fromP = (*fromP).offset(2 as ::core::ffi::c_int as isize);
    }
    if *toP == toLim as *mut ::core::ffi::c_ushort && *fromP < fromLim {
        return crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
    } else {
        return res;
    };
}

static mut little2_encoding_ns: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            crate::src::xmltok::Scanner::Little2Prolog,
            crate::src::xmltok::Scanner::Little2Content,
            crate::src::xmltok::Scanner::Little2CdataSection,
            crate::src::xmltok::Scanner::Little2IgnoreSection,
        ],
        literalScanners: [
            LiteralScanner::Little2AttributeValue,
            LiteralScanner::Little2EntityValue,
        ],
        nameMatchesAscii: NameMatcher::Little2,
        nameLength: NameLength::Little2,
        skipS: WhitespaceSkipper::Little2,
        getAtts: AttributeScanner::Little2,
        charRefNumber: CharRefNumberDecoder::Little2,
        predefinedEntityName: PredefinedEntityNameMatcher::Little2,
        updatePosition: PositionUpdater::Little2,
        isPublicId: PublicIdChecker::Little2,
        utf8Convert: Utf8Converter::Little2,
        utf16Convert: Utf16Converter::Little2,
        minBytesPerChar: 2 as ::core::ffi::c_int,
        isUtf8: 0 as ::core::ffi::c_char,
        isUtf16: 1 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: Name2Checker::Never,
    isName3: Name3Checker::Never,
    isName4: Name4Checker::Never,
    isNmstrt2: NameStart2Checker::Never,
    isNmstrt3: NameStart3Checker::Never,
    isNmstrt4: NameStart4Checker::Never,
    invalid2: Invalid2Checker::Never,
    invalid3: Invalid3Checker::Never,
    invalid4: Invalid4Checker::Never,
};

static mut little2_encoding: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            crate::src::xmltok::Scanner::Little2Prolog,
            crate::src::xmltok::Scanner::Little2Content,
            crate::src::xmltok::Scanner::Little2CdataSection,
            crate::src::xmltok::Scanner::Little2IgnoreSection,
        ],
        literalScanners: [
            LiteralScanner::Little2AttributeValue,
            LiteralScanner::Little2EntityValue,
        ],
        nameMatchesAscii: NameMatcher::Little2,
        nameLength: NameLength::Little2,
        skipS: WhitespaceSkipper::Little2,
        getAtts: AttributeScanner::Little2,
        charRefNumber: CharRefNumberDecoder::Little2,
        predefinedEntityName: PredefinedEntityNameMatcher::Little2,
        updatePosition: PositionUpdater::Little2,
        isPublicId: PublicIdChecker::Little2,
        utf8Convert: Utf8Converter::Little2,
        utf16Convert: Utf16Converter::Little2,
        minBytesPerChar: 2 as ::core::ffi::c_int,
        isUtf8: 0 as ::core::ffi::c_char,
        isUtf16: 1 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: Name2Checker::Never,
    isName3: Name3Checker::Never,
    isName4: Name4Checker::Never,
    isNmstrt2: NameStart2Checker::Never,
    isNmstrt3: NameStart3Checker::Never,
    isNmstrt4: NameStart4Checker::Never,
    invalid2: Invalid2Checker::Never,
    invalid3: Invalid3Checker::Never,
    invalid4: Invalid4Checker::Never,
};

static mut internal_little2_encoding_ns: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            crate::src::xmltok::Scanner::Little2Prolog,
            crate::src::xmltok::Scanner::Little2Content,
            crate::src::xmltok::Scanner::Little2CdataSection,
            crate::src::xmltok::Scanner::Little2IgnoreSection,
        ],
        literalScanners: [
            LiteralScanner::Little2AttributeValue,
            LiteralScanner::Little2EntityValue,
        ],
        nameMatchesAscii: NameMatcher::Little2,
        nameLength: NameLength::Little2,
        skipS: WhitespaceSkipper::Little2,
        getAtts: AttributeScanner::Little2,
        charRefNumber: CharRefNumberDecoder::Little2,
        predefinedEntityName: PredefinedEntityNameMatcher::Little2,
        updatePosition: PositionUpdater::Little2,
        isPublicId: PublicIdChecker::Little2,
        utf8Convert: Utf8Converter::Little2,
        utf16Convert: Utf16Converter::Little2,
        minBytesPerChar: 2 as ::core::ffi::c_int,
        isUtf8: 0 as ::core::ffi::c_char,
        isUtf16: 1 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: Name2Checker::Never,
    isName3: Name3Checker::Never,
    isName4: Name4Checker::Never,
    isNmstrt2: NameStart2Checker::Never,
    isNmstrt3: NameStart3Checker::Never,
    isNmstrt4: NameStart4Checker::Never,
    invalid2: Invalid2Checker::Never,
    invalid3: Invalid3Checker::Never,
    invalid4: Invalid4Checker::Never,
};

static mut internal_little2_encoding: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            crate::src::xmltok::Scanner::Little2Prolog,
            crate::src::xmltok::Scanner::Little2Content,
            crate::src::xmltok::Scanner::Little2CdataSection,
            crate::src::xmltok::Scanner::Little2IgnoreSection,
        ],
        literalScanners: [
            LiteralScanner::Little2AttributeValue,
            LiteralScanner::Little2EntityValue,
        ],
        nameMatchesAscii: NameMatcher::Little2,
        nameLength: NameLength::Little2,
        skipS: WhitespaceSkipper::Little2,
        getAtts: AttributeScanner::Little2,
        charRefNumber: CharRefNumberDecoder::Little2,
        predefinedEntityName: PredefinedEntityNameMatcher::Little2,
        updatePosition: PositionUpdater::Little2,
        isPublicId: PublicIdChecker::Little2,
        utf8Convert: Utf8Converter::Little2,
        utf16Convert: Utf16Converter::Little2,
        minBytesPerChar: 2 as ::core::ffi::c_int,
        isUtf8: 0 as ::core::ffi::c_char,
        isUtf16: 1 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: Name2Checker::Never,
    isName3: Name3Checker::Never,
    isName4: Name4Checker::Never,
    isNmstrt2: NameStart2Checker::Never,
    isNmstrt3: NameStart3Checker::Never,
    isNmstrt4: NameStart4Checker::Never,
    invalid2: Invalid2Checker::Never,
    invalid3: Invalid3Checker::Never,
    invalid4: Invalid4Checker::Never,
};

static mut big2_encoding_ns: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            crate::src::xmltok::Scanner::Big2Prolog,
            crate::src::xmltok::Scanner::Big2Content,
            crate::src::xmltok::Scanner::Big2CdataSection,
            crate::src::xmltok::Scanner::Big2IgnoreSection,
        ],
        literalScanners: [
            LiteralScanner::Big2AttributeValue,
            LiteralScanner::Big2EntityValue,
        ],
        nameMatchesAscii: NameMatcher::Big2,
        nameLength: NameLength::Big2,
        skipS: WhitespaceSkipper::Big2,
        getAtts: AttributeScanner::Big2,
        charRefNumber: CharRefNumberDecoder::Big2,
        predefinedEntityName: PredefinedEntityNameMatcher::Big2,
        updatePosition: PositionUpdater::Big2,
        isPublicId: PublicIdChecker::Big2,
        utf8Convert: Utf8Converter::Big2,
        utf16Convert: Utf16Converter::Big2,
        minBytesPerChar: 2 as ::core::ffi::c_int,
        isUtf8: 0 as ::core::ffi::c_char,
        isUtf16: 0 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: Name2Checker::Never,
    isName3: Name3Checker::Never,
    isName4: Name4Checker::Never,
    isNmstrt2: NameStart2Checker::Never,
    isNmstrt3: NameStart3Checker::Never,
    isNmstrt4: NameStart4Checker::Never,
    invalid2: Invalid2Checker::Never,
    invalid3: Invalid3Checker::Never,
    invalid4: Invalid4Checker::Never,
};

static mut big2_encoding: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            crate::src::xmltok::Scanner::Big2Prolog,
            crate::src::xmltok::Scanner::Big2Content,
            crate::src::xmltok::Scanner::Big2CdataSection,
            crate::src::xmltok::Scanner::Big2IgnoreSection,
        ],
        literalScanners: [
            LiteralScanner::Big2AttributeValue,
            LiteralScanner::Big2EntityValue,
        ],
        nameMatchesAscii: NameMatcher::Big2,
        nameLength: NameLength::Big2,
        skipS: WhitespaceSkipper::Big2,
        getAtts: AttributeScanner::Big2,
        charRefNumber: CharRefNumberDecoder::Big2,
        predefinedEntityName: PredefinedEntityNameMatcher::Big2,
        updatePosition: PositionUpdater::Big2,
        isPublicId: PublicIdChecker::Big2,
        utf8Convert: Utf8Converter::Big2,
        utf16Convert: Utf16Converter::Big2,
        minBytesPerChar: 2 as ::core::ffi::c_int,
        isUtf8: 0 as ::core::ffi::c_char,
        isUtf16: 0 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: Name2Checker::Never,
    isName3: Name3Checker::Never,
    isName4: Name4Checker::Never,
    isNmstrt2: NameStart2Checker::Never,
    isNmstrt3: NameStart3Checker::Never,
    isNmstrt4: NameStart4Checker::Never,
    invalid2: Invalid2Checker::Never,
    invalid3: Invalid3Checker::Never,
    invalid4: Invalid4Checker::Never,
};

unsafe extern "C" fn streqci(
    mut s1: *const ::core::ffi::c_char,
    mut s2: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    loop {
        let c2rust_fresh34 = s1;
        s1 = s1.offset(1);
        let mut c1: ::core::ffi::c_char = *c2rust_fresh34;
        let c2rust_fresh35 = s2;
        s2 = s2.offset(1);
        let mut c2: ::core::ffi::c_char = *c2rust_fresh35;
        if crate::ascii_h::ASCII_a_1 <= c1 as ::core::ffi::c_int
            && c1 as ::core::ffi::c_int <= crate::ascii_h::ASCII_z
        {
            c1 = (c1 as ::core::ffi::c_int + (crate::ascii_h::ASCII_A - crate::ascii_h::ASCII_a_1))
                as ::core::ffi::c_char;
        }
        if crate::ascii_h::ASCII_a_1 <= c2 as ::core::ffi::c_int
            && c2 as ::core::ffi::c_int <= crate::ascii_h::ASCII_z
        {
            c2 = (c2 as ::core::ffi::c_int + (crate::ascii_h::ASCII_A - crate::ascii_h::ASCII_a_1))
                as ::core::ffi::c_char;
        }
        if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if c1 == 0 {
            break;
        }
    }
    return 1 as ::core::ffi::c_int;
}

pub(crate) unsafe fn initUpdatePosition(
    updater: crate::src::xmltok::PositionUpdater,
    _enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut pos: *mut crate::src::xmltok::POSITION,
) {
    match updater {
        crate::src::xmltok::PositionUpdater::Init => {
            normal_updatePosition(&raw const utf8_encoding.enc, ptr, end, pos)
        }
        crate::src::xmltok::PositionUpdater::Normal => normal_updatePosition(_enc, ptr, end, pos),
        crate::src::xmltok::PositionUpdater::Little2 => {
            crate::src::xmltok::xmltok_impl_c::little2_updatePosition(_enc, ptr, end, pos)
        }
        crate::src::xmltok::PositionUpdater::Big2 => {
            crate::src::xmltok::xmltok_impl_c::big2_updatePosition(_enc, ptr, end, pos)
        }
    }
}

const XML_DECL_VERSION: &[u8] = b"version";
const XML_DECL_ENCODING: &[u8] = b"encoding";
const XML_DECL_STANDALONE: &[u8] = b"standalone";
const XML_DECL_YES: &[u8] = b"yes";
const XML_DECL_NO: &[u8] = b"no";

#[derive(Clone)]
struct XmlDeclAttribute {
    name: core::ops::Range<usize>,
    value: core::ops::Range<usize>,
    next: usize,
}

struct XmlDeclResult {
    version: Option<core::ops::Range<usize>>,
    version_end: Option<usize>,
    encoding_name: Option<core::ops::Range<usize>>,
    encoding_end: Option<usize>,
    standalone: Option<::core::ffi::c_int>,
}

fn xml_decl_ascii_at(enc: &encoding, input: &[u8], offset: usize) -> Option<u8> {
    let width = usize::try_from(enc.minBytesPerChar).ok()?;
    let bytes = input.get(offset..offset.checked_add(width)?)?;
    match enc.nameMatchesAscii {
        NameMatcher::Normal => bytes.first().copied(),
        NameMatcher::Little2 if bytes.len() == 2 && bytes[1] == 0 => Some(bytes[0]),
        NameMatcher::Big2 if bytes.len() == 2 && bytes[0] == 0 => Some(bytes[1]),
        NameMatcher::Little2 | NameMatcher::Big2 => Some(u8::MAX),
    }
}

fn xml_decl_matches_ascii(
    enc: &encoding,
    input: &[u8],
    span: core::ops::Range<usize>,
    word: &[u8],
) -> bool {
    let width = match usize::try_from(enc.minBytesPerChar) {
        Ok(width) => width,
        Err(_) => return false,
    };
    if span.len() != word.len().saturating_mul(width) {
        return false;
    }
    word.iter().enumerate().all(|(index, &expected)| {
        xml_decl_ascii_at(enc, input, span.start + index * width) == Some(expected)
    })
}

fn xml_decl_is_space(character: Option<u8>) -> bool {
    matches!(character, Some(b' ' | b'\r' | b'\n' | b'\t'))
}

fn parse_xml_decl_pseudo_attribute(
    enc: &encoding,
    input: &[u8],
    mut cursor: usize,
    end: usize,
) -> Result<Option<XmlDeclAttribute>, usize> {
    let width = usize::try_from(enc.minBytesPerChar).map_err(|_| cursor)?;
    if cursor == end {
        return Ok(None);
    }
    if !xml_decl_is_space(xml_decl_ascii_at(enc, input, cursor)) {
        return Err(cursor);
    }
    loop {
        cursor = cursor.checked_add(width).ok_or(cursor)?;
        if !xml_decl_is_space(xml_decl_ascii_at(enc, input, cursor)) {
            break;
        }
    }
    if cursor == end {
        return Ok(None);
    }

    let name_start = cursor;
    let name_end;
    loop {
        match xml_decl_ascii_at(enc, input, cursor) {
            None => return Err(cursor),
            Some(b'=') => {
                name_end = cursor;
                break;
            }
            character if xml_decl_is_space(character) => {
                name_end = cursor;
                loop {
                    cursor = cursor.checked_add(width).ok_or(cursor)?;
                    let character = xml_decl_ascii_at(enc, input, cursor);
                    if !xml_decl_is_space(character) {
                        if character != Some(b'=') {
                            return Err(cursor);
                        }
                        break;
                    }
                }
                break;
            }
            Some(_) => cursor = cursor.checked_add(width).ok_or(cursor)?,
        }
    }
    if cursor == name_start {
        return Err(cursor);
    }

    cursor = cursor.checked_add(width).ok_or(cursor)?;
    while xml_decl_is_space(xml_decl_ascii_at(enc, input, cursor)) {
        cursor = cursor.checked_add(width).ok_or(cursor)?;
    }
    let quote = match xml_decl_ascii_at(enc, input, cursor) {
        Some(quote @ (b'\'' | b'"')) => quote,
        _ => return Err(cursor),
    };
    cursor = cursor.checked_add(width).ok_or(cursor)?;
    let value_start = cursor;
    loop {
        let character = xml_decl_ascii_at(enc, input, cursor).ok_or(cursor)?;
        if character == quote {
            break;
        }
        if !matches!(
            character,
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'.' | b'-' | b'_'
        ) {
            return Err(cursor);
        }
        cursor = cursor.checked_add(width).ok_or(cursor)?;
    }
    let next = cursor.checked_add(width).ok_or(cursor)?;
    Ok(Some(XmlDeclAttribute {
        name: name_start..name_end,
        value: value_start..cursor,
        next,
    }))
}

fn parse_xml_decl(
    is_general_text_entity: bool,
    enc: &encoding,
    input: &[u8],
) -> Result<XmlDeclResult, usize> {
    let width = usize::try_from(enc.minBytesPerChar).map_err(|_| 0usize)?;
    let start = width.checked_mul(5).ok_or(0usize)?;
    let suffix = width.checked_mul(2).ok_or(0usize)?;
    let end = input.len().checked_sub(suffix).ok_or(input.len())?;
    if start > end {
        return Err(input.len());
    }
    let mut cursor = start;
    let mut result = XmlDeclResult {
        version: None,
        version_end: None,
        encoding_name: None,
        encoding_end: None,
        standalone: None,
    };

    let mut attribute = parse_xml_decl_pseudo_attribute(enc, input, cursor, end)?;
    let Some(ref first) = attribute else {
        return Err(cursor);
    };
    if xml_decl_matches_ascii(enc, input, first.name.clone(), XML_DECL_VERSION) {
        result.version = Some(first.value.clone());
        result.version_end = Some(first.next);
        cursor = first.next;
        attribute = parse_xml_decl_pseudo_attribute(enc, input, cursor, end)?;
        if attribute.is_none() {
            return if is_general_text_entity {
                Err(cursor)
            } else {
                Ok(result)
            };
        }
    } else if !is_general_text_entity {
        return Err(first.name.start);
    }

    let current = attribute
        .as_ref()
        .expect("XML declaration attribute is present");
    if xml_decl_matches_ascii(enc, input, current.name.clone(), XML_DECL_ENCODING) {
        let value_start = current.value.start;
        if !matches!(
            xml_decl_ascii_at(enc, input, value_start),
            Some(b'a'..=b'z' | b'A'..=b'Z')
        ) {
            return Err(value_start);
        }
        result.encoding_name = Some(current.value.clone());
        result.encoding_end = current.next.checked_sub(width);
        cursor = current.next;
        attribute = parse_xml_decl_pseudo_attribute(enc, input, cursor, end)?;
        if attribute.is_none() {
            return Ok(result);
        }
    }

    let attribute = attribute.expect("XML declaration attribute is present");
    if is_general_text_entity
        || !xml_decl_matches_ascii(enc, input, attribute.name.clone(), XML_DECL_STANDALONE)
    {
        return Err(attribute.name.start);
    }
    if xml_decl_matches_ascii(enc, input, attribute.value.clone(), XML_DECL_YES) {
        result.standalone = Some(1);
    } else if xml_decl_matches_ascii(enc, input, attribute.value.clone(), XML_DECL_NO) {
        result.standalone = Some(0);
    } else {
        return Err(attribute.value.start);
    }
    cursor = attribute.next;
    while xml_decl_is_space(xml_decl_ascii_at(enc, input, cursor)) {
        cursor = cursor.checked_add(width).ok_or(cursor)?;
    }
    if cursor == end {
        Ok(result)
    } else {
        Err(cursor)
    }
}

unsafe extern "C" fn doParseXmlDecl(
    mut encodingFinder: Option<
        unsafe extern "C" fn(
            *const crate::src::xmltok::ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> *const crate::src::xmltok::ENCODING,
    >,
    mut isGeneralTextEntity: ::core::ffi::c_int,
    mut enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut badPtr: *mut *const ::core::ffi::c_char,
    mut versionPtr: *mut *const ::core::ffi::c_char,
    mut versionEndPtr: *mut *const ::core::ffi::c_char,
    mut encodingName: *mut *const ::core::ffi::c_char,
    mut encoding: *mut *const crate::src::xmltok::ENCODING,
    mut standalone: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if enc.is_null() || ptr.is_null() || end.addr() < ptr.addr() {
        return 0;
    }
    let input = core::slice::from_raw_parts(ptr.cast::<u8>(), end.addr() - ptr.addr());
    let result = match parse_xml_decl(isGeneralTextEntity != 0, &*enc, input) {
        Ok(result) => result,
        Err(offset) => {
            if let Some(bad) = badPtr.as_mut() {
                *bad = ptr.wrapping_add(offset.min(input.len()));
            }
            return 0;
        }
    };
    if let (Some(range), Some(version)) = (result.version, versionPtr.as_mut()) {
        *version = ptr.wrapping_add(range.start);
    }
    if let (Some(offset), Some(version_end)) = (result.version_end, versionEndPtr.as_mut()) {
        *version_end = ptr.wrapping_add(offset);
    }
    if let (Some(range), Some(name)) = (result.encoding_name.clone(), encodingName.as_mut()) {
        *name = ptr.wrapping_add(range.start);
    }
    if let (Some(range), Some(value_end), Some(found_encoding)) =
        (result.encoding_name, result.encoding_end, encoding.as_mut())
    {
        *found_encoding = encodingFinder.expect("non-null function pointer")(
            enc,
            ptr.wrapping_add(range.start),
            ptr.wrapping_add(value_end),
        );
    }
    if let (Some(value), Some(output)) = (result.standalone, standalone.as_mut()) {
        *output = value;
    }
    1
}

fn checkCharRefNumber(mut result: ::core::ffi::c_int) -> ::core::ffi::c_int {
    match result >> 8 as ::core::ffi::c_int {
        216 | 217 | 218 | 219 | 220 | 221 | 222 | 223 => return -1 as ::core::ffi::c_int,
        0 => {
            if latin1_encoding.type_0[result as usize] as ::core::ffi::c_int
                == crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int
            {
                return -1 as ::core::ffi::c_int;
            }
        }
        255 => {
            if result == 0xfffe as ::core::ffi::c_int || result == 0xffff as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
        }
        _ => {}
    }
    return result;
}
pub unsafe extern "C" fn XmlUtf8Encode(
    mut c: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if c < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if c < min2 as ::core::ffi::c_int {
        *buf.offset(0 as isize) = (c | UTF8_cval1 as ::core::ffi::c_int) as ::core::ffi::c_char;
        return 1 as ::core::ffi::c_int;
    }
    if c < min3 as ::core::ffi::c_int {
        *buf.offset(0 as isize) = (c >> 6 as ::core::ffi::c_int | UTF8_cval2 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        *buf.offset(1 as isize) =
            (c & 0x3f as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
        return 2 as ::core::ffi::c_int;
    }
    if c < min4 as ::core::ffi::c_int {
        *buf.offset(0 as isize) = (c >> 12 as ::core::ffi::c_int | UTF8_cval3 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        *buf.offset(1 as isize) = (c >> 6 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int
            | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
        *buf.offset(2 as isize) =
            (c & 0x3f as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
        return 3 as ::core::ffi::c_int;
    }
    if c < 0x110000 as ::core::ffi::c_int {
        *buf.offset(0 as isize) = (c >> 18 as ::core::ffi::c_int | UTF8_cval4 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        *buf.offset(1 as isize) = (c >> 12 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int
            | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
        *buf.offset(2 as isize) = (c >> 6 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int
            | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
        *buf.offset(3 as isize) =
            (c & 0x3f as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
        return 4 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[export_name = "XmlUtf8Encode"]

pub unsafe extern "C" fn XmlUtf8Encode_ffi(
    mut c: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    XmlUtf8Encode(c, buf)
}
pub unsafe extern "C" fn XmlUtf16Encode(
    mut charNum: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_ushort,
) -> ::core::ffi::c_int {
    if charNum < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if charNum < 0x10000 as ::core::ffi::c_int {
        *buf.offset(0 as isize) = charNum as ::core::ffi::c_ushort;
        return 1 as ::core::ffi::c_int;
    }
    if charNum < 0x110000 as ::core::ffi::c_int {
        charNum -= 0x10000 as ::core::ffi::c_int;
        *buf.offset(0 as isize) = ((charNum >> 10 as ::core::ffi::c_int)
            + 0xd800 as ::core::ffi::c_int)
            as ::core::ffi::c_ushort;
        *buf.offset(1 as isize) = ((charNum & 0x3ff as ::core::ffi::c_int)
            + 0xdc00 as ::core::ffi::c_int)
            as ::core::ffi::c_ushort;
        return 2 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[export_name = "XmlUtf16Encode"]

pub unsafe extern "C" fn XmlUtf16Encode_ffi(
    mut charNum: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_ushort,
) -> ::core::ffi::c_int {
    XmlUtf16Encode(charNum, buf)
}
pub unsafe extern "C" fn XmlSizeOfUnknownEncoding() -> ::core::ffi::c_int {
    return ::core::mem::size_of::<unknown_encoding>() as ::core::ffi::c_int;
}
#[export_name = "XmlSizeOfUnknownEncoding"]

pub unsafe extern "C" fn XmlSizeOfUnknownEncoding_ffi() -> ::core::ffi::c_int {
    XmlSizeOfUnknownEncoding()
}
unsafe extern "C" fn unknown_isName(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut uenc: *const unknown_encoding = enc as *const unknown_encoding;
    let mut c: ::core::ffi::c_int = unknown_encoding_converter((*uenc).converter_id)
        .expect("unknown encoding converter is registered")
        .invoke((*uenc).userData, p);
    if c & !(0xffff as ::core::ffi::c_int) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    return (namingBitmap[(((namePages[(c >> 8 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int)
        << 3 as ::core::ffi::c_int)
        + ((c & 0xff as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int))
        as usize]
        & (1 as ::core::ffi::c_uint)
            << (c & 0xff as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
        as ::core::ffi::c_int;
}

unsafe extern "C" fn unknown_isNmstrt(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut uenc: *const unknown_encoding = enc as *const unknown_encoding;
    let mut c: ::core::ffi::c_int = unknown_encoding_converter((*uenc).converter_id)
        .expect("unknown encoding converter is registered")
        .invoke((*uenc).userData, p);
    if c & !(0xffff as ::core::ffi::c_int) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    return (namingBitmap[(((nmstrtPages[(c >> 8 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int)
        << 3 as ::core::ffi::c_int)
        + ((c & 0xff as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int))
        as usize]
        & (1 as ::core::ffi::c_uint)
            << (c & 0xff as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
        as ::core::ffi::c_int;
}

unsafe extern "C" fn unknown_isInvalid(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut uenc: *const unknown_encoding = enc as *const unknown_encoding;
    let mut c: ::core::ffi::c_int = unknown_encoding_converter((*uenc).converter_id)
        .expect("unknown encoding converter is registered")
        .invoke((*uenc).userData, p);
    return (c & !(0xffff as ::core::ffi::c_int) != 0
        || checkCharRefNumber(c) < 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}

unsafe extern "C" fn unknown_toUtf8(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_char,
    mut toLim: *const ::core::ffi::c_char,
) -> crate::src::xmltok::XML_Convert_Result {
    let mut uenc: *const unknown_encoding = enc as *const unknown_encoding;
    let mut buf: [::core::ffi::c_char; 4] = [0; 4];
    loop {
        let mut utf8: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut n: ::core::ffi::c_int = 0;
        if *fromP == fromLim {
            return crate::src::xmltok::XML_CONVERT_COMPLETED;
        }
        utf8 = &raw const *(&raw const (*uenc).utf8 as *const [::core::ffi::c_char; 4])
            .offset(**fromP as ::core::ffi::c_uchar as isize)
            as *const ::core::ffi::c_char;
        let c2rust_fresh37 = utf8;
        utf8 = utf8.offset(1);
        n = *c2rust_fresh37 as ::core::ffi::c_int;
        if n == 0 as ::core::ffi::c_int {
            let mut c: ::core::ffi::c_int = unknown_encoding_converter((*uenc).converter_id)
                .expect("unknown encoding converter is registered")
                .invoke((*uenc).userData, *fromP);
            n = encode_unknown_utf8(c, &mut buf) as ::core::ffi::c_int;
            if n as isize > toLim.offset_from(*toP) {
                return crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
            }
            utf8 = &raw mut buf as *mut ::core::ffi::c_char;
            *fromP = (*fromP).offset(
                ((*(enc as *const normal_encoding)).type_0[**fromP as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                    - (crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int
                        - 2 as ::core::ffi::c_int)) as isize,
            );
        } else {
            if n as isize > toLim.offset_from(*toP) {
                return crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
            }
            *fromP = (*fromP).offset(1);
        }
        crate::stdlib::memcpy(
            *toP as *mut ::core::ffi::c_void,
            utf8 as *const ::core::ffi::c_void,
            n as crate::__stddef_size_t_h::size_t,
        );
        *toP = (*toP).offset(n as isize);
    }
}

unsafe extern "C" fn unknown_toUtf16(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_ushort,
    mut toLim: *const ::core::ffi::c_ushort,
) -> crate::src::xmltok::XML_Convert_Result {
    let mut uenc: *const unknown_encoding = enc as *const unknown_encoding;
    while *fromP < fromLim && *toP < toLim as *mut ::core::ffi::c_ushort {
        let mut c: ::core::ffi::c_ushort = (*uenc).utf16[**fromP as ::core::ffi::c_uchar as usize];
        if c as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            c = unknown_encoding_converter((*uenc).converter_id)
                .expect("unknown encoding converter is registered")
                .invoke((*uenc).userData, *fromP) as ::core::ffi::c_ushort;
            *fromP = (*fromP).offset(
                ((*(enc as *const normal_encoding)).type_0[**fromP as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                    - (crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int
                        - 2 as ::core::ffi::c_int)) as isize,
            );
        } else {
            *fromP = (*fromP).offset(1);
        }
        let c2rust_fresh36 = *toP;
        *toP = (*toP).offset(1);
        *c2rust_fresh36 = c;
    }
    if *toP == toLim as *mut ::core::ffi::c_ushort && *fromP < fromLim {
        return crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
    } else {
        return crate::src::xmltok::XML_CONVERT_COMPLETED;
    };
}
fn char_ref_number_is_valid(result: ::core::ffi::c_int, latin1: &normal_encoding) -> bool {
    match result >> 8 {
        216..=223 => false,
        0 => {
            latin1.type_0[result as usize] as ::core::ffi::c_int
                != crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int
        }
        255 => result != 0xfffe && result != 0xffff,
        _ => true,
    }
}

fn bitmap_contains(
    page_table: &[::core::ffi::c_uchar; 256],
    character: ::core::ffi::c_int,
) -> bool {
    let word = ((page_table[(character >> 8) as usize] as usize) << 3)
        + ((character as usize & 0xff) >> 5);
    namingBitmap[word] & (1 << (character & 0x1f)) != 0
}

fn encode_unknown_utf8(character: ::core::ffi::c_int, output: &mut [::core::ffi::c_char]) -> usize {
    if character < 0 || output.is_empty() {
        0
    } else if character < min2 as ::core::ffi::c_int {
        output[0] = character as ::core::ffi::c_char;
        1
    } else if character < min3 as ::core::ffi::c_int && output.len() >= 2 {
        output[0] = (character >> 6 | UTF8_cval2 as ::core::ffi::c_int) as ::core::ffi::c_char;
        output[1] = (character & 0x3f | 0x80) as ::core::ffi::c_char;
        2
    } else if character < min4 as ::core::ffi::c_int && output.len() >= 3 {
        output[0] = (character >> 12 | UTF8_cval3 as ::core::ffi::c_int) as ::core::ffi::c_char;
        output[1] = (character >> 6 & 0x3f | 0x80) as ::core::ffi::c_char;
        output[2] = (character & 0x3f | 0x80) as ::core::ffi::c_char;
        3
    } else if character < 0x110000 && output.len() >= 4 {
        output[0] = (character >> 18 | UTF8_cval4 as ::core::ffi::c_int) as ::core::ffi::c_char;
        output[1] = (character >> 12 & 0x3f | 0x80) as ::core::ffi::c_char;
        output[2] = (character >> 6 & 0x3f | 0x80) as ::core::ffi::c_char;
        output[3] = (character & 0x3f | 0x80) as ::core::ffi::c_char;
        4
    } else {
        0
    }
}

fn initialize_unknown_encoding(
    encoding: &mut unknown_encoding,
    table: &[::core::ffi::c_int; 256],
    latin1: &normal_encoding,
    has_converter: bool,
) -> bool {
    encoding.normal = *latin1;
    for (index, &character) in table.iter().take(128).enumerate() {
        if latin1.type_0[index] as ::core::ffi::c_int
            != crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int
            && latin1.type_0[index] as ::core::ffi::c_int
                != crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int
            && character != index as ::core::ffi::c_int
        {
            return false;
        }
    }

    for (index, &character) in table.iter().enumerate() {
        if character == -1 {
            encoding.normal.type_0[index] =
                crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_uchar;
            encoding.utf16[index] = 0xffff;
            encoding.utf8[index] = [1, 0, 0, 0];
        } else if character < 0 {
            if character < -4 || !has_converter {
                return false;
            }
            encoding.normal.type_0[index] = (crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int
                - (character + 2))
                as ::core::ffi::c_uchar;
            encoding.utf8[index][0] = 0;
            encoding.utf16[index] = 0;
        } else if character < 0x80 {
            if latin1.type_0[character as usize] as ::core::ffi::c_int
                != crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int
                && latin1.type_0[character as usize] as ::core::ffi::c_int
                    != crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int
                && character != index as ::core::ffi::c_int
            {
                return false;
            }
            encoding.normal.type_0[index] = latin1.type_0[character as usize];
            encoding.utf8[index] = [1, character as ::core::ffi::c_char, 0, 0];
            encoding.utf16[index] = if character == 0 {
                0xffff
            } else {
                character as ::core::ffi::c_ushort
            };
        } else if !char_ref_number_is_valid(character, latin1) {
            encoding.normal.type_0[index] = crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_uchar;
            encoding.utf16[index] = 0xffff;
            encoding.utf8[index] = [1, 0, 0, 0];
        } else {
            if character > 0xffff {
                return false;
            }
            encoding.normal.type_0[index] = if bitmap_contains(&nmstrtPages, character) {
                crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_uchar
            } else if bitmap_contains(&namePages, character) {
                crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_uchar
            } else {
                crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_uchar
            };
            encoding.utf8[index][0] = encode_unknown_utf8(character, &mut encoding.utf8[index][1..])
                as ::core::ffi::c_char;
            encoding.utf16[index] = character as ::core::ffi::c_ushort;
        }
    }
    true
}

fn install_unknown_name_checks(encoding: &mut unknown_encoding) {
    encoding.normal.isName2 = Name2Checker::Unknown;
    encoding.normal.isName3 = Name3Checker::Unknown;
    encoding.normal.isName4 = Name4Checker::Unknown;
    encoding.normal.isNmstrt2 = NameStart2Checker::Unknown;
    encoding.normal.isNmstrt3 = NameStart3Checker::Unknown;
    encoding.normal.isNmstrt4 = NameStart4Checker::Unknown;
    encoding.normal.invalid2 = Invalid2Checker::Unknown;
    encoding.normal.invalid3 = Invalid3Checker::Unknown;
    encoding.normal.invalid4 = Invalid4Checker::Unknown;
}

pub unsafe extern "C" fn XmlInitUnknownEncoding(
    mem: *mut ::core::ffi::c_void,
    table: *const ::core::ffi::c_int,
    convert: crate::src::xmltok::CONVERTER,
    userData: *mut ::core::ffi::c_void,
) -> *mut crate::src::xmltok::ENCODING {
    register_unknown_encoding_converter(mem as usize, None);
    let (encoding, table, latin1) = unsafe {
        (
            &mut *(mem as *mut unknown_encoding),
            &*(table as *const [::core::ffi::c_int; 256]),
            &latin1_encoding,
        )
    };
    if !initialize_unknown_encoding(encoding, table, latin1, convert.is_some()) {
        return ::core::ptr::null_mut();
    }
    encoding.converter_id = mem as usize;
    encoding.userData = userData;
    register_unknown_encoding_converter(
        encoding.converter_id,
        convert.map(|callback| {
            std::sync::Arc::new(callback) as std::sync::Arc<dyn UnknownEncodingConverter>
        }),
    );
    if convert.is_some() {
        install_unknown_name_checks(encoding);
    }
    encoding.normal.enc.utf8Convert = Utf8Converter::Unknown;
    encoding.normal.enc.utf16Convert = Utf16Converter::Unknown;
    &mut encoding.normal.enc
}
#[export_name = "XmlInitUnknownEncoding"]

pub unsafe extern "C" fn XmlInitUnknownEncoding_ffi(
    mut mem: *mut ::core::ffi::c_void,
    mut table: *const ::core::ffi::c_int,
    mut convert: crate::src::xmltok::CONVERTER,
    mut userData: *mut ::core::ffi::c_void,
) -> *mut crate::src::xmltok::ENCODING {
    XmlInitUnknownEncoding(mem, table, convert, userData)
}
static mut KW_ISO_8859_1: [::core::ffi::c_char; 11] = [
    crate::ascii_h::ASCII_I as ::core::ffi::c_char,
    crate::ascii_h::ASCII_S as ::core::ffi::c_char,
    crate::ascii_h::ASCII_O as ::core::ffi::c_char,
    crate::ascii_h::ASCII_MINUS as ::core::ffi::c_char,
    crate::ascii_h::ASCII_8_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_8_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_5 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_9_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_MINUS as ::core::ffi::c_char,
    crate::ascii_h::ASCII_1_1 as ::core::ffi::c_char,
    '\0' as ::core::ffi::c_char,
];

static mut KW_US_ASCII: [::core::ffi::c_char; 9] = [
    crate::ascii_h::ASCII_U as ::core::ffi::c_char,
    crate::ascii_h::ASCII_S as ::core::ffi::c_char,
    crate::ascii_h::ASCII_MINUS as ::core::ffi::c_char,
    crate::ascii_h::ASCII_A as ::core::ffi::c_char,
    crate::ascii_h::ASCII_S as ::core::ffi::c_char,
    crate::ascii_h::ASCII_C as ::core::ffi::c_char,
    crate::ascii_h::ASCII_I as ::core::ffi::c_char,
    crate::ascii_h::ASCII_I as ::core::ffi::c_char,
    '\0' as ::core::ffi::c_char,
];

static mut KW_UTF_8: [::core::ffi::c_char; 6] = [
    crate::ascii_h::ASCII_U as ::core::ffi::c_char,
    crate::ascii_h::ASCII_T as ::core::ffi::c_char,
    crate::ascii_h::ASCII_F_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_MINUS as ::core::ffi::c_char,
    crate::ascii_h::ASCII_8_1 as ::core::ffi::c_char,
    '\0' as ::core::ffi::c_char,
];

static mut KW_UTF_16: [::core::ffi::c_char; 7] = [
    crate::ascii_h::ASCII_U as ::core::ffi::c_char,
    crate::ascii_h::ASCII_T as ::core::ffi::c_char,
    crate::ascii_h::ASCII_F_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_MINUS as ::core::ffi::c_char,
    crate::ascii_h::ASCII_1_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_6 as ::core::ffi::c_char,
    '\0' as ::core::ffi::c_char,
];

static mut KW_UTF_16BE: [::core::ffi::c_char; 9] = [
    crate::ascii_h::ASCII_U as ::core::ffi::c_char,
    crate::ascii_h::ASCII_T as ::core::ffi::c_char,
    crate::ascii_h::ASCII_F_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_MINUS as ::core::ffi::c_char,
    crate::ascii_h::ASCII_1_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_6 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_B_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_E_1 as ::core::ffi::c_char,
    '\0' as ::core::ffi::c_char,
];

static mut KW_UTF_16LE: [::core::ffi::c_char; 9] = [
    crate::ascii_h::ASCII_U as ::core::ffi::c_char,
    crate::ascii_h::ASCII_T as ::core::ffi::c_char,
    crate::ascii_h::ASCII_F_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_MINUS as ::core::ffi::c_char,
    crate::ascii_h::ASCII_1_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_6 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_L_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_E_1 as ::core::ffi::c_char,
    '\0' as ::core::ffi::c_char,
];

unsafe extern "C" fn getEncodingIndex(mut name: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let encoding_names: [*const ::core::ffi::c_char; 6] = [
        &raw const KW_ISO_8859_1 as *const ::core::ffi::c_char,
        &raw const KW_US_ASCII as *const ::core::ffi::c_char,
        &raw const KW_UTF_8 as *const ::core::ffi::c_char,
        &raw const KW_UTF_16 as *const ::core::ffi::c_char,
        &raw const KW_UTF_16BE as *const ::core::ffi::c_char,
        &raw const KW_UTF_16LE as *const ::core::ffi::c_char,
    ];
    let mut i: ::core::ffi::c_int = 0;
    if name.is_null() {
        return NO_ENC as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < ::core::mem::size_of::<[*const ::core::ffi::c_char; 6]>()
        .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
        as ::core::ffi::c_int
    {
        if streqci(name, encoding_names[i as usize]) != 0 {
            return i;
        }
        i += 1;
    }
    return UNKNOWN_ENC as ::core::ffi::c_int;
}

#[derive(Copy, Clone)]
enum InitScanState {
    Prolog,
    Content,
}

impl InitScanState {
    fn is_content(self) -> bool {
        matches!(self, Self::Content)
    }

    fn scanner_index(self) -> usize {
        match self {
            Self::Prolog => crate::src::xmltok::XML_PROLOG_STATE as usize,
            Self::Content => crate::src::xmltok::XML_CONTENT_STATE as usize,
        }
    }
}

#[derive(Copy, Clone)]
enum InitScanAction {
    None,
    Partial,
    Bom {
        encoding_index: usize,
        consumed: usize,
    },
    Scan {
        encoding_index: usize,
    },
}

/// Decides which initial encoding scanner to use from the bytes already made
/// available to the tokenizer.  The caller retains the raw cursor only to
/// update Expat's fixed C ABI state after this bounded inspection.
fn init_scan_action(
    initial_encoding: ::core::ffi::c_char,
    state: InitScanState,
    input: &[u8],
) -> InitScanAction {
    let initial_encoding = initial_encoding as ::core::ffi::c_int;
    let content_uses_latin1 =
        initial_encoding == ISO_8859_1_ENC as ::core::ffi::c_int && state.is_content();

    let Some(&first) = input.first() else {
        return InitScanAction::None;
    };
    if input.len() == 1 {
        if matches!(initial_encoding, 3 | 5 | 4) {
            return InitScanAction::Partial;
        }
        if matches!(first, 254 | 255 | 239) && !content_uses_latin1 || matches!(first, 0 | 60) {
            return InitScanAction::Partial;
        }
        return InitScanAction::Scan {
            encoding_index: initial_encoding as usize,
        };
    }

    let second = input[1];
    match u16::from_be_bytes([first, second]) {
        0xfeff if !content_uses_latin1 => InitScanAction::Bom {
            encoding_index: UTF_16BE_ENC as usize,
            consumed: 2,
        },
        0x3c00
            if !((initial_encoding == UTF_16BE_ENC as ::core::ffi::c_int
                || initial_encoding == UTF_16_ENC as ::core::ffi::c_int)
                && state.is_content()) =>
        {
            InitScanAction::Scan {
                encoding_index: UTF_16LE_ENC as usize,
            }
        }
        0xfffe if !content_uses_latin1 => InitScanAction::Bom {
            encoding_index: UTF_16LE_ENC as usize,
            consumed: 2,
        },
        0xefbb => {
            if state.is_content()
                && matches!(
                    initial_encoding,
                    value
                        if value == ISO_8859_1_ENC as ::core::ffi::c_int
                            || value == UTF_16BE_ENC as ::core::ffi::c_int
                            || value == UTF_16LE_ENC as ::core::ffi::c_int
                            || value == UTF_16_ENC as ::core::ffi::c_int
                )
            {
                InitScanAction::Scan {
                    encoding_index: initial_encoding as usize,
                }
            } else if input.len() == 2 {
                InitScanAction::Partial
            } else if input[2] == 0xbf {
                InitScanAction::Bom {
                    encoding_index: UTF_8_ENC as usize,
                    consumed: 3,
                }
            } else {
                InitScanAction::Scan {
                    encoding_index: initial_encoding as usize,
                }
            }
        }
        _ if first == 0
            && !(state.is_content() && initial_encoding == UTF_16LE_ENC as ::core::ffi::c_int) =>
        {
            InitScanAction::Scan {
                encoding_index: UTF_16BE_ENC as usize,
            }
        }
        _ if second == 0 && !state.is_content() => InitScanAction::Scan {
            encoding_index: UTF_16LE_ENC as usize,
        },
        _ => InitScanAction::Scan {
            encoding_index: initial_encoding as usize,
        },
    }
}

/// # Safety
///
/// The tokenizer dispatch contract supplies a valid, readable `ptr..end`
/// range from one allocation, a seven-entry encoding table, and writable
/// output slots in both `enc` and `nextTokPtr`.  This adapter confines those
/// C ABI cursors to the final state update and scanner dispatch.
unsafe extern "C" fn initScan(
    mut encodingTable: *const *const crate::src::xmltok::ENCODING,
    mut enc: *const crate::src::xmltok::INIT_ENCODING,
    mut state: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let input_len = end.offset_from(ptr);
    if input_len <= 0 {
        return crate::src::xmltok::XML_TOK_NONE_1;
    }
    let state = if state == crate::src::xmltok::XML_CONTENT_STATE {
        InitScanState::Content
    } else {
        InitScanState::Prolog
    };
    let input = ::core::slice::from_raw_parts(ptr.cast::<u8>(), input_len as usize);
    let initial_encoding = &mut *(enc as *mut crate::src::xmltok::INIT_ENCODING);
    match init_scan_action(initial_encoding.initEnc.isUtf16, state, input) {
        InitScanAction::None => crate::src::xmltok::XML_TOK_NONE_1,
        InitScanAction::Partial => crate::src::xmltok::XML_TOK_PARTIAL_1,
        InitScanAction::Bom {
            encoding_index,
            consumed,
        } => {
            *nextTokPtr = ptr.add(consumed);
            initial_encoding.selected_encoding = Some(encoding_index);
            crate::src::xmltok::XML_TOK_BOM_1
        }
        InitScanAction::Scan { encoding_index } => {
            let selected_encoding = *encodingTable.add(encoding_index);
            initial_encoding.selected_encoding = Some(encoding_index);
            (*selected_encoding).scanners[state.scanner_index()].scan(
                selected_encoding,
                ptr,
                end,
                nextTokPtr,
            )
        }
    }
}
pub unsafe extern "C" fn XmlInitUnknownEncodingNS(
    mut mem: *mut ::core::ffi::c_void,
    mut table: *const ::core::ffi::c_int,
    mut convert: crate::src::xmltok::CONVERTER,
    mut userData: *mut ::core::ffi::c_void,
) -> *mut crate::src::xmltok::ENCODING {
    let mut enc: *mut crate::src::xmltok::ENCODING =
        XmlInitUnknownEncoding(mem, table, convert, userData);
    if !enc.is_null() {
        (*(enc as *mut normal_encoding)).type_0[crate::ascii_h::ASCII_COLON as usize] =
            crate::xmltok_impl_h::BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar;
    }
    return enc;
}
#[export_name = "XmlInitUnknownEncodingNS"]

pub unsafe extern "C" fn XmlInitUnknownEncodingNS_ffi(
    mut mem: *mut ::core::ffi::c_void,
    mut table: *const ::core::ffi::c_int,
    mut convert: crate::src::xmltok::CONVERTER,
    mut userData: *mut ::core::ffi::c_void,
) -> *mut crate::src::xmltok::ENCODING {
    XmlInitUnknownEncodingNS(mem, table, convert, userData)
}
unsafe extern "C" fn c2rust_run_static_initializers() {
    encodings = [
        &raw const latin1_encoding.enc,
        &raw const ascii_encoding.enc,
        &raw const utf8_encoding.enc,
        &raw const big2_encoding.enc,
        &raw const big2_encoding.enc,
        &raw const little2_encoding.enc,
        &raw const utf8_encoding.enc,
    ];
    encodingsNS = [
        &raw const latin1_encoding_ns.enc,
        &raw const ascii_encoding_ns.enc,
        &raw const utf8_encoding_ns.enc,
        &raw const big2_encoding_ns.enc,
        &raw const big2_encoding_ns.enc,
        &raw const little2_encoding_ns.enc,
        &raw const utf8_encoding_ns.enc,
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
