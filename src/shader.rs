use bindings::*;
use {Color, Matrix, Point};

pub use bindings::sk_shader_tilemode_t as TileMode;

pub struct Shader {
    pub(crate) raw_pointer: *mut sk_shader_t,
}

impl Shader {
    pub fn new_linear_gradient(
        points: [Point; 2],
        colors: Vec<Color>,
        color_pos: Vec<f32>,
        tile_mode: TileMode,
        matrix: &Matrix,
    ) -> Shader {
        let colors: Vec<u32> = colors.iter().map(|c| c.0).collect();

        let raw_pointer = unsafe {
            sk_shader_new_linear_gradient(
                points.as_ptr(),
                colors[..].as_ptr(),
                color_pos[..].as_ptr(),
                colors.len() as i32,
                tile_mode,
                matrix,
            )
        };

        if raw_pointer.is_null() {
            panic!("Cannot create gradient");
        }

        Shader { raw_pointer }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { sk_shader_unref(self.raw_pointer) };
    }
}
