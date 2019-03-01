#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod bindings;
mod context;
pub use self::context::*;
mod paint;
pub use self::paint::*;
mod color;
pub use self::color::*;
mod canvas;
pub use self::canvas::*;
mod path;
pub use self::path::*;
mod image;
pub use self::image::*;
mod color_space;
pub use self::color_space::*;
mod picture;
pub use self::picture::*;
mod data;
pub use self::data::*;
mod maskfilter;
pub use self::maskfilter::*;
mod shader;
pub use self::shader::*;
mod surface;
pub use self::surface::*;
mod typeface;
pub use self::typeface::*;
mod colorfilter;
pub use self::colorfilter::*;
mod imagefilter;
pub use self::imagefilter::*;
mod region;
pub use self::region::*;

pub use bindings::gr_pixelconfig_t as GrPixelConfig;
pub use bindings::sk_alphatype_t as AlphaType;
pub use bindings::sk_blendmode_t;
pub use bindings::sk_clipop_t;
pub use bindings::sk_imageinfo_t as ImageInfo;
pub use bindings::sk_point_t as Point;
pub use bindings::sk_shader_tilemode_t as ShaderTileMode;
pub use bindings::sk_text_encoding_t as TextEncoding;

#[macro_export]
macro_rules! unwrap_raw_pointer {
  ( $x:expr  ) => {{
    if let Some(wrap) = $x {
      wrap.raw_pointer
    } else {
      std::ptr::null_mut()
    }
  }};
}

#[macro_export]
macro_rules! wrap_safe_type {
  ($name: ident <= $body: expr ) => {
    $name {
      raw_pointer: unsafe { $body },
    }
  };
}
