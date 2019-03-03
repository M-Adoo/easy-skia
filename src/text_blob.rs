use bindings::sk_textblob_builder_runbuffer_t as RunBuffer;
use bindings::*;
use {wrap_safe_type, Paint, Point, Rect, SKString};
pub struct TextBlob {
  pub(crate) raw_pointer: *mut sk_textblob_t,
}

impl Drop for TextBlob {
  fn drop(&mut self) {
    unsafe { sk_textblob_unref(self.raw_pointer) }
  }
}

impl TextBlob {
  pub fn unique_id(&self) -> u32 {
    unsafe { sk_textblob_get_unique_id(self.raw_pointer) }
  }

  pub fn get_bounds(&self) -> Rect {
    let mut rect = Rect {
      left: 0.0,
      right: 0.,
      top: 0.,
      bottom: 0.,
    };
    unsafe { sk_textblob_get_bounds(self.raw_pointer, &mut rect) };
    rect
  }
}

pub struct TextBlobBuilder {
  pub(crate) raw_pointer: *mut sk_textblob_builder_t,
}

impl Drop for TextBlobBuilder {
  fn drop(&mut self) {
    unsafe { sk_textblob_builder_delete(self.raw_pointer) }
  }
}

impl TextBlobBuilder {
  pub fn new() -> Self {
    wrap_safe_type! {
      TextBlobBuilder <= sk_textblob_builder_new()
    }
  }

  pub fn make(&mut self) -> TextBlob {
    wrap_safe_type! {
      TextBlob <= sk_textblob_builder_make(self.raw_pointer)
    }
  }

  pub fn alloc_run_text(
    &mut self,
    font: &Paint,
    count: i32,
    x: f32,
    y: f32,
    text_byte_count: i32,
    lang: &SKString,
    bounds: &Rect,
  ) -> RunBuffer {
    let mut buffer = RunBuffer {
      glyphs: std::ptr::null_mut(),
      pos: std::ptr::null_mut(),
      utf8text: std::ptr::null_mut(),
      clusters: std::ptr::null_mut(),
    };
    unsafe {
      sk_textblob_builder_alloc_run_text(
        self.raw_pointer,
        font.raw_pointer,
        count,
        x,
        y,
        text_byte_count,
        lang.raw_pointer,
        bounds,
        &mut buffer,
      );
    }
    buffer
  }

  pub fn alloc_run_text_pos_h(
    &mut self,
    font: &Paint,
    count: i32,
    y: f32,
    text_byte_count: i32,
    lang: &SKString,
    bounds: &Rect,
  ) -> RunBuffer {
    let mut buffer = RunBuffer {
      glyphs: std::ptr::null_mut(),
      pos: std::ptr::null_mut(),
      utf8text: std::ptr::null_mut(),
      clusters: std::ptr::null_mut(),
    };
    unsafe {
      sk_textblob_builder_alloc_run_text_pos_h(
        self.raw_pointer,
        font.raw_pointer,
        count,
        y,
        text_byte_count,
        lang.raw_pointer,
        bounds,
        &mut buffer,
      );
    }
    buffer
  }

  pub fn alloc_run_text_pos(
    &mut self,
    font: &Paint,
    count: i32,
    text_byte_count: i32,
    lang: &SKString,
    bounds: &Rect,
  ) -> RunBuffer {
    let mut buffer = RunBuffer {
      glyphs: std::ptr::null_mut(),
      pos: std::ptr::null_mut(),
      utf8text: std::ptr::null_mut(),
      clusters: std::ptr::null_mut(),
    };
    unsafe {
      sk_textblob_builder_alloc_run_text_pos(
        self.raw_pointer,
        font.raw_pointer,
        count,
        text_byte_count,
        lang.raw_pointer,
        bounds,
        &mut buffer,
      );
    }
    buffer
  }
}

impl RunBuffer {
  pub fn set_glyphs(&self, glyphs: &[u16]) -> &Self {
    unsafe {
      sk_textblob_builder_runbuffer_set_glyphs(self, glyphs.as_ptr(), glyphs.len() as i32);
    }
    self
  }

  pub fn set_pos(&self, pos: &[f32]) -> &Self {
    unsafe {
      sk_textblob_builder_runbuffer_set_pos(self, pos.as_ptr(), pos.len() as i32);
    }
    self
  }

  pub fn set_pos_points(&self, point: &[Point]) -> &Self {
    unsafe {
      sk_textblob_builder_runbuffer_set_pos_points(self, point.as_ptr(), point.len() as i32);
    };
    self
  }

  pub fn set_utf8_text(&self, text: &str) -> &Self {
    let c_text = std::ffi::CString::new(text).unwrap();
    unsafe {
      sk_textblob_builder_runbuffer_set_utf8_text(self, c_text.as_ptr(), text.len() as i32);
    }
    self
  }

  pub fn set_clusters(&self, clusters: &[u32]) -> &Self {
    unsafe {
      sk_textblob_builder_runbuffer_set_clusters(self, clusters.as_ptr(), clusters.len() as i32);
    }
    self
  }
}
