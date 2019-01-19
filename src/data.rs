use std::mem::forget;
use std::os::raw::c_void;
use std::slice;

use bindings::*;

pub struct Data {
    pub(crate) native_pointer: *mut sk_data_t,
}

impl Data {
    fn new_from_pointer(native_pointer: *mut sk_data_t) -> Data {
        if native_pointer.is_null() {
            panic!("Cannot create empty data");
        }

        Data { native_pointer }
    }

    pub fn new_empty() -> Data {
        let native_pointer = unsafe { sk_data_new_empty() };
        Data::new_from_pointer(native_pointer)
    }

    pub fn new_with_copy(bytes: &[c_void]) -> Data {
        let native_pointer = unsafe { sk_data_new_with_copy(bytes.as_ptr(), bytes.len()) };
        Data::new_from_pointer(native_pointer)
    }

    pub fn new_from_malloc(bytes: Vec<c_void>) -> Data {
        forget(&bytes);
        let native_pointer = unsafe { sk_data_new_with_copy(bytes.as_ptr(), bytes.len()) };
        Data::new_from_pointer(native_pointer)
    }

    pub fn new_from_subset(data: &Data, offset: usize, length: usize) -> Data {
        let native_pointer = unsafe { sk_data_new_subset(data.native_pointer, offset, length) };
        Data::new_from_pointer(native_pointer)
    }

    pub fn size(&self) -> usize {
        unsafe { sk_data_get_size(self.native_pointer) }
    }

    pub fn get_data(&self) -> &[u8] {
        let slice = unsafe { sk_data_get_data(self.native_pointer) } as *const u8;
        unsafe { slice::from_raw_parts(slice, self.size()) }
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        unsafe { sk_data_unref(self.native_pointer) };
    }
}
