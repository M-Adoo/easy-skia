use std::marker::PhantomData;
use std::ptr;

pub use bindings::gr_surfaceorigin_t as SurfaceOrigin;
pub use bindings::sk_pixelgeometry_t as PixelGeometry;
use bindings::*;
use context::*;
use {Canvas, ColorSpace, GrContext, Image, ImageInfo, Paint};

pub use bindings::sk_colortype_t as ColorType;

pub fn get_default_8888() -> ColorType {
    unsafe { sk_colortype_get_default_8888() }
}
pub struct Surface {
    pub(crate) raw_pointer: *mut sk_surface_t,
}

impl Surface {
    pub fn new_null(width: i32, height: i32) -> Surface {
        Surface {
            raw_pointer: unsafe { sk_surface_new_null(width, height) },
        }
    }

    pub fn new_raster(
        image_info: &ImageInfo,
        rowBytes: usize,
        surface_props: &Option<SurfaceProps>,
    ) -> Surface {
        let surface_props = if let Some(ref val) = *surface_props {
            val
        } else {
            ptr::null()
        };

        let raw_pointer =
            unsafe { sk_surface_new_raster(image_info, rowBytes, (*surface_props).raw_pointer) };
        if raw_pointer.is_null() {
            panic!("Cannot create surface");
        }

        Surface { raw_pointer }
    }

    pub fn new_render_target(
        context: &GrContext,
        budgeted: bool,
        image_info: &ImageInfo,
        sample_count: i32,
        origin: SurfaceOrigin,
        props: &SurfaceProps,
        create_mips: bool,
    ) -> Surface {
        Surface {
            raw_pointer: unsafe {
                sk_surface_new_render_target(
                    context.raw_pointer,
                    budgeted,
                    image_info,
                    sample_count,
                    origin,
                    props.raw_pointer,
                    create_mips,
                )
            },
        }
    }

    pub fn new_backend_render_target(
        context: &GrContext,
        target: &GrBackendRenderTarget,
        origin: SurfaceOrigin,
        color_type: ColorType,
        color_space: Option<ColorSpace>,
        props: &SurfaceProps,
    ) -> Surface {
        Surface {
            raw_pointer: unsafe {
                sk_surface_new_backend_render_target(
                    context.raw_pointer,
                    target.raw_pointer,
                    origin,
                    color_type,
                    if let Some(cs) = color_space {
                        cs.raw_pointer
                    } else {
                        std::ptr::null_mut()
                    },
                    props.raw_pointer,
                )
            },
        }
    }
    pub fn get_canvas<'a>(&'a self) -> Canvas<'a> {
        let canvas_ptr = unsafe { sk_surface_get_canvas(self.raw_pointer) };
        Canvas {
            raw_pointer: canvas_ptr,
            phantom: PhantomData,
        }
    }

    pub fn new_image_snapshot(&self) -> Image {
        Image::new_from_pointer(unsafe { sk_surface_new_image_snapshot(self.raw_pointer) }).unwrap()
    }

    pub fn draw(&self, canvas: &Canvas, x: f32, y: f32, paint: &Paint) {
        unsafe {
            sk_surface_draw(
                self.raw_pointer,
                canvas.raw_pointer,
                x,
                y,
                paint.raw_pointer,
            )
        };
    }

    // pub fn get_props(&mut self) -> SurfaceProps {
    //     unsafe {
    //         let raw_pointer = sk_surface_get_props(self.raw_pointer);
    //         SurfaceProps::new(
    //             sk_surfaceprops_get_flags(raw_pointer),
    //             sk_surfaceprops_get_pixel_geometry(raw_pointer),
    //         )
    //     }
    // }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe { sk_surface_unref(self.raw_pointer) };
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
