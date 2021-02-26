
#include <rust/cxx.h>



namespace wrapper {

#ifdef __cplusplus
    extern "C" {
#endif
#include <snappy-c.h>
#ifdef __cplusplus
    }  // extern "C"
#endif

    size_t max_compressed_length(size_t source_length) {
        return snappy_max_compressed_length(source_length);
    }

    size_t compress_raw_into(rust::Slice<const uint8_t> input, rust::Slice <uint8_t> output) {

        size_t in_len = input.length();
        const char *in_data = reinterpret_cast<const char *>(input.data());

        size_t n_bytes;
        char *out_data = reinterpret_cast<char *>(output.data());

        snappy_compress(in_data, in_len, out_data, &n_bytes);
        return n_bytes;
    }
}



