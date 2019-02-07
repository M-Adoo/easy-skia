use std::os::raw::c_void;

use bindings::*;
use Data;

pub use bindings::sk_irect_t as IRect;
pub use bindings::sk_imageinfo_t as ImageInfo;

pub struct Image {
    pub(crate) raw_pointer: *mut sk_image_t,
}

impl Image {
    pub(crate) fn new_from_pointer(raw_pointer: *mut sk_image_t) -> Option<Image> {
        if raw_pointer.is_null() {
            None
        } else {
            Some(Image { raw_pointer })
        }
    }

    pub fn new_raster_copy(image_info: &sk_imageinfo_t, pixels: &[c_void]) -> Option<Image> {
        unsafe {
            Image::new_from_pointer(sk_image_new_raster_copy(
                image_info,
                pixels.as_ptr(),
                pixels.len(),
            ))
        }
    }

    pub fn new_from_encoded(data: &Data, irect: &IRect) -> Option<Image> {
        unsafe { Image::new_from_pointer(sk_image_new_from_encoded(data.raw_pointer, irect)) }
    }

    pub fn encode(&self) -> Data {
        Data {
            raw_pointer: unsafe { sk_image_encode(self.raw_pointer) },
        }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { sk_image_get_width(self.raw_pointer) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { sk_image_get_height(self.raw_pointer) }
    }

    pub fn get_unique_id(&self) -> u32 {
        unsafe { sk_image_get_unique_id(self.raw_pointer) }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { sk_image_unref(self.raw_pointer) };
    }
}
