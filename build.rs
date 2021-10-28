use cmake::Config;
//use std::env;
//use std::path::Path;

fn main() {
    // check OS and select cpp compiler
    #[cfg(not(target_family = "unix"))]
    compile_error!("this only supports unix systems");
    #[cfg(target_os = "macos")]
    let cpp_lib = "c++";
    #[cfg(not(target_os = "macos"))]
    let cpp_lib = "stdc++";

    //let dir = env::var("OUT_DIR").unwrap();
    //let dest_path = Path::new(&dir).join("hello.rs");

    // set config for library
    let dst = Config::new(".")
        .define("COORDGEN_BUILD_TESTS", "OFF")
        .define("COORDGEN_BUILD_EXAMPLE", "OFF")
        .define("COORDGEN_BUILD_SHARED_LIBS", "OFF")
        .define("COORDGEN_BUILD_DIR", "/lib")
        .define("CMAKE_BUILD_TYPE", "Release")
        .uses_cxx11()
        .build();
    // creates link to lib
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    // creates link to lib64
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib64").display()
    );

    println!("cargo:rustc-link-lib=static=coordgen");
    println!("cargo:rustc-link-lib=static=wrappedcoordgen");
    println!("cargo:rustc-link-lib={}", cpp_lib);
}
