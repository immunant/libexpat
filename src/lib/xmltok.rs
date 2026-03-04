// =============== BEGIN xmltok_h ================
pub const XML_TOK_TRAILING_RSQB: c_int = -5;

pub const XML_TOK_TRAILING_RSQB_1: c_int = -(5);

pub const XML_TOK_NONE: c_int = -4;

pub const XML_TOK_NONE_1: c_int = -(4);

pub const XML_TOK_TRAILING_CR: c_int = -3;

pub const XML_TOK_TRAILING_CR_1: c_int = -(3);

pub const XML_TOK_PARTIAL_CHAR: c_int = -2;

pub const XML_TOK_PARTIAL_CHAR_1: c_int = -(2);

pub const XML_TOK_PARTIAL: c_int = -1;

pub const XML_TOK_PARTIAL_1: c_int = -(1);

pub const XML_TOK_INVALID: c_int = 0;

pub const XML_TOK_INVALID_1: c_int = 0;

pub const XML_TOK_START_TAG_WITH_ATTS: c_int = 1;

pub const XML_TOK_START_TAG_WITH_ATTS_1: c_int = 1;

pub const XML_TOK_START_TAG_NO_ATTS: c_int = 2;

pub const XML_TOK_START_TAG_NO_ATTS_1: c_int = 2;

pub const XML_TOK_EMPTY_ELEMENT_WITH_ATTS: c_int = 3;

pub const XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1: c_int = 3;

pub const XML_TOK_EMPTY_ELEMENT_NO_ATTS: c_int = 4;

pub const XML_TOK_EMPTY_ELEMENT_NO_ATTS_1: c_int = 4;

pub const XML_TOK_END_TAG: c_int = 5;

pub const XML_TOK_END_TAG_1: c_int = 5;

pub const XML_TOK_DATA_CHARS: c_int = 6;

pub const XML_TOK_DATA_CHARS_1: c_int = 6;

pub const XML_TOK_DATA_NEWLINE: c_int = 7;

pub const XML_TOK_DATA_NEWLINE_1: c_int = 7;

pub const XML_TOK_CDATA_SECT_OPEN: c_int = 8;

pub const XML_TOK_CDATA_SECT_OPEN_1: c_int = 8;

pub const XML_TOK_ENTITY_REF: c_int = 9;

pub const XML_TOK_ENTITY_REF_1: c_int = 9;

pub const XML_TOK_CHAR_REF: c_int = 10;

pub const XML_TOK_CHAR_REF_1: c_int = 10;

pub const XML_TOK_PI: c_int = 11;

pub const XML_TOK_PI_1: c_int = 11;

pub const XML_TOK_XML_DECL: c_int = 12;

pub const XML_TOK_XML_DECL_1: c_int = 12;

pub const XML_TOK_COMMENT: c_int = 13;

pub const XML_TOK_COMMENT_1: c_int = 13;

pub const XML_TOK_BOM: c_int = 14;

pub const XML_TOK_BOM_1: c_int = 14;

pub const XML_TOK_PROLOG_S: c_int = 15;

pub const XML_TOK_PROLOG_S_1: c_int = 15;

pub const XML_TOK_DECL_OPEN: c_int = 16;

pub const XML_TOK_DECL_OPEN_1: c_int = 16;

pub const XML_TOK_DECL_CLOSE: c_int = 17;

pub const XML_TOK_DECL_CLOSE_1: c_int = 17;

pub const XML_TOK_NAME: c_int = 18;

pub const XML_TOK_NMTOKEN: c_int = 19;

pub const XML_TOK_NMTOKEN_1: c_int = 19;

pub const XML_TOK_POUND_NAME: c_int = 20;

pub const XML_TOK_POUND_NAME_1: c_int = 20;

pub const XML_TOK_OR: c_int = 21;

pub const XML_TOK_OR_1: c_int = 21;

pub const XML_TOK_PERCENT: c_int = 22;

pub const XML_TOK_PERCENT_1: c_int = 22;

pub const XML_TOK_OPEN_PAREN: c_int = 23;

pub const XML_TOK_OPEN_PAREN_1: c_int = 23;

pub const XML_TOK_CLOSE_PAREN: c_int = 24;

pub const XML_TOK_CLOSE_PAREN_1: c_int = 24;

pub const XML_TOK_OPEN_BRACKET: c_int = 25;

pub const XML_TOK_OPEN_BRACKET_1: c_int = 25;

pub const XML_TOK_CLOSE_BRACKET: c_int = 26;

pub const XML_TOK_CLOSE_BRACKET_1: c_int = 26;

pub const XML_TOK_LITERAL: c_int = 27;

pub const XML_TOK_LITERAL_1: c_int = 27;

pub const XML_TOK_PARAM_ENTITY_REF: c_int = 28;

pub const XML_TOK_PARAM_ENTITY_REF_1: c_int = 28;

pub const XML_TOK_INSTANCE_START: c_int = 29;

pub const XML_TOK_INSTANCE_START_1: c_int = 29;

pub const XML_TOK_NAME_QUESTION: c_int = 30;

pub const XML_TOK_NAME_QUESTION_1: c_int = 30;

pub const XML_TOK_NAME_ASTERISK: c_int = 31;

pub const XML_TOK_NAME_ASTERISK_1: c_int = 31;

pub const XML_TOK_NAME_PLUS: c_int = 32;

pub const XML_TOK_NAME_PLUS_1: c_int = 32;

pub const XML_TOK_COND_SECT_OPEN: c_int = 33;

pub const XML_TOK_COND_SECT_OPEN_1: c_int = 33;

pub const XML_TOK_COND_SECT_CLOSE: c_int = 34;

pub const XML_TOK_COND_SECT_CLOSE_1: c_int = 34;

pub const XML_TOK_CLOSE_PAREN_QUESTION: c_int = 35;

pub const XML_TOK_CLOSE_PAREN_QUESTION_1: c_int = 35;

pub const XML_TOK_CLOSE_PAREN_ASTERISK: c_int = 36;

pub const XML_TOK_CLOSE_PAREN_ASTERISK_1: c_int = 36;

pub const XML_TOK_CLOSE_PAREN_PLUS: c_int = 37;

pub const XML_TOK_CLOSE_PAREN_PLUS_1: c_int = 37;

pub const XML_TOK_COMMA: c_int = 38;

pub const XML_TOK_COMMA_1: c_int = 38;

pub const XML_TOK_ATTRIBUTE_VALUE_S: c_int = 39;

pub const XML_TOK_ATTRIBUTE_VALUE_S_1: c_int = 39;

pub const XML_TOK_CDATA_SECT_CLOSE: c_int = 40;

pub const XML_TOK_CDATA_SECT_CLOSE_1: c_int = 40;

pub const XML_TOK_PREFIXED_NAME: c_int = 41;

pub const XML_TOK_IGNORE_SECT: c_int = 42;

pub const XML_TOK_IGNORE_SECT_1: c_int = 42;

pub const XML_PROLOG_STATE: c_int = 0;

pub const XML_CONTENT_STATE: c_int = 1;

pub type POSITION = crate::src::lib::xmltok::position;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct position {
    pub lineNumber: XML_Size,
    pub columnNumber: XML_Size,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ATTRIBUTE {
    pub name: *const c_char,
    pub valuePtr: *const c_char,
    pub valueEnd: *const c_char,
    pub normalized: c_char,
}

pub type ENCODING = crate::src::lib::xmltok::encoding;

pub type ScannerResult = (c_int, *const c_char);
pub type SCANNER = fn(&ENCODING, &[c_char]) -> ScannerResult;

pub type XML_Convert_Result = c_uint;

pub const XML_CONVERT_COMPLETED: XML_Convert_Result = 0;

pub const XML_CONVERT_INPUT_INCOMPLETE: XML_Convert_Result = 1;

pub const XML_CONVERT_OUTPUT_EXHAUSTED: XML_Convert_Result = 2;

pub(crate) type XmlInitEncodingResult = (c_int, *const ENCODING);
pub(crate) type ParsePseudoAttributeResult = (
    c_int,
    *const c_char,
    *const c_char,
    *const c_char,
    *const c_char,
);
pub(crate) type ParseXmlDeclResult = (
    c_int,
    *const c_char,
    *const c_char,
    *const c_char,
    *const c_char,
    *const ENCODING,
    c_int,
);
pub(crate) type IsPublicIdResult = (c_int, *const c_char);
pub(crate) type Utf8ConvertResult = (XML_Convert_Result, *const c_char, *mut c_char);
pub(crate) type Utf16ConvertResult = (XML_Convert_Result, *const c_char, *mut c_ushort);
pub(crate) type CheckPiTargetResult = (c_int, c_int);

trait EncodingFunctions {
    fn nameMatchesAscii(&self, enc: &ENCODING, input: &[c_char], kw: *const c_char) -> c_int;
    fn nameLength(&self, enc: &ENCODING, ptr: *const c_char) -> c_int;
    fn skipS(&self, enc: &ENCODING, ptr: *const c_char) -> *const c_char;
    fn getAtts(
        &self,
        enc: &ENCODING,
        ptr: *const c_char,
        n: c_int,
        atts: *mut crate::src::lib::xmltok::ATTRIBUTE,
    ) -> c_int;
    fn charRefNumber(&self, enc: &ENCODING, ptr: *const c_char) -> c_int;
    fn predefinedEntityName(&self, enc: &ENCODING, input: &[c_char]) -> c_int;
    fn updatePosition(
        &self,
        enc: &ENCODING,
        input: &[c_char],
        pos: *mut crate::src::lib::xmltok::POSITION,
    );
    fn isPublicId(&self, enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult;
    fn utf8Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_char,
        to_lim: *const c_char,
    ) -> Utf8ConvertResult;
    fn utf16Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_ushort,
        to_lim: *const c_ushort,
    ) -> Utf16ConvertResult;
}

#[derive(Copy, Clone)]
#[repr(C)]

pub struct encoding {
    pub scanners: [crate::src::lib::xmltok::SCANNER; 4],
    pub literalScanners: [crate::src::lib::xmltok::SCANNER; 2],
    functions: &'static (dyn EncodingFunctions + Sync),
    pub minBytesPerChar: c_int,
    pub isUtf8: c_char,
    pub isUtf16: c_char,
}

impl encoding {
    #[inline]
    fn functions(&self) -> &(dyn EncodingFunctions + Sync) {
        self.functions
    }

    pub(crate) fn nameMatchesAscii(
        &self,
        enc: &ENCODING,
        input: &[c_char],
        kw: *const c_char,
    ) -> c_int {
        self.functions().nameMatchesAscii(enc, input, kw)
    }

    pub(crate) fn nameLength(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        self.functions().nameLength(enc, ptr)
    }

    pub(crate) fn skipS(&self, enc: &ENCODING, ptr: *const c_char) -> *const c_char {
        self.functions().skipS(enc, ptr)
    }

    pub(crate) fn getAtts(
        &self,
        enc: &ENCODING,
        ptr: *const c_char,
        n: c_int,
        atts: *mut crate::src::lib::xmltok::ATTRIBUTE,
    ) -> c_int {
        self.functions().getAtts(enc, ptr, n, atts)
    }

    pub(crate) fn charRefNumber(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        self.functions().charRefNumber(enc, ptr)
    }

    pub(crate) fn predefinedEntityName(&self, enc: &ENCODING, input: &[c_char]) -> c_int {
        self.functions().predefinedEntityName(enc, input)
    }

    pub(crate) fn updatePosition(
        &self,
        enc: &ENCODING,
        input: &[c_char],
        pos: *mut crate::src::lib::xmltok::POSITION,
    ) {
        self.functions().updatePosition(enc, input, pos)
    }

    pub(crate) fn isPublicId(&self, enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult {
        self.functions().isPublicId(enc, input)
    }

    pub(crate) fn utf8Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_char,
        to_lim: *const c_char,
    ) -> Utf8ConvertResult {
        self.functions()
            .utf8Convert(enc, from_p, from_lim, to_p, to_lim)
    }

    pub(crate) fn utf16Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_ushort,
        to_lim: *const c_ushort,
    ) -> Utf16ConvertResult {
        self.functions()
            .utf16Convert(enc, from_p, from_lim, to_p, to_lim)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct INIT_ENCODING {
    pub initEnc: ENCODING,
    pub encPtr: *mut *const ENCODING,
}

pub type CONVERTER = Option<extern "C" fn(*mut c_void, *const c_char) -> c_int>;

#[inline]
fn as_normal_encoding(enc: &ENCODING) -> &normal_encoding {
    unsafe { &*(enc as *const ENCODING as *const normal_encoding) }
}

#[inline]
fn as_normal_encoding_mut<'a>(enc: *mut ENCODING) -> &'a mut normal_encoding {
    unsafe { &mut *(enc as *mut normal_encoding) }
}

#[inline]
fn as_unknown_encoding(enc: &ENCODING) -> &unknown_encoding {
    unsafe { &*(enc as *const ENCODING as *const unknown_encoding) }
}

#[inline]
fn as_init_encoding(enc: &ENCODING) -> &INIT_ENCODING {
    unsafe { &*(enc as *const ENCODING as *const INIT_ENCODING) }
}

#[inline]
fn c_char_slice_from_ptr_end<'a>(ptr: *const c_char, end: *const c_char) -> &'a [c_char] {
    unsafe { core::slice::from_raw_parts(ptr, end.offset_from(ptr) as usize) }
}

#[inline]
fn read_c_char(ptr: *const c_char) -> c_char {
    unsafe { *ptr }
}

#[inline]
fn read_ptr<T: Copy>(ptr: *const T) -> T {
    unsafe { *ptr }
}

#[inline]
fn read_ptr_at<T: Copy>(ptr: *const T, offset: isize) -> T {
    read_ptr(ptr.wrapping_offset(offset))
}

#[inline]
fn write_ptr<T>(ptr: *mut T, value: T) {
    unsafe {
        *ptr = value;
    }
}

#[inline]
fn write_ptr_at<T>(ptr: *mut T, offset: isize, value: T) {
    write_ptr(ptr.wrapping_offset(offset), value)
}

#[inline]
fn copy_nonoverlapping_c_char(dst: *mut c_char, src: *const c_char, count: usize) {
    unsafe {
        core::ptr::copy_nonoverlapping(src, dst, count);
    }
}

#[inline]
fn read_c_uchar_at(ptr: *const c_char, offset: isize) -> c_uchar {
    unsafe { *((ptr as *const c_uchar).offset(offset)) }
}

#[inline]
fn read_c_char_at(ptr: *const c_char, offset: isize) -> c_char {
    read_c_char(ptr.wrapping_offset(offset))
}

#[inline]
fn c_char_ptr_diff(end: *const c_char, start: *const c_char) -> c_long {
    (end as usize).wrapping_sub(start as usize) as c_long
}

#[inline]
fn c_ushort_ptr_diff(end: *const c_ushort, start: *const c_ushort) -> c_long {
    ((end as usize).wrapping_sub(start as usize) / size_of::<c_ushort>()) as c_long
}

trait UsizeWrappingOffset {
    fn wrapping_offset(self, offset: isize) -> usize;
}

impl UsizeWrappingOffset for usize {
    #[inline]
    fn wrapping_offset(self, offset: isize) -> usize {
        if offset >= 0 {
            self.wrapping_add(offset as usize)
        } else {
            self.wrapping_sub((-offset) as usize)
        }
    }
}

#[inline]
fn mut_ref_from_ptr<'a, T>(ptr: *mut T) -> &'a mut T {
    unsafe { &mut *ptr }
}

#[inline]
fn ref_from_ptr<'a, T>(ptr: *const T) -> &'a T {
    unsafe { &*ptr }
}

#[inline]
fn attribute_mut<'a>(atts: *mut ATTRIBUTE, index: c_int) -> &'a mut ATTRIBUTE {
    unsafe { &mut *atts.wrapping_offset(index as isize) }
}

pub mod xmltok_impl_c {
    use super::*;
    use crate::src::lib::xmltok::XML_TOK_COMMENT_1;
    use crate::src::lib::xmltok::XML_TOK_COND_SECT_OPEN_1;
    use crate::src::lib::xmltok::XML_TOK_DECL_OPEN_1;
    use crate::src::lib::xmltok::XML_TOK_INVALID_1;
    use crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    use crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
    use core::ffi::c_char;
    use core::ffi::c_int;
    use core::ffi::c_long;
    use core::ffi::c_uchar;

    pub(crate) fn normal_scanComment(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let normal_enc = as_normal_encoding(enc);
        let mut next_tok: *const c_char = input.as_ptr();
        let tok: c_int = 'iife_ret_1: {
            let mut offset = 0usize;
            let end = input.len();
            if end >= 1 {
                if input[offset] as c_int != 0x2d {
                    next_tok = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_1 XML_TOK_INVALID_1;
                }
                offset += 1;
                while end - offset >= 1 {
                    match normal_enc.type_0[input[offset] as c_uchar as usize] as c_uint {
                        BT_LEAD2 => {
                            if end - offset < 2 {
                                break 'iife_ret_1 XML_TOK_PARTIAL_CHAR_1;
                            }
                            let ptr = input[offset..].as_ptr();
                            if normal_enc.isInvalid2(enc, ptr) != 0 {
                                next_tok = ptr;
                                break 'iife_ret_1 XML_TOK_INVALID_1;
                            }
                            offset += 2;
                        }
                        BT_LEAD3 => {
                            if end - offset < 3 {
                                break 'iife_ret_1 XML_TOK_PARTIAL_CHAR_1;
                            }
                            let ptr = input[offset..].as_ptr();
                            if normal_enc.isInvalid3(enc, ptr) != 0 {
                                next_tok = ptr;
                                break 'iife_ret_1 XML_TOK_INVALID_1;
                            }
                            offset += 3;
                        }
                        BT_LEAD4 => {
                            if end - offset < 4 {
                                break 'iife_ret_1 XML_TOK_PARTIAL_CHAR_1;
                            }
                            let ptr = input[offset..].as_ptr();
                            if normal_enc.isInvalid4(enc, ptr) != 0 {
                                next_tok = ptr;
                                break 'iife_ret_1 XML_TOK_INVALID_1;
                            }
                            offset += 4;
                        }
                        BT_NONXML | BT_MALFORM | BT_TRAIL => {
                            next_tok = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_1 XML_TOK_INVALID_1;
                        }
                        BT_MINUS => {
                            offset += 1;
                            if end - offset < 1 {
                                break 'iife_ret_1 XML_TOK_PARTIAL_1;
                            }
                            if input[offset] as c_int == 0x2d {
                                offset += 1;
                                if end - offset < 1 {
                                    break 'iife_ret_1 XML_TOK_PARTIAL_1;
                                }
                                if input[offset] as c_int != 0x3e {
                                    next_tok = input.as_ptr().wrapping_add(offset);
                                    break 'iife_ret_1 XML_TOK_INVALID_1;
                                }
                                next_tok = input.as_ptr().wrapping_add(offset + 1);
                                break 'iife_ret_1 XML_TOK_COMMENT_1;
                            }
                        }
                        _ => {
                            offset += 1;
                        }
                    }
                }
            }
            break 'iife_ret_1 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn normal_scanDecl(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_2: {
            let mut offset = 0usize;
            let end = input.len();
            if end - offset < 1 {
                break 'iife_ret_2 XML_TOK_PARTIAL_1;
            }
            match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                BT_MINUS => {
                    break 'iife_ret_2 ({
                        let (tok_value, next_tok_value) =
                            normal_scanComment(enc, &input[(offset + 1)..end]);
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_LSQB => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                    break 'iife_ret_2 XML_TOK_COND_SECT_OPEN_1;
                }
                BT_NMSTRT | BT_HEX => {
                    offset += 1;
                }
                _ => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_2 XML_TOK_INVALID_1;
                }
            }
            while end - offset >= 1 {
                's_129: {
                    match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize]
                        as c_uint
                    {
                        BT_PERCNT => {
                            if end - offset < 2 {
                                break 'iife_ret_2 XML_TOK_PARTIAL_1;
                            }
                            match as_normal_encoding(enc).type_0
                                [input[offset + 1] as c_uchar as usize]
                                as c_uint
                            {
                                BT_S | BT_CR | BT_LF | BT_PERCNT => {
                                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                    break 'iife_ret_2 XML_TOK_INVALID_1;
                                }
                                _ => {}
                            }
                        }
                        BT_S | BT_CR | BT_LF => {}
                        BT_NMSTRT | BT_HEX => {
                            offset += 1;
                            break 's_129;
                        }
                        _ => {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_2 XML_TOK_INVALID_1;
                        }
                    }
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_2 XML_TOK_DECL_OPEN_1;
                }
            }
            break 'iife_ret_2 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn normal_checkPiTarget(_enc: &ENCODING, input: &[c_char]) -> CheckPiTargetResult {
        let mut tok: c_int = 0;
        let tokPtr = &mut tok;
        let result: c_int = 'iife_ret_3: {
            let mut upper: c_int = 0;
            *tokPtr = XML_TOK_PI_1;
            if input.len() != 3 {
                break 'iife_ret_3 1i32;
            }
            match input[0] as c_int {
                ASCII_x_1 => {}
                ASCII_X_1 => {
                    upper = 1i32;
                }
                _ => break 'iife_ret_3 1,
            }
            match input[1] as c_int {
                ASCII_m_1 => {}
                ASCII_M_1 => {
                    upper = 1i32;
                }
                _ => break 'iife_ret_3 1,
            }
            match input[2] as c_int {
                ASCII_l_1 => {}
                ASCII_L_1 => {
                    upper = 1i32;
                }
                _ => break 'iife_ret_3 1,
            }
            if upper != 0 {
                break 'iife_ret_3 0i32;
            }
            *tokPtr = XML_TOK_XML_DECL_1;
            break 'iife_ret_3 1;
        };
        (result, tok)
    }

    pub(crate) fn normal_scanPi(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_4: {
            let mut offset = 0usize;
            let end = input.len();
            let mut tok: c_int = 0;
            let target_offset = offset;
            if end - offset < 1 {
                break 'iife_ret_4 XML_TOK_PARTIAL_1;
            }
            let mut current_block_32: u64;
            match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                BT_NONASCII => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_4 XML_TOK_INVALID_1;
                }
                BT_NMSTRT | BT_HEX => {
                    current_block_32 = 11470911313929454839;
                }
                BT_LEAD2 => {
                    if end - offset < 2 {
                        break 'iife_ret_4 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr()) != 0
                        || as_normal_encoding(enc).isNmstrt2(enc, input[offset..].as_ptr()) == 0
                    {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_4 XML_TOK_INVALID_1;
                    }
                    offset += 2;
                    current_block_32 = 14763689060501151050;
                }
                BT_LEAD3 => {
                    if end - offset < 3 {
                        break 'iife_ret_4 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr()) != 0
                        || as_normal_encoding(enc).isNmstrt3(enc, input[offset..].as_ptr()) == 0
                    {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_4 XML_TOK_INVALID_1;
                    }
                    offset += 3;
                    current_block_32 = 14763689060501151050;
                }
                BT_LEAD4 => {
                    if end - offset < 4 {
                        break 'iife_ret_4 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr()) != 0
                        || as_normal_encoding(enc).isNmstrt4(enc, input[offset..].as_ptr()) == 0
                    {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_4 XML_TOK_INVALID_1;
                    }
                    offset += 4;
                    current_block_32 = 14763689060501151050;
                }
                _ => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_4 XML_TOK_INVALID_1;
                }
            }
            if current_block_32 == 11470911313929454839 {
                offset += 1;
            }
            while end - offset >= 1 {
                let mut current_block_118: u64;
                match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                    BT_NONASCII => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_4 XML_TOK_INVALID_1;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_118 = 8485341570193076947;
                    }
                    BT_LEAD2 => {
                        if end - offset < 2 {
                            break 'iife_ret_4 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName2(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_4 XML_TOK_INVALID_1;
                        }
                        offset += 2;
                        current_block_118 = 13349765058737954042;
                    }
                    BT_LEAD3 => {
                        if end - offset < 3 {
                            break 'iife_ret_4 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName3(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_4 XML_TOK_INVALID_1;
                        }
                        offset += 3;
                        current_block_118 = 13349765058737954042;
                    }
                    BT_LEAD4 => {
                        if end - offset < 4 {
                            break 'iife_ret_4 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName4(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_4 XML_TOK_INVALID_1;
                        }
                        offset += 4;
                        current_block_118 = 13349765058737954042;
                    }
                    BT_S | BT_CR | BT_LF => {
                        if {
                            let (ok_value, tok_value) =
                                normal_checkPiTarget(enc, &input[target_offset..offset]);
                            tok = tok_value;
                            ok_value
                        } == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_4 XML_TOK_INVALID_1;
                        }
                        offset += 1;
                        while end - offset >= 1 {
                            match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize]
                                as c_uint
                            {
                                BT_LEAD2 => {
                                    if end - offset < 2 {
                                        break 'iife_ret_4 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if as_normal_encoding(enc)
                                        .isInvalid2(enc, input[offset..].as_ptr())
                                        != 0
                                    {
                                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                        break 'iife_ret_4 XML_TOK_INVALID_1;
                                    }
                                    offset += 2;
                                }
                                BT_LEAD3 => {
                                    if end - offset < 3 {
                                        break 'iife_ret_4 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if as_normal_encoding(enc)
                                        .isInvalid3(enc, input[offset..].as_ptr())
                                        != 0
                                    {
                                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                        break 'iife_ret_4 XML_TOK_INVALID_1;
                                    }
                                    offset += 3;
                                }
                                BT_LEAD4 => {
                                    if end - offset < 4 {
                                        break 'iife_ret_4 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if as_normal_encoding(enc)
                                        .isInvalid4(enc, input[offset..].as_ptr())
                                        != 0
                                    {
                                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                        break 'iife_ret_4 XML_TOK_INVALID_1;
                                    }
                                    offset += 4;
                                }
                                BT_NONXML | BT_MALFORM | BT_TRAIL => {
                                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                    break 'iife_ret_4 XML_TOK_INVALID_1;
                                }
                                BT_QUEST => {
                                    offset += 1;
                                    if end - offset < 1 {
                                        break 'iife_ret_4 XML_TOK_PARTIAL_1;
                                    }
                                    if input[offset] as c_int == 0x3e {
                                        *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                                        break 'iife_ret_4 tok;
                                    }
                                }
                                _ => {
                                    offset += 1;
                                }
                            }
                        }
                        break 'iife_ret_4 XML_TOK_PARTIAL_1;
                    }
                    BT_QUEST => {
                        if {
                            let (ok_value, tok_value) =
                                normal_checkPiTarget(enc, &input[target_offset..offset]);
                            tok = tok_value;
                            ok_value
                        } == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_4 XML_TOK_INVALID_1;
                        }
                        offset += 1;
                        if end - offset < 1 {
                            break 'iife_ret_4 XML_TOK_PARTIAL_1;
                        }
                        if input[offset] as c_int == 0x3e {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                            break 'iife_ret_4 tok;
                        }
                        current_block_118 = 11310415194689177606;
                    }
                    _ => {
                        current_block_118 = 11310415194689177606;
                    }
                }
                match current_block_118 {
                    11310415194689177606 => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_4 XML_TOK_INVALID_1;
                    }
                    8485341570193076947 => {
                        offset += 1;
                    }
                    _ => {}
                }
            }
            break 'iife_ret_4 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn normal_scanCdataSection(_enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_5: {
            let mut offset = 0usize;
            let end = input.len();
            pub static CDATA_LSQB: [c_char; 6] = [
                ASCII_C as c_char,
                ASCII_D as c_char,
                ASCII_A as c_char,
                ASCII_T as c_char,
                ASCII_A as c_char,
                ASCII_LSQB as c_char,
            ];
            let mut i: c_int = 0;
            if end - offset < 6 {
                break 'iife_ret_5 XML_TOK_PARTIAL_1;
            }
            i = 0;
            while i < 6 {
                if input[offset] as c_int != CDATA_LSQB[i as usize] as c_int {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_5 XML_TOK_INVALID_1;
                }
                i += 1;
                offset += 1;
            }
            *nextTokPtr = input.as_ptr().wrapping_add(offset);
            break 'iife_ret_5 XML_TOK_CDATA_SECT_OPEN_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn normal_cdataSectionTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_6: {
            let mut offset = 0usize;
            let end = input.len();
            if offset >= end {
                break 'iife_ret_6 XML_TOK_NONE_1;
            }
            match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                BT_RSQB => {
                    offset += 1;
                    if end - offset < 1 {
                        break 'iife_ret_6 XML_TOK_PARTIAL_1;
                    }
                    if input[offset] as c_int == 0x5d {
                        offset += 1;
                        if end - offset < 1 {
                            break 'iife_ret_6 XML_TOK_PARTIAL_1;
                        }
                        if input[offset] as c_int != 0x3e {
                            offset -= 1;
                        } else {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                            break 'iife_ret_6 XML_TOK_CDATA_SECT_CLOSE_1;
                        }
                    }
                }
                BT_CR => {
                    offset += 1;
                    if end - offset < 1 {
                        break 'iife_ret_6 XML_TOK_PARTIAL_1;
                    }
                    if as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_int
                        == BT_LF as c_int
                    {
                        offset += 1;
                    }
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_6 XML_TOK_DATA_NEWLINE_1;
                }
                BT_LF => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                    break 'iife_ret_6 XML_TOK_DATA_NEWLINE_1;
                }
                BT_LEAD2 => {
                    if end - offset < 2 {
                        break 'iife_ret_6 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr()) != 0 {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_6 XML_TOK_INVALID_1;
                    }
                    offset += 2;
                }
                BT_LEAD3 => {
                    if end - offset < 3 {
                        break 'iife_ret_6 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr()) != 0 {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_6 XML_TOK_INVALID_1;
                    }
                    offset += 3;
                }
                BT_LEAD4 => {
                    if end - offset < 4 {
                        break 'iife_ret_6 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr()) != 0 {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_6 XML_TOK_INVALID_1;
                    }
                    offset += 4;
                }
                BT_NONXML | BT_MALFORM | BT_TRAIL => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_6 XML_TOK_INVALID_1;
                }
                _ => {
                    offset += 1;
                }
            }
            while end - offset >= 1 {
                match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                    BT_LEAD2 => {
                        if end - offset < 2
                            || as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr())
                                != 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_6 XML_TOK_DATA_CHARS_1;
                        }
                        offset += 2;
                    }
                    BT_LEAD3 => {
                        if end - offset < 3
                            || as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr())
                                != 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_6 XML_TOK_DATA_CHARS_1;
                        }
                        offset += 3;
                    }
                    BT_LEAD4 => {
                        if end - offset < 4
                            || as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr())
                                != 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_6 XML_TOK_DATA_CHARS_1;
                        }
                        offset += 4;
                    }
                    BT_NONXML | BT_MALFORM | BT_TRAIL | BT_CR | BT_LF | BT_RSQB => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_6 XML_TOK_DATA_CHARS_1;
                    }
                    _ => {
                        offset += 1;
                    }
                }
            }
            *nextTokPtr = input.as_ptr().wrapping_add(offset);
            break 'iife_ret_6 XML_TOK_DATA_CHARS_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn normal_scanEndTag(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_7: {
            let mut offset = 0usize;
            let end = input.len();
            if end - offset < 1 {
                break 'iife_ret_7 XML_TOK_PARTIAL_1;
            }
            let mut current_block_32: u64;
            match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                BT_NONASCII => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_7 XML_TOK_INVALID_1;
                }
                BT_NMSTRT | BT_HEX => {
                    current_block_32 = 4324628675098861213;
                }
                BT_LEAD2 => {
                    if end - offset < 2 {
                        break 'iife_ret_7 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr()) != 0
                        || as_normal_encoding(enc).isNmstrt2(enc, input[offset..].as_ptr()) == 0
                    {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_7 XML_TOK_INVALID_1;
                    }
                    offset += 2;
                    current_block_32 = 7056779235015430508;
                }
                BT_LEAD3 => {
                    if end - offset < 3 {
                        break 'iife_ret_7 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr()) != 0
                        || as_normal_encoding(enc).isNmstrt3(enc, input[offset..].as_ptr()) == 0
                    {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_7 XML_TOK_INVALID_1;
                    }
                    offset += 3;
                    current_block_32 = 7056779235015430508;
                }
                BT_LEAD4 => {
                    if end - offset < 4 {
                        break 'iife_ret_7 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr()) != 0
                        || as_normal_encoding(enc).isNmstrt4(enc, input[offset..].as_ptr()) == 0
                    {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_7 XML_TOK_INVALID_1;
                    }
                    offset += 4;
                    current_block_32 = 7056779235015430508;
                }
                _ => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_7 XML_TOK_INVALID_1;
                }
            }
            if current_block_32 == 4324628675098861213 {
                offset += 1;
            }
            while end - offset >= 1 {
                let mut current_block_73: u64;
                match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                    BT_NONASCII => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_7 XML_TOK_INVALID_1;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_73 = 14883924698754021420;
                    }
                    BT_LEAD2 => {
                        if end - offset < 2 {
                            break 'iife_ret_7 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName2(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_7 XML_TOK_INVALID_1;
                        }
                        offset += 2;
                        current_block_73 = 981995395831942902;
                    }
                    BT_LEAD3 => {
                        if end - offset < 3 {
                            break 'iife_ret_7 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName3(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_7 XML_TOK_INVALID_1;
                        }
                        offset += 3;
                        current_block_73 = 981995395831942902;
                    }
                    BT_LEAD4 => {
                        if end - offset < 4 {
                            break 'iife_ret_7 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName4(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_7 XML_TOK_INVALID_1;
                        }
                        offset += 4;
                        current_block_73 = 981995395831942902;
                    }
                    BT_S | BT_CR | BT_LF => {
                        offset += 1;
                        while end - offset >= 1 {
                            match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize]
                                as c_uint
                            {
                                BT_S | BT_CR | BT_LF => {}
                                BT_GT => {
                                    *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                                    break 'iife_ret_7 XML_TOK_END_TAG_1;
                                }
                                _ => {
                                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                    break 'iife_ret_7 XML_TOK_INVALID_1;
                                }
                            }
                            offset += 1;
                        }
                        break 'iife_ret_7 XML_TOK_PARTIAL_1;
                    }
                    BT_COLON_0 => {
                        offset += 1;
                        current_block_73 = 981995395831942902;
                    }
                    BT_GT => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                        break 'iife_ret_7 XML_TOK_END_TAG_1;
                    }
                    _ => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_7 XML_TOK_INVALID_1;
                    }
                }
                if current_block_73 == 14883924698754021420 {
                    offset += 1;
                }
            }
            break 'iife_ret_7 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn normal_scanHexCharRef(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_8: {
            let mut offset = 0usize;
            let end = input.len();
            if end - offset >= 1 {
                match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                    BT_DIGIT | BT_HEX => {}
                    _ => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_8 XML_TOK_INVALID_1;
                    }
                }
                offset += 1;
                while end - offset >= 1 {
                    match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize]
                        as c_uint
                    {
                        BT_DIGIT | BT_HEX => {}
                        BT_SEMI => {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                            break 'iife_ret_8 XML_TOK_CHAR_REF_1;
                        }
                        _ => {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_8 XML_TOK_INVALID_1;
                        }
                    }
                    offset += 1;
                }
            }
            break 'iife_ret_8 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn normal_scanCharRef(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_9: {
            let mut offset = 0usize;
            let end = input.len();
            if end - offset >= 1 {
                if input[offset] as c_int == 0x78 {
                    break 'iife_ret_9 ({
                        let (tok_value, next_tok_value) =
                            normal_scanHexCharRef(enc, &input[(offset + 1)..end]);
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                    BT_DIGIT => {}
                    _ => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_9 XML_TOK_INVALID_1;
                    }
                }
                offset += 1;
                while end - offset >= 1 {
                    match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize]
                        as c_uint
                    {
                        BT_DIGIT => {}
                        BT_SEMI => {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                            break 'iife_ret_9 XML_TOK_CHAR_REF_1;
                        }
                        _ => {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_9 XML_TOK_INVALID_1;
                        }
                    }
                    offset += 1;
                }
            }
            break 'iife_ret_9 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn normal_scanRef(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_10: {
            let mut offset = 0usize;
            let end = input.len();
            if end - offset < 1 {
                break 'iife_ret_10 XML_TOK_PARTIAL_1;
            }
            let mut current_block_33: u64;
            match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                BT_NONASCII => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_10 XML_TOK_INVALID_1;
                }
                BT_NMSTRT | BT_HEX => {
                    current_block_33 = 8911980980495988282;
                }
                BT_LEAD2 => {
                    if end - offset < 2 {
                        break 'iife_ret_10 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr()) != 0
                        || as_normal_encoding(enc).isNmstrt2(enc, input[offset..].as_ptr()) == 0
                    {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_10 XML_TOK_INVALID_1;
                    }
                    offset += 2;
                    current_block_33 = 14763689060501151050;
                }
                BT_LEAD3 => {
                    if end - offset < 3 {
                        break 'iife_ret_10 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr()) != 0
                        || as_normal_encoding(enc).isNmstrt3(enc, input[offset..].as_ptr()) == 0
                    {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_10 XML_TOK_INVALID_1;
                    }
                    offset += 3;
                    current_block_33 = 14763689060501151050;
                }
                BT_LEAD4 => {
                    if end - offset < 4 {
                        break 'iife_ret_10 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr()) != 0
                        || as_normal_encoding(enc).isNmstrt4(enc, input[offset..].as_ptr()) == 0
                    {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_10 XML_TOK_INVALID_1;
                    }
                    offset += 4;
                    current_block_33 = 14763689060501151050;
                }
                BT_NUM => {
                    break 'iife_ret_10 ({
                        let (tok_value, next_tok_value) =
                            normal_scanCharRef(enc, &input[(offset + 1)..end]);
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                _ => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_10 XML_TOK_INVALID_1;
                }
            }
            if current_block_33 == 8911980980495988282 {
                offset += 1;
            }
            while end - offset >= 1 {
                let mut current_block_64: u64;
                match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                    BT_NONASCII => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_10 XML_TOK_INVALID_1;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_64 = 11948064939145634034;
                    }
                    BT_LEAD2 => {
                        if end - offset < 2 {
                            break 'iife_ret_10 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName2(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_10 XML_TOK_INVALID_1;
                        }
                        offset += 2;
                        current_block_64 = 10930818133215224067;
                    }
                    BT_LEAD3 => {
                        if end - offset < 3 {
                            break 'iife_ret_10 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName3(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_10 XML_TOK_INVALID_1;
                        }
                        offset += 3;
                        current_block_64 = 10930818133215224067;
                    }
                    BT_LEAD4 => {
                        if end - offset < 4 {
                            break 'iife_ret_10 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName4(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_10 XML_TOK_INVALID_1;
                        }
                        offset += 4;
                        current_block_64 = 10930818133215224067;
                    }
                    BT_SEMI => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                        break 'iife_ret_10 XML_TOK_ENTITY_REF_1;
                    }
                    _ => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_10 XML_TOK_INVALID_1;
                    }
                }
                if current_block_64 == 11948064939145634034 {
                    offset += 1;
                }
            }
            break 'iife_ret_10 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn normal_scanAtts(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_11: {
            let mut offset = 0usize;
            let end = input.len();
            let mut hadColon: c_int = 0;
            while offset < end {
                let mut current_block_186: u64;
                match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                    BT_NONASCII => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_11 XML_TOK_INVALID_1;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_186 = 3818392175876617014;
                    }
                    BT_LEAD2 => {
                        if end - offset < 2 {
                            break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName2(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_11 XML_TOK_INVALID_1;
                        }
                        offset += 2;
                        current_block_186 = 1634947208139838470;
                    }
                    BT_LEAD3 => {
                        if end - offset < 3 {
                            break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName3(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_11 XML_TOK_INVALID_1;
                        }
                        offset += 3;
                        current_block_186 = 1634947208139838470;
                    }
                    BT_LEAD4 => {
                        if end - offset < 4 {
                            break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName4(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_11 XML_TOK_INVALID_1;
                        }
                        offset += 4;
                        current_block_186 = 1634947208139838470;
                    }
                    BT_COLON_0 => {
                        if hadColon != 0 {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_11 XML_TOK_INVALID_1;
                        }
                        hadColon = 1;
                        offset += 1;
                        if end - offset < 1 {
                            break 'iife_ret_11 XML_TOK_PARTIAL_1;
                        }
                        let mut current_block_64: u64;
                        match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize]
                            as c_uint
                        {
                            BT_NONASCII => {
                                *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                break 'iife_ret_11 XML_TOK_INVALID_1;
                            }
                            BT_NMSTRT | BT_HEX => {
                                current_block_64 = 7083593080606520045;
                            }
                            BT_LEAD2 => {
                                if end - offset < 2 {
                                    break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                                }
                                if as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr())
                                    != 0
                                    || as_normal_encoding(enc)
                                        .isNmstrt2(enc, input[offset..].as_ptr())
                                        == 0
                                {
                                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                    break 'iife_ret_11 XML_TOK_INVALID_1;
                                }
                                offset += 2;
                                current_block_64 = 10930818133215224067;
                            }
                            BT_LEAD3 => {
                                if end - offset < 3 {
                                    break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                                }
                                if as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr())
                                    != 0
                                    || as_normal_encoding(enc)
                                        .isNmstrt3(enc, input[offset..].as_ptr())
                                        == 0
                                {
                                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                    break 'iife_ret_11 XML_TOK_INVALID_1;
                                }
                                offset += 3;
                                current_block_64 = 10930818133215224067;
                            }
                            BT_LEAD4 => {
                                if end - offset < 4 {
                                    break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                                }
                                if as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr())
                                    != 0
                                    || as_normal_encoding(enc)
                                        .isNmstrt4(enc, input[offset..].as_ptr())
                                        == 0
                                {
                                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                    break 'iife_ret_11 XML_TOK_INVALID_1;
                                }
                                offset += 4;
                                current_block_64 = 10930818133215224067;
                            }
                            _ => {
                                *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                break 'iife_ret_11 XML_TOK_INVALID_1;
                            }
                        }
                        if current_block_64 == 7083593080606520045 {
                            offset += 1;
                        }
                        current_block_186 = 1634947208139838470;
                    }
                    BT_S | BT_CR | BT_LF => {
                        loop {
                            let mut t: c_int = 0;
                            offset += 1;
                            if end - offset < 1 {
                                break 'iife_ret_11 XML_TOK_PARTIAL_1;
                            }
                            t = as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize]
                                as c_int;
                            if t == BT_EQUALS as c_int {
                                break;
                            }
                            match t {
                                21 | 10 | 9 => {}
                                _ => {
                                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                    break 'iife_ret_11 XML_TOK_INVALID_1;
                                }
                            }
                        }
                        current_block_186 = 10853015579903106591;
                    }
                    BT_EQUALS => {
                        current_block_186 = 10853015579903106591;
                    }
                    _ => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_11 XML_TOK_INVALID_1;
                    }
                }
                match current_block_186 {
                    10853015579903106591 => {
                        let mut open: c_int = 0;
                        hadColon = 0;
                        loop {
                            offset += 1;
                            if end - offset < 1 {
                                break 'iife_ret_11 XML_TOK_PARTIAL_1;
                            }
                            open = as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize]
                                as c_int;
                            if open == BT_QUOT as c_int || open == BT_APOS as c_int {
                                break;
                            }
                            match open {
                                21 | 10 | 9 => {}
                                _ => {
                                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                    break 'iife_ret_11 XML_TOK_INVALID_1;
                                }
                            }
                        }
                        offset += 1;
                        loop {
                            let mut t_0: c_int = 0;
                            if end - offset < 1 {
                                break 'iife_ret_11 XML_TOK_PARTIAL_1;
                            }
                            t_0 = as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize]
                                as c_int;
                            if t_0 == open {
                                break;
                            }
                            match t_0 {
                                5 => {
                                    if end - offset < 2 {
                                        break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if as_normal_encoding(enc)
                                        .isInvalid2(enc, input[offset..].as_ptr())
                                        != 0
                                    {
                                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                        break 'iife_ret_11 XML_TOK_INVALID_1;
                                    }
                                    offset += 2;
                                }
                                6 => {
                                    if end - offset < 3 {
                                        break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if as_normal_encoding(enc)
                                        .isInvalid3(enc, input[offset..].as_ptr())
                                        != 0
                                    {
                                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                        break 'iife_ret_11 XML_TOK_INVALID_1;
                                    }
                                    offset += 3;
                                }
                                7 => {
                                    if end - offset < 4 {
                                        break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if as_normal_encoding(enc)
                                        .isInvalid4(enc, input[offset..].as_ptr())
                                        != 0
                                    {
                                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                        break 'iife_ret_11 XML_TOK_INVALID_1;
                                    }
                                    offset += 4;
                                }
                                0 | 1 | 8 => {
                                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                    break 'iife_ret_11 XML_TOK_INVALID_1;
                                }
                                3 => {
                                    let mut tok: c_int = {
                                        let (tok_value, next_tok_value) =
                                            normal_scanRef(enc, &input[(offset + 1)..end]);
                                        offset = c_char_ptr_diff(next_tok_value, input.as_ptr())
                                            as usize;
                                        tok_value
                                    };
                                    if tok <= 0 {
                                        if tok == XML_TOK_INVALID_1 {
                                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                        }
                                        break 'iife_ret_11 tok;
                                    }
                                }
                                2 => {
                                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                    break 'iife_ret_11 XML_TOK_INVALID_1;
                                }
                                _ => {
                                    offset += 1;
                                }
                            }
                        }
                        offset += 1;
                        if end - offset < 1 {
                            break 'iife_ret_11 XML_TOK_PARTIAL_1;
                        }
                        match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize]
                            as c_uint
                        {
                            BT_S | BT_CR | BT_LF => {
                                loop {
                                    offset += 1;
                                    if end - offset < 1 {
                                        break 'iife_ret_11 XML_TOK_PARTIAL_1;
                                    }
                                    match as_normal_encoding(enc).type_0
                                        [input[offset] as c_uchar as usize]
                                        as c_uint
                                    {
                                        BT_NONASCII => {
                                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                            break 'iife_ret_11 XML_TOK_INVALID_1;
                                        }
                                        BT_NMSTRT | BT_HEX => {
                                            current_block_186 = 11210999262882855128;
                                            break;
                                        }
                                        BT_LEAD2 => {
                                            if end - offset < 2 {
                                                break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                                            }
                                            if as_normal_encoding(enc)
                                                .isInvalid2(enc, input[offset..].as_ptr())
                                                != 0
                                                || as_normal_encoding(enc)
                                                    .isNmstrt2(enc, input[offset..].as_ptr())
                                                    == 0
                                            {
                                                *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                                break 'iife_ret_11 XML_TOK_INVALID_1;
                                            }
                                            offset += 2;
                                            current_block_186 = 1634947208139838470;
                                            break;
                                        }
                                        BT_LEAD3 => {
                                            if end - offset < 3 {
                                                break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                                            }
                                            if as_normal_encoding(enc)
                                                .isInvalid3(enc, input[offset..].as_ptr())
                                                != 0
                                                || as_normal_encoding(enc)
                                                    .isNmstrt3(enc, input[offset..].as_ptr())
                                                    == 0
                                            {
                                                *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                                break 'iife_ret_11 XML_TOK_INVALID_1;
                                            }
                                            offset += 3;
                                            current_block_186 = 1634947208139838470;
                                            break;
                                        }
                                        BT_LEAD4 => {
                                            if end - offset < 4 {
                                                break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                                            }
                                            if as_normal_encoding(enc)
                                                .isInvalid4(enc, input[offset..].as_ptr())
                                                != 0
                                                || as_normal_encoding(enc)
                                                    .isNmstrt4(enc, input[offset..].as_ptr())
                                                    == 0
                                            {
                                                *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                                break 'iife_ret_11 XML_TOK_INVALID_1;
                                            }
                                            offset += 4;
                                            current_block_186 = 1634947208139838470;
                                            break;
                                        }
                                        BT_S | BT_CR | BT_LF => {}
                                        BT_GT => {
                                            current_block_186 = 2944436519209994553;
                                            break;
                                        }
                                        BT_SOL => {
                                            current_block_186 = 398073151373002430;
                                            break;
                                        }
                                        _ => {
                                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                            break 'iife_ret_11 XML_TOK_INVALID_1;
                                        }
                                    }
                                }
                                match current_block_186 {
                                    2944436519209994553 => {}
                                    398073151373002430 => {}
                                    1634947208139838470 => {}
                                    _ => {
                                        offset += 1;
                                        current_block_186 = 1634947208139838470;
                                    }
                                }
                            }
                            BT_SOL => {
                                current_block_186 = 398073151373002430;
                            }
                            BT_GT => {
                                current_block_186 = 2944436519209994553;
                            }
                            _ => {
                                *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                break 'iife_ret_11 XML_TOK_INVALID_1;
                            }
                        }
                        match current_block_186 {
                            1634947208139838470 => {}
                            _ => match current_block_186 {
                                398073151373002430 => {
                                    offset += 1;
                                    if end - offset < 1 {
                                        break 'iife_ret_11 XML_TOK_PARTIAL_1;
                                    }
                                    if input[offset] as c_int != 0x3e {
                                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                        break 'iife_ret_11 XML_TOK_INVALID_1;
                                    }
                                    *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                                    break 'iife_ret_11 XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1;
                                }
                                _ => {
                                    *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                                    break 'iife_ret_11 XML_TOK_START_TAG_WITH_ATTS_1;
                                }
                            },
                        }
                    }
                    3818392175876617014 => {
                        offset += 1;
                    }
                    _ => {}
                }
            }
            break 'iife_ret_11 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn normal_scanLt(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_12: {
            let mut offset = 0usize;
            let end = input.len();
            let mut hadColon: c_int = 0;
            if end - offset < 1 {
                break 'iife_ret_12 XML_TOK_PARTIAL_1;
            }
            let mut current_block_45: u64;
            match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                BT_NONASCII => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_12 XML_TOK_INVALID_1;
                }
                BT_NMSTRT | BT_HEX => {
                    current_block_45 = 2165477741955893522;
                }
                BT_LEAD2 => {
                    if end - offset < 2 {
                        break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr()) != 0
                        || as_normal_encoding(enc).isNmstrt2(enc, input[offset..].as_ptr()) == 0
                    {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_12 XML_TOK_INVALID_1;
                    }
                    offset += 2;
                    current_block_45 = 8180496224585318153;
                }
                BT_LEAD3 => {
                    if end - offset < 3 {
                        break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr()) != 0
                        || as_normal_encoding(enc).isNmstrt3(enc, input[offset..].as_ptr()) == 0
                    {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_12 XML_TOK_INVALID_1;
                    }
                    offset += 3;
                    current_block_45 = 8180496224585318153;
                }
                BT_LEAD4 => {
                    if end - offset < 4 {
                        break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr()) != 0
                        || as_normal_encoding(enc).isNmstrt4(enc, input[offset..].as_ptr()) == 0
                    {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_12 XML_TOK_INVALID_1;
                    }
                    offset += 4;
                    current_block_45 = 8180496224585318153;
                }
                BT_EXCL => {
                    offset += 1;
                    if end - offset < 1 {
                        break 'iife_ret_12 XML_TOK_PARTIAL_1;
                    }
                    match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize]
                        as c_uint
                    {
                        BT_MINUS => {
                            break 'iife_ret_12 ({
                                let (tok_value, next_tok_value) =
                                    normal_scanComment(enc, &input[(offset + 1)..end]);
                                *nextTokPtr = next_tok_value;
                                tok_value
                            });
                        }
                        BT_LSQB => {
                            break 'iife_ret_12 ({
                                let (tok_value, next_tok_value) =
                                    normal_scanCdataSection(enc, &input[(offset + 1)..end]);
                                *nextTokPtr = next_tok_value;
                                tok_value
                            });
                        }
                        _ => {}
                    }
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_12 XML_TOK_INVALID_1;
                }
                BT_QUEST => {
                    break 'iife_ret_12 ({
                        let (tok_value, next_tok_value) =
                            normal_scanPi(enc, &input[(offset + 1)..end]);
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_SOL => {
                    break 'iife_ret_12 ({
                        let (tok_value, next_tok_value) =
                            normal_scanEndTag(enc, &input[(offset + 1)..end]);
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                _ => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_12 XML_TOK_INVALID_1;
                }
            }
            if current_block_45 == 2165477741955893522 {
                offset += 1;
            }
            hadColon = 0;
            while offset < end {
                let mut current_block_161: u64;
                match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                    BT_NONASCII => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_12 XML_TOK_INVALID_1;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_161 = 6701753098489376273;
                    }
                    BT_LEAD2 => {
                        if end - offset < 2 {
                            break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName2(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_12 XML_TOK_INVALID_1;
                        }
                        offset += 2;
                        current_block_161 = 14714495436747744489;
                    }
                    BT_LEAD3 => {
                        if end - offset < 3 {
                            break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName3(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_12 XML_TOK_INVALID_1;
                        }
                        offset += 3;
                        current_block_161 = 14714495436747744489;
                    }
                    BT_LEAD4 => {
                        if end - offset < 4 {
                            break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName4(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_12 XML_TOK_INVALID_1;
                        }
                        offset += 4;
                        current_block_161 = 14714495436747744489;
                    }
                    BT_COLON_0 => {
                        if hadColon != 0 {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_12 XML_TOK_INVALID_1;
                        }
                        hadColon = 1;
                        offset += 1;
                        if end - offset < 1 {
                            break 'iife_ret_12 XML_TOK_PARTIAL_1;
                        }
                        let mut current_block_112: u64;
                        match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize]
                            as c_uint
                        {
                            BT_NONASCII => {
                                *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                break 'iife_ret_12 XML_TOK_INVALID_1;
                            }
                            BT_NMSTRT | BT_HEX => {
                                current_block_112 = 9169466483824547789;
                            }
                            BT_LEAD2 => {
                                if end - offset < 2 {
                                    break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                                }
                                if as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr())
                                    != 0
                                    || as_normal_encoding(enc)
                                        .isNmstrt2(enc, input[offset..].as_ptr())
                                        == 0
                                {
                                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                    break 'iife_ret_12 XML_TOK_INVALID_1;
                                }
                                offset += 2;
                                current_block_112 = 2616667235040759262;
                            }
                            BT_LEAD3 => {
                                if end - offset < 3 {
                                    break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                                }
                                if as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr())
                                    != 0
                                    || as_normal_encoding(enc)
                                        .isNmstrt3(enc, input[offset..].as_ptr())
                                        == 0
                                {
                                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                    break 'iife_ret_12 XML_TOK_INVALID_1;
                                }
                                offset += 3;
                                current_block_112 = 2616667235040759262;
                            }
                            BT_LEAD4 => {
                                if end - offset < 4 {
                                    break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                                }
                                if as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr())
                                    != 0
                                    || as_normal_encoding(enc)
                                        .isNmstrt4(enc, input[offset..].as_ptr())
                                        == 0
                                {
                                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                    break 'iife_ret_12 XML_TOK_INVALID_1;
                                }
                                offset += 4;
                                current_block_112 = 2616667235040759262;
                            }
                            _ => {
                                *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                break 'iife_ret_12 XML_TOK_INVALID_1;
                            }
                        }
                        if current_block_112 == 9169466483824547789 {
                            offset += 1;
                        }
                        current_block_161 = 14714495436747744489;
                    }
                    BT_S | BT_CR | BT_LF => {
                        offset += 1;
                        loop {
                            if end - offset < 1 {
                                current_block_161 = 13215501469961642988;
                                break;
                            }
                            match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize]
                                as c_uint
                            {
                                BT_NONASCII => {
                                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                    break 'iife_ret_12 XML_TOK_INVALID_1;
                                }
                                BT_NMSTRT | BT_HEX => {
                                    current_block_161 = 7939927167482451446;
                                }
                                BT_LEAD2 => {
                                    if end - offset < 2 {
                                        break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if as_normal_encoding(enc)
                                        .isInvalid2(enc, input[offset..].as_ptr())
                                        != 0
                                        || as_normal_encoding(enc)
                                            .isNmstrt2(enc, input[offset..].as_ptr())
                                            == 0
                                    {
                                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                        break 'iife_ret_12 XML_TOK_INVALID_1;
                                    }
                                    offset += 2;
                                    current_block_161 = 16314074004867283505;
                                }
                                BT_LEAD3 => {
                                    if end - offset < 3 {
                                        break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if as_normal_encoding(enc)
                                        .isInvalid3(enc, input[offset..].as_ptr())
                                        != 0
                                        || as_normal_encoding(enc)
                                            .isNmstrt3(enc, input[offset..].as_ptr())
                                            == 0
                                    {
                                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                        break 'iife_ret_12 XML_TOK_INVALID_1;
                                    }
                                    offset += 3;
                                    current_block_161 = 16314074004867283505;
                                }
                                BT_LEAD4 => {
                                    if end - offset < 4 {
                                        break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if as_normal_encoding(enc)
                                        .isInvalid4(enc, input[offset..].as_ptr())
                                        != 0
                                        || as_normal_encoding(enc)
                                            .isNmstrt4(enc, input[offset..].as_ptr())
                                            == 0
                                    {
                                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                        break 'iife_ret_12 XML_TOK_INVALID_1;
                                    }
                                    offset += 4;
                                    current_block_161 = 16314074004867283505;
                                }
                                BT_GT => {
                                    current_block_161 = 5640065479517572396;
                                    break;
                                }
                                BT_SOL => {
                                    current_block_161 = 12549409781983877175;
                                    break;
                                }
                                BT_S | BT_CR | BT_LF => {
                                    offset += 1;
                                    continue;
                                }
                                _ => {
                                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                    break 'iife_ret_12 XML_TOK_INVALID_1;
                                }
                            }
                            if current_block_161 == 7939927167482451446 {
                                offset += 1;
                            }
                            break 'iife_ret_12 ({
                                let (tok_value, next_tok_value) =
                                    normal_scanAtts(enc, &input[offset..end]);
                                *nextTokPtr = next_tok_value;
                                tok_value
                            });
                        }
                        match current_block_161 {
                            5640065479517572396 => {}
                            12549409781983877175 => {}
                            _ => break 'iife_ret_12 XML_TOK_PARTIAL_1,
                        }
                    }
                    BT_GT => {
                        current_block_161 = 5640065479517572396;
                    }
                    BT_SOL => {
                        current_block_161 = 12549409781983877175;
                    }
                    _ => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_12 XML_TOK_INVALID_1;
                    }
                }
                match current_block_161 {
                    12549409781983877175 => {
                        offset += 1;
                        if end - offset < 1 {
                            break 'iife_ret_12 XML_TOK_PARTIAL_1;
                        }
                        if input[offset] as c_int != 0x3e {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_12 XML_TOK_INVALID_1;
                        }
                        *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                        break 'iife_ret_12 XML_TOK_EMPTY_ELEMENT_NO_ATTS_1;
                    }
                    5640065479517572396 => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                        break 'iife_ret_12 XML_TOK_START_TAG_NO_ATTS_1;
                    }
                    6701753098489376273 => {
                        offset += 1;
                    }
                    _ => {}
                }
            }
            break 'iife_ret_12 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn normal_contentTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_13: {
            let mut offset = 0usize;
            let end = input.len();
            if offset >= end {
                break 'iife_ret_13 XML_TOK_NONE_1;
            }
            match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                BT_LT => {
                    break 'iife_ret_13 ({
                        let (tok_value, next_tok_value) =
                            normal_scanLt(enc, &input[(offset + 1)..end]);
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_AMP => {
                    break 'iife_ret_13 ({
                        let (tok_value, next_tok_value) =
                            normal_scanRef(enc, &input[(offset + 1)..end]);
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_CR => {
                    offset += 1;
                    if end - offset < 1 {
                        break 'iife_ret_13 XML_TOK_TRAILING_CR_1;
                    }
                    if as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_int
                        == BT_LF as c_int
                    {
                        offset += 1;
                    }
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_13 XML_TOK_DATA_NEWLINE_1;
                }
                BT_LF => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                    break 'iife_ret_13 XML_TOK_DATA_NEWLINE_1;
                }
                BT_RSQB => {
                    offset += 1;
                    if end - offset < 1 {
                        break 'iife_ret_13 XML_TOK_TRAILING_RSQB_1;
                    }
                    if input[offset] as c_int == 0x5d {
                        offset += 1;
                        if end - offset < 1 {
                            break 'iife_ret_13 XML_TOK_TRAILING_RSQB_1;
                        }
                        if input[offset] as c_int != 0x3e {
                            offset -= 1;
                        } else {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_13 XML_TOK_INVALID_1;
                        }
                    }
                }
                BT_LEAD2 => {
                    if end - offset < 2 {
                        break 'iife_ret_13 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr()) != 0 {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_13 XML_TOK_INVALID_1;
                    }
                    offset += 2;
                }
                BT_LEAD3 => {
                    if end - offset < 3 {
                        break 'iife_ret_13 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr()) != 0 {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_13 XML_TOK_INVALID_1;
                    }
                    offset += 3;
                }
                BT_LEAD4 => {
                    if end - offset < 4 {
                        break 'iife_ret_13 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr()) != 0 {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_13 XML_TOK_INVALID_1;
                    }
                    offset += 4;
                }
                BT_NONXML | BT_MALFORM | BT_TRAIL => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_13 XML_TOK_INVALID_1;
                }
                _ => {
                    offset += 1;
                }
            }
            while offset < end {
                let mut current_block_76: u64;
                match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                    BT_LEAD2 => {
                        if end - offset < 2
                            || as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr())
                                != 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_13 XML_TOK_DATA_CHARS_1;
                        }
                        offset += 2;
                        current_block_76 = 7158658067966855297;
                    }
                    BT_LEAD3 => {
                        if end - offset < 3
                            || as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr())
                                != 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_13 XML_TOK_DATA_CHARS_1;
                        }
                        offset += 3;
                        current_block_76 = 7158658067966855297;
                    }
                    BT_LEAD4 => {
                        if end - offset < 4
                            || as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr())
                                != 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_13 XML_TOK_DATA_CHARS_1;
                        }
                        offset += 4;
                        current_block_76 = 7158658067966855297;
                    }
                    BT_RSQB => {
                        if end - offset >= 2 {
                            if input[offset + 1] as c_int != 0x5d {
                                offset += 1;
                                current_block_76 = 7158658067966855297;
                            } else if end - offset >= 3 {
                                if input[offset + 2] as c_int != 0x3e {
                                    offset += 1;
                                } else {
                                    *nextTokPtr = input.as_ptr().wrapping_add(offset + 2);
                                    break 'iife_ret_13 XML_TOK_INVALID_1;
                                }
                                current_block_76 = 7158658067966855297;
                            } else {
                                current_block_76 = 1999360611754201214;
                            }
                        } else {
                            current_block_76 = 1999360611754201214;
                        }
                    }
                    BT_AMP | BT_LT | BT_NONXML | BT_MALFORM | BT_TRAIL | BT_CR | BT_LF => {
                        current_block_76 = 1999360611754201214;
                    }
                    _ => {
                        offset += 1;
                        current_block_76 = 7158658067966855297;
                    }
                }
                match current_block_76 {
                    7158658067966855297 => {}
                    _ => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_13 XML_TOK_DATA_CHARS_1;
                    }
                }
            }
            *nextTokPtr = input.as_ptr().wrapping_add(offset);
            break 'iife_ret_13 XML_TOK_DATA_CHARS_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn normal_scanPercent(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_14: {
            let mut offset = 0usize;
            let end = input.len();
            if end - offset < 1 {
                break 'iife_ret_14 XML_TOK_PARTIAL_1;
            }
            let mut current_block_34: u64;
            match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                BT_NONASCII => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_14 XML_TOK_INVALID_1;
                }
                BT_NMSTRT | BT_HEX => {
                    current_block_34 = 12478441211659886388;
                }
                BT_LEAD2 => {
                    if end - offset < 2 {
                        break 'iife_ret_14 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr()) != 0
                        || as_normal_encoding(enc).isNmstrt2(enc, input[offset..].as_ptr()) == 0
                    {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_14 XML_TOK_INVALID_1;
                    }
                    offset += 2;
                    current_block_34 = 4761528863920922185;
                }
                BT_LEAD3 => {
                    if end - offset < 3 {
                        break 'iife_ret_14 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr()) != 0
                        || as_normal_encoding(enc).isNmstrt3(enc, input[offset..].as_ptr()) == 0
                    {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_14 XML_TOK_INVALID_1;
                    }
                    offset += 3;
                    current_block_34 = 4761528863920922185;
                }
                BT_LEAD4 => {
                    if end - offset < 4 {
                        break 'iife_ret_14 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr()) != 0
                        || as_normal_encoding(enc).isNmstrt4(enc, input[offset..].as_ptr()) == 0
                    {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_14 XML_TOK_INVALID_1;
                    }
                    offset += 4;
                    current_block_34 = 4761528863920922185;
                }
                BT_S | BT_LF | BT_CR | BT_PERCNT => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_14 XML_TOK_PERCENT_1;
                }
                _ => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_14 XML_TOK_INVALID_1;
                }
            }
            if current_block_34 == 12478441211659886388 {
                offset += 1;
            }
            while end - offset >= 1 {
                let mut current_block_65: u64;
                match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                    BT_NONASCII => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_14 XML_TOK_INVALID_1;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_65 = 7770117754142564343;
                    }
                    BT_LEAD2 => {
                        if end - offset < 2 {
                            break 'iife_ret_14 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName2(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_14 XML_TOK_INVALID_1;
                        }
                        offset += 2;
                        current_block_65 = 16415152177862271243;
                    }
                    BT_LEAD3 => {
                        if end - offset < 3 {
                            break 'iife_ret_14 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName3(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_14 XML_TOK_INVALID_1;
                        }
                        offset += 3;
                        current_block_65 = 16415152177862271243;
                    }
                    BT_LEAD4 => {
                        if end - offset < 4 {
                            break 'iife_ret_14 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName4(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_14 XML_TOK_INVALID_1;
                        }
                        offset += 4;
                        current_block_65 = 16415152177862271243;
                    }
                    BT_SEMI => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                        break 'iife_ret_14 XML_TOK_PARAM_ENTITY_REF_1;
                    }
                    _ => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_14 XML_TOK_INVALID_1;
                    }
                }
                if current_block_65 == 7770117754142564343 {
                    offset += 1;
                }
            }
            break 'iife_ret_14 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn normal_scanPoundName(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_15: {
            let mut offset = 0usize;
            let end = input.len();
            if end - offset < 1 {
                break 'iife_ret_15 XML_TOK_PARTIAL_1;
            }
            let mut current_block_32: u64;
            match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                BT_NONASCII => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_15 XML_TOK_INVALID_1;
                }
                BT_NMSTRT | BT_HEX => {
                    current_block_32 = 1867613116081924762;
                }
                BT_LEAD2 => {
                    if end - offset < 2 {
                        break 'iife_ret_15 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr()) != 0
                        || as_normal_encoding(enc).isNmstrt2(enc, input[offset..].as_ptr()) == 0
                    {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_15 XML_TOK_INVALID_1;
                    }
                    offset += 2;
                    current_block_32 = 7056779235015430508;
                }
                BT_LEAD3 => {
                    if end - offset < 3 {
                        break 'iife_ret_15 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr()) != 0
                        || as_normal_encoding(enc).isNmstrt3(enc, input[offset..].as_ptr()) == 0
                    {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_15 XML_TOK_INVALID_1;
                    }
                    offset += 3;
                    current_block_32 = 7056779235015430508;
                }
                BT_LEAD4 => {
                    if end - offset < 4 {
                        break 'iife_ret_15 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr()) != 0
                        || as_normal_encoding(enc).isNmstrt4(enc, input[offset..].as_ptr()) == 0
                    {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_15 XML_TOK_INVALID_1;
                    }
                    offset += 4;
                    current_block_32 = 7056779235015430508;
                }
                _ => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_15 XML_TOK_INVALID_1;
                }
            }
            if current_block_32 == 1867613116081924762 {
                offset += 1;
            }
            while end - offset >= 1 {
                let mut current_block_63: u64;
                match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                    BT_NONASCII => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_15 XML_TOK_INVALID_1;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_63 = 226587729178875444;
                    }
                    BT_LEAD2 => {
                        if end - offset < 2 {
                            break 'iife_ret_15 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName2(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_15 XML_TOK_INVALID_1;
                        }
                        offset += 2;
                        current_block_63 = 10380409671385728102;
                    }
                    BT_LEAD3 => {
                        if end - offset < 3 {
                            break 'iife_ret_15 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName3(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_15 XML_TOK_INVALID_1;
                        }
                        offset += 3;
                        current_block_63 = 10380409671385728102;
                    }
                    BT_LEAD4 => {
                        if end - offset < 4 {
                            break 'iife_ret_15 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName4(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_15 XML_TOK_INVALID_1;
                        }
                        offset += 4;
                        current_block_63 = 10380409671385728102;
                    }
                    BT_CR | BT_LF | BT_S | BT_RPAR | BT_GT | BT_PERCNT | BT_VERBAR => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_15 XML_TOK_POUND_NAME_1;
                    }
                    _ => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_15 XML_TOK_INVALID_1;
                    }
                }
                if current_block_63 == 226587729178875444 {
                    offset += 1;
                }
            }
            break 'iife_ret_15 -XML_TOK_POUND_NAME_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn normal_scanLit(
        mut open: c_int,
        enc: &ENCODING,
        input: &[c_char],
    ) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_16: {
            let mut offset = 0usize;
            let end = input.len();
            while end - offset >= 1 {
                let t: c_int =
                    as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_int;
                match t {
                    5 => {
                        if end - offset < 2 {
                            break 'iife_ret_16 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr()) != 0 {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_16 XML_TOK_INVALID_1;
                        }
                        offset += 2;
                    }
                    6 => {
                        if end - offset < 3 {
                            break 'iife_ret_16 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr()) != 0 {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_16 XML_TOK_INVALID_1;
                        }
                        offset += 3;
                    }
                    7 => {
                        if end - offset < 4 {
                            break 'iife_ret_16 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr()) != 0 {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_16 XML_TOK_INVALID_1;
                        }
                        offset += 4;
                    }
                    0 | 1 | 8 => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_16 XML_TOK_INVALID_1;
                    }
                    12 | 13 => {
                        offset += 1;
                        if t == open {
                            if end - offset < 1 {
                                break 'iife_ret_16 -XML_TOK_LITERAL_1;
                            }
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize]
                                as c_uint
                            {
                                BT_S | BT_CR | BT_LF | BT_GT | BT_PERCNT | BT_LSQB => {
                                    break 'iife_ret_16 XML_TOK_LITERAL_1
                                }
                                _ => break 'iife_ret_16 XML_TOK_INVALID_1,
                            }
                        }
                    }
                    _ => {
                        offset += 1;
                    }
                }
            }
            break 'iife_ret_16 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn normal_prologTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_17: {
            let mut offset = 0usize;
            let end = input.len();
            let mut tok: c_int = 0;
            if offset >= end {
                break 'iife_ret_17 XML_TOK_NONE_1;
            }
            let mut current_block_124: u64;
            match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                BT_QUOT => {
                    break 'iife_ret_17 ({
                        let (tok_value, next_tok_value) =
                            normal_scanLit(BT_QUOT as c_int, enc, &input[(offset + 1)..end]);
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_APOS => {
                    break 'iife_ret_17 ({
                        let (tok_value, next_tok_value) =
                            normal_scanLit(BT_APOS as c_int, enc, &input[(offset + 1)..end]);
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_LT => {
                    offset += 1;
                    if end - offset < 1 {
                        break 'iife_ret_17 XML_TOK_PARTIAL_1;
                    }
                    match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize]
                        as c_uint
                    {
                        BT_EXCL => {
                            break 'iife_ret_17 ({
                                let (tok_value, next_tok_value) =
                                    normal_scanDecl(enc, &input[(offset + 1)..end]);
                                *nextTokPtr = next_tok_value;
                                tok_value
                            });
                        }
                        BT_QUEST => {
                            break 'iife_ret_17 ({
                                let (tok_value, next_tok_value) =
                                    normal_scanPi(enc, &input[(offset + 1)..end]);
                                *nextTokPtr = next_tok_value;
                                tok_value
                            });
                        }
                        BT_NMSTRT | BT_HEX | BT_NONASCII | BT_LEAD2 | BT_LEAD3 | BT_LEAD4 => {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset - 1);
                            break 'iife_ret_17 XML_TOK_INSTANCE_START;
                        }
                        _ => {}
                    }
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_17 XML_TOK_INVALID_1;
                }
                BT_CR => {
                    if offset + 1 == end {
                        *nextTokPtr = input.as_ptr().wrapping_add(end);
                        break 'iife_ret_17 -XML_TOK_PROLOG_S_1;
                    }
                    current_block_124 = 6405334113228567422;
                }
                BT_S | BT_LF => {
                    current_block_124 = 6405334113228567422;
                }
                BT_PERCNT => {
                    break 'iife_ret_17 ({
                        let (tok_value, next_tok_value) =
                            normal_scanPercent(enc, &input[(offset + 1)..end]);
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_COMMA => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                    break 'iife_ret_17 XML_TOK_COMMA_1;
                }
                BT_LSQB => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                    break 'iife_ret_17 XML_TOK_OPEN_BRACKET_1;
                }
                BT_RSQB => {
                    offset += 1;
                    if end - offset < 1 {
                        break 'iife_ret_17 -XML_TOK_CLOSE_BRACKET_1;
                    }
                    if input[offset] as c_int == 0x5d {
                        if end - offset < 2 {
                            break 'iife_ret_17 XML_TOK_PARTIAL_1;
                        }
                        if input[offset + 1] as c_int == 0x3e {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset + 2);
                            break 'iife_ret_17 XML_TOK_COND_SECT_CLOSE_1;
                        }
                    }
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_17 XML_TOK_CLOSE_BRACKET_1;
                }
                BT_LPAR => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                    break 'iife_ret_17 XML_TOK_OPEN_PAREN_1;
                }
                BT_RPAR => {
                    offset += 1;
                    if end - offset < 1 {
                        break 'iife_ret_17 -XML_TOK_CLOSE_PAREN_1;
                    }
                    match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize]
                        as c_uint
                    {
                        BT_AST => {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                            break 'iife_ret_17 XML_TOK_CLOSE_PAREN_ASTERISK_1;
                        }
                        BT_QUEST => {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                            break 'iife_ret_17 XML_TOK_CLOSE_PAREN_QUESTION_1;
                        }
                        BT_PLUS => {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                            break 'iife_ret_17 XML_TOK_CLOSE_PAREN_PLUS_1;
                        }
                        BT_CR | BT_LF | BT_S | BT_GT | BT_COMMA | BT_VERBAR | BT_RPAR => {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_17 XML_TOK_CLOSE_PAREN_1;
                        }
                        _ => {}
                    }
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_17 XML_TOK_INVALID_1;
                }
                BT_VERBAR => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                    break 'iife_ret_17 XML_TOK_OR_1;
                }
                BT_GT => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                    break 'iife_ret_17 XML_TOK_DECL_CLOSE_1;
                }
                BT_NUM => {
                    break 'iife_ret_17 ({
                        let (tok_value, next_tok_value) =
                            normal_scanPoundName(enc, &input[(offset + 1)..end]);
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_LEAD2 => {
                    if end - offset < 2 {
                        break 'iife_ret_17 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr()) != 0 {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_17 XML_TOK_INVALID_1;
                    }
                    if as_normal_encoding(enc).isNmstrt2(enc, input[offset..].as_ptr()) != 0 {
                        offset += 2;
                        tok = XML_TOK_NAME;
                    } else if as_normal_encoding(enc).isName2(enc, input[offset..].as_ptr()) != 0 {
                        offset += 2;
                        tok = XML_TOK_NMTOKEN_1;
                    } else {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_17 XML_TOK_INVALID_1;
                    }
                    current_block_124 = 2956972668325154207;
                }
                BT_LEAD3 => {
                    if end - offset < 3 {
                        break 'iife_ret_17 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr()) != 0 {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_17 XML_TOK_INVALID_1;
                    }
                    if as_normal_encoding(enc).isNmstrt3(enc, input[offset..].as_ptr()) != 0 {
                        offset += 3;
                        tok = XML_TOK_NAME;
                    } else if as_normal_encoding(enc).isName3(enc, input[offset..].as_ptr()) != 0 {
                        offset += 3;
                        tok = XML_TOK_NMTOKEN_1;
                    } else {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_17 XML_TOK_INVALID_1;
                    }
                    current_block_124 = 2956972668325154207;
                }
                BT_LEAD4 => {
                    if end - offset < 4 {
                        break 'iife_ret_17 XML_TOK_PARTIAL_CHAR_1;
                    }
                    if as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr()) != 0 {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_17 XML_TOK_INVALID_1;
                    }
                    if as_normal_encoding(enc).isNmstrt4(enc, input[offset..].as_ptr()) != 0 {
                        offset += 4;
                        tok = XML_TOK_NAME;
                    } else if as_normal_encoding(enc).isName4(enc, input[offset..].as_ptr()) != 0 {
                        offset += 4;
                        tok = XML_TOK_NMTOKEN_1;
                    } else {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_17 XML_TOK_INVALID_1;
                    }
                    current_block_124 = 2956972668325154207;
                }
                BT_NMSTRT | BT_HEX => {
                    tok = XML_TOK_NAME;
                    offset += 1;
                    current_block_124 = 2956972668325154207;
                }
                BT_DIGIT | BT_NAME | BT_MINUS | BT_COLON_0 => {
                    tok = XML_TOK_NMTOKEN_1;
                    offset += 1;
                    current_block_124 = 2956972668325154207;
                }
                29 | _ => {
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_17 XML_TOK_INVALID_1;
                }
            }
            match current_block_124 {
                2956972668325154207 => {}
                _ => {
                    loop {
                        offset += 1;
                        if end - offset < 1 {
                            break;
                        }
                        let mut current_block_32: u64;
                        match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize]
                            as c_uint
                        {
                            BT_S | BT_LF => {
                                current_block_32 = 17500079516916021833;
                            }
                            BT_CR => {
                                if offset + 1 != end {
                                    current_block_32 = 17500079516916021833;
                                } else {
                                    current_block_32 = 3687018382384043009;
                                }
                            }
                            _ => {
                                current_block_32 = 3687018382384043009;
                            }
                        }
                        match current_block_32 {
                            17500079516916021833 => {}
                            _ => {
                                *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                break 'iife_ret_17 XML_TOK_PROLOG_S_1;
                            }
                        }
                    }
                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                    break 'iife_ret_17 XML_TOK_PROLOG_S_1;
                }
            }
            while offset < end {
                let mut current_block_210: u64;
                match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                    BT_NONASCII => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_17 XML_TOK_INVALID_1;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_210 = 17210391895989911948;
                    }
                    BT_LEAD2 => {
                        if end - offset < 2 {
                            break 'iife_ret_17 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName2(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_17 XML_TOK_INVALID_1;
                        }
                        offset += 2;
                        current_block_210 = 14244298717249035578;
                    }
                    BT_LEAD3 => {
                        if end - offset < 3 {
                            break 'iife_ret_17 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName3(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_17 XML_TOK_INVALID_1;
                        }
                        offset += 3;
                        current_block_210 = 14244298717249035578;
                    }
                    BT_LEAD4 => {
                        if end - offset < 4 {
                            break 'iife_ret_17 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr()) != 0
                            || as_normal_encoding(enc).isName4(enc, input[offset..].as_ptr()) == 0
                        {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_17 XML_TOK_INVALID_1;
                        }
                        offset += 4;
                        current_block_210 = 14244298717249035578;
                    }
                    BT_GT | BT_RPAR | BT_COMMA | BT_VERBAR | BT_LSQB | BT_PERCNT | BT_S | BT_CR
                    | BT_LF => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_17 tok;
                    }
                    BT_COLON_0 => {
                        offset += 1;
                        match tok {
                            XML_TOK_NAME => {
                                if end - offset < 1 {
                                    break 'iife_ret_17 XML_TOK_PARTIAL_1;
                                }
                                tok = XML_TOK_PREFIXED_NAME;
                                let mut current_block_187: u64;
                                match as_normal_encoding(enc).type_0
                                    [input[offset] as c_uchar as usize]
                                    as c_uint
                                {
                                    BT_NONASCII => {
                                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                        break 'iife_ret_17 XML_TOK_INVALID_1;
                                    }
                                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                                        current_block_187 = 2692573546887820791;
                                    }
                                    BT_LEAD2 => {
                                        if end - offset < 2 {
                                            break 'iife_ret_17 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if as_normal_encoding(enc)
                                            .isInvalid2(enc, input[offset..].as_ptr())
                                            != 0
                                            || as_normal_encoding(enc)
                                                .isName2(enc, input[offset..].as_ptr())
                                                == 0
                                        {
                                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                            break 'iife_ret_17 XML_TOK_INVALID_1;
                                        }
                                        offset += 2;
                                        current_block_187 = 9812798724717783973;
                                    }
                                    BT_LEAD3 => {
                                        if end - offset < 3 {
                                            break 'iife_ret_17 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if as_normal_encoding(enc)
                                            .isInvalid3(enc, input[offset..].as_ptr())
                                            != 0
                                            || as_normal_encoding(enc)
                                                .isName3(enc, input[offset..].as_ptr())
                                                == 0
                                        {
                                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                            break 'iife_ret_17 XML_TOK_INVALID_1;
                                        }
                                        offset += 3;
                                        current_block_187 = 9812798724717783973;
                                    }
                                    BT_LEAD4 => {
                                        if end - offset < 4 {
                                            break 'iife_ret_17 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if as_normal_encoding(enc)
                                            .isInvalid4(enc, input[offset..].as_ptr())
                                            != 0
                                            || as_normal_encoding(enc)
                                                .isName4(enc, input[offset..].as_ptr())
                                                == 0
                                        {
                                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                            break 'iife_ret_17 XML_TOK_INVALID_1;
                                        }
                                        offset += 4;
                                        current_block_187 = 9812798724717783973;
                                    }
                                    _ => {
                                        tok = XML_TOK_NMTOKEN_1;
                                        current_block_187 = 9812798724717783973;
                                    }
                                }
                                if current_block_187 == 2692573546887820791 {
                                    offset += 1;
                                }
                            }
                            XML_TOK_PREFIXED_NAME => {
                                tok = XML_TOK_NMTOKEN_1;
                            }
                            _ => {}
                        }
                        current_block_210 = 14244298717249035578;
                    }
                    BT_PLUS => {
                        if tok == XML_TOK_NMTOKEN_1 {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_17 XML_TOK_INVALID_1;
                        }
                        *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                        break 'iife_ret_17 XML_TOK_NAME_PLUS_1;
                    }
                    BT_AST => {
                        if tok == XML_TOK_NMTOKEN_1 {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_17 XML_TOK_INVALID_1;
                        }
                        *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                        break 'iife_ret_17 XML_TOK_NAME_ASTERISK_1;
                    }
                    BT_QUEST => {
                        if tok == XML_TOK_NMTOKEN_1 {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_17 XML_TOK_INVALID_1;
                        }
                        *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                        break 'iife_ret_17 XML_TOK_NAME_QUESTION_1;
                    }
                    _ => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_17 XML_TOK_INVALID_1;
                    }
                }
                if current_block_210 == 17210391895989911948 {
                    offset += 1;
                }
            }
            break 'iife_ret_17 -tok;
        };
        (tok, next_tok)
    }

    pub(crate) fn normal_attributeValueTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_18: {
            let mut offset = 0usize;
            let end = input.len();
            let start_offset = offset;
            if offset >= end {
                break 'iife_ret_18 XML_TOK_NONE_1;
            }
            while offset < end {
                match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                    BT_LEAD2 => {
                        offset += 2;
                    }
                    BT_LEAD3 => {
                        offset += 3;
                    }
                    BT_LEAD4 => {
                        offset += 4;
                    }
                    BT_AMP => {
                        if offset == start_offset {
                            break 'iife_ret_18 ({
                                let (tok_value, next_tok_value) =
                                    normal_scanRef(enc, &input[(offset + 1)..end]);
                                *nextTokPtr = next_tok_value;
                                tok_value
                            });
                        }
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_18 XML_TOK_DATA_CHARS_1;
                    }
                    BT_LT => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_18 XML_TOK_INVALID_1;
                    }
                    BT_LF => {
                        if offset == start_offset {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                            break 'iife_ret_18 XML_TOK_DATA_NEWLINE_1;
                        }
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_18 XML_TOK_DATA_CHARS_1;
                    }
                    BT_CR => {
                        if offset == start_offset {
                            offset += 1;
                            if end - offset < 1 {
                                break 'iife_ret_18 XML_TOK_TRAILING_CR_1;
                            }
                            if as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize]
                                as c_int
                                == BT_LF as c_int
                            {
                                offset += 1;
                            }
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_18 XML_TOK_DATA_NEWLINE_1;
                        }
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_18 XML_TOK_DATA_CHARS_1;
                    }
                    BT_S => {
                        if offset == start_offset {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                            break 'iife_ret_18 XML_TOK_ATTRIBUTE_VALUE_S_1;
                        }
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_18 XML_TOK_DATA_CHARS_1;
                    }
                    _ => {
                        offset += 1;
                    }
                }
            }
            *nextTokPtr = input.as_ptr().wrapping_add(offset);
            break 'iife_ret_18 XML_TOK_DATA_CHARS_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn normal_entityValueTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_19: {
            let mut offset = 0usize;
            let end = input.len();
            let start_offset = offset;
            if offset >= end {
                break 'iife_ret_19 XML_TOK_NONE_1;
            }
            while offset < end {
                match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                    BT_LEAD2 => {
                        offset += 2;
                    }
                    BT_LEAD3 => {
                        offset += 3;
                    }
                    BT_LEAD4 => {
                        offset += 4;
                    }
                    BT_AMP => {
                        if offset == start_offset {
                            break 'iife_ret_19 ({
                                let (tok_value, next_tok_value) =
                                    normal_scanRef(enc, &input[(offset + 1)..end]);
                                *nextTokPtr = next_tok_value;
                                tok_value
                            });
                        }
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_19 XML_TOK_DATA_CHARS_1;
                    }
                    BT_PERCNT => {
                        if offset == start_offset {
                            let mut tok: c_int = {
                                let (tok_value, next_tok_value) =
                                    normal_scanPercent(enc, &input[(offset + 1)..end]);
                                *nextTokPtr = next_tok_value;
                                tok_value
                            };
                            break 'iife_ret_19 if tok == XML_TOK_PERCENT_1 {
                                XML_TOK_INVALID_1
                            } else {
                                tok
                            };
                        }
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_19 XML_TOK_DATA_CHARS_1;
                    }
                    BT_LF => {
                        if offset == start_offset {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset + 1);
                            break 'iife_ret_19 XML_TOK_DATA_NEWLINE_1;
                        }
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_19 XML_TOK_DATA_CHARS_1;
                    }
                    BT_CR => {
                        if offset == start_offset {
                            offset += 1;
                            if end - offset < 1 {
                                break 'iife_ret_19 XML_TOK_TRAILING_CR_1;
                            }
                            if as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize]
                                as c_int
                                == BT_LF as c_int
                            {
                                offset += 1;
                            }
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_19 XML_TOK_DATA_NEWLINE_1;
                        }
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_19 XML_TOK_DATA_CHARS_1;
                    }
                    _ => {
                        offset += 1;
                    }
                }
            }
            *nextTokPtr = input.as_ptr().wrapping_add(offset);
            break 'iife_ret_19 XML_TOK_DATA_CHARS_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn normal_ignoreSectionTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_20: {
            let mut offset = 0usize;
            let end = input.len();
            let mut level: c_int = 0;
            while offset < end {
                match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                    BT_LEAD2 => {
                        if end - offset < 2 {
                            break 'iife_ret_20 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid2(enc, input[offset..].as_ptr()) != 0 {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_20 XML_TOK_INVALID_1;
                        }
                        offset += 2;
                    }
                    BT_LEAD3 => {
                        if end - offset < 3 {
                            break 'iife_ret_20 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid3(enc, input[offset..].as_ptr()) != 0 {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_20 XML_TOK_INVALID_1;
                        }
                        offset += 3;
                    }
                    BT_LEAD4 => {
                        if end - offset < 4 {
                            break 'iife_ret_20 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid4(enc, input[offset..].as_ptr()) != 0 {
                            *nextTokPtr = input.as_ptr().wrapping_add(offset);
                            break 'iife_ret_20 XML_TOK_INVALID_1;
                        }
                        offset += 4;
                    }
                    BT_NONXML | BT_MALFORM | BT_TRAIL => {
                        *nextTokPtr = input.as_ptr().wrapping_add(offset);
                        break 'iife_ret_20 XML_TOK_INVALID_1;
                    }
                    BT_LT => {
                        offset += 1;
                        if end - offset < 1 {
                            break 'iife_ret_20 XML_TOK_PARTIAL_1;
                        }
                        if input[offset] as c_int == 0x21 {
                            offset += 1;
                            if end - offset < 1 {
                                break 'iife_ret_20 XML_TOK_PARTIAL_1;
                            }
                            if input[offset] as c_int == 0x5b {
                                level += 1;
                                offset += 1;
                            }
                        }
                    }
                    BT_RSQB => {
                        offset += 1;
                        if end - offset < 1 {
                            break 'iife_ret_20 XML_TOK_PARTIAL_1;
                        }
                        if input[offset] as c_int == 0x5d {
                            offset += 1;
                            if end - offset < 1 {
                                break 'iife_ret_20 XML_TOK_PARTIAL_1;
                            }
                            if input[offset] as c_int == 0x3e {
                                offset += 1;
                                if level == 0 {
                                    *nextTokPtr = input.as_ptr().wrapping_add(offset);
                                    break 'iife_ret_20 XML_TOK_IGNORE_SECT_1;
                                }
                                level -= 1;
                            }
                        }
                    }
                    _ => {
                        offset += 1;
                    }
                }
            }
            break 'iife_ret_20 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn normal_isPublicId(enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult {
        let mut badPtrVal: *const c_char = null::<c_char>();
        let badPtr = &mut badPtrVal;
        let mut offset = 1usize;
        let end = input.len().saturating_sub(1);
        while offset < end {
            let mut current_block_8: u64;
            match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                25 | 24 | 27 | 13 | 31 | 32 | 34 | 35 | 17 | 14 | 15 | 9 | 10 | 18 | 16 | 33
                | 30 | 19 | 23 => {
                    current_block_8 = 5143058163439228106;
                }
                BT_S => {
                    if input[offset] as c_int == 0x9 {
                        *badPtr = input.as_ptr().wrapping_add(offset);
                        return (0i32, badPtrVal);
                    }
                    current_block_8 = 5143058163439228106;
                }
                BT_NAME | BT_NMSTRT => {
                    if input[offset] as c_int & !(0x7f) == 0 {
                        current_block_8 = 5143058163439228106;
                    } else {
                        current_block_8 = 10293610489198370764;
                    }
                }
                _ => {
                    current_block_8 = 10293610489198370764;
                }
            }
            if current_block_8 == 10293610489198370764 {
                match input[offset] as c_int {
                    36 | 64 => {}
                    _ => {
                        *badPtr = input.as_ptr().wrapping_add(offset);
                        return (0i32, badPtrVal);
                    }
                }
            }
            offset += 1;
        }
        (1, badPtrVal)
    }

    pub(crate) fn normal_getAtts(
        enc: &ENCODING,
        mut ptr: *const c_char,
        mut attsMax: c_int,
        mut atts: *mut ATTRIBUTE,
    ) -> c_int {
        let mut state: C2RustUnnamed_3 = inName;
        let mut nAtts: c_int = 0;
        let mut open: c_int = 0;
        ptr = ptr.wrapping_offset(1);
        loop {
            match as_normal_encoding(enc).type_0[read_c_char(ptr) as c_uchar as usize] as c_uint {
                BT_LEAD2 => {
                    if state == other {
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.name = ptr;
                            att.normalized = 1i8;
                        }
                        state = inName;
                    }
                    ptr = ptr.wrapping_offset((2i32 - 1i32) as isize);
                }
                BT_LEAD3 => {
                    if state == other {
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.name = ptr;
                            att.normalized = 1i8;
                        }
                        state = inName;
                    }
                    ptr = ptr.wrapping_offset((3i32 - 1i32) as isize);
                }
                BT_LEAD4 => {
                    if state == other {
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.name = ptr;
                            att.normalized = 1i8;
                        }
                        state = inName;
                    }
                    ptr = ptr.wrapping_offset((4i32 - 1i32) as isize);
                }
                BT_NONASCII | BT_NMSTRT | BT_HEX => {
                    if state == other {
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.name = ptr;
                            att.normalized = 1i8;
                        }
                        state = inName;
                    }
                }
                BT_QUOT => {
                    if state != inValue {
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.valuePtr = ptr.wrapping_offset(1);
                        }
                        state = inValue;
                        open = BT_QUOT as c_int;
                    } else if open == BT_QUOT as c_int {
                        state = other;
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.valueEnd = ptr;
                        }
                        nAtts += 1;
                    }
                }
                BT_APOS => {
                    if state != inValue {
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.valuePtr = ptr.wrapping_offset(1);
                        }
                        state = inValue;
                        open = BT_APOS as c_int;
                    } else if open == BT_APOS as c_int {
                        state = other;
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.valueEnd = ptr;
                        }
                        nAtts += 1;
                    }
                }
                BT_AMP => {
                    if nAtts < attsMax {
                        attribute_mut(atts, nAtts).normalized = 0i8;
                    }
                }
                BT_S => {
                    if state == inName {
                        state = other;
                    } else if state == inValue && nAtts < attsMax {
                        let att = attribute_mut(atts, nAtts);
                        if att.normalized as c_int != 0
                            && (ptr == att.valuePtr
                                || read_c_char(ptr) as c_int != ASCII_SPACE
                                || read_c_char_at(ptr, 1) as c_int == ASCII_SPACE
                                || as_normal_encoding(enc).type_0
                                    [read_c_char_at(ptr, 1) as c_uchar as usize]
                                    as c_int
                                    == open)
                        {
                            att.normalized = 0i8;
                        }
                    }
                }
                BT_CR | BT_LF => {
                    if state == inName {
                        state = other;
                    } else if state == inValue && nAtts < attsMax {
                        attribute_mut(atts, nAtts).normalized = 0i8;
                    }
                }
                BT_GT | BT_SOL => {
                    if state != inValue {
                        return nAtts;
                    }
                }
                _ => {}
            }
            ptr = ptr.wrapping_offset(1);
        }
    }

    pub(crate) fn normal_charRefNumber(_enc: &ENCODING, mut ptr: *const c_char) -> c_int {
        let mut result: c_int = 0;
        ptr = ptr.wrapping_offset(2);
        if read_c_char(ptr) as c_int == 0x78 {
            ptr = ptr.wrapping_offset(1);
            while read_c_char(ptr) as c_int != 0x3b {
                let c: c_int = read_c_char(ptr) as c_int;
                match c {
                    ASCII_0 | ASCII_1_1 | ASCII_2_1 | ASCII_3_1 | ASCII_4 | ASCII_5 | ASCII_6
                    | ASCII_7 | ASCII_8_1 | ASCII_9_1 => {
                        result <<= 4;
                        result |= c - ASCII_0;
                    }
                    ASCII_A | ASCII_B_1 | ASCII_C | ASCII_D | ASCII_E_1 | ASCII_F_1 => {
                        result <<= 4;
                        result += 10i32 + (c - ASCII_A);
                    }
                    ASCII_a_1 | ASCII_b | ASCII_c_1 | ASCII_d | ASCII_e_1 | ASCII_f => {
                        result <<= 4;
                        result += 10i32 + (c - ASCII_a_1);
                    }
                    _ => {}
                }
                if result >= 0x110000 {
                    return -(1i32);
                }
                ptr = ptr.wrapping_offset(1);
            }
        } else {
            while read_c_char(ptr) as c_int != 0x3b {
                let c_0: c_int = read_c_char(ptr) as c_int;
                result *= 10;
                result += c_0 - ASCII_0;
                if result >= 0x110000 {
                    return -(1i32);
                }
                ptr = ptr.wrapping_offset(1);
            }
        }
        checkCharRefNumber(result)
    }

    pub(crate) fn normal_predefinedEntityName(_enc: &ENCODING, input: &[c_char]) -> c_int {
        let mut offset = 0usize;
        match input.len() as c_long {
            2 => {
                if input[offset + 1] as c_int == 0x74 {
                    match input[offset] as c_int {
                        ASCII_l_1 => return ASCII_LT,
                        ASCII_g_1 => return ASCII_GT,
                        _ => {}
                    }
                }
            }
            3 => {
                if input[offset] as c_int == 0x61 {
                    offset += 1;
                    if input[offset] as c_int == 0x6d {
                        offset += 1;
                        if input[offset] as c_int == 0x70 {
                            return ASCII_AMP;
                        }
                    }
                }
            }
            4 => match input[offset] as c_int {
                ASCII_q => {
                    offset += 1;
                    if input[offset] as c_int == 0x75 {
                        offset += 1;
                        if input[offset] as c_int == 0x6f {
                            offset += 1;
                            if input[offset] as c_int == 0x74 {
                                return ASCII_QUOT;
                            }
                        }
                    }
                }
                ASCII_a_1 => {
                    offset += 1;
                    if input[offset] as c_int == 0x70 {
                        offset += 1;
                        if input[offset] as c_int == 0x6f {
                            offset += 1;
                            if input[offset] as c_int == 0x73 {
                                return ASCII_APOS;
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
        0
    }

    pub(crate) fn normal_nameMatchesAscii(
        _enc: &ENCODING,
        input: &[c_char],
        mut ptr2: *const c_char,
    ) -> c_int {
        let mut ptr1 = 0usize;
        let end1 = input.len();
        while read_c_char(ptr2) != 0 {
            if end1.wrapping_sub(ptr1) < 1 {
                return 0i32;
            }
            if input[ptr1] as c_int != read_c_char(ptr2) as c_int {
                return 0i32;
            }
            ptr1 = ptr1.wrapping_offset(1);
            ptr2 = ptr2.wrapping_offset(1);
        }
        (ptr1 == end1) as c_int
    }

    pub(crate) fn normal_nameLength(enc: &ENCODING, mut ptr: *const c_char) -> c_int {
        let start: *const c_char = ptr;
        loop {
            match as_normal_encoding(enc).type_0[read_c_uchar_at(ptr, 0) as usize] as c_uint {
                BT_LEAD2 => {
                    ptr = ptr.wrapping_offset(2);
                }
                BT_LEAD3 => {
                    ptr = ptr.wrapping_offset(3);
                }
                BT_LEAD4 => {
                    ptr = ptr.wrapping_offset(4);
                }
                BT_NONASCII | BT_NMSTRT | BT_COLON_0 | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                    ptr = ptr.wrapping_offset(1);
                }
                _ => {
                    return c_char_ptr_diff(ptr, start) as c_int;
                }
            }
        }
    }

    pub(crate) fn normal_skipS(enc: &ENCODING, mut ptr: *const c_char) -> *const c_char {
        loop {
            match as_normal_encoding(enc).type_0[read_c_uchar_at(ptr, 0) as usize] as c_uint {
                BT_LF | BT_CR | BT_S => {
                    ptr = ptr.wrapping_offset(1);
                }
                _ => return ptr,
            }
        }
    }

    pub(crate) fn normal_updatePosition(enc: &ENCODING, input: &[c_char], mut pos: *mut POSITION) {
        let pos = mut_ref_from_ptr(pos);
        let mut offset = 0usize;
        let end = input.len();
        while offset < end {
            match as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize] as c_uint {
                BT_LEAD2 => {
                    offset += 2;
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                BT_LEAD3 => {
                    offset += 3;
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                BT_LEAD4 => {
                    offset += 4;
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                BT_LF => {
                    pos.columnNumber = 0u64;
                    pos.lineNumber = pos.lineNumber.wrapping_add(1);
                    offset += 1;
                }
                BT_CR => {
                    pos.lineNumber = pos.lineNumber.wrapping_add(1);
                    offset += 1;
                    if offset < end
                        && as_normal_encoding(enc).type_0[input[offset] as c_uchar as usize]
                            as c_int
                            == BT_LF as c_int
                    {
                        offset += 1;
                    }
                    pos.columnNumber = 0u64;
                }
                _ => {
                    offset += 1;
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
            }
        }
    }

    pub(crate) fn little2_scanComment(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_21: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr = |ptr: usize| input[ptr];
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            if c_char_ptr_diff(end, ptr) >= 2 as c_long {
                if !(read_ptr_at(ptr, 1) as c_int == 0 && read_ptr_at(ptr, 0) as c_int == 0x2d) {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_21 XML_TOK_INVALID_1;
                }
                ptr = ptr.wrapping_offset(2);
                while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                    match if read_ptr_at(ptr, 1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                    } {
                        BT_LEAD2 => {
                            if (c_char_ptr_diff(end, ptr)) < 2 {
                                break 'iife_ret_21 XML_TOK_PARTIAL_CHAR_1;
                            }
                            ptr = ptr.wrapping_offset(2isize);
                        }
                        BT_LEAD3 => {
                            if (c_char_ptr_diff(end, ptr)) < 3 {
                                break 'iife_ret_21 XML_TOK_PARTIAL_CHAR_1;
                            }
                            ptr = ptr.wrapping_offset(3isize);
                        }
                        BT_LEAD4 => {
                            if (c_char_ptr_diff(end, ptr)) < 4 {
                                break 'iife_ret_21 XML_TOK_PARTIAL_CHAR_1;
                            }
                            ptr = ptr.wrapping_offset(4isize);
                        }
                        BT_NONXML | BT_MALFORM | BT_TRAIL => {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_21 XML_TOK_INVALID_1;
                        }
                        BT_MINUS => {
                            ptr = ptr.wrapping_offset(2);
                            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                break 'iife_ret_21 XML_TOK_PARTIAL_1;
                            }
                            if read_ptr_at(ptr, 1) as c_int == 0
                                && read_ptr_at(ptr, 0) as c_int == 0x2d
                            {
                                ptr = ptr.wrapping_offset(2);
                                if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                    break 'iife_ret_21 XML_TOK_PARTIAL_1;
                                }
                                if !(read_ptr_at(ptr, 1) as c_int == 0
                                    && read_ptr_at(ptr, 0) as c_int == 0x3e)
                                {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_21 XML_TOK_INVALID_1;
                                }
                                *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                                break 'iife_ret_21 XML_TOK_COMMENT_1;
                            }
                        }
                        _ => {
                            ptr = ptr.wrapping_offset(2isize);
                        }
                    }
                }
            }
            break 'iife_ret_21 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn little2_scanDecl(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_22: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr = |ptr: usize| input[ptr];
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let c_char_slice_from_ptr_end = |ptr: usize, end: usize| &input[ptr..end];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                break 'iife_ret_22 XML_TOK_PARTIAL_1;
            }
            match if read_ptr_at(ptr, 1) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
            } {
                BT_MINUS => {
                    break 'iife_ret_22 ({
                        let (tok_value, next_tok_value) = little2_scanComment(
                            enc,
                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                        );
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_LSQB => {
                    *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                    break 'iife_ret_22 XML_TOK_COND_SECT_OPEN_1;
                }
                BT_NMSTRT | BT_HEX => {
                    ptr = ptr.wrapping_offset(2isize);
                }
                _ => {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_22 XML_TOK_INVALID_1;
                }
            }
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                's_129: {
                    match if read_ptr_at(ptr, 1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                    } {
                        BT_PERCNT => {
                            if (c_char_ptr_diff(end, ptr)) < (2i32 * 2) as c_long {
                                break 'iife_ret_22 XML_TOK_PARTIAL_1;
                            }
                            match if read_ptr_at(ptr.wrapping_offset(2), 1) as c_int == 0 {
                                as_normal_encoding(enc).type_0
                                    [read_ptr_at(ptr, 2) as c_uchar as usize]
                                    as c_uint
                            } else {
                                unicode_byte_type(
                                    read_ptr_at(ptr.wrapping_offset(2), 1),
                                    read_ptr_at(ptr.wrapping_offset(2), 0),
                                ) as c_uint
                            } {
                                BT_S | BT_CR | BT_LF | BT_PERCNT => {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_22 XML_TOK_INVALID_1;
                                }
                                _ => {}
                            }
                        }
                        BT_S | BT_CR | BT_LF => {}
                        BT_NMSTRT | BT_HEX => {
                            ptr = ptr.wrapping_offset(2);
                            break 's_129;
                        }
                        _ => {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_22 XML_TOK_INVALID_1;
                        }
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_22 XML_TOK_DECL_OPEN_1;
                }
            }
            break 'iife_ret_22 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn little2_checkPiTarget(_enc: &ENCODING, input: &[c_char]) -> CheckPiTargetResult {
        let mut tok: c_int = 0;
        let tokPtr = &mut tok;
        let result: c_int = 'iife_ret_23: {
            let mut ptr = 0usize;
            let end = input.len();
            let mut upper: c_int = 0;
            *tokPtr = XML_TOK_PI_1;
            if end.wrapping_sub(ptr) != 2 * 3 {
                break 'iife_ret_23 1i32;
            }
            match if input[ptr.wrapping_offset(1)] as c_int == 0 {
                input[ptr.wrapping_offset(0)] as c_int
            } else {
                -(1)
            } {
                ASCII_x_1 => {}
                ASCII_X_1 => {
                    upper = 1i32;
                }
                _ => break 'iife_ret_23 1,
            }
            ptr = ptr.wrapping_offset(2);
            match if input[ptr.wrapping_offset(1)] as c_int == 0 {
                input[ptr.wrapping_offset(0)] as c_int
            } else {
                -(1)
            } {
                ASCII_m_1 => {}
                ASCII_M_1 => {
                    upper = 1i32;
                }
                _ => break 'iife_ret_23 1,
            }
            ptr = ptr.wrapping_offset(2);
            match if input[ptr.wrapping_offset(1)] as c_int == 0 {
                input[ptr.wrapping_offset(0)] as c_int
            } else {
                -(1)
            } {
                ASCII_l_1 => {}
                ASCII_L_1 => {
                    upper = 1i32;
                }
                _ => break 'iife_ret_23 1,
            }
            if upper != 0 {
                break 'iife_ret_23 0i32;
            }
            *tokPtr = XML_TOK_XML_DECL_1;
            break 'iife_ret_23 1;
        };
        (result, tok)
    }

    pub(crate) fn little2_scanPi(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_24: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr = |ptr: usize| input[ptr];
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let c_char_slice_from_ptr_end = |ptr: usize, end: usize| &input[ptr..end];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            let mut tok: c_int = 0;
            let mut target: usize = ptr;
            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                break 'iife_ret_24 XML_TOK_PARTIAL_1;
            }
            let mut current_block_32: u64;
            match if read_ptr_at(ptr, 1) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
            } {
                BT_NONASCII => {
                    if namingBitmap[(((nmstrtPages[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_int)
                        << 3)
                        + (read_ptr_at(ptr, 0) as c_uchar as c_int >> 5))
                        as usize]
                        & (1) << (read_ptr_at(ptr, 0) as c_uchar as c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_24 XML_TOK_INVALID_1;
                    }
                    current_block_32 = 14358794669692889688;
                }
                BT_NMSTRT | BT_HEX => {
                    current_block_32 = 14358794669692889688;
                }
                BT_LEAD2 => {
                    if (c_char_ptr_diff(end, ptr)) < 2 {
                        break 'iife_ret_24 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_24 XML_TOK_INVALID_1;
                }
                BT_LEAD3 => {
                    if (c_char_ptr_diff(end, ptr)) < 3 {
                        break 'iife_ret_24 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_24 XML_TOK_INVALID_1;
                }
                BT_LEAD4 => {
                    if (c_char_ptr_diff(end, ptr)) < 4 {
                        break 'iife_ret_24 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_24 XML_TOK_INVALID_1;
                }
                _ => {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_24 XML_TOK_INVALID_1;
                }
            }
            if current_block_32 == 14358794669692889688 {
                ptr = ptr.wrapping_offset(2isize);
            }
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                let mut current_block_118: u64;
                match if read_ptr_at(ptr, 1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((namePages[read_ptr_at(ptr, 1) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (read_ptr_at(ptr, 0) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (read_ptr_at(ptr, 0) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_24 XML_TOK_INVALID_1;
                        }
                        current_block_118 = 15890151712677504458;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_118 = 15890151712677504458;
                    }
                    BT_LEAD2 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            break 'iife_ret_24 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_24 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            break 'iife_ret_24 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_24 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            break 'iife_ret_24 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_24 XML_TOK_INVALID_1;
                    }
                    BT_S | BT_CR | BT_LF => {
                        if {
                            let (ok_value, tok_value) =
                                little2_checkPiTarget(enc, c_char_slice_from_ptr_end(target, ptr));
                            tok = tok_value;
                            ok_value
                        } == 0
                        {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_24 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.wrapping_offset(2);
                        while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                            match if read_ptr_at(ptr, 1) as c_int == 0 {
                                as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize]
                                    as c_uint
                            } else {
                                unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0))
                                    as c_uint
                            } {
                                BT_LEAD2 => {
                                    if (c_char_ptr_diff(end, ptr)) < 2 {
                                        break 'iife_ret_24 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    ptr = ptr.wrapping_offset(2isize);
                                }
                                BT_LEAD3 => {
                                    if (c_char_ptr_diff(end, ptr)) < 3 {
                                        break 'iife_ret_24 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    ptr = ptr.wrapping_offset(3isize);
                                }
                                BT_LEAD4 => {
                                    if (c_char_ptr_diff(end, ptr)) < 4 {
                                        break 'iife_ret_24 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    ptr = ptr.wrapping_offset(4isize);
                                }
                                BT_NONXML | BT_MALFORM | BT_TRAIL => {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_24 XML_TOK_INVALID_1;
                                }
                                BT_QUEST => {
                                    ptr = ptr.wrapping_offset(2);
                                    if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                        break 'iife_ret_24 XML_TOK_PARTIAL_1;
                                    }
                                    if read_ptr_at(ptr, 1) as c_int == 0
                                        && read_ptr_at(ptr, 0) as c_int == 0x3e
                                    {
                                        *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                                        break 'iife_ret_24 tok;
                                    }
                                }
                                _ => {
                                    ptr = ptr.wrapping_offset(2isize);
                                }
                            }
                        }
                        break 'iife_ret_24 XML_TOK_PARTIAL_1;
                    }
                    BT_QUEST => {
                        if {
                            let (ok_value, tok_value) =
                                little2_checkPiTarget(enc, c_char_slice_from_ptr_end(target, ptr));
                            tok = tok_value;
                            ok_value
                        } == 0
                        {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_24 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.wrapping_offset(2);
                        if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                            break 'iife_ret_24 XML_TOK_PARTIAL_1;
                        }
                        if read_ptr_at(ptr, 1) as c_int == 0 && read_ptr_at(ptr, 0) as c_int == 0x3e
                        {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                            break 'iife_ret_24 tok;
                        }
                        current_block_118 = 7312756018063861309;
                    }
                    _ => {
                        current_block_118 = 7312756018063861309;
                    }
                }
                match current_block_118 {
                    7312756018063861309 => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_24 XML_TOK_INVALID_1;
                    }
                    15890151712677504458 => {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                    _ => {}
                }
            }
            break 'iife_ret_24 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn little2_scanCdataSection(_enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_25: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            pub static CDATA_LSQB: [c_char; 6] = [
                ASCII_C as c_char,
                ASCII_D as c_char,
                ASCII_A as c_char,
                ASCII_T as c_char,
                ASCII_A as c_char,
                ASCII_LSQB as c_char,
            ];
            let mut i: c_int = 0;
            if (c_char_ptr_diff(end, ptr)) < (6i32 * 2) as c_long {
                break 'iife_ret_25 XML_TOK_PARTIAL_1;
            }
            i = 0;
            while i < 6 {
                if !(read_ptr_at(ptr, 1) as c_int == 0
                    && read_ptr_at(ptr, 0) as c_int == CDATA_LSQB[i as usize] as c_int)
                {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_25 XML_TOK_INVALID_1;
                }
                i += 1;
                ptr = ptr.wrapping_offset(2);
            }
            *nextTokPtr = ptr_to(ptr);
            break 'iife_ret_25 XML_TOK_CDATA_SECT_OPEN_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn little2_cdataSectionTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_26: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr = |ptr: usize| input[ptr];
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            if ptr >= end {
                break 'iife_ret_26 XML_TOK_NONE_1;
            }
            {
                let mut n: size_t = c_char_ptr_diff(end, ptr) as size_t;
                if n & (2i32 - 1) as size_t != 0 {
                    n &= !(2i32 - 1) as size_t;
                    if n == 0 {
                        break 'iife_ret_26 XML_TOK_PARTIAL_1;
                    }
                    end = ptr.wrapping_add(n);
                }
            }
            match if read_ptr_at(ptr, 1) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
            } {
                BT_RSQB => {
                    ptr = ptr.wrapping_offset(2);
                    if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                        break 'iife_ret_26 XML_TOK_PARTIAL_1;
                    }
                    if read_ptr_at(ptr, 1) as c_int == 0 && read_ptr_at(ptr, 0) as c_int == 0x5d {
                        ptr = ptr.wrapping_offset(2);
                        if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                            break 'iife_ret_26 XML_TOK_PARTIAL_1;
                        }
                        if !(read_ptr_at(ptr, 1) as c_int == 0
                            && read_ptr_at(ptr, 0) as c_int == 0x3e)
                        {
                            ptr = ptr.wrapping_offset(-(2isize));
                        } else {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                            break 'iife_ret_26 XML_TOK_CDATA_SECT_CLOSE_1;
                        }
                    }
                }
                BT_CR => {
                    ptr = ptr.wrapping_offset(2);
                    if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                        break 'iife_ret_26 XML_TOK_PARTIAL_1;
                    }
                    if (if read_ptr_at(ptr, 1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_int
                    } else {
                        unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0))
                    }) == BT_LF as c_int
                    {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_26 XML_TOK_DATA_NEWLINE_1;
                }
                BT_LF => {
                    *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                    break 'iife_ret_26 XML_TOK_DATA_NEWLINE_1;
                }
                BT_LEAD2 => {
                    if (c_char_ptr_diff(end, ptr)) < 2 {
                        break 'iife_ret_26 XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.wrapping_offset(2isize);
                }
                BT_LEAD3 => {
                    if (c_char_ptr_diff(end, ptr)) < 3 {
                        break 'iife_ret_26 XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.wrapping_offset(3isize);
                }
                BT_LEAD4 => {
                    if (c_char_ptr_diff(end, ptr)) < 4 {
                        break 'iife_ret_26 XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.wrapping_offset(4isize);
                }
                BT_NONXML | BT_MALFORM | BT_TRAIL => {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_26 XML_TOK_INVALID_1;
                }
                _ => {
                    ptr = ptr.wrapping_offset(2isize);
                }
            }
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                match if read_ptr_at(ptr, 1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                } {
                    BT_LEAD2 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_26 XML_TOK_DATA_CHARS_1;
                        }
                        ptr = ptr.wrapping_offset(2isize);
                    }
                    BT_LEAD3 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_26 XML_TOK_DATA_CHARS_1;
                        }
                        ptr = ptr.wrapping_offset(3isize);
                    }
                    BT_LEAD4 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_26 XML_TOK_DATA_CHARS_1;
                        }
                        ptr = ptr.wrapping_offset(4isize);
                    }
                    BT_NONXML | BT_MALFORM | BT_TRAIL | BT_CR | BT_LF | BT_RSQB => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_26 XML_TOK_DATA_CHARS_1;
                    }
                    _ => {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                }
            }
            *nextTokPtr = ptr_to(ptr);
            break 'iife_ret_26 XML_TOK_DATA_CHARS_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn little2_scanEndTag(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_27: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr = |ptr: usize| input[ptr];
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                break 'iife_ret_27 XML_TOK_PARTIAL_1;
            }
            let mut current_block_32: u64;
            match if read_ptr_at(ptr, 1) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
            } {
                BT_NONASCII => {
                    if namingBitmap[(((nmstrtPages[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_int)
                        << 3)
                        + (read_ptr_at(ptr, 0) as c_uchar as c_int >> 5))
                        as usize]
                        & (1) << (read_ptr_at(ptr, 0) as c_uchar as c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_27 XML_TOK_INVALID_1;
                    }
                    current_block_32 = 8654814784450400207;
                }
                BT_NMSTRT | BT_HEX => {
                    current_block_32 = 8654814784450400207;
                }
                BT_LEAD2 => {
                    if (c_char_ptr_diff(end, ptr)) < 2 {
                        break 'iife_ret_27 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_27 XML_TOK_INVALID_1;
                }
                BT_LEAD3 => {
                    if (c_char_ptr_diff(end, ptr)) < 3 {
                        break 'iife_ret_27 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_27 XML_TOK_INVALID_1;
                }
                BT_LEAD4 => {
                    if (c_char_ptr_diff(end, ptr)) < 4 {
                        break 'iife_ret_27 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_27 XML_TOK_INVALID_1;
                }
                _ => {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_27 XML_TOK_INVALID_1;
                }
            }
            if current_block_32 == 8654814784450400207 {
                ptr = ptr.wrapping_offset(2isize);
            }
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                let mut current_block_73: u64;
                match if read_ptr_at(ptr, 1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((namePages[read_ptr_at(ptr, 1) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (read_ptr_at(ptr, 0) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (read_ptr_at(ptr, 0) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_27 XML_TOK_INVALID_1;
                        }
                        current_block_73 = 16411184819389759620;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_73 = 16411184819389759620;
                    }
                    BT_LEAD2 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            break 'iife_ret_27 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_27 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            break 'iife_ret_27 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_27 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            break 'iife_ret_27 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_27 XML_TOK_INVALID_1;
                    }
                    BT_S | BT_CR | BT_LF => {
                        ptr = ptr.wrapping_offset(2);
                        while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                            match if read_ptr_at(ptr, 1) as c_int == 0 {
                                as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize]
                                    as c_uint
                            } else {
                                unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0))
                                    as c_uint
                            } {
                                BT_S | BT_CR | BT_LF => {}
                                BT_GT => {
                                    *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                                    break 'iife_ret_27 XML_TOK_END_TAG_1;
                                }
                                _ => {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_27 XML_TOK_INVALID_1;
                                }
                            }
                            ptr = ptr.wrapping_offset(2);
                        }
                        break 'iife_ret_27 XML_TOK_PARTIAL_1;
                    }
                    BT_COLON_0 => {
                        ptr = ptr.wrapping_offset(2);
                        current_block_73 = 981995395831942902;
                    }
                    BT_GT => {
                        *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                        break 'iife_ret_27 XML_TOK_END_TAG_1;
                    }
                    _ => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_27 XML_TOK_INVALID_1;
                    }
                }
                if current_block_73 == 16411184819389759620 {
                    ptr = ptr.wrapping_offset(2isize);
                }
            }
            break 'iife_ret_27 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn little2_scanHexCharRef(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_28: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr = |ptr: usize| input[ptr];
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            if c_char_ptr_diff(end, ptr) >= 2 as c_long {
                match if read_ptr_at(ptr, 1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                } {
                    BT_DIGIT | BT_HEX => {}
                    _ => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_28 XML_TOK_INVALID_1;
                    }
                }
                ptr = ptr.wrapping_offset(2);
                while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                    match if read_ptr_at(ptr, 1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                    } {
                        BT_DIGIT | BT_HEX => {}
                        BT_SEMI => {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                            break 'iife_ret_28 XML_TOK_CHAR_REF_1;
                        }
                        _ => {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_28 XML_TOK_INVALID_1;
                        }
                    }
                    ptr = ptr.wrapping_offset(2);
                }
            }
            break 'iife_ret_28 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn little2_scanCharRef(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_29: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr = |ptr: usize| input[ptr];
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let c_char_slice_from_ptr_end = |ptr: usize, end: usize| &input[ptr..end];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            if c_char_ptr_diff(end, ptr) >= 2 as c_long {
                if read_ptr_at(ptr, 1) as c_int == 0 && read_ptr_at(ptr, 0) as c_int == 0x78 {
                    break 'iife_ret_29 ({
                        let (tok_value, next_tok_value) = little2_scanHexCharRef(
                            enc,
                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                        );
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                match if read_ptr_at(ptr, 1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                } {
                    BT_DIGIT => {}
                    _ => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_29 XML_TOK_INVALID_1;
                    }
                }
                ptr = ptr.wrapping_offset(2);
                while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                    match if read_ptr_at(ptr, 1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                    } {
                        BT_DIGIT => {}
                        BT_SEMI => {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                            break 'iife_ret_29 XML_TOK_CHAR_REF_1;
                        }
                        _ => {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_29 XML_TOK_INVALID_1;
                        }
                    }
                    ptr = ptr.wrapping_offset(2);
                }
            }
            break 'iife_ret_29 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn little2_scanRef(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_30: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr = |ptr: usize| input[ptr];
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let c_char_slice_from_ptr_end = |ptr: usize, end: usize| &input[ptr..end];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                break 'iife_ret_30 XML_TOK_PARTIAL_1;
            }
            let mut current_block_33: u64;
            match if read_ptr_at(ptr, 1) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
            } {
                BT_NONASCII => {
                    if namingBitmap[(((nmstrtPages[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_int)
                        << 3)
                        + (read_ptr_at(ptr, 0) as c_uchar as c_int >> 5))
                        as usize]
                        & (1) << (read_ptr_at(ptr, 0) as c_uchar as c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_30 XML_TOK_INVALID_1;
                    }
                    current_block_33 = 6679362556518655255;
                }
                BT_NMSTRT | BT_HEX => {
                    current_block_33 = 6679362556518655255;
                }
                BT_LEAD2 => {
                    if (c_char_ptr_diff(end, ptr)) < 2 {
                        break 'iife_ret_30 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_30 XML_TOK_INVALID_1;
                }
                BT_LEAD3 => {
                    if (c_char_ptr_diff(end, ptr)) < 3 {
                        break 'iife_ret_30 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_30 XML_TOK_INVALID_1;
                }
                BT_LEAD4 => {
                    if (c_char_ptr_diff(end, ptr)) < 4 {
                        break 'iife_ret_30 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_30 XML_TOK_INVALID_1;
                }
                BT_NUM => {
                    break 'iife_ret_30 ({
                        let (tok_value, next_tok_value) = little2_scanCharRef(
                            enc,
                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                        );
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                _ => {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_30 XML_TOK_INVALID_1;
                }
            }
            if current_block_33 == 6679362556518655255 {
                ptr = ptr.wrapping_offset(2isize);
            }
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                let mut current_block_64: u64;
                match if read_ptr_at(ptr, 1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((namePages[read_ptr_at(ptr, 1) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (read_ptr_at(ptr, 0) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (read_ptr_at(ptr, 0) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_30 XML_TOK_INVALID_1;
                        }
                        current_block_64 = 405996089697802199;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_64 = 405996089697802199;
                    }
                    BT_LEAD2 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            break 'iife_ret_30 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_30 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            break 'iife_ret_30 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_30 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            break 'iife_ret_30 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_30 XML_TOK_INVALID_1;
                    }
                    BT_SEMI => {
                        *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                        break 'iife_ret_30 XML_TOK_ENTITY_REF_1;
                    }
                    _ => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_30 XML_TOK_INVALID_1;
                    }
                }
                if current_block_64 == 405996089697802199 {
                    ptr = ptr.wrapping_offset(2isize);
                }
            }
            break 'iife_ret_30 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn little2_scanAtts(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_31: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr = |ptr: usize| input[ptr];
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let c_char_slice_from_ptr_end = |ptr: usize, end: usize| &input[ptr..end];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            let mut hadColon: c_int = 0;
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                let mut current_block_186: u64;
                match if read_ptr_at(ptr, 1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((namePages[read_ptr_at(ptr, 1) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (read_ptr_at(ptr, 0) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (read_ptr_at(ptr, 0) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_31 XML_TOK_INVALID_1;
                        }
                        current_block_186 = 17747718632989559416;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_186 = 17747718632989559416;
                    }
                    BT_LEAD2 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_31 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_31 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_31 XML_TOK_INVALID_1;
                    }
                    BT_COLON_0 => {
                        if hadColon != 0 {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_31 XML_TOK_INVALID_1;
                        }
                        hadColon = 1;
                        ptr = ptr.wrapping_offset(2);
                        if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                            break 'iife_ret_31 XML_TOK_PARTIAL_1;
                        }
                        let mut current_block_64: u64;
                        match if read_ptr_at(ptr, 1) as c_int == 0 {
                            as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize]
                                as c_uint
                        } else {
                            unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                        } {
                            BT_NONASCII => {
                                if namingBitmap[(((nmstrtPages
                                    [read_ptr_at(ptr, 1) as c_uchar as usize]
                                    as c_int)
                                    << 3)
                                    + (read_ptr_at(ptr, 0) as c_uchar as c_int >> 5))
                                    as usize]
                                    & (1) << (read_ptr_at(ptr, 0) as c_uchar as c_int & 0x1f)
                                    == 0
                                {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_31 XML_TOK_INVALID_1;
                                }
                                current_block_64 = 12531724302225488581;
                            }
                            BT_NMSTRT | BT_HEX => {
                                current_block_64 = 12531724302225488581;
                            }
                            BT_LEAD2 => {
                                if (c_char_ptr_diff(end, ptr)) < 2 {
                                    break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                                }
                                *nextTokPtr = ptr_to(ptr);
                                break 'iife_ret_31 XML_TOK_INVALID_1;
                            }
                            BT_LEAD3 => {
                                if (c_char_ptr_diff(end, ptr)) < 3 {
                                    break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                                }
                                *nextTokPtr = ptr_to(ptr);
                                break 'iife_ret_31 XML_TOK_INVALID_1;
                            }
                            BT_LEAD4 => {
                                if (c_char_ptr_diff(end, ptr)) < 4 {
                                    break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                                }
                                *nextTokPtr = ptr_to(ptr);
                                break 'iife_ret_31 XML_TOK_INVALID_1;
                            }
                            _ => {
                                *nextTokPtr = ptr_to(ptr);
                                break 'iife_ret_31 XML_TOK_INVALID_1;
                            }
                        }
                        if current_block_64 == 12531724302225488581 {
                            ptr = ptr.wrapping_offset(2isize);
                        }
                        current_block_186 = 1634947208139838470;
                    }
                    BT_S | BT_CR | BT_LF => {
                        loop {
                            let mut t: c_int = 0;
                            ptr = ptr.wrapping_offset(2);
                            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                break 'iife_ret_31 XML_TOK_PARTIAL_1;
                            }
                            t = if read_ptr_at(ptr, 1) as c_int == 0 {
                                as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize]
                                    as c_int
                            } else {
                                unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0))
                            };
                            if t == BT_EQUALS as c_int {
                                break;
                            }
                            match t {
                                21 | 10 | 9 => {}
                                _ => {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_31 XML_TOK_INVALID_1;
                                }
                            }
                        }
                        current_block_186 = 10853015579903106591;
                    }
                    BT_EQUALS => {
                        current_block_186 = 10853015579903106591;
                    }
                    _ => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_31 XML_TOK_INVALID_1;
                    }
                }
                match current_block_186 {
                    10853015579903106591 => {
                        let mut open: c_int = 0;
                        hadColon = 0;
                        loop {
                            ptr = ptr.wrapping_offset(2);
                            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                break 'iife_ret_31 XML_TOK_PARTIAL_1;
                            }
                            open = if read_ptr_at(ptr, 1) as c_int == 0 {
                                as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize]
                                    as c_int
                            } else {
                                unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0))
                            };
                            if open == BT_QUOT as c_int || open == BT_APOS as c_int {
                                break;
                            }
                            match open {
                                21 | 10 | 9 => {}
                                _ => {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_31 XML_TOK_INVALID_1;
                                }
                            }
                        }
                        ptr = ptr.wrapping_offset(2);
                        loop {
                            let mut t_0: c_int = 0;
                            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                break 'iife_ret_31 XML_TOK_PARTIAL_1;
                            }
                            t_0 = if read_ptr_at(ptr, 1) as c_int == 0 {
                                as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize]
                                    as c_int
                            } else {
                                unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0))
                            };
                            if t_0 == open {
                                break;
                            }
                            match t_0 {
                                5 => {
                                    if (c_char_ptr_diff(end, ptr)) < 2 {
                                        break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    ptr = ptr.wrapping_offset(2isize);
                                }
                                6 => {
                                    if (c_char_ptr_diff(end, ptr)) < 3 {
                                        break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    ptr = ptr.wrapping_offset(3isize);
                                }
                                7 => {
                                    if (c_char_ptr_diff(end, ptr)) < 4 {
                                        break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    ptr = ptr.wrapping_offset(4isize);
                                }
                                0 | 1 | 8 => {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_31 XML_TOK_INVALID_1;
                                }
                                3 => {
                                    let mut tok: c_int = {
                                        let (tok_value, next_tok_value) = little2_scanRef(
                                            enc,
                                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2), end),
                                        );
                                        ptr = super::c_char_ptr_diff(next_tok_value, input.as_ptr())
                                            as usize;
                                        tok_value
                                    };
                                    if tok <= 0 {
                                        if tok == XML_TOK_INVALID_1 {
                                            *nextTokPtr = ptr_to(ptr);
                                        }
                                        break 'iife_ret_31 tok;
                                    }
                                }
                                2 => {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_31 XML_TOK_INVALID_1;
                                }
                                _ => {
                                    ptr = ptr.wrapping_offset(2isize);
                                }
                            }
                        }
                        ptr = ptr.wrapping_offset(2);
                        if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                            break 'iife_ret_31 XML_TOK_PARTIAL_1;
                        }
                        match if read_ptr_at(ptr, 1) as c_int == 0 {
                            as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize]
                                as c_uint
                        } else {
                            unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                        } {
                            BT_S | BT_CR | BT_LF => {
                                loop {
                                    ptr = ptr.wrapping_offset(2);
                                    if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                        break 'iife_ret_31 XML_TOK_PARTIAL_1;
                                    }
                                    match if read_ptr_at(ptr, 1) as c_int == 0 {
                                        as_normal_encoding(enc).type_0
                                            [read_ptr(ptr) as c_uchar as usize]
                                            as c_uint
                                    } else {
                                        unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0))
                                            as c_uint
                                    } {
                                        BT_NONASCII => {
                                            if namingBitmap[(((nmstrtPages
                                                [read_ptr_at(ptr, 1) as c_uchar as usize]
                                                as c_int)
                                                << 3)
                                                + (read_ptr_at(ptr, 0) as c_uchar as c_int >> 5))
                                                as usize]
                                                & (1)
                                                    << (read_ptr_at(ptr, 0) as c_uchar as c_int
                                                        & 0x1f)
                                                == 0
                                            {
                                                *nextTokPtr = ptr_to(ptr);
                                                break 'iife_ret_31 XML_TOK_INVALID_1;
                                            }
                                            current_block_186 = 923465642386550266;
                                            break;
                                        }
                                        BT_NMSTRT | BT_HEX => {
                                            current_block_186 = 923465642386550266;
                                            break;
                                        }
                                        BT_LEAD2 => {
                                            if (c_char_ptr_diff(end, ptr)) < 2 {
                                                break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                                            }
                                            *nextTokPtr = ptr_to(ptr);
                                            break 'iife_ret_31 XML_TOK_INVALID_1;
                                        }
                                        BT_LEAD3 => {
                                            if (c_char_ptr_diff(end, ptr)) < 3 {
                                                break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                                            }
                                            *nextTokPtr = ptr_to(ptr);
                                            break 'iife_ret_31 XML_TOK_INVALID_1;
                                        }
                                        BT_LEAD4 => {
                                            if (c_char_ptr_diff(end, ptr)) < 4 {
                                                break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                                            }
                                            *nextTokPtr = ptr_to(ptr);
                                            break 'iife_ret_31 XML_TOK_INVALID_1;
                                        }
                                        BT_S | BT_CR | BT_LF => {}
                                        BT_GT => {
                                            current_block_186 = 15103464935601583148;
                                            break;
                                        }
                                        BT_SOL => {
                                            current_block_186 = 619033562305054167;
                                            break;
                                        }
                                        _ => {
                                            *nextTokPtr = ptr_to(ptr);
                                            break 'iife_ret_31 XML_TOK_INVALID_1;
                                        }
                                    }
                                }
                                match current_block_186 {
                                    15103464935601583148 => {}
                                    619033562305054167 => {}
                                    1634947208139838470 => {}
                                    _ => {
                                        ptr = ptr.wrapping_offset(2);
                                        current_block_186 = 1634947208139838470;
                                    }
                                }
                            }
                            BT_SOL => {
                                current_block_186 = 619033562305054167;
                            }
                            BT_GT => {
                                current_block_186 = 15103464935601583148;
                            }
                            _ => {
                                *nextTokPtr = ptr_to(ptr);
                                break 'iife_ret_31 XML_TOK_INVALID_1;
                            }
                        }
                        match current_block_186 {
                            1634947208139838470 => {}
                            _ => match current_block_186 {
                                619033562305054167 => {
                                    ptr = ptr.wrapping_offset(2);
                                    if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                        break 'iife_ret_31 XML_TOK_PARTIAL_1;
                                    }
                                    if !(read_ptr_at(ptr, 1) as c_int == 0
                                        && read_ptr_at(ptr, 0) as c_int == 0x3e)
                                    {
                                        *nextTokPtr = ptr_to(ptr);
                                        break 'iife_ret_31 XML_TOK_INVALID_1;
                                    }
                                    *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                                    break 'iife_ret_31 XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1;
                                }
                                _ => {
                                    *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                                    break 'iife_ret_31 XML_TOK_START_TAG_WITH_ATTS_1;
                                }
                            },
                        }
                    }
                    17747718632989559416 => {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                    _ => {}
                }
            }
            break 'iife_ret_31 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn little2_scanLt(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_32: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr = |ptr: usize| input[ptr];
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let c_char_slice_from_ptr_end = |ptr: usize, end: usize| &input[ptr..end];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            let mut hadColon: c_int = 0;
            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                break 'iife_ret_32 XML_TOK_PARTIAL_1;
            }
            let mut current_block_45: u64;
            match if read_ptr_at(ptr, 1) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
            } {
                BT_NONASCII => {
                    if namingBitmap[(((nmstrtPages[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_int)
                        << 3)
                        + (read_ptr_at(ptr, 0) as c_uchar as c_int >> 5))
                        as usize]
                        & (1) << (read_ptr_at(ptr, 0) as c_uchar as c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_32 XML_TOK_INVALID_1;
                    }
                    current_block_45 = 18046087305847344724;
                }
                BT_NMSTRT | BT_HEX => {
                    current_block_45 = 18046087305847344724;
                }
                BT_LEAD2 => {
                    if (c_char_ptr_diff(end, ptr)) < 2 {
                        break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_32 XML_TOK_INVALID_1;
                }
                BT_LEAD3 => {
                    if (c_char_ptr_diff(end, ptr)) < 3 {
                        break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_32 XML_TOK_INVALID_1;
                }
                BT_LEAD4 => {
                    if (c_char_ptr_diff(end, ptr)) < 4 {
                        break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_32 XML_TOK_INVALID_1;
                }
                BT_EXCL => {
                    ptr = ptr.wrapping_offset(2);
                    if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                        break 'iife_ret_32 XML_TOK_PARTIAL_1;
                    }
                    match if read_ptr_at(ptr, 1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                    } {
                        BT_MINUS => {
                            break 'iife_ret_32 ({
                                let (tok_value, next_tok_value) = little2_scanComment(
                                    enc,
                                    c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                                );
                                *nextTokPtr = next_tok_value;
                                tok_value
                            });
                        }
                        BT_LSQB => {
                            break 'iife_ret_32 ({
                                let (tok_value, next_tok_value) = little2_scanCdataSection(
                                    enc,
                                    c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                                );
                                *nextTokPtr = next_tok_value;
                                tok_value
                            });
                        }
                        _ => {}
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_32 XML_TOK_INVALID_1;
                }
                BT_QUEST => {
                    break 'iife_ret_32 ({
                        let (tok_value, next_tok_value) = little2_scanPi(
                            enc,
                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                        );
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_SOL => {
                    break 'iife_ret_32 ({
                        let (tok_value, next_tok_value) = little2_scanEndTag(
                            enc,
                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                        );
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                _ => {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_32 XML_TOK_INVALID_1;
                }
            }
            if current_block_45 == 18046087305847344724 {
                ptr = ptr.wrapping_offset(2isize);
            }
            hadColon = 0;
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                let mut current_block_161: u64;
                match if read_ptr_at(ptr, 1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((namePages[read_ptr_at(ptr, 1) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (read_ptr_at(ptr, 0) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (read_ptr_at(ptr, 0) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_32 XML_TOK_INVALID_1;
                        }
                        current_block_161 = 8998928240368606981;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_161 = 8998928240368606981;
                    }
                    BT_LEAD2 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_32 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_32 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_32 XML_TOK_INVALID_1;
                    }
                    BT_COLON_0 => {
                        if hadColon != 0 {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_32 XML_TOK_INVALID_1;
                        }
                        hadColon = 1;
                        ptr = ptr.wrapping_offset(2);
                        if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                            break 'iife_ret_32 XML_TOK_PARTIAL_1;
                        }
                        let mut current_block_112: u64;
                        match if read_ptr_at(ptr, 1) as c_int == 0 {
                            as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize]
                                as c_uint
                        } else {
                            unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                        } {
                            BT_NONASCII => {
                                if namingBitmap[(((nmstrtPages
                                    [read_ptr_at(ptr, 1) as c_uchar as usize]
                                    as c_int)
                                    << 3)
                                    + (read_ptr_at(ptr, 0) as c_uchar as c_int >> 5))
                                    as usize]
                                    & (1) << (read_ptr_at(ptr, 0) as c_uchar as c_int & 0x1f)
                                    == 0
                                {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_32 XML_TOK_INVALID_1;
                                }
                                current_block_112 = 14391208795021697965;
                            }
                            BT_NMSTRT | BT_HEX => {
                                current_block_112 = 14391208795021697965;
                            }
                            BT_LEAD2 => {
                                if (c_char_ptr_diff(end, ptr)) < 2 {
                                    break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                                }
                                *nextTokPtr = ptr_to(ptr);
                                break 'iife_ret_32 XML_TOK_INVALID_1;
                            }
                            BT_LEAD3 => {
                                if (c_char_ptr_diff(end, ptr)) < 3 {
                                    break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                                }
                                *nextTokPtr = ptr_to(ptr);
                                break 'iife_ret_32 XML_TOK_INVALID_1;
                            }
                            BT_LEAD4 => {
                                if (c_char_ptr_diff(end, ptr)) < 4 {
                                    break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                                }
                                *nextTokPtr = ptr_to(ptr);
                                break 'iife_ret_32 XML_TOK_INVALID_1;
                            }
                            _ => {
                                *nextTokPtr = ptr_to(ptr);
                                break 'iife_ret_32 XML_TOK_INVALID_1;
                            }
                        }
                        if current_block_112 == 14391208795021697965 {
                            ptr = ptr.wrapping_offset(2isize);
                        }
                        current_block_161 = 14714495436747744489;
                    }
                    BT_S | BT_CR | BT_LF => {
                        ptr = ptr.wrapping_offset(2);
                        loop {
                            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                current_block_161 = 13215501469961642988;
                                break;
                            }
                            match if read_ptr_at(ptr, 1) as c_int == 0 {
                                as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize]
                                    as c_uint
                            } else {
                                unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0))
                                    as c_uint
                            } {
                                BT_NONASCII => {
                                    if namingBitmap[(((nmstrtPages
                                        [read_ptr_at(ptr, 1) as c_uchar as usize]
                                        as c_int)
                                        << 3)
                                        + (read_ptr_at(ptr, 0) as c_uchar as c_int >> 5))
                                        as usize]
                                        & (1) << (read_ptr_at(ptr, 0) as c_uchar as c_int & 0x1f)
                                        == 0
                                    {
                                        *nextTokPtr = ptr_to(ptr);
                                        break 'iife_ret_32 XML_TOK_INVALID_1;
                                    }
                                    current_block_161 = 2369392326157537288;
                                }
                                BT_NMSTRT | BT_HEX => {
                                    current_block_161 = 2369392326157537288;
                                }
                                BT_LEAD2 => {
                                    if (c_char_ptr_diff(end, ptr)) < 2 {
                                        break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_32 XML_TOK_INVALID_1;
                                }
                                BT_LEAD3 => {
                                    if (c_char_ptr_diff(end, ptr)) < 3 {
                                        break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_32 XML_TOK_INVALID_1;
                                }
                                BT_LEAD4 => {
                                    if (c_char_ptr_diff(end, ptr)) < 4 {
                                        break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_32 XML_TOK_INVALID_1;
                                }
                                BT_GT => {
                                    current_block_161 = 1918622160084604696;
                                    break;
                                }
                                BT_SOL => {
                                    current_block_161 = 1114269873380682160;
                                    break;
                                }
                                BT_S | BT_CR | BT_LF => {
                                    ptr = ptr.wrapping_offset(2);
                                    continue;
                                }
                                _ => {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_32 XML_TOK_INVALID_1;
                                }
                            }
                            if current_block_161 == 2369392326157537288 {
                                ptr = ptr.wrapping_offset(2isize);
                            }
                            break 'iife_ret_32 ({
                                let (tok_value, next_tok_value) =
                                    little2_scanAtts(enc, c_char_slice_from_ptr_end(ptr, end));
                                *nextTokPtr = next_tok_value;
                                tok_value
                            });
                        }
                        match current_block_161 {
                            1918622160084604696 => {}
                            1114269873380682160 => {}
                            _ => break 'iife_ret_32 XML_TOK_PARTIAL_1,
                        }
                    }
                    BT_GT => {
                        current_block_161 = 1918622160084604696;
                    }
                    BT_SOL => {
                        current_block_161 = 1114269873380682160;
                    }
                    _ => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_32 XML_TOK_INVALID_1;
                    }
                }
                match current_block_161 {
                    1114269873380682160 => {
                        ptr = ptr.wrapping_offset(2);
                        if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                            break 'iife_ret_32 XML_TOK_PARTIAL_1;
                        }
                        if !(read_ptr_at(ptr, 1) as c_int == 0
                            && read_ptr_at(ptr, 0) as c_int == 0x3e)
                        {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_32 XML_TOK_INVALID_1;
                        }
                        *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                        break 'iife_ret_32 XML_TOK_EMPTY_ELEMENT_NO_ATTS_1;
                    }
                    1918622160084604696 => {
                        *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                        break 'iife_ret_32 XML_TOK_START_TAG_NO_ATTS_1;
                    }
                    8998928240368606981 => {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                    _ => {}
                }
            }
            break 'iife_ret_32 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn little2_contentTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_33: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr = |ptr: usize| input[ptr];
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let c_char_slice_from_ptr_end = |ptr: usize, end: usize| &input[ptr..end];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            if ptr >= end {
                break 'iife_ret_33 XML_TOK_NONE_1;
            }
            {
                let mut n: size_t = c_char_ptr_diff(end, ptr) as size_t;
                if n & (2i32 - 1) as size_t != 0 {
                    n &= !(2i32 - 1) as size_t;
                    if n == 0 {
                        break 'iife_ret_33 XML_TOK_PARTIAL_1;
                    }
                    end = ptr.wrapping_add(n);
                }
            }
            match if read_ptr_at(ptr, 1) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
            } {
                BT_LT => {
                    break 'iife_ret_33 ({
                        let (tok_value, next_tok_value) = little2_scanLt(
                            enc,
                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                        );
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_AMP => {
                    break 'iife_ret_33 ({
                        let (tok_value, next_tok_value) = little2_scanRef(
                            enc,
                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                        );
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_CR => {
                    ptr = ptr.wrapping_offset(2);
                    if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                        break 'iife_ret_33 XML_TOK_TRAILING_CR_1;
                    }
                    if (if read_ptr_at(ptr, 1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_int
                    } else {
                        unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0))
                    }) == BT_LF as c_int
                    {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_33 XML_TOK_DATA_NEWLINE_1;
                }
                BT_LF => {
                    *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                    break 'iife_ret_33 XML_TOK_DATA_NEWLINE_1;
                }
                BT_RSQB => {
                    ptr = ptr.wrapping_offset(2);
                    if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                        break 'iife_ret_33 XML_TOK_TRAILING_RSQB_1;
                    }
                    if read_ptr_at(ptr, 1) as c_int == 0 && read_ptr_at(ptr, 0) as c_int == 0x5d {
                        ptr = ptr.wrapping_offset(2);
                        if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                            break 'iife_ret_33 XML_TOK_TRAILING_RSQB_1;
                        }
                        if !(read_ptr_at(ptr, 1) as c_int == 0
                            && read_ptr_at(ptr, 0) as c_int == 0x3e)
                        {
                            ptr = ptr.wrapping_offset(-(2isize));
                        } else {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_33 XML_TOK_INVALID_1;
                        }
                    }
                }
                BT_LEAD2 => {
                    if (c_char_ptr_diff(end, ptr)) < 2 {
                        break 'iife_ret_33 XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.wrapping_offset(2isize);
                }
                BT_LEAD3 => {
                    if (c_char_ptr_diff(end, ptr)) < 3 {
                        break 'iife_ret_33 XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.wrapping_offset(3isize);
                }
                BT_LEAD4 => {
                    if (c_char_ptr_diff(end, ptr)) < 4 {
                        break 'iife_ret_33 XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.wrapping_offset(4isize);
                }
                BT_NONXML | BT_MALFORM | BT_TRAIL => {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_33 XML_TOK_INVALID_1;
                }
                _ => {
                    ptr = ptr.wrapping_offset(2isize);
                }
            }
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                let mut current_block_76: u64;
                match if read_ptr_at(ptr, 1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                } {
                    BT_LEAD2 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_33 XML_TOK_DATA_CHARS_1;
                        }
                        ptr = ptr.wrapping_offset(2);
                        current_block_76 = 7158658067966855297;
                    }
                    BT_LEAD3 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_33 XML_TOK_DATA_CHARS_1;
                        }
                        ptr = ptr.wrapping_offset(3);
                        current_block_76 = 7158658067966855297;
                    }
                    BT_LEAD4 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_33 XML_TOK_DATA_CHARS_1;
                        }
                        ptr = ptr.wrapping_offset(4);
                        current_block_76 = 7158658067966855297;
                    }
                    BT_RSQB => {
                        if c_char_ptr_diff(end, ptr) >= (2i32 * 2) as c_long {
                            if !(read_ptr_at(ptr.wrapping_offset(2), 1) as c_int == 0
                                && read_ptr_at(ptr.wrapping_offset(2), 0) as c_int == 0x5d)
                            {
                                ptr = ptr.wrapping_offset(2);
                                current_block_76 = 7158658067966855297;
                            } else if c_char_ptr_diff(end, ptr) >= (3i32 * 2) as c_long {
                                if !(read_ptr_at(ptr.wrapping_offset((2i32 * 2) as isize), 1)
                                    as c_int
                                    == 0
                                    && read_ptr_at(ptr.wrapping_offset((2i32 * 2) as isize), 0)
                                        as c_int
                                        == 0x3e)
                                {
                                    ptr = ptr.wrapping_offset(2isize);
                                } else {
                                    *nextTokPtr = ptr_to(ptr.wrapping_offset((2i32 * 2) as isize));
                                    break 'iife_ret_33 XML_TOK_INVALID_1;
                                }
                                current_block_76 = 7158658067966855297;
                            } else {
                                current_block_76 = 17804070343020517427;
                            }
                        } else {
                            current_block_76 = 17804070343020517427;
                        }
                    }
                    BT_AMP | BT_LT | BT_NONXML | BT_MALFORM | BT_TRAIL | BT_CR | BT_LF => {
                        current_block_76 = 17804070343020517427;
                    }
                    _ => {
                        ptr = ptr.wrapping_offset(2);
                        current_block_76 = 7158658067966855297;
                    }
                }
                match current_block_76 {
                    7158658067966855297 => {}
                    _ => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_33 XML_TOK_DATA_CHARS_1;
                    }
                }
            }
            *nextTokPtr = ptr_to(ptr);
            break 'iife_ret_33 XML_TOK_DATA_CHARS_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn little2_scanPercent(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_34: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr = |ptr: usize| input[ptr];
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                break 'iife_ret_34 XML_TOK_PARTIAL_1;
            }
            let mut current_block_34: u64;
            match if read_ptr_at(ptr, 1) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
            } {
                BT_NONASCII => {
                    if namingBitmap[(((nmstrtPages[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_int)
                        << 3)
                        + (read_ptr_at(ptr, 0) as c_uchar as c_int >> 5))
                        as usize]
                        & (1) << (read_ptr_at(ptr, 0) as c_uchar as c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_34 XML_TOK_INVALID_1;
                    }
                    current_block_34 = 27123471380826226;
                }
                BT_NMSTRT | BT_HEX => {
                    current_block_34 = 27123471380826226;
                }
                BT_LEAD2 => {
                    if (c_char_ptr_diff(end, ptr)) < 2 {
                        break 'iife_ret_34 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_34 XML_TOK_INVALID_1;
                }
                BT_LEAD3 => {
                    if (c_char_ptr_diff(end, ptr)) < 3 {
                        break 'iife_ret_34 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_34 XML_TOK_INVALID_1;
                }
                BT_LEAD4 => {
                    if (c_char_ptr_diff(end, ptr)) < 4 {
                        break 'iife_ret_34 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_34 XML_TOK_INVALID_1;
                }
                BT_S | BT_LF | BT_CR | BT_PERCNT => {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_34 XML_TOK_PERCENT_1;
                }
                _ => {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_34 XML_TOK_INVALID_1;
                }
            }
            if current_block_34 == 27123471380826226 {
                ptr = ptr.wrapping_offset(2isize);
            }
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                let mut current_block_65: u64;
                match if read_ptr_at(ptr, 1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((namePages[read_ptr_at(ptr, 1) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (read_ptr_at(ptr, 0) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (read_ptr_at(ptr, 0) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_34 XML_TOK_INVALID_1;
                        }
                        current_block_65 = 8394962855094477842;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_65 = 8394962855094477842;
                    }
                    BT_LEAD2 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            break 'iife_ret_34 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_34 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            break 'iife_ret_34 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_34 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            break 'iife_ret_34 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_34 XML_TOK_INVALID_1;
                    }
                    BT_SEMI => {
                        *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                        break 'iife_ret_34 XML_TOK_PARAM_ENTITY_REF_1;
                    }
                    _ => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_34 XML_TOK_INVALID_1;
                    }
                }
                if current_block_65 == 8394962855094477842 {
                    ptr = ptr.wrapping_offset(2isize);
                }
            }
            break 'iife_ret_34 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn little2_scanPoundName(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_35: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr = |ptr: usize| input[ptr];
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                break 'iife_ret_35 XML_TOK_PARTIAL_1;
            }
            let mut current_block_32: u64;
            match if read_ptr_at(ptr, 1) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
            } {
                BT_NONASCII => {
                    if namingBitmap[(((nmstrtPages[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_int)
                        << 3)
                        + (read_ptr_at(ptr, 0) as c_uchar as c_int >> 5))
                        as usize]
                        & (1) << (read_ptr_at(ptr, 0) as c_uchar as c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_35 XML_TOK_INVALID_1;
                    }
                    current_block_32 = 14940290876465470105;
                }
                BT_NMSTRT | BT_HEX => {
                    current_block_32 = 14940290876465470105;
                }
                BT_LEAD2 => {
                    if (c_char_ptr_diff(end, ptr)) < 2 {
                        break 'iife_ret_35 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_35 XML_TOK_INVALID_1;
                }
                BT_LEAD3 => {
                    if (c_char_ptr_diff(end, ptr)) < 3 {
                        break 'iife_ret_35 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_35 XML_TOK_INVALID_1;
                }
                BT_LEAD4 => {
                    if (c_char_ptr_diff(end, ptr)) < 4 {
                        break 'iife_ret_35 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_35 XML_TOK_INVALID_1;
                }
                _ => {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_35 XML_TOK_INVALID_1;
                }
            }
            if current_block_32 == 14940290876465470105 {
                ptr = ptr.wrapping_offset(2isize);
            }
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                let mut current_block_63: u64;
                match if read_ptr_at(ptr, 1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((namePages[read_ptr_at(ptr, 1) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (read_ptr_at(ptr, 0) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (read_ptr_at(ptr, 0) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_35 XML_TOK_INVALID_1;
                        }
                        current_block_63 = 11497795575834122789;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_63 = 11497795575834122789;
                    }
                    BT_LEAD2 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            break 'iife_ret_35 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_35 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            break 'iife_ret_35 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_35 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            break 'iife_ret_35 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_35 XML_TOK_INVALID_1;
                    }
                    BT_CR | BT_LF | BT_S | BT_RPAR | BT_GT | BT_PERCNT | BT_VERBAR => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_35 XML_TOK_POUND_NAME_1;
                    }
                    _ => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_35 XML_TOK_INVALID_1;
                    }
                }
                if current_block_63 == 11497795575834122789 {
                    ptr = ptr.wrapping_offset(2isize);
                }
            }
            break 'iife_ret_35 -XML_TOK_POUND_NAME_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn little2_scanLit(
        mut open: c_int,
        enc: &ENCODING,
        input: &[c_char],
    ) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_36: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr = |ptr: usize| input[ptr];
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                let mut t: c_int = if read_ptr_at(ptr, 1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_int
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0))
                };
                match t {
                    5 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            break 'iife_ret_36 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.wrapping_offset(2isize);
                    }
                    6 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            break 'iife_ret_36 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.wrapping_offset(3isize);
                    }
                    7 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            break 'iife_ret_36 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.wrapping_offset(4isize);
                    }
                    0 | 1 | 8 => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_36 XML_TOK_INVALID_1;
                    }
                    12 | 13 => {
                        ptr = ptr.wrapping_offset(2);
                        if t == open {
                            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                break 'iife_ret_36 -XML_TOK_LITERAL_1;
                            }
                            *nextTokPtr = ptr_to(ptr);
                            match if read_ptr_at(ptr, 1) as c_int == 0 {
                                as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize]
                                    as c_uint
                            } else {
                                unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0))
                                    as c_uint
                            } {
                                BT_S | BT_CR | BT_LF | BT_GT | BT_PERCNT | BT_LSQB => {
                                    break 'iife_ret_36 XML_TOK_LITERAL_1
                                }
                                _ => break 'iife_ret_36 XML_TOK_INVALID_1,
                            }
                        }
                    }
                    _ => {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                }
            }
            break 'iife_ret_36 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn little2_prologTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_37: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr = |ptr: usize| input[ptr];
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let c_char_slice_from_ptr_end = |ptr: usize, end: usize| &input[ptr..end];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            let mut tok: c_int = 0;
            if ptr >= end {
                break 'iife_ret_37 XML_TOK_NONE_1;
            }
            {
                let mut n: size_t = c_char_ptr_diff(end, ptr) as size_t;
                if n & (2i32 - 1) as size_t != 0 {
                    n &= !(2i32 - 1) as size_t;
                    if n == 0 {
                        break 'iife_ret_37 XML_TOK_PARTIAL_1;
                    }
                    end = ptr.wrapping_add(n);
                }
            }
            let mut current_block_124: u64;
            match if read_ptr_at(ptr, 1) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
            } {
                BT_QUOT => {
                    break 'iife_ret_37 ({
                        let (tok_value, next_tok_value) = little2_scanLit(
                            BT_QUOT as c_int,
                            enc,
                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                        );
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_APOS => {
                    break 'iife_ret_37 ({
                        let (tok_value, next_tok_value) = little2_scanLit(
                            BT_APOS as c_int,
                            enc,
                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                        );
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_LT => {
                    ptr = ptr.wrapping_offset(2);
                    if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                        break 'iife_ret_37 XML_TOK_PARTIAL_1;
                    }
                    match if read_ptr_at(ptr, 1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                    } {
                        BT_EXCL => {
                            break 'iife_ret_37 ({
                                let (tok_value, next_tok_value) = little2_scanDecl(
                                    enc,
                                    c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                                );
                                *nextTokPtr = next_tok_value;
                                tok_value
                            });
                        }
                        BT_QUEST => {
                            break 'iife_ret_37 ({
                                let (tok_value, next_tok_value) = little2_scanPi(
                                    enc,
                                    c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                                );
                                *nextTokPtr = next_tok_value;
                                tok_value
                            });
                        }
                        BT_NMSTRT | BT_HEX | BT_NONASCII | BT_LEAD2 | BT_LEAD3 | BT_LEAD4 => {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(-(2)));
                            break 'iife_ret_37 XML_TOK_INSTANCE_START;
                        }
                        _ => {}
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_37 XML_TOK_INVALID_1;
                }
                BT_CR => {
                    if ptr.wrapping_offset(2) == end {
                        *nextTokPtr = ptr_to(end);
                        break 'iife_ret_37 -XML_TOK_PROLOG_S_1;
                    }
                    current_block_124 = 17513858719706519675;
                }
                BT_S | BT_LF => {
                    current_block_124 = 17513858719706519675;
                }
                BT_PERCNT => {
                    break 'iife_ret_37 ({
                        let (tok_value, next_tok_value) = little2_scanPercent(
                            enc,
                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                        );
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_COMMA => {
                    *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                    break 'iife_ret_37 XML_TOK_COMMA_1;
                }
                BT_LSQB => {
                    *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                    break 'iife_ret_37 XML_TOK_OPEN_BRACKET_1;
                }
                BT_RSQB => {
                    ptr = ptr.wrapping_offset(2);
                    if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                        break 'iife_ret_37 -XML_TOK_CLOSE_BRACKET_1;
                    }
                    if read_ptr_at(ptr, 1) as c_int == 0 && read_ptr_at(ptr, 0) as c_int == 0x5d {
                        if (c_char_ptr_diff(end, ptr)) < (2i32 * 2) as c_long {
                            break 'iife_ret_37 XML_TOK_PARTIAL_1;
                        }
                        if read_ptr_at(ptr.wrapping_offset(2), 1) as c_int == 0
                            && read_ptr_at(ptr.wrapping_offset(2), 0) as c_int == 0x3e
                        {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset((2i32 * 2) as isize));
                            break 'iife_ret_37 XML_TOK_COND_SECT_CLOSE_1;
                        }
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_37 XML_TOK_CLOSE_BRACKET_1;
                }
                BT_LPAR => {
                    *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                    break 'iife_ret_37 XML_TOK_OPEN_PAREN_1;
                }
                BT_RPAR => {
                    ptr = ptr.wrapping_offset(2);
                    if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                        break 'iife_ret_37 -XML_TOK_CLOSE_PAREN_1;
                    }
                    match if read_ptr_at(ptr, 1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                    } {
                        BT_AST => {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                            break 'iife_ret_37 XML_TOK_CLOSE_PAREN_ASTERISK_1;
                        }
                        BT_QUEST => {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                            break 'iife_ret_37 XML_TOK_CLOSE_PAREN_QUESTION_1;
                        }
                        BT_PLUS => {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                            break 'iife_ret_37 XML_TOK_CLOSE_PAREN_PLUS_1;
                        }
                        BT_CR | BT_LF | BT_S | BT_GT | BT_COMMA | BT_VERBAR | BT_RPAR => {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_37 XML_TOK_CLOSE_PAREN_1;
                        }
                        _ => {}
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_37 XML_TOK_INVALID_1;
                }
                BT_VERBAR => {
                    *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                    break 'iife_ret_37 XML_TOK_OR_1;
                }
                BT_GT => {
                    *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                    break 'iife_ret_37 XML_TOK_DECL_CLOSE_1;
                }
                BT_NUM => {
                    break 'iife_ret_37 ({
                        let (tok_value, next_tok_value) = little2_scanPoundName(
                            enc,
                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                        );
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_LEAD2 => {
                    if (c_char_ptr_diff(end, ptr)) < 2 {
                        break 'iife_ret_37 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_37 XML_TOK_INVALID_1;
                }
                BT_LEAD3 => {
                    if (c_char_ptr_diff(end, ptr)) < 3 {
                        break 'iife_ret_37 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_37 XML_TOK_INVALID_1;
                }
                BT_LEAD4 => {
                    if (c_char_ptr_diff(end, ptr)) < 4 {
                        break 'iife_ret_37 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_37 XML_TOK_INVALID_1;
                }
                BT_NMSTRT | BT_HEX => {
                    tok = XML_TOK_NAME;
                    ptr = ptr.wrapping_offset(2);
                    current_block_124 = 2956972668325154207;
                }
                BT_DIGIT | BT_NAME | BT_MINUS | BT_COLON_0 => {
                    tok = XML_TOK_NMTOKEN_1;
                    ptr = ptr.wrapping_offset(2);
                    current_block_124 = 2956972668325154207;
                }
                BT_NONASCII => {
                    if namingBitmap[(((nmstrtPages[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_int)
                        << 3)
                        + (read_ptr_at(ptr, 0) as c_uchar as c_int >> 5))
                        as usize]
                        & (1) << (read_ptr_at(ptr, 0) as c_uchar as c_int & 0x1f)
                        != 0
                    {
                        ptr = ptr.wrapping_offset(2);
                        tok = XML_TOK_NAME;
                        current_block_124 = 2956972668325154207;
                    } else if namingBitmap[(((namePages[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_int)
                        << 3)
                        + (read_ptr_at(ptr, 0) as c_uchar as c_int >> 5))
                        as usize]
                        & (1) << (read_ptr_at(ptr, 0) as c_uchar as c_int & 0x1f)
                        != 0
                    {
                        ptr = ptr.wrapping_offset(2);
                        tok = XML_TOK_NMTOKEN_1;
                        current_block_124 = 2956972668325154207;
                    } else {
                        current_block_124 = 2543942683527618915;
                    }
                }
                _ => {
                    current_block_124 = 2543942683527618915;
                }
            }
            match current_block_124 {
                2956972668325154207 => {}
                17513858719706519675 => {
                    loop {
                        ptr = ptr.wrapping_offset(2);
                        if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                            break;
                        }
                        let mut current_block_32: u64;
                        match if read_ptr_at(ptr, 1) as c_int == 0 {
                            as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize]
                                as c_uint
                        } else {
                            unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                        } {
                            BT_S | BT_LF => {
                                current_block_32 = 17500079516916021833;
                            }
                            BT_CR => {
                                if ptr.wrapping_offset(2) != end {
                                    current_block_32 = 17500079516916021833;
                                } else {
                                    current_block_32 = 17471725448347076649;
                                }
                            }
                            _ => {
                                current_block_32 = 17471725448347076649;
                            }
                        }
                        match current_block_32 {
                            17500079516916021833 => {}
                            _ => {
                                *nextTokPtr = ptr_to(ptr);
                                break 'iife_ret_37 XML_TOK_PROLOG_S_1;
                            }
                        }
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_37 XML_TOK_PROLOG_S_1;
                }
                _ => {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_37 XML_TOK_INVALID_1;
                }
            }
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                let mut current_block_210: u64;
                match if read_ptr_at(ptr, 1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((namePages[read_ptr_at(ptr, 1) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (read_ptr_at(ptr, 0) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (read_ptr_at(ptr, 0) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_37 XML_TOK_INVALID_1;
                        }
                        current_block_210 = 786388639404123072;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_210 = 786388639404123072;
                    }
                    BT_LEAD2 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            break 'iife_ret_37 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_37 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            break 'iife_ret_37 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_37 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            break 'iife_ret_37 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_37 XML_TOK_INVALID_1;
                    }
                    BT_GT | BT_RPAR | BT_COMMA | BT_VERBAR | BT_LSQB | BT_PERCNT | BT_S | BT_CR
                    | BT_LF => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_37 tok;
                    }
                    BT_COLON_0 => {
                        ptr = ptr.wrapping_offset(2);
                        match tok {
                            XML_TOK_NAME => {
                                if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                    break 'iife_ret_37 XML_TOK_PARTIAL_1;
                                }
                                tok = XML_TOK_PREFIXED_NAME;
                                let mut current_block_187: u64;
                                match if read_ptr_at(ptr, 1) as c_int == 0 {
                                    as_normal_encoding(enc).type_0
                                        [read_ptr(ptr) as c_uchar as usize]
                                        as c_uint
                                } else {
                                    unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0))
                                        as c_uint
                                } {
                                    BT_NONASCII => {
                                        if namingBitmap[(((namePages
                                            [read_ptr_at(ptr, 1) as c_uchar as usize]
                                            as c_int)
                                            << 3)
                                            + (read_ptr_at(ptr, 0) as c_uchar as c_int >> 5))
                                            as usize]
                                            & (1)
                                                << (read_ptr_at(ptr, 0) as c_uchar as c_int & 0x1f)
                                            == 0
                                        {
                                            *nextTokPtr = ptr_to(ptr);
                                            break 'iife_ret_37 XML_TOK_INVALID_1;
                                        }
                                        current_block_187 = 16869951820887225088;
                                    }
                                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                                        current_block_187 = 16869951820887225088;
                                    }
                                    BT_LEAD2 => {
                                        if (c_char_ptr_diff(end, ptr)) < 2 {
                                            break 'iife_ret_37 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        *nextTokPtr = ptr_to(ptr);
                                        break 'iife_ret_37 XML_TOK_INVALID_1;
                                    }
                                    BT_LEAD3 => {
                                        if (c_char_ptr_diff(end, ptr)) < 3 {
                                            break 'iife_ret_37 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        *nextTokPtr = ptr_to(ptr);
                                        break 'iife_ret_37 XML_TOK_INVALID_1;
                                    }
                                    BT_LEAD4 => {
                                        if (c_char_ptr_diff(end, ptr)) < 4 {
                                            break 'iife_ret_37 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        *nextTokPtr = ptr_to(ptr);
                                        break 'iife_ret_37 XML_TOK_INVALID_1;
                                    }
                                    _ => {
                                        tok = XML_TOK_NMTOKEN_1;
                                        current_block_187 = 9812798724717783973;
                                    }
                                }
                                if current_block_187 == 16869951820887225088 {
                                    ptr = ptr.wrapping_offset(2isize);
                                }
                            }
                            XML_TOK_PREFIXED_NAME => {
                                tok = XML_TOK_NMTOKEN_1;
                            }
                            _ => {}
                        }
                        current_block_210 = 14244298717249035578;
                    }
                    BT_PLUS => {
                        if tok == XML_TOK_NMTOKEN_1 {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_37 XML_TOK_INVALID_1;
                        }
                        *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                        break 'iife_ret_37 XML_TOK_NAME_PLUS_1;
                    }
                    BT_AST => {
                        if tok == XML_TOK_NMTOKEN_1 {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_37 XML_TOK_INVALID_1;
                        }
                        *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                        break 'iife_ret_37 XML_TOK_NAME_ASTERISK_1;
                    }
                    BT_QUEST => {
                        if tok == XML_TOK_NMTOKEN_1 {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_37 XML_TOK_INVALID_1;
                        }
                        *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                        break 'iife_ret_37 XML_TOK_NAME_QUESTION_1;
                    }
                    _ => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_37 XML_TOK_INVALID_1;
                    }
                }
                if current_block_210 == 786388639404123072 {
                    ptr = ptr.wrapping_offset(2isize);
                }
            }
            break 'iife_ret_37 -tok;
        };
        (tok, next_tok)
    }

    pub(crate) fn little2_attributeValueTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_38: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr = |ptr: usize| input[ptr];
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let c_char_slice_from_ptr_end = |ptr: usize, end: usize| &input[ptr..end];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            let mut start: usize = 0;
            if ptr >= end {
                break 'iife_ret_38 XML_TOK_NONE_1;
            } else if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                break 'iife_ret_38 XML_TOK_PARTIAL_1;
            }
            start = ptr;
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                match if read_ptr_at(ptr, 1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                } {
                    BT_LEAD2 => {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                    BT_LEAD3 => {
                        ptr = ptr.wrapping_offset(3isize);
                    }
                    BT_LEAD4 => {
                        ptr = ptr.wrapping_offset(4isize);
                    }
                    BT_AMP => {
                        if ptr == start {
                            break 'iife_ret_38 ({
                                let (tok_value, next_tok_value) = little2_scanRef(
                                    enc,
                                    c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                                );
                                *nextTokPtr = next_tok_value;
                                tok_value
                            });
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_38 XML_TOK_DATA_CHARS_1;
                    }
                    BT_LT => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_38 XML_TOK_INVALID_1;
                    }
                    BT_LF => {
                        if ptr == start {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                            break 'iife_ret_38 XML_TOK_DATA_NEWLINE_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_38 XML_TOK_DATA_CHARS_1;
                    }
                    BT_CR => {
                        if ptr == start {
                            ptr = ptr.wrapping_offset(2);
                            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                break 'iife_ret_38 XML_TOK_TRAILING_CR_1;
                            }
                            if (if read_ptr_at(ptr, 1) as c_int == 0 {
                                as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize]
                                    as c_int
                            } else {
                                unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0))
                            }) == BT_LF as c_int
                            {
                                ptr = ptr.wrapping_offset(2isize);
                            }
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_38 XML_TOK_DATA_NEWLINE_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_38 XML_TOK_DATA_CHARS_1;
                    }
                    BT_S => {
                        if ptr == start {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                            break 'iife_ret_38 XML_TOK_ATTRIBUTE_VALUE_S_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_38 XML_TOK_DATA_CHARS_1;
                    }
                    _ => {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                }
            }
            *nextTokPtr = ptr_to(ptr);
            break 'iife_ret_38 XML_TOK_DATA_CHARS_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn little2_entityValueTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_39: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr = |ptr: usize| input[ptr];
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let c_char_slice_from_ptr_end = |ptr: usize, end: usize| &input[ptr..end];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            let mut start: usize = 0;
            if ptr >= end {
                break 'iife_ret_39 XML_TOK_NONE_1;
            } else if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                break 'iife_ret_39 XML_TOK_PARTIAL_1;
            }
            start = ptr;
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                match if read_ptr_at(ptr, 1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                } {
                    BT_LEAD2 => {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                    BT_LEAD3 => {
                        ptr = ptr.wrapping_offset(3isize);
                    }
                    BT_LEAD4 => {
                        ptr = ptr.wrapping_offset(4isize);
                    }
                    BT_AMP => {
                        if ptr == start {
                            break 'iife_ret_39 ({
                                let (tok_value, next_tok_value) = little2_scanRef(
                                    enc,
                                    c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                                );
                                *nextTokPtr = next_tok_value;
                                tok_value
                            });
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_39 XML_TOK_DATA_CHARS_1;
                    }
                    BT_PERCNT => {
                        if ptr == start {
                            let mut tok: c_int = {
                                let (tok_value, next_tok_value) = little2_scanPercent(
                                    enc,
                                    c_char_slice_from_ptr_end(ptr.wrapping_offset(2), end),
                                );
                                *nextTokPtr = next_tok_value;
                                tok_value
                            };
                            break 'iife_ret_39 if tok == XML_TOK_PERCENT_1 {
                                XML_TOK_INVALID_1
                            } else {
                                tok
                            };
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_39 XML_TOK_DATA_CHARS_1;
                    }
                    BT_LF => {
                        if ptr == start {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                            break 'iife_ret_39 XML_TOK_DATA_NEWLINE_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_39 XML_TOK_DATA_CHARS_1;
                    }
                    BT_CR => {
                        if ptr == start {
                            ptr = ptr.wrapping_offset(2);
                            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                break 'iife_ret_39 XML_TOK_TRAILING_CR_1;
                            }
                            if (if read_ptr_at(ptr, 1) as c_int == 0 {
                                as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize]
                                    as c_int
                            } else {
                                unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0))
                            }) == BT_LF as c_int
                            {
                                ptr = ptr.wrapping_offset(2isize);
                            }
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_39 XML_TOK_DATA_NEWLINE_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_39 XML_TOK_DATA_CHARS_1;
                    }
                    _ => {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                }
            }
            *nextTokPtr = ptr_to(ptr);
            break 'iife_ret_39 XML_TOK_DATA_CHARS_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn little2_ignoreSectionTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_40: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr = |ptr: usize| input[ptr];
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            let mut level: c_int = 0;
            {
                let mut n: size_t = c_char_ptr_diff(end, ptr) as size_t;
                if n & (2i32 - 1) as size_t != 0 {
                    n &= !(2i32 - 1) as size_t;
                    end = ptr.wrapping_add(n);
                }
            }
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                match if read_ptr_at(ptr, 1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
                } {
                    BT_LEAD2 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            break 'iife_ret_40 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.wrapping_offset(2isize);
                    }
                    BT_LEAD3 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            break 'iife_ret_40 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.wrapping_offset(3isize);
                    }
                    BT_LEAD4 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            break 'iife_ret_40 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.wrapping_offset(4isize);
                    }
                    BT_NONXML | BT_MALFORM | BT_TRAIL => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_40 XML_TOK_INVALID_1;
                    }
                    BT_LT => {
                        ptr = ptr.wrapping_offset(2);
                        if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                            break 'iife_ret_40 XML_TOK_PARTIAL_1;
                        }
                        if read_ptr_at(ptr, 1) as c_int == 0 && read_ptr_at(ptr, 0) as c_int == 0x21
                        {
                            ptr = ptr.wrapping_offset(2);
                            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                break 'iife_ret_40 XML_TOK_PARTIAL_1;
                            }
                            if read_ptr_at(ptr, 1) as c_int == 0
                                && read_ptr_at(ptr, 0) as c_int == 0x5b
                            {
                                level += 1;
                                ptr = ptr.wrapping_offset(2isize);
                            }
                        }
                    }
                    BT_RSQB => {
                        ptr = ptr.wrapping_offset(2);
                        if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                            break 'iife_ret_40 XML_TOK_PARTIAL_1;
                        }
                        if read_ptr_at(ptr, 1) as c_int == 0 && read_ptr_at(ptr, 0) as c_int == 0x5d
                        {
                            ptr = ptr.wrapping_offset(2);
                            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                break 'iife_ret_40 XML_TOK_PARTIAL_1;
                            }
                            if read_ptr_at(ptr, 1) as c_int == 0
                                && read_ptr_at(ptr, 0) as c_int == 0x3e
                            {
                                ptr = ptr.wrapping_offset(2);
                                if level == 0 {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_40 XML_TOK_IGNORE_SECT_1;
                                }
                                level -= 1;
                            }
                        }
                    }
                    _ => {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                }
            }
            break 'iife_ret_40 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn little2_isPublicId(enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult {
        let mut badPtrVal: *const c_char = null::<c_char>();
        let badPtr = &mut badPtrVal;
        let mut ptr = 0usize;
        let mut end = input.len();
        let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
        let read_ptr = |ptr: usize| input[ptr];
        let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
        let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
        ptr = ptr.wrapping_offset(2);
        end = end.wrapping_offset(-(2));
        while c_char_ptr_diff(end, ptr) >= 2 as c_long {
            let mut current_block_8: u64;
            match if read_ptr_at(ptr, 1) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
            } {
                25 | 24 | 27 | 13 | 31 | 32 | 34 | 35 | 17 | 14 | 15 | 9 | 10 | 18 | 16 | 33
                | 30 | 19 | 23 => {
                    current_block_8 = 5143058163439228106;
                }
                BT_S => {
                    if read_ptr_at(ptr, 1) as c_int == 0 && read_ptr_at(ptr, 0) as c_int == 0x9 {
                        *badPtr = ptr_to(ptr);
                        return (0i32, badPtrVal);
                    }
                    current_block_8 = 5143058163439228106;
                }
                BT_NAME | BT_NMSTRT => {
                    if (if read_ptr_at(ptr, 1) as c_int == 0 {
                        read_ptr_at(ptr, 0) as c_int
                    } else {
                        -(1)
                    }) & !(0x7f)
                        == 0
                    {
                        current_block_8 = 5143058163439228106;
                    } else {
                        current_block_8 = 5251475129761746025;
                    }
                }
                _ => {
                    current_block_8 = 5251475129761746025;
                }
            }
            if current_block_8 == 5251475129761746025 {
                match if read_ptr_at(ptr, 1) as c_int == 0 {
                    read_ptr_at(ptr, 0) as c_int
                } else {
                    -(1)
                } {
                    36 | 64 => {}
                    _ => {
                        *badPtr = ptr_to(ptr);
                        return (0i32, badPtrVal);
                    }
                }
            }
            ptr = ptr.wrapping_offset(2);
        }
        (1, badPtrVal)
    }

    pub(crate) fn little2_getAtts(
        enc: &ENCODING,
        mut ptr: *const c_char,
        mut attsMax: c_int,
        mut atts: *mut ATTRIBUTE,
    ) -> c_int {
        let mut state: C2RustUnnamed_3 = inName_0;
        let mut nAtts: c_int = 0;
        let mut open: c_int = 0;
        ptr = ptr.wrapping_offset(2);
        loop {
            match if read_ptr_at(ptr, 1) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr(ptr) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 1), read_ptr_at(ptr, 0)) as c_uint
            } {
                BT_LEAD2 => {
                    if state == other_0 {
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.name = ptr;
                            att.normalized = 1i8;
                        }
                        state = inName_0;
                    }
                    ptr = ptr.wrapping_offset((2i32 - 2i32) as isize);
                }
                BT_LEAD3 => {
                    if state == other_0 {
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.name = ptr;
                            att.normalized = 1i8;
                        }
                        state = inName_0;
                    }
                    ptr = ptr.wrapping_offset((3i32 - 2i32) as isize);
                }
                BT_LEAD4 => {
                    if state == other_0 {
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.name = ptr;
                            att.normalized = 1i8;
                        }
                        state = inName_0;
                    }
                    ptr = ptr.wrapping_offset((4i32 - 2i32) as isize);
                }
                BT_NONASCII | BT_NMSTRT | BT_HEX => {
                    if state == other_0 {
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.name = ptr;
                            att.normalized = 1i8;
                        }
                        state = inName_0;
                    }
                }
                BT_QUOT => {
                    if state != inValue_0 {
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.valuePtr = ptr.wrapping_offset(2isize);
                        }
                        state = inValue_0;
                        open = BT_QUOT as c_int;
                    } else if open == BT_QUOT as c_int {
                        state = other_0;
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.valueEnd = ptr;
                        }
                        nAtts += 1;
                    }
                }
                BT_APOS => {
                    if state != inValue_0 {
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.valuePtr = ptr.wrapping_offset(2isize);
                        }
                        state = inValue_0;
                        open = BT_APOS as c_int;
                    } else if open == BT_APOS as c_int {
                        state = other_0;
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.valueEnd = ptr;
                        }
                        nAtts += 1;
                    }
                }
                BT_AMP => {
                    if nAtts < attsMax {
                        attribute_mut(atts, nAtts).normalized = 0i8;
                    }
                }
                BT_S => {
                    if state == inName_0 {
                        state = other_0;
                    } else if state == inValue_0 && nAtts < attsMax {
                        let att = attribute_mut(atts, nAtts);
                        if att.normalized as c_int != 0
                            && (ptr == att.valuePtr
                                || (if read_ptr_at(ptr, 1) as c_int == 0 {
                                    read_ptr_at(ptr, 0) as c_int
                                } else {
                                    -(1)
                                }) != ASCII_SPACE
                                || (if read_ptr_at(ptr.wrapping_offset(2), 1) as c_int == 0 {
                                    read_ptr_at(ptr.wrapping_offset(2), 0) as c_int
                                } else {
                                    -(1)
                                }) == ASCII_SPACE
                                || (if read_ptr_at(ptr.wrapping_offset(2), 1) as c_int == 0 {
                                    as_normal_encoding(enc).type_0
                                        [read_ptr_at(ptr, 2) as c_uchar as usize]
                                        as c_int
                                } else {
                                    unicode_byte_type(
                                        read_ptr_at(ptr.wrapping_offset(2), 1),
                                        read_ptr_at(ptr.wrapping_offset(2), 0),
                                    )
                                }) == open)
                        {
                            att.normalized = 0i8;
                        }
                    }
                }
                BT_CR | BT_LF => {
                    if state == inName_0 {
                        state = other_0;
                    } else if state == inValue_0 && nAtts < attsMax {
                        attribute_mut(atts, nAtts).normalized = 0i8;
                    }
                }
                BT_GT | BT_SOL => {
                    if state != inValue_0 {
                        return nAtts;
                    }
                }
                _ => {}
            }
            ptr = ptr.wrapping_offset(2);
        }
    }

    pub(crate) fn little2_charRefNumber(_enc: &ENCODING, mut ptr: *const c_char) -> c_int {
        let mut result: c_int = 0;
        ptr = ptr.wrapping_offset(4);
        if read_c_char_at(ptr, 1) as c_int == 0 && read_c_char_at(ptr, 0) as c_int == 0x78 {
            ptr = ptr.wrapping_offset(2);
            while !(read_c_char_at(ptr, 1) as c_int == 0 && read_c_char_at(ptr, 0) as c_int == 0x3b)
            {
                let c: c_int = if read_c_char_at(ptr, 1) as c_int == 0 {
                    read_c_char_at(ptr, 0) as c_int
                } else {
                    -(1)
                };
                match c {
                    ASCII_0 | ASCII_1_1 | ASCII_2_1 | ASCII_3_1 | ASCII_4 | ASCII_5 | ASCII_6
                    | ASCII_7 | ASCII_8_1 | ASCII_9_1 => {
                        result <<= 4;
                        result |= c - ASCII_0;
                    }
                    ASCII_A | ASCII_B_1 | ASCII_C | ASCII_D | ASCII_E_1 | ASCII_F_1 => {
                        result <<= 4;
                        result += 10i32 + (c - ASCII_A);
                    }
                    ASCII_a_1 | ASCII_b | ASCII_c_1 | ASCII_d | ASCII_e_1 | ASCII_f => {
                        result <<= 4;
                        result += 10i32 + (c - ASCII_a_1);
                    }
                    _ => {}
                }
                if result >= 0x110000 {
                    return -(1i32);
                }
                ptr = ptr.wrapping_offset(2);
            }
        } else {
            while !(read_c_char_at(ptr, 1) as c_int == 0 && read_c_char_at(ptr, 0) as c_int == 0x3b)
            {
                let c_0: c_int = if read_c_char_at(ptr, 1) as c_int == 0 {
                    read_c_char_at(ptr, 0) as c_int
                } else {
                    -(1)
                };
                result *= 10;
                result += c_0 - ASCII_0;
                if result >= 0x110000 {
                    return -(1i32);
                }
                ptr = ptr.wrapping_offset(2);
            }
        }
        checkCharRefNumber(result)
    }

    pub(crate) fn little2_predefinedEntityName(_enc: &ENCODING, input: &[c_char]) -> c_int {
        let mut ptr = 0usize;
        let end = input.len();
        match end.wrapping_sub(ptr) / 2 {
            2 => {
                if input[ptr.wrapping_offset(3)] as c_int == 0
                    && input[ptr.wrapping_offset(2)] as c_int == 0x74
                {
                    match if input[ptr.wrapping_offset(1)] as c_int == 0 {
                        input[ptr.wrapping_offset(0)] as c_int
                    } else {
                        -(1)
                    } {
                        ASCII_l_1 => return ASCII_LT,
                        ASCII_g_1 => return ASCII_GT,
                        _ => {}
                    }
                }
            }
            3 => {
                if input[ptr.wrapping_offset(1)] as c_int == 0
                    && input[ptr.wrapping_offset(0)] as c_int == 0x61
                {
                    ptr = ptr.wrapping_offset(2);
                    if input[ptr.wrapping_offset(1)] as c_int == 0
                        && input[ptr.wrapping_offset(0)] as c_int == 0x6d
                    {
                        ptr = ptr.wrapping_offset(2);
                        if input[ptr.wrapping_offset(1)] as c_int == 0
                            && input[ptr.wrapping_offset(0)] as c_int == 0x70
                        {
                            return ASCII_AMP;
                        }
                    }
                }
            }
            4 => {
                match if input[ptr.wrapping_offset(1)] as c_int == 0 {
                    input[ptr.wrapping_offset(0)] as c_int
                } else {
                    -(1)
                } {
                    ASCII_q => {
                        ptr = ptr.wrapping_offset(2);
                        if input[ptr.wrapping_offset(1)] as c_int == 0
                            && input[ptr.wrapping_offset(0)] as c_int == 0x75
                        {
                            ptr = ptr.wrapping_offset(2);
                            if input[ptr.wrapping_offset(1)] as c_int == 0
                                && input[ptr.wrapping_offset(0)] as c_int == 0x6f
                            {
                                ptr = ptr.wrapping_offset(2);
                                if input[ptr.wrapping_offset(1)] as c_int == 0
                                    && input[ptr.wrapping_offset(0)] as c_int == 0x74
                                {
                                    return ASCII_QUOT;
                                }
                            }
                        }
                    }
                    ASCII_a_1 => {
                        ptr = ptr.wrapping_offset(2);
                        if input[ptr.wrapping_offset(1)] as c_int == 0
                            && input[ptr.wrapping_offset(0)] as c_int == 0x70
                        {
                            ptr = ptr.wrapping_offset(2);
                            if input[ptr.wrapping_offset(1)] as c_int == 0
                                && input[ptr.wrapping_offset(0)] as c_int == 0x6f
                            {
                                ptr = ptr.wrapping_offset(2);
                                if input[ptr.wrapping_offset(1)] as c_int == 0
                                    && input[ptr.wrapping_offset(0)] as c_int == 0x73
                                {
                                    return ASCII_APOS;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        0
    }

    pub(crate) fn little2_nameMatchesAscii(
        _enc: &ENCODING,
        input: &[c_char],
        mut ptr2: *const c_char,
    ) -> c_int {
        let mut ptr1 = 0usize;
        let end1 = input.len();
        while read_c_char(ptr2) != 0 {
            if end1.wrapping_sub(ptr1) < 2 {
                return 0i32;
            }
            if !(input[ptr1.wrapping_offset(1)] as c_int == 0
                && input[ptr1.wrapping_offset(0)] as c_int == read_c_char(ptr2) as c_int)
            {
                return 0i32;
            }
            ptr1 = ptr1.wrapping_offset(2);
            ptr2 = ptr2.wrapping_offset(1);
        }
        (ptr1 == end1) as c_int
    }

    pub(crate) fn little2_nameLength(enc: &ENCODING, mut ptr: *const c_char) -> c_int {
        let start: *const c_char = ptr;
        loop {
            match if read_c_char_at(ptr, 1) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_c_uchar_at(ptr, 0) as usize] as c_uint
            } else {
                unicode_byte_type(read_c_char_at(ptr, 1), read_c_char_at(ptr, 0)) as c_uint
            } {
                BT_LEAD2 => {
                    ptr = ptr.wrapping_offset(2);
                }
                BT_LEAD3 => {
                    ptr = ptr.wrapping_offset(3);
                }
                BT_LEAD4 => {
                    ptr = ptr.wrapping_offset(4);
                }
                BT_NONASCII | BT_NMSTRT | BT_COLON_0 | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                    ptr = ptr.wrapping_offset(2);
                }
                _ => {
                    return c_char_ptr_diff(ptr, start) as c_int;
                }
            }
        }
    }

    pub(crate) fn little2_skipS(enc: &ENCODING, mut ptr: *const c_char) -> *const c_char {
        loop {
            match if read_c_char_at(ptr, 1) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_c_uchar_at(ptr, 0) as usize] as c_uint
            } else {
                unicode_byte_type(read_c_char_at(ptr, 1), read_c_char_at(ptr, 0)) as c_uint
            } {
                BT_LF | BT_CR | BT_S => {
                    ptr = ptr.wrapping_offset(2);
                }
                _ => return ptr,
            }
        }
    }

    pub(crate) fn little2_updatePosition(enc: &ENCODING, input: &[c_char], mut pos: *mut POSITION) {
        let pos = mut_ref_from_ptr(pos);
        let mut ptr = 0usize;
        let end = input.len();
        while end.wrapping_sub(ptr) >= 2 {
            match if input[ptr.wrapping_offset(1)] as c_int == 0 {
                as_normal_encoding(enc).type_0[input[ptr.wrapping_offset(0)] as c_uchar as usize]
                    as c_uint
            } else {
                unicode_byte_type(input[ptr.wrapping_offset(1)], input[ptr.wrapping_offset(0)])
                    as c_uint
            } {
                BT_LEAD2 => {
                    ptr = ptr.wrapping_offset(2);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                BT_LEAD3 => {
                    ptr = ptr.wrapping_offset(3);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                BT_LEAD4 => {
                    ptr = ptr.wrapping_offset(4);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                BT_LF => {
                    pos.columnNumber = 0u64;
                    pos.lineNumber = pos.lineNumber.wrapping_add(1);
                    ptr = ptr.wrapping_offset(2);
                }
                BT_CR => {
                    pos.lineNumber = pos.lineNumber.wrapping_add(1);
                    ptr = ptr.wrapping_offset(2);
                    if end.wrapping_sub(ptr) >= 2
                        && (if input[ptr.wrapping_offset(1)] as c_int == 0 {
                            as_normal_encoding(enc).type_0
                                [input[ptr.wrapping_offset(0)] as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                input[ptr.wrapping_offset(1)],
                                input[ptr.wrapping_offset(0)],
                            )
                        }) == BT_LF as c_int
                    {
                        ptr = ptr.wrapping_offset(2);
                    }
                    pos.columnNumber = 0u64;
                }
                _ => {
                    ptr = ptr.wrapping_offset(2);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
            }
        }
    }

    pub(crate) fn big2_scanComment(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_41: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            if c_char_ptr_diff(end, ptr) >= 2 as c_long {
                if !(read_ptr_at(ptr, 0) as c_int == 0 && read_ptr_at(ptr, 1) as c_int == 0x2d) {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_41 XML_TOK_INVALID_1;
                }
                ptr = ptr.wrapping_offset(2);
                while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                    match if read_ptr_at(ptr, 0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                            as c_uint
                    } else {
                        unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                    } {
                        BT_LEAD2 => {
                            if (c_char_ptr_diff(end, ptr)) < 2 {
                                break 'iife_ret_41 XML_TOK_PARTIAL_CHAR_1;
                            }
                            ptr = ptr.wrapping_offset(2isize);
                        }
                        BT_LEAD3 => {
                            if (c_char_ptr_diff(end, ptr)) < 3 {
                                break 'iife_ret_41 XML_TOK_PARTIAL_CHAR_1;
                            }
                            ptr = ptr.wrapping_offset(3isize);
                        }
                        BT_LEAD4 => {
                            if (c_char_ptr_diff(end, ptr)) < 4 {
                                break 'iife_ret_41 XML_TOK_PARTIAL_CHAR_1;
                            }
                            ptr = ptr.wrapping_offset(4isize);
                        }
                        BT_NONXML | BT_MALFORM | BT_TRAIL => {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_41 XML_TOK_INVALID_1;
                        }
                        BT_MINUS => {
                            ptr = ptr.wrapping_offset(2);
                            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                break 'iife_ret_41 XML_TOK_PARTIAL_1;
                            }
                            if read_ptr_at(ptr, 0) as c_int == 0
                                && read_ptr_at(ptr, 1) as c_int == 0x2d
                            {
                                ptr = ptr.wrapping_offset(2);
                                if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                    break 'iife_ret_41 XML_TOK_PARTIAL_1;
                                }
                                if !(read_ptr_at(ptr, 0) as c_int == 0
                                    && read_ptr_at(ptr, 1) as c_int == 0x3e)
                                {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_41 XML_TOK_INVALID_1;
                                }
                                *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                                break 'iife_ret_41 XML_TOK_COMMENT_1;
                            }
                        }
                        _ => {
                            ptr = ptr.wrapping_offset(2isize);
                        }
                    }
                }
            }
            break 'iife_ret_41 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn big2_scanDecl(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_42: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let c_char_slice_from_ptr_end = |ptr: usize, end: usize| &input[ptr..end];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                break 'iife_ret_42 XML_TOK_PARTIAL_1;
            }
            match if read_ptr_at(ptr, 0) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
            } {
                BT_MINUS => {
                    break 'iife_ret_42 ({
                        let (tok_value, next_tok_value) = big2_scanComment(
                            enc,
                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                        );
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_LSQB => {
                    *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                    break 'iife_ret_42 XML_TOK_COND_SECT_OPEN_1;
                }
                BT_NMSTRT | BT_HEX => {
                    ptr = ptr.wrapping_offset(2isize);
                }
                _ => {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_42 XML_TOK_INVALID_1;
                }
            }
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                's_129: {
                    match if read_ptr_at(ptr, 0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                            as c_uint
                    } else {
                        unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                    } {
                        BT_PERCNT => {
                            if (c_char_ptr_diff(end, ptr)) < (2i32 * 2) as c_long {
                                break 'iife_ret_42 XML_TOK_PARTIAL_1;
                            }
                            match if read_ptr_at(ptr.wrapping_offset(2), 0) as c_int == 0 {
                                as_normal_encoding(enc).type_0
                                    [read_ptr_at(ptr.wrapping_offset(2), 1) as c_uchar as usize]
                                    as c_uint
                            } else {
                                unicode_byte_type(
                                    read_ptr_at(ptr.wrapping_offset(2), 0),
                                    read_ptr_at(ptr.wrapping_offset(2), 1),
                                ) as c_uint
                            } {
                                BT_S | BT_CR | BT_LF | BT_PERCNT => {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_42 XML_TOK_INVALID_1;
                                }
                                _ => {}
                            }
                        }
                        BT_S | BT_CR | BT_LF => {}
                        BT_NMSTRT | BT_HEX => {
                            ptr = ptr.wrapping_offset(2);
                            break 's_129;
                        }
                        _ => {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_42 XML_TOK_INVALID_1;
                        }
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_42 XML_TOK_DECL_OPEN_1;
                }
            }
            break 'iife_ret_42 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn big2_checkPiTarget(_enc: &ENCODING, input: &[c_char]) -> CheckPiTargetResult {
        let mut tok: c_int = 0;
        let tokPtr = &mut tok;
        let result: c_int = 'iife_ret_43: {
            let mut ptr = 0usize;
            let end = input.len();
            let mut upper: c_int = 0;
            *tokPtr = XML_TOK_PI_1;
            if end.wrapping_sub(ptr) != 2 * 3 {
                break 'iife_ret_43 1i32;
            }
            match if input[ptr.wrapping_offset(0)] as c_int == 0 {
                input[ptr.wrapping_offset(1)] as c_int
            } else {
                -(1)
            } {
                ASCII_x_1 => {}
                ASCII_X_1 => {
                    upper = 1i32;
                }
                _ => break 'iife_ret_43 1,
            }
            ptr = ptr.wrapping_offset(2);
            match if input[ptr.wrapping_offset(0)] as c_int == 0 {
                input[ptr.wrapping_offset(1)] as c_int
            } else {
                -(1)
            } {
                ASCII_m_1 => {}
                ASCII_M_1 => {
                    upper = 1i32;
                }
                _ => break 'iife_ret_43 1,
            }
            ptr = ptr.wrapping_offset(2);
            match if input[ptr.wrapping_offset(0)] as c_int == 0 {
                input[ptr.wrapping_offset(1)] as c_int
            } else {
                -(1)
            } {
                ASCII_l_1 => {}
                ASCII_L_1 => {
                    upper = 1i32;
                }
                _ => break 'iife_ret_43 1,
            }
            if upper != 0 {
                break 'iife_ret_43 0i32;
            }
            *tokPtr = XML_TOK_XML_DECL_1;
            break 'iife_ret_43 1;
        };
        (result, tok)
    }

    pub(crate) fn big2_scanPi(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_44: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let c_char_slice_from_ptr_end = |ptr: usize, end: usize| &input[ptr..end];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            let mut tok: c_int = 0;
            let mut target: usize = ptr;
            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                break 'iife_ret_44 XML_TOK_PARTIAL_1;
            }
            let mut current_block_32: u64;
            match if read_ptr_at(ptr, 0) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
            } {
                BT_NONASCII => {
                    if namingBitmap[(((nmstrtPages[read_ptr_at(ptr, 0) as c_uchar as usize]
                        as c_int)
                        << 3)
                        + (read_ptr_at(ptr, 1) as c_uchar as c_int >> 5))
                        as usize]
                        & (1) << (read_ptr_at(ptr, 1) as c_uchar as c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_44 XML_TOK_INVALID_1;
                    }
                    current_block_32 = 2802485987355401260;
                }
                BT_NMSTRT | BT_HEX => {
                    current_block_32 = 2802485987355401260;
                }
                BT_LEAD2 => {
                    if (c_char_ptr_diff(end, ptr)) < 2 {
                        break 'iife_ret_44 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_44 XML_TOK_INVALID_1;
                }
                BT_LEAD3 => {
                    if (c_char_ptr_diff(end, ptr)) < 3 {
                        break 'iife_ret_44 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_44 XML_TOK_INVALID_1;
                }
                BT_LEAD4 => {
                    if (c_char_ptr_diff(end, ptr)) < 4 {
                        break 'iife_ret_44 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_44 XML_TOK_INVALID_1;
                }
                _ => {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_44 XML_TOK_INVALID_1;
                }
            }
            if current_block_32 == 2802485987355401260 {
                ptr = ptr.wrapping_offset(2isize);
            }
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                let mut current_block_118: u64;
                match if read_ptr_at(ptr, 0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((namePages[read_ptr_at(ptr, 0) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (read_ptr_at(ptr, 1) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (read_ptr_at(ptr, 1) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_44 XML_TOK_INVALID_1;
                        }
                        current_block_118 = 11190361564366887465;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_118 = 11190361564366887465;
                    }
                    BT_LEAD2 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            break 'iife_ret_44 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_44 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            break 'iife_ret_44 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_44 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            break 'iife_ret_44 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_44 XML_TOK_INVALID_1;
                    }
                    BT_S | BT_CR | BT_LF => {
                        if {
                            let (ok_value, tok_value) =
                                big2_checkPiTarget(enc, c_char_slice_from_ptr_end(target, ptr));
                            tok = tok_value;
                            ok_value
                        } == 0
                        {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_44 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.wrapping_offset(2);
                        while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                            match if read_ptr_at(ptr, 0) as c_int == 0 {
                                as_normal_encoding(enc).type_0
                                    [read_ptr_at(ptr, 1) as c_uchar as usize]
                                    as c_uint
                            } else {
                                unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1))
                                    as c_uint
                            } {
                                BT_LEAD2 => {
                                    if (c_char_ptr_diff(end, ptr)) < 2 {
                                        break 'iife_ret_44 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    ptr = ptr.wrapping_offset(2isize);
                                }
                                BT_LEAD3 => {
                                    if (c_char_ptr_diff(end, ptr)) < 3 {
                                        break 'iife_ret_44 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    ptr = ptr.wrapping_offset(3isize);
                                }
                                BT_LEAD4 => {
                                    if (c_char_ptr_diff(end, ptr)) < 4 {
                                        break 'iife_ret_44 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    ptr = ptr.wrapping_offset(4isize);
                                }
                                BT_NONXML | BT_MALFORM | BT_TRAIL => {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_44 XML_TOK_INVALID_1;
                                }
                                BT_QUEST => {
                                    ptr = ptr.wrapping_offset(2);
                                    if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                        break 'iife_ret_44 XML_TOK_PARTIAL_1;
                                    }
                                    if read_ptr_at(ptr, 0) as c_int == 0
                                        && read_ptr_at(ptr, 1) as c_int == 0x3e
                                    {
                                        *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                                        break 'iife_ret_44 tok;
                                    }
                                }
                                _ => {
                                    ptr = ptr.wrapping_offset(2isize);
                                }
                            }
                        }
                        break 'iife_ret_44 XML_TOK_PARTIAL_1;
                    }
                    BT_QUEST => {
                        if {
                            let (ok_value, tok_value) =
                                big2_checkPiTarget(enc, c_char_slice_from_ptr_end(target, ptr));
                            tok = tok_value;
                            ok_value
                        } == 0
                        {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_44 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.wrapping_offset(2);
                        if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                            break 'iife_ret_44 XML_TOK_PARTIAL_1;
                        }
                        if read_ptr_at(ptr, 0) as c_int == 0 && read_ptr_at(ptr, 1) as c_int == 0x3e
                        {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                            break 'iife_ret_44 tok;
                        }
                        current_block_118 = 161625824724629686;
                    }
                    _ => {
                        current_block_118 = 161625824724629686;
                    }
                }
                match current_block_118 {
                    161625824724629686 => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_44 XML_TOK_INVALID_1;
                    }
                    11190361564366887465 => {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                    _ => {}
                }
            }
            break 'iife_ret_44 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn big2_scanCdataSection(_enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_45: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            pub static CDATA_LSQB: [c_char; 6] = [
                ASCII_C as c_char,
                ASCII_D as c_char,
                ASCII_A as c_char,
                ASCII_T as c_char,
                ASCII_A as c_char,
                ASCII_LSQB as c_char,
            ];
            let mut i: c_int = 0;
            if (c_char_ptr_diff(end, ptr)) < (6i32 * 2) as c_long {
                break 'iife_ret_45 XML_TOK_PARTIAL_1;
            }
            i = 0;
            while i < 6 {
                if !(read_ptr_at(ptr, 0) as c_int == 0
                    && read_ptr_at(ptr, 1) as c_int == CDATA_LSQB[i as usize] as c_int)
                {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_45 XML_TOK_INVALID_1;
                }
                i += 1;
                ptr = ptr.wrapping_offset(2);
            }
            *nextTokPtr = ptr_to(ptr);
            break 'iife_ret_45 XML_TOK_CDATA_SECT_OPEN_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn big2_cdataSectionTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_46: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            if ptr >= end {
                break 'iife_ret_46 XML_TOK_NONE_1;
            }
            {
                let mut n: size_t = c_char_ptr_diff(end, ptr) as size_t;
                if n & (2i32 - 1) as size_t != 0 {
                    n &= !(2i32 - 1) as size_t;
                    if n == 0 {
                        break 'iife_ret_46 XML_TOK_PARTIAL_1;
                    }
                    end = ptr.wrapping_add(n);
                }
            }
            match if read_ptr_at(ptr, 0) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
            } {
                BT_RSQB => {
                    ptr = ptr.wrapping_offset(2);
                    if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                        break 'iife_ret_46 XML_TOK_PARTIAL_1;
                    }
                    if read_ptr_at(ptr, 0) as c_int == 0 && read_ptr_at(ptr, 1) as c_int == 0x5d {
                        ptr = ptr.wrapping_offset(2);
                        if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                            break 'iife_ret_46 XML_TOK_PARTIAL_1;
                        }
                        if !(read_ptr_at(ptr, 0) as c_int == 0
                            && read_ptr_at(ptr, 1) as c_int == 0x3e)
                        {
                            ptr = ptr.wrapping_offset(-(2isize));
                        } else {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                            break 'iife_ret_46 XML_TOK_CDATA_SECT_CLOSE_1;
                        }
                    }
                }
                BT_CR => {
                    ptr = ptr.wrapping_offset(2);
                    if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                        break 'iife_ret_46 XML_TOK_PARTIAL_1;
                    }
                    if (if read_ptr_at(ptr, 0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                            as c_int
                    } else {
                        unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1))
                    }) == BT_LF as c_int
                    {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_46 XML_TOK_DATA_NEWLINE_1;
                }
                BT_LF => {
                    *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                    break 'iife_ret_46 XML_TOK_DATA_NEWLINE_1;
                }
                BT_LEAD2 => {
                    if (c_char_ptr_diff(end, ptr)) < 2 {
                        break 'iife_ret_46 XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.wrapping_offset(2isize);
                }
                BT_LEAD3 => {
                    if (c_char_ptr_diff(end, ptr)) < 3 {
                        break 'iife_ret_46 XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.wrapping_offset(3isize);
                }
                BT_LEAD4 => {
                    if (c_char_ptr_diff(end, ptr)) < 4 {
                        break 'iife_ret_46 XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.wrapping_offset(4isize);
                }
                BT_NONXML | BT_MALFORM | BT_TRAIL => {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_46 XML_TOK_INVALID_1;
                }
                _ => {
                    ptr = ptr.wrapping_offset(2isize);
                }
            }
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                match if read_ptr_at(ptr, 0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                } {
                    BT_LEAD2 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_46 XML_TOK_DATA_CHARS_1;
                        }
                        ptr = ptr.wrapping_offset(2isize);
                    }
                    BT_LEAD3 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_46 XML_TOK_DATA_CHARS_1;
                        }
                        ptr = ptr.wrapping_offset(3isize);
                    }
                    BT_LEAD4 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_46 XML_TOK_DATA_CHARS_1;
                        }
                        ptr = ptr.wrapping_offset(4isize);
                    }
                    BT_NONXML | BT_MALFORM | BT_TRAIL | BT_CR | BT_LF | BT_RSQB => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_46 XML_TOK_DATA_CHARS_1;
                    }
                    _ => {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                }
            }
            *nextTokPtr = ptr_to(ptr);
            break 'iife_ret_46 XML_TOK_DATA_CHARS_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn big2_scanEndTag(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_47: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                break 'iife_ret_47 XML_TOK_PARTIAL_1;
            }
            let mut current_block_32: u64;
            match if read_ptr_at(ptr, 0) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
            } {
                BT_NONASCII => {
                    if namingBitmap[(((nmstrtPages[read_ptr_at(ptr, 0) as c_uchar as usize]
                        as c_int)
                        << 3)
                        + (read_ptr_at(ptr, 1) as c_uchar as c_int >> 5))
                        as usize]
                        & (1) << (read_ptr_at(ptr, 1) as c_uchar as c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_47 XML_TOK_INVALID_1;
                    }
                    current_block_32 = 12738221189273011712;
                }
                BT_NMSTRT | BT_HEX => {
                    current_block_32 = 12738221189273011712;
                }
                BT_LEAD2 => {
                    if (c_char_ptr_diff(end, ptr)) < 2 {
                        break 'iife_ret_47 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_47 XML_TOK_INVALID_1;
                }
                BT_LEAD3 => {
                    if (c_char_ptr_diff(end, ptr)) < 3 {
                        break 'iife_ret_47 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_47 XML_TOK_INVALID_1;
                }
                BT_LEAD4 => {
                    if (c_char_ptr_diff(end, ptr)) < 4 {
                        break 'iife_ret_47 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_47 XML_TOK_INVALID_1;
                }
                _ => {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_47 XML_TOK_INVALID_1;
                }
            }
            if current_block_32 == 12738221189273011712 {
                ptr = ptr.wrapping_offset(2isize);
            }
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                let mut current_block_73: u64;
                match if read_ptr_at(ptr, 0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((namePages[read_ptr_at(ptr, 0) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (read_ptr_at(ptr, 1) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (read_ptr_at(ptr, 1) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_47 XML_TOK_INVALID_1;
                        }
                        current_block_73 = 1281007054303163758;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_73 = 1281007054303163758;
                    }
                    BT_LEAD2 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            break 'iife_ret_47 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_47 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            break 'iife_ret_47 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_47 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            break 'iife_ret_47 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_47 XML_TOK_INVALID_1;
                    }
                    BT_S | BT_CR | BT_LF => {
                        ptr = ptr.wrapping_offset(2);
                        while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                            match if read_ptr_at(ptr, 0) as c_int == 0 {
                                as_normal_encoding(enc).type_0
                                    [read_ptr_at(ptr, 1) as c_uchar as usize]
                                    as c_uint
                            } else {
                                unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1))
                                    as c_uint
                            } {
                                BT_S | BT_CR | BT_LF => {}
                                BT_GT => {
                                    *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                                    break 'iife_ret_47 XML_TOK_END_TAG_1;
                                }
                                _ => {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_47 XML_TOK_INVALID_1;
                                }
                            }
                            ptr = ptr.wrapping_offset(2);
                        }
                        break 'iife_ret_47 XML_TOK_PARTIAL_1;
                    }
                    BT_COLON_0 => {
                        ptr = ptr.wrapping_offset(2);
                        current_block_73 = 981995395831942902;
                    }
                    BT_GT => {
                        *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                        break 'iife_ret_47 XML_TOK_END_TAG_1;
                    }
                    _ => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_47 XML_TOK_INVALID_1;
                    }
                }
                if current_block_73 == 1281007054303163758 {
                    ptr = ptr.wrapping_offset(2isize);
                }
            }
            break 'iife_ret_47 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn big2_scanHexCharRef(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_48: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            if c_char_ptr_diff(end, ptr) >= 2 as c_long {
                match if read_ptr_at(ptr, 0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                } {
                    BT_DIGIT | BT_HEX => {}
                    _ => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_48 XML_TOK_INVALID_1;
                    }
                }
                ptr = ptr.wrapping_offset(2);
                while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                    match if read_ptr_at(ptr, 0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                            as c_uint
                    } else {
                        unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                    } {
                        BT_DIGIT | BT_HEX => {}
                        BT_SEMI => {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                            break 'iife_ret_48 XML_TOK_CHAR_REF_1;
                        }
                        _ => {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_48 XML_TOK_INVALID_1;
                        }
                    }
                    ptr = ptr.wrapping_offset(2);
                }
            }
            break 'iife_ret_48 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn big2_scanCharRef(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_49: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let c_char_slice_from_ptr_end = |ptr: usize, end: usize| &input[ptr..end];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            if c_char_ptr_diff(end, ptr) >= 2 as c_long {
                if read_ptr_at(ptr, 0) as c_int == 0 && read_ptr_at(ptr, 1) as c_int == 0x78 {
                    break 'iife_ret_49 ({
                        let (tok_value, next_tok_value) = big2_scanHexCharRef(
                            enc,
                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                        );
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                match if read_ptr_at(ptr, 0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                } {
                    BT_DIGIT => {}
                    _ => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_49 XML_TOK_INVALID_1;
                    }
                }
                ptr = ptr.wrapping_offset(2);
                while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                    match if read_ptr_at(ptr, 0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                            as c_uint
                    } else {
                        unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                    } {
                        BT_DIGIT => {}
                        BT_SEMI => {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                            break 'iife_ret_49 XML_TOK_CHAR_REF_1;
                        }
                        _ => {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_49 XML_TOK_INVALID_1;
                        }
                    }
                    ptr = ptr.wrapping_offset(2);
                }
            }
            break 'iife_ret_49 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn big2_scanRef(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_50: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let c_char_slice_from_ptr_end = |ptr: usize, end: usize| &input[ptr..end];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                break 'iife_ret_50 XML_TOK_PARTIAL_1;
            }
            let mut current_block_33: u64;
            match if read_ptr_at(ptr, 0) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
            } {
                BT_NONASCII => {
                    if namingBitmap[(((nmstrtPages[read_ptr_at(ptr, 0) as c_uchar as usize]
                        as c_int)
                        << 3)
                        + (read_ptr_at(ptr, 1) as c_uchar as c_int >> 5))
                        as usize]
                        & (1) << (read_ptr_at(ptr, 1) as c_uchar as c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_50 XML_TOK_INVALID_1;
                    }
                    current_block_33 = 17794167657114565097;
                }
                BT_NMSTRT | BT_HEX => {
                    current_block_33 = 17794167657114565097;
                }
                BT_LEAD2 => {
                    if (c_char_ptr_diff(end, ptr)) < 2 {
                        break 'iife_ret_50 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_50 XML_TOK_INVALID_1;
                }
                BT_LEAD3 => {
                    if (c_char_ptr_diff(end, ptr)) < 3 {
                        break 'iife_ret_50 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_50 XML_TOK_INVALID_1;
                }
                BT_LEAD4 => {
                    if (c_char_ptr_diff(end, ptr)) < 4 {
                        break 'iife_ret_50 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_50 XML_TOK_INVALID_1;
                }
                BT_NUM => {
                    break 'iife_ret_50 ({
                        let (tok_value, next_tok_value) = big2_scanCharRef(
                            enc,
                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                        );
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                _ => {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_50 XML_TOK_INVALID_1;
                }
            }
            if current_block_33 == 17794167657114565097 {
                ptr = ptr.wrapping_offset(2isize);
            }
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                let mut current_block_64: u64;
                match if read_ptr_at(ptr, 0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((namePages[read_ptr_at(ptr, 0) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (read_ptr_at(ptr, 1) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (read_ptr_at(ptr, 1) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_50 XML_TOK_INVALID_1;
                        }
                        current_block_64 = 17251590314240005670;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_64 = 17251590314240005670;
                    }
                    BT_LEAD2 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            break 'iife_ret_50 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_50 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            break 'iife_ret_50 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_50 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            break 'iife_ret_50 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_50 XML_TOK_INVALID_1;
                    }
                    BT_SEMI => {
                        *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                        break 'iife_ret_50 XML_TOK_ENTITY_REF_1;
                    }
                    _ => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_50 XML_TOK_INVALID_1;
                    }
                }
                if current_block_64 == 17251590314240005670 {
                    ptr = ptr.wrapping_offset(2isize);
                }
            }
            break 'iife_ret_50 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn big2_scanAtts(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_51: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let c_char_slice_from_ptr_end = |ptr: usize, end: usize| &input[ptr..end];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            let mut hadColon: c_int = 0;
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                let mut current_block_186: u64;
                match if read_ptr_at(ptr, 0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((namePages[read_ptr_at(ptr, 0) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (read_ptr_at(ptr, 1) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (read_ptr_at(ptr, 1) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_51 XML_TOK_INVALID_1;
                        }
                        current_block_186 = 6092917267242331817;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_186 = 6092917267242331817;
                    }
                    BT_LEAD2 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_51 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_51 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_51 XML_TOK_INVALID_1;
                    }
                    BT_COLON_0 => {
                        if hadColon != 0 {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_51 XML_TOK_INVALID_1;
                        }
                        hadColon = 1;
                        ptr = ptr.wrapping_offset(2);
                        if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                            break 'iife_ret_51 XML_TOK_PARTIAL_1;
                        }
                        let mut current_block_64: u64;
                        match if read_ptr_at(ptr, 0) as c_int == 0 {
                            as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                                as c_uint
                        } else {
                            unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                        } {
                            BT_NONASCII => {
                                if namingBitmap[(((nmstrtPages
                                    [read_ptr_at(ptr, 0) as c_uchar as usize]
                                    as c_int)
                                    << 3)
                                    + (read_ptr_at(ptr, 1) as c_uchar as c_int >> 5))
                                    as usize]
                                    & (1) << (read_ptr_at(ptr, 1) as c_uchar as c_int & 0x1f)
                                    == 0
                                {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_51 XML_TOK_INVALID_1;
                                }
                                current_block_64 = 6604085902723260545;
                            }
                            BT_NMSTRT | BT_HEX => {
                                current_block_64 = 6604085902723260545;
                            }
                            BT_LEAD2 => {
                                if (c_char_ptr_diff(end, ptr)) < 2 {
                                    break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                                }
                                *nextTokPtr = ptr_to(ptr);
                                break 'iife_ret_51 XML_TOK_INVALID_1;
                            }
                            BT_LEAD3 => {
                                if (c_char_ptr_diff(end, ptr)) < 3 {
                                    break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                                }
                                *nextTokPtr = ptr_to(ptr);
                                break 'iife_ret_51 XML_TOK_INVALID_1;
                            }
                            BT_LEAD4 => {
                                if (c_char_ptr_diff(end, ptr)) < 4 {
                                    break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                                }
                                *nextTokPtr = ptr_to(ptr);
                                break 'iife_ret_51 XML_TOK_INVALID_1;
                            }
                            _ => {
                                *nextTokPtr = ptr_to(ptr);
                                break 'iife_ret_51 XML_TOK_INVALID_1;
                            }
                        }
                        if current_block_64 == 6604085902723260545 {
                            ptr = ptr.wrapping_offset(2isize);
                        }
                        current_block_186 = 1634947208139838470;
                    }
                    BT_S | BT_CR | BT_LF => {
                        loop {
                            let mut t: c_int = 0;
                            ptr = ptr.wrapping_offset(2);
                            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                break 'iife_ret_51 XML_TOK_PARTIAL_1;
                            }
                            t = if read_ptr_at(ptr, 0) as c_int == 0 {
                                as_normal_encoding(enc).type_0
                                    [read_ptr_at(ptr, 1) as c_uchar as usize]
                                    as c_int
                            } else {
                                unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1))
                            };
                            if t == BT_EQUALS as c_int {
                                break;
                            }
                            match t {
                                21 | 10 | 9 => {}
                                _ => {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_51 XML_TOK_INVALID_1;
                                }
                            }
                        }
                        current_block_186 = 10853015579903106591;
                    }
                    BT_EQUALS => {
                        current_block_186 = 10853015579903106591;
                    }
                    _ => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_51 XML_TOK_INVALID_1;
                    }
                }
                match current_block_186 {
                    10853015579903106591 => {
                        let mut open: c_int = 0;
                        hadColon = 0;
                        loop {
                            ptr = ptr.wrapping_offset(2);
                            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                break 'iife_ret_51 XML_TOK_PARTIAL_1;
                            }
                            open = if read_ptr_at(ptr, 0) as c_int == 0 {
                                as_normal_encoding(enc).type_0
                                    [read_ptr_at(ptr, 1) as c_uchar as usize]
                                    as c_int
                            } else {
                                unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1))
                            };
                            if open == BT_QUOT as c_int || open == BT_APOS as c_int {
                                break;
                            }
                            match open {
                                21 | 10 | 9 => {}
                                _ => {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_51 XML_TOK_INVALID_1;
                                }
                            }
                        }
                        ptr = ptr.wrapping_offset(2);
                        loop {
                            let mut t_0: c_int = 0;
                            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                break 'iife_ret_51 XML_TOK_PARTIAL_1;
                            }
                            t_0 = if read_ptr_at(ptr, 0) as c_int == 0 {
                                as_normal_encoding(enc).type_0
                                    [read_ptr_at(ptr, 1) as c_uchar as usize]
                                    as c_int
                            } else {
                                unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1))
                            };
                            if t_0 == open {
                                break;
                            }
                            match t_0 {
                                5 => {
                                    if (c_char_ptr_diff(end, ptr)) < 2 {
                                        break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    ptr = ptr.wrapping_offset(2isize);
                                }
                                6 => {
                                    if (c_char_ptr_diff(end, ptr)) < 3 {
                                        break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    ptr = ptr.wrapping_offset(3isize);
                                }
                                7 => {
                                    if (c_char_ptr_diff(end, ptr)) < 4 {
                                        break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    ptr = ptr.wrapping_offset(4isize);
                                }
                                0 | 1 | 8 => {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_51 XML_TOK_INVALID_1;
                                }
                                3 => {
                                    let mut tok: c_int = {
                                        let (tok_value, next_tok_value) = big2_scanRef(
                                            enc,
                                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2), end),
                                        );
                                        ptr = super::c_char_ptr_diff(next_tok_value, input.as_ptr())
                                            as usize;
                                        tok_value
                                    };
                                    if tok <= 0 {
                                        if tok == XML_TOK_INVALID_1 {
                                            *nextTokPtr = ptr_to(ptr);
                                        }
                                        break 'iife_ret_51 tok;
                                    }
                                }
                                2 => {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_51 XML_TOK_INVALID_1;
                                }
                                _ => {
                                    ptr = ptr.wrapping_offset(2isize);
                                }
                            }
                        }
                        ptr = ptr.wrapping_offset(2);
                        if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                            break 'iife_ret_51 XML_TOK_PARTIAL_1;
                        }
                        match if read_ptr_at(ptr, 0) as c_int == 0 {
                            as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                                as c_uint
                        } else {
                            unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                        } {
                            BT_S | BT_CR | BT_LF => {
                                loop {
                                    ptr = ptr.wrapping_offset(2);
                                    if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                        break 'iife_ret_51 XML_TOK_PARTIAL_1;
                                    }
                                    match if read_ptr_at(ptr, 0) as c_int == 0 {
                                        as_normal_encoding(enc).type_0
                                            [read_ptr_at(ptr, 1) as c_uchar as usize]
                                            as c_uint
                                    } else {
                                        unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1))
                                            as c_uint
                                    } {
                                        BT_NONASCII => {
                                            if namingBitmap[(((nmstrtPages
                                                [read_ptr_at(ptr, 0) as c_uchar as usize]
                                                as c_int)
                                                << 3)
                                                + (read_ptr_at(ptr, 1) as c_uchar as c_int >> 5))
                                                as usize]
                                                & (1)
                                                    << (read_ptr_at(ptr, 1) as c_uchar as c_int
                                                        & 0x1f)
                                                == 0
                                            {
                                                *nextTokPtr = ptr_to(ptr);
                                                break 'iife_ret_51 XML_TOK_INVALID_1;
                                            }
                                            current_block_186 = 7794494472231011433;
                                            break;
                                        }
                                        BT_NMSTRT | BT_HEX => {
                                            current_block_186 = 7794494472231011433;
                                            break;
                                        }
                                        BT_LEAD2 => {
                                            if (c_char_ptr_diff(end, ptr)) < 2 {
                                                break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                                            }
                                            *nextTokPtr = ptr_to(ptr);
                                            break 'iife_ret_51 XML_TOK_INVALID_1;
                                        }
                                        BT_LEAD3 => {
                                            if (c_char_ptr_diff(end, ptr)) < 3 {
                                                break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                                            }
                                            *nextTokPtr = ptr_to(ptr);
                                            break 'iife_ret_51 XML_TOK_INVALID_1;
                                        }
                                        BT_LEAD4 => {
                                            if (c_char_ptr_diff(end, ptr)) < 4 {
                                                break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                                            }
                                            *nextTokPtr = ptr_to(ptr);
                                            break 'iife_ret_51 XML_TOK_INVALID_1;
                                        }
                                        BT_S | BT_CR | BT_LF => {}
                                        BT_GT => {
                                            current_block_186 = 1783713129665224809;
                                            break;
                                        }
                                        BT_SOL => {
                                            current_block_186 = 18153789983347219713;
                                            break;
                                        }
                                        _ => {
                                            *nextTokPtr = ptr_to(ptr);
                                            break 'iife_ret_51 XML_TOK_INVALID_1;
                                        }
                                    }
                                }
                                match current_block_186 {
                                    1783713129665224809 => {}
                                    18153789983347219713 => {}
                                    1634947208139838470 => {}
                                    _ => {
                                        ptr = ptr.wrapping_offset(2);
                                        current_block_186 = 1634947208139838470;
                                    }
                                }
                            }
                            BT_SOL => {
                                current_block_186 = 18153789983347219713;
                            }
                            BT_GT => {
                                current_block_186 = 1783713129665224809;
                            }
                            _ => {
                                *nextTokPtr = ptr_to(ptr);
                                break 'iife_ret_51 XML_TOK_INVALID_1;
                            }
                        }
                        match current_block_186 {
                            1634947208139838470 => {}
                            _ => match current_block_186 {
                                18153789983347219713 => {
                                    ptr = ptr.wrapping_offset(2);
                                    if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                        break 'iife_ret_51 XML_TOK_PARTIAL_1;
                                    }
                                    if !(read_ptr_at(ptr, 0) as c_int == 0
                                        && read_ptr_at(ptr, 1) as c_int == 0x3e)
                                    {
                                        *nextTokPtr = ptr_to(ptr);
                                        break 'iife_ret_51 XML_TOK_INVALID_1;
                                    }
                                    *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                                    break 'iife_ret_51 XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1;
                                }
                                _ => {
                                    *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                                    break 'iife_ret_51 XML_TOK_START_TAG_WITH_ATTS_1;
                                }
                            },
                        }
                    }
                    6092917267242331817 => {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                    _ => {}
                }
            }
            break 'iife_ret_51 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn big2_scanLt(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_52: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let c_char_slice_from_ptr_end = |ptr: usize, end: usize| &input[ptr..end];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            let mut hadColon: c_int = 0;
            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                break 'iife_ret_52 XML_TOK_PARTIAL_1;
            }
            let mut current_block_45: u64;
            match if read_ptr_at(ptr, 0) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
            } {
                BT_NONASCII => {
                    if namingBitmap[(((nmstrtPages[read_ptr_at(ptr, 0) as c_uchar as usize]
                        as c_int)
                        << 3)
                        + (read_ptr_at(ptr, 1) as c_uchar as c_int >> 5))
                        as usize]
                        & (1) << (read_ptr_at(ptr, 1) as c_uchar as c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_52 XML_TOK_INVALID_1;
                    }
                    current_block_45 = 6477200489819026004;
                }
                BT_NMSTRT | BT_HEX => {
                    current_block_45 = 6477200489819026004;
                }
                BT_LEAD2 => {
                    if (c_char_ptr_diff(end, ptr)) < 2 {
                        break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_52 XML_TOK_INVALID_1;
                }
                BT_LEAD3 => {
                    if (c_char_ptr_diff(end, ptr)) < 3 {
                        break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_52 XML_TOK_INVALID_1;
                }
                BT_LEAD4 => {
                    if (c_char_ptr_diff(end, ptr)) < 4 {
                        break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_52 XML_TOK_INVALID_1;
                }
                BT_EXCL => {
                    ptr = ptr.wrapping_offset(2);
                    if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                        break 'iife_ret_52 XML_TOK_PARTIAL_1;
                    }
                    match if read_ptr_at(ptr, 0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                            as c_uint
                    } else {
                        unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                    } {
                        BT_MINUS => {
                            break 'iife_ret_52 ({
                                let (tok_value, next_tok_value) = big2_scanComment(
                                    enc,
                                    c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                                );
                                *nextTokPtr = next_tok_value;
                                tok_value
                            });
                        }
                        BT_LSQB => {
                            break 'iife_ret_52 ({
                                let (tok_value, next_tok_value) = big2_scanCdataSection(
                                    enc,
                                    c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                                );
                                *nextTokPtr = next_tok_value;
                                tok_value
                            });
                        }
                        _ => {}
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_52 XML_TOK_INVALID_1;
                }
                BT_QUEST => {
                    break 'iife_ret_52 ({
                        let (tok_value, next_tok_value) = big2_scanPi(
                            enc,
                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                        );
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_SOL => {
                    break 'iife_ret_52 ({
                        let (tok_value, next_tok_value) = big2_scanEndTag(
                            enc,
                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                        );
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                _ => {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_52 XML_TOK_INVALID_1;
                }
            }
            if current_block_45 == 6477200489819026004 {
                ptr = ptr.wrapping_offset(2isize);
            }
            hadColon = 0;
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                let mut current_block_161: u64;
                match if read_ptr_at(ptr, 0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((namePages[read_ptr_at(ptr, 0) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (read_ptr_at(ptr, 1) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (read_ptr_at(ptr, 1) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_52 XML_TOK_INVALID_1;
                        }
                        current_block_161 = 18151815167355992796;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_161 = 18151815167355992796;
                    }
                    BT_LEAD2 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_52 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_52 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_52 XML_TOK_INVALID_1;
                    }
                    BT_COLON_0 => {
                        if hadColon != 0 {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_52 XML_TOK_INVALID_1;
                        }
                        hadColon = 1;
                        ptr = ptr.wrapping_offset(2);
                        if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                            break 'iife_ret_52 XML_TOK_PARTIAL_1;
                        }
                        let mut current_block_112: u64;
                        match if read_ptr_at(ptr, 0) as c_int == 0 {
                            as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                                as c_uint
                        } else {
                            unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                        } {
                            BT_NONASCII => {
                                if namingBitmap[(((nmstrtPages
                                    [read_ptr_at(ptr, 0) as c_uchar as usize]
                                    as c_int)
                                    << 3)
                                    + (read_ptr_at(ptr, 1) as c_uchar as c_int >> 5))
                                    as usize]
                                    & (1) << (read_ptr_at(ptr, 1) as c_uchar as c_int & 0x1f)
                                    == 0
                                {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_52 XML_TOK_INVALID_1;
                                }
                                current_block_112 = 16337619596932156899;
                            }
                            BT_NMSTRT | BT_HEX => {
                                current_block_112 = 16337619596932156899;
                            }
                            BT_LEAD2 => {
                                if (c_char_ptr_diff(end, ptr)) < 2 {
                                    break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                                }
                                *nextTokPtr = ptr_to(ptr);
                                break 'iife_ret_52 XML_TOK_INVALID_1;
                            }
                            BT_LEAD3 => {
                                if (c_char_ptr_diff(end, ptr)) < 3 {
                                    break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                                }
                                *nextTokPtr = ptr_to(ptr);
                                break 'iife_ret_52 XML_TOK_INVALID_1;
                            }
                            BT_LEAD4 => {
                                if (c_char_ptr_diff(end, ptr)) < 4 {
                                    break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                                }
                                *nextTokPtr = ptr_to(ptr);
                                break 'iife_ret_52 XML_TOK_INVALID_1;
                            }
                            _ => {
                                *nextTokPtr = ptr_to(ptr);
                                break 'iife_ret_52 XML_TOK_INVALID_1;
                            }
                        }
                        if current_block_112 == 16337619596932156899 {
                            ptr = ptr.wrapping_offset(2isize);
                        }
                        current_block_161 = 14714495436747744489;
                    }
                    BT_S | BT_CR | BT_LF => {
                        ptr = ptr.wrapping_offset(2);
                        loop {
                            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                current_block_161 = 13215501469961642988;
                                break;
                            }
                            match if read_ptr_at(ptr, 0) as c_int == 0 {
                                as_normal_encoding(enc).type_0
                                    [read_ptr_at(ptr, 1) as c_uchar as usize]
                                    as c_uint
                            } else {
                                unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1))
                                    as c_uint
                            } {
                                BT_NONASCII => {
                                    if namingBitmap[(((nmstrtPages
                                        [read_ptr_at(ptr, 0) as c_uchar as usize]
                                        as c_int)
                                        << 3)
                                        + (read_ptr_at(ptr, 1) as c_uchar as c_int >> 5))
                                        as usize]
                                        & (1) << (read_ptr_at(ptr, 1) as c_uchar as c_int & 0x1f)
                                        == 0
                                    {
                                        *nextTokPtr = ptr_to(ptr);
                                        break 'iife_ret_52 XML_TOK_INVALID_1;
                                    }
                                    current_block_161 = 11066148936714919733;
                                }
                                BT_NMSTRT | BT_HEX => {
                                    current_block_161 = 11066148936714919733;
                                }
                                BT_LEAD2 => {
                                    if (c_char_ptr_diff(end, ptr)) < 2 {
                                        break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_52 XML_TOK_INVALID_1;
                                }
                                BT_LEAD3 => {
                                    if (c_char_ptr_diff(end, ptr)) < 3 {
                                        break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_52 XML_TOK_INVALID_1;
                                }
                                BT_LEAD4 => {
                                    if (c_char_ptr_diff(end, ptr)) < 4 {
                                        break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_52 XML_TOK_INVALID_1;
                                }
                                BT_GT => {
                                    current_block_161 = 13089361350718158941;
                                    break;
                                }
                                BT_SOL => {
                                    current_block_161 = 11384015785330443424;
                                    break;
                                }
                                BT_S | BT_CR | BT_LF => {
                                    ptr = ptr.wrapping_offset(2);
                                    continue;
                                }
                                _ => {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_52 XML_TOK_INVALID_1;
                                }
                            }
                            if current_block_161 == 11066148936714919733 {
                                ptr = ptr.wrapping_offset(2isize);
                            }
                            break 'iife_ret_52 ({
                                let (tok_value, next_tok_value) =
                                    big2_scanAtts(enc, c_char_slice_from_ptr_end(ptr, end));
                                *nextTokPtr = next_tok_value;
                                tok_value
                            });
                        }
                        match current_block_161 {
                            13089361350718158941 => {}
                            11384015785330443424 => {}
                            _ => break 'iife_ret_52 XML_TOK_PARTIAL_1,
                        }
                    }
                    BT_GT => {
                        current_block_161 = 13089361350718158941;
                    }
                    BT_SOL => {
                        current_block_161 = 11384015785330443424;
                    }
                    _ => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_52 XML_TOK_INVALID_1;
                    }
                }
                match current_block_161 {
                    11384015785330443424 => {
                        ptr = ptr.wrapping_offset(2);
                        if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                            break 'iife_ret_52 XML_TOK_PARTIAL_1;
                        }
                        if !(read_ptr_at(ptr, 0) as c_int == 0
                            && read_ptr_at(ptr, 1) as c_int == 0x3e)
                        {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_52 XML_TOK_INVALID_1;
                        }
                        *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                        break 'iife_ret_52 XML_TOK_EMPTY_ELEMENT_NO_ATTS_1;
                    }
                    13089361350718158941 => {
                        *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                        break 'iife_ret_52 XML_TOK_START_TAG_NO_ATTS_1;
                    }
                    18151815167355992796 => {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                    _ => {}
                }
            }
            break 'iife_ret_52 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn big2_contentTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_53: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let c_char_slice_from_ptr_end = |ptr: usize, end: usize| &input[ptr..end];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            if ptr >= end {
                break 'iife_ret_53 XML_TOK_NONE_1;
            }
            {
                let mut n: size_t = c_char_ptr_diff(end, ptr) as size_t;
                if n & (2i32 - 1) as size_t != 0 {
                    n &= !(2i32 - 1) as size_t;
                    if n == 0 {
                        break 'iife_ret_53 XML_TOK_PARTIAL_1;
                    }
                    end = ptr.wrapping_add(n);
                }
            }
            match if read_ptr_at(ptr, 0) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
            } {
                BT_LT => {
                    break 'iife_ret_53 ({
                        let (tok_value, next_tok_value) = big2_scanLt(
                            enc,
                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                        );
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_AMP => {
                    break 'iife_ret_53 ({
                        let (tok_value, next_tok_value) = big2_scanRef(
                            enc,
                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                        );
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_CR => {
                    ptr = ptr.wrapping_offset(2);
                    if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                        break 'iife_ret_53 XML_TOK_TRAILING_CR_1;
                    }
                    if (if read_ptr_at(ptr, 0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                            as c_int
                    } else {
                        unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1))
                    }) == BT_LF as c_int
                    {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_53 XML_TOK_DATA_NEWLINE_1;
                }
                BT_LF => {
                    *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                    break 'iife_ret_53 XML_TOK_DATA_NEWLINE_1;
                }
                BT_RSQB => {
                    ptr = ptr.wrapping_offset(2);
                    if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                        break 'iife_ret_53 XML_TOK_TRAILING_RSQB_1;
                    }
                    if read_ptr_at(ptr, 0) as c_int == 0 && read_ptr_at(ptr, 1) as c_int == 0x5d {
                        ptr = ptr.wrapping_offset(2);
                        if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                            break 'iife_ret_53 XML_TOK_TRAILING_RSQB_1;
                        }
                        if !(read_ptr_at(ptr, 0) as c_int == 0
                            && read_ptr_at(ptr, 1) as c_int == 0x3e)
                        {
                            ptr = ptr.wrapping_offset(-(2isize));
                        } else {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_53 XML_TOK_INVALID_1;
                        }
                    }
                }
                BT_LEAD2 => {
                    if (c_char_ptr_diff(end, ptr)) < 2 {
                        break 'iife_ret_53 XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.wrapping_offset(2isize);
                }
                BT_LEAD3 => {
                    if (c_char_ptr_diff(end, ptr)) < 3 {
                        break 'iife_ret_53 XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.wrapping_offset(3isize);
                }
                BT_LEAD4 => {
                    if (c_char_ptr_diff(end, ptr)) < 4 {
                        break 'iife_ret_53 XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.wrapping_offset(4isize);
                }
                BT_NONXML | BT_MALFORM | BT_TRAIL => {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_53 XML_TOK_INVALID_1;
                }
                _ => {
                    ptr = ptr.wrapping_offset(2isize);
                }
            }
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                let mut current_block_76: u64;
                match if read_ptr_at(ptr, 0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                } {
                    BT_LEAD2 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_53 XML_TOK_DATA_CHARS_1;
                        }
                        ptr = ptr.wrapping_offset(2);
                        current_block_76 = 7158658067966855297;
                    }
                    BT_LEAD3 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_53 XML_TOK_DATA_CHARS_1;
                        }
                        ptr = ptr.wrapping_offset(3);
                        current_block_76 = 7158658067966855297;
                    }
                    BT_LEAD4 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_53 XML_TOK_DATA_CHARS_1;
                        }
                        ptr = ptr.wrapping_offset(4);
                        current_block_76 = 7158658067966855297;
                    }
                    BT_RSQB => {
                        if c_char_ptr_diff(end, ptr) >= (2i32 * 2) as c_long {
                            if !(read_ptr_at(ptr.wrapping_offset(2), 0) as c_int == 0
                                && read_ptr_at(ptr.wrapping_offset(2), 1) as c_int == 0x5d)
                            {
                                ptr = ptr.wrapping_offset(2);
                                current_block_76 = 7158658067966855297;
                            } else if c_char_ptr_diff(end, ptr) >= (3i32 * 2) as c_long {
                                if !(read_ptr_at(ptr.wrapping_offset((2i32 * 2) as isize), 0)
                                    as c_int
                                    == 0
                                    && read_ptr_at(ptr.wrapping_offset((2i32 * 2) as isize), 1)
                                        as c_int
                                        == 0x3e)
                                {
                                    ptr = ptr.wrapping_offset(2isize);
                                } else {
                                    *nextTokPtr = ptr_to(ptr.wrapping_offset((2i32 * 2) as isize));
                                    break 'iife_ret_53 XML_TOK_INVALID_1;
                                }
                                current_block_76 = 7158658067966855297;
                            } else {
                                current_block_76 = 11890188771060868767;
                            }
                        } else {
                            current_block_76 = 11890188771060868767;
                        }
                    }
                    BT_AMP | BT_LT | BT_NONXML | BT_MALFORM | BT_TRAIL | BT_CR | BT_LF => {
                        current_block_76 = 11890188771060868767;
                    }
                    _ => {
                        ptr = ptr.wrapping_offset(2);
                        current_block_76 = 7158658067966855297;
                    }
                }
                match current_block_76 {
                    7158658067966855297 => {}
                    _ => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_53 XML_TOK_DATA_CHARS_1;
                    }
                }
            }
            *nextTokPtr = ptr_to(ptr);
            break 'iife_ret_53 XML_TOK_DATA_CHARS_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn big2_scanPercent(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_54: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                break 'iife_ret_54 XML_TOK_PARTIAL_1;
            }
            let mut current_block_34: u64;
            match if read_ptr_at(ptr, 0) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
            } {
                BT_NONASCII => {
                    if namingBitmap[(((nmstrtPages[read_ptr_at(ptr, 0) as c_uchar as usize]
                        as c_int)
                        << 3)
                        + (read_ptr_at(ptr, 1) as c_uchar as c_int >> 5))
                        as usize]
                        & (1) << (read_ptr_at(ptr, 1) as c_uchar as c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_54 XML_TOK_INVALID_1;
                    }
                    current_block_34 = 9652455934050855438;
                }
                BT_NMSTRT | BT_HEX => {
                    current_block_34 = 9652455934050855438;
                }
                BT_LEAD2 => {
                    if (c_char_ptr_diff(end, ptr)) < 2 {
                        break 'iife_ret_54 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_54 XML_TOK_INVALID_1;
                }
                BT_LEAD3 => {
                    if (c_char_ptr_diff(end, ptr)) < 3 {
                        break 'iife_ret_54 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_54 XML_TOK_INVALID_1;
                }
                BT_LEAD4 => {
                    if (c_char_ptr_diff(end, ptr)) < 4 {
                        break 'iife_ret_54 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_54 XML_TOK_INVALID_1;
                }
                BT_S | BT_LF | BT_CR | BT_PERCNT => {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_54 XML_TOK_PERCENT_1;
                }
                _ => {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_54 XML_TOK_INVALID_1;
                }
            }
            if current_block_34 == 9652455934050855438 {
                ptr = ptr.wrapping_offset(2isize);
            }
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                let mut current_block_65: u64;
                match if read_ptr_at(ptr, 0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((namePages[read_ptr_at(ptr, 0) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (read_ptr_at(ptr, 1) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (read_ptr_at(ptr, 1) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_54 XML_TOK_INVALID_1;
                        }
                        current_block_65 = 3947837075391501242;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_65 = 3947837075391501242;
                    }
                    BT_LEAD2 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            break 'iife_ret_54 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_54 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            break 'iife_ret_54 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_54 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            break 'iife_ret_54 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_54 XML_TOK_INVALID_1;
                    }
                    BT_SEMI => {
                        *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                        break 'iife_ret_54 XML_TOK_PARAM_ENTITY_REF_1;
                    }
                    _ => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_54 XML_TOK_INVALID_1;
                    }
                }
                if current_block_65 == 3947837075391501242 {
                    ptr = ptr.wrapping_offset(2isize);
                }
            }
            break 'iife_ret_54 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn big2_scanPoundName(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_55: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                break 'iife_ret_55 XML_TOK_PARTIAL_1;
            }
            let mut current_block_32: u64;
            match if read_ptr_at(ptr, 0) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
            } {
                BT_NONASCII => {
                    if namingBitmap[(((nmstrtPages[read_ptr_at(ptr, 0) as c_uchar as usize]
                        as c_int)
                        << 3)
                        + (read_ptr_at(ptr, 1) as c_uchar as c_int >> 5))
                        as usize]
                        & (1) << (read_ptr_at(ptr, 1) as c_uchar as c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_55 XML_TOK_INVALID_1;
                    }
                    current_block_32 = 12219479933348349998;
                }
                BT_NMSTRT | BT_HEX => {
                    current_block_32 = 12219479933348349998;
                }
                BT_LEAD2 => {
                    if (c_char_ptr_diff(end, ptr)) < 2 {
                        break 'iife_ret_55 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_55 XML_TOK_INVALID_1;
                }
                BT_LEAD3 => {
                    if (c_char_ptr_diff(end, ptr)) < 3 {
                        break 'iife_ret_55 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_55 XML_TOK_INVALID_1;
                }
                BT_LEAD4 => {
                    if (c_char_ptr_diff(end, ptr)) < 4 {
                        break 'iife_ret_55 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_55 XML_TOK_INVALID_1;
                }
                _ => {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_55 XML_TOK_INVALID_1;
                }
            }
            if current_block_32 == 12219479933348349998 {
                ptr = ptr.wrapping_offset(2isize);
            }
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                let mut current_block_63: u64;
                match if read_ptr_at(ptr, 0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((namePages[read_ptr_at(ptr, 0) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (read_ptr_at(ptr, 1) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (read_ptr_at(ptr, 1) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_55 XML_TOK_INVALID_1;
                        }
                        current_block_63 = 1647491770914889697;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_63 = 1647491770914889697;
                    }
                    BT_LEAD2 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            break 'iife_ret_55 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_55 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            break 'iife_ret_55 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_55 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            break 'iife_ret_55 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_55 XML_TOK_INVALID_1;
                    }
                    BT_CR | BT_LF | BT_S | BT_RPAR | BT_GT | BT_PERCNT | BT_VERBAR => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_55 XML_TOK_POUND_NAME_1;
                    }
                    _ => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_55 XML_TOK_INVALID_1;
                    }
                }
                if current_block_63 == 1647491770914889697 {
                    ptr = ptr.wrapping_offset(2isize);
                }
            }
            break 'iife_ret_55 -XML_TOK_POUND_NAME_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn big2_scanLit(mut open: c_int, enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_56: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                let mut t: c_int = if read_ptr_at(ptr, 0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize] as c_int
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1))
                };
                match t {
                    5 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            break 'iife_ret_56 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.wrapping_offset(2isize);
                    }
                    6 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            break 'iife_ret_56 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.wrapping_offset(3isize);
                    }
                    7 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            break 'iife_ret_56 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.wrapping_offset(4isize);
                    }
                    0 | 1 | 8 => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_56 XML_TOK_INVALID_1;
                    }
                    12 | 13 => {
                        ptr = ptr.wrapping_offset(2);
                        if t == open {
                            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                break 'iife_ret_56 -XML_TOK_LITERAL_1;
                            }
                            *nextTokPtr = ptr_to(ptr);
                            match if read_ptr_at(ptr, 0) as c_int == 0 {
                                as_normal_encoding(enc).type_0
                                    [read_ptr_at(ptr, 1) as c_uchar as usize]
                                    as c_uint
                            } else {
                                unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1))
                                    as c_uint
                            } {
                                BT_S | BT_CR | BT_LF | BT_GT | BT_PERCNT | BT_LSQB => {
                                    break 'iife_ret_56 XML_TOK_LITERAL_1
                                }
                                _ => break 'iife_ret_56 XML_TOK_INVALID_1,
                            }
                        }
                    }
                    _ => {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                }
            }
            break 'iife_ret_56 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn big2_prologTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_57: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let c_char_slice_from_ptr_end = |ptr: usize, end: usize| &input[ptr..end];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            let mut tok: c_int = 0;
            if ptr >= end {
                break 'iife_ret_57 XML_TOK_NONE_1;
            }
            {
                let mut n: size_t = c_char_ptr_diff(end, ptr) as size_t;
                if n & (2i32 - 1) as size_t != 0 {
                    n &= !(2i32 - 1) as size_t;
                    if n == 0 {
                        break 'iife_ret_57 XML_TOK_PARTIAL_1;
                    }
                    end = ptr.wrapping_add(n);
                }
            }
            let mut current_block_124: u64;
            match if read_ptr_at(ptr, 0) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
            } {
                BT_QUOT => {
                    break 'iife_ret_57 ({
                        let (tok_value, next_tok_value) = big2_scanLit(
                            BT_QUOT as c_int,
                            enc,
                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                        );
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_APOS => {
                    break 'iife_ret_57 ({
                        let (tok_value, next_tok_value) = big2_scanLit(
                            BT_APOS as c_int,
                            enc,
                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                        );
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_LT => {
                    ptr = ptr.wrapping_offset(2);
                    if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                        break 'iife_ret_57 XML_TOK_PARTIAL_1;
                    }
                    match if read_ptr_at(ptr, 0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                            as c_uint
                    } else {
                        unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                    } {
                        BT_EXCL => {
                            break 'iife_ret_57 ({
                                let (tok_value, next_tok_value) = big2_scanDecl(
                                    enc,
                                    c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                                );
                                *nextTokPtr = next_tok_value;
                                tok_value
                            });
                        }
                        BT_QUEST => {
                            break 'iife_ret_57 ({
                                let (tok_value, next_tok_value) = big2_scanPi(
                                    enc,
                                    c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                                );
                                *nextTokPtr = next_tok_value;
                                tok_value
                            });
                        }
                        BT_NMSTRT | BT_HEX | BT_NONASCII | BT_LEAD2 | BT_LEAD3 | BT_LEAD4 => {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(-(2)));
                            break 'iife_ret_57 XML_TOK_INSTANCE_START;
                        }
                        _ => {}
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_57 XML_TOK_INVALID_1;
                }
                BT_CR => {
                    if ptr.wrapping_offset(2) == end {
                        *nextTokPtr = ptr_to(end);
                        break 'iife_ret_57 -XML_TOK_PROLOG_S_1;
                    }
                    current_block_124 = 16869865525854146339;
                }
                BT_S | BT_LF => {
                    current_block_124 = 16869865525854146339;
                }
                BT_PERCNT => {
                    break 'iife_ret_57 ({
                        let (tok_value, next_tok_value) = big2_scanPercent(
                            enc,
                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                        );
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_COMMA => {
                    *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                    break 'iife_ret_57 XML_TOK_COMMA_1;
                }
                BT_LSQB => {
                    *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                    break 'iife_ret_57 XML_TOK_OPEN_BRACKET_1;
                }
                BT_RSQB => {
                    ptr = ptr.wrapping_offset(2);
                    if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                        break 'iife_ret_57 -XML_TOK_CLOSE_BRACKET_1;
                    }
                    if read_ptr_at(ptr, 0) as c_int == 0 && read_ptr_at(ptr, 1) as c_int == 0x5d {
                        if (c_char_ptr_diff(end, ptr)) < (2i32 * 2) as c_long {
                            break 'iife_ret_57 XML_TOK_PARTIAL_1;
                        }
                        if read_ptr_at(ptr.wrapping_offset(2), 0) as c_int == 0
                            && read_ptr_at(ptr.wrapping_offset(2), 1) as c_int == 0x3e
                        {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset((2i32 * 2) as isize));
                            break 'iife_ret_57 XML_TOK_COND_SECT_CLOSE_1;
                        }
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_57 XML_TOK_CLOSE_BRACKET_1;
                }
                BT_LPAR => {
                    *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                    break 'iife_ret_57 XML_TOK_OPEN_PAREN_1;
                }
                BT_RPAR => {
                    ptr = ptr.wrapping_offset(2);
                    if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                        break 'iife_ret_57 -XML_TOK_CLOSE_PAREN_1;
                    }
                    match if read_ptr_at(ptr, 0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                            as c_uint
                    } else {
                        unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                    } {
                        BT_AST => {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                            break 'iife_ret_57 XML_TOK_CLOSE_PAREN_ASTERISK_1;
                        }
                        BT_QUEST => {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                            break 'iife_ret_57 XML_TOK_CLOSE_PAREN_QUESTION_1;
                        }
                        BT_PLUS => {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                            break 'iife_ret_57 XML_TOK_CLOSE_PAREN_PLUS_1;
                        }
                        BT_CR | BT_LF | BT_S | BT_GT | BT_COMMA | BT_VERBAR | BT_RPAR => {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_57 XML_TOK_CLOSE_PAREN_1;
                        }
                        _ => {}
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_57 XML_TOK_INVALID_1;
                }
                BT_VERBAR => {
                    *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                    break 'iife_ret_57 XML_TOK_OR_1;
                }
                BT_GT => {
                    *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                    break 'iife_ret_57 XML_TOK_DECL_CLOSE_1;
                }
                BT_NUM => {
                    break 'iife_ret_57 ({
                        let (tok_value, next_tok_value) = big2_scanPoundName(
                            enc,
                            c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                        );
                        *nextTokPtr = next_tok_value;
                        tok_value
                    });
                }
                BT_LEAD2 => {
                    if (c_char_ptr_diff(end, ptr)) < 2 {
                        break 'iife_ret_57 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_57 XML_TOK_INVALID_1;
                }
                BT_LEAD3 => {
                    if (c_char_ptr_diff(end, ptr)) < 3 {
                        break 'iife_ret_57 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_57 XML_TOK_INVALID_1;
                }
                BT_LEAD4 => {
                    if (c_char_ptr_diff(end, ptr)) < 4 {
                        break 'iife_ret_57 XML_TOK_PARTIAL_CHAR_1;
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_57 XML_TOK_INVALID_1;
                }
                BT_NMSTRT | BT_HEX => {
                    tok = XML_TOK_NAME;
                    ptr = ptr.wrapping_offset(2);
                    current_block_124 = 2956972668325154207;
                }
                BT_DIGIT | BT_NAME | BT_MINUS | BT_COLON_0 => {
                    tok = XML_TOK_NMTOKEN_1;
                    ptr = ptr.wrapping_offset(2);
                    current_block_124 = 2956972668325154207;
                }
                BT_NONASCII => {
                    if namingBitmap[(((nmstrtPages[read_ptr_at(ptr, 0) as c_uchar as usize]
                        as c_int)
                        << 3)
                        + (read_ptr_at(ptr, 1) as c_uchar as c_int >> 5))
                        as usize]
                        & (1) << (read_ptr_at(ptr, 1) as c_uchar as c_int & 0x1f)
                        != 0
                    {
                        ptr = ptr.wrapping_offset(2);
                        tok = XML_TOK_NAME;
                        current_block_124 = 2956972668325154207;
                    } else if namingBitmap[(((namePages[read_ptr_at(ptr, 0) as c_uchar as usize]
                        as c_int)
                        << 3)
                        + (read_ptr_at(ptr, 1) as c_uchar as c_int >> 5))
                        as usize]
                        & (1) << (read_ptr_at(ptr, 1) as c_uchar as c_int & 0x1f)
                        != 0
                    {
                        ptr = ptr.wrapping_offset(2);
                        tok = XML_TOK_NMTOKEN_1;
                        current_block_124 = 2956972668325154207;
                    } else {
                        current_block_124 = 6428058487030868344;
                    }
                }
                _ => {
                    current_block_124 = 6428058487030868344;
                }
            }
            match current_block_124 {
                2956972668325154207 => {}
                16869865525854146339 => {
                    loop {
                        ptr = ptr.wrapping_offset(2);
                        if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                            break;
                        }
                        let mut current_block_32: u64;
                        match if read_ptr_at(ptr, 0) as c_int == 0 {
                            as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                                as c_uint
                        } else {
                            unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                        } {
                            BT_S | BT_LF => {
                                current_block_32 = 17500079516916021833;
                            }
                            BT_CR => {
                                if ptr.wrapping_offset(2) != end {
                                    current_block_32 = 17500079516916021833;
                                } else {
                                    current_block_32 = 11299462660638600073;
                                }
                            }
                            _ => {
                                current_block_32 = 11299462660638600073;
                            }
                        }
                        match current_block_32 {
                            17500079516916021833 => {}
                            _ => {
                                *nextTokPtr = ptr_to(ptr);
                                break 'iife_ret_57 XML_TOK_PROLOG_S_1;
                            }
                        }
                    }
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_57 XML_TOK_PROLOG_S_1;
                }
                _ => {
                    *nextTokPtr = ptr_to(ptr);
                    break 'iife_ret_57 XML_TOK_INVALID_1;
                }
            }
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                let mut current_block_210: u64;
                match if read_ptr_at(ptr, 0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((namePages[read_ptr_at(ptr, 0) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (read_ptr_at(ptr, 1) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (read_ptr_at(ptr, 1) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_57 XML_TOK_INVALID_1;
                        }
                        current_block_210 = 9794574411605359176;
                    }
                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                        current_block_210 = 9794574411605359176;
                    }
                    BT_LEAD2 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            break 'iife_ret_57 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_57 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            break 'iife_ret_57 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_57 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            break 'iife_ret_57 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_57 XML_TOK_INVALID_1;
                    }
                    BT_GT | BT_RPAR | BT_COMMA | BT_VERBAR | BT_LSQB | BT_PERCNT | BT_S | BT_CR
                    | BT_LF => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_57 tok;
                    }
                    BT_COLON_0 => {
                        ptr = ptr.wrapping_offset(2);
                        match tok {
                            XML_TOK_NAME => {
                                if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                    break 'iife_ret_57 XML_TOK_PARTIAL_1;
                                }
                                tok = XML_TOK_PREFIXED_NAME;
                                let mut current_block_187: u64;
                                match if read_ptr_at(ptr, 0) as c_int == 0 {
                                    as_normal_encoding(enc).type_0
                                        [read_ptr_at(ptr, 1) as c_uchar as usize]
                                        as c_uint
                                } else {
                                    unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1))
                                        as c_uint
                                } {
                                    BT_NONASCII => {
                                        if namingBitmap[(((namePages
                                            [read_ptr_at(ptr, 0) as c_uchar as usize]
                                            as c_int)
                                            << 3)
                                            + (read_ptr_at(ptr, 1) as c_uchar as c_int >> 5))
                                            as usize]
                                            & (1)
                                                << (read_ptr_at(ptr, 1) as c_uchar as c_int & 0x1f)
                                            == 0
                                        {
                                            *nextTokPtr = ptr_to(ptr);
                                            break 'iife_ret_57 XML_TOK_INVALID_1;
                                        }
                                        current_block_187 = 17275381528970576968;
                                    }
                                    BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                                        current_block_187 = 17275381528970576968;
                                    }
                                    BT_LEAD2 => {
                                        if (c_char_ptr_diff(end, ptr)) < 2 {
                                            break 'iife_ret_57 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        *nextTokPtr = ptr_to(ptr);
                                        break 'iife_ret_57 XML_TOK_INVALID_1;
                                    }
                                    BT_LEAD3 => {
                                        if (c_char_ptr_diff(end, ptr)) < 3 {
                                            break 'iife_ret_57 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        *nextTokPtr = ptr_to(ptr);
                                        break 'iife_ret_57 XML_TOK_INVALID_1;
                                    }
                                    BT_LEAD4 => {
                                        if (c_char_ptr_diff(end, ptr)) < 4 {
                                            break 'iife_ret_57 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        *nextTokPtr = ptr_to(ptr);
                                        break 'iife_ret_57 XML_TOK_INVALID_1;
                                    }
                                    _ => {
                                        tok = XML_TOK_NMTOKEN_1;
                                        current_block_187 = 9812798724717783973;
                                    }
                                }
                                if current_block_187 == 17275381528970576968 {
                                    ptr = ptr.wrapping_offset(2isize);
                                }
                            }
                            XML_TOK_PREFIXED_NAME => {
                                tok = XML_TOK_NMTOKEN_1;
                            }
                            _ => {}
                        }
                        current_block_210 = 14244298717249035578;
                    }
                    BT_PLUS => {
                        if tok == XML_TOK_NMTOKEN_1 {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_57 XML_TOK_INVALID_1;
                        }
                        *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                        break 'iife_ret_57 XML_TOK_NAME_PLUS_1;
                    }
                    BT_AST => {
                        if tok == XML_TOK_NMTOKEN_1 {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_57 XML_TOK_INVALID_1;
                        }
                        *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                        break 'iife_ret_57 XML_TOK_NAME_ASTERISK_1;
                    }
                    BT_QUEST => {
                        if tok == XML_TOK_NMTOKEN_1 {
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_57 XML_TOK_INVALID_1;
                        }
                        *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                        break 'iife_ret_57 XML_TOK_NAME_QUESTION_1;
                    }
                    _ => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_57 XML_TOK_INVALID_1;
                    }
                }
                if current_block_210 == 9794574411605359176 {
                    ptr = ptr.wrapping_offset(2isize);
                }
            }
            break 'iife_ret_57 -tok;
        };
        (tok, next_tok)
    }

    pub(crate) fn big2_attributeValueTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_58: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let c_char_slice_from_ptr_end = |ptr: usize, end: usize| &input[ptr..end];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            let mut start: usize = 0;
            if ptr >= end {
                break 'iife_ret_58 XML_TOK_NONE_1;
            } else if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                break 'iife_ret_58 XML_TOK_PARTIAL_1;
            }
            start = ptr;
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                match if read_ptr_at(ptr, 0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                } {
                    BT_LEAD2 => {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                    BT_LEAD3 => {
                        ptr = ptr.wrapping_offset(3isize);
                    }
                    BT_LEAD4 => {
                        ptr = ptr.wrapping_offset(4isize);
                    }
                    BT_AMP => {
                        if ptr == start {
                            break 'iife_ret_58 ({
                                let (tok_value, next_tok_value) = big2_scanRef(
                                    enc,
                                    c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                                );
                                *nextTokPtr = next_tok_value;
                                tok_value
                            });
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_58 XML_TOK_DATA_CHARS_1;
                    }
                    BT_LT => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_58 XML_TOK_INVALID_1;
                    }
                    BT_LF => {
                        if ptr == start {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                            break 'iife_ret_58 XML_TOK_DATA_NEWLINE_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_58 XML_TOK_DATA_CHARS_1;
                    }
                    BT_CR => {
                        if ptr == start {
                            ptr = ptr.wrapping_offset(2);
                            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                break 'iife_ret_58 XML_TOK_TRAILING_CR_1;
                            }
                            if (if read_ptr_at(ptr, 0) as c_int == 0 {
                                as_normal_encoding(enc).type_0
                                    [read_ptr_at(ptr, 1) as c_uchar as usize]
                                    as c_int
                            } else {
                                unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1))
                            }) == BT_LF as c_int
                            {
                                ptr = ptr.wrapping_offset(2isize);
                            }
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_58 XML_TOK_DATA_NEWLINE_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_58 XML_TOK_DATA_CHARS_1;
                    }
                    BT_S => {
                        if ptr == start {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                            break 'iife_ret_58 XML_TOK_ATTRIBUTE_VALUE_S_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_58 XML_TOK_DATA_CHARS_1;
                    }
                    _ => {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                }
            }
            *nextTokPtr = ptr_to(ptr);
            break 'iife_ret_58 XML_TOK_DATA_CHARS_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn big2_entityValueTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_59: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let c_char_slice_from_ptr_end = |ptr: usize, end: usize| &input[ptr..end];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            let mut start: usize = 0;
            if ptr >= end {
                break 'iife_ret_59 XML_TOK_NONE_1;
            } else if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                break 'iife_ret_59 XML_TOK_PARTIAL_1;
            }
            start = ptr;
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                match if read_ptr_at(ptr, 0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                } {
                    BT_LEAD2 => {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                    BT_LEAD3 => {
                        ptr = ptr.wrapping_offset(3isize);
                    }
                    BT_LEAD4 => {
                        ptr = ptr.wrapping_offset(4isize);
                    }
                    BT_AMP => {
                        if ptr == start {
                            break 'iife_ret_59 ({
                                let (tok_value, next_tok_value) = big2_scanRef(
                                    enc,
                                    c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize), end),
                                );
                                *nextTokPtr = next_tok_value;
                                tok_value
                            });
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_59 XML_TOK_DATA_CHARS_1;
                    }
                    BT_PERCNT => {
                        if ptr == start {
                            let mut tok: c_int = {
                                let (tok_value, next_tok_value) = big2_scanPercent(
                                    enc,
                                    c_char_slice_from_ptr_end(ptr.wrapping_offset(2), end),
                                );
                                *nextTokPtr = next_tok_value;
                                tok_value
                            };
                            break 'iife_ret_59 if tok == XML_TOK_PERCENT_1 {
                                XML_TOK_INVALID_1
                            } else {
                                tok
                            };
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_59 XML_TOK_DATA_CHARS_1;
                    }
                    BT_LF => {
                        if ptr == start {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                            break 'iife_ret_59 XML_TOK_DATA_NEWLINE_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_59 XML_TOK_DATA_CHARS_1;
                    }
                    BT_CR => {
                        if ptr == start {
                            ptr = ptr.wrapping_offset(2);
                            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                break 'iife_ret_59 XML_TOK_TRAILING_CR_1;
                            }
                            if (if read_ptr_at(ptr, 0) as c_int == 0 {
                                as_normal_encoding(enc).type_0
                                    [read_ptr_at(ptr, 1) as c_uchar as usize]
                                    as c_int
                            } else {
                                unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1))
                            }) == BT_LF as c_int
                            {
                                ptr = ptr.wrapping_offset(2isize);
                            }
                            *nextTokPtr = ptr_to(ptr);
                            break 'iife_ret_59 XML_TOK_DATA_NEWLINE_1;
                        }
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_59 XML_TOK_DATA_CHARS_1;
                    }
                    _ => {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                }
            }
            *nextTokPtr = ptr_to(ptr);
            break 'iife_ret_59 XML_TOK_DATA_CHARS_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn big2_ignoreSectionTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_60: {
            let mut ptr = 0usize;
            let mut end = input.len();
            let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
            let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
            let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
            let mut level: c_int = 0;
            {
                let mut n: size_t = c_char_ptr_diff(end, ptr) as size_t;
                if n & (2i32 - 1) as size_t != 0 {
                    n &= !(2i32 - 1) as size_t;
                    end = ptr.wrapping_add(n);
                }
            }
            while c_char_ptr_diff(end, ptr) >= 2 as c_long {
                match if read_ptr_at(ptr, 0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize]
                        as c_uint
                } else {
                    unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
                } {
                    BT_LEAD2 => {
                        if (c_char_ptr_diff(end, ptr)) < 2 {
                            break 'iife_ret_60 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.wrapping_offset(2isize);
                    }
                    BT_LEAD3 => {
                        if (c_char_ptr_diff(end, ptr)) < 3 {
                            break 'iife_ret_60 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.wrapping_offset(3isize);
                    }
                    BT_LEAD4 => {
                        if (c_char_ptr_diff(end, ptr)) < 4 {
                            break 'iife_ret_60 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.wrapping_offset(4isize);
                    }
                    BT_NONXML | BT_MALFORM | BT_TRAIL => {
                        *nextTokPtr = ptr_to(ptr);
                        break 'iife_ret_60 XML_TOK_INVALID_1;
                    }
                    BT_LT => {
                        ptr = ptr.wrapping_offset(2);
                        if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                            break 'iife_ret_60 XML_TOK_PARTIAL_1;
                        }
                        if read_ptr_at(ptr, 0) as c_int == 0 && read_ptr_at(ptr, 1) as c_int == 0x21
                        {
                            ptr = ptr.wrapping_offset(2);
                            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                break 'iife_ret_60 XML_TOK_PARTIAL_1;
                            }
                            if read_ptr_at(ptr, 0) as c_int == 0
                                && read_ptr_at(ptr, 1) as c_int == 0x5b
                            {
                                level += 1;
                                ptr = ptr.wrapping_offset(2isize);
                            }
                        }
                    }
                    BT_RSQB => {
                        ptr = ptr.wrapping_offset(2);
                        if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                            break 'iife_ret_60 XML_TOK_PARTIAL_1;
                        }
                        if read_ptr_at(ptr, 0) as c_int == 0 && read_ptr_at(ptr, 1) as c_int == 0x5d
                        {
                            ptr = ptr.wrapping_offset(2);
                            if (c_char_ptr_diff(end, ptr)) < 2 as c_long {
                                break 'iife_ret_60 XML_TOK_PARTIAL_1;
                            }
                            if read_ptr_at(ptr, 0) as c_int == 0
                                && read_ptr_at(ptr, 1) as c_int == 0x3e
                            {
                                ptr = ptr.wrapping_offset(2);
                                if level == 0 {
                                    *nextTokPtr = ptr_to(ptr);
                                    break 'iife_ret_60 XML_TOK_IGNORE_SECT_1;
                                }
                                level -= 1;
                            }
                        }
                    }
                    _ => {
                        ptr = ptr.wrapping_offset(2isize);
                    }
                }
            }
            break 'iife_ret_60 XML_TOK_PARTIAL_1;
        };
        (tok, next_tok)
    }

    pub(crate) fn big2_isPublicId(enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult {
        let mut badPtrVal: *const c_char = null::<c_char>();
        let badPtr = &mut badPtrVal;
        let mut ptr = 0usize;
        let mut end = input.len();
        let c_char_ptr_diff = |end: usize, start: usize| end.wrapping_sub(start) as c_long;
        let read_ptr_at = |ptr: usize, offset: isize| input[ptr.wrapping_offset(offset)];
        let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
        ptr = ptr.wrapping_offset(2);
        end = end.wrapping_offset(-(2));
        while c_char_ptr_diff(end, ptr) >= 2 as c_long {
            let mut current_block_8: u64;
            match if read_ptr_at(ptr, 0) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
            } {
                25 | 24 | 27 | 13 | 31 | 32 | 34 | 35 | 17 | 14 | 15 | 9 | 10 | 18 | 16 | 33
                | 30 | 19 | 23 => {
                    current_block_8 = 5143058163439228106;
                }
                BT_S => {
                    if read_ptr_at(ptr, 0) as c_int == 0 && read_ptr_at(ptr, 1) as c_int == 0x9 {
                        *badPtr = ptr_to(ptr);
                        return (0i32, badPtrVal);
                    }
                    current_block_8 = 5143058163439228106;
                }
                BT_NAME | BT_NMSTRT => {
                    if (if read_ptr_at(ptr, 0) as c_int == 0 {
                        read_ptr_at(ptr, 1) as c_int
                    } else {
                        -(1)
                    }) & !(0x7f)
                        == 0
                    {
                        current_block_8 = 5143058163439228106;
                    } else {
                        current_block_8 = 9906551679175889830;
                    }
                }
                _ => {
                    current_block_8 = 9906551679175889830;
                }
            }
            if current_block_8 == 9906551679175889830 {
                match if read_ptr_at(ptr, 0) as c_int == 0 {
                    read_ptr_at(ptr, 1) as c_int
                } else {
                    -(1)
                } {
                    36 | 64 => {}
                    _ => {
                        *badPtr = ptr_to(ptr);
                        return (0i32, badPtrVal);
                    }
                }
            }
            ptr = ptr.wrapping_offset(2);
        }
        (1, badPtrVal)
    }

    pub(crate) fn big2_getAtts(
        enc: &ENCODING,
        mut ptr: *const c_char,
        mut attsMax: c_int,
        mut atts: *mut ATTRIBUTE,
    ) -> c_int {
        let mut state: C2RustUnnamed_3 = inName_1;
        let mut nAtts: c_int = 0;
        let mut open: c_int = 0;
        ptr = ptr.wrapping_offset(2);
        loop {
            match if read_ptr_at(ptr, 0) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_ptr_at(ptr, 1) as c_uchar as usize] as c_uint
            } else {
                unicode_byte_type(read_ptr_at(ptr, 0), read_ptr_at(ptr, 1)) as c_uint
            } {
                BT_LEAD2 => {
                    if state == other_1 {
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.name = ptr;
                            att.normalized = 1i8;
                        }
                        state = inName_1;
                    }
                    ptr = ptr.wrapping_offset((2i32 - 2i32) as isize);
                }
                BT_LEAD3 => {
                    if state == other_1 {
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.name = ptr;
                            att.normalized = 1i8;
                        }
                        state = inName_1;
                    }
                    ptr = ptr.wrapping_offset((3i32 - 2i32) as isize);
                }
                BT_LEAD4 => {
                    if state == other_1 {
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.name = ptr;
                            att.normalized = 1i8;
                        }
                        state = inName_1;
                    }
                    ptr = ptr.wrapping_offset((4i32 - 2i32) as isize);
                }
                BT_NONASCII | BT_NMSTRT | BT_HEX => {
                    if state == other_1 {
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.name = ptr;
                            att.normalized = 1i8;
                        }
                        state = inName_1;
                    }
                }
                BT_QUOT => {
                    if state != inValue_1 {
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.valuePtr = ptr.wrapping_offset(2isize);
                        }
                        state = inValue_1;
                        open = BT_QUOT as c_int;
                    } else if open == BT_QUOT as c_int {
                        state = other_1;
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.valueEnd = ptr;
                        }
                        nAtts += 1;
                    }
                }
                BT_APOS => {
                    if state != inValue_1 {
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.valuePtr = ptr.wrapping_offset(2isize);
                        }
                        state = inValue_1;
                        open = BT_APOS as c_int;
                    } else if open == BT_APOS as c_int {
                        state = other_1;
                        if nAtts < attsMax {
                            let att = attribute_mut(atts, nAtts);
                            att.valueEnd = ptr;
                        }
                        nAtts += 1;
                    }
                }
                BT_AMP => {
                    if nAtts < attsMax {
                        attribute_mut(atts, nAtts).normalized = 0i8;
                    }
                }
                BT_S => {
                    if state == inName_1 {
                        state = other_1;
                    } else if state == inValue_1 && nAtts < attsMax {
                        let att = attribute_mut(atts, nAtts);
                        if att.normalized as c_int != 0
                            && (ptr == att.valuePtr
                                || (if read_ptr_at(ptr, 0) as c_int == 0 {
                                    read_ptr_at(ptr, 1) as c_int
                                } else {
                                    -(1)
                                }) != ASCII_SPACE
                                || (if read_ptr_at(ptr.wrapping_offset(2), 0) as c_int == 0 {
                                    read_ptr_at(ptr.wrapping_offset(2), 1) as c_int
                                } else {
                                    -(1)
                                }) == ASCII_SPACE
                                || (if read_ptr_at(ptr.wrapping_offset(2), 0) as c_int == 0 {
                                    as_normal_encoding(enc).type_0
                                        [read_ptr_at(ptr.wrapping_offset(2), 1) as c_uchar as usize]
                                        as c_int
                                } else {
                                    unicode_byte_type(
                                        read_ptr_at(ptr.wrapping_offset(2), 0),
                                        read_ptr_at(ptr.wrapping_offset(2), 1),
                                    )
                                }) == open)
                        {
                            att.normalized = 0i8;
                        }
                    }
                }
                BT_CR | BT_LF => {
                    if state == inName_1 {
                        state = other_1;
                    } else if state == inValue_1 && nAtts < attsMax {
                        attribute_mut(atts, nAtts).normalized = 0i8;
                    }
                }
                BT_GT | BT_SOL => {
                    if state != inValue_1 {
                        return nAtts;
                    }
                }
                _ => {}
            }
            ptr = ptr.wrapping_offset(2);
        }
    }

    pub(crate) fn big2_charRefNumber(_enc: &ENCODING, mut ptr: *const c_char) -> c_int {
        let mut result: c_int = 0;
        ptr = ptr.wrapping_offset(4);
        if read_c_char_at(ptr, 0) as c_int == 0 && read_c_char_at(ptr, 1) as c_int == 0x78 {
            ptr = ptr.wrapping_offset(2);
            while !(read_c_char_at(ptr, 0) as c_int == 0 && read_c_char_at(ptr, 1) as c_int == 0x3b)
            {
                let c: c_int = if read_c_char_at(ptr, 0) as c_int == 0 {
                    read_c_char_at(ptr, 1) as c_int
                } else {
                    -(1)
                };
                match c {
                    ASCII_0 | ASCII_1_1 | ASCII_2_1 | ASCII_3_1 | ASCII_4 | ASCII_5 | ASCII_6
                    | ASCII_7 | ASCII_8_1 | ASCII_9_1 => {
                        result <<= 4;
                        result |= c - ASCII_0;
                    }
                    ASCII_A | ASCII_B_1 | ASCII_C | ASCII_D | ASCII_E_1 | ASCII_F_1 => {
                        result <<= 4;
                        result += 10i32 + (c - ASCII_A);
                    }
                    ASCII_a_1 | ASCII_b | ASCII_c_1 | ASCII_d | ASCII_e_1 | ASCII_f => {
                        result <<= 4;
                        result += 10i32 + (c - ASCII_a_1);
                    }
                    _ => {}
                }
                if result >= 0x110000 {
                    return -(1i32);
                }
                ptr = ptr.wrapping_offset(2);
            }
        } else {
            while !(read_c_char_at(ptr, 0) as c_int == 0 && read_c_char_at(ptr, 1) as c_int == 0x3b)
            {
                let c_0: c_int = if read_c_char_at(ptr, 0) as c_int == 0 {
                    read_c_char_at(ptr, 1) as c_int
                } else {
                    -(1)
                };
                result *= 10;
                result += c_0 - ASCII_0;
                if result >= 0x110000 {
                    return -(1i32);
                }
                ptr = ptr.wrapping_offset(2);
            }
        }
        checkCharRefNumber(result)
    }

    pub(crate) fn big2_predefinedEntityName(_enc: &ENCODING, input: &[c_char]) -> c_int {
        let mut ptr = 0usize;
        let end = input.len();
        match end.wrapping_sub(ptr) / 2 {
            2 => {
                if input[ptr.wrapping_offset(2)] as c_int == 0
                    && input[ptr.wrapping_offset(3)] as c_int == 0x74
                {
                    match if input[ptr.wrapping_offset(0)] as c_int == 0 {
                        input[ptr.wrapping_offset(1)] as c_int
                    } else {
                        -(1)
                    } {
                        ASCII_l_1 => return ASCII_LT,
                        ASCII_g_1 => return ASCII_GT,
                        _ => {}
                    }
                }
            }
            3 => {
                if input[ptr.wrapping_offset(0)] as c_int == 0
                    && input[ptr.wrapping_offset(1)] as c_int == 0x61
                {
                    ptr = ptr.wrapping_offset(2);
                    if input[ptr.wrapping_offset(0)] as c_int == 0
                        && input[ptr.wrapping_offset(1)] as c_int == 0x6d
                    {
                        ptr = ptr.wrapping_offset(2);
                        if input[ptr.wrapping_offset(0)] as c_int == 0
                            && input[ptr.wrapping_offset(1)] as c_int == 0x70
                        {
                            return ASCII_AMP;
                        }
                    }
                }
            }
            4 => {
                match if input[ptr.wrapping_offset(0)] as c_int == 0 {
                    input[ptr.wrapping_offset(1)] as c_int
                } else {
                    -(1)
                } {
                    ASCII_q => {
                        ptr = ptr.wrapping_offset(2);
                        if input[ptr.wrapping_offset(0)] as c_int == 0
                            && input[ptr.wrapping_offset(1)] as c_int == 0x75
                        {
                            ptr = ptr.wrapping_offset(2);
                            if input[ptr.wrapping_offset(0)] as c_int == 0
                                && input[ptr.wrapping_offset(1)] as c_int == 0x6f
                            {
                                ptr = ptr.wrapping_offset(2);
                                if input[ptr.wrapping_offset(0)] as c_int == 0
                                    && input[ptr.wrapping_offset(1)] as c_int == 0x74
                                {
                                    return ASCII_QUOT;
                                }
                            }
                        }
                    }
                    ASCII_a_1 => {
                        ptr = ptr.wrapping_offset(2);
                        if input[ptr.wrapping_offset(0)] as c_int == 0
                            && input[ptr.wrapping_offset(1)] as c_int == 0x70
                        {
                            ptr = ptr.wrapping_offset(2);
                            if input[ptr.wrapping_offset(0)] as c_int == 0
                                && input[ptr.wrapping_offset(1)] as c_int == 0x6f
                            {
                                ptr = ptr.wrapping_offset(2);
                                if input[ptr.wrapping_offset(0)] as c_int == 0
                                    && input[ptr.wrapping_offset(1)] as c_int == 0x73
                                {
                                    return ASCII_APOS;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        0
    }

    pub(crate) fn big2_nameMatchesAscii(
        _enc: &ENCODING,
        input: &[c_char],
        mut ptr2: *const c_char,
    ) -> c_int {
        let mut ptr1 = 0usize;
        let end1 = input.len();
        while read_c_char(ptr2) != 0 {
            if end1.wrapping_sub(ptr1) < 2 {
                return 0i32;
            }
            if !(input[ptr1.wrapping_offset(0)] as c_int == 0
                && input[ptr1.wrapping_offset(1)] as c_int == read_c_char(ptr2) as c_int)
            {
                return 0i32;
            }
            ptr1 = ptr1.wrapping_offset(2);
            ptr2 = ptr2.wrapping_offset(1);
        }
        (ptr1 == end1) as c_int
    }

    pub(crate) fn big2_nameLength(enc: &ENCODING, mut ptr: *const c_char) -> c_int {
        let start: *const c_char = ptr;
        loop {
            match if read_c_char_at(ptr, 0) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_c_uchar_at(ptr, 1) as usize] as c_uint
            } else {
                unicode_byte_type(read_c_char_at(ptr, 0), read_c_char_at(ptr, 1)) as c_uint
            } {
                BT_LEAD2 => {
                    ptr = ptr.wrapping_offset(2);
                }
                BT_LEAD3 => {
                    ptr = ptr.wrapping_offset(3);
                }
                BT_LEAD4 => {
                    ptr = ptr.wrapping_offset(4);
                }
                BT_NONASCII | BT_NMSTRT | BT_COLON_0 | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                    ptr = ptr.wrapping_offset(2);
                }
                _ => {
                    return c_char_ptr_diff(ptr, start) as c_int;
                }
            }
        }
    }

    pub(crate) fn big2_skipS(enc: &ENCODING, mut ptr: *const c_char) -> *const c_char {
        loop {
            match if read_c_char_at(ptr, 0) as c_int == 0 {
                as_normal_encoding(enc).type_0[read_c_uchar_at(ptr, 1) as usize] as c_uint
            } else {
                unicode_byte_type(read_c_char_at(ptr, 0), read_c_char_at(ptr, 1)) as c_uint
            } {
                BT_LF | BT_CR | BT_S => {
                    ptr = ptr.wrapping_offset(2);
                }
                _ => return ptr,
            }
        }
    }

    pub(crate) fn big2_updatePosition(enc: &ENCODING, input: &[c_char], mut pos: *mut POSITION) {
        let pos = mut_ref_from_ptr(pos);
        let mut ptr = 0usize;
        let end = input.len();
        while end.wrapping_sub(ptr) >= 2 {
            match if input[ptr.wrapping_offset(0)] as c_int == 0 {
                as_normal_encoding(enc).type_0[input[ptr.wrapping_offset(1)] as c_uchar as usize]
                    as c_uint
            } else {
                unicode_byte_type(input[ptr.wrapping_offset(0)], input[ptr.wrapping_offset(1)])
                    as c_uint
            } {
                BT_LEAD2 => {
                    ptr = ptr.wrapping_offset(2);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                BT_LEAD3 => {
                    ptr = ptr.wrapping_offset(3);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                BT_LEAD4 => {
                    ptr = ptr.wrapping_offset(4);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                BT_LF => {
                    pos.columnNumber = 0u64;
                    pos.lineNumber = pos.lineNumber.wrapping_add(1);
                    ptr = ptr.wrapping_offset(2);
                }
                BT_CR => {
                    pos.lineNumber = pos.lineNumber.wrapping_add(1);
                    ptr = ptr.wrapping_offset(2);
                    if end.wrapping_sub(ptr) >= 2
                        && (if input[ptr.wrapping_offset(0)] as c_int == 0 {
                            as_normal_encoding(enc).type_0
                                [input[ptr.wrapping_offset(1)] as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(
                                input[ptr.wrapping_offset(0)],
                                input[ptr.wrapping_offset(1)],
                            )
                        }) == BT_LF as c_int
                    {
                        ptr = ptr.wrapping_offset(2);
                    }
                    pos.columnNumber = 0u64;
                }
                _ => {
                    ptr = ptr.wrapping_offset(2);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
            }
        }
    }

    use crate::src::lib::xmltok::checkCharRefNumber;
    use crate::src::lib::xmltok::nametab_h::namePages;
    use crate::src::lib::xmltok::nametab_h::namingBitmap;
    use crate::src::lib::xmltok::nametab_h::nmstrtPages;
    use crate::src::lib::xmltok::unicode_byte_type;
}

pub mod xmltok_ns_c {
    use super::*;
    pub(crate) fn XmlGetUtf8InternalEncoding() -> *const ENCODING {
        &raw const internal_utf8_encoding.enc
    }
    pub(crate) fn XmlGetUtf16InternalEncoding() -> *const ENCODING {
        &raw const internal_little2_encoding.enc
    }

    pub static encodings: [&ENCODING; 7] = [
        &crate::src::lib::xmltok::latin1_encoding.enc,
        &crate::src::lib::xmltok::ascii_encoding.enc,
        &crate::src::lib::xmltok::utf8_encoding.enc,
        &crate::src::lib::xmltok::big2_encoding.enc,
        &crate::src::lib::xmltok::big2_encoding.enc,
        &crate::src::lib::xmltok::little2_encoding.enc,
        &crate::src::lib::xmltok::utf8_encoding.enc,
    ];

    pub(crate) fn initScanProlog(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        initScan(
            encodings.as_ptr() as *const *const ENCODING,
            as_init_encoding(enc),
            XML_PROLOG_STATE,
            input,
        )
    }

    pub(crate) fn initScanContent(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        initScan(
            encodings.as_ptr() as *const *const ENCODING,
            as_init_encoding(enc),
            XML_CONTENT_STATE,
            input,
        )
    }
    pub(crate) fn XmlInitEncoding(
        mut p: *mut INIT_ENCODING,
        mut encPtr: *mut *const ENCODING,
        mut name: *const c_char,
    ) -> XmlInitEncodingResult {
        let i: c_int = getEncodingIndex(name);
        if i == UNKNOWN_ENC {
            return (0, null::<ENCODING>());
        }
        let init = mut_ref_from_ptr(p);
        init.initEnc = utf8_encoding.enc;
        init.initEnc.functions = &INIT_ENCODING_FUNCTIONS;
        init.initEnc.isUtf16 = i as c_char;
        init.initEnc.scanners[XML_PROLOG_STATE as usize] = initScanProlog as SCANNER;
        init.initEnc.scanners[XML_CONTENT_STATE as usize] = initScanContent as SCANNER;
        init.encPtr = encPtr;
        (1, &raw mut init.initEnc)
    }

    pub(crate) fn findEncoding(enc: &ENCODING, input: &[c_char]) -> *const ENCODING {
        let mut ptr = 0usize;
        let end = input.len();
        let mut buf: [c_char; 128] = [0; 128];
        let buf_start = buf.as_mut_ptr();
        let mut p: *mut c_char = buf_start;
        let input_ptr = input.as_ptr();

        let ptr_value;
        (_, ptr_value, p) = enc.utf8Convert(
            enc,
            input_ptr.wrapping_add(ptr),
            input_ptr.wrapping_add(end),
            p,
            p.wrapping_offset(127),
        );
        ptr = c_char_ptr_diff(ptr_value, input_ptr) as usize;
        if ptr != end {
            return null::<ENCODING>();
        }
        write_ptr(p, 0);
        if streqci(buf.as_ptr(), &raw const KW_UTF_16 as *const c_char) != 0
            && enc.minBytesPerChar == 2
        {
            return enc;
        }
        let i: c_int = getEncodingIndex(buf.as_ptr());
        if i == UNKNOWN_ENC {
            return null::<ENCODING>();
        }
        encodings[i as usize] as *const ENCODING
    }
    pub(crate) fn XmlParseXmlDecl(
        mut isGeneralTextEntity: c_int,
        enc: &ENCODING,
        input: &[c_char],
    ) -> ParseXmlDeclResult {
        doParseXmlDecl(
            Some(findEncoding as fn(&ENCODING, &[c_char]) -> *const ENCODING),
            isGeneralTextEntity,
            enc,
            input,
        )
    }
    pub(crate) fn XmlGetUtf8InternalEncodingNS() -> *const ENCODING {
        &raw const internal_utf8_encoding_ns.enc
    }
    pub(crate) fn XmlGetUtf16InternalEncodingNS() -> *const ENCODING {
        &raw const internal_little2_encoding_ns.enc
    }

    pub static encodingsNS: [&ENCODING; 7] = [
        &crate::src::lib::xmltok::latin1_encoding_ns.enc,
        &crate::src::lib::xmltok::ascii_encoding_ns.enc,
        &crate::src::lib::xmltok::utf8_encoding_ns.enc,
        &crate::src::lib::xmltok::big2_encoding_ns.enc,
        &crate::src::lib::xmltok::big2_encoding_ns.enc,
        &crate::src::lib::xmltok::little2_encoding_ns.enc,
        &crate::src::lib::xmltok::utf8_encoding_ns.enc,
    ];

    pub(crate) fn initScanPrologNS(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        initScan(
            encodingsNS.as_ptr() as *const *const ENCODING,
            as_init_encoding(enc),
            XML_PROLOG_STATE,
            input,
        )
    }

    pub(crate) fn initScanContentNS(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        initScan(
            encodingsNS.as_ptr() as *const *const ENCODING,
            as_init_encoding(enc),
            XML_CONTENT_STATE,
            input,
        )
    }
    pub(crate) fn XmlInitEncodingNS(
        mut p: *mut INIT_ENCODING,
        mut encPtr: *mut *const ENCODING,
        mut name: *const c_char,
    ) -> XmlInitEncodingResult {
        let i: c_int = getEncodingIndex(name);
        if i == UNKNOWN_ENC {
            return (0, null::<ENCODING>());
        }
        let init = mut_ref_from_ptr(p);
        init.initEnc = utf8_encoding_ns.enc;
        init.initEnc.functions = &INIT_ENCODING_FUNCTIONS;
        init.initEnc.isUtf16 = i as c_char;
        init.initEnc.scanners[XML_PROLOG_STATE as usize] = initScanPrologNS as SCANNER;
        init.initEnc.scanners[XML_CONTENT_STATE as usize] = initScanContentNS as SCANNER;
        init.encPtr = encPtr;
        (1, &raw mut init.initEnc)
    }

    pub(crate) fn findEncodingNS(enc: &ENCODING, input: &[c_char]) -> *const ENCODING {
        let mut ptr = 0usize;
        let end = input.len();
        let mut buf: [c_char; 128] = [0; 128];
        let buf_start = buf.as_mut_ptr();
        let mut p: *mut c_char = buf_start;
        let input_ptr = input.as_ptr();

        let ptr_value;
        (_, ptr_value, p) = enc.utf8Convert(
            enc,
            input_ptr.wrapping_add(ptr),
            input_ptr.wrapping_add(end),
            p,
            p.wrapping_offset(127),
        );
        ptr = c_char_ptr_diff(ptr_value, input_ptr) as usize;
        if ptr != end {
            return null::<ENCODING>();
        }
        write_ptr(p, 0);
        if streqci(buf.as_ptr(), &raw const KW_UTF_16 as *const c_char) != 0
            && enc.minBytesPerChar == 2
        {
            return enc;
        }
        let i: c_int = getEncodingIndex(buf.as_ptr());
        if i == UNKNOWN_ENC {
            return null::<ENCODING>();
        }
        encodingsNS[i as usize] as *const ENCODING
    }
    pub(crate) fn XmlParseXmlDeclNS(
        mut isGeneralTextEntity: c_int,
        enc: &ENCODING,
        input: &[c_char],
    ) -> ParseXmlDeclResult {
        doParseXmlDecl(
            Some(findEncodingNS as fn(&ENCODING, &[c_char]) -> *const ENCODING),
            isGeneralTextEntity,
            enc,
            input,
        )
    }

    use crate::src::lib::xmltok::doParseXmlDecl;
    use crate::src::lib::xmltok::getEncodingIndex;
    use crate::src::lib::xmltok::initScan;
    use crate::src::lib::xmltok::internal_little2_encoding;
    use crate::src::lib::xmltok::internal_little2_encoding_ns;
    use crate::src::lib::xmltok::internal_utf8_encoding;
    use crate::src::lib::xmltok::internal_utf8_encoding_ns;
    use crate::src::lib::xmltok::streqci;

    use crate::src::lib::xmltok::KW_UTF_16;

    use crate::src::lib::xmltok::UNKNOWN_ENC;
}

pub mod nametab_h {
    use super::*;

    pub static namingBitmap: [core::ffi::c_uint; 320] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
        0xffffffff, 0xffffffff, 0xffffffff, 0, 0x4000000, 0x87fffffe, 0x7fffffe, 0, 0, 0xff7fffff,
        0xff7fffff, 0xffffffff, 0x7ff3ffff, 0xfffffdfe, 0x7fffffff, 0xffffffff, 0xffffffff,
        0xffffe00f, 0xfc31ffff, 0xffffff, 0, 0xffff0000, 0xffffffff, 0xffffffff, 0xf80001ff, 0x3,
        0, 0, 0, 0, 0, 0xffffd740, 0xfffffffb, 0x547f7fff, 0xffffd, 0xffffdffe, 0xffffffff,
        0xdffeffff, 0xffffffff, 0xffff0003, 0xffffffff, 0xffff199f, 0x33fcfff, 0, 0xfffe0000,
        0x27fffff, 0xfffffffe, 0x7f, 0, 0xffff0000, 0x707ff, 0, 0x7fffffe, 0x7fe, 0xfffe0000,
        0xffffffff, 0x7cffffff, 0x2f7fff, 0x60, 0xffffffe0, 0x23ffffff, 0xff000000, 0x3,
        0xfff99fe0, 0x3c5fdff, 0xb0000000, 0x30003, 0xfff987e0, 0x36dfdff, 0x5e000000, 0x1c0000,
        0xfffbafe0, 0x23edfdff, 0, 0x1, 0xfff99fe0, 0x23cdfdff, 0xb0000000, 0x3, 0xd63dc7e0,
        0x3bfc718, 0, 0, 0xfffddfe0, 0x3effdff, 0, 0x3, 0xfffddfe0, 0x3effdff, 0x40000000, 0x3,
        0xfffddfe0, 0x3fffdff, 0, 0x3, 0, 0, 0, 0, 0xfffffffe, 0xd7fff, 0x3f, 0, 0xfef02596,
        0x200d6cae, 0x1f, 0, 0, 0, 0xfffffeff, 0x3ff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xffffffff,
        0xffff003f, 0x7fffff, 0x7daed, 0x50000000, 0x82315001, 0x2c62ab, 0x40000000, 0xf580c900,
        0x7, 0x2010800, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xfffffff, 0xffffffff,
        0xffffffff, 0x3ffffff, 0x3f3fffff, 0xffffffff, 0xaaff3f3f, 0x3fffffff, 0xffffffff,
        0x5fdfffff, 0xfcf1fdc, 0x1fdc1fff, 0, 0x4c40, 0, 0, 0x7, 0, 0, 0, 0x80, 0x3fe, 0xfffffffe,
        0xffffffff, 0x1fffff, 0xfffffffe, 0xffffffff, 0x7ffffff, 0xffffffe0, 0x1fff, 0, 0, 0, 0, 0,
        0, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0x3f, 0, 0, 0xffffffff,
        0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xf, 0, 0, 0, 0x7ff6000, 0x87fffffe,
        0x7fffffe, 0, 0x800000, 0xff7fffff, 0xff7fffff, 0xffffff, 0, 0xffff0000, 0xffffffff,
        0xffffffff, 0xf80001ff, 0x30003, 0, 0xffffffff, 0xffffffff, 0x3f, 0x3, 0xffffd7c0,
        0xfffffffb, 0x547f7fff, 0xffffd, 0xffffdffe, 0xffffffff, 0xdffeffff, 0xffffffff,
        0xffff007b, 0xffffffff, 0xffff199f, 0x33fcfff, 0, 0xfffe0000, 0x27fffff, 0xfffffffe,
        0xfffe007f, 0xbbfffffb, 0xffff0016, 0x707ff, 0, 0x7fffffe, 0x7ffff, 0xffff03ff, 0xffffffff,
        0x7cffffff, 0xffef7fff, 0x3ff3dff, 0xffffffee, 0xf3ffffff, 0xff1e3fff, 0xffcf, 0xfff99fee,
        0xd3c5fdff, 0xb080399f, 0x3ffcf, 0xfff987e4, 0xd36dfdff, 0x5e003987, 0x1fffc0, 0xfffbafee,
        0xf3edfdff, 0x3bbf, 0xffc1, 0xfff99fee, 0xf3cdfdff, 0xb0c0398f, 0xffc3, 0xd63dc7ec,
        0xc3bfc718, 0x803dc7, 0xff80, 0xfffddfee, 0xc3effdff, 0x603ddf, 0xffc3, 0xfffddfec,
        0xc3effdff, 0x40603ddf, 0xffc3, 0xfffddfec, 0xc3fffdff, 0x803dcf, 0xffc3, 0, 0, 0, 0,
        0xfffffffe, 0x7ff7fff, 0x3ff7fff, 0, 0xfef02596, 0x3bff6cae, 0x3ff3f5f, 0, 0x3000000,
        0xc2a003ff, 0xfffffeff, 0xfffe03ff, 0xfebf0fdf, 0x2fe3fff, 0, 0, 0, 0, 0, 0, 0, 0,
        0x1fff0000, 0x2, 0xa0, 0x3efffe, 0xfffffffe, 0xffffffff, 0x661fffff, 0xfffffffe,
        0xffffffff, 0x77ffffff,
    ];

    pub static nmstrtPages: [c_uchar; 256] = [
        0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0, 0, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x10, 0x11, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x12, 0x13, 0, 0x14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0x15, 0x16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    pub static namePages: [c_uchar; 256] = [
        0x19, 0x3, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0, 0, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25,
        0x10, 0x11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x12, 0x13, 0x26, 0x14, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0x27, 0x16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x17, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x18, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
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

pub use crate::stdbool_h::false_0;
pub use crate::stdbool_h::true_0;
pub use nametab_h::namePages;
pub use nametab_h::namingBitmap;
pub use nametab_h::nmstrtPages;

pub use crate::xmltok_impl_c::inName;
pub use crate::xmltok_impl_c::inName_0;
pub use crate::xmltok_impl_c::inName_1;
pub use crate::xmltok_impl_c::inValue;
pub use crate::xmltok_impl_c::inValue_0;
pub use crate::xmltok_impl_c::inValue_1;
pub use crate::xmltok_impl_c::other;
pub use crate::xmltok_impl_c::other_0;
pub use crate::xmltok_impl_c::other_1;
pub use crate::xmltok_impl_h::C2RustUnnamed_3;
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
use core::ffi::c_char;
use core::ffi::c_int;
use core::ffi::c_long;
use core::ffi::c_uchar;
use core::ffi::c_uint;
use core::ffi::c_ushort;
use core::ffi::c_void;
use core::mem::size_of;
use core::ptr::null;
use core::ptr::null_mut;
pub(crate) use xmltok_impl_c::big2_attributeValueTok;
pub(crate) use xmltok_impl_c::big2_cdataSectionTok;
pub(crate) use xmltok_impl_c::big2_charRefNumber;
pub(crate) use xmltok_impl_c::big2_contentTok;
pub(crate) use xmltok_impl_c::big2_entityValueTok;
pub(crate) use xmltok_impl_c::big2_getAtts;
pub(crate) use xmltok_impl_c::big2_ignoreSectionTok;
pub(crate) use xmltok_impl_c::big2_isPublicId;
pub(crate) use xmltok_impl_c::big2_nameLength;
pub(crate) use xmltok_impl_c::big2_nameMatchesAscii;
pub(crate) use xmltok_impl_c::big2_predefinedEntityName;
pub(crate) use xmltok_impl_c::big2_prologTok;
pub(crate) use xmltok_impl_c::big2_skipS;
pub(crate) use xmltok_impl_c::big2_updatePosition;
pub(crate) use xmltok_impl_c::little2_attributeValueTok;
pub(crate) use xmltok_impl_c::little2_cdataSectionTok;
pub(crate) use xmltok_impl_c::little2_charRefNumber;
pub(crate) use xmltok_impl_c::little2_contentTok;
pub(crate) use xmltok_impl_c::little2_entityValueTok;
pub(crate) use xmltok_impl_c::little2_getAtts;
pub(crate) use xmltok_impl_c::little2_ignoreSectionTok;
pub(crate) use xmltok_impl_c::little2_isPublicId;
pub(crate) use xmltok_impl_c::little2_nameLength;
pub(crate) use xmltok_impl_c::little2_nameMatchesAscii;
pub(crate) use xmltok_impl_c::little2_predefinedEntityName;
pub(crate) use xmltok_impl_c::little2_prologTok;
pub(crate) use xmltok_impl_c::little2_skipS;
pub(crate) use xmltok_impl_c::little2_updatePosition;
pub(crate) use xmltok_impl_c::normal_attributeValueTok;
pub(crate) use xmltok_impl_c::normal_cdataSectionTok;
pub(crate) use xmltok_impl_c::normal_charRefNumber;
pub(crate) use xmltok_impl_c::normal_contentTok;
pub(crate) use xmltok_impl_c::normal_entityValueTok;
pub(crate) use xmltok_impl_c::normal_getAtts;
pub(crate) use xmltok_impl_c::normal_ignoreSectionTok;
pub(crate) use xmltok_impl_c::normal_isPublicId;
pub(crate) use xmltok_impl_c::normal_nameLength;
pub(crate) use xmltok_impl_c::normal_nameMatchesAscii;
pub(crate) use xmltok_impl_c::normal_predefinedEntityName;
pub(crate) use xmltok_impl_c::normal_prologTok;
pub(crate) use xmltok_impl_c::normal_skipS;
pub(crate) use xmltok_impl_c::normal_updatePosition;
trait NormalEncodingCheckFunctions {
    fn isName2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int;
    fn isName3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int;
    fn isName4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int;
    fn isNmstrt2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int;
    fn isNmstrt3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int;
    fn isNmstrt4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int;
    fn isInvalid2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int;
    fn isInvalid3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int;
    fn isInvalid4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct normal_encoding {
    pub enc: ENCODING,
    pub type_0: [c_uchar; 256],
    check_functions: &'static (dyn NormalEncodingCheckFunctions + Sync),
}

impl normal_encoding {
    #[inline]
    fn checkFunctions(&self) -> &(dyn NormalEncodingCheckFunctions + Sync) {
        self.check_functions
    }

    #[inline]
    fn isName2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        self.checkFunctions().isName2(enc, ptr)
    }

    #[inline]
    fn isName3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        self.checkFunctions().isName3(enc, ptr)
    }

    #[inline]
    fn isName4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        self.checkFunctions().isName4(enc, ptr)
    }

    #[inline]
    fn isNmstrt2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        self.checkFunctions().isNmstrt2(enc, ptr)
    }

    #[inline]
    fn isNmstrt3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        self.checkFunctions().isNmstrt3(enc, ptr)
    }

    #[inline]
    fn isNmstrt4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        self.checkFunctions().isNmstrt4(enc, ptr)
    }

    #[inline]
    fn isInvalid2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        self.checkFunctions().isInvalid2(enc, ptr)
    }

    #[inline]
    fn isInvalid3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        self.checkFunctions().isInvalid3(enc, ptr)
    }

    #[inline]
    fn isInvalid4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        self.checkFunctions().isInvalid4(enc, ptr)
    }
}

pub const UTF8_cval2: C2RustUnnamed_8 = 192;

pub const UTF8_cval4: C2RustUnnamed_8 = 240;

pub const UTF8_cval3: C2RustUnnamed_8 = 224;

pub const UNKNOWN_ENC: C2RustUnnamed_9 = -1;

pub const NO_ENC: C2RustUnnamed_9 = 6;

pub const UTF_16LE_ENC: C2RustUnnamed_9 = 5;

pub const UTF_16BE_ENC: C2RustUnnamed_9 = 4;

pub const UTF_8_ENC: C2RustUnnamed_9 = 2;

pub const UTF_16_ENC: C2RustUnnamed_9 = 3;

pub const ISO_8859_1_ENC: C2RustUnnamed_9 = 0;

pub const min4: C2RustUnnamed_7 = 65536;

pub const min3: C2RustUnnamed_7 = 2048;

pub const UTF8_cval1: C2RustUnnamed_8 = 0;

pub const min2: C2RustUnnamed_7 = 128;

pub type C2RustUnnamed_7 = c_uint;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct unknown_encoding {
    pub normal: normal_encoding,
    pub convert: CONVERTER,
    pub userData: *mut c_void,
    pub utf16: [c_ushort; 256],
    pub utf8: [[c_char; 4]; 256],
}

pub type C2RustUnnamed_8 = c_uint;

pub type C2RustUnnamed_9 = c_int;

pub const US_ASCII_ENC: C2RustUnnamed_9 = 1;

fn isNever(_enc: &ENCODING, mut _p: *const c_char) -> c_int {
    0
}

fn utf8_isName2(_enc: &ENCODING, p: *const c_char) -> c_int {
    let b0 = read_c_uchar_at(p, 0) as c_int;
    let b1 = read_c_uchar_at(p, 1) as c_int;
    (namingBitmap[(((namePages[(b0 >> 2 & 7) as usize] as c_int) << 3)
        + ((b0 & 3) << 1)
        + (b1 >> 5 & 1)) as usize]
        & (1) << (b1 & 0x1f)) as c_int
}

fn utf8_isName3(_enc: &ENCODING, p: *const c_char) -> c_int {
    let b0 = read_c_uchar_at(p, 0) as c_int;
    let b1 = read_c_uchar_at(p, 1) as c_int;
    let b2 = read_c_uchar_at(p, 2) as c_int;
    (namingBitmap[(((namePages[(((b0 & 0xf) << 4) + (b1 >> 2 & 0xf)) as usize] as c_int) << 3)
        + ((b1 & 3) << 1)
        + (b2 >> 5 & 1)) as usize]
        & (1) << (b2 & 0x1f)) as c_int
}

fn utf8_isNmstrt2(_enc: &ENCODING, p: *const c_char) -> c_int {
    let b0 = read_c_uchar_at(p, 0) as c_int;
    let b1 = read_c_uchar_at(p, 1) as c_int;
    (namingBitmap[(((nmstrtPages[(b0 >> 2 & 7) as usize] as c_int) << 3)
        + ((b0 & 3) << 1)
        + (b1 >> 5 & 1)) as usize]
        & (1) << (b1 & 0x1f)) as c_int
}

fn utf8_isNmstrt3(_enc: &ENCODING, p: *const c_char) -> c_int {
    let b0 = read_c_uchar_at(p, 0) as c_int;
    let b1 = read_c_uchar_at(p, 1) as c_int;
    let b2 = read_c_uchar_at(p, 2) as c_int;
    (namingBitmap[(((nmstrtPages[(((b0 & 0xf) << 4) + (b1 >> 2 & 0xf)) as usize] as c_int) << 3)
        + ((b1 & 3) << 1)
        + (b2 >> 5 & 1)) as usize]
        & (1) << (b2 & 0x1f)) as c_int
}

fn utf8_isInvalid2(_enc: &ENCODING, p: *const c_char) -> c_int {
    let b0 = read_c_uchar_at(p, 0) as c_int;
    let b1 = read_c_uchar_at(p, 1) as c_int;
    (b0 < 0xc2 || b1 & 0x80 == 0 || b1 & 0xc0 == 0xc0) as c_int
}

fn utf8_isInvalid3(_enc: &ENCODING, p: *const c_char) -> c_int {
    let b0 = read_c_uchar_at(p, 0) as c_int;
    let b1 = read_c_uchar_at(p, 1) as c_int;
    let b2 = read_c_uchar_at(p, 2) as c_int;
    (b2 & 0x80 == 0
        || (if b0 == 0xef && b1 == 0xbf {
            (b2 > 0xbd) as c_int
        } else {
            (b2 & 0xc0 == 0xc0) as c_int
        }) != 0
        || (if b0 == 0xe0 {
            (b1 < 0xa0 || b1 & 0xc0 == 0xc0) as c_int
        } else {
            (b1 & 0x80 == 0
                || (if b0 == 0xed {
                    (b1 > 0x9f) as c_int
                } else {
                    (b1 & 0xc0 == 0xc0) as c_int
                }) != 0) as c_int
        }) != 0) as c_int
}

fn utf8_isInvalid4(_enc: &ENCODING, p: *const c_char) -> c_int {
    let b0 = read_c_uchar_at(p, 0) as c_int;
    let b1 = read_c_uchar_at(p, 1) as c_int;
    let b2 = read_c_uchar_at(p, 2) as c_int;
    let b3 = read_c_uchar_at(p, 3) as c_int;
    (b3 & 0x80 == 0
        || b3 & 0xc0 == 0xc0
        || b2 & 0x80 == 0
        || b2 & 0xc0 == 0xc0
        || (if b0 == 0xf0 {
            (b1 < 0x90 || b1 & 0xc0 == 0xc0) as c_int
        } else {
            (b1 & 0x80 == 0
                || (if b0 == 0xf4 {
                    (b1 > 0x8f) as c_int
                } else {
                    (b1 & 0xc0 == 0xc0) as c_int
                }) != 0) as c_int
        }) != 0) as c_int
}

#[inline(never)]
#[cold]
fn missingNormalEncodingFunction() -> ! {
    panic!("non-null function pointer");
}

struct Utf8NormalEncodingCheckFunctions;

impl NormalEncodingCheckFunctions for Utf8NormalEncodingCheckFunctions {
    fn isName2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        utf8_isName2(enc, ptr)
    }

    fn isName3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        utf8_isName3(enc, ptr)
    }

    fn isName4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        isNever(enc, ptr)
    }

    fn isNmstrt2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        utf8_isNmstrt2(enc, ptr)
    }

    fn isNmstrt3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        utf8_isNmstrt3(enc, ptr)
    }

    fn isNmstrt4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        isNever(enc, ptr)
    }

    fn isInvalid2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        utf8_isInvalid2(enc, ptr)
    }

    fn isInvalid3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        utf8_isInvalid3(enc, ptr)
    }

    fn isInvalid4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        utf8_isInvalid4(enc, ptr)
    }
}

struct UnknownNormalEncodingCheckFunctions;

impl NormalEncodingCheckFunctions for UnknownNormalEncodingCheckFunctions {
    fn isName2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        unknown_isName(enc, ptr)
    }

    fn isName3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        unknown_isName(enc, ptr)
    }

    fn isName4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        unknown_isName(enc, ptr)
    }

    fn isNmstrt2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        unknown_isNmstrt(enc, ptr)
    }

    fn isNmstrt3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        unknown_isNmstrt(enc, ptr)
    }

    fn isNmstrt4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        unknown_isNmstrt(enc, ptr)
    }

    fn isInvalid2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        unknown_isInvalid(enc, ptr)
    }

    fn isInvalid3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        unknown_isInvalid(enc, ptr)
    }

    fn isInvalid4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        unknown_isInvalid(enc, ptr)
    }
}

struct MissingNormalEncodingCheckFunctions;

impl NormalEncodingCheckFunctions for MissingNormalEncodingCheckFunctions {
    fn isName2(&self, _enc: &ENCODING, _ptr: *const c_char) -> c_int {
        missingNormalEncodingFunction()
    }

    fn isName3(&self, _enc: &ENCODING, _ptr: *const c_char) -> c_int {
        missingNormalEncodingFunction()
    }

    fn isName4(&self, _enc: &ENCODING, _ptr: *const c_char) -> c_int {
        missingNormalEncodingFunction()
    }

    fn isNmstrt2(&self, _enc: &ENCODING, _ptr: *const c_char) -> c_int {
        missingNormalEncodingFunction()
    }

    fn isNmstrt3(&self, _enc: &ENCODING, _ptr: *const c_char) -> c_int {
        missingNormalEncodingFunction()
    }

    fn isNmstrt4(&self, _enc: &ENCODING, _ptr: *const c_char) -> c_int {
        missingNormalEncodingFunction()
    }

    fn isInvalid2(&self, _enc: &ENCODING, _ptr: *const c_char) -> c_int {
        missingNormalEncodingFunction()
    }

    fn isInvalid3(&self, _enc: &ENCODING, _ptr: *const c_char) -> c_int {
        missingNormalEncodingFunction()
    }

    fn isInvalid4(&self, _enc: &ENCODING, _ptr: *const c_char) -> c_int {
        missingNormalEncodingFunction()
    }
}

pub(crate) fn _INTERNAL_trim_to_complete_utf8_characters(
    mut from: *const c_char,
    mut fromLim: *const c_char,
) -> *const c_char {
    let mut walked: size_t = 0;
    while fromLim > from {
        let prev: c_uchar = read_c_uchar_at(fromLim, -1);
        if prev as c_uint & 0xf8 == 0xf0 {
            if walked.wrapping_add(1usize) >= 4usize {
                fromLim = fromLim.wrapping_offset(4 - 1);
                break;
            } else {
                walked = 0usize;
            }
        } else if prev as c_uint & 0xf0 == 0xe0 {
            if walked.wrapping_add(1usize) >= 3usize {
                fromLim = fromLim.wrapping_offset(3 - 1);
                break;
            } else {
                walked = 0usize;
            }
        } else if prev as c_uint & 0xe0 == 0xc0 {
            if walked.wrapping_add(1usize) >= 2usize {
                fromLim = fromLim.wrapping_offset(2 - 1);
                break;
            } else {
                walked = 0usize;
            }
        } else if prev as c_uint & 0x80 == 0 {
            break;
        }
        fromLim = fromLim.wrapping_offset(-1);
        walked = walked.wrapping_add(1);
    }
    fromLim
}

#[cfg(feature = "expat_test_shims")]
#[export_name = "_INTERNAL_trim_to_complete_utf8_characters"]
fn internal_trim_to_complete_utf8_characters_test_shim(
    from: *const c_char,
    fromLimRef: *mut *const c_char,
) {
    let from_lim_ref = mut_ref_from_ptr(fromLimRef);
    *from_lim_ref = _INTERNAL_trim_to_complete_utf8_characters(from, *from_lim_ref);
}

fn utf8_toUtf8(
    _enc: &ENCODING,
    fromP: *const c_char,
    mut fromLim: *const c_char,
    toP: *mut c_char,
    mut toLim: *const c_char,
) -> Utf8ConvertResult {
    let mut from_cursor: *const c_char = fromP;
    let mut to_cursor: *mut c_char = toP;
    let fromP = &mut from_cursor;
    let toP = &mut to_cursor;
    let mut input_incomplete: bool = false_0 != 0;
    let mut output_exhausted: bool = false_0 != 0;
    let bytesAvailable: ptrdiff_t = c_char_ptr_diff(fromLim, *fromP) as ptrdiff_t;
    let bytesStorable: ptrdiff_t = c_char_ptr_diff(toLim, *toP as *const c_char) as ptrdiff_t;
    if bytesAvailable > bytesStorable {
        fromLim = (*fromP).wrapping_offset(bytesStorable);
        output_exhausted = true_0 != 0;
    }
    let fromLimBefore: *const c_char = fromLim;
    fromLim = _INTERNAL_trim_to_complete_utf8_characters(*fromP, fromLim);
    if fromLim < fromLimBefore {
        input_incomplete = true_0 != 0;
    }
    let bytesToCopy: ptrdiff_t = c_char_ptr_diff(fromLim, *fromP) as ptrdiff_t;
    if bytesToCopy > 0 {
        copy_nonoverlapping_c_char(*toP, *fromP, bytesToCopy as usize);
    }
    *fromP = (*fromP).wrapping_offset(bytesToCopy);
    *toP = (*toP).wrapping_offset(bytesToCopy);
    let res = if output_exhausted {
        XML_CONVERT_OUTPUT_EXHAUSTED
    } else if input_incomplete {
        XML_CONVERT_INPUT_INCOMPLETE
    } else {
        XML_CONVERT_COMPLETED
    };
    (res, from_cursor, to_cursor)
}

fn utf8_toUtf16(
    enc: &ENCODING,
    fromP: *const c_char,
    mut fromLim: *const c_char,
    toP: *mut c_ushort,
    mut toLim: *const c_ushort,
) -> Utf16ConvertResult {
    let mut from_cursor: *const c_char = fromP;
    let mut to_cursor: *mut c_ushort = toP;
    let fromP = &mut from_cursor;
    let toP = &mut to_cursor;
    let mut res: XML_Convert_Result = XML_CONVERT_COMPLETED;
    let mut to: *mut c_ushort = *toP;
    let mut from: *const c_char = *fromP;
    while from < fromLim && to < toLim as *mut c_ushort {
        match as_normal_encoding(enc).type_0[read_c_char(from) as c_uchar as usize] as c_uint {
            BT_LEAD2 => {
                if c_char_ptr_diff(fromLim, from) < 2 {
                    res = XML_CONVERT_INPUT_INCOMPLETE;
                    break;
                }
                write_ptr(
                    to,
                    (((read_c_char_at(from, 0) as c_int & 0x1f) << 6)
                        | (read_c_char_at(from, 1) as c_int & 0x3f))
                        as c_ushort,
                );
                to = to.wrapping_offset(1);
                from = from.wrapping_offset(2);
            }
            BT_LEAD3 => {
                if c_char_ptr_diff(fromLim, from) < 3 {
                    res = XML_CONVERT_INPUT_INCOMPLETE;
                    break;
                }
                write_ptr(
                    to,
                    (((read_c_char_at(from, 0) as c_int & 0xf) << 12)
                        | ((read_c_char_at(from, 1) as c_int & 0x3f) << 6)
                        | (read_c_char_at(from, 2) as c_int & 0x3f))
                        as c_ushort,
                );
                to = to.wrapping_offset(1);
                from = from.wrapping_offset(3);
            }
            BT_LEAD4 => {
                let mut n: core::ffi::c_ulong;
                if c_ushort_ptr_diff(toLim, to as *const c_ushort) < 2 {
                    res = XML_CONVERT_OUTPUT_EXHAUSTED;
                    break;
                }
                if c_char_ptr_diff(fromLim, from) < 4 {
                    res = XML_CONVERT_INPUT_INCOMPLETE;
                    break;
                }
                n = (((read_c_char_at(from, 0) as c_int & 0x7) << 18)
                    | ((read_c_char_at(from, 1) as c_int & 0x3f) << 12)
                    | ((read_c_char_at(from, 2) as c_int & 0x3f) << 6)
                    | (read_c_char_at(from, 3) as c_int & 0x3f))
                    as core::ffi::c_ulong;
                n = n.wrapping_sub(0x10000u64);
                write_ptr_at(to, 0, (n >> 10 | 0xd800) as c_ushort);
                write_ptr_at(to, 1, (n & 0x3ff | 0xdc00) as c_ushort);
                to = to.wrapping_offset(2);
                from = from.wrapping_offset(4);
            }
            _ => {
                write_ptr(to, read_c_char(from) as c_ushort);
                from = from.wrapping_offset(1);
                to = to.wrapping_offset(1);
            }
        }
    }
    if res == XML_CONVERT_COMPLETED && from < fromLim {
        res = XML_CONVERT_OUTPUT_EXHAUSTED;
    }
    *fromP = from;
    *toP = to;
    (res, from_cursor, to_cursor)
}

struct Utf8EncodingFunctions;

impl EncodingFunctions for Utf8EncodingFunctions {
    fn nameMatchesAscii(&self, enc: &ENCODING, input: &[c_char], kw: *const c_char) -> c_int {
        normal_nameMatchesAscii(enc, input, kw)
    }

    fn nameLength(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        normal_nameLength(enc, ptr)
    }

    fn skipS(&self, enc: &ENCODING, ptr: *const c_char) -> *const c_char {
        normal_skipS(enc, ptr)
    }

    fn getAtts(&self, enc: &ENCODING, ptr: *const c_char, n: c_int, atts: *mut ATTRIBUTE) -> c_int {
        normal_getAtts(enc, ptr, n, atts)
    }

    fn charRefNumber(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        normal_charRefNumber(enc, ptr)
    }

    fn predefinedEntityName(&self, enc: &ENCODING, input: &[c_char]) -> c_int {
        normal_predefinedEntityName(enc, input)
    }

    fn updatePosition(&self, enc: &ENCODING, input: &[c_char], pos: *mut POSITION) {
        normal_updatePosition(enc, input, pos);
    }

    fn isPublicId(&self, enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult {
        normal_isPublicId(enc, input)
    }

    fn utf8Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_char,
        to_lim: *const c_char,
    ) -> Utf8ConvertResult {
        utf8_toUtf8(enc, from_p, from_lim, to_p, to_lim)
    }

    fn utf16Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_ushort,
        to_lim: *const c_ushort,
    ) -> Utf16ConvertResult {
        utf8_toUtf16(enc, from_p, from_lim, to_p, to_lim)
    }
}

struct Latin1EncodingFunctions;

impl EncodingFunctions for Latin1EncodingFunctions {
    fn nameMatchesAscii(&self, enc: &ENCODING, input: &[c_char], kw: *const c_char) -> c_int {
        normal_nameMatchesAscii(enc, input, kw)
    }

    fn nameLength(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        normal_nameLength(enc, ptr)
    }

    fn skipS(&self, enc: &ENCODING, ptr: *const c_char) -> *const c_char {
        normal_skipS(enc, ptr)
    }

    fn getAtts(&self, enc: &ENCODING, ptr: *const c_char, n: c_int, atts: *mut ATTRIBUTE) -> c_int {
        normal_getAtts(enc, ptr, n, atts)
    }

    fn charRefNumber(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        normal_charRefNumber(enc, ptr)
    }

    fn predefinedEntityName(&self, enc: &ENCODING, input: &[c_char]) -> c_int {
        normal_predefinedEntityName(enc, input)
    }

    fn updatePosition(&self, enc: &ENCODING, input: &[c_char], pos: *mut POSITION) {
        normal_updatePosition(enc, input, pos);
    }

    fn isPublicId(&self, enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult {
        normal_isPublicId(enc, input)
    }

    fn utf8Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_char,
        to_lim: *const c_char,
    ) -> Utf8ConvertResult {
        latin1_toUtf8(enc, from_p, from_lim, to_p, to_lim)
    }

    fn utf16Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_ushort,
        to_lim: *const c_ushort,
    ) -> Utf16ConvertResult {
        latin1_toUtf16(enc, from_p, from_lim, to_p, to_lim)
    }
}

struct AsciiEncodingFunctions;

impl EncodingFunctions for AsciiEncodingFunctions {
    fn nameMatchesAscii(&self, enc: &ENCODING, input: &[c_char], kw: *const c_char) -> c_int {
        normal_nameMatchesAscii(enc, input, kw)
    }

    fn nameLength(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        normal_nameLength(enc, ptr)
    }

    fn skipS(&self, enc: &ENCODING, ptr: *const c_char) -> *const c_char {
        normal_skipS(enc, ptr)
    }

    fn getAtts(&self, enc: &ENCODING, ptr: *const c_char, n: c_int, atts: *mut ATTRIBUTE) -> c_int {
        normal_getAtts(enc, ptr, n, atts)
    }

    fn charRefNumber(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        normal_charRefNumber(enc, ptr)
    }

    fn predefinedEntityName(&self, enc: &ENCODING, input: &[c_char]) -> c_int {
        normal_predefinedEntityName(enc, input)
    }

    fn updatePosition(&self, enc: &ENCODING, input: &[c_char], pos: *mut POSITION) {
        normal_updatePosition(enc, input, pos);
    }

    fn isPublicId(&self, enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult {
        normal_isPublicId(enc, input)
    }

    fn utf8Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_char,
        to_lim: *const c_char,
    ) -> Utf8ConvertResult {
        ascii_toUtf8(enc, from_p, from_lim, to_p, to_lim)
    }

    fn utf16Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_ushort,
        to_lim: *const c_ushort,
    ) -> Utf16ConvertResult {
        latin1_toUtf16(enc, from_p, from_lim, to_p, to_lim)
    }
}

struct Little2EncodingFunctions;

impl EncodingFunctions for Little2EncodingFunctions {
    fn nameMatchesAscii(&self, enc: &ENCODING, input: &[c_char], kw: *const c_char) -> c_int {
        little2_nameMatchesAscii(enc, input, kw)
    }

    fn nameLength(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        little2_nameLength(enc, ptr)
    }

    fn skipS(&self, enc: &ENCODING, ptr: *const c_char) -> *const c_char {
        little2_skipS(enc, ptr)
    }

    fn getAtts(&self, enc: &ENCODING, ptr: *const c_char, n: c_int, atts: *mut ATTRIBUTE) -> c_int {
        little2_getAtts(enc, ptr, n, atts)
    }

    fn charRefNumber(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        little2_charRefNumber(enc, ptr)
    }

    fn predefinedEntityName(&self, enc: &ENCODING, input: &[c_char]) -> c_int {
        little2_predefinedEntityName(enc, input)
    }

    fn updatePosition(&self, enc: &ENCODING, input: &[c_char], pos: *mut POSITION) {
        little2_updatePosition(enc, input, pos);
    }

    fn isPublicId(&self, enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult {
        little2_isPublicId(enc, input)
    }

    fn utf8Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_char,
        to_lim: *const c_char,
    ) -> Utf8ConvertResult {
        little2_toUtf8(enc, from_p, from_lim, to_p, to_lim)
    }

    fn utf16Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_ushort,
        to_lim: *const c_ushort,
    ) -> Utf16ConvertResult {
        little2_toUtf16(enc, from_p, from_lim, to_p, to_lim)
    }
}

struct Big2EncodingFunctions;

impl EncodingFunctions for Big2EncodingFunctions {
    fn nameMatchesAscii(&self, enc: &ENCODING, input: &[c_char], kw: *const c_char) -> c_int {
        big2_nameMatchesAscii(enc, input, kw)
    }

    fn nameLength(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        big2_nameLength(enc, ptr)
    }

    fn skipS(&self, enc: &ENCODING, ptr: *const c_char) -> *const c_char {
        big2_skipS(enc, ptr)
    }

    fn getAtts(&self, enc: &ENCODING, ptr: *const c_char, n: c_int, atts: *mut ATTRIBUTE) -> c_int {
        big2_getAtts(enc, ptr, n, atts)
    }

    fn charRefNumber(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        big2_charRefNumber(enc, ptr)
    }

    fn predefinedEntityName(&self, enc: &ENCODING, input: &[c_char]) -> c_int {
        big2_predefinedEntityName(enc, input)
    }

    fn updatePosition(&self, enc: &ENCODING, input: &[c_char], pos: *mut POSITION) {
        big2_updatePosition(enc, input, pos);
    }

    fn isPublicId(&self, enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult {
        big2_isPublicId(enc, input)
    }

    fn utf8Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_char,
        to_lim: *const c_char,
    ) -> Utf8ConvertResult {
        big2_toUtf8(enc, from_p, from_lim, to_p, to_lim)
    }

    fn utf16Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_ushort,
        to_lim: *const c_ushort,
    ) -> Utf16ConvertResult {
        big2_toUtf16(enc, from_p, from_lim, to_p, to_lim)
    }
}

struct InitEncodingFunctions;

impl EncodingFunctions for InitEncodingFunctions {
    fn nameMatchesAscii(&self, enc: &ENCODING, input: &[c_char], kw: *const c_char) -> c_int {
        normal_nameMatchesAscii(enc, input, kw)
    }

    fn nameLength(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        normal_nameLength(enc, ptr)
    }

    fn skipS(&self, enc: &ENCODING, ptr: *const c_char) -> *const c_char {
        normal_skipS(enc, ptr)
    }

    fn getAtts(&self, enc: &ENCODING, ptr: *const c_char, n: c_int, atts: *mut ATTRIBUTE) -> c_int {
        normal_getAtts(enc, ptr, n, atts)
    }

    fn charRefNumber(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        normal_charRefNumber(enc, ptr)
    }

    fn predefinedEntityName(&self, enc: &ENCODING, input: &[c_char]) -> c_int {
        normal_predefinedEntityName(enc, input)
    }

    fn updatePosition(&self, enc: &ENCODING, input: &[c_char], pos: *mut POSITION) {
        initUpdatePosition(enc, input, pos);
    }

    fn isPublicId(&self, enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult {
        normal_isPublicId(enc, input)
    }

    fn utf8Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_char,
        to_lim: *const c_char,
    ) -> Utf8ConvertResult {
        utf8_toUtf8(enc, from_p, from_lim, to_p, to_lim)
    }

    fn utf16Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_ushort,
        to_lim: *const c_ushort,
    ) -> Utf16ConvertResult {
        utf8_toUtf16(enc, from_p, from_lim, to_p, to_lim)
    }
}

struct UnknownEncodingFunctions;

impl EncodingFunctions for UnknownEncodingFunctions {
    fn nameMatchesAscii(&self, enc: &ENCODING, input: &[c_char], kw: *const c_char) -> c_int {
        normal_nameMatchesAscii(enc, input, kw)
    }

    fn nameLength(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        normal_nameLength(enc, ptr)
    }

    fn skipS(&self, enc: &ENCODING, ptr: *const c_char) -> *const c_char {
        normal_skipS(enc, ptr)
    }

    fn getAtts(&self, enc: &ENCODING, ptr: *const c_char, n: c_int, atts: *mut ATTRIBUTE) -> c_int {
        normal_getAtts(enc, ptr, n, atts)
    }

    fn charRefNumber(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        normal_charRefNumber(enc, ptr)
    }

    fn predefinedEntityName(&self, enc: &ENCODING, input: &[c_char]) -> c_int {
        normal_predefinedEntityName(enc, input)
    }

    fn updatePosition(&self, enc: &ENCODING, input: &[c_char], pos: *mut POSITION) {
        normal_updatePosition(enc, input, pos);
    }

    fn isPublicId(&self, enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult {
        normal_isPublicId(enc, input)
    }

    fn utf8Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_char,
        to_lim: *const c_char,
    ) -> Utf8ConvertResult {
        unknown_toUtf8(enc, from_p, from_lim, to_p, to_lim)
    }

    fn utf16Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_ushort,
        to_lim: *const c_ushort,
    ) -> Utf16ConvertResult {
        unknown_toUtf16(enc, from_p, from_lim, to_p, to_lim)
    }
}

static UTF8_ENCODING_FUNCTIONS: Utf8EncodingFunctions = Utf8EncodingFunctions;
static LATIN1_ENCODING_FUNCTIONS: Latin1EncodingFunctions = Latin1EncodingFunctions;
static ASCII_ENCODING_FUNCTIONS: AsciiEncodingFunctions = AsciiEncodingFunctions;
static LITTLE2_ENCODING_FUNCTIONS: Little2EncodingFunctions = Little2EncodingFunctions;
static BIG2_ENCODING_FUNCTIONS: Big2EncodingFunctions = Big2EncodingFunctions;
static INIT_ENCODING_FUNCTIONS: InitEncodingFunctions = InitEncodingFunctions;
static UNKNOWN_ENCODING_FUNCTIONS: UnknownEncodingFunctions = UnknownEncodingFunctions;
static UTF8_NORMAL_ENCODING_CHECK_FUNCTIONS: Utf8NormalEncodingCheckFunctions =
    Utf8NormalEncodingCheckFunctions;
static UNKNOWN_NORMAL_ENCODING_CHECK_FUNCTIONS: UnknownNormalEncodingCheckFunctions =
    UnknownNormalEncodingCheckFunctions;
static MISSING_NORMAL_ENCODING_CHECK_FUNCTIONS: MissingNormalEncodingCheckFunctions =
    MissingNormalEncodingCheckFunctions;

static utf8_encoding_ns: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            normal_prologTok as SCANNER,
            normal_contentTok as SCANNER,
            normal_cdataSectionTok as SCANNER,
            normal_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            normal_attributeValueTok as SCANNER,
            normal_entityValueTok as SCANNER,
        ],
        functions: &UTF8_ENCODING_FUNCTIONS,
        minBytesPerChar: 1,
        isUtf8: 1i8,
        isUtf16: 0i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_CR as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_COLON_0 as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_MALFORM as c_uchar,
        BT_MALFORM as c_uchar,
    ],
    check_functions: &UTF8_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

static utf8_encoding: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            normal_prologTok as SCANNER,
            normal_contentTok as SCANNER,
            normal_cdataSectionTok as SCANNER,
            normal_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            normal_attributeValueTok as SCANNER,
            normal_entityValueTok as SCANNER,
        ],
        functions: &UTF8_ENCODING_FUNCTIONS,
        minBytesPerChar: 1,
        isUtf8: 1i8,
        isUtf16: 0i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_CR as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_MALFORM as c_uchar,
        BT_MALFORM as c_uchar,
    ],
    check_functions: &UTF8_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

static internal_utf8_encoding_ns: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            normal_prologTok as SCANNER,
            normal_contentTok as SCANNER,
            normal_cdataSectionTok as SCANNER,
            normal_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            normal_attributeValueTok as SCANNER,
            normal_entityValueTok as SCANNER,
        ],
        functions: &UTF8_ENCODING_FUNCTIONS,
        minBytesPerChar: 1,
        isUtf8: 1i8,
        isUtf16: 0i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_COLON_0 as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_MALFORM as c_uchar,
        BT_MALFORM as c_uchar,
    ],
    check_functions: &UTF8_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

static internal_utf8_encoding: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            normal_prologTok as SCANNER,
            normal_contentTok as SCANNER,
            normal_cdataSectionTok as SCANNER,
            normal_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            normal_attributeValueTok as SCANNER,
            normal_entityValueTok as SCANNER,
        ],
        functions: &UTF8_ENCODING_FUNCTIONS,
        minBytesPerChar: 1,
        isUtf8: 1i8,
        isUtf16: 0i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_MALFORM as c_uchar,
        BT_MALFORM as c_uchar,
    ],
    check_functions: &UTF8_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

fn latin1_toUtf8(
    _enc: &ENCODING,
    fromP: *const c_char,
    mut fromLim: *const c_char,
    toP: *mut c_char,
    mut toLim: *const c_char,
) -> Utf8ConvertResult {
    let mut from_cursor: *const c_char = fromP;
    let mut to_cursor: *mut c_char = toP;
    let fromP = &mut from_cursor;
    let toP = &mut to_cursor;
    loop {
        let c: c_uchar;
        if *fromP == fromLim {
            return (XML_CONVERT_COMPLETED, from_cursor, to_cursor);
        }
        c = read_c_char(*fromP) as c_uchar;
        if c as c_int & 0x80 != 0 {
            if c_char_ptr_diff(toLim, *toP as *const c_char) < 2 {
                return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
            }
            write_ptr(*toP, (c as c_int >> 6 | UTF8_cval2 as c_int) as c_char);
            *toP = (*toP).wrapping_offset(1);
            write_ptr(*toP, (c as c_int & 0x3f | 0x80) as c_char);
            *toP = (*toP).wrapping_offset(1);
            *fromP = (*fromP).wrapping_offset(1);
        } else {
            if std::ptr::eq(*toP, toLim) {
                return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
            }
            let src = *fromP;
            *fromP = (*fromP).wrapping_offset(1);
            write_ptr(*toP, read_c_char(src));
            *toP = (*toP).wrapping_offset(1);
        }
    }
}

fn latin1_toUtf16(
    _enc: &ENCODING,
    fromP: *const c_char,
    mut fromLim: *const c_char,
    toP: *mut c_ushort,
    mut toLim: *const c_ushort,
) -> Utf16ConvertResult {
    let mut from_cursor: *const c_char = fromP;
    let mut to_cursor: *mut c_ushort = toP;
    let fromP = &mut from_cursor;
    let toP = &mut to_cursor;
    while *fromP < fromLim && *toP < toLim as *mut c_ushort {
        let src = *fromP;
        *fromP = (*fromP).wrapping_offset(1);
        write_ptr(*toP, read_c_char(src) as c_uchar as c_ushort);
        *toP = (*toP).wrapping_offset(1);
    }
    if std::ptr::eq(*toP, toLim) && *fromP < fromLim {
        (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor)
    } else {
        (XML_CONVERT_COMPLETED, from_cursor, to_cursor)
    }
}

fn ascii_toUtf8(
    _enc: &ENCODING,
    fromP: *const c_char,
    mut fromLim: *const c_char,
    toP: *mut c_char,
    mut toLim: *const c_char,
) -> Utf8ConvertResult {
    let mut from_cursor: *const c_char = fromP;
    let mut to_cursor: *mut c_char = toP;
    let fromP = &mut from_cursor;
    let toP = &mut to_cursor;
    while *fromP < fromLim && *toP < toLim as *mut c_char {
        let src = *fromP;
        *fromP = (*fromP).wrapping_offset(1);
        write_ptr(*toP, read_c_char(src));
        *toP = (*toP).wrapping_offset(1);
    }
    if std::ptr::eq(*toP, toLim) && *fromP < fromLim {
        (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor)
    } else {
        (XML_CONVERT_COMPLETED, from_cursor, to_cursor)
    }
}

static latin1_encoding_ns: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            normal_prologTok as SCANNER,
            normal_contentTok as SCANNER,
            normal_cdataSectionTok as SCANNER,
            normal_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            normal_attributeValueTok as SCANNER,
            normal_entityValueTok as SCANNER,
        ],
        functions: &LATIN1_ENCODING_FUNCTIONS,
        minBytesPerChar: 1,
        isUtf8: 0i8,
        isUtf16: 0i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_CR as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_COLON_0 as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NAME as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
    ],
    check_functions: &MISSING_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

static latin1_encoding: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            normal_prologTok as SCANNER,
            normal_contentTok as SCANNER,
            normal_cdataSectionTok as SCANNER,
            normal_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            normal_attributeValueTok as SCANNER,
            normal_entityValueTok as SCANNER,
        ],
        functions: &LATIN1_ENCODING_FUNCTIONS,
        minBytesPerChar: 1,
        isUtf8: 0i8,
        isUtf16: 0i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_CR as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NAME as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
    ],
    check_functions: &MISSING_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

static ascii_encoding_ns: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            normal_prologTok as SCANNER,
            normal_contentTok as SCANNER,
            normal_cdataSectionTok as SCANNER,
            normal_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            normal_attributeValueTok as SCANNER,
            normal_entityValueTok as SCANNER,
        ],
        functions: &ASCII_ENCODING_FUNCTIONS,
        minBytesPerChar: 1,
        isUtf8: 1i8,
        isUtf16: 0i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_CR as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_COLON_0 as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    check_functions: &MISSING_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

static ascii_encoding: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            normal_prologTok as SCANNER,
            normal_contentTok as SCANNER,
            normal_cdataSectionTok as SCANNER,
            normal_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            normal_attributeValueTok as SCANNER,
            normal_entityValueTok as SCANNER,
        ],
        functions: &ASCII_ENCODING_FUNCTIONS,
        minBytesPerChar: 1,
        isUtf8: 1i8,
        isUtf16: 0i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_CR as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    check_functions: &MISSING_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

fn unicode_byte_type(mut hi: c_char, mut lo: c_char) -> c_int {
    match hi as c_uchar as c_int {
        216..=219 => return BT_LEAD4 as c_int,
        220..=223 => return BT_TRAIL as c_int,
        255 => match lo as c_uchar as c_int {
            255 | 254 => return BT_NONXML as c_int,
            _ => {}
        },
        _ => {}
    }
    BT_NONASCII as c_int
}

fn little2_toUtf8(
    _enc: &ENCODING,
    fromP: *const c_char,
    mut fromLim: *const c_char,
    toP: *mut c_char,
    mut toLim: *const c_char,
) -> Utf8ConvertResult {
    let mut from_cursor: *const c_char = fromP;
    let mut to_cursor: *mut c_char = toP;
    let fromP = &mut from_cursor;
    let toP = &mut to_cursor;
    let mut from: *const c_char = *fromP;
    let aligned_bytes = (c_char_ptr_diff(fromLim, from) >> 1) << 1;
    fromLim = from.wrapping_offset(aligned_bytes as isize);
    while from < fromLim {
        let lo: c_uchar = read_c_char_at(from, 0) as c_uchar;
        let hi: c_uchar = read_c_char_at(from, 1) as c_uchar;
        let mut encode_two_byte = false_0 != 0;
        match hi as c_int {
            0 => {
                if (lo as c_int) < 0x80 {
                    if c_char_ptr_diff(toLim, *toP as *const c_char) < 1 {
                        *fromP = from;
                        return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
                    }
                    write_ptr(*toP, lo as c_char);
                    *toP = (*toP).wrapping_offset(1);
                } else {
                    encode_two_byte = true_0 != 0;
                }
            }
            1..=7 => {
                encode_two_byte = true_0 != 0;
            }
            216..=219 => {
                if c_char_ptr_diff(toLim, *toP as *const c_char) < 4 {
                    *fromP = from;
                    return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
                }
                if c_char_ptr_diff(fromLim, from) < 4 {
                    *fromP = from;
                    return (XML_CONVERT_INPUT_INCOMPLETE, from_cursor, to_cursor);
                }
                let plane: c_int = ((hi as c_int & 0x3) << 2 | lo as c_int >> 6 & 0x3) + 1;
                write_ptr_at(*toP, 0, (plane >> 2 | UTF8_cval4 as c_int) as c_char);
                write_ptr_at(
                    *toP,
                    1,
                    (lo as c_int >> 2 & 0xf | (plane & 0x3) << 4 | 0x80) as c_char,
                );
                let from2 = from.wrapping_offset(2);
                let lo2: c_uchar = read_c_char_at(from2, 0) as c_uchar;
                let hi2: c_uchar = read_c_char_at(from2, 1) as c_uchar;
                write_ptr_at(
                    *toP,
                    2,
                    ((lo as c_int & 0x3) << 4
                        | (hi2 as c_int & 0x3) << 2
                        | lo2 as c_int >> 6
                        | 0x80) as c_char,
                );
                write_ptr_at(*toP, 3, (lo2 as c_int & 0x3f | 0x80) as c_char);
                *toP = (*toP).wrapping_offset(4);
                from = from2;
            }
            _ => {
                if c_char_ptr_diff(toLim, *toP as *const c_char) < 3 {
                    *fromP = from;
                    return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
                }
                write_ptr_at(*toP, 0, (hi as c_int >> 4 | UTF8_cval3 as c_int) as c_char);
                write_ptr_at(
                    *toP,
                    1,
                    ((hi as c_int & 0xf) << 2 | lo as c_int >> 6 | 0x80) as c_char,
                );
                write_ptr_at(*toP, 2, (lo as c_int & 0x3f | 0x80) as c_char);
                *toP = (*toP).wrapping_offset(3);
            }
        }
        if encode_two_byte {
            if c_char_ptr_diff(toLim, *toP as *const c_char) < 2 {
                *fromP = from;
                return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
            }
            write_ptr_at(
                *toP,
                0,
                (lo as c_int >> 6 | (hi as c_int) << 2 | UTF8_cval2 as c_int) as c_char,
            );
            write_ptr_at(*toP, 1, (lo as c_int & 0x3fi32 | 0x80i32) as c_char);
            *toP = (*toP).wrapping_offset(2);
        }
        from = from.wrapping_offset(2);
    }
    *fromP = from;
    if from < fromLim {
        (XML_CONVERT_INPUT_INCOMPLETE, from_cursor, to_cursor)
    } else {
        (XML_CONVERT_COMPLETED, from_cursor, to_cursor)
    }
}

fn little2_toUtf16(
    _enc: &ENCODING,
    fromP: *const c_char,
    mut fromLim: *const c_char,
    toP: *mut c_ushort,
    mut toLim: *const c_ushort,
) -> Utf16ConvertResult {
    let mut from_cursor: *const c_char = fromP;
    let mut to_cursor: *mut c_ushort = toP;
    let fromP = &mut from_cursor;
    let toP = &mut to_cursor;
    let mut res: XML_Convert_Result = XML_CONVERT_COMPLETED;
    let aligned_bytes = (c_char_ptr_diff(fromLim, *fromP) >> 1) << 1;
    fromLim = (*fromP).wrapping_offset(aligned_bytes as isize);
    if c_char_ptr_diff(fromLim, *fromP) > (c_ushort_ptr_diff(toLim, *toP as *const c_ushort) << 1)
        && (read_c_char_at(fromLim, -1) as c_uchar as c_int & 0xf8) == 0xd8
    {
        fromLim = fromLim.wrapping_offset(-2);
        res = XML_CONVERT_INPUT_INCOMPLETE;
    }
    while *fromP < fromLim && *toP < toLim as *mut c_ushort {
        let value = ((read_c_char_at(*fromP, 1) as c_uchar as c_int) << 8
            | read_c_char_at(*fromP, 0) as c_uchar as c_int) as c_ushort;
        write_ptr(*toP, value);
        *toP = (*toP).wrapping_offset(1);
        *fromP = (*fromP).wrapping_offset(2);
    }
    if std::ptr::eq(*toP, toLim) && *fromP < fromLim {
        (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor)
    } else {
        (res, from_cursor, to_cursor)
    }
}

fn big2_toUtf8(
    _enc: &ENCODING,
    fromP: *const c_char,
    mut fromLim: *const c_char,
    toP: *mut c_char,
    mut toLim: *const c_char,
) -> Utf8ConvertResult {
    let mut from_cursor: *const c_char = fromP;
    let mut to_cursor: *mut c_char = toP;
    let fromP = &mut from_cursor;
    let toP = &mut to_cursor;
    let mut from: *const c_char = *fromP;
    let aligned_bytes = (c_char_ptr_diff(fromLim, from) >> 1) << 1;
    fromLim = from.wrapping_offset(aligned_bytes as isize);
    while from < fromLim {
        let lo: c_uchar = read_c_char_at(from, 1) as c_uchar;
        let hi: c_uchar = read_c_char_at(from, 0) as c_uchar;
        let mut encode_two_byte = false_0 != 0;
        match hi as c_int {
            0 => {
                if (lo as c_int) < 0x80 {
                    if c_char_ptr_diff(toLim, *toP as *const c_char) < 1 {
                        *fromP = from;
                        return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
                    }
                    write_ptr(*toP, lo as c_char);
                    *toP = (*toP).wrapping_offset(1);
                } else {
                    encode_two_byte = true_0 != 0;
                }
            }
            1..=7 => {
                encode_two_byte = true_0 != 0;
            }
            216..=219 => {
                if c_char_ptr_diff(toLim, *toP as *const c_char) < 4 {
                    *fromP = from;
                    return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
                }
                if c_char_ptr_diff(fromLim, from) < 4 {
                    *fromP = from;
                    return (XML_CONVERT_INPUT_INCOMPLETE, from_cursor, to_cursor);
                }
                let plane: c_int = ((hi as c_int & 0x3) << 2 | lo as c_int >> 6 & 0x3) + 1;
                write_ptr_at(*toP, 0, (plane >> 2 | UTF8_cval4 as c_int) as c_char);
                write_ptr_at(
                    *toP,
                    1,
                    (lo as c_int >> 2 & 0xf | (plane & 0x3) << 4 | 0x80) as c_char,
                );
                let from2 = from.wrapping_offset(2);
                let lo2: c_uchar = read_c_char_at(from2, 1) as c_uchar;
                let hi2: c_uchar = read_c_char_at(from2, 0) as c_uchar;
                write_ptr_at(
                    *toP,
                    2,
                    ((lo as c_int & 0x3) << 4
                        | (hi2 as c_int & 0x3) << 2
                        | lo2 as c_int >> 6
                        | 0x80) as c_char,
                );
                write_ptr_at(*toP, 3, (lo2 as c_int & 0x3f | 0x80) as c_char);
                *toP = (*toP).wrapping_offset(4);
                from = from2;
            }
            _ => {
                if c_char_ptr_diff(toLim, *toP as *const c_char) < 3 {
                    *fromP = from;
                    return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
                }
                write_ptr_at(*toP, 0, (hi as c_int >> 4 | UTF8_cval3 as c_int) as c_char);
                write_ptr_at(
                    *toP,
                    1,
                    ((hi as c_int & 0xf) << 2 | lo as c_int >> 6 | 0x80) as c_char,
                );
                write_ptr_at(*toP, 2, (lo as c_int & 0x3f | 0x80) as c_char);
                *toP = (*toP).wrapping_offset(3);
            }
        }
        if encode_two_byte {
            if c_char_ptr_diff(toLim, *toP as *const c_char) < 2 {
                *fromP = from;
                return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
            }
            write_ptr_at(
                *toP,
                0,
                (lo as c_int >> 6 | (hi as c_int) << 2 | UTF8_cval2 as c_int) as c_char,
            );
            write_ptr_at(*toP, 1, (lo as c_int & 0x3fi32 | 0x80i32) as c_char);
            *toP = (*toP).wrapping_offset(2);
        }
        from = from.wrapping_offset(2);
    }
    *fromP = from;
    if from < fromLim {
        (XML_CONVERT_INPUT_INCOMPLETE, from_cursor, to_cursor)
    } else {
        (XML_CONVERT_COMPLETED, from_cursor, to_cursor)
    }
}

fn big2_toUtf16(
    _enc: &ENCODING,
    fromP: *const c_char,
    mut fromLim: *const c_char,
    toP: *mut c_ushort,
    mut toLim: *const c_ushort,
) -> Utf16ConvertResult {
    let mut from_cursor: *const c_char = fromP;
    let mut to_cursor: *mut c_ushort = toP;
    let fromP = &mut from_cursor;
    let toP = &mut to_cursor;
    let mut res: XML_Convert_Result = XML_CONVERT_COMPLETED;
    let aligned_bytes = (c_char_ptr_diff(fromLim, *fromP) >> 1) << 1;
    fromLim = (*fromP).wrapping_offset(aligned_bytes as isize);
    if c_char_ptr_diff(fromLim, *fromP) > (c_ushort_ptr_diff(toLim, *toP as *const c_ushort) << 1)
        && (read_c_char_at(fromLim, -2) as c_uchar as c_int & 0xf8) == 0xd8
    {
        fromLim = fromLim.wrapping_offset(-2);
        res = XML_CONVERT_INPUT_INCOMPLETE;
    }
    while *fromP < fromLim && *toP < toLim as *mut c_ushort {
        let value = ((read_c_char_at(*fromP, 0) as c_uchar as c_int) << 8
            | read_c_char_at(*fromP, 1) as c_uchar as c_int) as c_ushort;
        write_ptr(*toP, value);
        *toP = (*toP).wrapping_offset(1);
        *fromP = (*fromP).wrapping_offset(2);
    }
    if std::ptr::eq(*toP, toLim) && *fromP < fromLim {
        (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor)
    } else {
        (res, from_cursor, to_cursor)
    }
}

static little2_encoding_ns: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            little2_prologTok as SCANNER,
            little2_contentTok as SCANNER,
            little2_cdataSectionTok as SCANNER,
            little2_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            little2_attributeValueTok as SCANNER,
            little2_entityValueTok as SCANNER,
        ],
        functions: &LITTLE2_ENCODING_FUNCTIONS,
        minBytesPerChar: 2,
        isUtf8: 0i8,
        isUtf16: 1i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_CR as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_COLON_0 as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NAME as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
    ],
    check_functions: &MISSING_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

static little2_encoding: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            little2_prologTok as SCANNER,
            little2_contentTok as SCANNER,
            little2_cdataSectionTok as SCANNER,
            little2_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            little2_attributeValueTok as SCANNER,
            little2_entityValueTok as SCANNER,
        ],
        functions: &LITTLE2_ENCODING_FUNCTIONS,
        minBytesPerChar: 2,
        isUtf8: 0i8,
        isUtf16: 1i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_CR as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NAME as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
    ],
    check_functions: &MISSING_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

static internal_little2_encoding_ns: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            little2_prologTok as SCANNER,
            little2_contentTok as SCANNER,
            little2_cdataSectionTok as SCANNER,
            little2_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            little2_attributeValueTok as SCANNER,
            little2_entityValueTok as SCANNER,
        ],
        functions: &LITTLE2_ENCODING_FUNCTIONS,
        minBytesPerChar: 2,
        isUtf8: 0i8,
        isUtf16: 1i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_COLON_0 as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NAME as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
    ],
    check_functions: &MISSING_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

static internal_little2_encoding: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            little2_prologTok as SCANNER,
            little2_contentTok as SCANNER,
            little2_cdataSectionTok as SCANNER,
            little2_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            little2_attributeValueTok as SCANNER,
            little2_entityValueTok as SCANNER,
        ],
        functions: &LITTLE2_ENCODING_FUNCTIONS,
        minBytesPerChar: 2,
        isUtf8: 0i8,
        isUtf16: 1i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NAME as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
    ],
    check_functions: &MISSING_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

static big2_encoding_ns: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            big2_prologTok as SCANNER,
            big2_contentTok as SCANNER,
            big2_cdataSectionTok as SCANNER,
            big2_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            big2_attributeValueTok as SCANNER,
            big2_entityValueTok as SCANNER,
        ],
        functions: &BIG2_ENCODING_FUNCTIONS,
        minBytesPerChar: 2,
        isUtf8: 0i8,
        isUtf16: 0i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_CR as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_COLON_0 as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NAME as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
    ],
    check_functions: &MISSING_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

static big2_encoding: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            big2_prologTok as SCANNER,
            big2_contentTok as SCANNER,
            big2_cdataSectionTok as SCANNER,
            big2_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            big2_attributeValueTok as SCANNER,
            big2_entityValueTok as SCANNER,
        ],
        functions: &BIG2_ENCODING_FUNCTIONS,
        minBytesPerChar: 2,
        isUtf8: 0i8,
        isUtf16: 0i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_CR as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NAME as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
    ],
    check_functions: &MISSING_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

fn streqci(mut s1: *const c_char, mut s2: *const c_char) -> c_int {
    loop {
        let mut c1: c_char = read_c_char(s1);
        s1 = s1.wrapping_offset(1);
        let mut c2: c_char = read_c_char(s2);
        s2 = s2.wrapping_offset(1);
        if ASCII_a_1 <= c1 as c_int && c1 as c_int <= ASCII_z {
            c1 = (c1 as c_int + (ASCII_A - ASCII_a_1)) as c_char;
        }
        if ASCII_a_1 <= c2 as c_int && c2 as c_int <= ASCII_z {
            c2 = (c2 as c_int + (ASCII_A - ASCII_a_1)) as c_char;
        }
        if c1 as c_int != c2 as c_int {
            return 0i32;
        }
        if c1 == 0 {
            break;
        }
    }
    1
}

fn initUpdatePosition(_enc: &ENCODING, input: &[c_char], mut pos: *mut POSITION) {
    normal_updatePosition(&utf8_encoding.enc, input, pos);
}

fn toAscii(enc: &ENCODING, input: &[c_char]) -> c_int {
    let end = input.len();
    let mut buf: [c_char; 1] = [0; 1];
    let buf_start = buf.as_mut_ptr();
    let mut p: *mut c_char = buf_start;
    let input_ptr = input.as_ptr();
    (_, _, p) = enc.utf8Convert(
        enc,
        input_ptr,
        input_ptr.wrapping_add(end),
        p,
        p.wrapping_offset(1),
    );
    if p == buf_start {
        -(1i32)
    } else {
        buf[0usize] as c_int
    }
}

fn isSpace(mut c: c_int) -> c_int {
    match c {
        32 | 13 | 10 | 9 => return 1,
        _ => {}
    }
    0
}

fn parsePseudoAttribute(enc: &ENCODING, input: &[c_char]) -> ParsePseudoAttributeResult {
    let mut ptr = 0usize;
    let end = input.len();
    let mut c: c_int = 0;
    let mut open: c_char = 0;
    let mut name: *const c_char = null::<c_char>();
    let mut nameEnd: *const c_char = null::<c_char>();
    let mut val: *const c_char = null::<c_char>();
    let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
    if ptr == end {
        return (
            1,
            null::<c_char>(),
            null::<c_char>(),
            null::<c_char>(),
            ptr_to(ptr),
        );
    }
    if isSpace(toAscii(enc, &input[ptr..end])) == 0 {
        return (0, name, nameEnd, val, ptr_to(ptr));
    }
    loop {
        ptr = ptr.wrapping_offset(enc.minBytesPerChar as isize);
        if isSpace(toAscii(enc, &input[ptr..end])) == 0 {
            break;
        }
    }
    if ptr == end {
        return (
            1,
            null::<c_char>(),
            null::<c_char>(),
            null::<c_char>(),
            ptr_to(ptr),
        );
    }
    name = ptr_to(ptr);
    loop {
        c = toAscii(enc, &input[ptr..end]);
        if c == -(1) {
            return (0, name, nameEnd, val, ptr_to(ptr));
        }
        if c == ASCII_EQUALS {
            nameEnd = ptr_to(ptr);
            break;
        } else if isSpace(c) != 0 {
            nameEnd = ptr_to(ptr);
            loop {
                ptr = ptr.wrapping_offset(enc.minBytesPerChar as isize);
                c = toAscii(enc, &input[ptr..end]);
                if isSpace(c) == 0 {
                    break;
                }
            }
            if c != ASCII_EQUALS {
                return (0, name, nameEnd, val, ptr_to(ptr));
            }
            break;
        } else {
            ptr = ptr.wrapping_offset(enc.minBytesPerChar as isize);
        }
    }
    if ptr_to(ptr) == name {
        return (0, name, nameEnd, val, ptr_to(ptr));
    }
    ptr = ptr.wrapping_offset(enc.minBytesPerChar as isize);
    c = toAscii(enc, &input[ptr..end]);
    while isSpace(c) != 0 {
        ptr = ptr.wrapping_offset(enc.minBytesPerChar as isize);
        c = toAscii(enc, &input[ptr..end]);
    }
    if c != ASCII_QUOT && c != ASCII_APOS {
        return (0, name, nameEnd, val, ptr_to(ptr));
    }
    open = c as c_char;
    ptr = ptr.wrapping_offset(enc.minBytesPerChar as isize);
    val = ptr_to(ptr);
    loop {
        c = toAscii(enc, &input[ptr..end]);
        if c == open as c_int {
            break;
        }
        if !(ASCII_a_1..=ASCII_z).contains(&c)
            && !(ASCII_A..=ASCII_Z).contains(&c)
            && !(ASCII_0..=ASCII_9_1).contains(&c)
            && c != ASCII_PERIOD
            && c != ASCII_MINUS
            && c != ASCII_UNDERSCORE
        {
            return (0, name, nameEnd, val, ptr_to(ptr));
        }
        ptr = ptr.wrapping_offset(enc.minBytesPerChar as isize);
    }
    (
        1,
        name,
        nameEnd,
        val,
        ptr_to(ptr.wrapping_offset(enc.minBytesPerChar as isize)),
    )
}

static KW_version: [c_char; 8] = [
    ASCII_v as c_char,
    ASCII_e_1 as c_char,
    ASCII_r as c_char,
    ASCII_s as c_char,
    ASCII_i as c_char,
    ASCII_o as c_char,
    ASCII_n as c_char,
    '\0' as c_char,
];

static KW_encoding: [c_char; 9] = [
    ASCII_e_1 as c_char,
    ASCII_n as c_char,
    ASCII_c_1 as c_char,
    ASCII_o as c_char,
    ASCII_d as c_char,
    ASCII_i as c_char,
    ASCII_n as c_char,
    ASCII_g_1 as c_char,
    '\0' as c_char,
];

static KW_standalone: [c_char; 11] = [
    ASCII_s as c_char,
    ASCII_t as c_char,
    ASCII_a_1 as c_char,
    ASCII_n as c_char,
    ASCII_d as c_char,
    ASCII_a_1 as c_char,
    ASCII_l_1 as c_char,
    ASCII_o as c_char,
    ASCII_n as c_char,
    ASCII_e_1 as c_char,
    '\0' as c_char,
];

static KW_yes: [c_char; 4] = [
    ASCII_y as c_char,
    ASCII_e_1 as c_char,
    ASCII_s as c_char,
    '\0' as c_char,
];

static KW_no: [c_char; 3] = [ASCII_n as c_char, ASCII_o as c_char, '\0' as c_char];

fn doParseXmlDecl(
    mut encodingFinder: Option<fn(&ENCODING, &[c_char]) -> *const ENCODING>,
    mut isGeneralTextEntity: c_int,
    enc: &ENCODING,
    input: &[c_char],
) -> ParseXmlDeclResult {
    let input_ptr = input.as_ptr();
    let ptr_to = |ptr: usize| input_ptr.wrapping_add(ptr);
    let ptr_from = |ptr: *const c_char| c_char_ptr_diff(ptr, input_ptr) as usize;
    let mut ptr = 0usize;
    let mut end = input.len();
    let mut val: *const c_char = null::<c_char>();
    let mut name: *const c_char = null::<c_char>();
    let mut nameEnd: *const c_char = null::<c_char>();
    let mut badPtr: *const c_char = null::<c_char>();
    let mut versionPtr: *const c_char = null::<c_char>();
    let mut versionEndPtr: *const c_char = null::<c_char>();
    let mut encodingName: *const c_char = null::<c_char>();
    let mut encoding: *const ENCODING = null::<ENCODING>();
    let mut standalone: c_int = -(1);
    let mut ok: c_int = 0;
    let mut ptr_value: *const c_char = null::<c_char>();
    ptr = ptr.wrapping_offset((5i32 * enc.minBytesPerChar) as isize);
    end = end.wrapping_offset(-((2i32 * enc.minBytesPerChar) as isize));
    (ok, name, nameEnd, val, ptr_value) = parsePseudoAttribute(enc, &input[ptr..end]);
    ptr = ptr_from(ptr_value);
    if ok == 0 || name.is_null() {
        badPtr = ptr_to(ptr);
        return (
            0,
            badPtr,
            versionPtr,
            versionEndPtr,
            encodingName,
            encoding,
            standalone,
        );
    }
    if enc.nameMatchesAscii(
        enc,
        c_char_slice_from_ptr_end(name, nameEnd),
        &raw const KW_version as *const c_char,
    ) == 0
    {
        if isGeneralTextEntity == 0 {
            badPtr = name;
            return (
                0,
                badPtr,
                versionPtr,
                versionEndPtr,
                encodingName,
                encoding,
                standalone,
            );
        }
    } else {
        versionPtr = val;
        versionEndPtr = ptr_to(ptr);
        (ok, name, nameEnd, val, ptr_value) = parsePseudoAttribute(enc, &input[ptr..end]);
        ptr = ptr_from(ptr_value);
        if ok == 0 {
            badPtr = ptr_to(ptr);
            return (
                0,
                badPtr,
                versionPtr,
                versionEndPtr,
                encodingName,
                encoding,
                standalone,
            );
        }
        if name.is_null() {
            if isGeneralTextEntity != 0 {
                badPtr = ptr_to(ptr);
                return (
                    0,
                    badPtr,
                    versionPtr,
                    versionEndPtr,
                    encodingName,
                    encoding,
                    standalone,
                );
            }
            return (
                1,
                badPtr,
                versionPtr,
                versionEndPtr,
                encodingName,
                encoding,
                standalone,
            );
        }
    }
    if enc.nameMatchesAscii(
        enc,
        c_char_slice_from_ptr_end(name, nameEnd),
        &raw const KW_encoding as *const c_char,
    ) != 0
    {
        let c: c_int = toAscii(enc, c_char_slice_from_ptr_end(val, ptr_to(end)));
        if !(ASCII_a_1..=ASCII_z).contains(&c) && !(ASCII_A..=ASCII_Z).contains(&c) {
            badPtr = val;
            return (
                0,
                badPtr,
                versionPtr,
                versionEndPtr,
                encodingName,
                encoding,
                standalone,
            );
        }
        encodingName = val;
        encoding = encodingFinder.expect("non-null function pointer")(
            enc,
            c_char_slice_from_ptr_end(
                val,
                ptr_to(ptr.wrapping_offset(-((enc.minBytesPerChar) as isize))),
            ),
        );
        (ok, name, nameEnd, val, ptr_value) = parsePseudoAttribute(enc, &input[ptr..end]);
        ptr = ptr_from(ptr_value);
        if ok == 0 {
            badPtr = ptr_to(ptr);
            return (
                0,
                badPtr,
                versionPtr,
                versionEndPtr,
                encodingName,
                encoding,
                standalone,
            );
        }
        if name.is_null() {
            return (
                1,
                badPtr,
                versionPtr,
                versionEndPtr,
                encodingName,
                encoding,
                standalone,
            );
        }
    }
    if enc.nameMatchesAscii(
        enc,
        c_char_slice_from_ptr_end(name, nameEnd),
        &raw const KW_standalone as *const c_char,
    ) == 0
        || isGeneralTextEntity != 0
    {
        badPtr = name;
        return (
            0,
            badPtr,
            versionPtr,
            versionEndPtr,
            encodingName,
            encoding,
            standalone,
        );
    }
    if enc.nameMatchesAscii(
        enc,
        c_char_slice_from_ptr_end(
            val,
            ptr_to(ptr.wrapping_offset(-((enc.minBytesPerChar) as isize))),
        ),
        &raw const KW_yes as *const c_char,
    ) != 0
    {
        standalone = 1i32;
    } else if enc.nameMatchesAscii(
        enc,
        c_char_slice_from_ptr_end(
            val,
            ptr_to(ptr.wrapping_offset(-((enc.minBytesPerChar) as isize))),
        ),
        &raw const KW_no as *const c_char,
    ) != 0
    {
        standalone = 0i32;
    } else {
        badPtr = val;
        return (
            0,
            badPtr,
            versionPtr,
            versionEndPtr,
            encodingName,
            encoding,
            standalone,
        );
    }
    while isSpace(toAscii(enc, &input[ptr..end])) != 0 {
        ptr = ptr.wrapping_offset(enc.minBytesPerChar as isize);
    }
    if ptr != end {
        badPtr = ptr_to(ptr);
        return (
            0,
            badPtr,
            versionPtr,
            versionEndPtr,
            encodingName,
            encoding,
            standalone,
        );
    }
    (
        1,
        badPtr,
        versionPtr,
        versionEndPtr,
        encodingName,
        encoding,
        standalone,
    )
}

fn checkCharRefNumber(mut result: c_int) -> c_int {
    match result >> 8 {
        216..=223 => {
            return -(1i32);
        }
        0 => {
            if latin1_encoding.type_0[result as usize] as c_int == BT_NONXML as c_int {
                return -(1i32);
            }
        }
        255 => {
            if result == 0xfffe || result == 0xffff {
                return -(1i32);
            }
        }
        _ => {}
    }
    result
}
pub(crate) fn XmlUtf8Encode(mut c: c_int, mut buf: *mut c_char) -> c_int {
    if c < 0 {
        return 0i32;
    }
    if c < min2 as c_int {
        write_ptr_at(buf, 0, (c | UTF8_cval1 as c_int) as c_char);
        return 1i32;
    }
    if c < min3 as c_int {
        write_ptr_at(buf, 0, (c >> 6 | UTF8_cval2 as c_int) as c_char);
        write_ptr_at(buf, 1, (c & 0x3fi32 | 0x80) as c_char);
        return 2i32;
    }
    if c < min4 as c_int {
        write_ptr_at(buf, 0, (c >> 12 | UTF8_cval3 as c_int) as c_char);
        write_ptr_at(buf, 1, (c >> 6 & 0x3fi32 | 0x80) as c_char);
        write_ptr_at(buf, 2, (c & 0x3fi32 | 0x80) as c_char);
        return 3i32;
    }
    if c < 0x110000 {
        write_ptr_at(buf, 0, (c >> 18 | UTF8_cval4 as c_int) as c_char);
        write_ptr_at(buf, 1, (c >> 12 & 0x3fi32 | 0x80) as c_char);
        write_ptr_at(buf, 2, (c >> 6 & 0x3fi32 | 0x80) as c_char);
        write_ptr_at(buf, 3, (c & 0x3fi32 | 0x80) as c_char);
        return 4i32;
    }
    0
}
pub(crate) fn XmlUtf16Encode(mut charNum: c_int, mut buf: *mut c_ushort) -> c_int {
    if charNum < 0 {
        return 0i32;
    }
    if charNum < 0x10000 {
        write_ptr_at(buf, 0, charNum as c_ushort);
        return 1i32;
    }
    if charNum < 0x110000 {
        charNum -= 0x10000;
        write_ptr_at(buf, 0, ((charNum >> 10) + 0xd800i32) as c_ushort);
        write_ptr_at(buf, 1, ((charNum & 0x3ffi32) + 0xdc00) as c_ushort);
        return 2i32;
    }
    0
}
pub(crate) fn XmlSizeOfUnknownEncoding() -> c_int {
    size_of::<unknown_encoding>() as c_int
}

fn unknown_isName(enc: &ENCODING, mut p: *const c_char) -> c_int {
    let uenc: &unknown_encoding = as_unknown_encoding(enc);
    let mut c: c_int = uenc.convert.expect("non-null function pointer")(uenc.userData, p);
    if c & !(0xffff) != 0 {
        return 0i32;
    }
    (namingBitmap[(((namePages[(c >> 8) as usize] as c_int) << 3) + ((c & 0xff) >> 5)) as usize]
        & (1) << (c & 0xff & 0x1f)) as c_int
}

fn unknown_isNmstrt(enc: &ENCODING, mut p: *const c_char) -> c_int {
    let uenc: &unknown_encoding = as_unknown_encoding(enc);
    let mut c: c_int = uenc.convert.expect("non-null function pointer")(uenc.userData, p);
    if c & !(0xffff) != 0 {
        return 0i32;
    }
    (namingBitmap[(((nmstrtPages[(c >> 8) as usize] as c_int) << 3) + ((c & 0xff) >> 5)) as usize]
        & (1) << (c & 0xff & 0x1f)) as c_int
}

fn unknown_isInvalid(enc: &ENCODING, mut p: *const c_char) -> c_int {
    let uenc: &unknown_encoding = as_unknown_encoding(enc);
    let mut c: c_int = uenc.convert.expect("non-null function pointer")(uenc.userData, p);
    (c & !(0xffff) != 0 || checkCharRefNumber(c) < 0) as c_int
}

fn unknown_toUtf8(
    enc: &ENCODING,
    fromP: *const c_char,
    mut fromLim: *const c_char,
    toP: *mut c_char,
    mut toLim: *const c_char,
) -> Utf8ConvertResult {
    let mut from_cursor: *const c_char = fromP;
    let mut to_cursor: *mut c_char = toP;
    let fromP = &mut from_cursor;
    let toP = &mut to_cursor;
    let uenc: &unknown_encoding = as_unknown_encoding(enc);
    let mut buf: [c_char; 4] = [0; 4];
    loop {
        if *fromP == fromLim {
            return (XML_CONVERT_COMPLETED, from_cursor, to_cursor);
        }
        let source_byte = read_c_char(*fromP) as c_uchar as usize;
        let entry = &uenc.utf8[source_byte];
        let mut n: c_int = entry[0] as c_int;
        let mut use_buf = false_0 != 0;
        if n == 0 {
            let c: c_int = uenc.convert.expect("non-null function pointer")(uenc.userData, *fromP);
            n = XmlUtf8Encode(c, buf.as_mut_ptr());
            if n as c_long > c_char_ptr_diff(toLim, *toP as *const c_char) {
                return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
            }
            use_buf = true_0 != 0;
            *fromP = (*fromP).wrapping_offset(
                (as_normal_encoding(enc).type_0[source_byte] as c_int - (BT_LEAD2 as c_int - 2i32))
                    as isize,
            );
        } else {
            if n as c_long > c_char_ptr_diff(toLim, *toP as *const c_char) {
                return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
            }
            *fromP = (*fromP).wrapping_offset(1);
        }
        let mut i: c_int = 0;
        while i < n {
            let value = if use_buf {
                buf[i as usize]
            } else {
                entry[(i + 1) as usize]
            };
            write_ptr_at(*toP, i as isize, value);
            i += 1;
        }
        *toP = (*toP).wrapping_offset(n as isize);
    }
}

fn unknown_toUtf16(
    enc: &ENCODING,
    fromP: *const c_char,
    mut fromLim: *const c_char,
    toP: *mut c_ushort,
    mut toLim: *const c_ushort,
) -> Utf16ConvertResult {
    let mut from_cursor: *const c_char = fromP;
    let mut to_cursor: *mut c_ushort = toP;
    let fromP = &mut from_cursor;
    let toP = &mut to_cursor;
    let uenc: &unknown_encoding = as_unknown_encoding(enc);
    while *fromP < fromLim && *toP < toLim as *mut c_ushort {
        let source_byte = read_c_char(*fromP) as c_uchar as usize;
        let mut c: c_ushort = uenc.utf16[source_byte];
        if c as c_int == 0 {
            c = uenc.convert.expect("non-null function pointer")(uenc.userData, *fromP) as c_ushort;
            *fromP = (*fromP).wrapping_offset(
                (as_normal_encoding(enc).type_0[source_byte] as c_int - (BT_LEAD2 as c_int - 2i32))
                    as isize,
            );
        } else {
            *fromP = (*fromP).wrapping_offset(1);
        }
        write_ptr(*toP, c);
        *toP = (*toP).wrapping_offset(1);
    }
    if std::ptr::eq(*toP as *const c_ushort, toLim) && *fromP < fromLim {
        (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor)
    } else {
        (XML_CONVERT_COMPLETED, from_cursor, to_cursor)
    }
}
pub(crate) fn XmlInitUnknownEncoding(
    mut mem: *mut c_void,
    mut table: *const c_int,
    mut convert: CONVERTER,
    mut userData: *mut c_void,
) -> *mut ENCODING {
    let e: &mut unknown_encoding = mut_ref_from_ptr(mem as *mut unknown_encoding);
    e.normal = latin1_encoding;
    let mut i: c_int = 0;
    while i < 128 {
        if latin1_encoding.type_0[i as usize] as c_int != BT_OTHER as c_int
            && latin1_encoding.type_0[i as usize] as c_int != BT_NONXML as c_int
            && read_ptr_at(table, i as isize) != i
        {
            return null_mut::<ENCODING>();
        }
        i += 1;
    }
    i = 0;
    while i < 256 {
        let c: c_int = read_ptr_at(table, i as isize);
        if c == -(1) {
            e.normal.type_0[i as usize] = BT_MALFORM as c_uchar;
            e.utf16[i as usize] = 0xffff;
            e.utf8[i as usize][0] = 1;
            e.utf8[i as usize][1usize] = 0i8;
        } else if c < 0 {
            if c < -(4) {
                return null_mut::<ENCODING>();
            }
            if convert.is_none() {
                return null_mut::<ENCODING>();
            }
            e.normal.type_0[i as usize] = (BT_LEAD2 as c_int - (c + 2)) as c_uchar;
            e.utf8[i as usize][0] = 0;
            e.utf16[i as usize] = 0u16;
        } else if c < 0x80 {
            if latin1_encoding.type_0[c as usize] as c_int != BT_OTHER as c_int
                && latin1_encoding.type_0[c as usize] as c_int != BT_NONXML as c_int
                && c != i
            {
                return null_mut::<ENCODING>();
            }
            e.normal.type_0[i as usize] = latin1_encoding.type_0[c as usize];
            e.utf8[i as usize][0] = 1;
            e.utf8[i as usize][1] = c as c_char;
            e.utf16[i as usize] = (if c == 0i32 { 0xffffi32 } else { c }) as c_ushort;
        } else if checkCharRefNumber(c) < 0 {
            e.normal.type_0[i as usize] = BT_NONXML as c_uchar;
            e.utf16[i as usize] = 0xffff;
            e.utf8[i as usize][0] = 1;
            e.utf8[i as usize][1usize] = 0i8;
        } else {
            if c > 0xffff {
                return null_mut::<ENCODING>();
            }
            if namingBitmap
                [(((nmstrtPages[(c >> 8) as usize] as c_int) << 3) + ((c & 0xff) >> 5)) as usize]
                & (1) << (c & 0xff & 0x1f)
                != 0
            {
                e.normal.type_0[i as usize] = BT_NMSTRT as c_uchar;
            } else if namingBitmap
                [(((namePages[(c >> 8) as usize] as c_int) << 3) + ((c & 0xff) >> 5)) as usize]
                & (1) << (c & 0xff & 0x1f)
                != 0
            {
                e.normal.type_0[i as usize] = BT_NAME as c_uchar;
            } else {
                e.normal.type_0[i as usize] = BT_OTHER as c_uchar;
            }
            e.utf8[i as usize][0] =
                XmlUtf8Encode(c, e.utf8[i as usize].as_mut_ptr().wrapping_offset(1)) as c_char;
            e.utf16[i as usize] = c as c_ushort;
        }
        i += 1;
    }
    e.userData = userData;
    e.convert = convert;
    if convert.is_some() {
        e.normal.check_functions = &UNKNOWN_NORMAL_ENCODING_CHECK_FUNCTIONS;
    }
    e.normal.enc.functions = &UNKNOWN_ENCODING_FUNCTIONS;
    &raw mut e.normal.enc
}

static KW_ISO_8859_1: [c_char; 11] = [
    ASCII_I as c_char,
    ASCII_S as c_char,
    ASCII_O as c_char,
    ASCII_MINUS as c_char,
    ASCII_8_1 as c_char,
    ASCII_8_1 as c_char,
    ASCII_5 as c_char,
    ASCII_9_1 as c_char,
    ASCII_MINUS as c_char,
    ASCII_1_1 as c_char,
    '\0' as c_char,
];

static KW_US_ASCII: [c_char; 9] = [
    ASCII_U as c_char,
    ASCII_S as c_char,
    ASCII_MINUS as c_char,
    ASCII_A as c_char,
    ASCII_S as c_char,
    ASCII_C as c_char,
    ASCII_I as c_char,
    ASCII_I as c_char,
    '\0' as c_char,
];

static KW_UTF_8: [c_char; 6] = [
    ASCII_U as c_char,
    ASCII_T as c_char,
    ASCII_F_1 as c_char,
    ASCII_MINUS as c_char,
    ASCII_8_1 as c_char,
    '\0' as c_char,
];

static KW_UTF_16: [c_char; 7] = [
    ASCII_U as c_char,
    ASCII_T as c_char,
    ASCII_F_1 as c_char,
    ASCII_MINUS as c_char,
    ASCII_1_1 as c_char,
    ASCII_6 as c_char,
    '\0' as c_char,
];

static KW_UTF_16BE: [c_char; 9] = [
    ASCII_U as c_char,
    ASCII_T as c_char,
    ASCII_F_1 as c_char,
    ASCII_MINUS as c_char,
    ASCII_1_1 as c_char,
    ASCII_6 as c_char,
    ASCII_B_1 as c_char,
    ASCII_E_1 as c_char,
    '\0' as c_char,
];

static KW_UTF_16LE: [c_char; 9] = [
    ASCII_U as c_char,
    ASCII_T as c_char,
    ASCII_F_1 as c_char,
    ASCII_MINUS as c_char,
    ASCII_1_1 as c_char,
    ASCII_6 as c_char,
    ASCII_L_1 as c_char,
    ASCII_E_1 as c_char,
    '\0' as c_char,
];

fn getEncodingIndex(mut name: *const c_char) -> c_int {
    let encodingNames: [*const c_char; 6] = [
        &raw const KW_ISO_8859_1 as *const c_char,
        &raw const KW_US_ASCII as *const c_char,
        &raw const KW_UTF_8 as *const c_char,
        &raw const KW_UTF_16 as *const c_char,
        &raw const KW_UTF_16BE as *const c_char,
        &raw const KW_UTF_16LE as *const c_char,
    ];
    let mut i: c_int = 0;
    if name.is_null() {
        return NO_ENC;
    }
    i = 0;
    while i < (size_of::<[*const c_char; 6]>()).wrapping_div(size_of::<*const c_char>()) as c_int {
        if streqci(name, encodingNames[i as usize]) != 0 {
            return i;
        }
        i += 1;
    }
    UNKNOWN_ENC
}

fn initScan(
    mut encodingTable: *const *const ENCODING,
    enc: &INIT_ENCODING,
    mut state: c_int,
    input: &[c_char],
) -> ScannerResult {
    let mut next_tok: *const c_char = input.as_ptr();
    let nextTokPtr = &mut next_tok;
    let ptr_to = |ptr: usize| input.as_ptr().wrapping_add(ptr);
    let scan_current = |next_tok_ptr: &mut *const c_char,
                        enc_ptr_ptr: *mut *const ENCODING,
                        state: c_int,
                        ptr: usize,
                        end: usize|
     -> c_int {
        let enc_ptr = read_ptr(enc_ptr_ptr);
        let enc_ref = ref_from_ptr(enc_ptr);
        let (tok_value, next_tok_value) =
            enc_ref.scanners[state as usize](enc_ref, &input[ptr..end]);
        *next_tok_ptr = next_tok_value;
        tok_value
    };
    let tok: c_int = 'iife_ret_61: {
        let ptr = 0usize;
        let end = input.len();
        if ptr >= end {
            break 'iife_ret_61 XML_TOK_NONE_1;
        }
        let encPtr = enc.encPtr;
        let init_enc = enc.initEnc.isUtf16 as c_int;
        if ptr.wrapping_offset(1) == end {
            if let 3..=5 = init_enc {
                break 'iife_ret_61 XML_TOK_PARTIAL_1;
            }
            match input[ptr] as c_uchar as c_int {
                254 | 255 | 239 => {
                    if !(init_enc == ISO_8859_1_ENC && state == XML_CONTENT_STATE) {
                        break 'iife_ret_61 XML_TOK_PARTIAL_1;
                    }
                }
                0 | 60 => {
                    break 'iife_ret_61 XML_TOK_PARTIAL_1;
                }
                _ => {}
            }
        } else {
            let byte_pair = ((input[ptr.wrapping_offset(0)] as c_uchar as c_int) << 8)
                | input[ptr.wrapping_offset(1)] as c_uchar as c_int;
            match byte_pair {
                65279 => {
                    if !(init_enc == ISO_8859_1_ENC && state == XML_CONTENT_STATE) {
                        *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                        write_ptr(encPtr, read_ptr_at(encodingTable, UTF_16BE_ENC as isize));
                        break 'iife_ret_61 XML_TOK_BOM_1;
                    }
                }
                15360 => {
                    if !((init_enc == UTF_16BE_ENC || init_enc == UTF_16_ENC)
                        && state == XML_CONTENT_STATE)
                    {
                        write_ptr(encPtr, read_ptr_at(encodingTable, UTF_16LE_ENC as isize));
                        break 'iife_ret_61 scan_current(nextTokPtr, encPtr, state, ptr, end);
                    }
                }
                65534 => {
                    if !(init_enc == ISO_8859_1_ENC && state == XML_CONTENT_STATE) {
                        *nextTokPtr = ptr_to(ptr.wrapping_offset(2));
                        write_ptr(encPtr, read_ptr_at(encodingTable, UTF_16LE_ENC as isize));
                        break 'iife_ret_61 XML_TOK_BOM_1;
                    }
                }
                61371 => {
                    let in_content_with_fixed_enc = state == XML_CONTENT_STATE
                        && [ISO_8859_1_ENC, UTF_16BE_ENC, UTF_16LE_ENC, UTF_16_ENC]
                            .contains(&init_enc);
                    if !in_content_with_fixed_enc {
                        if ptr.wrapping_offset(2) == end {
                            break 'iife_ret_61 XML_TOK_PARTIAL_1;
                        }
                        if input[ptr.wrapping_offset(2)] as c_uchar as c_int == 0xbf {
                            *nextTokPtr = ptr_to(ptr.wrapping_offset(3));
                            write_ptr(encPtr, read_ptr_at(encodingTable, UTF_8_ENC as isize));
                            break 'iife_ret_61 XML_TOK_BOM_1;
                        }
                    }
                }
                _ => {
                    if input[ptr.wrapping_offset(0)] as c_int == '\0' as i32 {
                        if !(state == XML_CONTENT_STATE && init_enc == UTF_16LE_ENC) {
                            write_ptr(encPtr, read_ptr_at(encodingTable, UTF_16BE_ENC as isize));
                            break 'iife_ret_61 scan_current(nextTokPtr, encPtr, state, ptr, end);
                        }
                    } else if input[ptr.wrapping_offset(1)] as c_int == '\0' as i32
                        && state != XML_CONTENT_STATE
                    {
                        write_ptr(encPtr, read_ptr_at(encodingTable, UTF_16LE_ENC as isize));
                        break 'iife_ret_61 scan_current(nextTokPtr, encPtr, state, ptr, end);
                    }
                }
            }
        }
        write_ptr(encPtr, read_ptr_at(encodingTable, init_enc as isize));
        break 'iife_ret_61 scan_current(nextTokPtr, encPtr, state, ptr, end);
    };
    (tok, next_tok)
}
pub(crate) fn XmlInitUnknownEncodingNS(
    mut mem: *mut c_void,
    mut table: *const c_int,
    mut convert: CONVERTER,
    mut userData: *mut c_void,
) -> *mut ENCODING {
    let enc: *mut ENCODING = XmlInitUnknownEncoding(mem, table, convert, userData);
    if !enc.is_null() {
        let normal = as_normal_encoding_mut(enc);
        normal.type_0[ASCII_COLON as usize] = BT_COLON_0 as c_uchar;
    }
    enc
}
