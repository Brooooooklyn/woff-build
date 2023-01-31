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

// `#[inline]` always causes llvm codegen error on i686-pc-windows-msvc
#[cfg_attr(not(target_arch = "x86"), inline)]
fn convert(input_buf_value: &[u8]) -> Result<Vec<u8>> {
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
  unsafe { output_buf.set_len(output_buf_len) };
  Ok(output_buf)
}

#[napi(js_name = "convertTTFToWOFF2")]
pub fn convert_ttf_to_woff2(input: JsBuffer) -> Result<Buffer> {
  let input_buf_value = input.into_value()?;

  Ok(convert(input_buf_value.as_ref())?.into())
}

pub struct ConvertTTFToWOFF2Task {
  input: Buffer,
}

#[napi]
impl Task for ConvertTTFToWOFF2Task {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    convert(self.input.as_ref())
  }
  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }
}

#[napi(js_name = "convertTTFToWOFF2Async")]
pub fn convert_ttf_to_woff2_async(
  input: Buffer,
  signal: Option<AbortSignal>,
) -> AsyncTask<ConvertTTFToWOFF2Task> {
  AsyncTask::with_optional_signal(ConvertTTFToWOFF2Task { input }, signal)
}
