use std::cell::RefCell;
use std::ffi::{CStr, CString};

extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn _fail(
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        msg: *const ::core::ffi::c_char,
    ) -> !;
}
pub type XML_Char = ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StructDataEntry {
    pub str: *const XML_Char,
    pub data0: ::core::ffi::c_int,
    pub data1: ::core::ffi::c_int,
    pub data2: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StructData {
    pub count: ::core::ffi::c_int,
    pub max_count: ::core::ffi::c_int,
    pub entries: *mut StructDataEntry,
}
pub const STRUCT_EXTENSION_COUNT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

const FILE_PATH: &[u8] = b"/root/work/expat/tests/structdata.c\0";
const FN_INIT: &[u8] = b"void StructData_Init(StructData *)\0";
const FN_ADD_ITEM: &[u8] =
    b"void StructData_AddItem(StructData *, const XML_Char *, int, int, int)\0";
const FN_CHECK_ITEMS: &[u8] =
    b"void StructData_CheckItems(StructData *, const StructDataEntry *, int)\0";
const FN_DISPOSE: &[u8] = b"void StructData_Dispose(StructData *)\0";

enum StructDataError {
    Static {
        line: ::core::ffi::c_int,
        msg: &'static [u8],
    },
    Formatted {
        line: ::core::ffi::c_int,
        msg: String,
    },
}

impl StructDataError {
    fn static_msg(line: ::core::ffi::c_int, msg: &'static [u8]) -> Self {
        Self::Static { line, msg }
    }

    fn formatted(line: ::core::ffi::c_int, msg: String) -> Self {
        Self::Formatted { line, msg }
    }
}

struct OwnedEntry {
    text: CString,
    data0: ::core::ffi::c_int,
    data1: ::core::ffi::c_int,
    data2: ::core::ffi::c_int,
}

struct ExpectedEntry {
    text: CString,
    data0: ::core::ffi::c_int,
    data1: ::core::ffi::c_int,
    data2: ::core::ffi::c_int,
}

struct StructDataModel {
    entries: Vec<OwnedEntry>,
    c_entries: Box<[StructDataEntry]>,
    max_count: ::core::ffi::c_int,
}

impl StructDataModel {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
            c_entries: Vec::new().into_boxed_slice(),
            max_count: 0 as ::core::ffi::c_int,
        }
    }

    fn with_max_count(max_count: ::core::ffi::c_int) -> Self {
        let mut model = Self::new();
        model.max_count = max_count.max(0 as ::core::ffi::c_int);
        model
            .entries
            .reserve(usize::try_from(model.max_count).expect("max_count should fit into usize"));
        model
    }

    fn count(&self) -> ::core::ffi::c_int {
        ::core::ffi::c_int::try_from(self.entries.len()).expect("entry count should fit into c_int")
    }

    fn add_item(
        &mut self,
        text: &CStr,
        data0: ::core::ffi::c_int,
        data1: ::core::ffi::c_int,
        data2: ::core::ffi::c_int,
    ) {
        if self.count() == self.max_count {
            self.max_count += STRUCT_EXTENSION_COUNT;
            self.entries.reserve(
                usize::try_from(STRUCT_EXTENSION_COUNT)
                    .expect("extension count should fit into usize"),
            );
        }

        self.entries.push(OwnedEntry {
            text: text.to_owned(),
            data0,
            data1,
            data2,
        });
    }

    fn refresh_c_entries(&mut self) {
        self.c_entries = self
            .entries
            .iter()
            .map(|entry| StructDataEntry {
                str: entry.text.as_ptr(),
                data0: entry.data0,
                data1: entry.data1,
                data2: entry.data2,
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
    }

    fn sync_storage(&mut self, storage: &mut StructData) {
        self.refresh_c_entries();
        storage.count = self.count();
        storage.max_count = self.max_count;
        storage.entries = if self.c_entries.is_empty() {
            ::core::ptr::null_mut::<StructDataEntry>()
        } else {
            self.c_entries.as_mut_ptr()
        };
    }

    fn check_items(&self, expected: &[ExpectedEntry]) -> Result<(), StructDataError> {
        let expected_count = ::core::ffi::c_int::try_from(expected.len())
            .expect("expected length should fit into c_int");
        let actual_count = self.count();
        if expected_count != actual_count {
            return Err(StructDataError::formatted(
                119 as ::core::ffi::c_int,
                format!(
                    "wrong number of entries: got {}, expected {}",
                    actual_count, expected_count
                ),
            ));
        }

        for (got, want) in self.entries.iter().zip(expected.iter()) {
            if got.text.as_c_str() != want.text.as_c_str() {
                return Err(StructDataError::static_msg(
                    130 as ::core::ffi::c_int,
                    b"structure got bad string\0",
                ));
            }

            if got.data0 != want.data0 || got.data1 != want.data1 || got.data2 != want.data2 {
                return Err(StructDataError::formatted(
                    140 as ::core::ffi::c_int,
                    format!(
                        "struct '{}' expected ({},{},{}), got ({},{},{})",
                        got.text.to_string_lossy(),
                        want.data0,
                        want.data1,
                        want.data2,
                        got.data0,
                        got.data1,
                        got.data2,
                    ),
                ));
            }
        }

        Ok(())
    }
}

struct RegistryEntry {
    storage: usize,
    model: StructDataModel,
}

thread_local! {
    static REGISTRY: RefCell<Vec<RegistryEntry>> = const { RefCell::new(Vec::new()) };
}

fn with_registry<R>(f: impl FnOnce(&mut Vec<RegistryEntry>) -> R) -> R {
    REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        f(&mut registry)
    })
}

fn registry_index(entries: &[RegistryEntry], storage: usize) -> Option<usize> {
    entries.iter().position(|entry| entry.storage == storage)
}

fn get_or_insert_model<'a>(
    entries: &'a mut Vec<RegistryEntry>,
    storage: &StructData,
) -> &'a mut StructDataModel {
    let storage_key = storage as *const StructData as usize;
    if let Some(index) = registry_index(entries, storage_key) {
        return &mut entries[index].model;
    }

    entries.push(RegistryEntry {
        storage: storage_key,
        model: StructDataModel::with_max_count(storage.max_count),
    });
    &mut entries
        .last_mut()
        .expect("inserted registry entry should exist")
        .model
}

fn remove_model(entries: &mut Vec<RegistryEntry>, storage: &StructData) {
    let storage_key = storage as *const StructData as usize;
    if let Some(index) = registry_index(entries, storage_key) {
        entries.remove(index);
    }
}

fn reset_storage(storage: &mut StructData) {
    storage.count = 0 as ::core::ffi::c_int;
    storage.entries = ::core::ptr::null_mut::<StructDataEntry>();
}

fn reset_storage_for_init(storage: &mut StructData) {
    reset_storage(storage);
    storage.max_count = 0 as ::core::ffi::c_int;
}

fn clear_storage(entries: &mut Vec<RegistryEntry>, storage: &mut StructData) {
    remove_model(entries, storage);
    reset_storage(storage);
}

#[no_mangle]
pub unsafe extern "C" fn StructData_Init(storage: *mut StructData) {
    let storage = if storage.is_null() {
        unsafe {
            __assert_fail(
                b"storage != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                FILE_PATH.as_ptr() as *const ::core::ffi::c_char,
                73 as ::core::ffi::c_uint,
                FN_INIT.as_ptr() as *const ::core::ffi::c_char,
            )
        }
    } else {
        unsafe { &mut *storage }
    };

    with_registry(|registry| remove_model(registry, storage));
    reset_storage_for_init(storage);
    with_registry(|registry| {
        registry.push(RegistryEntry {
            storage: storage as *mut StructData as usize,
            model: StructDataModel::new(),
        });
    });
}

#[no_mangle]
pub unsafe extern "C" fn StructData_AddItem(
    storage: *mut StructData,
    s: *const XML_Char,
    data0: ::core::ffi::c_int,
    data1: ::core::ffi::c_int,
    data2: ::core::ffi::c_int,
) {
    let storage = if storage.is_null() {
        unsafe {
            __assert_fail(
                b"storage != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                FILE_PATH.as_ptr() as *const ::core::ffi::c_char,
                84 as ::core::ffi::c_uint,
                FN_ADD_ITEM.as_ptr() as *const ::core::ffi::c_char,
            )
        }
    } else {
        unsafe { &mut *storage }
    };
    let s = if s.is_null() {
        unsafe {
            __assert_fail(
                b"s != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                FILE_PATH.as_ptr() as *const ::core::ffi::c_char,
                85 as ::core::ffi::c_uint,
                FN_ADD_ITEM.as_ptr() as *const ::core::ffi::c_char,
            )
        }
    } else {
        unsafe { CStr::from_ptr(s) }
    };

    with_registry(|registry| {
        let model = get_or_insert_model(registry, storage);
        model.add_item(s, data0, data1, data2);
        model.sync_storage(storage);
    });
}

#[no_mangle]
pub unsafe extern "C" fn StructData_CheckItems(
    storage: *mut StructData,
    expected: *const StructDataEntry,
    count: ::core::ffi::c_int,
) {
    let storage = if storage.is_null() {
        unsafe {
            __assert_fail(
                b"storage != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                FILE_PATH.as_ptr() as *const ::core::ffi::c_char,
                112 as ::core::ffi::c_uint,
                FN_CHECK_ITEMS.as_ptr() as *const ::core::ffi::c_char,
            )
        }
    } else {
        unsafe { &mut *storage }
    };
    if expected.is_null() {
        unsafe {
            __assert_fail(
                b"expected != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                FILE_PATH.as_ptr() as *const ::core::ffi::c_char,
                113 as ::core::ffi::c_uint,
                FN_CHECK_ITEMS.as_ptr() as *const ::core::ffi::c_char,
            )
        }
    }

    if count != storage.count {
        let actual_count = storage.count;

        with_registry(|registry| clear_storage(registry, storage));

        let msg = CString::new(format!(
            "wrong number of entries: got {}, expected {}",
            actual_count, count
        ))
        .expect("formatted failure messages must not contain NUL");
        unsafe {
            _fail(
                FILE_PATH.as_ptr() as *const ::core::ffi::c_char,
                119,
                msg.as_ptr(),
            )
        }
    }

    let expected = unsafe {
        ::core::slice::from_raw_parts(
            expected,
            usize::try_from(count).expect("expected count should be non-negative"),
        )
    };

    let expected_entries: Vec<ExpectedEntry> = expected
        .iter()
        .map(|entry| ExpectedEntry {
            text: unsafe { CStr::from_ptr(entry.str) }.to_owned(),
            data0: entry.data0,
            data1: entry.data1,
            data2: entry.data2,
        })
        .collect();

    let result = {
        with_registry(|registry| {
            let model = get_or_insert_model(registry, storage);
            model.check_items(&expected_entries)
        })
    };

    match result {
        Ok(()) => {}
        Err(StructDataError::Static { line, msg }) => {
            with_registry(|registry| clear_storage(registry, storage));
            unsafe {
                _fail(
                    FILE_PATH.as_ptr() as *const ::core::ffi::c_char,
                    line,
                    msg.as_ptr() as *const ::core::ffi::c_char,
                )
            }
        }
        Err(StructDataError::Formatted { line, msg }) => {
            with_registry(|registry| clear_storage(registry, storage));
            let msg = CString::new(msg).expect("formatted failure messages must not contain NUL");
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

#[no_mangle]
pub unsafe extern "C" fn StructData_Dispose(storage: *mut StructData) {
    let storage = if storage.is_null() {
        unsafe {
            __assert_fail(
                b"storage != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                FILE_PATH.as_ptr() as *const ::core::ffi::c_char,
                151 as ::core::ffi::c_uint,
                FN_DISPOSE.as_ptr() as *const ::core::ffi::c_char,
            )
        }
    } else {
        unsafe { &mut *storage }
    };
    with_registry(|registry| clear_storage(registry, storage));
}
