use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::ptr::NonNull;

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

macro_rules! assert_failed {
    ($assertion:expr, $line:expr, $function:expr) => {{
        unsafe {
            __assert_fail(
                $assertion.as_ptr() as *const ::core::ffi::c_char,
                FILE_PATH.as_ptr() as *const ::core::ffi::c_char,
                $line,
                $function.as_ptr() as *const ::core::ffi::c_char,
            )
        }
    }};
}

macro_rules! call_fail {
    ($line:expr, $msg:expr) => {{
        unsafe {
            _fail(
                FILE_PATH.as_ptr() as *const ::core::ffi::c_char,
                $line,
                $msg.as_ptr(),
            )
        }
    }};
}

macro_rules! clone_c_string_arg {
    ($ptr:expr, $assertion:expr, $line:expr, $function:expr $(,)?) => {{
        let ptr = require_non_null($ptr, $assertion, $line, $function);
        unsafe { CStr::from_ptr(ptr.as_ptr()) }.to_owned()
    }};
}

macro_rules! expected_slice {
    ($expected:expr, $count:expr $(,)?) => {{
        let expected = require_non_null(
            $expected,
            b"expected != NULL\0",
            113 as ::core::ffi::c_uint,
            FN_CHECK_ITEMS,
        );
        unsafe {
            ::core::slice::from_raw_parts(
                expected.as_ptr(),
                usize::try_from($count).expect("expected count should be non-negative"),
            )
        }
    }};
}

fn fail_with_static(line: ::core::ffi::c_int, msg: &'static [u8]) -> ! {
    let msg =
        CStr::from_bytes_with_nul(msg).expect("static failure messages must be NUL-terminated");
    call_fail!(line, msg)
}

fn fail_with_string(line: ::core::ffi::c_int, msg: String) -> ! {
    let msg = CString::new(msg).expect("formatted failure messages must not contain NUL");
    call_fail!(line, msg.as_c_str())
}

fn require_non_null_mut<T>(
    ptr: *mut T,
    assertion: &'static [u8],
    line: ::core::ffi::c_uint,
    function: &'static [u8],
) -> NonNull<T> {
    NonNull::new(ptr).unwrap_or_else(|| assert_failed!(assertion, line, function))
}

fn require_non_null<T>(
    ptr: *const T,
    assertion: &'static [u8],
    line: ::core::ffi::c_uint,
    function: &'static [u8],
) -> NonNull<T> {
    NonNull::new(ptr.cast_mut()).unwrap_or_else(|| assert_failed!(assertion, line, function))
}

#[derive(Copy, Clone)]
struct StorageHandle(NonNull<StructData>);

impl StorageHandle {
    fn new(storage: *mut StructData, line: ::core::ffi::c_uint, function: &'static [u8]) -> Self {
        Self(require_non_null_mut(
            storage,
            b"storage != NULL\0",
            line,
            function,
        ))
    }

    fn key(self) -> NonNull<StructData> {
        self.0
    }

    fn count(self) -> ::core::ffi::c_int {
        unsafe { self.0.as_ref() }.count
    }

    fn max_count(self) -> ::core::ffi::c_int {
        unsafe { self.0.as_ref() }.max_count
    }

    fn reset(self) {
        unsafe {
            let storage = self.0.as_ptr();
            (*storage).count = 0 as ::core::ffi::c_int;
            (*storage).entries = ::core::ptr::null_mut::<StructDataEntry>();
        }
    }

    fn reset_for_init(self) {
        self.reset();
        unsafe {
            (*self.0.as_ptr()).max_count = 0 as ::core::ffi::c_int;
        }
    }

    fn sync(
        self,
        count: ::core::ffi::c_int,
        max_count: ::core::ffi::c_int,
        entries: *mut StructDataEntry,
    ) {
        unsafe {
            let storage = &mut *self.0.as_ptr();
            storage.count = count;
            storage.max_count = max_count;
            storage.entries = entries;
        }
    }
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
        text: CString,
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
            text,
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

    fn sync_storage(&mut self, storage: StorageHandle) {
        self.refresh_c_entries();
        let entries = if self.c_entries.is_empty() {
            ::core::ptr::null_mut::<StructDataEntry>()
        } else {
            self.c_entries.as_mut_ptr()
        };
        storage.sync(self.count(), self.max_count, entries);
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
    storage: NonNull<StructData>,
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

fn registry_index(entries: &[RegistryEntry], storage: NonNull<StructData>) -> Option<usize> {
    entries.iter().position(|entry| entry.storage == storage)
}

fn get_or_insert_model<'a>(
    entries: &'a mut Vec<RegistryEntry>,
    storage: StorageHandle,
) -> &'a mut StructDataModel {
    let storage_key = storage.key();
    if let Some(index) = registry_index(entries, storage_key) {
        return &mut entries[index].model;
    }

    entries.push(RegistryEntry {
        storage: storage_key,
        model: StructDataModel::with_max_count(storage.max_count()),
    });
    &mut entries
        .last_mut()
        .expect("inserted registry entry should exist")
        .model
}

fn remove_model(entries: &mut Vec<RegistryEntry>, storage: StorageHandle) {
    let storage_key = storage.key();
    if let Some(index) = registry_index(entries, storage_key) {
        entries.remove(index);
    }
}

fn clear_storage(entries: &mut Vec<RegistryEntry>, storage: StorageHandle) {
    remove_model(entries, storage);
    storage.reset();
}

#[no_mangle]
pub unsafe extern "C" fn StructData_Init(storage: *mut StructData) {
    let storage = StorageHandle::new(storage, 73 as ::core::ffi::c_uint, FN_INIT);

    with_registry(|registry| remove_model(registry, storage));
    storage.reset_for_init();
    with_registry(|registry| {
        registry.push(RegistryEntry {
            storage: storage.key(),
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
    let storage = StorageHandle::new(storage, 84 as ::core::ffi::c_uint, FN_ADD_ITEM);
    let s = clone_c_string_arg!(s, b"s != NULL\0", 85 as ::core::ffi::c_uint, FN_ADD_ITEM);

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
    let storage = StorageHandle::new(storage, 112 as ::core::ffi::c_uint, FN_CHECK_ITEMS);

    if count != storage.count() {
        let actual_count = storage.count();

        with_registry(|registry| clear_storage(registry, storage));

        fail_with_string(
            119,
            format!(
                "wrong number of entries: got {}, expected {}",
                actual_count, count
            ),
        )
    }

    let expected = expected_slice!(expected, count);

    let expected_entries: Vec<ExpectedEntry> = expected
        .iter()
        .map(|entry| ExpectedEntry {
            text: clone_c_string_arg!(
                entry.str,
                b"expected[i].str != NULL\0",
                130 as ::core::ffi::c_uint,
                FN_CHECK_ITEMS,
            ),
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
            fail_with_static(line, msg)
        }
        Err(StructDataError::Formatted { line, msg }) => {
            with_registry(|registry| clear_storage(registry, storage));
            fail_with_string(line, msg)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn StructData_Dispose(storage: *mut StructData) {
    let storage = StorageHandle::new(storage, 151 as ::core::ffi::c_uint, FN_DISPOSE);
    with_registry(|registry| clear_storage(registry, storage));
}
