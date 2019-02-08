extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() {
    assert!(
        Command::new("git")
            .args(&["submodule", "init"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .unwrap()
            .success(),
        "Cannot init submodule"
    );

    assert!(
        Command::new("git")
            .args(&["submodule", "update"])
            .status()
            .unwrap()
            .success(),
        "Cannot update submodule"
    );

    let out_dir = env::var("OUT_DIR").unwrap();

    let skia_out_dir = format!("{}/skia", &out_dir);

    assert!(
        Command::new("python")
            .current_dir("skia")
            .args(&["tools/git-sync-deps"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .unwrap()
            .success(),
        "Cannot download skia depenedencies"
    );

    assert!(Command::new("bin/gn")
                .current_dir("skia")
                .args(&["gen", &skia_out_dir, "--args=is_official_build=true skia_use_system_expat=false skia_use_system_icu=false skia_use_system_libjpeg_turbo=false skia_use_system_libpng=false skia_use_system_libwebp=false skia_use_system_zlib=false skia_use_system_freetype2=false cc=\"clang\" cxx=\"clang++\""])
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status().unwrap().success(), "Cannot generate build files");

    assert!(
        Command::new("ninja")
            .current_dir("skia")
            .args(&["-C", &skia_out_dir])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .unwrap()
            .success(),
        "Cannot build skia"
    );

    println!("cargo:rustc-link-search=native={}", skia_out_dir);
    println!("cargo:rustc-link-lib=static=skia");

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        println!("cargo:rustc-link-lib=framework=CoreText");
        println!("cargo:rustc-link-lib=framework=CoreServices");
        println!("cargo:rustc-link-lib=dylib=c++");
    }

    #[cfg(target_os = "linux")]
    {
        assert!(
            Command::new("sh")
                .current_dir("skia")
                .args(&["tools/install_dependencies.sh"])
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .unwrap()
                .success(),
            "Cannot install skia depenedencies"
        );
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=fontconfig");
    }

    let bindings = bindgen::Builder::default()
        .rustified_enum("*")
        .header("./sk_includes.h")
        .rustfmt_bindings(true)
        .blacklist_type("max_align_t")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
