pub use bindings::sk_displacement_map_effect_channel_selector_type_t as DisplacementMapEffectChannelSelectorType;
pub use bindings::sk_drop_shadow_image_filter_shadow_mode_t as DropShadowImageFilterShadowMode;
pub use bindings::sk_ipoint_t as IPoint;
pub use bindings::sk_isize_t as ISize;
pub use bindings::sk_matrix_convolution_tilemode_t as MatrixConvolutionTilemode;
pub use bindings::sk_matrix_t as Matrix;
pub use bindings::sk_point3_t as Point3;
pub use bindings::sk_rect_t as Rect;
use bindings::*;
pub use bindings::*;
use {
  unwrap_raw_pointer, wrap_safe_type, BlendMode, Color, ColorFilter, FilterQuality, Image, Paint,
  Picture, Region,
};

pub struct CropRect {
  pub(crate) raw_pointer: *mut sk_imagefilter_croprect_t,
}

impl Drop for CropRect {
  fn drop(&mut self) {
    unsafe { sk_imagefilter_croprect_destructor(self.raw_pointer) };
  }
}

impl CropRect {
  fn new() -> Self {
    wrap_safe_type! {
      CropRect <= sk_imagefilter_croprect_new()
    }
  }

  fn new_with_rect(rect: &Rect, flags: u32) -> Self {
    wrap_safe_type! {
      CropRect <= sk_imagefilter_croprect_new_with_rect(rect, flags)
    }
  }

  fn get_rect(&self) -> Rect {
    let mut rect = Rect {
      left: 0.,
      right: 0.,
      bottom: 0.,
      top: 0.,
    };
    unsafe { sk_imagefilter_croprect_get_rect(self.raw_pointer, &mut rect) };
    rect
  }

  fn flags(&self) -> u32 {
    unsafe { sk_imagefilter_croprect_get_flags(self.raw_pointer) }
  }
}

pub struct ImageFilter {
  pub(crate) raw_pointer: *mut sk_imagefilter_t,
}

impl Drop for ImageFilter {
  fn drop(&mut self) {
    unsafe { sk_imagefilter_unref(self.raw_pointer) }
  }
}

impl ImageFilter {
  fn new_matrix(
    matrix: &Matrix,
    filter_quality: FilterQuality,
    input: Option<&mut ImageFilter>,
  ) -> Self {
    wrap_safe_type! {
    ImageFilter <=
      sk_imagefilter_new_matrix(matrix, filter_quality, unwrap_raw_pointer!(input))
    }
  }

  fn new_alpha_threshold(
    region: &Region,
    inner_threshold: f32,
    outer_threshold: f32,
    input: Option<&mut ImageFilter>,
  ) -> Self {
    wrap_safe_type! {
      ImageFilter <=
      sk_imagefilter_new_alpha_threshold(
        region,
        inner_threshold,
        outer_threshold,
        unwrap_raw_pointer!(input))
    }
  }

  fn new_blur(sigmaX: f32, sigmaY: f32, input: Option<&mut ImageFilter>) -> Self {
    wrap_safe_type! {
      ImageFilter <= sk_imagefilter_new_blur(sigmaX, sigmaY, unwrap_raw_pointer!(input))
    }
  }

  fn new_color_filter(
    cf: &ColorFilter,
    input: Option<&mut ImageFilter>,
    crop: Option<&CropRect>,
  ) -> Self {
    wrap_safe_type! {
      ImageFilter <=
      sk_imagefilter_new_color_filter(
          cf.raw_pointer,
          unwrap_raw_pointer!(input),
          unwrap_raw_pointer!(crop))
    }
  }

  fn new_compose(outer: &mut ImageFilter, inner: &mut ImageFilter) -> Self {
    wrap_safe_type! {
      ImageFilter <= sk_imagefilter_new_compose(outer.raw_pointer, inner.raw_pointer)
    }
  }

  fn new_displacement_map_effect(
    x_channel_selector: DisplacementMapEffectChannelSelectorType,
    y_channel_selector: DisplacementMapEffectChannelSelectorType,
    sacle: f32,
    displacement: &mut ImageFilter,
    color: Option<&mut ColorFilter>,
    crop: Option<&CropRect>,
  ) -> Self {
    wrap_safe_type! {
      ImageFilter <=
      sk_imagefilter_new_displacement_map_effect(
          x_channel_selector,
          y_channel_selector,
          displacement.raw_pointer,
          unwrap_raw_pointer!(color),
          unwrap_raw_pointer!(crop))
    }
  }

  fn new_drop_shadow(
    dx: f32,
    dy: f32,
    sigmaX: f32,
    sigmaY: f32,
    color: Color,
    shadow_mode: DropShadowImageFilterShadowMode,
    input: Option<&mut ImageFilter>,
    crop: Option<&CropRect>,
  ) -> Self {
    wrap_safe_type! {
      ImageFilter <=
      sk_imagefilter_new_drop_shadow(
          dx,
          dy,
          sigmaX,
          sigmaY,
          color.raw_pointer,
          unwrap_raw_pointer!(input),
          unwrap_raw_pointer!(crop))
    }
  }

  fn new_distant_lit_diffuse(
    direction: &Point3,
    light_color: Color,
    surface_scale: f32,
    kd: f32,
    input: Option<&mut ImageFilter>,
    crop: Option<&CropRect>,
  ) -> Self {
    wrap_safe_type! {
      ImageFilter <=
      sk_imagefilter_new_distant_lit_diffuse(
          direction,
          light_color,
          surface_scale,
          kd,
          unwrap_raw_pointer!(input),
          unwrap_raw_pointer!(crop))
    }
  }

  fn new_point_lit_diffuse(
    location: &Point3,
    light_color: Color,
    surface_scale: f32,
    kd: f32,
    input: Option<&mut ImageFilter>,
    crop: Option<&CropRect>,
  ) -> Self {
    wrap_safe_type! {
      ImageFilter <=
      sk_imagefilter_new_point_lit_diffuse(
          location,
          light_color,
          surface_scale,
          kd,
          unwrap_raw_pointer!(input),
          unwrap_raw_pointer!(crop))
    }
  }

  fn new_spot_lit_diffuse(
    location: &Point3,
    target: &Point3,
    specular_exponent: f32,
    cutoff_angle: f32,
    light_color: Color,
    surface_scale: f32,
    kd: f32,
    input: Option<&mut ImageFilter>,
    crop: Option<&CropRect>,
  ) -> Self {
    wrap_safe_type! {
      ImageFilter <=
      sk_imagefilter_new_spot_lit_diffuse(
          location,
          target,
          specular_exponent,
          cutoff_angle,
          light_color,
          surface_scale,
          kd,
          unwrap_raw_pointer!(input),
          unwrap_raw_pointer!(crop))
    }
  }

  fn new_distant_lit_specular(
    direction: &Point3,
    light_color: Color,
    surface_scale: f32,
    ks: f32,
    shininess: f32,
    input: Option<&mut ImageFilter>,
    crop: Option<&CropRect>,
  ) -> Self {
    wrap_safe_type! {
      ImageFilter <=
      sk_imagefilter_new_distant_lit_specular(
          direction,
          light_color,
          surface_scale,
          ks,
          shininess,
          unwrap_raw_pointer!(input),
          unwrap_raw_pointer!(crop))
    }
  }

  fn new_point_lit_specular(
    location: &Point3,
    light_color: Color,
    surface_scale: f32,
    ks: f32,
    shininess: f32,
    input: Option<&mut ImageFilter>,
    crop: Option<&CropRect>,
  ) -> Self {
    wrap_safe_type! {
    ImageFilter <=
    sk_imagefilter_new_point_lit_specular(
      location,
      light_color,
      surface_scale,
      ks,
      shininess,
      unwrap_raw_pointer!(input),
      unwrap_raw_pointer!(crop))
    }
  }

  fn new_spot_lit_specular(
    location: &Point3,
    target: &Point3,
    specular_exponent: f32,
    cutoff_angle: f32,
    light_color: Color,
    surface_scale: f32,
    ks: f32,
    shininess: f32,
    input: Option<&mut ImageFilter>,
  ) -> Self {
    wrap_safe_type! {
      ImageFilter <=
      sk_imagefilter_new_spot_lit_specular(
        location, target,
        specular_exponent, cutoff_angle,
        light_color, surface_scale, ks, shininess,
        unwrap_raw_pointer!(input))
    }
  }

  fn new_magnifier(
    src: &Rect,
    inset: f32,
    input: Option<&mut ImageFilter>,
    crop: Option<&CropRect>,
  ) -> Self {
    wrap_safe_type! {
      ImageFilter <=
      sk_imagefilter_new_magnifier(src, inset, unwrap_raw_pointer!(input), unwrap_raw_pointer!(crop))
    }
  }

  fn new_matrix_convolution(
    kernel_size: &ISize,
    kernel: &[f32],
    gain: f32,
    bias: f32,
    kernel_offset: &IPoint,
    tile_mode: MatrixConvolutionTilemode,
    convolve_alpha: bool,
    input: Option<&mut ImageFilter>,
    crop: Option<&CropRect>,
  ) -> Self {
    wrap_safe_type! {
      ImageFilter <=
      sk_imagefilter_new_matrix_convolution(
        kernel_size,
        kernel.as_ptr(),
        gain, bias,
        kernel_offset,
        tile_mode,
        convolve_alpha,
        unwrap_raw_pointer!(input),
        unwrap_raw_pointer!(crop))
    }
  }

  fn new_merge(filters: &[ImageFilter], crop: Option<&CropRect>) -> Self {
    wrap_safe_type! {
      ImageFilter <=
      sk_imagefilter_new_merge(filters.as_ptr(), filters.len(), unwrap_raw_pointer!(crop))
    }
  }

  fn new_dilate(
    radius_x: i32,
    radius_y: i32,
    input: Option<&mut ImageFilter>,
    crop: Option<&CropRect>,
  ) -> Self {
    wrap_safe_type! {
      ImageFilter <=
      sk_imagefilter_new_dilate(radius_x, radius_y, unwrap_raw_pointer!(input),
      unwrap_raw_pointer!(crop))
    }
  }

  fn new_erode(
    radius_x: i32,
    radius_y: i32,
    input: Option<&mut ImageFilter>,
    crop: Option<&CropRect>,
  ) -> Self {
    wrap_safe_type! {
      ImageFilter <=
      sk_imagefilter_new_erode(
        radius_x, radius_y,
        unwrap_raw_pointer!(input),
        unwrap_raw_pointer!(crop))
    }
  }

  fn new_offset(
    dx: f32,
    dy: f32,
    input: Option<&mut ImageFilter>,
    crop: Option<&CropRect>,
  ) -> Self {
    wrap_safe_type! {
      ImageFilter <=
      sk_imagefilter_new_offset(
        dx, dy,
        unwrap_raw_pointer!(input),
        unwrap_raw_pointer!(crop))
    }
  }

  fn new_picture(picture: &Picture) -> Self {
    wrap_safe_type! {
      ImageFilter <= sk_imagefilter_new_picture(picture.raw_pointer)
    }
  }

  fn new_picture_with_croprect(pic: &Picture, rect: &CropRect) -> Self {
    wrap_safe_type! {
      ImageFilter <= sk_imagefilter_new_picture_with_croprect(pic.raw_pointer, rect.raw_pointer)
    }
  }

  fn new_tile(&src: Rect, dst: &Rect, input: &mut ImageFilter) -> Self {
    wrap_safe_type! {
      ImageFilter <= sk_imagefilter_new_tile(src, dst, input.raw_pointer)
    }
  }

  fn new_xfermode(
    mode: BlendMode,
    &background: &mut ImageFilter,
    foreground: Option<&mut ImageFilter>,
    crop: Option<&CropRect>,
  ) -> Self {
    wrap_safe_type! {
      ImageFilter <= sk_imagefilter_new_xfermode(
        mode,
        background.raw_pointer,
        unwrap_raw_pointer!(foreground),
        unwrap_raw_pointer!(crop))
    }
  }

  fn new_arithmetic(
    k1: f32,
    k2: f32,
    k3: f32,
    k4: f32,
    enforce_PM_color: bool,
    background: &mut ImageFilter,
    foreground: Option<&mut ImageFilter>,
    crop: Option<&CropRect>,
  ) -> Self {
    wrap_safe_type! {
      ImageFilter <= sk_imagefilter_new_arithmetic(
        k1, k2, k3, k4, enforce_PM_color,
        background,
        unwrap_raw_pointer!(foreground),
        unwrap_raw_pointer!(crop))
    }
  }

  fn new_image_source(image: Image, src: &Rect, dst: &Rect, filter_quality: FilterQuality) -> Self {
    wrap_safe_type! {
      ImageFilter <= sk_imagefilter_new_image_source(image.raw_pointer, src, dst, filter_quality)
    }
  }

  fn new_image_source_default(image: &mut Image) -> Self {
    wrap_safe_type! {
      ImageFilter <= sk_imagefilter_new_image_source_default(image.raw_pointer)
    }
  }

  fn new_paint(paint: &Paint, crop: Option<&CropRect>) -> Self {
    wrap_safe_type! {
      ImageFilter <= sk_imagefilter_new_paint(paint.raw_pointer, unwrap_raw_pointer!(crop))
    }
  }
}
