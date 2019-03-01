pub use bindings::sk_irect_t as IRect;
pub use bindings::sk_region_op_t as RegionOP;
pub use bindings::*;
use wrap_safe_type;

use Path;

pub struct Region {
  pub(crate) raw_pointer: *mut sk_region_t,
}

impl Drop for Region {
  fn drop(&mut self) {
    unsafe {
      sk_region_delete(self.raw_pointer);
    }
  }
}

impl Clone for Region {
  fn clone(&self) -> Region {
    wrap_safe_type! {
      Region <= sk_region_new2(self.raw_pointer)
    }
  }
}

impl Region {
  fn new() -> Self {
    wrap_safe_type! {
      Region <= sk_region_new()
    }
  }

  // todo
  // these below C API is not correct
  // SK_C_API void sk_region_contains(sk_region_t* r, const sk_region_t* region);
  // SK_C_API void sk_region_contains2(sk_region_t* r, int x, int y);

  fn intersects(&mut self, dst: &Region) -> bool {
    unsafe { sk_region_intersects(dst.raw_pointer, dst.raw_pointer) }
  }

  fn set_path(&mut self, path: &Path, clip: &Region) -> bool {
    unsafe { sk_region_set_path(self.raw_pointer, path.raw_pointer, clip.raw_pointer) }
  }

  fn set_rect(&mut self, rect: &IRect) -> bool {
    unsafe { sk_region_set_rect(self.raw_pointer, rect) }
  }

  fn set_region(&mut self, region: &Region) -> bool {
    unsafe { sk_region_set_region(self.raw_pointer, region.raw_pointer) }
  }

  fn op(&mut self, left: i32, top: i32, right: i32, bottom: i32, op: RegionOP) -> bool {
    unsafe { sk_region_op(self.raw_pointer, left, top, right, bottom, op) }
  }

  fn op2(&mut self, src: &Region, op: RegionOP) -> bool {
    unsafe { sk_region_op2(self.raw_pointer, src.raw_pointer, op) }
  }

  fn get_bounds(&mut self) -> IRect {
    let mut rect = IRect {
      left: 0,
      bottom: 0,
      right: 0,
      top: 0,
    };
    unsafe { sk_region_get_bounds(self.raw_pointer, &rect) }
    rect
  }
}
