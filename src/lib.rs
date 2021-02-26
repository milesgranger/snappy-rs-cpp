#![allow(unused_unsafe)]  // IDE doesn't recognize when calls to cxx are actually safe.

#[cxx::bridge]
mod ffi {

    #[namespace = "snappy"]
    unsafe extern "C++" {
        include!("snappy-sys/include/snappy.h");
        pub fn MaxCompressedLength(input_len: usize) -> usize;
    }

    #[namespace = "wrapper"]
    unsafe extern "C++" {
        include!("snappy-sys/wrapper.h");

        pub fn compress_raw_into(input: &[u8], output: &mut [u8]) -> usize;
    }


}


pub fn max_compressed_len(input_len: usize) -> usize {
    unsafe { ffi::MaxCompressedLength(input_len) }
}


pub fn compress_raw_into(input: &[u8], output: &mut [u8]) -> usize {
    if input.len() > 0 {
        unsafe { ffi::compress_raw_into(input, output) }
    } else {
        0
    }
}


#[cfg(test)]
mod tests {
    use crate::{compress_raw_into, max_compressed_len};

    #[test]
    fn test_max_compressed_len() {
        let size = max_compressed_len(10);
        assert!(size > 0);
    }

    #[test]
    fn test_compress_raw_into() {
        let input = b"some bytes here".to_vec();
        let mut output = vec![0; max_compressed_len(input.len())];
        let n_bytes = compress_raw_into(&input, output.as_mut_slice());
        println!("{:?}", String::from_utf8(output[..n_bytes].to_vec()));
    }
}