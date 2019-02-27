pub use bindings::sk_highcontrastconfig_invertstyle_t as HighcontrastconfigInvertstyle;
pub use bindings::sk_highcontrastconfig_t as Highcontrastconfig;
use bindings::*;
use {BlendMode, Color};

pub struct ColorFilter {
  pub(crate) raw_pointer: *mut sk_colorfilter_t,
}

impl Drop for ColorFilter {
  fn drop(&mut self) {
    unsafe { sk_colorfilter_unref(self.raw_pointer) };
  }
}

impl ColorFilter {
  fn new_mode(color: Color, mode: BlendMode) -> Self {
    ColorFilter {
      raw_pointer: unsafe { sk_colorfilter_new_mode(color.0, mode) },
    }
  }

  fn new_lighting(mul: Color, add: Color) -> Self {
    ColorFilter {
      raw_pointer: unsafe { sk_colorfilter_new_lighting(mul.0, add.0) },
    }
  }

  fn new_compose(&mut self, inner: &mut Self) -> Self {
    ColorFilter {
      raw_pointer: unsafe { sk_colorfilter_new_compose(self.raw_pointer, inner.raw_pointer) },
    }
  }

  fn new_color_matrix(arr: &[f32; 20]) -> Self {
    ColorFilter {
      raw_pointer: unsafe { sk_colorfilter_new_color_matrix(arr.as_ptr()) },
    }
  }

  fn new_luma_color() -> Self {
    ColorFilter {
      raw_pointer: unsafe { sk_colorfilter_new_luma_color() },
    }
  }

  fn new_high_contrast(config: &Highcontrastconfig) -> Self {
    ColorFilter {
      raw_pointer: unsafe { sk_colorfilter_new_high_contrast(config) },
    }
  }

  fn new_table(table: &[u8; 256]) -> Self {
    ColorFilter {
      raw_pointer: unsafe { sk_colorfilter_new_table(table.as_ptr()) },
    }
  }

  fn new_table_argb(
    tableA: &[u8; 256],
    tableR: &[u8; 256],
    tableG: &[u8; 256],
    tableB: &[u8; 256],
  ) -> Self {
    ColorFilter {
      raw_pointer: unsafe {
        sk_colorfilter_new_table_argb(
          tableA.as_ptr(),
          tableR.as_ptr(),
          tableG.as_ptr(),
          tableB.as_ptr(),
        )
      },
    }
  }
}
