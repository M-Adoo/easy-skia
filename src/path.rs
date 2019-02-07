use bindings::*;
use Rect;

pub use bindings::sk_path_direction_t as PathDirection;

pub struct Path {
    pub(crate) raw_pointer: *mut sk_path_t,
}

impl Path {
    pub fn new() -> Path {
        let raw_pointer = unsafe { sk_path_new() };
        if raw_pointer.is_null() {
            panic!("Cannot create path");
        }

        Path { raw_pointer }
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        unsafe { sk_path_move_to(self.raw_pointer, x, y) };
    }

    pub fn line_to(&mut self, x: f32, y: f32) {
        unsafe { sk_path_line_to(self.raw_pointer, x, y) };
    }

    pub fn quad_to(&mut self, x0: f32, y0: f32, x1: f32, y1: f32) {
        unsafe { sk_path_quad_to(self.raw_pointer, x0, y0, x1, y1) };
    }

    pub fn conic_to(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, w: f32) {
        unsafe { sk_path_conic_to(self.raw_pointer, x0, y0, x1, y1, w) };
    }

    pub fn cubic_to(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32) {
        unsafe { sk_path_cubic_to(self.raw_pointer, x0, y0, x1, y1, x2, y2) };
    }

    pub fn close(&mut self) {
        unsafe { sk_path_close(self.raw_pointer) };
    }

    pub fn add_rect(&mut self, rect: &Rect, direction: PathDirection) {
        unsafe { sk_path_add_rect(self.raw_pointer, rect, direction) };
    }

    pub fn add_oval(&mut self, rect: &Rect, direction: PathDirection) {
        unsafe { sk_path_add_oval(self.raw_pointer, rect, direction) };
    }

    pub fn get_bounds(&mut self, direction: PathDirection) -> Rect {
        let rect = Rect {
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
        };

        unsafe { sk_path_add_rect(self.raw_pointer, &rect, direction) };

        rect
    }
}

impl Drop for Path {
    fn drop(&mut self) {
        unsafe { sk_path_delete(self.raw_pointer) };
    }
}
