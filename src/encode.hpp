#ifndef WOFF2_CAPI_H
#define WOFF2_CAPI_H

#include <stddef.h>
#include <inttypes.h>

#include <woff2/encode.h>

extern "C"
{
  size_t MaxWOFF2CompressedSize(const uint8_t *data, size_t length);
  bool ConvertTTFToWOFF2(const uint8_t *data, size_t length,
                         uint8_t *result, size_t *result_length);
}

#endif // WOFF2_CAPI_H
