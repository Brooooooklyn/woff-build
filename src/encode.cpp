#include "encode.hpp"

extern "C"
{
  size_t MaxWOFF2CompressedSize(const uint8_t *data, size_t length)
  {
    return woff2::MaxWOFF2CompressedSize(data, length);
  }

  bool ConvertTTFToWOFF2(const uint8_t *data, size_t length,
                         uint8_t *result, size_t *result_length)
  {
    return woff2::ConvertTTFToWOFF2(data, length, result, result_length);
  }
}