use std::ffi::{CStr, CString};

extern "C" {
    fn _fail(
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        msg: *const ::core::ffi::c_char,
    ) -> !;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
}
pub type XML_Char = ::core::ffi::c_char;

const FILE_PATH: &[u8] = b"/root/work/expat/tests/chardata.c\0";
const FN_XMLSTRLEN: &[u8] = b"int xmlstrlen(const XML_Char *)\0";
const FN_INIT: &[u8] = b"void CharData_Init(CharData *)\0";
const FN_APPEND: &[u8] = b"void CharData_AppendXMLChars(CharData *, const XML_Char *, int)\0";
const FN_CHECK: &[u8] = b"int CharData_CheckXMLChars(CharData *, const XML_Char *)\0";

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CharData {
    pub count: ::core::ffi::c_int,
    pub data: [XML_Char; 2048],
}

enum CharDataError {
    Static {
        line: ::core::ffi::c_int,
        msg: &'static [u8],
    },
    Formatted {
        line: ::core::ffi::c_int,
        msg: String,
    },
}

impl CharDataError {
    fn static_msg(line: ::core::ffi::c_int, msg: &'static [u8]) -> Self {
        Self::Static { line, msg }
    }

    fn formatted(line: ::core::ffi::c_int, msg: String) -> Self {
        Self::Formatted { line, msg }
    }
}

impl CharData {
    fn normalized_count(&self) -> ::core::ffi::c_int {
        self.count.max(0 as ::core::ffi::c_int)
    }

    fn append_xml_chars(&mut self, input: &[XML_Char]) {
        let maxchars = self.data.len() as ::core::ffi::c_int;
        if self.count < 0 as ::core::ffi::c_int {
            self.count = 0 as ::core::ffi::c_int;
        }

        let mut len =
            ::core::ffi::c_int::try_from(input.len()).expect("append length should fit into c_int");
        if len + self.count > maxchars {
            len = maxchars - self.count;
        }

        if len + self.count < maxchars {
            let start = self.count as usize;
            let len = usize::try_from(len).expect("append length should be non-negative");
            let end = start + len;
            self.data[start..end].copy_from_slice(&input[..len]);
            self.count += len as ::core::ffi::c_int;
        }
    }

    fn check_xml_chars(&self, expected: &[XML_Char]) -> Result<::core::ffi::c_int, CharDataError> {
        let len = ::core::ffi::c_int::try_from(expected.len())
            .expect("expected length should fit into c_int");
        let count = self.normalized_count();
        if len != count {
            return Err(CharDataError::formatted(
                98 as ::core::ffi::c_int,
                format!(
                    "wrong number of data characters: got {}, expected {}",
                    count, len
                ),
            ));
        }

        let len = len as usize;
        if expected != &self.data[..len] {
            return Err(CharDataError::static_msg(
                102 as ::core::ffi::c_int,
                b"got bad data bytes\0",
            ));
        }

        Ok(1 as ::core::ffi::c_int)
    }
}

#[no_mangle]
pub unsafe extern "C" fn CharData_Init(storage: *mut CharData) {
    if storage.is_null() {
        unsafe {
            __assert_fail(
                b"storage != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                FILE_PATH.as_ptr() as *const ::core::ffi::c_char,
                62 as ::core::ffi::c_uint,
                FN_INIT.as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }

    let storage = unsafe { &mut *storage };
    storage.count = -(1 as ::core::ffi::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn CharData_AppendXMLChars(
    storage: *mut CharData,
    s: *const XML_Char,
    len: ::core::ffi::c_int,
) {
    if storage.is_null() {
        unsafe {
            __assert_fail(
                b"storage != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                FILE_PATH.as_ptr() as *const ::core::ffi::c_char,
                70 as ::core::ffi::c_uint,
                FN_APPEND.as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    if s.is_null() {
        unsafe {
            __assert_fail(
                b"s != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                FILE_PATH.as_ptr() as *const ::core::ffi::c_char,
                71 as ::core::ffi::c_uint,
                FN_APPEND.as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }

    let storage = unsafe { &mut *storage };
    let len = if len < 0 as ::core::ffi::c_int {
        unsafe { CStr::from_ptr(s) }.to_bytes().len()
    } else {
        usize::try_from(len).expect("append length should be non-negative")
    };
    let input = unsafe { ::core::slice::from_raw_parts(s, len) };
    storage.append_xml_chars(input);
}

#[no_mangle]
pub unsafe extern "C" fn CharData_CheckXMLChars(
    storage: *mut CharData,
    expected: *const XML_Char,
) -> ::core::ffi::c_int {
    if storage.is_null() {
        unsafe {
            __assert_fail(
                b"storage != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                FILE_PATH.as_ptr() as *const ::core::ffi::c_char,
                91 as ::core::ffi::c_uint,
                FN_CHECK.as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    if expected.is_null() {
        unsafe {
            __assert_fail(
                b"s != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                FILE_PATH.as_ptr() as *const ::core::ffi::c_char,
                54 as ::core::ffi::c_uint,
                FN_XMLSTRLEN.as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }

    let storage = unsafe { &mut *storage };
    let expected_len = unsafe { CStr::from_ptr(expected) }.to_bytes().len();
    let expected = unsafe { ::core::slice::from_raw_parts(expected, expected_len) };

    match storage.check_xml_chars(expected) {
        Ok(result) => result,
        Err(CharDataError::Static { line, msg }) => unsafe {
            _fail(
                FILE_PATH.as_ptr() as *const ::core::ffi::c_char,
                line,
                msg.as_ptr() as *const ::core::ffi::c_char,
            )
        },
        Err(CharDataError::Formatted { line, msg }) => {
            let msg =
                CString::new(msg).expect("formatted failure messages must not contain NUL bytes");
            unsafe {
                _fail(
                    FILE_PATH.as_ptr() as *const ::core::ffi::c_char,
                    line,
                    msg.as_ptr(),
                )
            }
        }
    }
}
