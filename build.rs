use cmake::Config;

fn out_dir(){
    
}

fn main() {
    // check OS and select cpp compiler
    #[cfg(not(target_family = "unix"))]
    compile_error!("this only supports unix systems");
    #[cfg(target_os = "macos")]
    let cpp_lib = "c++";
    #[cfg(not(target_os = "macos"))]
    let cpp_lib = "stdc++";

    let dst = Config::new(".")
        .define("COORDGEN_BUILD_TESTS", "OFF")
        .define("COORDGEN_BUILD_EXAMPLE", "OFF")
        .define("COORDGEN_BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_BUILD_TYPE", "Release")
        .uses_cxx11()
        .build();
    // in either of two directories
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib64").display()
    );
    //link libraries
    println!("cargo:rustc-link-lib=static=coordgen");
    println!("cargo:rustc-link-lib=static=wrappedcoordgen");
    println!("cargo:rustc-link-lib={}", cpp_lib);
}