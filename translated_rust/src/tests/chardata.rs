use std::ffi::{CStr, CString};
use std::slice;

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

fn assert_not_null<T>(
    ptr: *const T,
    assertion: &'static [u8],
    line: ::core::ffi::c_uint,
    function: &'static [u8],
) {
    if ptr.is_null() {
        unsafe {
            __assert_fail(
                assertion.as_ptr() as *const ::core::ffi::c_char,
                FILE_PATH.as_ptr() as *const ::core::ffi::c_char,
                line,
                function.as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}

fn fail_static(line: ::core::ffi::c_int, msg: &'static [u8]) -> ! {
    unsafe {
        _fail(
            FILE_PATH.as_ptr() as *const ::core::ffi::c_char,
            line,
            msg.as_ptr() as *const ::core::ffi::c_char,
        )
    }
}

fn fail_formatted(line: ::core::ffi::c_int, msg: String) -> ! {
    let msg = CString::new(msg).expect("formatted failure messages must not contain NUL bytes");
    unsafe {
        _fail(
            FILE_PATH.as_ptr() as *const ::core::ffi::c_char,
            line,
            msg.as_ptr(),
        )
    }
}

fn storage_mut<'a>(
    storage: *mut CharData,
    line: ::core::ffi::c_uint,
    function: &'static [u8],
) -> &'a mut CharData {
    assert_not_null(
        storage as *const CharData,
        b"storage != NULL\0",
        line,
        function,
    );
    unsafe { &mut *storage }
}

fn xml_c_str<'a>(
    s: *const XML_Char,
    line: ::core::ffi::c_uint,
    function: &'static [u8],
) -> &'a CStr {
    assert_not_null(s, b"s != NULL\0", line, function);
    unsafe { CStr::from_ptr(s) }
}

fn xml_slice<'a>(
    s: *const XML_Char,
    len: usize,
    line: ::core::ffi::c_uint,
    function: &'static [u8],
) -> &'a [XML_Char] {
    assert_not_null(s, b"s != NULL\0", line, function);
    unsafe { slice::from_raw_parts(s, len) }
}

fn xmlstrlen(s: *const XML_Char) -> ::core::ffi::c_int {
    xml_c_str(s, 54 as ::core::ffi::c_uint, FN_XMLSTRLEN)
        .to_bytes()
        .len() as ::core::ffi::c_int
}

impl CharData {
    fn normalized_count(&self) -> ::core::ffi::c_int {
        self.count.max(0 as ::core::ffi::c_int)
    }

    fn append_xml_chars(&mut self, s: *const XML_Char, mut len: ::core::ffi::c_int) {
        let maxchars = self.data.len() as ::core::ffi::c_int;
        if self.count < 0 as ::core::ffi::c_int {
            self.count = 0 as ::core::ffi::c_int;
        }
        if len < 0 as ::core::ffi::c_int {
            len = xmlstrlen(s);
        }
        if len + self.count > maxchars {
            len = maxchars - self.count;
        }
        if len + self.count < self.data.len() as ::core::ffi::c_int {
            let start = self.count as usize;
            let len = usize::try_from(len).expect("append length should be non-negative");
            let end = start + len;
            self.data[start..end].copy_from_slice(xml_slice(
                s,
                len,
                71 as ::core::ffi::c_uint,
                FN_APPEND,
            ));
            self.count += len as ::core::ffi::c_int;
        }
    }

    fn check_xml_chars(&self, expected: *const XML_Char) -> ::core::ffi::c_int {
        let len = xmlstrlen(expected);
        let count = self.normalized_count();
        if len != count {
            fail_formatted(
                98 as ::core::ffi::c_int,
                format!(
                    "wrong number of data characters: got {}, expected {}",
                    count, len
                ),
            );
        }

        let len = len as usize;
        if xml_slice(expected, len, 91 as ::core::ffi::c_uint, FN_CHECK) != &self.data[..len] {
            fail_static(102 as ::core::ffi::c_int, b"got bad data bytes\0");
        }
        1 as ::core::ffi::c_int
    }
}

#[no_mangle]
pub unsafe extern "C" fn CharData_Init(mut storage: *mut CharData) {
    storage_mut(storage, 62 as ::core::ffi::c_uint, FN_INIT).count = -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn CharData_AppendXMLChars(
    mut storage: *mut CharData,
    mut s: *const XML_Char,
    mut len: ::core::ffi::c_int,
) {
    assert_not_null(s, b"s != NULL\0", 71 as ::core::ffi::c_uint, FN_APPEND);
    storage_mut(storage, 70 as ::core::ffi::c_uint, FN_APPEND).append_xml_chars(s, len);
}
#[no_mangle]
pub unsafe extern "C" fn CharData_CheckXMLChars(
    mut storage: *mut CharData,
    mut expected: *const XML_Char,
) -> ::core::ffi::c_int {
    storage_mut(storage, 91 as ::core::ffi::c_uint, FN_CHECK).check_xml_chars(expected)
}
