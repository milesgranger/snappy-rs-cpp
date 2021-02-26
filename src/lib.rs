use std::ffi::CStr;
use std::os::raw::c_char;

#[cxx::bridge]
mod ffi {

    #[namespace = "snappy"]

    unsafe extern "C++" {
        //include!("snappy-sys/include/snappy.h");
        include!("snappy-sys/include/snappy-sinksource.h");

        type Source;
        type Sink;

        fn Append(self: &Sink, ptr: &CxxString, len: u64);

    }
}


pub fn compress_raw_into(input: &[u8], output: &mut [u8]) {
    let src = ffi::Source();
}


#[cfg(test)]
mod tests {
    use crate::compress_raw_into;

    #[test]
    fn test_compress_raw_into() {
        let input = vec![];
        let mut output = vec![];
        compress_raw_into(&input, output.as_mut_slice());
    }
}