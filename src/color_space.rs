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

    pub fn unref(&mut self) {
        unsafe {
            sk_colorspace_unref(self.raw_pointer);
        }
    }

    pub fn raw_pointer(&mut self) -> *mut sk_colorspace_t {
        self.raw_pointer
    }
}

impl Drop for ColorSpace {
    fn drop(&mut self) {
        unsafe { sk_colorspace_unref(self.raw_pointer) };
    }
}
