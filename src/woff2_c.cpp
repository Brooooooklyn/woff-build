#include "woff2_c.hpp"

extern "C"
{
  size_t MaxWOFF2CompressedSize(const uint8_t *data, size_t length)
  {
    return woff2::MaxWOFF2CompressedSize(data, length);
  }

  bool ConvertTTFToWOFF2(const uint8_t *data, size_t length,
                         uint8_t *result, size_t *result_length, Woff2EncodeParams params)
  {
    if (!data || !length || !result || !result_length) {
      return false;
    }
    woff2::WOFF2Params woff2_params;
    woff2_params.allow_transforms = params.allow_transforms;
    woff2_params.brotli_quality = params.brotli_quality;
    if (params.extended_metadata) {
      woff2_params.extended_metadata = std::string(params.extended_metadata);
    }
    return woff2::ConvertTTFToWOFF2(data, length, result, result_length, woff2_params);
  }

  bool ConvertWOFF2ToTTF(
      const uint8_t *data, size_t length,
      uint8_t *out_buffer, size_t out_buffer_length,Woff2MemoryOut *out)
  {
    if (!data || !length || !out) {
      return false;
    }

    auto memory_out = new woff2::WOFF2MemoryOut(out_buffer, out_buffer_length);
    out->inner = reinterpret_cast<Woff2MemoryOutInner *>(memory_out);
    out->data = out_buffer;
    out->length = out_buffer_length;
    return woff2::ConvertWOFF2ToTTF(data, length, memory_out);
  }

  void FreeMemoryOutput(Woff2MemoryOutInner *out)
  {
    delete reinterpret_cast<woff2::WOFF2MemoryOut *>(out);
  }
}
