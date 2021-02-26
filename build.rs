fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("include/snappy.cc")
        .file("include/snappy-sinksource.cc")
        .file("include/snappy-c.cc")
        .compile("snappy-sys");
}