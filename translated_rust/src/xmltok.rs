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
    /// Offset of the attribute name within the start-tag token passed to the
    /// attribute scanner.  Keeping this as an offset prevents the scanner's
    /// transient input address from escaping into parser scratch state.
    pub name: usize,
    /// Offset of the attribute value's first byte within the start-tag token
    /// passed to the scanner.  Keeping this as an offset prevents the
    /// scanner's transient input address from escaping into parser scratch
    /// state.
    pub valueStart: usize,
    /// Offset of the attribute value's exclusive end within the start-tag
    /// token passed to the scanner.  Like `name`, this keeps the scanner's
    /// transient input address out of parser scratch state.
    pub valueEnd: usize,
    pub normalized: ::core::ffi::c_char,
}

pub type ENCODING = crate::src::xmltok::encoding;

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

/// A tokenizer result expressed relative to an already-validated input slice.
/// Boundary adapters alone turn `next` back into a C cursor.
#[derive(Copy, Clone)]
pub(crate) struct ScannerResult {
    pub(crate) token: ::core::ffi::c_int,
    pub(crate) next: Option<usize>,
}

impl ScannerResult {
    pub(crate) const fn new(token: ::core::ffi::c_int, next: Option<usize>) -> Self {
        Self { token, next }
    }
}

/// Two read-only views of the same tokenizer-bounded bytes.  UTF-16 helpers
/// retain their historical `XML_Char` view while byte-oriented scanners use
/// the ordinary byte slice.  Both are created only by the C cursor adapter.
pub(crate) struct ScannerInput<'a> {
    pub(crate) bytes: &'a [u8],
    pub(crate) chars: &'a [::core::ffi::c_char],
}

/// A tokenizer dispatch request whose C cursors have been checked and
/// translated at the boundary.  Scanner implementations work entirely with
/// the bounded input and the typed encoding reference retained here.
pub(crate) struct ScannerContext<'a>(ScannerContextKind<'a>);

enum ScannerContextKind<'a> {
    InvalidRange,
    Initial {
        scanner: Scanner,
        initial: &'a mut INIT_ENCODING,
        input: ScannerInput<'a>,
    },
    Normal {
        scanner: Scanner,
        encoding: &'a normal_encoding,
        input: ScannerInput<'a>,
        unknown_converter_id: Option<usize>,
    },
}

impl<'a> ScannerContext<'a> {
    /// Builds an initial-encoding request from a caller-validated character
    /// slice.  Raw cursor validation belongs to the parser boundary; scanner
    /// dispatch itself only retains bounded input and a typed encoding state.
    pub(crate) fn initial(
        scanner: Scanner,
        initial: &'a mut INIT_ENCODING,
        chars: &'a [::core::ffi::c_char],
    ) -> Self {
        let input = ScannerInput {
            // `c_char` and `u8` have identical one-byte layouts.
            bytes: bytemuck::cast_slice(chars),
            chars,
        };
        Self(ScannerContextKind::Initial {
            scanner,
            initial,
            input,
        })
    }

    /// Builds a normal-encoding request from a caller-validated character
    /// slice.  Unknown encodings retain their registration key in the typed
    /// encoding state, so scanner dispatch never derives an identity from a
    /// pointer.
    pub(crate) fn normal(
        scanner: Scanner,
        encoding: &'a normal_encoding,
        chars: &'a [::core::ffi::c_char],
    ) -> Self {
        let input = ScannerInput {
            bytes: bytemuck::cast_slice(chars),
            chars,
        };
        Self(ScannerContextKind::Normal {
            scanner,
            encoding,
            input,
            unknown_converter_id: encoding.unknown_converter_id,
        })
    }

    /// Represents a rejected parser cursor without ever constructing a slice
    /// from it.
    pub(crate) fn invalid() -> Self {
        Self(ScannerContextKind::InvalidRange)
    }

    /// Returns the checked character view retained by this dispatch request.
    /// A scanner result is an offset in precisely this slice, so boundary
    /// adapters can recover a C cursor without performing raw arithmetic.
    pub(crate) fn chars(&self) -> &'a [::core::ffi::c_char] {
        match &self.0 {
            ScannerContextKind::InvalidRange => &[],
            ScannerContextKind::Initial { input, .. }
            | ScannerContextKind::Normal { input, .. } => input.chars,
        }
    }

    pub(crate) fn scan(self) -> ScannerResult {
        match self.0 {
            ScannerContextKind::InvalidRange => {
                ScannerResult::new(crate::src::xmltok::XML_TOK_NONE_1, None)
            }
            ScannerContextKind::Initial {
                scanner,
                initial,
                input,
            } => {
                let state = match scanner {
                    Scanner::InitProlog | Scanner::InitPrologNS => InitScanState::Prolog,
                    Scanner::InitContent | Scanner::InitContentNS => InitScanState::Content,
                    _ => unreachable!("initial scanner context has an initial scanner"),
                };
                let namespace_aware =
                    matches!(scanner, Scanner::InitPrologNS | Scanner::InitContentNS);
                initial_scan_result(initial, namespace_aware, state, input)
            }
            ScannerContextKind::Normal {
                scanner,
                encoding,
                input,
                unknown_converter_id,
            } => scanner.scan_result(encoding, input, unknown_converter_id),
        }
    }
}
impl Scanner {
    pub(crate) fn scan_result(
        self,
        encoding: &normal_encoding,
        input: ScannerInput<'_>,
        unknown_converter_id: Option<usize>,
    ) -> ScannerResult {
        crate::src::xmltok::xmltok_impl_c::scan_result(
            self,
            encoding,
            input,
            unknown_converter_id,
        )
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

/// Validates a quoted public identifier and reports the offset of its first
/// invalid code unit.  The quote delimiters occupy one encoded character at
/// each end, so malformed or incomplete delimiters remain valid here just as
/// the historical scanner treated them.
pub fn quoted_public_id_bad_offset(
    quoted: &[u8],
    byte_types: &[::core::ffi::c_uchar; 256],
    checker: PublicIdChecker,
) -> Option<usize> {
    let width = match checker {
        PublicIdChecker::Normal => 1,
        PublicIdChecker::Little2 | PublicIdChecker::Big2 => 2,
    };
    let Some(contents_end) = quoted.len().checked_sub(width) else {
        return None;
    };
    let contents_start = width;
    if contents_start > contents_end {
        return None;
    }
    public_id_bad_offset(&quoted[contents_start..contents_end], byte_types, checker)
        .map(|offset| contents_start + offset)
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

impl NameLength {
    /// Measures the name prefix of a tokenizer-bounded byte range.  The
    /// caller retains ownership of the range, so this never reconstructs a
    /// slice from a raw cursor.
    pub(crate) fn measure(
        self,
        encoding: &normal_encoding,
        source: AttributeSource<'_>,
    ) -> ::core::ffi::c_int {
        match source {
            AttributeSource::Bytes(bytes) => {
                self.measure_with(encoding, bytes.len(), |offset| bytes.get(offset).copied())
            }
            AttributeSource::Chars(chars) => self.measure_with(encoding, chars.len(), |offset| {
                chars.get(offset).map(|&byte| byte as u8)
            }),
        }
    }

    fn measure_with(
        self,
        encoding: &normal_encoding,
        input_len: usize,
        byte_at: impl Fn(usize) -> Option<u8>,
    ) -> ::core::ffi::c_int {
        let mut offset = 0usize;
        loop {
            let Some(first) = byte_at(offset) else {
                break;
            };
            let byte_type = match self {
                Self::Normal => encoding.type_0[first as usize] as ::core::ffi::c_int,
                Self::Little2 => {
                    let Some(second_offset) = offset.checked_add(1) else {
                        break;
                    };
                    let Some(second) = byte_at(second_offset) else {
                        break;
                    };
                    if second == 0 {
                        encoding.type_0[first as usize] as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(
                            second as ::core::ffi::c_char,
                            first as ::core::ffi::c_char,
                        )
                    }
                }
                Self::Big2 => {
                    let Some(second_offset) = offset.checked_add(1) else {
                        break;
                    };
                    let Some(second) = byte_at(second_offset) else {
                        break;
                    };
                    if first == 0 {
                        encoding.type_0[second as usize] as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(
                            first as ::core::ffi::c_char,
                            second as ::core::ffi::c_char,
                        )
                    }
                }
            };
            let width = match byte_type {
                5 => 2,
                6 => 3,
                7 => 4,
                29 | 22 | 23 | 24 | 25 | 26 | 27 => match self {
                    Self::Normal => 1,
                    Self::Little2 | Self::Big2 => 2,
                },
                _ => break,
            };
            let Some(next) = offset.checked_add(width) else {
                break;
            };
            if next > input_len {
                break;
            }
            offset = next;
        }
        ::core::ffi::c_int::try_from(offset).unwrap_or(::core::ffi::c_int::MAX)
    }
}

impl NameMatcher {
    fn matches_ascii_bytes(self, input: &[u8], expected: &[u8]) -> ::core::ffi::c_int {
        let width = match self {
            Self::Normal => 1,
            Self::Little2 | Self::Big2 => 2,
        };
        let Some(expected_len) = expected.len().checked_mul(width) else {
            return 0;
        };
        if input.len() != expected_len {
            return 0;
        }
        let matches = match self {
            Self::Normal => input == expected,
            Self::Little2 => expected
                .iter()
                .zip(input.chunks_exact(2))
                .all(|(&byte, code_unit)| code_unit == [byte, 0]),
            Self::Big2 => expected
                .iter()
                .zip(input.chunks_exact(2))
                .all(|(&byte, code_unit)| code_unit == [0, byte]),
        };
        matches as ::core::ffi::c_int
    }
}

#[derive(Copy, Clone)]
pub enum AttributeScanner {
    Normal,
    Little2,
    Big2,
}

/// A complete start-tag token whose allocation has already been validated by
/// the parser.  Internal entity text is kept as `XML_Char` storage, while the
/// parser input buffer is naturally bytes.  Keeping both representations here
/// avoids fabricating a byte slice from an entity cursor just to run the
/// tokenizer.
pub(crate) enum AttributeSource<'a> {
    Bytes(&'a [u8]),
    Chars(&'a [::core::ffi::c_char]),
}

impl AttributeSource<'_> {
    pub(crate) fn len(&self) -> usize {
        match self {
            Self::Bytes(bytes) => bytes.len(),
            Self::Chars(chars) => chars.len(),
        }
    }
}

impl AttributeScanner {
    /// Scans one complete start-tag token already bounded by its tokenizer
    /// cursor.  Attribute pointers are derived only from that verified slice;
    /// the scanner itself never needs to reconstruct a slice from raw parser
    /// cursors.
    pub(crate) fn scan(
        self,
        encoding: &normal_encoding,
        source: AttributeSource<'_>,
        attributes: &mut [ATTRIBUTE],
    ) -> ::core::ffi::c_int {
        xmltok_impl_c::scan_atts(self, &encoding.type_0, source, attributes)
    }
}

/// Selects the fixed whitespace scanner without retaining a raw callback.
#[derive(Copy, Clone)]
pub enum WhitespaceSkipper {
    Normal,
    Little2,
    Big2,
}

impl WhitespaceSkipper {
    /// Returns the number of encoded bytes occupied by leading XML whitespace.
    fn skip_s_bytes(self, bytes: &[::core::ffi::c_char]) -> usize {
        match self {
            Self::Normal => bytes
                .iter()
                .position(|&byte| !matches!(byte as u8, b' ' | b'\t' | b'\n' | b'\r'))
                .unwrap_or(bytes.len()),
            Self::Little2 | Self::Big2 => {
                let mut skipped = 0;
                for code_unit in bytes.chunks_exact(2) {
                    let is_space = match self {
                        Self::Little2 => {
                            code_unit[1] == 0
                                && matches!(code_unit[0] as u8, b' ' | b'\t' | b'\n' | b'\r')
                        }
                        Self::Big2 => {
                            code_unit[0] == 0
                                && matches!(code_unit[1] as u8, b' ' | b'\t' | b'\n' | b'\r')
                        }
                        Self::Normal => unreachable!(),
                    };
                    if !is_space {
                        break;
                    }
                    skipped += 2;
                }
                skipped
            }
        }
    }
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
    /// Decodes a complete tokenizer-recognized character-reference token.
    ///
    /// `input` includes the leading `&#` and trailing semicolon.  Keeping the
    /// token bounded here avoids the old callback's unbounded raw reads.
    pub fn decode(self, input: &[u8]) -> ::core::ffi::c_int {
        self.decode_units(input.len(), |offset| input.get(offset).copied())
    }

    /// Decodes a complete character-reference token retained as XML character
    /// storage.  Internal entity text is represented this way by the parser;
    /// reading its units directly avoids creating a byte slice from a raw
    /// event cursor.
    pub(crate) fn decode_chars(self, input: &[::core::ffi::c_char]) -> ::core::ffi::c_int {
        self.decode_units(input.len(), |offset| {
            input.get(offset).map(|&byte| byte as u8)
        })
    }

    fn decode_units(
        self,
        input_len: usize,
        byte_at: impl Fn(usize) -> Option<u8>,
    ) -> ::core::ffi::c_int {
        match self {
            Self::Normal => decode_char_ref_number_units(
                (2..input_len).filter_map(|offset| byte_at(offset).map(|byte| byte as _)),
            ),
            Self::Little2 => decode_char_ref_number_units(
                (4..input_len)
                    .step_by(2)
                    .filter_map(|offset| Some((byte_at(offset)?, byte_at(offset.checked_add(1)?)?)))
                    .map(|(first, second)| {
                        if second == 0 {
                            first as ::core::ffi::c_int
                        } else {
                            -1
                        }
                    }),
            ),
            Self::Big2 => decode_char_ref_number_units(
                (4..input_len)
                    .step_by(2)
                    .filter_map(|offset| Some((byte_at(offset)?, byte_at(offset.checked_add(1)?)?)))
                    .map(|(first, second)| {
                        if first == 0 {
                            second as ::core::ffi::c_int
                        } else {
                            -1
                        }
                    }),
            ),
        }
    }
}

/// Decodes the ASCII units after the `&#` prefix of a recognized character
/// reference.  The tokenizer guarantees the terminating semicolon; malformed
/// input is still rejected rather than reading beyond the supplied slice.
fn decode_char_ref_number_units(
    mut units: impl Iterator<Item = ::core::ffi::c_int>,
) -> ::core::ffi::c_int {
    let Some(first) = units.next() else {
        return -1;
    };
    let mut result: ::core::ffi::c_int = 0;
    if first == crate::ascii_h::ASCII_x {
        for c in units {
            if c == 0x3b {
                return checkCharRefNumber(result);
            }
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
                    result <<= 4;
                    result |= c - crate::ascii_h::ASCII_0;
                }
                crate::ascii_h::ASCII_A
                | crate::ascii_h::ASCII_B_1
                | crate::ascii_h::ASCII_C
                | crate::ascii_h::ASCII_D
                | crate::ascii_h::ASCII_E_1
                | crate::ascii_h::ASCII_F_1 => {
                    result <<= 4;
                    result += 10 + (c - crate::ascii_h::ASCII_A);
                }
                crate::ascii_h::ASCII_a_1
                | crate::ascii_h::ASCII_b
                | crate::ascii_h::ASCII_c_1
                | crate::ascii_h::ASCII_d
                | crate::ascii_h::ASCII_e_1
                | crate::ascii_h::ASCII_f => {
                    result <<= 4;
                    result += 10 + (c - crate::ascii_h::ASCII_a_1);
                }
                _ => return -1,
            }
            if result >= 0x110000 {
                return -1;
            }
        }
    } else {
        let mut c = first;
        loop {
            if c == 0x3b {
                return checkCharRefNumber(result);
            }
            if !(crate::ascii_h::ASCII_0..=crate::ascii_h::ASCII_9_1).contains(&c) {
                return -1;
            }
            result *= 10;
            result += c - crate::ascii_h::ASCII_0;
            if result >= 0x110000 {
                return -1;
            }
            let Some(next) = units.next() else {
                break;
            };
            c = next;
        }
    }
    -1
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

/// Matches one bounded encoded entity name using the selected fixed encoding.
fn predefined_entity_name_chars(
    matcher: PredefinedEntityNameMatcher,
    input: &[::core::ffi::c_char],
) -> ::core::ffi::c_int {
    const PREDEFINED: [(&[u8], ::core::ffi::c_int); 5] = [
        (b"lt", crate::ascii_h::ASCII_LT),
        (b"gt", crate::ascii_h::ASCII_GT),
        (b"amp", crate::ascii_h::ASCII_AMP),
        (b"quot", crate::ascii_h::ASCII_QUOT),
        (b"apos", crate::ascii_h::ASCII_APOS),
    ];

    for &(name, value) in &PREDEFINED {
        let width = match matcher {
            PredefinedEntityNameMatcher::Normal => 1,
            PredefinedEntityNameMatcher::Little2 | PredefinedEntityNameMatcher::Big2 => 2,
        };
        if input.len() != name.len() * width {
            continue;
        }
        let matches = match matcher {
            PredefinedEntityNameMatcher::Normal => input
                .iter()
                .zip(name.iter())
                .all(|(&input, &expected)| input as u8 == expected),
            PredefinedEntityNameMatcher::Little2 => input
                .chunks_exact(2)
                .zip(name.iter())
                .all(|(code_unit, &byte)| code_unit[0] as u8 == byte && code_unit[1] == 0),
            PredefinedEntityNameMatcher::Big2 => input
                .chunks_exact(2)
                .zip(name.iter())
                .all(|(code_unit, &byte)| code_unit[0] == 0 && code_unit[1] as u8 == byte),
        };
        if matches {
            return value;
        }
    }
    0
}

/// Matches a tokenizer-bounded encoded entity name using the selected fixed
/// encoding.  The caller supplies the token's already-validated byte range;
/// an empty slice remains a valid (non-matching) XML name.
pub fn predefined_entity_name(
    matcher: PredefinedEntityNameMatcher,
    input: &[::core::ffi::c_char],
) -> ::core::ffi::c_int {
    let width = match matcher {
        PredefinedEntityNameMatcher::Normal => 1,
        PredefinedEntityNameMatcher::Little2 | PredefinedEntityNameMatcher::Big2 => 2,
    };
    // Keep the original floor division: a dangling UTF-16 byte is not part of
    // the encoded name.  Only 2-, 3-, and 4-character predefined names exist,
    // so this also bounds the raw-to-slice conversion to eight bytes.
    let name_len = input.len() / width;
    if !(2..=4).contains(&name_len) {
        return 0;
    }
    predefined_entity_name_chars(matcher, &input[..name_len * width])
}

fn convert_to_utf8_window(
    converter: Utf8Converter,
    unknown_encoding: Option<&unknown_encoding>,
    input: &[u8],
    output: &mut [u8],
) -> (crate::src::xmltok::XML_Convert_Result, usize, usize) {
    match converter {
        Utf8Converter::Utf8 => {
            let (result, copied) = utf8_to_utf8_window(input, output);
            (result, copied, copied)
        }
        Utf8Converter::Latin1 => latin1_to_utf8_window(input, output),
        Utf8Converter::Ascii => {
            let copied = input.len().min(output.len());
            output[..copied].copy_from_slice(&input[..copied]);
            let result = if copied < input.len() {
                crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED
            } else {
                crate::src::xmltok::XML_CONVERT_COMPLETED
            };
            (result, copied, copied)
        }
        Utf8Converter::Little2 => little2_to_utf8_window(input, output),
        Utf8Converter::Big2 => big2_to_utf8_window(input, output),
        Utf8Converter::Unknown => {
            let encoding = unknown_encoding.expect("unknown encoding must have state");
            unknown_to_utf8_window(encoding, input, output, |source| {
                unknown_encoding_converter(encoding.converter_id)
                    .expect("unknown encoding converter is registered")
                    .invoke
                    .invoke(source)
            })
        }
    }
}

/// Converts one already-bounded input window without recreating C cursors.
///
/// The optional unknown-encoding state is supplied by its parser-owned
/// storage.  Built-in encodings do not need it; an unknown converter is only
/// selected when the caller has retained the initialized state that owns its
/// registration key.
pub(crate) fn convert_to_utf8_slice(
    encoding: &ENCODING,
    unknown_encoding: Option<&unknown_encoding>,
    input: &[u8],
    output: &mut [u8],
) -> (XML_Convert_Result, usize, usize) {
    convert_to_utf8_window(encoding.utf8Convert, unknown_encoding, input, output)
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
    fn invoke(&self, input: &[u8]) -> ::core::ffi::c_int;
}

impl<F> UnknownEncodingConverter for F
where
    F: Fn(&[u8]) -> ::core::ffi::c_int + Send + Sync,
{
    fn invoke(&self, input: &[u8]) -> ::core::ffi::c_int {
        self(input)
    }
}

#[derive(Clone)]
pub(crate) struct UnknownEncodingConverterRegistration {
    invoke: std::sync::Arc<dyn UnknownEncodingConverter>,
}

#[derive(Clone)]
struct UnknownEncodingRegistration {
    converter: Option<UnknownEncodingConverterRegistration>,
    /// A typed copy of the initialized conversion tables.  The storage address
    /// remains the registry key, while callers that only retain its leading
    /// `ENCODING` view can recover these tables without casting that view back
    /// into a larger `unknown_encoding` object.
    encoding: unknown_encoding,
}

pub(crate) fn unknown_encoding_callback<F>(
    callback: Option<F>,
) -> Option<UnknownEncodingConverterRegistration>
where
    F: Fn(&[u8]) -> ::core::ffi::c_int + Send + Sync + 'static,
{
    callback.map(|callback| UnknownEncodingConverterRegistration {
        invoke: std::sync::Arc::new(callback),
    })
}

// Unknown encodings are initialized in caller-provided storage.  Keep the
// foreign callback in this boundary adapter instead of retaining it in that
// internal tokenizer object; the storage address is a stable key until the
// parser resets or is freed.
static UNKNOWN_ENCODING_CONVERTERS: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<usize, UnknownEncodingRegistration>>,
> = std::sync::OnceLock::new();

fn register_unknown_encoding_converter(
    storage_id: usize,
    converter: Option<UnknownEncodingConverterRegistration>,
    encoding: Option<unknown_encoding>,
) {
    let mut converters = UNKNOWN_ENCODING_CONVERTERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(encoding) = encoding {
        converters.insert(
            storage_id,
            UnknownEncodingRegistration {
                converter,
                encoding,
            },
        );
    } else {
        converters.remove(&storage_id);
    }
}

fn unknown_encoding_converter(storage_id: usize) -> Option<UnknownEncodingConverterRegistration> {
    UNKNOWN_ENCODING_CONVERTERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&storage_id)
        .and_then(|registration| registration.converter.clone())
}

pub(crate) fn registered_unknown_encoding(
    storage_id: Option<usize>,
) -> Option<unknown_encoding> {
    let storage_id = storage_id?;
    UNKNOWN_ENCODING_CONVERTERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&storage_id)
        .map(|registration| registration.encoding)
}

pub fn unregister_unknown_encoding_converter(storage_id: usize) {
    UNKNOWN_ENCODING_CONVERTERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&storage_id);
}

pub mod xmltok_impl_c {

    /// The normal tokenizer accepts its bounded input as either C chars at a
    /// pointer adapter or bytes at a slice-based caller.  Both representations
    /// describe the same octets.
    pub(super) trait XmlTokenByte: Copy {
        fn token_byte(self) -> u8;
    }

    impl XmlTokenByte for i8 {
        fn token_byte(self) -> u8 {
            self as u8
        }
    }

    impl XmlTokenByte for u8 {
        fn token_byte(self) -> u8 {
            self
        }
    }

    pub(super) enum NormalCharCheck {
        Invalid,
        NameStart,
        Name,
    }

    pub(super) fn normal_char_check(
        normal: &normal_encoding,
        kind: NormalCharCheck,
        width: usize,
        input: &[u8],
        unknown_character: impl FnOnce() -> ::core::ffi::c_int,
    ) -> bool {
        let unknown_check = match kind {
            NormalCharCheck::Invalid => match width {
                2 => match normal.invalid2 {
                    Invalid2Checker::Never => return false,
                    Invalid2Checker::Utf8 => return utf8_invalid2(input),
                    Invalid2Checker::Unknown => Some(unknown_is_invalid as fn(_) -> _),
                },
                3 => match normal.invalid3 {
                    Invalid3Checker::Never => return false,
                    Invalid3Checker::Utf8 => return utf8_invalid3(input),
                    Invalid3Checker::Unknown => Some(unknown_is_invalid as fn(_) -> _),
                },
                4 => match normal.invalid4 {
                    Invalid4Checker::Never => return false,
                    Invalid4Checker::Utf8 => return utf8_invalid4(input),
                    Invalid4Checker::Unknown => Some(unknown_is_invalid as fn(_) -> _),
                },
                _ => unreachable!(),
            },
            NormalCharCheck::NameStart => match width {
                2 => match normal.isNmstrt2 {
                    crate::src::xmltok::NameStart2Checker::Never => {
                        return false;
                    }
                    crate::src::xmltok::NameStart2Checker::Utf8 => {
                        return crate::src::xmltok::utf8_is_name_start2(input);
                    }
                    crate::src::xmltok::NameStart2Checker::Unknown => {
                        Some(crate::src::xmltok::unknown_is_name_start as fn(_) -> _)
                    }
                },
                3 => match normal.isNmstrt3 {
                    NameStart3Checker::Never => return false,
                    NameStart3Checker::Utf8 => return utf8_is_name_start3(input),
                    NameStart3Checker::Unknown => Some(unknown_is_name_start as fn(_) -> _),
                },
                4 => match normal.isNmstrt4 {
                    NameStart4Checker::Never => return false,
                    NameStart4Checker::Unknown => Some(unknown_is_name_start as fn(_) -> _),
                },
                _ => unreachable!(),
            },
            NormalCharCheck::Name => match width {
                2 => match normal.isName2 {
                    Name2Checker::Never => return false,
                    Name2Checker::Utf8 => return utf8_is_name2(input),
                    Name2Checker::Unknown => Some(unknown_is_name as fn(_) -> _),
                },
                3 => match normal.isName3 {
                    Name3Checker::Never => return false,
                    Name3Checker::Utf8 => return utf8_is_name3(input),
                    Name3Checker::Unknown => Some(unknown_is_name as fn(_) -> _),
                },
                4 => match normal.isName4 {
                    Name4Checker::Never => return false,
                    Name4Checker::Unknown => Some(unknown_is_name as fn(_) -> _),
                },
                _ => unreachable!(),
            },
        };
        unknown_check.expect("unknown character check")(unknown_character())
    }

    pub(super) fn normal_scan_comment_impl(
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

    pub(super) enum NormalScanDeclAction {
        Comment,
        Token(::core::ffi::c_int, Option<usize>),
    }

    /// Scans the portion following a declaration opener using a bounded input
    /// window.  The pointer adapter only translates the resulting offset back
    /// to Expat's cursor ABI.
    pub(super) fn normal_scan_decl_impl(
        byte_types: &[::core::ffi::c_uchar; 256],
        input: &[u8],
    ) -> NormalScanDeclAction {
        let Some(first) = input.first().copied() else {
            return NormalScanDeclAction::Token(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        };
        let mut offset = 1;

        match byte_types[first as usize] as ::core::ffi::c_int {
            27 => return NormalScanDeclAction::Comment,
            20 => {
                return NormalScanDeclAction::Token(
                    crate::src::xmltok::XML_TOK_COND_SECT_OPEN_1,
                    Some(offset),
                );
            }
            22 | 24 => {}
            _ => {
                return NormalScanDeclAction::Token(crate::src::xmltok::XML_TOK_INVALID_1, Some(0));
            }
        }

        while offset < input.len() {
            match byte_types[input[offset] as usize] as ::core::ffi::c_int {
                30 => {
                    if input.len() - offset < 2 {
                        return NormalScanDeclAction::Token(
                            crate::src::xmltok::XML_TOK_PARTIAL_1,
                            None,
                        );
                    }
                    match byte_types[input[offset + 1] as usize] as ::core::ffi::c_int {
                        21 | 9 | 10 | 30 => {
                            return NormalScanDeclAction::Token(
                                crate::src::xmltok::XML_TOK_INVALID_1,
                                Some(offset),
                            );
                        }
                        _ => {}
                    }
                }
                21 | 9 | 10 => {}
                22 | 24 => {
                    offset += 1;
                    continue;
                }
                _ => {
                    return NormalScanDeclAction::Token(
                        crate::src::xmltok::XML_TOK_INVALID_1,
                        Some(offset),
                    );
                }
            }
            return NormalScanDeclAction::Token(
                crate::src::xmltok::XML_TOK_DECL_OPEN_1,
                Some(offset),
            );
        }
        NormalScanDeclAction::Token(crate::src::xmltok::XML_TOK_PARTIAL_1, None)
    }

    pub fn normal_checkPiTarget(
        target: &[u8],
        token: &mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        *token = crate::src::xmltok::XML_TOK_PI_1;
        let [x, m, l] = target else {
            return 1;
        };

        let mut has_uppercase = false;
        for (byte, lowercase, uppercase) in [
            (
                *x,
                crate::ascii_h::ASCII_x_1 as u8,
                crate::ascii_h::ASCII_X_1 as u8,
            ),
            (
                *m,
                crate::ascii_h::ASCII_m_1 as u8,
                crate::ascii_h::ASCII_M_1 as u8,
            ),
            (
                *l,
                crate::ascii_h::ASCII_l_1 as u8,
                crate::ascii_h::ASCII_L_1 as u8,
            ),
        ] {
            if byte == lowercase {
                continue;
            }
            if byte == uppercase {
                has_uppercase = true;
                continue;
            }
            return 1;
        }

        if has_uppercase {
            return 0;
        }
        *token = crate::src::xmltok::XML_TOK_XML_DECL_1;
        1
    }

    fn normal_byte_type<T: XmlTokenByte>(
        enc: &normal_encoding,
        input: &[T],
        offset: usize,
    ) -> ::core::ffi::c_int {
        enc.type_0[input[offset].token_byte() as usize] as ::core::ffi::c_int
    }

    fn normal_utf8_invalid<T: XmlTokenByte>(input: &[T], offset: usize, width: usize) -> bool {
        let bytes = &input[offset..offset + width];
        let b0 = bytes[0].token_byte();
        let b1 = bytes[1].token_byte();
        match width {
            2 => b0 < 0xc2 || b1 & 0xc0 != 0x80,
            3 => {
                let b2 = bytes[2].token_byte();
                b2 & 0xc0 != 0x80
                    || (b0 == 0xef && b1 == 0xbf && b2 > 0xbd)
                    || (b0 == 0xe0 && (b1 < 0xa0 || b1 & 0xc0 == 0xc0))
                    || (b0 != 0xe0 && (b1 & 0xc0 != 0x80 || (b0 == 0xed && b1 > 0x9f)))
            }
            4 => {
                let b2 = bytes[2].token_byte();
                let b3 = bytes[3].token_byte();
                b2 & 0xc0 != 0x80
                    || b3 & 0xc0 != 0x80
                    || (b0 == 0xf0 && (b1 < 0x90 || b1 & 0xc0 == 0xc0))
                    || (b0 != 0xf0 && (b1 & 0xc0 != 0x80 || (b0 == 0xf4 && b1 > 0x8f)))
            }
            _ => true,
        }
    }

    fn normal_utf8_name_char<T: XmlTokenByte>(
        input: &[T],
        offset: usize,
        width: usize,
        pages: &[::core::ffi::c_uchar; 256],
    ) -> bool {
        if width == 4 {
            return false;
        }
        let b0 = input[offset].token_byte();
        let b1 = input[offset + 1].token_byte();
        let bitmap_index = if width == 2 {
            pages[((b0 >> 2) & 7) as usize] as usize * 8
                + ((b0 & 3) as usize) * 2
                + ((b1 >> 5) & 1) as usize
        } else {
            pages[(((b0 & 0x0f) << 4) + (b1 >> 2 & 0x0f)) as usize] as usize * 8
                + ((b1 & 3) as usize) * 2
                + ((input[offset + 2].token_byte() >> 5) & 1) as usize
        };
        namingBitmap[bitmap_index] & (1 << (input[offset + width - 1].token_byte() & 0x1f)) != 0
    }

    fn normal_pi_utf8_invalid(input: &[u8], offset: usize, width: usize) -> bool {
        let bytes = &input[offset..offset + width];
        let b0 = bytes[0];
        let b1 = bytes[1];
        match width {
            2 => b0 < 0xc2 || b1 & 0xc0 != 0x80,
            3 => {
                let b2 = bytes[2];
                b2 & 0xc0 != 0x80
                    || (b0 == 0xef && b1 == 0xbf && b2 > 0xbd)
                    || (b0 == 0xe0 && (b1 < 0xa0 || b1 & 0xc0 == 0xc0))
                    || (b0 != 0xe0 && (b1 & 0xc0 != 0x80 || (b0 == 0xed && b1 > 0x9f)))
            }
            4 => {
                let b2 = bytes[2];
                let b3 = bytes[3];
                b2 & 0xc0 != 0x80
                    || b3 & 0xc0 != 0x80
                    || (b0 == 0xf0 && (b1 < 0x90 || b1 & 0xc0 == 0xc0))
                    || (b0 != 0xf0 && (b1 & 0xc0 != 0x80 || (b0 == 0xf4 && b1 > 0x8f)))
            }
            _ => true,
        }
    }

    fn normal_pi_utf8_name_char(
        input: &[u8],
        offset: usize,
        width: usize,
        pages: &[::core::ffi::c_uchar; 256],
    ) -> bool {
        if width == 4 {
            return false;
        }
        let b0 = input[offset];
        let b1 = input[offset + 1];
        let bitmap_index = if width == 2 {
            pages[((b0 >> 2) & 7) as usize] as usize * 8
                + ((b0 & 3) as usize) * 2
                + ((b1 >> 5) & 1) as usize
        } else {
            pages[(((b0 & 0x0f) << 4) + (b1 >> 2 & 0x0f)) as usize] as usize * 8
                + ((b1 & 3) as usize) * 2
                + ((input[offset + 2] >> 5) & 1) as usize
        };
        namingBitmap[bitmap_index] & (1 << (input[offset + width - 1] & 0x1f)) != 0
    }

    fn normal_pi_target_token(
        input: &[u8],
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
            match input[target + offset] {
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

    pub(super) fn normal_scan_pi_impl(
        enc: &normal_encoding,
        input: &[u8],
    ) -> (::core::ffi::c_int, Option<usize>) {
        if input.is_empty() {
            return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }
        let target = 0;
        let mut offset = 0;
        match enc.type_0[input[offset] as usize] as ::core::ffi::c_int {
            22 | 24 => offset += 1,
            29 => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
            kind @ (5 | 6 | 7) => {
                let width = kind as usize - 3;
                if input.len() < width {
                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                }
                if enc.enc.isUtf8 == 0
                    || normal_pi_utf8_invalid(input, offset, width)
                    || !normal_pi_utf8_name_char(input, offset, width, &nmstrtPages)
                {
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                }
                offset += width;
            }
            _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
        }

        while offset < input.len() {
            match enc.type_0[input[offset] as usize] as ::core::ffi::c_int {
                22 | 24 | 25 | 26 | 27 => offset += 1,
                29 => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
                kind @ (5 | 6 | 7) => {
                    let width = kind as usize - 3;
                    if input.len() - offset < width {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                    }
                    if enc.enc.isUtf8 == 0
                        || normal_pi_utf8_invalid(input, offset, width)
                        || !normal_pi_utf8_name_char(input, offset, width, &namePages)
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
                        match enc.type_0[input[offset] as usize] as ::core::ffi::c_int {
                            kind @ (5 | 6 | 7) => {
                                let width = kind as usize - 3;
                                if input.len() - offset < width {
                                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                                }
                                if enc.enc.isUtf8 != 0
                                    && normal_pi_utf8_invalid(input, offset, width)
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
                                if input[offset] == 0x3e {
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
                    return if input[offset] == 0x3e {
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

    /// Scans the bytes after a `<![` opener for the CDATA section keyword.
    ///
    /// This is retained as a named implementation adapter for callers that
    /// used the translated scanner name.  Cursor conversion belongs at the
    /// FFI boundary; tokenizer logic only needs the already-bounded bytes and
    /// returns the next position as an offset.
    pub fn normal_scanCdataSection(
        input: &[u8],
    ) -> (::core::ffi::c_int, Option<usize>) {
        let result = normal_scan_cdata_section_open(input);
        (result.token, result.next)
    }

    trait CdataByte: Copy {
        fn as_u8(self) -> u8;
    }

    impl CdataByte for u8 {
        fn as_u8(self) -> u8 {
            self
        }
    }

    impl CdataByte for ::core::ffi::c_char {
        fn as_u8(self) -> u8 {
            self as u8
        }
    }

    fn utf8_sequence_is_invalid<T: CdataByte>(bytes: &[T]) -> bool {
        let first = bytes.first().map(|byte| byte.as_u8());
        let second = bytes.get(1).map(|byte| byte.as_u8());
        let third = bytes.get(2).map(|byte| byte.as_u8());
        let fourth = bytes.get(3).map(|byte| byte.as_u8());
        match (first, second, third, fourth) {
            (Some(first), Some(second), None, None) => {
                first < 0xc2 || second & 0x80 == 0 || second & 0xc0 == 0xc0
            }
            (Some(first), Some(second), Some(third), None) => {
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
            (Some(first), Some(second), Some(third), Some(fourth)) => {
                fourth & 0x80 == 0
                    || fourth & 0xc0 == 0xc0
                    || third & 0x80 == 0
                    || third & 0xc0 == 0xc0
                    || if first == 0xf0 {
                        second < 0x90 || second & 0xc0 == 0xc0
                    } else {
                        second & 0x80 == 0
                            || if first == 0xf4 {
                                second > 0x8f
                            } else {
                                second & 0xc0 == 0xc0
                            }
                    }
            }
            _ => false,
        }
    }

    fn normal_cdata_section_tok<T: CdataByte>(
        input: &[T],
        byte_types: &[::core::ffi::c_uchar; 256],
        is_utf8: bool,
    ) -> (::core::ffi::c_int, Option<usize>) {
        if input.is_empty() {
            return (crate::src::xmltok::XML_TOK_NONE_1, None);
        }

        let byte_type = |offset: usize| byte_types[input[offset].as_u8() as usize] as u32;
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
                if input[offset].as_u8() == b']' {
                    offset += 1;
                    if offset == input.len() {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                    }
                    if input[offset].as_u8() == b'>' {
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

    struct NormalScanLtResult {
        token: ::core::ffi::c_int,
        next: Option<usize>,
    }

    fn normal_scan_lt_result(token: ::core::ffi::c_int, next: Option<usize>) -> NormalScanLtResult {
        NormalScanLtResult { token, next }
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

    fn normal_scan_cdata_section_open(input: &[u8]) -> NormalScanLtResult {
        const CDATA_LSQB: &[u8; 6] = b"CDATA[";

        if input.len() < CDATA_LSQB.len() {
            return normal_scan_lt_result(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }
        for (offset, expected) in CDATA_LSQB.iter().enumerate() {
            if input[offset] != *expected {
                return normal_scan_lt_result(crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
            }
        }
        normal_scan_lt_result(
            crate::src::xmltok::XML_TOK_CDATA_SECT_OPEN_1,
            Some(CDATA_LSQB.len()),
        )
    }

    fn normal_scan_ref_bytes_impl(
        enc: &normal_encoding,
        input: &[u8],
    ) -> (::core::ffi::c_int, Option<usize>) {
        if input.is_empty() {
            return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }

        let byte_type = |offset: usize| enc.type_0[input[offset] as usize] as ::core::ffi::c_int;
        let mut offset = match byte_type(0) {
            22 | 24 => 1,
            29 => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(0)),
            kind @ (5 | 6 | 7) => {
                let width = kind as usize - 3;
                if input.len() < width {
                    return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                }
                if enc.enc.isUtf8 == 0
                    || normal_pi_utf8_invalid(input, 0, width)
                    || !normal_pi_utf8_name_char(input, 0, width, &nmstrtPages)
                {
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(0));
                }
                width
            }
            19 => {
                let (token, next) = normal_scan_char_ref_bytes_impl(enc, &input[1..]);
                return (token, next.map(|next| next + 1));
            }
            _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(0)),
        };

        while offset < input.len() {
            match byte_type(offset) {
                22 | 24 | 25 | 26 | 27 => offset += 1,
                29 => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
                kind @ (5 | 6 | 7) => {
                    let width = kind as usize - 3;
                    if input.len() - offset < width {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                    }
                    if enc.enc.isUtf8 == 0
                        || normal_pi_utf8_invalid(input, offset, width)
                        || !normal_pi_utf8_name_char(input, offset, width, &namePages)
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

    fn normal_scan_char_ref_bytes_impl(
        enc: &normal_encoding,
        input: &[u8],
    ) -> (::core::ffi::c_int, Option<usize>) {
        if input.is_empty() {
            return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }

        let hex = input[0] == b'x';
        let mut offset = usize::from(hex);
        if offset == input.len() {
            return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }
        let byte_type = |offset: usize| enc.type_0[input[offset] as usize] as ::core::ffi::c_int;
        let first_kind = byte_type(offset);
        if !(first_kind == 25 || (hex && first_kind == 24)) {
            return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
        }
        offset += 1;

        while offset < input.len() {
            match byte_type(offset) {
                25 | 24 if hex => offset += 1,
                25 if !hex => offset += 1,
                18 => return (crate::src::xmltok::XML_TOK_CHAR_REF_1, Some(offset + 1)),
                _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
            }
        }
        (crate::src::xmltok::XML_TOK_PARTIAL_1, None)
    }

    fn normal_scan_lt_with_check(
        normal: &normal_encoding,
        input: &[u8],
        check: &dyn Fn(NormalCharCheck, usize, usize) -> bool,
    ) -> NormalScanLtResult {
        match normal_scan_lt_impl(normal, input, |kind, offset, width| {
            check(
                match kind {
                    NormalScanLtCharCheck::Invalid => NormalCharCheck::Invalid,
                    NormalScanLtCharCheck::NameStart => NormalCharCheck::NameStart,
                    NormalScanLtCharCheck::Name => NormalCharCheck::Name,
                },
                offset,
                width,
            )
        }) {
            NormalScanLtAction::Token(token, next) => normal_scan_lt_result(token, next),
            NormalScanLtAction::Comment(start) => {
                let (token, next) =
                    normal_scan_comment_impl(&normal.type_0, &input[start..], |offset, width| {
                        check(NormalCharCheck::Invalid, start + offset, width)
                    });
                normal_scan_lt_result(token, next.map(|next| start + next))
            }
            NormalScanLtAction::CdataSection(start) => {
                let result = normal_scan_cdata_section_open(&input[start..]);
                normal_scan_lt_result(result.token, result.next.map(|next| start + next))
            }
            NormalScanLtAction::ProcessingInstruction(start) => {
                let (token, next) = normal_scan_pi_impl(normal, &input[start..]);
                normal_scan_lt_result(token, next.map(|next| start + next))
            }
            NormalScanLtAction::EndTag(start) => {
                let result =
                    normal_scan_end_tag_impl(normal, &input[start..], |kind, offset, width| {
                        check(
                            match kind {
                                NormalScanEndTagCharCheck::Invalid => NormalCharCheck::Invalid,
                                NormalScanEndTagCharCheck::NameStart => NormalCharCheck::NameStart,
                                NormalScanEndTagCharCheck::Name => NormalCharCheck::Name,
                            },
                            start + offset,
                            width,
                        )
                    });
                normal_scan_lt_result(result.token, result.next.map(|next| start + next))
            }
            NormalScanLtAction::Attributes(start) => {
                let result = normal_scan_atts_impl(
                    normal,
                    &input[start..],
                    |kind, offset, width| {
                        check(
                            match kind {
                                NormalScanAttsCharCheck::Invalid => NormalCharCheck::Invalid,
                                NormalScanAttsCharCheck::Name => NormalCharCheck::Name,
                                NormalScanAttsCharCheck::NameStart => NormalCharCheck::NameStart,
                            },
                            start + offset,
                            width,
                        )
                    },
                    |ref_start| {
                        let (token, next) =
                            normal_scan_ref_bytes_impl(normal, &input[start + ref_start..]);
                        (token, next.map_or(0, |next| ref_start + next))
                    },
                );
                normal_scan_lt_result(result.token, result.next.map(|next| start + next))
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

    /// Scans normal-encoding content entirely through bounded byte slices.
    /// The result retains an offset into `input`; only the fixed C scanner
    /// adapter converts that offset back into a cursor.
    fn normal_content_result(
        encoding: &normal_encoding,
        input: &[u8],
    ) -> crate::src::xmltok::ScannerResult {
        match normal_content_tok_impl(&encoding.type_0, input, |offset, width| {
            normal_char_check(
                encoding,
                NormalCharCheck::Invalid,
                width,
                &input[offset..],
                || unknown_character_value_for(encoding.unknown_converter_id, &input[offset..]),
            )
        }) {
            NormalContentAction::Token(token, next) => {
                crate::src::xmltok::ScannerResult::new(token, next)
            }
            NormalContentAction::ScanLt(start) => {
                let result =
                    normal_scan_lt_with_check(encoding, &input[start..], &|kind, offset, width| {
                        normal_char_check(encoding, kind, width, &input[start + offset..], || {
                            unknown_character_value_for(
                                encoding.unknown_converter_id,
                                &input[start + offset..],
                            )
                        })
                    });
                crate::src::xmltok::ScannerResult::new(
                    result.token,
                    result.next.map(|next| start + next),
                )
            }
            NormalContentAction::ScanRef(start) => {
                let (token, next) = normal_scan_ref_bytes_impl(encoding, &input[start..]);
                crate::src::xmltok::ScannerResult::new(token, next.map(|next| start + next))
            }
        }
    }

    pub(super) fn normal_scan_percent_impl<T: XmlTokenByte>(
        enc: &normal_encoding,
        input: &[T],
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

    pub(super) fn normal_scan_pound_name_impl<T: XmlTokenByte>(
        enc: &normal_encoding,
        input: &[T],
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

    /// Scans a normal-encoding literal using offsets within a bounded input
    /// slice.  The pointer adapter below owns the raw cursor contract.
    pub(super) fn normal_scan_lit_impl(
        open: ::core::ffi::c_int,
        enc: &normal_encoding,
        input: &[u8],
        mut is_invalid: impl FnMut(usize, usize) -> bool,
    ) -> (::core::ffi::c_int, Option<usize>) {
        let mut offset = 0;
        while offset < input.len() {
            let t = enc.type_0[input[offset] as usize] as ::core::ffi::c_int;
            match t {
                5 => {
                    if input.len() - offset < 2 {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                    }
                    if is_invalid(offset, 2) {
                        return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                    }
                    offset += 2;
                }
                6 => {
                    if input.len() - offset < 3 {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                    }
                    if is_invalid(offset, 3) {
                        return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                    }
                    offset += 3;
                }
                7 => {
                    if input.len() - offset < 4 {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1, None);
                    }
                    if is_invalid(offset, 4) {
                        return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                    }
                    offset += 4;
                }
                0 | 1 | 8 => {
                    return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset));
                }
                12 | 13 => {
                    offset += 1;
                    if t == open {
                        if offset == input.len() {
                            return (-crate::src::xmltok::XML_TOK_LITERAL_1, None);
                        }
                        return match enc.type_0[input[offset] as usize] as ::core::ffi::c_int {
                            21 | 9 | 10 | 11 | 30 | 20 => {
                                (crate::src::xmltok::XML_TOK_LITERAL_1, Some(offset))
                            }
                            _ => (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
                        };
                    }
                }
                _ => {
                    offset += 1;
                }
            }
        }
        (crate::src::xmltok::XML_TOK_PARTIAL_1, None)
    }

    pub(super) enum NormalPrologCharCheck {
        Invalid,
        NameStart,
        Name,
    }

    pub(super) enum NormalPrologAction {
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

    pub(super) fn normal_prolog_tok_impl<F>(
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

    struct NormalAttributeValueToken {
        token: ::core::ffi::c_int,
        next: Option<usize>,
    }

    fn normal_attribute_value_tok_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> NormalAttributeValueToken {
        if input.is_empty() {
            return NormalAttributeValueToken {
                token: crate::src::xmltok::XML_TOK_NONE_1,
                next: None,
            };
        }

        let mut offset = 0;
        while offset < input.len() {
            match enc.type_0[input[offset] as u8 as usize] as ::core::ffi::c_int {
                kind @ (5 | 6 | 7) => {
                    let width = kind as usize - 3;
                    if input.len() - offset < width {
                        return NormalAttributeValueToken {
                            token: crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                            next: None,
                        };
                    }
                    offset += width;
                }
                3 if offset == 0 => {
                    let (token, next) = normal_scan_ref_impl(enc, &input[1..]);
                    return NormalAttributeValueToken {
                        token,
                        next: next.map(|next| next + 1),
                    };
                }
                3 => {
                    return NormalAttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(offset),
                    };
                }
                2 => {
                    return NormalAttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_INVALID_1,
                        next: Some(offset),
                    };
                }
                10 if offset == 0 => {
                    return NormalAttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_NEWLINE_1,
                        next: Some(1),
                    };
                }
                10 => {
                    return NormalAttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(offset),
                    };
                }
                9 if offset == 0 => {
                    offset += 1;
                    if offset == input.len() {
                        return NormalAttributeValueToken {
                            token: crate::src::xmltok::XML_TOK_TRAILING_CR_1,
                            next: None,
                        };
                    }
                    if enc.type_0[input[offset] as u8 as usize] as ::core::ffi::c_int
                        == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                    {
                        offset += 1;
                    }
                    return NormalAttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_NEWLINE_1,
                        next: Some(offset),
                    };
                }
                9 => {
                    return NormalAttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(offset),
                    };
                }
                21 if offset == 0 => {
                    return NormalAttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_ATTRIBUTE_VALUE_S_1,
                        next: Some(1),
                    };
                }
                21 => {
                    return NormalAttributeValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(offset),
                    };
                }
                _ => offset += 1,
            }
        }

        NormalAttributeValueToken {
            token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
            next: Some(offset),
        }
    }

    struct NormalEntityValueToken {
        token: ::core::ffi::c_int,
        next: Option<usize>,
    }

    fn normal_entity_value_tok_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> NormalEntityValueToken {
        if input.is_empty() {
            return NormalEntityValueToken {
                token: crate::src::xmltok::XML_TOK_NONE_1,
                next: None,
            };
        }

        let mut offset = 0;
        while offset < input.len() {
            match enc.type_0[input[offset] as u8 as usize] as ::core::ffi::c_int {
                kind @ (5 | 6 | 7) => {
                    let width = kind as usize - 3;
                    if input.len() - offset < width {
                        return NormalEntityValueToken {
                            token: crate::src::xmltok::XML_TOK_PARTIAL_1,
                            next: None,
                        };
                    }
                    offset += width;
                }
                3 if offset == 0 => {
                    let (token, next) = normal_scan_ref_impl(enc, &input[1..]);
                    return NormalEntityValueToken {
                        token,
                        next: next.map(|next| next + 1),
                    };
                }
                3 => {
                    return NormalEntityValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(offset),
                    };
                }
                30 if offset == 0 => {
                    let (token, next) = normal_scan_percent_impl(enc, &input[1..]);
                    return NormalEntityValueToken {
                        token: if token == crate::src::xmltok::XML_TOK_PERCENT_1 {
                            crate::src::xmltok::XML_TOK_INVALID_1
                        } else {
                            token
                        },
                        next: next.map(|next| next + 1),
                    };
                }
                30 => {
                    return NormalEntityValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(offset),
                    };
                }
                10 if offset == 0 => {
                    return NormalEntityValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_NEWLINE_1,
                        next: Some(1),
                    };
                }
                10 => {
                    return NormalEntityValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(offset),
                    };
                }
                9 if offset == 0 => {
                    offset += 1;
                    if offset == input.len() {
                        return NormalEntityValueToken {
                            token: crate::src::xmltok::XML_TOK_TRAILING_CR_1,
                            next: None,
                        };
                    }
                    if enc.type_0[input[offset] as u8 as usize] as ::core::ffi::c_int
                        == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                    {
                        offset += 1;
                    }
                    return NormalEntityValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_NEWLINE_1,
                        next: Some(offset),
                    };
                }
                9 => {
                    return NormalEntityValueToken {
                        token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
                        next: Some(offset),
                    };
                }
                _ => offset += 1,
            }
        }

        NormalEntityValueToken {
            token: crate::src::xmltok::XML_TOK_DATA_CHARS_1,
            next: Some(offset),
        }
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
        input: &[u8],
        mut pos: usize,
        mut level: ::core::ffi::c_int,
    ) -> NormalIgnoreSectionOutcome {
        while pos < input.len() {
            match enc.type_0[input[pos] as usize] as ::core::ffi::c_int {
                5 => {
                    if input.len() - pos < 2 {
                        return NormalIgnoreSectionOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    let invalid = match enc.invalid2 {
                        Invalid2Checker::Never => false,
                        Invalid2Checker::Utf8 => utf8_invalid2(&[input[pos], input[pos + 1]]),
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
                        Invalid3Checker::Utf8 => {
                            utf8_invalid3(&[input[pos], input[pos + 1], input[pos + 2]])
                        }
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
                            input[pos],
                            input[pos + 1],
                            input[pos + 2],
                            input[pos + 3],
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

    #[derive(Copy, Clone)]
    enum NormalAttributeAction {
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

    /// Scans a complete single-byte start-tag token using only bounded slice
    /// indices, reporting offsets for the boundary adapter to translate.
    fn scan_normal_atts(
        byte_types: &[::core::ffi::c_uchar; 256],
        source: &[u8],
        mut report: impl FnMut(NormalAttributeAction),
    ) -> ::core::ffi::c_int {
        #[derive(Copy, Clone, Eq, PartialEq)]
        enum State {
            InName,
            InValue,
            Other,
        }

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
                        report(NormalAttributeAction::Name {
                            attribute: n_atts,
                            offset: index,
                        });
                        normalized = true;
                        report(NormalAttributeAction::Normalized {
                            attribute: n_atts,
                            value: 1,
                        });
                        state = State::InName;
                    }
                    index += match kind {
                        5 => 2,
                        6 => 3,
                        7 => 4,
                        _ => 1,
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
                        report(NormalAttributeAction::ValueStart {
                            attribute: n_atts,
                            offset: index + 1,
                        });
                        value_start = Some(index + 1);
                        state = State::InValue;
                        open = quote_kind;
                    } else if open == quote_kind {
                        state = State::Other;
                        report(NormalAttributeAction::ValueEnd {
                            attribute: n_atts,
                            offset: index,
                        });
                        n_atts += 1;
                        value_start = None;
                    }
                }
                3 => {
                    normalized = false;
                    report(NormalAttributeAction::Normalized {
                        attribute: n_atts,
                        value: 0,
                    });
                }
                21 => {
                    if state == State::InName {
                        state = State::Other;
                    } else if state == State::InValue
                        && normalized
                        && (value_start == Some(index)
                            || byte != crate::ascii_h::ASCII_SPACE as u8
                            || source.get(index + 1) == Some(&(crate::ascii_h::ASCII_SPACE as u8))
                            || source
                                .get(index + 1)
                                .map(|next| byte_types[*next as usize] as ::core::ffi::c_int)
                                == Some(open))
                    {
                        normalized = false;
                        report(NormalAttributeAction::Normalized {
                            attribute: n_atts,
                            value: 0,
                        });
                    }
                }
                9 | 10 => {
                    if state == State::InName {
                        state = State::Other;
                    } else if state == State::InValue {
                        normalized = false;
                        report(NormalAttributeAction::Normalized {
                            attribute: n_atts,
                            value: 0,
                        });
                    }
                }
                11 | 17 if state != State::InValue => return n_atts,
                _ => {}
            }
            index += 1;
        }
        n_atts
    }

    /// Skips leading XML whitespace in a caller-validated token window.
    ///
    /// The returned byte offset is relative to `input`; C cursor conversion
    /// belongs to the parser boundary that owns that window.
    pub fn skip_s(
        input: &[::core::ffi::c_char],
        skipper: crate::src::xmltok::WhitespaceSkipper,
    ) -> usize {
        skipper.skip_s_bytes(input)
    }

    pub(crate) fn normal_update_position(
        encoding: &normal_encoding,
        bytes: &[u8],
        pos: &mut crate::src::xmltok::POSITION,
    ) {
        let mut offset = 0;
        while offset < bytes.len() {
            match encoding.type_0[bytes[offset] as u8 as usize] as ::core::ffi::c_int {
                5 => {
                    offset = offset.saturating_add(2);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                6 => {
                    offset = offset.saturating_add(3);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                7 => {
                    offset = offset.saturating_add(4);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                10 => {
                    pos.columnNumber = 0 as crate::expat_external_h::XML_Size;
                    pos.lineNumber = pos.lineNumber.wrapping_add(1);
                    offset += 1;
                }
                9 => {
                    pos.lineNumber = pos.lineNumber.wrapping_add(1);
                    offset += 1;
                    if offset < bytes.len()
                        && encoding.type_0[bytes[offset] as u8 as usize] as ::core::ffi::c_int
                            == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                    {
                        offset += 1;
                    }
                    pos.columnNumber = 0 as crate::expat_external_h::XML_Size;
                }
                _ => {
                    offset += 1;
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

    /// Scans a bounded UTF-16LE comment and returns an offset relative to
    /// `input`.  The caller that owns the ABI cursor translates that offset
    /// back to a pointer after validating the input window.
    pub fn little2_scanComment(
        normal: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> (::core::ffi::c_int, Option<usize>) {
        match little2_scan_comment_impl(normal, input) {
            Little2ScanOutcome::Token(token, next) => (token, Some(next)),
            Little2ScanOutcome::Partial(token) => (token, None),
            Little2ScanOutcome::Invalid(at) => (crate::src::xmltok::XML_TOK_INVALID_1, Some(at)),
        }
    }

    enum Little2ScanDeclAction {
        ScanComment,
        Return {
            token: ::core::ffi::c_int,
            next: Option<usize>,
        },
    }

    /// Scan the declaration prefix using bounded UTF-16LE code units.  The
    /// boundary adapter owns converting the returned cursor offset to a C
    /// pointer.
    fn little2_scan_decl_impl(
        byte_types: &[::core::ffi::c_uchar; 256],
        input: &[::core::ffi::c_char],
    ) -> Little2ScanDeclAction {
        let partial = crate::src::xmltok::XML_TOK_PARTIAL_1;
        let invalid = crate::src::xmltok::XML_TOK_INVALID_1;
        if input.len() < 2 {
            return Little2ScanDeclAction::Return {
                token: partial,
                next: None,
            };
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
            _ => {
                return Little2ScanDeclAction::Return {
                    token: invalid,
                    next: Some(0),
                }
            }
        }

        let mut ptr = 2;
        while input.len().saturating_sub(ptr) >= 2 {
            match little2_byte_type(byte_types, input, ptr) {
                30 => {
                    if input.len().saturating_sub(ptr) < 4 {
                        return Little2ScanDeclAction::Return {
                            token: partial,
                            next: None,
                        };
                    }
                    if matches!(
                        little2_byte_type(byte_types, input, ptr + 2),
                        21 | 9 | 10 | 30
                    ) {
                        return Little2ScanDeclAction::Return {
                            token: invalid,
                            next: Some(ptr),
                        };
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
                _ => {
                    return Little2ScanDeclAction::Return {
                        token: invalid,
                        next: Some(ptr),
                    }
                }
            }
        }

        Little2ScanDeclAction::Return {
            token: partial,
            next: None,
        }
    }

    /// Scans a bounded UTF-16LE declaration prefix and returns an offset
    /// relative to `input`.  ABI cursor conversion belongs to the boundary
    /// that validates the input window.
    pub fn little2_scanDecl(
        normal: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> (::core::ffi::c_int, Option<usize>) {
        match little2_scan_decl_impl(&normal.type_0, input) {
            Little2ScanDeclAction::ScanComment => {
                let (token, next) = little2_scanComment(normal, &input[2..]);
                (token, next.map(|offset| 2 + offset))
            }
            Little2ScanDeclAction::Return { token, next } => (token, next),
        }
    }

    /// Classifies a bounded UTF-16LE processing-instruction target.  Keeping
    /// the target as a slice ensures that the six-byte `xml` check below is
    /// only performed after its bounds have been validated by the caller.
    pub fn little2_checkPiTarget(
        target: &[::core::ffi::c_char],
        token: &mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        *token = crate::src::xmltok::XML_TOK_PI_1;
        if target.len() != 6 {
            return 1 as ::core::ffi::c_int;
        }

        match little2_pi_target_token(target, 0, 6) {
            Some(classified_token) => {
                *token = classified_token;
                1
            }
            None => 0,
        }
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

    fn little2_scan_hex_char_ref_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> (::core::ffi::c_int, Option<usize>) {
        if input.len() < 2 {
            return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }

        match little2_byte_type(&enc.type_0, input, 0) {
            25 | 24 => {}
            _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(0)),
        }

        let mut offset = 2;
        while offset + 2 <= input.len() {
            match little2_byte_type(&enc.type_0, input, offset) {
                25 | 24 => offset += 2,
                18 => return (crate::src::xmltok::XML_TOK_CHAR_REF_1, Some(offset + 2)),
                _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
            }
        }

        (crate::src::xmltok::XML_TOK_PARTIAL_1, None)
    }

    pub unsafe extern "C" fn little2_scanHexCharRef(
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
        let (token, next) = little2_scan_hex_char_ref_impl(normal, input);
        if let Some(offset) = next {
            *nextTokPtr = ptr.add(offset);
        }
        token
    }

    pub unsafe extern "C" fn little2_scanCharRef(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let input_len = end.offset_from(ptr);
        // Avoid constructing a slice from an empty C range.  The scanner has
        // no token to inspect until a complete UTF-16 code unit is available.
        if input_len < 2 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        let input = ::core::slice::from_raw_parts(ptr, input_len as usize);
        let normal = &*(enc as *const normal_encoding);
        let result = little2_scan_char_ref_impl(&normal.type_0, input, 0);
        if let Some(next) = result.next {
            *nextTokPtr = ptr.add(next);
        }
        result.token
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

    fn little2_position_byte_type(
        byte_types: &[::core::ffi::c_uchar; 256],
        input: &[u8],
        offset: usize,
    ) -> ::core::ffi::c_int {
        let lo = input[offset];
        let hi = input[offset + 1];
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
            let result = little2_scan_ref_impl(normal, &input[ref_start..]);
            (result.token, result.next.map_or(0, |next| ref_start + next))
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

    /// Scans the `CDATA[` prefix after `<!` using the tokenizer's bounded
    /// UTF-16LE input.  The raw scanner historically reported the beginning
    /// of this prefix, rather than the mismatching byte, on failure.
    fn little2_scan_cdata_section_open_impl(input: &[u8]) -> Little2ScanResult {
        const CDATA_LSQB: [u8; 12] = [b'C', 0, b'D', 0, b'A', 0, b'T', 0, b'A', 0, b'[', 0];

        if input.len() < CDATA_LSQB.len() {
            return Little2ScanResult {
                token: crate::src::xmltok::XML_TOK_PARTIAL_1,
                next: None,
            };
        }
        if input[..CDATA_LSQB.len()] != CDATA_LSQB {
            return Little2ScanResult {
                token: crate::src::xmltok::XML_TOK_INVALID_1,
                next: Some(0),
            };
        }
        Little2ScanResult {
            token: crate::src::xmltok::XML_TOK_CDATA_SECT_OPEN_1,
            next: Some(CDATA_LSQB.len()),
        }
    }

    /// Resolves the follow-up scanner selected by a UTF-16LE `<` token using
    /// bounded views of the same input.  Keeping the offsets relative to the
    /// original input lets the pointer adapter perform a single final cursor
    /// update without re-entering another raw scanner.
    fn little2_scan_lt_result(
        enc: &normal_encoding,
        bytes: &[u8],
        chars: &[::core::ffi::c_char],
    ) -> Little2ScanResult {
        let result = match little2_scan_lt_impl(enc, bytes) {
            Little2ScanLtAction::Token(token, next) => return Little2ScanResult { token, next },
            Little2ScanLtAction::Comment(start) => {
                match little2_scan_comment_impl(enc, &chars[start..]) {
                    Little2ScanOutcome::Token(token, next) => (token, Some(start + next)),
                    Little2ScanOutcome::Partial(token) => (token, None),
                    Little2ScanOutcome::Invalid(at) => {
                        (crate::src::xmltok::XML_TOK_INVALID_1, Some(start + at))
                    }
                }
            }
            Little2ScanLtAction::CdataSection(start) => {
                let result = little2_scan_cdata_section_open_impl(&bytes[start..]);
                (result.token, result.next.map(|next| start + next))
            }
            Little2ScanLtAction::ProcessingInstruction(start) => {
                let (token, next) = little2_scan_pi_impl(enc, &chars[start..]);
                (token, next.map(|next| start + next))
            }
            Little2ScanLtAction::EndTag(start) => {
                match little2_scan_end_tag_impl(enc, &bytes[start..]) {
                    Little2ScanLtAction::Token(token, next) => {
                        (token, next.map(|next| start + next))
                    }
                    _ => unreachable!("end-tag scanning produces only token outcomes"),
                }
            }
            Little2ScanLtAction::Attributes(start) => {
                let result = little2_scan_atts_impl(&enc.type_0, &chars[start..], |ref_start| {
                    let result = little2_scan_ref_impl(enc, &chars[start + ref_start..]);
                    (result.token, result.next.map_or(0, |next| ref_start + next))
                });
                (result.token, result.next.map(|next| start + next))
            }
        };
        Little2ScanResult {
            token: result.0,
            next: result.1,
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

    /// Scans UTF-16LE content over a validated character slice.  The returned
    /// cursor remains an offset into `input`, including for the `<` and `&`
    /// follow-up scanners, so callers never need to re-enter a raw scanner.
    pub fn little2_contentTok(
        encoding: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> (::core::ffi::c_int, Option<usize>) {
        match little2_content_tok_impl(encoding, input) {
            Little2ContentToken::Result(token, next) => (token, next),
            Little2ContentToken::ScanLt => {
                let end = input.len() & !1;
                let input = &input[2..end];
                let result = little2_scan_lt_result(encoding, bytemuck::cast_slice(input), input);
                (result.token, result.next.map(|next| 2 + next))
            }
            Little2ContentToken::ScanRef => {
                let end = input.len() & !1;
                let result = little2_scan_ref_impl(encoding, &input[2..end]);
                (result.token, result.next.map(|next| 2 + next))
            }
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
                9 | 10 | 11 | 21 | 30 | 32 | 36 => {
                    return (crate::src::xmltok::XML_TOK_POUND_NAME_1, Some(offset))
                }
                _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(offset)),
            }
        }

        (-crate::src::xmltok::XML_TOK_POUND_NAME_1, None)
    }

    /// Scans a UTF-16LE pound-name from a validated bounded input.
    ///
    /// The scanner reports the next cursor as an offset.  Boundary code that
    /// needs a C pointer is responsible for validating its input range and
    /// translating this offset after the scan completes.
    pub fn little2_scanPoundName(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> (::core::ffi::c_int, Option<usize>) {
        little2_scan_pound_name_impl(enc, input)
    }

    /// Scans a UTF-16LE literal using offsets into a bounded input slice.
    /// The pointer adapter below only creates that slice and translates the
    /// resulting offset back to the ABI cursor pointer.
    fn little2_scan_lit_impl(
        open: ::core::ffi::c_int,
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> Little2ScanOutcome {
        let mut offset = 0;
        while input.len() - offset >= 2 {
            let t = little2_byte_type(&enc.type_0, input, offset);
            match t {
                5 => {
                    if input.len() - offset < 2 {
                        return Little2ScanOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    offset += 2;
                }
                6 => {
                    if input.len() - offset < 3 {
                        return Little2ScanOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    offset += 3;
                }
                7 => {
                    if input.len() - offset < 4 {
                        return Little2ScanOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    offset += 4;
                }
                0 | 1 | 8 => return Little2ScanOutcome::Invalid(offset),
                12 | 13 => {
                    offset += 2;
                    if t == open {
                        if input.len() - offset < 2 {
                            return Little2ScanOutcome::Partial(
                                -crate::src::xmltok::XML_TOK_LITERAL_1,
                            );
                        }
                        return match little2_byte_type(&enc.type_0, input, offset) {
                            21 | 9 | 10 | 11 | 30 | 20 => Little2ScanOutcome::Token(
                                crate::src::xmltok::XML_TOK_LITERAL_1,
                                offset,
                            ),
                            _ => Little2ScanOutcome::Invalid(offset),
                        };
                    }
                }
                _ => offset += 2,
            }
        }
        Little2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1)
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

    fn little2_prolog_tok_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> Little2PrologAction {
        let len = input.len();
        let result = |token, next| Little2PrologAction::Return { token, next };
        let mut ptr = 0usize;
        let mut tok: ::core::ffi::c_int;
        let first = little2_byte_type(&enc.type_0, input, ptr);
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
                return match little2_byte_type(&enc.type_0, input, ptr) {
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
                    match little2_byte_type(&enc.type_0, input, ptr) {
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
                if input[ptr + 1] == 0 && input[ptr] == b']' as ::core::ffi::c_char {
                    if len - ptr < 4 {
                        return result(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                    }
                    if input[ptr + 3] == 0 && input[ptr + 2] == b'>' as ::core::ffi::c_char {
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
                return match little2_byte_type(&enc.type_0, input, ptr) {
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
                if little2_in_name_bitmap(input, 0, &nmstrtPages) {
                    tok = crate::src::xmltok::XML_TOK_NAME;
                    ptr = 2;
                } else if little2_in_name_bitmap(input, 0, &namePages) {
                    tok = crate::src::xmltok::XML_TOK_NMTOKEN_1;
                    ptr = 2;
                } else {
                    return result(crate::src::xmltok::XML_TOK_INVALID_1, Some(0));
                }
            }
            _ => return result(crate::src::xmltok::XML_TOK_INVALID_1, Some(0)),
        }

        while len - ptr >= 2 {
            match little2_byte_type(&enc.type_0, input, ptr) {
                29 if !little2_in_name_bitmap(input, ptr, &namePages) => {
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
                            match little2_byte_type(&enc.type_0, input, ptr) {
                                29 if !little2_in_name_bitmap(input, ptr, &namePages) => {
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

    fn little2_scan_outcome_token(
        outcome: Little2ScanOutcome,
        base: usize,
    ) -> (::core::ffi::c_int, Option<usize>) {
        match outcome {
            Little2ScanOutcome::Token(token, next) => (token, Some(base + next)),
            Little2ScanOutcome::Partial(token) => (token, None),
            Little2ScanOutcome::Invalid(at) => {
                (crate::src::xmltok::XML_TOK_INVALID_1, Some(base + at))
            }
        }
    }

    /// Scans a UTF-16LE prolog token from bounded input.  The ABI wrapper
    /// below is solely responsible for translating its offset back to a C
    /// cursor.
    fn little2_prolog_tok(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> (::core::ffi::c_int, Option<usize>) {
        let input = &input[..input.len() & !1];
        match little2_prolog_tok_impl(enc, input) {
            Little2PrologAction::Return { token, next } => (token, next),
            Little2PrologAction::ScanLit { open, start } => {
                little2_scan_outcome_token(little2_scan_lit_impl(open, enc, &input[start..]), start)
            }
            Little2PrologAction::ScanDecl { start } => {
                match little2_scan_decl_impl(&enc.type_0, &input[start..]) {
                    Little2ScanDeclAction::ScanComment => little2_scan_outcome_token(
                        little2_scan_comment_impl(enc, &input[start + 2..]),
                        start + 2,
                    ),
                    Little2ScanDeclAction::Return { token, next } => {
                        (token, next.map(|next| start + next))
                    }
                }
            }
            Little2PrologAction::ScanPi { start } => {
                let (token, next) = little2_scan_pi_impl(enc, &input[start..]);
                (token, next.map(|next| start + next))
            }
            Little2PrologAction::ScanPercent { start } => {
                let (token, next) = little2_scan_percent_impl(enc, &input[start..]);
                (token, next.map(|next| start + next))
            }
            Little2PrologAction::ScanPoundName { start } => {
                let (token, next) = little2_scan_pound_name_impl(enc, &input[start..]);
                (token, next.map(|next| start + next))
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
                        Little2ScanResult { token, next: None } => {
                            Little2AttributeValueToken { token, next: None }
                        }
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
                        Little2ScanResult { token, next: None } => {
                            Little2EntityValueToken { token, next: None }
                        }
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

    /// Decodes one validated UTF-16LE code unit once its two bytes have crossed
    /// the tokenizer boundary.
    fn little2_char_ref_unit(
        low: ::core::ffi::c_char,
        high: ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let low = low as ::core::ffi::c_int;
        let high = high as ::core::ffi::c_int;
        if high == 0 {
            low
        } else {
            -1
        }
    }

    fn decode_char_ref_number(
        mut next_unit: impl FnMut() -> ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        let mut result: ::core::ffi::c_int = 0;
        let first = next_unit();
        if first == crate::ascii_h::ASCII_x {
            loop {
                let c = next_unit();
                if c == 0x3b {
                    break;
                }
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
                        result <<= 4;
                        result |= c - crate::ascii_h::ASCII_0;
                    }
                    crate::ascii_h::ASCII_A
                    | crate::ascii_h::ASCII_B_1
                    | crate::ascii_h::ASCII_C
                    | crate::ascii_h::ASCII_D
                    | crate::ascii_h::ASCII_E_1
                    | crate::ascii_h::ASCII_F_1 => {
                        result <<= 4;
                        result += 10 + (c - crate::ascii_h::ASCII_A);
                    }
                    crate::ascii_h::ASCII_a_1
                    | crate::ascii_h::ASCII_b
                    | crate::ascii_h::ASCII_c_1
                    | crate::ascii_h::ASCII_d
                    | crate::ascii_h::ASCII_e_1
                    | crate::ascii_h::ASCII_f => {
                        result <<= 4;
                        result += 10 + (c - crate::ascii_h::ASCII_a_1);
                    }
                    _ => {}
                }
                if result >= 0x110000 {
                    return -1;
                }
            }
        } else {
            let mut c = first;
            while c != 0x3b {
                result *= 10;
                result += c - crate::ascii_h::ASCII_0;
                if result >= 0x110000 {
                    return -1;
                }
                c = next_unit();
            }
        }
        checkCharRefNumber(result)
    }

    pub(crate) fn little2_update_position(
        encoding: &normal_encoding,
        bytes: &[u8],
        pos: &mut crate::src::xmltok::POSITION,
    ) {
        let mut offset = 0;
        while bytes.len().saturating_sub(offset) >= 2 {
            match little2_position_byte_type(&encoding.type_0, bytes, offset) {
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
                        && little2_position_byte_type(&encoding.type_0, bytes, offset)
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

    fn big2_scan_outcome_result(
        outcome: Big2ScanOutcome,
        base: usize,
    ) -> (::core::ffi::c_int, Option<usize>) {
        match outcome {
            Big2ScanOutcome::Token(token, next) => (token, Some(base + next)),
            Big2ScanOutcome::Partial(token) => (token, None),
            Big2ScanOutcome::Invalid(at) => {
                (crate::src::xmltok::XML_TOK_INVALID_1, Some(base + at))
            }
        }
    }

    /// Scans the part of a declaration following `<!` using offsets into the
    /// supplied UTF-16BE input. Prolog scanning uses this bounded form
    /// directly.
    fn big2_scan_decl_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> (::core::ffi::c_int, Option<usize>) {
        if input.len() < 2 {
            return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
        }

        match big2_byte_type(enc, input, 0) {
            27 => return big2_scan_outcome_result(big2_scan_comment_impl(enc, &input[2..]), 2),
            20 => return (crate::src::xmltok::XML_TOK_COND_SECT_OPEN_1, Some(2)),
            22 | 24 => {}
            _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(0)),
        }

        let mut pos = 2;
        let mut next = 2;
        while input.len() - pos >= 2 {
            match big2_byte_type(enc, input, pos) {
                30 => {
                    if input.len() - pos < 4 {
                        return (crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                    }
                    match big2_byte_type(enc, input, pos + 2) {
                        21 | 9 | 10 | 30 => {
                            return (crate::src::xmltok::XML_TOK_INVALID_1, Some(next));
                        }
                        _ => {}
                    }
                }
                21 | 9 | 10 => {}
                22 | 24 => {
                    pos += 2;
                    next += 2;
                    continue;
                }
                _ => return (crate::src::xmltok::XML_TOK_INVALID_1, Some(next)),
            }
            return (crate::src::xmltok::XML_TOK_DECL_OPEN_1, Some(next));
        }
        (crate::src::xmltok::XML_TOK_PARTIAL_1, None)
    }

    /// Classifies a complete UTF-16BE processing-instruction target.
    ///
    /// The tokenizer hands this helper the already bounded target range, so
    /// checking `xml` never requires dereferencing a cursor or deriving a
    /// range from unrelated raw endpoints.
    pub fn big2_checkPiTarget(
        target: &[::core::ffi::c_char],
        token: &mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        *token = crate::src::xmltok::XML_TOK_PI_1;
        match big2_pi_target_token(target, 0, target.len()) {
            Some(classified_token) => {
                *token = classified_token;
                1
            }
            None => 0,
        }
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

    fn big2_position_byte_type(
        enc: &normal_encoding,
        input: &[u8],
        offset: usize,
    ) -> ::core::ffi::c_int {
        let hi = input[offset];
        let lo = input[offset + 1];
        if hi == 0 {
            enc.type_0[lo as usize] as ::core::ffi::c_int
        } else {
            unicode_byte_type(hi as ::core::ffi::c_char, lo as ::core::ffi::c_char)
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

    // The parser owns its input buffer, so CDATA tokenization uses a checked
    // slice and returns an offset rather than exchanging raw cursors.
    pub(crate) fn cdata_token(
        normal: &normal_encoding,
        input: &[u8],
    ) -> (::core::ffi::c_int, Option<usize>) {
        match normal.enc.scanners[2] {
            crate::src::xmltok::Scanner::NormalCdataSection => {
                normal_cdata_section_tok(input, &normal.type_0, normal.enc.isUtf8 != 0)
            }
            crate::src::xmltok::Scanner::Little2CdataSection => {
                let result = little2_cdata_section_tok_impl(normal, input);
                (result.token, result.next)
            }
            crate::src::xmltok::Scanner::Big2CdataSection => {
                let result = big2_cdata_section_tok_impl(normal, input);
                (result.token, result.next)
            }
            _ => (crate::src::xmltok::XML_TOK_INVALID_1, Some(0)),
        }
    }

    // Internal entity replacement text is retained as `XML_Char` data.  Its
    // encoding is always the parser's UTF-8 internal encoding, so scan that
    // bounded character slice directly instead of reinterpreting it through
    // a raw byte pointer.
    pub(crate) fn internal_cdata_token(
        normal: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> (::core::ffi::c_int, Option<usize>) {
        match normal.enc.scanners[2] {
            crate::src::xmltok::Scanner::NormalCdataSection => {
                normal_cdata_section_tok(input, &normal.type_0, normal.enc.isUtf8 != 0)
            }
            _ => (crate::src::xmltok::XML_TOK_INVALID_1, Some(0)),
        }
    }

    // Kept as a disabled translation reference while the checked slice
    // dispatcher replaces it.
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

    pub enum Big2ScanOutcome {
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

    fn big2_scan_hex_char_ref_impl(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> Big2ScanOutcome {
        if input.len() < 2 {
            return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
        }
        match big2_byte_type(enc, input, 0) {
            24 | 25 => {}
            _ => return Big2ScanOutcome::Invalid(0),
        }

        let mut pos = 2;
        while pos + 2 <= input.len() {
            match big2_byte_type(enc, input, pos) {
                24 | 25 => pos += 2,
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
                    return Big2ScanOutcome::Token(crate::src::xmltok::XML_TOK_POUND_NAME_1, offset)
                }
                _ => match big2_check_name(enc, input, offset, false) {
                    Big2NameCheck::Advance(next) => offset = next,
                    Big2NameCheck::PartialChar => {
                        return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1)
                    }
                    Big2NameCheck::Invalid => return Big2ScanOutcome::Invalid(offset),
                },
            }
        }

        Big2ScanOutcome::Partial(-crate::src::xmltok::XML_TOK_POUND_NAME_1)
    }

    /// Scans a big-endian UTF-16 pound-name from a validated bounded input.
    ///
    /// Callers that need a cursor can translate the returned offset only after
    /// they have established the lifetime and extent of their input slice.
    pub fn big2_scanPoundName(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> Big2ScanOutcome {
        big2_scan_pound_name_impl(enc, input)
    }

    /// Scans a big-endian UTF-16 literal using offsets within a bounded input
    /// slice.  The pointer adapter below is responsible only for constructing
    /// that slice and translating an offset back to `nextTokPtr`.
    fn big2_scan_lit_impl(
        open: ::core::ffi::c_int,
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> Big2ScanOutcome {
        let mut offset = 0;
        while input.len() - offset >= 2 {
            let t = big2_byte_type(enc, input, offset);
            match t {
                5 => {
                    if input.len() - offset < 2 {
                        return Big2ScanOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    offset += 2;
                }
                6 => {
                    if input.len() - offset < 3 {
                        return Big2ScanOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    offset += 3;
                }
                7 => {
                    if input.len() - offset < 4 {
                        return Big2ScanOutcome::Partial(
                            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1,
                        );
                    }
                    offset += 4;
                }
                0 | 1 | 8 => return Big2ScanOutcome::Invalid(offset),
                12 | 13 => {
                    offset += 2;
                    if t == open {
                        if input.len() - offset < 2 {
                            return Big2ScanOutcome::Partial(
                                -crate::src::xmltok::XML_TOK_LITERAL_1,
                            );
                        }
                        return match big2_byte_type(enc, input, offset) {
                            21 | 9 | 10 | 11 | 30 | 20 => Big2ScanOutcome::Token(
                                crate::src::xmltok::XML_TOK_LITERAL_1,
                                offset,
                            ),
                            _ => Big2ScanOutcome::Invalid(offset),
                        };
                    }
                }
                _ => offset += 2,
            }
        }
        Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1)
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

    /// Scans a UTF-16BE prolog token from a bounded input slice.  The caller
    /// translates the returned offset back to Expat's C cursor.
    pub fn big2_prologTok(
        enc: &normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> (::core::ffi::c_int, Option<usize>) {
        let input = &input[..input.len() & !1];
        match big2_prolog_tok_impl(enc, input) {
            Big2PrologToken::Result(token, next) => (token, next),
            Big2PrologToken::ScanLit(open, offset) => {
                big2_scan_outcome_result(big2_scan_lit_impl(open, enc, &input[offset..]), offset)
            }
            Big2PrologToken::ScanDecl(offset) => {
                let (token, next) = big2_scan_decl_impl(enc, &input[offset..]);
                (token, next.map(|next| offset + next))
            }
            Big2PrologToken::ScanPi(offset) => {
                let (token, next) = big2_scan_pi_impl(enc, &input[offset..]);
                (token, next.map(|next| offset + next))
            }
            Big2PrologToken::ScanPercent(offset) => {
                big2_scan_outcome_result(big2_scan_percent_impl(enc, &input[offset..]), offset)
            }
            Big2PrologToken::ScanPoundName(offset) => {
                big2_scan_outcome_result(big2_scan_pound_name_impl(enc, &input[offset..]), offset)
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
                        Big2ScanOutcome::Partial(token) => {
                            Big2AttributeValueToken { token, next: None }
                        }
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

    /// Scans one attribute-value token from an input window that the caller
    /// has already bounded.  This is the parser-facing counterpart to the
    /// three C-cursor adapters above: it retains the selected normal-encoding
    /// table, but exposes the next position as an offset into `input`.
    pub(crate) fn attribute_value_token(
        enc: &normal_encoding,
        scanner: crate::src::xmltok::LiteralScanner,
        input: &[::core::ffi::c_char],
    ) -> crate::src::xmltok::ScannerResult {
        let (token, next) = match scanner {
            crate::src::xmltok::LiteralScanner::NormalAttributeValue => {
                let result = normal_attribute_value_tok_impl(enc, input);
                (result.token, result.next)
            }
            crate::src::xmltok::LiteralScanner::Little2AttributeValue => {
                let result = little2_attribute_value_tok_impl(enc, input);
                (result.token, result.next)
            }
            crate::src::xmltok::LiteralScanner::Big2AttributeValue => {
                let result = big2_attribute_value_tok_impl(enc, input);
                (result.token, result.next)
            }
            _ => unreachable!("attribute literal scanner must match its table slot"),
        };
        crate::src::xmltok::ScannerResult { token, next }
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
                        Big2ScanOutcome::Partial(token) => {
                            Big2EntityValueToken { token, next: None }
                        }
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
                        Big2ScanOutcome::Partial(token) => {
                            Big2EntityValueToken { token, next: None }
                        }
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

    /// Scans an entity value from an already-bounded XML-character slice.
    /// The C cursor adapters above remain for the public tokenizer ABI, while
    /// parser implementation code uses this offset-based path.
    pub(crate) fn scan_entity_value(
        encoding: &crate::src::xmltok::normal_encoding,
        input: &[::core::ffi::c_char],
    ) -> crate::src::xmltok::ScannerResult {
        match encoding.enc.literalScanners[1] {
            crate::src::xmltok::LiteralScanner::NormalEntityValue => {
                let result = normal_entity_value_tok_impl(encoding, input);
                crate::src::xmltok::ScannerResult::new(result.token, result.next)
            }
            crate::src::xmltok::LiteralScanner::Little2EntityValue => {
                let result = little2_entity_value_tok_impl(encoding, input);
                crate::src::xmltok::ScannerResult::new(result.token, result.next)
            }
            crate::src::xmltok::LiteralScanner::Big2EntityValue => {
                let result = big2_entity_value_tok_impl(encoding, input);
                crate::src::xmltok::ScannerResult::new(result.token, result.next)
            }
            _ => unreachable!("entity literal scanner must match its table slot"),
        }
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
                            return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
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
                            return Big2ScanOutcome::Partial(crate::src::xmltok::XML_TOK_PARTIAL_1);
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

    /// Fills parser-owned attribute records from one already-validated
    /// start-tag slice.  `ATTRIBUTE` retains only offsets, so the scanner's
    /// transient input address cannot escape into parser scratch state.
    pub(crate) fn scan_atts(
        scanner: crate::src::xmltok::AttributeScanner,
        byte_types: &[::core::ffi::c_uchar; 256],
        source: crate::src::xmltok::AttributeSource<'_>,
        attributes: &mut [crate::src::xmltok::ATTRIBUTE],
    ) -> ::core::ffi::c_int {
        // The token scanners operate on unsigned byte values.  Entity text is
        // already a checked `XML_Char` slice, so convert its signed storage
        // values without changing their byte representation rather than
        // constructing another slice with a raw pointer and a cursor-derived
        // length.
        let entity_bytes;
        let source = match source {
            crate::src::xmltok::AttributeSource::Bytes(bytes) => bytes,
            crate::src::xmltok::AttributeSource::Chars(chars) => {
                entity_bytes = chars.iter().map(|&byte| byte as u8).collect::<Vec<_>>();
                &entity_bytes
            }
        };
        let mut store = |attribute: ::core::ffi::c_int, update: AttributeUpdate| {
            let Ok(attribute) = usize::try_from(attribute) else {
                return;
            };
            let Some(slot) = attributes.get_mut(attribute) else {
                return;
            };
            match update {
                AttributeUpdate::Name(offset) => slot.name = offset,
                AttributeUpdate::ValueStart(offset) => slot.valueStart = offset,
                AttributeUpdate::ValueEnd(offset) => slot.valueEnd = offset,
                AttributeUpdate::Normalized(value) => slot.normalized = value,
            }
        };

        match scanner {
            crate::src::xmltok::AttributeScanner::Normal => {
                scan_normal_atts(byte_types, source, |action| match action {
                    NormalAttributeAction::Name { attribute, offset } => {
                        store(attribute, AttributeUpdate::Name(offset))
                    }
                    NormalAttributeAction::ValueStart { attribute, offset } => {
                        store(attribute, AttributeUpdate::ValueStart(offset))
                    }
                    NormalAttributeAction::ValueEnd { attribute, offset } => {
                        store(attribute, AttributeUpdate::ValueEnd(offset))
                    }
                    NormalAttributeAction::Normalized { attribute, value } => {
                        store(attribute, AttributeUpdate::Normalized(value))
                    }
                })
            }
            crate::src::xmltok::AttributeScanner::Little2 => {
                scan_little2_atts(byte_types, source, |action| match action {
                    Little2AttributeAction::Name { attribute, offset } => {
                        store(attribute, AttributeUpdate::Name(offset))
                    }
                    Little2AttributeAction::ValueStart { attribute, offset } => {
                        store(attribute, AttributeUpdate::ValueStart(offset))
                    }
                    Little2AttributeAction::ValueEnd { attribute, offset } => {
                        store(attribute, AttributeUpdate::ValueEnd(offset))
                    }
                    Little2AttributeAction::Normalized { attribute, value } => {
                        store(attribute, AttributeUpdate::Normalized(value))
                    }
                })
            }
            crate::src::xmltok::AttributeScanner::Big2 => {
                scan_big2_atts(byte_types, source, |action| match action {
                    Big2AttributeAction::Name { attribute, offset } => {
                        store(attribute, AttributeUpdate::Name(offset))
                    }
                    Big2AttributeAction::ValueStart { attribute, offset } => {
                        store(attribute, AttributeUpdate::ValueStart(offset))
                    }
                    Big2AttributeAction::ValueEnd { attribute, offset } => {
                        store(attribute, AttributeUpdate::ValueEnd(offset))
                    }
                    Big2AttributeAction::Normalized { attribute, value } => {
                        store(attribute, AttributeUpdate::Normalized(value))
                    }
                })
            }
        }
    }

    enum AttributeUpdate {
        Name(usize),
        ValueStart(usize),
        ValueEnd(usize),
        Normalized(::core::ffi::c_char),
    }

    /// Executes a fixed tokenizer scanner on views whose bounds were already
    /// checked by its caller.  The result remains an offset, so this internal
    /// dispatch never needs a C cursor or an unsafe function pointer.
    pub(super) fn scan_result(
        scanner: crate::src::xmltok::Scanner,
        encoding: &normal_encoding,
        input: crate::src::xmltok::ScannerInput<'_>,
        unknown_converter_id: Option<usize>,
    ) -> crate::src::xmltok::ScannerResult {
        use crate::src::xmltok::{Scanner, ScannerResult};

        let normal_prolog = || {
            let bytes = input.bytes;
            let action = normal_prolog_tok_impl(encoding, bytes, |kind, offset, width| {
                let kind = match kind {
                    NormalPrologCharCheck::Invalid => NormalCharCheck::Invalid,
                    NormalPrologCharCheck::NameStart => NormalCharCheck::NameStart,
                    NormalPrologCharCheck::Name => NormalCharCheck::Name,
                };
                normal_char_check(encoding, kind, width, &bytes[offset..], || {
                    unknown_character_value_for(unknown_converter_id, &bytes[offset..])
                })
            });
            match action {
                NormalPrologAction::Token(token, next) => ScannerResult::new(
                    token,
                    (token >= crate::src::xmltok::XML_TOK_INVALID_1
                        || token == -crate::src::xmltok::XML_TOK_PROLOG_S_1)
                        .then_some(next),
                ),
                NormalPrologAction::Literal(open, start) => {
                    let (token, next) = normal_scan_lit_impl(
                        open,
                        encoding,
                        &bytes[start..],
                        |offset, width| {
                            normal_char_check(
                                encoding,
                                NormalCharCheck::Invalid,
                                width,
                                &bytes[start + offset..],
                                || unknown_character_value_for(
                                    unknown_converter_id,
                                    &bytes[start + offset..],
                                ),
                            )
                        },
                    );
                    ScannerResult::new(token, next.map(|offset| start + offset))
                }
                NormalPrologAction::Declaration(start) => {
                    match normal_scan_decl_impl(&encoding.type_0, &bytes[start..]) {
                        NormalScanDeclAction::Comment => {
                            let comment_start = start + 1;
                            let (token, next) = normal_scan_comment_impl(
                                &encoding.type_0,
                                &bytes[comment_start..],
                                |offset, width| {
                                    normal_char_check(
                                        encoding,
                                        NormalCharCheck::Invalid,
                                        width,
                                        &bytes[comment_start + offset..],
                                        || unknown_character_value_for(
                                            unknown_converter_id,
                                            &bytes[comment_start + offset..],
                                        ),
                                    )
                                },
                            );
                            ScannerResult::new(token, next.map(|offset| comment_start + offset))
                        }
                        NormalScanDeclAction::Token(token, next) => {
                            ScannerResult::new(token, next.map(|offset| start + offset))
                        }
                    }
                }
                NormalPrologAction::ProcessingInstruction(start) => {
                    let (token, next) = normal_scan_pi_impl(encoding, &bytes[start..]);
                    ScannerResult::new(token, next.map(|offset| start + offset))
                }
                NormalPrologAction::Percent(start) => {
                    let (token, next) = normal_scan_percent_impl(encoding, &bytes[start..]);
                    ScannerResult::new(token, next.map(|offset| start + offset))
                }
                NormalPrologAction::PoundName(start) => {
                    let (token, next) = normal_scan_pound_name_impl(encoding, &bytes[start..]);
                    ScannerResult::new(token, next.map(|offset| start + offset))
                }
            }
        };

        let normal_content = || {
            debug_assert_eq!(unknown_converter_id, encoding.unknown_converter_id);
            normal_content_result(encoding, input.bytes)
        };

        let little2_content = || {
            let (token, next) = little2_contentTok(encoding, input.chars);
            ScannerResult::new(token, next)
        };

        let big2_result = |result: Big2ScanOutcome| match result {
            Big2ScanOutcome::Token(token, next) => ScannerResult::new(token, Some(next)),
            Big2ScanOutcome::Partial(token) => ScannerResult::new(token, None),
            Big2ScanOutcome::Invalid(at) => {
                ScannerResult::new(crate::src::xmltok::XML_TOK_INVALID_1, Some(at))
            }
        };
        let big2_content = || match big2_content_tok_impl(encoding, input.chars) {
            Big2ContentToken::Result(token, next) => ScannerResult::new(token, next),
            Big2ContentToken::ScanLt => {
                let result = big2_scan_lt_impl(encoding, &input.chars[2..]);
                match big2_result(result) {
                    ScannerResult { token, next } => ScannerResult::new(token, next.map(|next| 2 + next)),
                }
            }
            Big2ContentToken::ScanRef => {
                let result = big2_scan_ref(encoding, &input.chars[2..], 0);
                match big2_result(result) {
                    ScannerResult { token, next } => ScannerResult::new(token, next.map(|next| 2 + next)),
                }
            }
        };

        match scanner {
            Scanner::NormalProlog => normal_prolog(),
            Scanner::NormalContent => normal_content(),
            Scanner::NormalCdataSection => {
                let (token, next) = normal_cdata_section_tok(
                    input.bytes,
                    &encoding.type_0,
                    encoding.enc.isUtf8 != 0,
                );
                ScannerResult::new(token, next)
            }
            Scanner::NormalIgnoreSection => {
                let mut start = 0;
                let mut level = 0;
                loop {
                    match normal_ignore_section_tok_impl(encoding, input.bytes, start, level) {
                        NormalIgnoreSectionOutcome::Token(token, next) => {
                            break ScannerResult::new(token, Some(next));
                        }
                        NormalIgnoreSectionOutcome::Partial(token) => break ScannerResult::new(token, None),
                        NormalIgnoreSectionOutcome::Invalid(at) => {
                            break ScannerResult::new(crate::src::xmltok::XML_TOK_INVALID_1, Some(at));
                        }
                        NormalIgnoreSectionOutcome::UnknownInvalid { at, width, level: saved_level } => {
                            if unknown_is_invalid(unknown_character_value_for(
                                unknown_converter_id,
                                &input.bytes[at..],
                            )) {
                                break ScannerResult::new(crate::src::xmltok::XML_TOK_INVALID_1, Some(at));
                            }
                            start = at + width;
                            level = saved_level;
                        }
                    }
                }
            }
            Scanner::Little2Prolog => {
                if input.chars.is_empty() {
                    return ScannerResult::new(crate::src::xmltok::XML_TOK_NONE_1, None);
                }
                if input.chars.len() & !1 == 0 {
                    return ScannerResult::new(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                }
                let (token, next) = little2_prolog_tok(encoding, input.chars);
                ScannerResult::new(token, next)
            }
            Scanner::Little2Content => little2_content(),
            Scanner::Little2CdataSection => {
                let result = little2_cdata_section_tok_impl(encoding, input.bytes);
                ScannerResult::new(result.token, result.next)
            }
            Scanner::Little2IgnoreSection => match little2_ignore_section_tok_impl(encoding, input.chars) {
                Little2IgnoreSectionOutcome::Token(token, next) => ScannerResult::new(token, Some(next)),
                Little2IgnoreSectionOutcome::Partial(token) => ScannerResult::new(token, None),
                Little2IgnoreSectionOutcome::Invalid(at) => ScannerResult::new(crate::src::xmltok::XML_TOK_INVALID_1, Some(at)),
            },
            Scanner::Big2Prolog => {
                if input.chars.is_empty() {
                    return ScannerResult::new(crate::src::xmltok::XML_TOK_NONE_1, None);
                }
                if input.chars.len() & !1 == 0 {
                    return ScannerResult::new(crate::src::xmltok::XML_TOK_PARTIAL_1, None);
                }
                let (token, next) = big2_prologTok(encoding, input.chars);
                ScannerResult::new(token, next)
            }
            Scanner::Big2Content => big2_content(),
            Scanner::Big2CdataSection => {
                let result = big2_cdata_section_tok_impl(encoding, input.bytes);
                ScannerResult::new(result.token, result.next)
            }
            Scanner::Big2IgnoreSection => big2_result(big2_ignore_section_tok_impl(encoding, input.chars)),
            Scanner::InitProlog | Scanner::InitContent | Scanner::InitPrologNS | Scanner::InitContentNS => {
                unreachable!("initial scanners update INIT_ENCODING through their boundary adapter")
            }
        }
    }

    pub(crate) fn big2_update_position(
        encoding: &normal_encoding,
        bytes: &[u8],
        pos: &mut crate::src::xmltok::POSITION,
    ) {
        let mut offset = 0;
        while bytes.len().saturating_sub(offset) >= 2 {
            match big2_position_byte_type(encoding, bytes, offset) {
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
                        && big2_position_byte_type(encoding, bytes, offset)
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

    use crate::src::xmltok::checkCharRefNumber;
    use crate::src::xmltok::nametab_h::namePages;
    use crate::src::xmltok::nametab_h::namingBitmap;
    use crate::src::xmltok::nametab_h::nmstrtPages;
    use crate::src::xmltok::normal_encoding;
    use crate::src::xmltok::unicode_byte_type;
    use crate::src::xmltok::unknown_character_value_for;
    use crate::src::xmltok::unknown_is_invalid;
    use crate::src::xmltok::unknown_is_name;
    use crate::src::xmltok::unknown_is_name_start;
    use crate::src::xmltok::utf8_invalid2;
    use crate::src::xmltok::utf8_invalid3;
    use crate::src::xmltok::utf8_invalid4;
    use crate::src::xmltok::utf8_is_name2;
    use crate::src::xmltok::utf8_is_name3;
    use crate::src::xmltok::utf8_is_name_start3;
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
    pub fn XmlGetUtf8InternalEncoding() -> &'static crate::src::xmltok::ENCODING {
        &internal_utf8_encoding.enc
    }
    #[export_name = "XmlGetUtf8InternalEncoding"]

    pub unsafe extern "C" fn XmlGetUtf8InternalEncoding_ffi() -> *const crate::src::xmltok::ENCODING
    {
        XmlGetUtf8InternalEncoding() as *const crate::src::xmltok::ENCODING
    }
    pub fn XmlGetUtf16InternalEncoding() -> &'static crate::src::xmltok::ENCODING {
        &internal_little2_encoding.enc
    }
    #[export_name = "XmlGetUtf16InternalEncoding"]

    pub unsafe extern "C" fn XmlGetUtf16InternalEncoding_ffi() -> *const crate::src::xmltok::ENCODING
    {
        XmlGetUtf16InternalEncoding() as *const crate::src::xmltok::ENCODING
    }
    pub const encodings: [*const crate::src::xmltok::ENCODING; 7] = [
        &raw const crate::src::xmltok::latin1_encoding.enc,
        &raw const crate::src::xmltok::ascii_encoding.enc,
        &raw const crate::src::xmltok::utf8_encoding.enc,
        &raw const crate::src::xmltok::big2_encoding.enc,
        &raw const crate::src::xmltok::big2_encoding.enc,
        &raw const crate::src::xmltok::little2_encoding.enc,
        &raw const crate::src::xmltok::utf8_encoding.enc,
    ];

    /// Initializes the parser's non-namespace encoding state from an
    /// optional, already-bounded protocol encoding name.
    pub(crate) fn init_encoding(
        initial: &mut crate::src::xmltok::INIT_ENCODING,
        name: Option<&[u8]>,
    ) -> bool {
        let i: ::core::ffi::c_int = if let Some(name) = name {
            encoding_index(name)
        } else {
            NO_ENC as ::core::ffi::c_int
        };
        if i == UNKNOWN_ENC as ::core::ffi::c_int {
            return false;
        }
        initial.initEnc.isUtf16 = i as ::core::ffi::c_char;
        initial.initEnc.scanners[crate::src::xmltok::XML_PROLOG_STATE as usize] =
            crate::src::xmltok::Scanner::InitProlog;
        initial.initEnc.scanners[crate::src::xmltok::XML_CONTENT_STATE as usize] =
            crate::src::xmltok::Scanner::InitContent;
        initial.initEnc.updatePosition = crate::src::xmltok::PositionUpdater::Init;
        initial.selected_encoding = None;
        true
    }
    #[export_name = "XmlInitEncoding"]

    pub unsafe extern "C" fn XmlInitEncoding_ffi(
        mut p: *mut crate::src::xmltok::INIT_ENCODING,
        mut encPtr: *mut *const crate::src::xmltok::ENCODING,
        mut name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if p.is_null() || encPtr.is_null() || !p.is_aligned() || !encPtr.is_aligned() {
            return 0;
        }
        let name = if name.is_null() {
            None
        } else {
            Some(unsafe { core::ffi::CStr::from_ptr(name) }.to_bytes())
        };
        let initial = unsafe { &mut *p };
        if init_encoding(initial, name) {
            unsafe { *encPtr = &raw const initial.initEnc };
            1
        } else {
            0
        }
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
        if enc.is_null()
            || !enc.is_aligned()
            || ptr.is_null()
            || end.addr() < ptr.addr()
            || (!badPtr.is_null() && !badPtr.is_aligned())
            || (!versionPtr.is_null() && !versionPtr.is_aligned())
            || (!versionEndPtr.is_null() && !versionEndPtr.is_aligned())
            || (!encodingName.is_null() && !encodingName.is_aligned())
            || (!encoding.is_null() && !encoding.is_aligned())
            || (!standalone.is_null() && !standalone.is_aligned())
        {
            return 0;
        }
        let input = unsafe {
            core::slice::from_raw_parts(ptr.cast::<u8>(), end.addr() - ptr.addr())
        };
        crate::src::xmltok::parse_xml_decl_ffi(
            isGeneralTextEntity != 0,
            unsafe { &*enc },
            input,
            |event| match event {
                crate::src::xmltok::XmlDeclFfiEvent::Bad(offset) => {
                    if let Some(output) = unsafe { badPtr.as_mut() } {
                        *output = ptr.wrapping_add(offset);
                    }
                }
                crate::src::xmltok::XmlDeclFfiEvent::Version(offset) => {
                    if let Some(output) = unsafe { versionPtr.as_mut() } {
                        *output = ptr.wrapping_add(offset);
                    }
                }
                crate::src::xmltok::XmlDeclFfiEvent::VersionEnd(offset) => {
                    if let Some(output) = unsafe { versionEndPtr.as_mut() } {
                        *output = ptr.wrapping_add(offset);
                    }
                }
                crate::src::xmltok::XmlDeclFfiEvent::EncodingName(offset) => {
                    if let Some(output) = unsafe { encodingName.as_mut() } {
                        *output = ptr.wrapping_add(offset);
                    }
                }
                crate::src::xmltok::XmlDeclFfiEvent::Encoding(selected) => {
                    if let Some(output) = unsafe { encoding.as_mut() } {
                        *output = match selected {
                            crate::src::xmltok::XmlDeclEncoding::Current => enc,
                            crate::src::xmltok::XmlDeclEncoding::Known(index) => encodings[index],
                            crate::src::xmltok::XmlDeclEncoding::Unknown => core::ptr::null(),
                        };
                    }
                }
                crate::src::xmltok::XmlDeclFfiEvent::Standalone(value) => {
                    if let Some(output) = unsafe { standalone.as_mut() } {
                        *output = value;
                    }
                }
            },
        )
    }
    pub fn XmlGetUtf8InternalEncodingNS() -> &'static crate::src::xmltok::ENCODING {
        &internal_utf8_encoding_ns.enc
    }
    #[export_name = "XmlGetUtf8InternalEncodingNS"]

    pub unsafe extern "C" fn XmlGetUtf8InternalEncodingNS_ffi(
    ) -> *const crate::src::xmltok::ENCODING {
        XmlGetUtf8InternalEncodingNS() as *const crate::src::xmltok::ENCODING
    }
    pub fn XmlGetUtf16InternalEncodingNS() -> &'static crate::src::xmltok::ENCODING
    {
        &internal_little2_encoding_ns.enc
    }
    #[export_name = "XmlGetUtf16InternalEncodingNS"]

    pub unsafe extern "C" fn XmlGetUtf16InternalEncodingNS_ffi(
    ) -> *const crate::src::xmltok::ENCODING {
        XmlGetUtf16InternalEncodingNS() as *const crate::src::xmltok::ENCODING
    }
    pub const encodingsNS: [*const crate::src::xmltok::ENCODING; 7] = [
        &raw const crate::src::xmltok::latin1_encoding_ns.enc,
        &raw const crate::src::xmltok::ascii_encoding_ns.enc,
        &raw const crate::src::xmltok::utf8_encoding_ns.enc,
        &raw const crate::src::xmltok::big2_encoding_ns.enc,
        &raw const crate::src::xmltok::big2_encoding_ns.enc,
        &raw const crate::src::xmltok::little2_encoding_ns.enc,
        &raw const crate::src::xmltok::utf8_encoding_ns.enc,
    ];

    /// Initializes the parser's namespace-aware encoding state from an
    /// optional, already-bounded protocol encoding name.
    pub fn XmlInitEncodingNS(
        initial: &mut crate::src::xmltok::INIT_ENCODING,
        name: Option<&[u8]>,
    ) -> bool {
        let i: ::core::ffi::c_int = if let Some(name) = name {
            encoding_index(name)
        } else {
            NO_ENC as ::core::ffi::c_int
        };
        if i == UNKNOWN_ENC as ::core::ffi::c_int {
            return false;
        }
        initial.initEnc.isUtf16 = i as ::core::ffi::c_char;
        initial.initEnc.scanners[crate::src::xmltok::XML_PROLOG_STATE as usize] =
            crate::src::xmltok::Scanner::InitPrologNS;
        initial.initEnc.scanners[crate::src::xmltok::XML_CONTENT_STATE as usize] =
            crate::src::xmltok::Scanner::InitContentNS;
        initial.initEnc.updatePosition = crate::src::xmltok::PositionUpdater::Init;
        initial.selected_encoding = None;
        true
    }
    #[export_name = "XmlInitEncodingNS"]

    pub unsafe extern "C" fn XmlInitEncodingNS_ffi(
        mut p: *mut crate::src::xmltok::INIT_ENCODING,
        mut encPtr: *mut *const crate::src::xmltok::ENCODING,
        mut name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if p.is_null() || encPtr.is_null() || !p.is_aligned() || !encPtr.is_aligned() {
            return 0;
        }
        let name = if name.is_null() {
            None
        } else {
            Some(unsafe { core::ffi::CStr::from_ptr(name) }.to_bytes())
        };
        let initial = unsafe { &mut *p };
        if XmlInitEncodingNS(initial, name) {
            unsafe { *encPtr = &raw const initial.initEnc };
            1
        } else {
            0
        }
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
        if enc.is_null()
            || !enc.is_aligned()
            || ptr.is_null()
            || end.addr() < ptr.addr()
            || (!badPtr.is_null() && !badPtr.is_aligned())
            || (!versionPtr.is_null() && !versionPtr.is_aligned())
            || (!versionEndPtr.is_null() && !versionEndPtr.is_aligned())
            || (!encodingName.is_null() && !encodingName.is_aligned())
            || (!encoding.is_null() && !encoding.is_aligned())
            || (!standalone.is_null() && !standalone.is_aligned())
        {
            return 0;
        }
        let input = unsafe {
            core::slice::from_raw_parts(ptr.cast::<u8>(), end.addr() - ptr.addr())
        };
        crate::src::xmltok::parse_xml_decl_ffi(
            isGeneralTextEntity != 0,
            unsafe { &*enc },
            input,
            |event| match event {
                crate::src::xmltok::XmlDeclFfiEvent::Bad(offset) => {
                    if let Some(output) = unsafe { badPtr.as_mut() } {
                        *output = ptr.wrapping_add(offset);
                    }
                }
                crate::src::xmltok::XmlDeclFfiEvent::Version(offset) => {
                    if let Some(output) = unsafe { versionPtr.as_mut() } {
                        *output = ptr.wrapping_add(offset);
                    }
                }
                crate::src::xmltok::XmlDeclFfiEvent::VersionEnd(offset) => {
                    if let Some(output) = unsafe { versionEndPtr.as_mut() } {
                        *output = ptr.wrapping_add(offset);
                    }
                }
                crate::src::xmltok::XmlDeclFfiEvent::EncodingName(offset) => {
                    if let Some(output) = unsafe { encodingName.as_mut() } {
                        *output = ptr.wrapping_add(offset);
                    }
                }
                crate::src::xmltok::XmlDeclFfiEvent::Encoding(selected) => {
                    if let Some(output) = unsafe { encoding.as_mut() } {
                        *output = match selected {
                            crate::src::xmltok::XmlDeclEncoding::Current => enc,
                            crate::src::xmltok::XmlDeclEncoding::Known(index) => encodingsNS[index],
                            crate::src::xmltok::XmlDeclEncoding::Unknown => core::ptr::null(),
                        };
                    }
                }
                crate::src::xmltok::XmlDeclFfiEvent::Standalone(value) => {
                    if let Some(output) = unsafe { standalone.as_mut() } {
                        *output = value;
                    }
                }
            },
        )
    }
    use crate::src::xmltok::encoding_index;
    use crate::src::xmltok::internal_little2_encoding;
    use crate::src::xmltok::internal_little2_encoding_ns;
    use crate::src::xmltok::internal_utf8_encoding;
    use crate::src::xmltok::internal_utf8_encoding_ns;
    use crate::src::xmltok::NO_ENC;
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
pub use crate::src::xmltok::xmltok_impl_c::big2_cdataSectionTok;
pub use crate::src::xmltok::xmltok_impl_c::big2_checkPiTarget;
pub use crate::src::xmltok::xmltok_impl_c::big2_entityValueTok;
pub use crate::src::xmltok::xmltok_impl_c::big2_ignoreSectionTok;
pub use crate::src::xmltok::xmltok_impl_c::big2_prologTok;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanPoundName;
pub use crate::src::xmltok::xmltok_impl_c::little2_attributeValueTok;
pub use crate::src::xmltok::xmltok_impl_c::little2_checkPiTarget;
pub use crate::src::xmltok::xmltok_impl_c::little2_contentTok;
pub use crate::src::xmltok::xmltok_impl_c::little2_entityValueTok;
pub use crate::src::xmltok::xmltok_impl_c::little2_ignoreSectionTok;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanAtts;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanCharRef;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanComment;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanDecl;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanHexCharRef;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanPercent;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanPoundName;
pub use crate::src::xmltok::xmltok_impl_c::normal_checkPiTarget;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanCdataSection;
pub use crate::src::xmltok::xmltok_impl_c::skip_s;
pub use crate::src::xmltok::xmltok_impl_c::Big2AttributeAction;
pub use crate::src::xmltok::xmltok_ns_c::encodings;
pub use crate::src::xmltok::xmltok_ns_c::encodingsNS;
pub use crate::src::xmltok::xmltok_ns_c::XmlGetUtf16InternalEncoding;
pub use crate::src::xmltok::xmltok_ns_c::XmlGetUtf16InternalEncodingNS;
pub use crate::src::xmltok::xmltok_ns_c::XmlGetUtf8InternalEncoding;
pub use crate::src::xmltok::xmltok_ns_c::XmlGetUtf8InternalEncodingNS;
pub use crate::src::xmltok::xmltok_ns_c::XmlInitEncodingNS;
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
    /// Registry identity for an owned unknown encoding.  Built-in tables have
    /// no converter, so their normal scanner path carries `None`.
    pub(crate) unknown_converter_id: Option<usize>,
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
    // This field preserves the hidden C structure's callback-context slot and
    // therefore its queried size and alignment.  The context itself is an
    // opaque token: tokenizer state never dereferences it, and callback
    // invocation keeps the original pointer only in the boundary adapter.
    user_data_token: usize,
    pub utf16: [::core::ffi::c_ushort; 256],
    pub utf8: [[::core::ffi::c_char; 4]; 256],
}

pub type C2Rust_Unnamed_8 = ::core::ffi::c_uint;

pub type C2Rust_Unnamed_9 = ::core::ffi::c_int;

pub const US_ASCII_ENC: C2Rust_Unnamed_9 = 1;

fn utf8_is_name2(input: &[u8]) -> bool {
    (namingBitmap[(((namePages[(input[0] as ::core::ffi::c_int >> 2 as ::core::ffi::c_int
        & 7 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int)
        << 3 as ::core::ffi::c_int)
        + ((input[0] as ::core::ffi::c_int & 3 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
        + (input[1] as ::core::ffi::c_int >> 5 as ::core::ffi::c_int & 1 as ::core::ffi::c_int))
        as usize]
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
        + ((first as ::core::ffi::c_int & 3 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
        + (second as ::core::ffi::c_int >> 5 as ::core::ffi::c_int & 1 as ::core::ffi::c_int))
        as usize]
        & (1 as ::core::ffi::c_uint) << (second as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
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
/// Returns the largest prefix that ends at a complete UTF-8 character.
///
/// This deliberately retains Expat's byte-oriented behavior for malformed
/// input: it only removes a trailing incomplete lead sequence.
fn trim_to_complete_utf8_characters(input: &[u8]) -> usize {
    let mut end = input.len();
    let mut walked = 0usize;

    while end > 0 {
        let previous = input[end - 1];
        let complete_width = match previous {
            byte if byte & 0xf8 == 0xf0 => Some(4),
            byte if byte & 0xf0 == 0xe0 => Some(3),
            byte if byte & 0xe0 == 0xc0 => Some(2),
            byte if byte & 0x80 == 0 => break,
            _ => None,
        };
        if let Some(width) = complete_width {
            if walked + 1 >= width {
                end += width - 1;
                break;
            }
            walked = 0;
        }
        end -= 1;
        walked += 1;
    }
    end
}

/// Returns the byte offset for the longest complete UTF-8 prefix of a
/// tokenizer-bounded input slice.
fn trim_to_complete_utf8_cursor(input: &[u8]) -> usize {
    trim_to_complete_utf8_characters(input)
}

#[export_name = "_INTERNAL_trim_to_complete_utf8_characters"]

pub unsafe extern "C" fn _INTERNAL_trim_to_complete_utf8_characters_ffi(
    mut from: *const ::core::ffi::c_char,
    mut fromLimRef: *mut *const ::core::ffi::c_char,
) {
    let Some(from_lim_ref) = (unsafe { fromLimRef.as_mut() }) else {
        return;
    };
    if from.is_null() {
        return;
    }
    let from_lim = *from_lim_ref;
    if from_lim.is_null() || from == from_lim {
        return;
    }
    // The FFI contract guarantees a single readable allocation.  Retain the
    // historical no-op behavior for a reversed cursor range.
    let Some(length) = from_lim.addr().checked_sub(from.addr()) else {
        return;
    };
    let input = unsafe { core::slice::from_raw_parts(from.cast::<u8>(), length) };
    let trimmed = trim_to_complete_utf8_cursor(input);
    *from_lim_ref = input[trimmed..].as_ptr().cast::<::core::ffi::c_char>();
}
/// Copies the largest UTF-8 prefix that fits in `output` without splitting a
/// complete character.  This intentionally mirrors Expat's byte-oriented
/// trailing-character trim: malformed bytes are copied unchanged, while an
/// incomplete trailing lead sequence remains for the next conversion call.
fn utf8_to_utf8_window(
    input: &[u8],
    output: &mut [u8],
) -> (crate::src::xmltok::XML_Convert_Result, usize) {
    let output_exhausted = input.len() > output.len();
    let limited = &input[..input.len().min(output.len())];
    let end = trim_to_complete_utf8_characters(limited);

    output[..end]
        .iter_mut()
        .zip(&limited[..end])
        .for_each(|(destination, &source)| *destination = source);

    let result = if output_exhausted {
        crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED
    } else if end < limited.len() {
        crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE
    } else {
        crate::src::xmltok::XML_CONVERT_COMPLETED
    };
    (result, end)
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

static utf8_encoding_ns: normal_encoding = normal_encoding {
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
    unknown_converter_id: None,
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

static utf8_encoding: normal_encoding = normal_encoding {
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
    unknown_converter_id: None,
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

pub(crate) fn internal_utf8_encoding_table(namespace_aware: bool) -> &'static ENCODING {
    &internal_utf8_normal_encoding(namespace_aware).enc
}

/// Returns the full internal UTF-8 table for tokenizer-only slice dispatch.
/// The public table accessor above intentionally exposes just the ABI prefix.
pub(crate) fn internal_utf8_normal_encoding(
    namespace_aware: bool,
) -> &'static normal_encoding {
    if namespace_aware {
        &internal_utf8_encoding_ns
    } else {
        &internal_utf8_encoding
    }
}

static internal_utf8_encoding_ns: normal_encoding = normal_encoding {
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
    unknown_converter_id: None,
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

static internal_utf8_encoding: normal_encoding = normal_encoding {
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
    unknown_converter_id: None,
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

fn latin1_to_utf8_window(
    input: &[u8],
    output: &mut [u8],
) -> (crate::src::xmltok::XML_Convert_Result, usize, usize) {
    let mut input_used = 0;
    let mut output_used = 0;

    while input_used < input.len() {
        let byte = input[input_used];
        let encoded_len = if byte & 0x80 == 0 { 1 } else { 2 };
        if output.len() - output_used < encoded_len {
            return (
                crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED,
                input_used,
                output_used,
            );
        }

        if encoded_len == 1 {
            output[output_used] = byte;
        } else {
            output[output_used] = (byte >> 6) | UTF8_cval2 as u8;
            output[output_used + 1] = (byte & 0x3f) | 0x80;
        }
        input_used += 1;
        output_used += encoded_len;
    }

    (
        crate::src::xmltok::XML_CONVERT_COMPLETED,
        input_used,
        output_used,
    )
}

static latin1_encoding_ns: normal_encoding = normal_encoding {
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
    unknown_converter_id: None,
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
    unknown_converter_id: None,
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

static ascii_encoding_ns: normal_encoding = normal_encoding {
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
    unknown_converter_id: None,
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

static ascii_encoding: normal_encoding = normal_encoding {
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
    unknown_converter_id: None,
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

fn little2_toUtf16(
    input: &[u8],
    output: &mut [::core::ffi::c_ushort],
) -> (crate::src::xmltok::XML_Convert_Result, usize, usize) {
    // A trailing byte is not a complete UTF-16 code unit and remains for the
    // next conversion call.  If output pressure would split a surrogate pair,
    // leave the leading surrogate untouched as well.
    let mut input_len = input.len() & !1;
    let mut result = crate::src::xmltok::XML_CONVERT_COMPLETED;
    if input_len > output.len().saturating_mul(2) && input[input_len - 1] & 0xf8 == 0xd8 {
        input_len -= 2;
        result = crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
    }

    let output_len = (input_len / 2).min(output.len());
    for (slot, bytes) in output[..output_len]
        .iter_mut()
        .zip(input[..output_len * 2].chunks_exact(2))
    {
        *slot = u16::from_le_bytes([bytes[0], bytes[1]]);
    }

    let input_used = output_len * 2;
    if output_len == output.len() && input_used < input_len {
        result = crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
    }
    (result, input_used, output_len)
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

fn big2_toUtf16(
    input: &[u8],
    output: &mut [::core::ffi::c_ushort],
) -> (crate::src::xmltok::XML_Convert_Result, usize, usize) {
    // A trailing byte is not a complete UTF-16 code unit and remains for the
    // next conversion call.  If output pressure would split a surrogate pair,
    // leave the leading surrogate untouched as well.
    let mut input_len = input.len() & !1;
    let mut result = crate::src::xmltok::XML_CONVERT_COMPLETED;
    if input_len > output.len().saturating_mul(2) && input[input_len - 2] & 0xf8 == 0xd8 {
        input_len -= 2;
        result = crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
    }

    let output_len = (input_len / 2).min(output.len());
    for (slot, bytes) in output[..output_len]
        .iter_mut()
        .zip(input[..output_len * 2].chunks_exact(2))
    {
        *slot = u16::from_be_bytes([bytes[0], bytes[1]]);
    }

    let input_used = output_len * 2;
    if output_len == output.len() && input_used < input_len {
        result = crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
    }
    (result, input_used, output_len)
}

static little2_encoding_ns: normal_encoding = normal_encoding {
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
    unknown_converter_id: None,
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

static little2_encoding: normal_encoding = normal_encoding {
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
    unknown_converter_id: None,
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

static internal_little2_encoding_ns: normal_encoding = normal_encoding {
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
    unknown_converter_id: None,
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

static internal_little2_encoding: normal_encoding = normal_encoding {
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
    unknown_converter_id: None,
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

static big2_encoding_ns: normal_encoding = normal_encoding {
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
    unknown_converter_id: None,
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

static big2_encoding: normal_encoding = normal_encoding {
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
    unknown_converter_id: None,
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

fn ascii_case_insensitive_eq(left: &[u8], right: &[u8]) -> bool {
    left.len() == right.len()
        && left.iter().zip(right).all(|(&left, &right)| {
            let left = left.to_ascii_uppercase();
            let right = right.to_ascii_uppercase();
            left == right
        })
}

pub(crate) fn initUpdatePosition(
    updater: crate::src::xmltok::PositionUpdater,
    encoding: &normal_encoding,
    bytes: &[u8],
    pos: &mut crate::src::xmltok::POSITION,
) {
    match updater {
        crate::src::xmltok::PositionUpdater::Init => {
            xmltok_impl_c::normal_update_position(&utf8_encoding, bytes, pos)
        }
        crate::src::xmltok::PositionUpdater::Normal => {
            xmltok_impl_c::normal_update_position(encoding, bytes, pos)
        }
        crate::src::xmltok::PositionUpdater::Little2 => {
            xmltok_impl_c::little2_update_position(encoding, bytes, pos)
        }
        crate::src::xmltok::PositionUpdater::Big2 => {
            xmltok_impl_c::big2_update_position(encoding, bytes, pos)
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

pub(crate) struct XmlDeclResult {
    pub(crate) version: Option<core::ops::Range<usize>>,
    pub(crate) version_end: Option<usize>,
    pub(crate) encoding_name: Option<core::ops::Range<usize>>,
    pub(crate) encoding_end: Option<usize>,
    pub(crate) standalone: Option<::core::ffi::c_int>,
}

/// A parsed XML declaration together with the encoding selection requested by
/// its `encoding` pseudo-attribute.  All offsets remain relative to the
/// caller-owned declaration slice; C boundary adapters alone materialize
/// cursors from them.
pub(crate) struct XmlDeclBoundaryResult {
    pub(crate) declaration: XmlDeclResult,
    pub(crate) selected_encoding: Option<XmlDeclEncoding>,
}

/// A declaration result expressed solely in byte offsets and safe values.
/// C wrappers translate these events to their optional output cursor slots.
#[derive(Copy, Clone)]
pub(crate) enum XmlDeclFfiEvent {
    Bad(usize),
    Version(usize),
    VersionEnd(usize),
    EncodingName(usize),
    Encoding(XmlDeclEncoding),
    Standalone(::core::ffi::c_int),
}

#[derive(Copy, Clone)]
pub(crate) struct XmlDeclEncodingInfo {
    pub(crate) name_matcher: NameMatcher,
    pub(crate) min_bytes_per_char: ::core::ffi::c_int,
}

impl encoding {
    pub(crate) fn xml_decl_info(&self) -> XmlDeclEncodingInfo {
        XmlDeclEncodingInfo {
            name_matcher: self.nameMatchesAscii,
            min_bytes_per_char: self.minBytesPerChar,
        }
    }
}

/// The encoding selected by a syntactically valid XML declaration.  The
/// parser consumes this index using its own namespace-specific encoding
/// table, avoiding a raw tokenizer callback at the declaration boundary.
#[derive(Copy, Clone)]
pub(crate) enum XmlDeclEncoding {
    Current,
    Known(usize),
    Unknown,
}

fn xml_decl_name_matches(encoding: XmlDeclEncodingInfo, input: &[u8], expected: &[u8]) -> bool {
    let Ok(width) = usize::try_from(encoding.min_bytes_per_char) else {
        return false;
    };
    if width == 0 || input.len() != expected.len().saturating_mul(width) {
        return false;
    }
    expected.iter().enumerate().all(|(index, &expected)| {
        let Some(bytes) = input.get(index * width..(index + 1) * width) else {
            return false;
        };
        let actual = match encoding.name_matcher {
            NameMatcher::Normal => bytes.first().copied(),
            NameMatcher::Little2 if bytes.len() == 2 && bytes[1] == 0 => bytes.first().copied(),
            NameMatcher::Big2 if bytes.len() == 2 && bytes[0] == 0 => bytes.get(1).copied(),
            NameMatcher::Little2 | NameMatcher::Big2 => None,
        };
        actual.is_some_and(|actual| actual.eq_ignore_ascii_case(&expected))
    })
}

/// Resolves an XML declaration's bounded encoding-name token using the same
/// case-insensitive names as `findEncoding`, without materialising a raw
/// NUL-terminated buffer.
pub(crate) fn xml_decl_encoding(encoding: XmlDeclEncodingInfo, input: &[u8]) -> XmlDeclEncoding {
    const NAMES: [&[u8]; 6] = [
        b"iso-8859-1",
        b"us-ascii",
        b"utf-8",
        b"utf-16",
        b"utf-16be",
        b"utf-16le",
    ];

    if xml_decl_name_matches(encoding, input, NAMES[3]) && encoding.min_bytes_per_char == 2 {
        return XmlDeclEncoding::Current;
    }
    NAMES
        .iter()
        .position(|name| xml_decl_name_matches(encoding, input, name))
        .map_or(XmlDeclEncoding::Unknown, XmlDeclEncoding::Known)
}

/// Materializes a validated XML-declaration pseudo-attribute value as XML
/// characters.  XML declarations restrict these values to ASCII, but the
/// token itself can be encoded as UTF-8 or either UTF-16 byte order.
///
/// Keeping this conversion bounded by the parser result means callers do not
/// need to rescan a raw token or retain interior pointers into it.
pub(crate) fn xml_decl_ascii_value(
    encoding: XmlDeclEncodingInfo,
    input: &[u8],
    value: core::ops::Range<usize>,
) -> Option<Vec<crate::expat_external_h::XML_Char>> {
    let width = usize::try_from(encoding.min_bytes_per_char).ok()?;
    if width == 0 || value.start > value.end || value.end > input.len() || value.len() % width != 0
    {
        return None;
    }

    let mut result = Vec::new();
    result.try_reserve_exact(value.len() / width).ok()?;
    for offset in (value.start..value.end).step_by(width) {
        let character = xml_decl_ascii_at(encoding, input, offset)?;
        if character > 0x7f {
            return None;
        }
        result.push(character as crate::expat_external_h::XML_Char);
    }
    Some(result)
}

fn xml_decl_ascii_at(encoding: XmlDeclEncodingInfo, input: &[u8], offset: usize) -> Option<u8> {
    let width = usize::try_from(encoding.min_bytes_per_char).ok()?;
    let bytes = input.get(offset..offset.checked_add(width)?)?;
    match encoding.name_matcher {
        NameMatcher::Normal => bytes.first().copied(),
        NameMatcher::Little2 if bytes.len() == 2 && bytes[1] == 0 => Some(bytes[0]),
        NameMatcher::Big2 if bytes.len() == 2 && bytes[0] == 0 => Some(bytes[1]),
        NameMatcher::Little2 | NameMatcher::Big2 => Some(u8::MAX),
    }
}

fn xml_decl_matches_ascii(
    encoding: XmlDeclEncodingInfo,
    input: &[u8],
    span: core::ops::Range<usize>,
    word: &[u8],
) -> bool {
    let width = match usize::try_from(encoding.min_bytes_per_char) {
        Ok(width) => width,
        Err(_) => return false,
    };
    if span.len() != word.len().saturating_mul(width) {
        return false;
    }
    word.iter().enumerate().all(|(index, &expected)| {
        xml_decl_ascii_at(encoding, input, span.start + index * width) == Some(expected)
    })
}

fn xml_decl_is_space(character: Option<u8>) -> bool {
    matches!(character, Some(b' ' | b'\r' | b'\n' | b'\t'))
}

fn parse_xml_decl_pseudo_attribute(
    encoding: XmlDeclEncodingInfo,
    input: &[u8],
    mut cursor: usize,
    end: usize,
) -> Result<Option<XmlDeclAttribute>, usize> {
    let width = usize::try_from(encoding.min_bytes_per_char).map_err(|_| cursor)?;
    if cursor == end {
        return Ok(None);
    }
    if !xml_decl_is_space(xml_decl_ascii_at(encoding, input, cursor)) {
        return Err(cursor);
    }
    loop {
        cursor = cursor.checked_add(width).ok_or(cursor)?;
        if !xml_decl_is_space(xml_decl_ascii_at(encoding, input, cursor)) {
            break;
        }
    }
    if cursor == end {
        return Ok(None);
    }

    let name_start = cursor;
    let name_end;
    loop {
        match xml_decl_ascii_at(encoding, input, cursor) {
            None => return Err(cursor),
            Some(b'=') => {
                name_end = cursor;
                break;
            }
            character if xml_decl_is_space(character) => {
                name_end = cursor;
                loop {
                    cursor = cursor.checked_add(width).ok_or(cursor)?;
                    let character = xml_decl_ascii_at(encoding, input, cursor);
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
    while xml_decl_is_space(xml_decl_ascii_at(encoding, input, cursor)) {
        cursor = cursor.checked_add(width).ok_or(cursor)?;
    }
    let quote = match xml_decl_ascii_at(encoding, input, cursor) {
        Some(quote @ (b'\'' | b'"')) => quote,
        _ => return Err(cursor),
    };
    cursor = cursor.checked_add(width).ok_or(cursor)?;
    let value_start = cursor;
    loop {
        let character = xml_decl_ascii_at(encoding, input, cursor).ok_or(cursor)?;
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

/// Parses a bounded XML declaration token without retaining pointers into its
/// input.  Callers translate the returned byte offsets only at their own
/// boundary, so malformed-declaration cursors remain tied to the token that
/// was actually scanned.
pub(crate) fn parse_xml_decl(
    is_general_text_entity: bool,
    enc: &encoding,
    input: &[u8],
) -> Result<XmlDeclResult, usize> {
    parse_xml_decl_with_info(is_general_text_entity, enc.xml_decl_info(), input)
}

/// Parses an XML declaration and resolves its optional encoding name without
/// retaining a raw scanner callback or any pointer into the input token.
pub(crate) fn parse_xml_decl_boundary(
    is_general_text_entity: bool,
    enc: &encoding,
    input: &[u8],
) -> Result<XmlDeclBoundaryResult, usize> {
    let declaration = parse_xml_decl(is_general_text_entity, enc, input)?;
    let selected_encoding = declaration.encoding_name.as_ref().map(|range| {
        xml_decl_encoding(enc.xml_decl_info(), &input[range.clone()])
    });
    Ok(XmlDeclBoundaryResult {
        declaration,
        selected_encoding,
    })
}

/// Applies a parsed XML declaration to already-validated C output slots.
/// This keeps all declaration-result handling slice- and offset-based; the
/// exported wrappers only perform their boundary conversions.
pub(crate) fn parse_xml_decl_ffi(
    is_general_text_entity: bool,
    current_encoding: &encoding,
    input: &[u8],
    mut emit: impl FnMut(XmlDeclFfiEvent),
) -> ::core::ffi::c_int {
    let result = match parse_xml_decl_boundary(is_general_text_entity, current_encoding, input) {
        Ok(result) => result,
        Err(offset) => {
            emit(XmlDeclFfiEvent::Bad(offset.min(input.len())));
            return 0;
        }
    };

    let declaration = result.declaration;
    if let Some(range) = declaration.version {
        emit(XmlDeclFfiEvent::Version(range.start));
    }
    if let Some(offset) = declaration.version_end {
        emit(XmlDeclFfiEvent::VersionEnd(offset));
    }
    if let Some(range) = declaration.encoding_name {
        emit(XmlDeclFfiEvent::EncodingName(range.start));
    }
    if let Some(selected) = result.selected_encoding {
        emit(XmlDeclFfiEvent::Encoding(selected));
    }
    if let Some(value) = declaration.standalone {
        emit(XmlDeclFfiEvent::Standalone(value));
    }
    1
}

pub(crate) fn parse_xml_decl_with_info(
    is_general_text_entity: bool,
    encoding: XmlDeclEncodingInfo,
    input: &[u8],
) -> Result<XmlDeclResult, usize> {
    let width = usize::try_from(encoding.min_bytes_per_char).map_err(|_| 0usize)?;
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

    let mut attribute = parse_xml_decl_pseudo_attribute(encoding, input, cursor, end)?;
    let Some(ref first) = attribute else {
        return Err(cursor);
    };
    if xml_decl_matches_ascii(encoding, input, first.name.clone(), XML_DECL_VERSION) {
        result.version = Some(first.value.clone());
        result.version_end = Some(first.next);
        cursor = first.next;
        attribute = parse_xml_decl_pseudo_attribute(encoding, input, cursor, end)?;
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
    if xml_decl_matches_ascii(encoding, input, current.name.clone(), XML_DECL_ENCODING) {
        let value_start = current.value.start;
        if !matches!(
            xml_decl_ascii_at(encoding, input, value_start),
            Some(b'a'..=b'z' | b'A'..=b'Z')
        ) {
            return Err(value_start);
        }
        result.encoding_name = Some(current.value.clone());
        result.encoding_end = current.next.checked_sub(width);
        cursor = current.next;
        attribute = parse_xml_decl_pseudo_attribute(encoding, input, cursor, end)?;
        if attribute.is_none() {
            return Ok(result);
        }
    }

    let attribute = attribute.expect("XML declaration attribute is present");
    if is_general_text_entity
        || !xml_decl_matches_ascii(encoding, input, attribute.name.clone(), XML_DECL_STANDALONE)
    {
        return Err(attribute.name.start);
    }
    if xml_decl_matches_ascii(encoding, input, attribute.value.clone(), XML_DECL_YES) {
        result.standalone = Some(1);
    } else if xml_decl_matches_ascii(encoding, input, attribute.value.clone(), XML_DECL_NO) {
        result.standalone = Some(0);
    } else {
        return Err(attribute.value.start);
    }
    cursor = attribute.next;
    while xml_decl_is_space(xml_decl_ascii_at(encoding, input, cursor)) {
        cursor = cursor.checked_add(width).ok_or(cursor)?;
    }
    if cursor == end {
        Ok(result)
    } else {
        Err(cursor)
    }
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
pub fn XmlUtf8Encode(
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
    return 0 as ::core::ffi::c_int;
}
#[export_name = "XmlUtf8Encode"]

pub unsafe extern "C" fn XmlUtf8Encode_ffi(
    mut c: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if buf.is_null() {
        return 0;
    }
    let buf = unsafe { &mut *(buf as *mut [::core::ffi::c_char; 4]) };
    XmlUtf8Encode(c, buf)
}
pub fn XmlUtf16Encode(
    mut charNum: ::core::ffi::c_int,
    buf: &mut [::core::ffi::c_ushort; 2],
) -> ::core::ffi::c_int {
    if charNum < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if charNum < 0x10000 as ::core::ffi::c_int {
        buf[0] = charNum as ::core::ffi::c_ushort;
        return 1 as ::core::ffi::c_int;
    }
    if charNum < 0x110000 as ::core::ffi::c_int {
        charNum -= 0x10000 as ::core::ffi::c_int;
        buf[0] = ((charNum >> 10 as ::core::ffi::c_int) + 0xd800 as ::core::ffi::c_int)
            as ::core::ffi::c_ushort;
        buf[1] = ((charNum & 0x3ff as ::core::ffi::c_int) + 0xdc00 as ::core::ffi::c_int)
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
    if buf.is_null() {
        return 0;
    }
    let buf = unsafe { &mut *(buf as *mut [::core::ffi::c_ushort; 2]) };
    XmlUtf16Encode(charNum, buf)
}
pub extern "C" fn XmlSizeOfUnknownEncoding() -> ::core::ffi::c_int {
    return ::core::mem::size_of::<unknown_encoding>() as ::core::ffi::c_int;
}
#[export_name = "XmlSizeOfUnknownEncoding"]

pub unsafe extern "C" fn XmlSizeOfUnknownEncoding_ffi() -> ::core::ffi::c_int {
    XmlSizeOfUnknownEncoding()
}
/// Invokes the foreign unknown-encoding converter for one complete character.
///
/// Callers supply the initialized unknown-encoding storage address and the
/// remaining bounded input beginning at the character selected by that
/// table's byte classification.  The registry is keyed by this stable storage
/// address, which is also the address returned by `XmlInitUnknownEncoding`.
fn unknown_character_value(storage_id: usize, input: &[u8]) -> ::core::ffi::c_int {
    unknown_encoding_converter(storage_id)
        .expect("unknown encoding converter is registered")
        .invoke
        .invoke(input)
}

/// Resolves the converter retained by typed tokenizer state.  Reaching this
/// path without a registration is an internal state violation: only unknown
/// encodings install the corresponding character checks.
fn unknown_character_value_for(
    converter_id: Option<usize>,
    input: &[u8],
) -> ::core::ffi::c_int {
    unknown_character_value(
        converter_id.expect("unknown character checks require a converter registration"),
        input,
    )
}

fn unknown_is_name(c: ::core::ffi::c_int) -> bool {
    if c & !(0xffff as ::core::ffi::c_int) != 0 {
        return false;
    }
    (namingBitmap[(((namePages[(c >> 8 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int)
        << 3 as ::core::ffi::c_int)
        + ((c & 0xff as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int))
        as usize]
        & (1 as ::core::ffi::c_uint)
            << (c & 0xff as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
        != 0
}

fn unknown_is_name_start(c: ::core::ffi::c_int) -> bool {
    if c & !(0xffff as ::core::ffi::c_int) != 0 {
        return false;
    }
    (namingBitmap[(((nmstrtPages[(c >> 8 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int)
        << 3 as ::core::ffi::c_int)
        + ((c & 0xff as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int))
        as usize]
        & (1 as ::core::ffi::c_uint)
            << (c & 0xff as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
        != 0
}

fn unknown_is_invalid(c: ::core::ffi::c_int) -> bool {
    c & !(0xffff as ::core::ffi::c_int) != 0 || checkCharRefNumber(c) < 0 as ::core::ffi::c_int
}

fn unknown_to_utf8_window(
    encoding: &unknown_encoding,
    input: &[u8],
    output: &mut [u8],
    mut convert: impl FnMut(&[u8]) -> ::core::ffi::c_int,
) -> (crate::src::xmltok::XML_Convert_Result, usize, usize) {
    let mut input_used = 0;
    let mut output_used = 0;
    let mut converted = [0; 4];

    while input_used < input.len() {
        let byte = input[input_used] as ::core::ffi::c_uchar as usize;
        let table_entry = &encoding.utf8[byte];
        let (bytes, byte_count, input_advance) = if table_entry[0] == 0 {
            let character = convert(&input[input_used..]);
            let byte_count = encode_unknown_utf8(character, &mut converted);
            let input_advance = encoding.normal.type_0[byte] as usize
                - (crate::xmltok_impl_h::BT_LEAD2 as usize - 2);
            (&converted[..byte_count], byte_count, input_advance)
        } else {
            let byte_count = table_entry[0] as ::core::ffi::c_uchar as usize;
            (&table_entry[1..][..byte_count], byte_count, 1)
        };

        if output.len() - output_used < byte_count {
            return (
                crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED,
                input_used,
                output_used,
            );
        }
        output[output_used..output_used + byte_count]
            .iter_mut()
            .zip(bytes)
            .for_each(|(destination, &source)| *destination = source as u8);
        input_used += input_advance;
        output_used += byte_count;
    }

    (
        crate::src::xmltok::XML_CONVERT_COMPLETED,
        input_used,
        output_used,
    )
}

fn unknown_to_utf16_window(
    encoding: &unknown_encoding,
    input: &[u8],
    output: &mut [::core::ffi::c_ushort],
    mut convert: impl FnMut(&[u8]) -> ::core::ffi::c_int,
) -> (crate::src::xmltok::XML_Convert_Result, usize, usize) {
    let mut input_used = 0;
    let mut output_used = 0;

    while input_used < input.len() && output_used < output.len() {
        let byte = input[input_used] as ::core::ffi::c_uchar as usize;
        let (character, input_advance) = match encoding.utf16[byte] {
            0 => {
                let advance = encoding.normal.type_0[byte] as usize
                    - (crate::xmltok_impl_h::BT_LEAD2 as usize - 2);
                (
                    convert(&input[input_used..]) as ::core::ffi::c_ushort,
                    advance,
                )
            }
            character => (character, 1),
        };
        output[output_used] = character;
        input_used += input_advance.min(input.len() - input_used);
        output_used += 1;
    }

    let result = if output_used == output.len() && input_used < input.len() {
        crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED
    } else {
        crate::src::xmltok::XML_CONVERT_COMPLETED
    };
    (result, input_used, output_used)
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

/// Initializes owned unknown-encoding state from an already-validated table.
///
/// The foreign callback is represented by a boundary registration, so the
/// tokenizer itself only stores a stable registry key and never holds or
/// dereferences the callback context.
pub(crate) fn initialize_unknown_encoding_state(
    table: &[::core::ffi::c_int; 256],
    converter: Option<UnknownEncodingConverterRegistration>,
    storage_id: usize,
    user_data_token: usize,
    namespace_aware: bool,
) -> Option<unknown_encoding> {
    register_unknown_encoding_converter(storage_id, None, None);

    let mut encoding = unknown_encoding {
        normal: latin1_encoding,
        converter_id: storage_id,
        user_data_token,
        utf16: [0; 256],
        utf8: [[0; 4]; 256],
    };
    let has_converter = converter.is_some();
    if !initialize_unknown_encoding(&mut encoding, table, &latin1_encoding, has_converter) {
        return None;
    }

    if namespace_aware {
        encoding.normal.type_0[crate::ascii_h::ASCII_COLON as usize] =
            crate::xmltok_impl_h::BT_COLON_0 as ::core::ffi::c_uchar;
    }
    if has_converter {
        install_unknown_name_checks(&mut encoding);
    }
    encoding.normal.unknown_converter_id = Some(storage_id);
    encoding.normal.enc.utf8Convert = Utf8Converter::Unknown;
    encoding.normal.enc.utf16Convert = Utf16Converter::Unknown;
    register_unknown_encoding_converter(storage_id, converter, Some(encoding));
    Some(encoding)
}

#[export_name = "XmlInitUnknownEncoding"]
pub unsafe extern "C" fn XmlInitUnknownEncoding_ffi(
    mem: *mut ::core::ffi::c_void,
    table: *const ::core::ffi::c_int,
    convert: crate::src::xmltok::CONVERTER,
    user_data: *mut ::core::ffi::c_void,
) -> *mut crate::src::xmltok::ENCODING {
    if mem.is_null() || table.is_null() {
        return ::core::ptr::null_mut();
    }
    let storage = mem.cast::<unknown_encoding>();
    if !storage.is_aligned() || !table.is_aligned() {
        return ::core::ptr::null_mut();
    }
    let table = &*table.cast::<[::core::ffi::c_int; 256]>();
    let storage_id = mem.addr();
    // The callback context is an opaque C token.  AtomicPtr preserves it for
    // the callback adapter without Rust ever dereferencing it.
    let callback_arg = std::sync::Arc::new(std::sync::atomic::AtomicPtr::new(user_data));
    let converter = unknown_encoding_callback(convert.map(|callback| {
        let callback_arg = callback_arg.clone();
        move |input: &[u8]| {
            callback(
                callback_arg.load(std::sync::atomic::Ordering::Relaxed),
                input.as_ptr().cast::<::core::ffi::c_char>(),
            )
        }
    }));
    let Some(encoding) = initialize_unknown_encoding_state(
        table,
        converter,
        storage_id,
        user_data.addr(),
        false,
    ) else {
        return ::core::ptr::null_mut();
    };
    // `normal.enc` is the first repr(C) member of this storage, matching the
    // C API's returned `ENCODING *` without another raw dereference.
    let output = storage.cast::<crate::src::xmltok::ENCODING>();
    storage.write(encoding);
    output
}
fn encoding_index(name: &[u8]) -> ::core::ffi::c_int {
    const ENCODING_NAMES: [&[u8]; 6] = [
        b"iso-8859-1",
        b"us-ascii",
        b"utf-8",
        b"utf-16",
        b"utf-16be",
        b"utf-16le",
    ];

    ENCODING_NAMES
        .iter()
        .position(|candidate| ascii_case_insensitive_eq(name, candidate))
        .map_or(UNKNOWN_ENC as ::core::ffi::c_int, |index| {
            index as ::core::ffi::c_int
        })
}

/// Looks up the ASCII encoding names in a tokenizer-owned C-character buffer.
/// The buffer is already bounded and NUL-terminated by the converter, so no
/// C-string reconstruction or allocation is required here.
fn encoding_index_chars(name: &[::core::ffi::c_char]) -> ::core::ffi::c_int {
    const ENCODING_NAMES: [&[u8]; 6] = [
        b"iso-8859-1",
        b"us-ascii",
        b"utf-8",
        b"utf-16",
        b"utf-16be",
        b"utf-16le",
    ];

    ENCODING_NAMES
        .iter()
        .position(|candidate| {
            name.len() == candidate.len()
                && name.iter().zip(candidate.iter()).all(|(&left, &right)| {
                    (left as u8).to_ascii_uppercase() == right.to_ascii_uppercase()
                })
        })
        .map_or(UNKNOWN_ENC as ::core::ffi::c_int, |index| {
            index as ::core::ffi::c_int
        })
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

pub(crate) fn initial_known_encoding(
    index: usize,
    namespace_aware: bool,
) -> Option<&'static normal_encoding> {
    let encoding = match (namespace_aware, index) {
        (false, 0) => &latin1_encoding,
        (false, 1) => &ascii_encoding,
        (false, 2) | (false, 6) => &utf8_encoding,
        (false, 3) | (false, 4) => &big2_encoding,
        (false, 5) => &little2_encoding,
        (true, 0) => &latin1_encoding_ns,
        (true, 1) => &ascii_encoding_ns,
        (true, 2) | (true, 6) => &utf8_encoding_ns,
        (true, 3) | (true, 4) => &big2_encoding_ns,
        (true, 5) => &little2_encoding_ns,
        _ => return None,
    };
    Some(encoding)
}

/// Runs the initial-encoding state transition on an already bounded scanner
/// input.  The selected encoding is a fixed static table entry, so scanner
/// dispatch can stay entirely slice- and offset-based.
fn initial_scan_result(
    initial: &mut INIT_ENCODING,
    namespace_aware: bool,
    state: InitScanState,
    input: ScannerInput<'_>,
) -> ScannerResult {
    match init_scan_action(initial.initEnc.isUtf16, state, input.bytes) {
        InitScanAction::None => ScannerResult::new(crate::src::xmltok::XML_TOK_NONE_1, None),
        InitScanAction::Partial => ScannerResult::new(crate::src::xmltok::XML_TOK_PARTIAL_1, None),
        InitScanAction::Bom {
            encoding_index,
            consumed,
        } => {
            initial.selected_encoding = Some(encoding_index);
            ScannerResult::new(crate::src::xmltok::XML_TOK_BOM_1, Some(consumed))
        }
        InitScanAction::Scan { encoding_index } => {
            let Some(encoding) = initial_known_encoding(encoding_index, namespace_aware) else {
                return ScannerResult::new(crate::src::xmltok::XML_TOK_INVALID_1, Some(0));
            };
            initial.selected_encoding = Some(encoding_index);
            encoding.enc.scanners[state.scanner_index()].scan_result(encoding, input, None)
        }
    }
}

#[export_name = "XmlInitUnknownEncodingNS"]
pub unsafe extern "C" fn XmlInitUnknownEncodingNS_ffi(
    mem: *mut ::core::ffi::c_void,
    table: *const ::core::ffi::c_int,
    convert: crate::src::xmltok::CONVERTER,
    user_data: *mut ::core::ffi::c_void,
) -> *mut crate::src::xmltok::ENCODING {
    if mem.is_null() || table.is_null() {
        return ::core::ptr::null_mut();
    }
    let storage = mem.cast::<unknown_encoding>();
    if !storage.is_aligned() || !table.is_aligned() {
        return ::core::ptr::null_mut();
    }
    let table = &*table.cast::<[::core::ffi::c_int; 256]>();
    let storage_id = mem.addr();
    let callback_arg = std::sync::Arc::new(std::sync::atomic::AtomicPtr::new(user_data));
    let converter = unknown_encoding_callback(convert.map(|callback| {
        let callback_arg = callback_arg.clone();
        move |input: &[u8]| {
            callback(
                callback_arg.load(std::sync::atomic::Ordering::Relaxed),
                input.as_ptr().cast::<::core::ffi::c_char>(),
            )
        }
    }));
    let Some(encoding) = initialize_unknown_encoding_state(
        table,
        converter,
        storage_id,
        user_data.addr(),
        true,
    ) else {
        return ::core::ptr::null_mut();
    };
    let output = storage.cast::<crate::src::xmltok::ENCODING>();
    storage.write(encoding);
    output
}
