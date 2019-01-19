use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let skia_out_dir = format!("{}/skia", &out_dir);

    assert!(Command::new("python")
                .current_dir("skia")
                .args(&["tools/git-sync-deps"])
                .status().unwrap().success(), "Cannot download skia depenedencies");

    assert!(Command::new("bin/gn")
                .current_dir("skia")
                .args(&["gen", &skia_out_dir, "--args=is_official_build=true skia_use_system_expat=false skia_use_system_icu=false skia_use_system_libjpeg_turbo=false skia_use_system_libpng=false skia_use_system_libwebp=false skia_use_system_zlib=false"])
                .status().unwrap().success(), "Cannot generate build files");

    assert!(Command::new("ninja")
                .current_dir("skia")
                .args(&["-C", &skia_out_dir])
                .status().unwrap().success(), "Cannot build skia");

    println!("cargo:rustc-link-search=native={}", skia_out_dir);
    println!("cargo:rustc-link-lib=static=skia");
    println!("cargo:rustc-link-lib=dylib=c++");

    #[cfg(target_os="macos")]
    {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        println!("cargo:rustc-link-lib=framework=CoreText");
        println!("cargo:rustc-link-lib=framework=CoreServices");
    }
}