use cmake::Config;

fn main() {
    // check OS and select cpp compiler
    #[cfg(not(target_family = "unix"))]
    compile_error!("this only supports unix systems");
    #[cfg(target_os = "macos")]
    let cpp_lib = "c++";
    #[cfg(not(target_os = "macos"))]
    let cpp_lib = "stdc++";

    //let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    //println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("lib").display());


    // configure and create CMakeFiles
    let dst = Config::new(".")
        .define("COORDGEN_BUILD_TESTS", "OFF")
        .define("COORDGEN_BUILD_EXAMPLE", "OFF")
        .define("COORDGEN_BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .uses_cxx11()
        .build();
    
    //need to hardcode make --path="${prefix}/lib"
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );

    //link libraries to 
    println!("cargo:rustc-link-lib=static=coordgen");
    println!("cargo:rustc-link-lib=static=wrappedcoordgen");
    println!("cargo:rustc-link-lib={}", cpp_lib);
}
/*
        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("lib64").display()
        );*/
        //.define("CMAKE_LIBRARY_OUTPUT_DIRECTORY", "/lib")
        //.build_target("target/lib")

        //let dst = cmake::build("./");

            //let path = PathBuf::from("target/lib");
    //assert_eq!(Path::new("target/lib"), path.as_path());