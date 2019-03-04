pub use bindings::sk_path_effect_1d_style_t as PathEffect1dStyle;
pub use bindings::sk_path_effect_trim_mode_t as PathEffectTrimMode;
use bindings::*;
use {wrap_safe_type, Matrix, Path};

pub struct PathEffect {
  pub(crate) raw_pointer: *mut sk_path_effect_t,
}

impl Drop for PathEffect {
  fn drop(&mut self) {
    unsafe { sk_path_effect_unref(self.raw_pointer) }
  }
}

impl PathEffect {
  pub fn create_compose(outter: &mut Self, inner: &mut Self) -> Self {
    wrap_safe_type! {
      PathEffect <= sk_path_effect_create_compose(outter.raw_pointer, inner.raw_pointer)
    }
  }

  pub fn create_sum(first: &mut Self, second: &mut Self) -> Self {
    wrap_safe_type! {
      PathEffect <= sk_path_effect_create_sum(first.raw_pointer, second.raw_pointer)
    }
  }

  pub fn create_discrete(seg_len: f32, deviation: f32, seed_assist: u32) -> Self {
    wrap_safe_type! {
      PathEffect <= sk_path_effect_create_discrete(seg_len, deviation, seed_assist)
    }
  }

  pub fn create_corner(radius: f32) -> Self {
    wrap_safe_type! {
      PathEffect <= sk_path_effect_create_corner(radius)
    }
  }

  pub fn create_1d_path(path: &Path, advance: f32, phase: f32, style: PathEffect1dStyle) -> Self {
    wrap_safe_type! {
      PathEffect <= sk_path_effect_create_1d_path(path.raw_pointer, advance, phase, style)
    }
  }

  pub fn create_2d_line(width: f32, matrix: &Matrix) -> Self {
    wrap_safe_type! {
      PathEffect <= sk_path_effect_create_2d_line(width, matrix)
    }
  }

  pub fn create_2d_path(matrix: &Matrix, path: &Path) -> Self {
    wrap_safe_type! {
      PathEffect <= sk_path_effect_create_2d_path(matrix, path.raw_pointer)
    }
  }

  pub fn create_dash(intervals: &[f32], phase: f32) -> Self {
    wrap_safe_type! {
      PathEffect <= sk_path_effect_create_dash(intervals.as_ptr(), intervals.len() as i32, phase)
    }
  }

  pub fn create_trim(start: f32, stop: f32, mode: PathEffectTrimMode) -> Self {
    wrap_safe_type! {
      PathEffect <= sk_path_effect_create_trim(start, stop, mode)
    }
  }
}
