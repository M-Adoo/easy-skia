use std::marker::PhantomData;
use std::ptr;

use bindings::*;
use PixelGeometry;
use {Canvas, Image};

pub struct Surface {
    pub(crate) native_pointer: *mut sk_surface_t,
}

impl Surface {
    pub fn new_raster(
        image_info: &sk_imageinfo_t,
        rowBytes: usize,
        surface_props: &Option<SurfaceProps>,
    ) -> Surface {
        let surface_props = if let Some(ref val) = *surface_props {
            val
        } else {
            ptr::null()
        };

        let native_pointer =
            unsafe { sk_surface_new_raster(image_info, rowBytes, (*surface_props).raw_pointer) };
        if native_pointer.is_null() {
            panic!("Cannot create surface");
        }

        Surface { native_pointer }
    }

    pub fn get_canvas<'a>(&'a self) -> Canvas<'a> {
        let canvas_ptr = unsafe { sk_surface_get_canvas(self.native_pointer) };
        Canvas {
            native_pointer: canvas_ptr,
            phantom: PhantomData,
        }
    }

    pub fn new_image_snapshot(&self) -> Image {
        Image::new_from_pointer(unsafe { sk_surface_new_image_snapshot(self.native_pointer) })
            .unwrap()
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe { sk_surface_unref(self.native_pointer) };
    }
}

pub struct SurfaceProps {
    pub(crate) raw_pointer: *mut sk_surfaceprops_t,
}

impl SurfaceProps {
    pub fn new(flags: u32, geometry: PixelGeometry) -> SurfaceProps {
        SurfaceProps {
            raw_pointer: unsafe { sk_surfaceprops_new(flags, geometry) },
        }
    }

    pub fn flags(&self) -> u32 {
        unsafe { sk_surfaceprops_get_flags(self.raw_pointer) }
    }

    pub fn pixel_geometry(&self) -> PixelGeometry {
        unsafe { sk_surfaceprops_get_pixel_geometry(self.raw_pointer) }
    }
}

impl Drop for SurfaceProps {
    fn drop(&mut self) {
        unsafe { sk_surfaceprops_delete(self.raw_pointer) };
    }
}
