use std::ffi::CString;
use std::os::raw::c_void;

use bindings::*;
use color::Color;
use {MaskFilter, Rect, Shader, Typeface};

pub use bindings::sk_blendmode_t as BlendMode;
pub use bindings::sk_fontmetrics_t as FontMetrics;
pub use bindings::sk_fontmetrics_t;
pub use bindings::sk_stroke_cap_t as StrokeCap;
pub use bindings::sk_stroke_join_t as StrokeJoin;

use TextEncoding;

pub struct Paint {
    pub(crate) raw_pointer: *mut sk_paint_t,
}

impl Paint {
    pub fn new() -> Paint {
        let pointer = unsafe { sk_paint_new() };

        if pointer.is_null() {
            panic!("Cannot create paint object");
        }

        Paint {
            raw_pointer: pointer,
        }
    }

    pub fn is_antialias(&self) -> bool {
        unsafe { sk_paint_is_antialias(self.raw_pointer) }
    }

    pub fn set_antialias(&mut self, antialias: bool) -> &mut Self {
        unsafe { sk_paint_set_antialias(self.raw_pointer, antialias) };
        self
    }

    pub fn set_dither(&mut self, dither: bool) -> &mut Self {
        unsafe { sk_paint_set_dither(self.raw_pointer, dither) };
        self
    }

    pub fn is_dither(&self) -> bool {
        unsafe { sk_paint_is_dither(self.raw_pointer) }
    }

    pub fn get_color(&self) -> Color {
        Color {
            0: unsafe { sk_paint_get_color(self.raw_pointer) },
        }
    }

    pub fn set_color(&mut self, color: Color) -> &mut Self {
        unsafe { sk_paint_set_color(self.raw_pointer, color.0) };
        self
    }

    pub fn get_stroke_width(&self) -> f32 {
        unsafe { sk_paint_get_stroke_width(self.raw_pointer) }
    }

    pub fn set_stroke_width(&mut self, width: f32) -> &mut Self {
        unsafe { sk_paint_set_stroke_width(self.raw_pointer, width) };
        self
    }

    pub fn get_stroke_miter(&self) -> f32 {
        unsafe { sk_paint_get_stroke_miter(self.raw_pointer) }
    }

    pub fn set_stroke_miter(&mut self, miter: f32) -> &mut Self {
        unsafe { sk_paint_set_stroke_miter(self.raw_pointer, miter) };
        self
    }

    pub fn get_stroke_cap(&self) -> StrokeCap {
        unsafe { sk_paint_get_stroke_cap(self.raw_pointer) }
    }

    pub fn set_stroke_cap(&mut self, cap: StrokeCap) -> &mut Self {
        unsafe { sk_paint_set_stroke_cap(self.raw_pointer, cap) };
        self
    }

    pub fn get_stroke_join(&self) -> StrokeJoin {
        unsafe { sk_paint_get_stroke_join(self.raw_pointer) }
    }

    pub fn set_stroke_join(&mut self, join: StrokeJoin) -> &mut Self {
        unsafe { sk_paint_set_stroke_join(self.raw_pointer, join) };
        self
    }

    pub fn set_text_size(&mut self, size: f32) -> &mut Self {
        unsafe { sk_paint_set_textsize(self.raw_pointer, size) };
        self
    }

    pub fn get_text_size(&self) -> f32 {
        unsafe { sk_paint_get_textsize(self.raw_pointer) }
    }

    pub fn set_typeface(&mut self, typeface: &Typeface) -> &mut Self {
        unsafe { sk_paint_set_typeface(self.raw_pointer, typeface.raw_pointer) };
        self
    }

    pub fn get_font_metrics(&self, scale: f32) -> FontMetrics {
        let mut font_metrics = FontMetrics {
            fFlags: 0,
            fTop: 0.0,
            fAscent: 0.0,
            fDescent: 0.0,
            fBottom: 0.0,
            fLeading: 0.0,
            fAvgCharWidth: 0.0,
            fMaxCharWidth: 0.0,
            fXMin: 0.0,
            fXMax: 0.0,
            fXHeight: 0.0,
            fCapHeight: 0.0,
            fUnderlineThickness: 0.0,
            fUnderlinePosition: 0.0,
            fStrikeoutThickness: 0.0,
            fStrikeoutPosition: 0.0,
        };

        unsafe { sk_paint_get_fontmetrics(self.raw_pointer, &mut font_metrics, scale) };

        font_metrics
    }

    pub fn measure_text(&self, text: &str) -> (f32, Rect) {
        let ctext = CString::new(text).unwrap();
        let mut rect = Rect {
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
        };

        let width = unsafe {
            sk_paint_measure_text(
                self.raw_pointer,
                ctext.as_ptr() as *const c_void,
                text.len(),
                &mut rect,
            )
        };

        (width, rect)
    }

    pub fn measure_blob(&self, blob: &[u16]) -> (f32, Rect) {
        let mut rect = Rect {
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
        };

        let width = unsafe {
            sk_paint_measure_text(
                self.raw_pointer,
                blob.as_ptr() as *const c_void,
                2 * blob.len(),
                &mut rect,
            )
        };

        (width, rect)
    }

    pub fn set_text_encoding(&mut self, encoding: TextEncoding) -> &mut Self {
        unsafe { sk_paint_set_text_encoding(self.raw_pointer, encoding) };
        self
    }

    pub fn get_text_encoding(&self) -> TextEncoding {
        unsafe { sk_paint_get_text_encoding(self.raw_pointer) }
    }

    pub fn set_shader(&mut self, shader: &Shader) -> &mut Self {
        unsafe { sk_paint_set_shader(self.raw_pointer, shader.raw_pointer) };
        self
    }

    pub fn set_maskfilter(&mut self, filter: &MaskFilter) -> &mut Self {
        unsafe { sk_paint_set_maskfilter(self.raw_pointer, filter.raw_pointer) };
        self
    }

    pub fn sk_paint_get_blendmode(&self) -> BlendMode {
        unsafe { sk_paint_get_blendmode(self.raw_pointer) }
    }

    pub fn sk_paint_set_blendmode(&mut self, mode: BlendMode) -> &mut Self {
        unsafe {
            sk_paint_set_blendmode(self.raw_pointer, mode);
        };
        self
    }
}

impl Drop for Paint {
    fn drop(&mut self) {
        unsafe { sk_paint_delete(self.raw_pointer) };
    }
}
