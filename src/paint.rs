use std::os::raw::c_void;

use bindings::*;
use color::Color;
use {
  wrap_safe_type, ColorFilter, ImageFilter, MaskFilter, Path, Point, Rect, Shader, TextBlob,
  Typeface,
};

pub use bindings::sk_blendmode_t as BlendMode;
pub use bindings::sk_filter_quality_t as FilterQuality;
pub use bindings::sk_fontmetrics_t as FontMetrics;
pub use bindings::sk_paint_hinting_t as PaintHinting;
pub use bindings::sk_paint_style_t as PaintStyle;
pub use bindings::sk_path_effect_t as PathEffect;
pub use bindings::sk_stroke_cap_t as StrokeCap;
pub use bindings::sk_stroke_join_t as StrokeJoin;
pub use bindings::sk_text_align_t as TextAlign;

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

  pub fn get_style(&self) -> PaintStyle {
    unsafe { sk_paint_get_style(self.raw_pointer) }
  }

  pub fn set_style(&mut self, style: PaintStyle) -> &mut Self {
    unsafe { sk_paint_set_style(self.raw_pointer, style) };
    self
  }

  pub fn is_verticaltext(&self) -> bool {
    unsafe { sk_paint_is_verticaltext(self.raw_pointer) }
  }

  pub fn set_verticaltext(&mut self, vt: bool) -> &mut Self {
    unsafe { sk_paint_set_verticaltext(self.raw_pointer, vt) };
    self
  }

  pub fn set_colorfilter(&mut self, color_filter: &ColorFilter) -> &mut Self {
    unsafe { sk_paint_set_colorfilter(self.raw_pointer, color_filter.raw_pointer) };
    self
  }

  pub fn get_colorfilter(&mut self) -> ColorFilter {
    ColorFilter {
      raw_pointer: unsafe { sk_paint_get_colorfilter(self.raw_pointer) },
    }
  }

  pub fn get_imagefilter(&mut self) -> ImageFilter {
    wrap_safe_type! {
      ImageFilter <= sk_paint_get_imagefilter(self.raw_pointer)
    }
  }

  pub fn set_imagefilter(&mut self, image_filter: &mut ImageFilter) -> &mut Self {
    unsafe { sk_paint_set_imagefilter(self.raw_pointer, image_filter.raw_pointer) };
    self
  }

  pub fn set_filter_quality(&mut self, fq: FilterQuality) -> &mut Self {
    unsafe { sk_paint_set_filter_quality(self.raw_pointer, fq) };
    self
  }

  pub fn get_filter_quality(&self) -> FilterQuality {
    unsafe { sk_paint_get_filter_quality(self.raw_pointer) }
  }

  pub fn get_text_align(&self) -> TextAlign {
    unsafe { sk_paint_get_text_align(self.raw_pointer) }
  }

  pub fn set_text_align(&mut self, align: TextAlign) -> &mut Self {
    unsafe { sk_paint_set_text_align(self.raw_pointer, align) };
    self
  }

  pub fn get_text_scale_x(&self) -> f32 {
    unsafe { sk_paint_get_text_scale_x(self.raw_pointer) }
  }
  pub fn set_text_scale_x(&mut self, scale: f32) -> &mut Self {
    unsafe { sk_paint_set_text_scale_x(self.raw_pointer, scale) };
    self
  }
  pub fn get_text_skew_x(&self) -> f32 {
    unsafe { sk_paint_get_text_skew_x(self.raw_pointer) }
  }

  pub fn set_text_skew_x(&mut self, skew: f32) -> &mut Self {
    unsafe { sk_paint_set_text_skew_x(self.raw_pointer, skew) };
    self
  }

  pub fn break_text(&self, text: &str, maxWidth: f32) -> (usize, f32) {
    let mut measuredWidth = 0.0;
    let size = unsafe {
      sk_paint_break_text(
        self.raw_pointer,
        text.as_ptr() as *const c_void,
        text.len(),
        maxWidth,
        &mut measuredWidth,
      )
    };
    (size, measuredWidth)
  }

  pub fn get_text_path(&mut self, text: &str, x: f32, y: f32) -> Path {
    wrap_safe_type! {
      Path <= sk_paint_get_text_path(self.raw_pointer, text.as_ptr() as *const c_void, text.len(), x, y)
    }
  }

  pub fn get_pos_text_path(&mut self, text: &str, pos: &[Point]) -> Path {
    wrap_safe_type! {
      Path <= sk_paint_get_pos_text_path(self.raw_pointer, text.as_ptr() as *const c_void,text.len(), pos.as_ptr())
    }
  }

  pub fn get_fontmetrics(&mut self, scale: f32) -> (f32, FontMetrics) {
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
    let recommend_space =
      unsafe { sk_paint_get_fontmetrics(self.raw_pointer, &mut font_metrics, scale) };
    (recommend_space, font_metrics)
  }

  pub fn get_path_effect(&mut self) -> &PathEffect {
    unsafe { &*sk_paint_get_path_effect(self.raw_pointer) }
  }

  pub fn set_path_effect(&mut self, effect: &mut PathEffect) -> &mut Self {
    unsafe { sk_paint_set_path_effect(self.raw_pointer, effect) };
    self
  }
  pub fn is_linear_text(&self) -> bool {
    unsafe { sk_paint_is_linear_text(self.raw_pointer) }
  }

  pub fn set_linear_text(&mut self, linear: bool) -> &mut Self {
    unsafe { sk_paint_set_linear_text(self.raw_pointer, linear) };
    self
  }

  pub fn is_subpixel_text(&self) -> bool {
    unsafe { sk_paint_is_subpixel_text(self.raw_pointer) }
  }

  pub fn set_subpixel_text(&mut self, subpixel: bool) -> &mut Self {
    unsafe { sk_paint_set_subpixel_text(self.raw_pointer, subpixel) };
    self
  }

  pub fn is_lcd_render_text(&self) -> bool {
    unsafe { sk_paint_is_lcd_render_text(self.raw_pointer) }
  }

  pub fn set_lcd_render_text(&mut self, lcd: bool) -> &mut Self {
    unsafe { sk_paint_set_lcd_render_text(self.raw_pointer, lcd) };
    self
  }

  pub fn is_embedded_bitmap_text(&self) -> bool {
    unsafe { sk_paint_is_embedded_bitmap_text(self.raw_pointer) }
  }

  pub fn set_embedded_bitmap_text(&mut self, embedde_bitmap: bool) -> &mut Self {
    unsafe { sk_paint_set_embedded_bitmap_text(self.raw_pointer, embedde_bitmap) };
    self
  }

  pub fn get_hinting(&self) -> PaintHinting {
    unsafe { sk_paint_get_hinting(self.raw_pointer) }
  }

  pub fn set_hinting(&mut self, paint_hinting: PaintHinting) -> &mut Self {
    unsafe { sk_paint_set_hinting(self.raw_pointer, paint_hinting) };
    self
  }

  pub fn is_autohinted(&self) -> bool {
    unsafe { sk_paint_is_autohinted(self.raw_pointer) }
  }

  pub fn set_autohinted(&mut self, autohinted: bool) -> &mut Self {
    unsafe { sk_paint_set_autohinted(self.raw_pointer, autohinted) };
    self
  }
  pub fn is_fake_bold_text(&self) -> bool {
    unsafe { sk_paint_is_fake_bold_text(self.raw_pointer) }
  }

  pub fn set_fake_bold_text(&mut self, fake_bold: bool) -> &mut Self {
    unsafe { sk_paint_set_fake_bold_text(self.raw_pointer, fake_bold) };
    self
  }

  pub fn is_dev_kern_text(&self) -> bool {
    unsafe { sk_paint_is_dev_kern_text(self.raw_pointer) }
  }

  pub fn set_dev_kern_text(&mut self, dev_kern: bool) -> &mut Self {
    unsafe { sk_paint_set_dev_kern_text(self.raw_pointer, dev_kern) };
    self
  }

  pub fn get_fill_path(&self, src: &Path, dst: &Path, cull_rect: &Rect, res_scale: f32) -> bool {
    unsafe {
      sk_paint_get_fill_path(
        self.raw_pointer,
        src.raw_pointer,
        dst.raw_pointer,
        cull_rect,
        res_scale,
      )
    }
  }

  pub fn text_glyphs_count(&self, text: &str) -> usize {
    unsafe {
      sk_paint_text_to_glyphs(
        self.raw_pointer,
        text.as_ptr() as *const c_void,
        text.len(),
        std::ptr::null_mut(),
      ) as usize
    }
  }

  pub fn text_to_glyphs(&self, text: &str) -> Vec<u16> {
    let count = self.text_glyphs_count(text);
    let mut glyphs: Vec<u16> = Vec::with_capacity(count);
    unsafe {
      sk_paint_text_to_glyphs(
        self.raw_pointer,
        text.as_ptr() as *const c_void,
        text.len(),
        glyphs.as_mut_ptr(),
      )
    };
    glyphs
  }

  pub fn contains_text(&self, text: &str) -> bool {
    unsafe { sk_paint_contains_text(self.raw_pointer, text.as_ptr() as *const c_void, text.len()) }
  }

  pub fn count_text(&self, text: &str) -> i32 {
    unsafe { sk_paint_count_text(self.raw_pointer, text.as_ptr() as *const c_void, text.len()) }
  }

  pub fn get_text_widths(&self, text: &str) -> (Vec<f32>, Vec<Rect>) {
    let count = self.text_glyphs_count(text);
    let mut widths: Vec<f32> = Vec::with_capacity(count);
    let mut bounds: Vec<Rect> = Vec::with_capacity(count);

    unsafe {
      sk_paint_get_text_widths(
        self.raw_pointer,
        text.as_ptr() as *const c_void,
        text.len(),
        widths.as_mut_ptr(),
        bounds.as_mut_ptr(),
      )
    };
    (widths, bounds)
  }

  pub fn get_text_intercepts(
    &self,
    text: &str,
    x: f32,
    y: f32,
    upper_bounds: f32,
    lower_bounds: f32,
  ) -> Vec<f32> {
    let bounds = [upper_bounds, lower_bounds];
    unsafe {
      let count = sk_paint_get_text_intercepts(
        self.raw_pointer,
        text.as_ptr() as *const c_void,
        text.len(),
        x,
        y,
        bounds.as_ptr(),
        std::ptr::null_mut(),
      );
      let mut intervals: Vec<f32> = Vec::with_capacity(count as usize);
      sk_paint_get_text_intercepts(
        self.raw_pointer,
        text.as_ptr() as *const c_void,
        text.len(),
        x,
        y,
        bounds.as_ptr(),
        intervals.as_mut_ptr(),
      );
      intervals
    }
  }

  pub fn get_pos_text_intercepts(
    &self,
    text: &str,
    pos: &mut [Point],
    upper_bounds: f32,
    lower_bounds: f32,
  ) -> Vec<f32> {
    let bounds = [upper_bounds, lower_bounds];
    unsafe {
      let count = sk_paint_get_pos_text_intercepts(
        self.raw_pointer,
        text.as_ptr() as *const c_void,
        text.len(),
        pos.as_mut_ptr(),
        bounds.as_ptr(),
        std::ptr::null_mut(),
      );
      let mut intervals: Vec<f32> = Vec::with_capacity(count as usize);
      sk_paint_get_pos_text_intercepts(
        self.raw_pointer,
        text.as_ptr() as *const c_void,
        text.len(),
        pos.as_mut_ptr(),
        bounds.as_ptr(),
        intervals.as_mut_ptr(),
      );
      intervals
    }
  }

  pub fn get_pos_text_h_intercepts(
    &self,
    text: &str,
    xpos: &mut [f32],
    y: f32,
    upper_bounds: f32,
    lower_bounds: f32,
  ) -> Vec<f32> {
    let bounds = [upper_bounds, lower_bounds];
    unsafe {
      let count = sk_paint_get_pos_text_h_intercepts(
        self.raw_pointer,
        text.as_ptr() as *const c_void,
        text.len(),
        xpos.as_mut_ptr(),
        y,
        bounds.as_ptr(),
        std::ptr::null_mut(),
      );
      let mut intervals: Vec<f32> = Vec::with_capacity(count as usize);
      sk_paint_get_pos_text_h_intercepts(
        self.raw_pointer,
        text.as_ptr() as *const c_void,
        text.len(),
        xpos.as_mut_ptr(),
        y,
        bounds.as_ptr(),
        intervals.as_mut_ptr(),
      );
      intervals
    }
  }

  pub fn get_pos_text_blob_intercepts(
    &self,
    blob: &mut TextBlob,
    upper_bounds: f32,
    lower_bounds: f32,
  ) -> Vec<f32> {
    let bounds = [upper_bounds, lower_bounds];
    unsafe {
      let count = sk_paint_get_pos_text_blob_intercepts(
        self.raw_pointer,
        blob.raw_pointer,
        bounds.as_ptr(),
        std::ptr::null_mut(),
      ) as usize;
      let mut intervals: Vec<f32> = Vec::with_capacity(count);
      sk_paint_get_pos_text_blob_intercepts(
        self.raw_pointer,
        blob.raw_pointer,
        bounds.as_ptr(),
        intervals.as_mut_ptr(),
      );
      intervals
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

  pub fn get_text_size(&self) -> f32 {
    unsafe { sk_paint_get_textsize(self.raw_pointer) }
  }

  pub fn set_text_size(&mut self, size: f32) -> &mut Self {
    unsafe { sk_paint_set_textsize(self.raw_pointer, size) };
    self
  }

  pub fn get_typeface(&self) -> Typeface {
    Typeface {
      raw_pointer: unsafe { sk_paint_get_typeface(self.raw_pointer) },
    }
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
    let mut rect = Rect {
      left: 0.0,
      top: 0.0,
      right: 0.0,
      bottom: 0.0,
    };

    let width = unsafe {
      sk_paint_measure_text(
        self.raw_pointer,
        text.as_ptr() as *const c_void,
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

  pub fn get_shader(&self) -> Shader {
    let shader = unsafe { sk_paint_get_shader(self.raw_pointer) };
    Shader {
      raw_pointer: shader,
    }
  }

  pub fn set_shader(&mut self, shader: &Shader) -> &mut Self {
    unsafe { sk_paint_set_shader(self.raw_pointer, shader.raw_pointer) };
    self
  }

  pub fn get_maskfilter(&self) -> MaskFilter {
    MaskFilter {
      raw_pointer: unsafe { sk_paint_get_maskfilter(self.raw_pointer) },
    }
  }

  pub fn set_maskfilter(&mut self, filter: &MaskFilter) -> &mut Self {
    unsafe { sk_paint_set_maskfilter(self.raw_pointer, filter.raw_pointer) };
    self
  }

  pub fn get_blendmode(&self) -> BlendMode {
    unsafe { sk_paint_get_blendmode(self.raw_pointer) }
  }

  pub fn set_blendmode(&mut self, mode: BlendMode) -> &mut Self {
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

impl Clone for Paint {
  fn clone(&self) -> Paint {
    Paint {
      raw_pointer: unsafe { sk_paint_clone(self.raw_pointer) },
    }
  }
}
