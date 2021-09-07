use cmake::Config;

fn main() {
    #[cfg(not(target_family = "unix"))]
    compile_error!("this only supports unix systems");

    let dst = Config::new("coordgenlibs")
        .define("COORDGEN_BUILD_TESTS", "OFF")
        .define("COORDGEN_BUILD_EXAMPLE", "OFF")
        .define("COORDGEN_BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_BUILD_TYPE", "Release")
        .build();

    let lib_dir = dst.join("lib");
    let inc_dir = dst.join("include");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=coordgen");

    cc::Build::new()
        .file("wrapper/get_coordinates.cpp")
        .cpp(true)
        // this won't work for MSVC but we don't support windows
        .opt_level(3)
        .includes(vec!["wrapper".into(), inc_dir])
        .compile("wrappedcoordgen");
}
