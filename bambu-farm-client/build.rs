use std::fs::read_dir;

fn main() {
    // cc::Build::new()
    //     .file("cpp/api.cpp")
    //     .include("cpp")
    //     .link_lib_modifier("+whole-archive")
    //     .flag("-fvisibility=default")
    //     .flag("-export-all-symbols")
    //     .compile("bambu_networking_api");

    for entry in read_dir("cpp").unwrap() {
        println!(
            "cargo:rerun-if-changed=cpp/{}",
            entry.unwrap().file_name().to_str().unwrap()
        );
    }

    cxx_build::bridge("src/api.rs")
        .file("cpp/api.cpp")
        .include("cpp")
        .include("/opt/homebrew/include")
        .flag_if_supported("-std=c++11")
        .flag_if_supported("-Wno-unused-parameter")
        .flag("-DBOOST_ALL_NO_LIB")  // Disable auto-linking of Boost libraries
        .flag("-DBOOST_THREAD_USE_LIB")  // Specific Boost library flags if needed
        // .link_lib_modifier("+whole-archive")
        .flag_if_supported("-arch arm64")
        .compile("bambu_networking_api");

    tonic_build::compile_protos("../bambu-farm-server/proto/service.proto").unwrap();
    println!("cargo:rerun-if-changed=../bambu-farm-server/proto/service.proto");
    println!("cargo:rerun-if-changed=build.rs");
}
