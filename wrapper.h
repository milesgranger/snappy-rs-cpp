
#include <snappy.h>
#include <snappy-sinksource.h>
#include <rust/cxx.h>

namespace wrapper {

    size_t compress_raw_into(rust::Slice<const uint8_t> input, rust::Slice<uint8_t> output) {

        size_t in_len = input.length();
        const char* in_data = reinterpret_cast<const char*>(input.data());

        size_t n_bytes;
        char* out_data = reinterpret_cast<char*>(output.data());

        snappy::RawCompress(in_data, in_len, out_data, &n_bytes);
        return n_bytes;
    }

}
