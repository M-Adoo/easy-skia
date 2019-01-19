use bindings::*;

pub use bindings::sk_blurstyle_t as BlurStyle;

pub struct MaskFilter {
    pub(crate) native_pointer: *mut sk_maskfilter_t,
}

impl MaskFilter {
    pub fn new_blur(&mut self, blurstyle: BlurStyle, sigma: f32) -> MaskFilter {
        let native_pointer = unsafe { sk_maskfilter_new_blur(blurstyle, sigma) };
        if native_pointer.is_null() {
            panic!("Unable to create mask filter");
        }

        MaskFilter { native_pointer }
    }
}

impl Drop for MaskFilter {
    fn drop(&mut self) {
        unsafe { sk_maskfilter_unref(self.native_pointer) };
    }
}
