use bindings::*;
use std::ffi::CStr;
use wrap_safe_type;

pub struct SKString {
  pub(crate) raw_pointer: *mut sk_string_t,
}

impl Drop for SKString {
  fn drop(&mut self) {
    unsafe { sk_string_destructor(self.raw_pointer) }
  }
}

impl SKString {
  pub fn new() -> Self {
    wrap_safe_type! {
      SKString <= sk_string_new_empty()
    }
  }

  pub fn new_with_copy(text: &str) -> Self {
    wrap_safe_type! {
      SKString <= sk_string_new_with_copy(text.as_ptr() as *const i8, text.len())
    }
  }

  pub fn size(&self) -> usize {
    unsafe { sk_string_get_size(self.raw_pointer) }
  }

  pub fn as_str(&self) -> &str {
    let c_str = unsafe { CStr::from_ptr(sk_string_get_c_str(self.raw_pointer)) };
    c_str.to_str().unwrap()
  }
}
