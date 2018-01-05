extern crate bindgen;
extern crate cmake;

use std::path::PathBuf;

fn generate_bncsutil_bindings() {
    let bindings = bindgen::Builder::default()
        .header("bncsutil/src/bncsutil/bncsutil.h")
        .clang_arg("-I./bncsutil/src")
        .whitelisted_function("kd.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src/");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn compile_bncsutil() {
    let dst = cmake::Config::new("bncsutil/")
        .define("CMAKE_MODULE_PATH", format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/bncsutil/CMake/Modules"))
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=bncsutil");
}

fn main() {
    generate_bncsutil_bindings();
    compile_bncsutil();
}