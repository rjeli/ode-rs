use std::env;
use std::path::PathBuf;

fn main() {
    let upstream_dst = cmake::Config::new("upstream")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_DISABLE_FIND_PACKAGE_OpenGL", "ON")
        .define("ODE_WITH_DEMOS", "OFF")
        .define("ODE_DOUBLE_PRECISION", "ON")
        .define("ODE_NO_BUILTIN_THREADING_IMPL", "ON")
        .define("ODE_NO_THREADING_INTF", "ON")
        .define("CMAKE_VERBOSE_MAKEFILE", "ON")
        .build();

    println!("upstream_dst: {:?}", upstream_dst);

    println!(
        "cargo:rustc-link-search=native={}/lib",
        upstream_dst.display()
    );
    println!("cargo:rustc-link-lib=static=ode");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}/include", upstream_dst.display()))
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings");
}
