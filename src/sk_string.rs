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
  fn new() -> Self {
    wrap_safe_type! {
      SKString <= sk_string_new_empty()
    }
  }

  fn new_with_cop(text: &str) -> Self {
    let mut c_str = std::ffi::CString::new(text).unwrap();
    wrap_safe_type! {
      SKString <= sk_string_new_with_copy(c_str.as_ptr(), text.len())
    }
  }

  fn size(&self) -> usize {
    unsafe { sk_string_get_size(self.raw_pointer) }
  }

  fn as_str(&self) -> &str {
    let c_str = unsafe { CStr::from_ptr(sk_string_get_c_str(self.raw_pointer)) };
    c_str.to_str().unwrap()
  }
}
