fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/RustCApi.cc")
        .flag("-I")
        .flag_if_supported("-std=c++14")
        .compile(&format!("{}", env!("CARGO_PKG_NAME")));
}