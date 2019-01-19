extern crate easy_skia;

pub use easy_skia::*;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut cs = ColorSpace::new();
    let image_info = ImageInfo {
        colorspace: cs.raw_pointer(),
        width: 640,
        height: 840,
        colorType: ColorType::RGBA_8888_SK_COLORTYPE,
        alphaType: AlphaType::PREMUL_SK_ALPHATYPE,
    };
    let options = SurfaceProps::new(0, easy_skia::PixelGeometry::UNKNOWN_SK_PIXELGEOMETRY);
    let surface = Surface::new_raster(&image_info, 0, &Some(options));
    let mut canvas = surface.get_canvas();

    let mut fill = Paint::new();
    fill.set_color(Color::fromu32(0xffffff));
    canvas.draw_paint(&fill);

    fill.set_color(Color::fromu32(0xff00ffff));
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
    stroke.set_color(Color::fromu32(0xffff0000));
    stroke.set_antialias(true);
    stroke.set_stroke_width(5.);

    let mut path = Path::new();
    path.move_to(50., 50.);
    path.line_to(590., 50.);
    path.cubic_to(-490., 50., 1130., 430., 50., 430.);
    path.line_to(590., 430.);
    canvas.draw_path(&path, &stroke);

    let mut text_paint = Paint::new();
    text_paint.set_antialias(true);
    text_paint.set_color(Color::fromu32(0xff00000));
    text_paint.set_dither(true);
    text_paint.set_text_size(64.);

    let root_dir = env!("CARGO_MANIFEST_DIR");
    let typeface =
        Typeface::new_from_file(&format!("{}/examples/fonts/STIX2Math.otf", root_dir), 0).unwrap();
    text_paint.set_typeface(&typeface);

    let metrics = text_paint.get_font_metrics(0.);
    println!("{:?}", metrics);
    println!("{:?}", text_paint.measure_text("Hello world!"));
    println!("{:?}", text_paint.get_text_encoding());

    text_paint.set_text_encoding(easy_skia::TextEncoding::GLYPH_ID_SK_TEXT_ENCODING);
    println!("{:?}", text_paint.measure_blob(&[19, 19]));

    text_paint.set_text_encoding(TextEncoding::UTF8_SK_TEXT_ENCODING);
    println!("{:?}", text_paint.measure_text("QQ"));

    canvas.draw_text("Hello", 100., 100., &text_paint);

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

    let snapshot = surface.new_image_snapshot();
    let data = snapshot.encode();

    let bytes = data.get_data();

    let mut file = File::create("target/safe.png").expect("Cannot create file");
    file.write(bytes).expect("Cannot write to file");
}

#[test]
fn test_skia_bridge() {
    let mut text_paint = Paint::new();
    text_paint.set_antialias(true);
    text_paint.set_color(&Color {
        a: 255,
        r: 0,
        g: 0,
        b: 0,
    });
    text_paint.set_dither(true);
    text_paint.set_text_size(64.);

    let root_dir = env!("CARGO_MANIFEST_DIR");
    let typeface =
        Typeface::new_from_file(&format!("{}/examples/fonts/STIX2Math.otf", root_dir), 0).unwrap();
    text_paint.set_typeface(&typeface);

    let metrics = text_paint.get_font_metrics(0.);
}
