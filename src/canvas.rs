use std::ffi::CString;
use std::marker::PhantomData;

use bindings::*;
use {Image, Matrix, Paint, Path, Picture, Rect};

pub struct Canvas<'a> {
    pub(crate) raw_pointer: *mut sk_canvas_t,

    pub(crate) phantom: PhantomData<&'a sk_canvas_t>,
}

impl<'a> Canvas<'a> {
    pub fn save(&mut self) {
        unsafe { sk_canvas_save(self.raw_pointer) };
    }

    pub fn save_layer(&mut self, rect: &Rect, paint: &Paint) {
        unsafe { sk_canvas_save_layer(self.raw_pointer, rect, paint.raw_pointer) };
    }

    pub fn restore(&mut self) {
        unsafe { sk_canvas_restore(self.raw_pointer) };
    }

    pub fn flush(&mut self) {
        unsafe { sk_canvas_flush(self.raw_pointer)};
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        unsafe { sk_canvas_translate(self.raw_pointer, dx, dy) };
    }

    pub fn scale(&mut self, sx: f32, sy: f32) {
        unsafe { sk_canvas_scale(self.raw_pointer, sx, sy) };
    }

    pub fn rotate_degrees(&mut self, degrees: f32) {
        unsafe { sk_canvas_rotate_degrees(self.raw_pointer, degrees) };
    }

    pub fn rotate_radians(&mut self, radians: f32) {
        unsafe { sk_canvas_rotate_radians(self.raw_pointer, radians) };
    }

    pub fn skew(&mut self, sx: f32, sy: f32) {
        unsafe { sk_canvas_skew(self.raw_pointer, sx, sy) };
    }

    pub fn concat(&mut self, matrix: &Matrix) {
        unsafe { sk_canvas_concat(self.raw_pointer, matrix) };
    }

    pub fn draw_paint(&mut self, paint: &Paint) {
        unsafe { sk_canvas_draw_paint(self.raw_pointer, paint.raw_pointer) };
    }

    pub fn draw_rect(&mut self, rect: &Rect, paint: &Paint) {
        unsafe { sk_canvas_draw_rect(self.raw_pointer, rect, paint.raw_pointer) };
    }

    pub fn draw_circle(&mut self, cx: f32, cy: f32, rad: f32, paint: &Paint) {
        unsafe { sk_canvas_draw_circle(self.raw_pointer, cx, cy, rad, paint.raw_pointer) };
    }

    pub fn draw_oval(&mut self, rect: &Rect, paint: &Paint) {
        unsafe { sk_canvas_draw_oval(self.raw_pointer, rect, paint.raw_pointer) };
    }

    pub fn draw_path(&mut self, path: &Path, paint: &Paint) {
        unsafe {
            sk_canvas_draw_path(
                self.raw_pointer,
                path.raw_pointer,
                paint.raw_pointer,
            )
        };
    }

    pub fn draw_text(&mut self, text: &str, x: f32, y: f32, paint: &Paint) {
        let cstr = CString::new(text).unwrap();
        unsafe {
            sk_canvas_draw_text(
                self.raw_pointer,
                cstr.as_ptr(),
                text.len(),
                x,
                y,
                paint.raw_pointer,
            )
        };
    }

    pub fn draw_text_blob(&mut self, blob: *mut sk_textblob_t, x: f32, y: f32, paint: &Paint) {
        unsafe { sk_canvas_draw_text_blob(self.raw_pointer, blob, x, y, paint.raw_pointer) };
    }

    pub fn draw_image(&mut self, image: &Image, x: f32, y: f32, paint: &Paint) {
        unsafe {
            sk_canvas_draw_image(
                self.raw_pointer,
                image.raw_pointer,
                x,
                y,
                paint.raw_pointer,
            )
        };
    }

    pub fn draw_image_rect(&mut self, image: &Image, src: &Rect, dest: &Rect, paint: &Paint) {
        unsafe {
            sk_canvas_draw_image_rect(
                self.raw_pointer,
                image.raw_pointer,
                src,
                dest,
                paint.raw_pointer,
            )
        };
    }

    pub fn draw_picture(&mut self, picture: &Picture, matrix: &Matrix, paint: &Paint) {
        unsafe {
            sk_canvas_draw_picture(
                self.raw_pointer,
                picture.raw_pointer,
                matrix,
                paint.raw_pointer,
            )
        };
    }
}
