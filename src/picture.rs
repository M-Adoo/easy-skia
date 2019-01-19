use bindings::*;

pub struct Picture {
  pub(crate) native_pointer: *mut sk_picture_t,
}
