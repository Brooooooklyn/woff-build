#![deny(clippy::all)]

use napi::{bindgen_prelude::*, JsBuffer};

#[macro_use]
extern crate napi_derive;

mod woff2 {
  extern "C" {
    pub fn MaxWOFF2CompressedSize(data: *const u8, length: usize) -> usize;
    pub fn ConvertTTFToWOFF2(
      data: *const u8,
      length: usize,
      result: *mut u8,
      result_length: *mut usize,
    ) -> bool;
  }
}

#[napi]
pub fn convert_ttf_to_woff2(input: JsBuffer) -> Result<Buffer> {
  let input_buf_value = input.into_value()?;
  let len =
    unsafe { woff2::MaxWOFF2CompressedSize(input_buf_value.as_ptr(), input_buf_value.len()) };
  let mut output_buf = Vec::with_capacity(len);
  let mut output_buf_len = len;
  let ok = unsafe {
    woff2::ConvertTTFToWOFF2(
      input_buf_value.as_ptr(),
      input_buf_value.len(),
      output_buf.as_mut_ptr(),
      &mut output_buf_len as *mut usize,
    )
  };
  if !ok {
    return Err(Error::new(
      Status::GenericFailure,
      "ConvertTTFToWOFF2 failed".to_owned(),
    ));
  }
  Ok(output_buf.into())
}
