use bindings::*;

pub struct ColorSpace {
    pub(crate) raw_pointer: *mut sk_colorspace_t,
}

impl ColorSpace {
    pub fn new() -> ColorSpace {
        ColorSpace {
            raw_pointer: unsafe { sk_colorspace_new_srgb() },
        }
    }
}

impl Drop for ColorSpace {
    fn drop(&mut self) {
        unsafe { sk_colorspace_unref(self.raw_pointer) };
    }
}
