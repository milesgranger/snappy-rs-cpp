
#[cxx::bridge]
mod ffi {

    #[namespace = "snappy"]
    unsafe extern "C++" {
        include!("snappy-sys/include/snappy.h");
    }

    #[namespace = "wrapper"]
    unsafe extern "C++" {
        include!("snappy-sys/include/wrapper.h");

        fn compress_raw_into(input: &[u8], output: &mut [u8]) -> usize;
    }


}


pub fn compress_raw_into(input: &[u8], output: &mut [u8]) -> usize {
    if input.len() > 0 {
        ffi::compress_raw_into(input, output)
    } else {
        0
    }
}


#[cfg(test)]
mod tests {
    use crate::compress_raw_into;

    #[test]
    fn test_compress_raw_into() {
        let input = b"some bytes here".to_vec();
        let mut output = vec![0;500];
        let n_bytes = compress_raw_into(&input, output.as_mut_slice());
        println!("{:?}", String::from_utf8(output[..n_bytes].to_vec()));
    }
}