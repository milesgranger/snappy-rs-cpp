fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("include/snappy.cc")
        .file("include/snappy-sinksource.cc")
        .compile("snappy-sys");
}