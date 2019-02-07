extern crate easy_skia;

 use easy_skia::*;
extern crate gl;
extern crate glutin;

use glutin::dpi::*;
use glutin::GlContext;

fn main() {
    let smaples = 1;
    let stencils = 8;
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, easy skia!")
        .with_dimensions(LogicalSize::new(1024.0, 768.0));
    let context = glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_srgb(true)
        .with_stencil_buffer(8)
        .with_double_buffer(Some(true))
        .with_pixel_format(24, 8)
        .with_multisampling(smaples);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    let mut fobj_id: gl::types::GLint = 0;
    unsafe {
        gl_window.make_current().unwrap();
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);

        gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fobj_id);
    }

    let info = FrameBufferInfo {
        fFBOID: fobj_id as u32,
        fFormat: 0x8058,
    };

    let context = GrContext::make_gl(None);
    let props = SurfaceProps::new(0, easy_skia::PixelGeometry::UNKNOWN_SK_PIXELGEOMETRY);

    let render_target = GrBackendRenderTarget::new_gl(1024, 768, smaples as i32, stencils, &info);
    let surface = Surface::new_backend_render_target(
        &context,
        &render_target,
        SurfaceOrigin::BOTTOM_LEFT_GR_SURFACE_ORIGIN,
        get_default_8888(),
        None,
        &props,
    );
    let mut canvas = surface.get_canvas();
    let mut running = true;
  
    while running {
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => running = false,
                glutin::WindowEvent::Resized(logical_size) => {
                    let dpi_factor = gl_window.get_hidpi_factor();
                    gl_window.resize(logical_size.to_physical(dpi_factor));
                }
                _ => (),
            },
            _ => (),
        });
        draw(&mut canvas);
        canvas.flush();
        gl_window.swap_buffers().unwrap();
    }
}

fn draw(canvas: &mut Canvas) {
  let mut fill = Paint::new();
    fill.set_color(Color::from_argb(255, 255, 255, 255));
    canvas.draw_paint(&fill);

    fill.set_color(Color::from_u32(0xff00ffff));
    canvas.draw_rect(
        &Rect {
            left: 100.0,
            top: 100.0,
            right: 540.0,
            bottom: 380.0,
        },
        &fill,
    );

    let mut stroke = Paint::new();
    stroke.set_color(Color::from_u32(0xff4285F4));
    stroke.set_antialias(true);
    stroke.set_stroke_width(2.);

    let mut path = Path::new();
    let r: f32 = 115.2;
    let c = 128.0f32;
    path.move_to(c + r, c);
    for i in 0..8 {
        let a = 2.6927937 * i as f32;
        path.line_to(c + r * a.cos(), c + r * a.sin());
    }
    canvas.draw_path(&path, &stroke);

    let mut path = Path::new();
    stroke.set_color(Color::from_u32(0xccff0000));
    path.move_to(50., 50.);
    path.line_to(590., 50.);
    path.cubic_to(-490., 50., 1130., 430., 50., 430.);
    path.line_to(590., 430.);
    canvas.draw_path(&path, &stroke);

    let mut text_paint = Paint::new();
    text_paint.set_antialias(true);
    text_paint.set_color(Color::from_u32(0xff000000));

    text_paint.set_dither(true);
    text_paint.set_text_size(64.);

    let root_dir = env!("CARGO_MANIFEST_DIR");
    let typeface =
        Typeface::new_from_file(&format!("{}/examples/fonts/STIX2Math.otf", root_dir), 0).unwrap();
    text_paint.set_typeface(&typeface);
    canvas.draw_text("Hello, Easy Skia!", 100., 100., &text_paint);

    fill.set_color(Color::from_argb(128, 0, 255, 0));
    canvas.draw_oval(
        &Rect {
            left: 120.0,
            top: 120.0,
            right: 520.0,
            bottom: 360.0,
        },
        &fill,
    );
}