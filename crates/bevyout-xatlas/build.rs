fn main() {
    println!("cargo:rerun-if-changed=vendor/xatlas/xatlas.cpp");
    println!("cargo:rerun-if-changed=vendor/xatlas/xatlas.h");
    println!("cargo:rerun-if-changed=src/bridge.cpp");

    cc::Build::new()
        .cpp(true)
        .file("vendor/xatlas/xatlas.cpp")
        .file("src/bridge.cpp")
        .include("vendor/xatlas")
        .flag_if_supported("-std=c++11")
        .warnings(false)
        .compile("bevyout_xatlas");
}
