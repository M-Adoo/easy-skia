use bindings::*;
use {wrap_safe_type, Canvas, Drawable, Rect};

pub struct PictureRecorder {
  pub(crate) raw_pointer: *mut sk_picture_t,
}

impl Drop for PictureRecorder {
  fn drop(&mut self) {
    unsafe { sk_picture_recorder_delete(self.raw_pointer) }
  }
}

impl PictureRecorder {
  fn new() {
    wrap_safe_type! {
      PictureRecorder <= sk_picture_recorder_new()
    }
  }

  fn begin_recording(&mut self, rect: &Rect) -> Canvas {
    wrap_safe_type! {
      Canvas <= sk_picture_recorder_begin_recording(self.raw_pointer, rect)
    }
  }

  fn end_recording(&mut self) -> Picture {
    wrap_safe_type! {
      Picture <= sk_picture_recorder_end_recording(self.raw_pointer)
    }
  }

  fn nd_recording_as_drawable(&mut self) -> Drawable {
    wrap_safe_type! {
      Drawable <= sk_picture_recorder_end_recording_as_drawable(self.raw_pointer)
    }
  }

  fn get_recording_canvas(&mut self) -> Canvas {
    wrap_safe_type! {
      Canvas <= sk_picture_get_recording_canvas(self.raw_pointer)
    }
  }
}

pub struct Picture {
  pub(crate) raw_pointer: *mut sk_picture_t,
}

impl Drop for Picture {
  fn drop(&mut self) {
    unsafe { sk_picture_unref(self.raw_pointer) }
  }
}

impl Picture {
  fn unique_id(&mut self) -> u32 {
    unsafe { sk_picture_get_unique_id(self.raw_pointer) }
  }

  fn get_cull_rect(&mut self) -> Rect {
    let mut rect = Rect {
      left: 0,
      right: 0,
      top: 0,
      bottom: 0,
    };
    unsafe { sk_picture_get_cull_rect(self.raw_pointer, &mut rect) };
    rect
  }
}
