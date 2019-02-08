use bindings::*;

pub struct Picture {
    pub(crate) raw_pointer: *mut sk_picture_t,
}
