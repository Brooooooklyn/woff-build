#![deny(clippy::all)]

use std::ffi::CString;

use napi::{bindgen_prelude::*, JsBuffer};
use woff2::Woff2MemoryOut;

#[macro_use]
extern crate napi_derive;

mod woff2 {
  use std::ffi::c_char;

  #[repr(C)]
  pub struct Woff2MemoryOutInner {
    _unused: [u8; 0],
  }

  #[repr(C)]
  pub struct Woff2MemoryOut {
    pub data: *const u8,
    pub length: usize,
    inner: *mut Woff2MemoryOutInner,
  }

  #[repr(C)]
  pub struct Woff2EncodeParams {
    pub extended_metadata: *const c_char,
    pub brotli_quality: i32,
    pub allow_transforms: bool,
  }

  impl Default for Woff2EncodeParams {
    fn default() -> Self {
      Self {
        extended_metadata: std::ptr::null(),
        brotli_quality: 11,
        allow_transforms: true,
      }
    }
  }

  unsafe impl Send for Woff2MemoryOut {}

  impl Drop for Woff2MemoryOut {
    fn drop(&mut self) {
      if !self.inner.is_null() {
        unsafe {
          // SAFETY: if self.inner is not null, it is a valid pointer
          // and should be created by `ConvertWOFF2ToTTF`, which can
          // be dealloced by `FreeMemoryOutput` safely
          FreeMemoryOutput(self.inner);
        }
      }
      unsafe {
        // SAFETY: WE KNOW self.data IS LEAKED FROM A VALID VEC
        // NOTE(CGQAQ): implicit drop here will free the memory
        Vec::from_raw_parts(self.data as *mut u8, self.length, self.length);
      }
    }
  }

  extern "C" {
    pub fn MaxWOFF2CompressedSize(data: *const u8, length: usize) -> usize;
    pub fn ComputeWOFF2FinalSize(data: *const u8, length: usize) -> usize;
    pub fn ConvertTTFToWOFF2(
      data: *const u8,
      length: usize,
      result: *mut u8,
      result_length: *mut usize,
      params: Woff2EncodeParams,
    ) -> bool;
    pub fn ConvertWOFF2ToTTF(
      data: *const u8,
      length: usize,
      out_buffer: *const u8,
      out_len: usize,
      out: *mut Woff2MemoryOut,
    ) -> bool;
    pub fn FreeMemoryOutput(out: *mut Woff2MemoryOutInner);
  }
}

// `#[inline]` always causes llvm codegen error on i686-pc-windows-msvc
#[cfg_attr(not(target_arch = "x86"), inline)]
fn convert_to_woff2(input_buf_value: &[u8], params: woff2::Woff2EncodeParams) -> Result<Vec<u8>> {
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
      params,
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

// `#[inline]` always causes llvm codegen error on i686-pc-windows-msvc
#[cfg_attr(not(target_arch = "x86"), inline)]
fn convert_to_ttf(input_buf_value: &[u8]) -> Result<Woff2MemoryOut> {
  let mut output = std::mem::MaybeUninit::<woff2::Woff2MemoryOut>::uninit();
  // the actually size could be larger than the `result_length`, give it a little more space
  let out_buffer_len =
    unsafe { woff2::ComputeWOFF2FinalSize(input_buf_value.as_ptr(), input_buf_value.len()) + 4096 };

  if out_buffer_len == 0 {
    return Err(Error::new(
      Status::InvalidArg,
      "Final result is zero sized".to_owned(),
    ));
  }

  let out_buffer = Vec::leak(Vec::with_capacity(out_buffer_len)).as_mut_ptr();

  if !unsafe {
    woff2::ConvertWOFF2ToTTF(
      input_buf_value.as_ptr(),
      input_buf_value.len(),
      out_buffer,
      out_buffer_len,
      output.as_mut_ptr(),
    )
  } {
    return Err(Error::new(
      Status::GenericFailure,
      "ConvertWOFF2ToTTF failed".to_owned(),
    ));
  }
  Ok(unsafe { output.assume_init() })
}

#[napi(js_name = "convertTTFToWOFF2")]
pub fn convert_ttf_to_woff2(input: JsBuffer, params: Option<Woff2Params>) -> Result<Buffer> {
  let input_buf_value = input.into_value()?;

  Ok(convert_to_woff2(input_buf_value.as_ref(), (&params).into())?.into())
}

pub struct ConvertTTFToWOFF2Task {
  input: Buffer,
  params: Option<Woff2Params>,
}

#[napi]
impl Task for ConvertTTFToWOFF2Task {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    convert_to_woff2(self.input.as_ref(), (&self.params).into())
  }
  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }
}

#[napi(object)]
#[derive(Clone, Debug)]
pub struct Woff2Params {
  pub extended_metadata: Option<String>,
  pub brotli_quality: Option<u8>,
  pub allow_transforms: Option<bool>,
}

impl From<&Option<Woff2Params>> for woff2::Woff2EncodeParams {
  fn from(params: &Option<Woff2Params>) -> Self {
    if let Some(params) = params {
      Self {
        extended_metadata: CString::new(
          params
            .extended_metadata
            .as_ref()
            .map(|s| s.clone())
            .unwrap_or_else(String::new),
        )
        .unwrap()
        .into_raw(),
        brotli_quality: params.brotli_quality.unwrap_or(11) as i32,
        allow_transforms: params.allow_transforms.unwrap_or(true),
      }
    } else {
      Self::default()
    }
  }
}

#[napi(js_name = "convertTTFToWOFF2Async")]
pub fn convert_ttf_to_woff2_async(
  input: Buffer,
  params: Option<Woff2Params>,
  signal: Option<AbortSignal>,
) -> AsyncTask<ConvertTTFToWOFF2Task> {
  AsyncTask::with_optional_signal(ConvertTTFToWOFF2Task { input, params }, signal)
}

pub struct ConvertWOFF2ToTTFTask {
  input: Buffer,
}

#[napi]
impl Task for ConvertWOFF2ToTTFTask {
  type Output = Woff2MemoryOut;
  type JsValue = JsBuffer;

  fn compute(&mut self) -> Result<Self::Output> {
    convert_to_ttf(self.input.as_ref())
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    unsafe {
      env.create_buffer_with_borrowed_data(output.data, output.length, output, |h, _| drop(h))
    }
    .map(|b| b.into_raw())
  }
}

#[napi(js_name = "convertWOFF2ToTTF")]
pub fn convert_woff2_to_ttf(env: Env, input: JsBuffer) -> Result<JsBuffer> {
  let input_buf_value = input.into_value()?;
  let o = convert_to_ttf(input_buf_value.as_ref())?;
  unsafe { env.create_buffer_with_borrowed_data(o.data, o.length, o, |h, _| drop(h)) }
    .map(|b| b.into_raw())
}

#[napi(js_name = "convertWOFF2ToTTFAsync")]
pub fn convert_woff2_to_ttf_async(
  input: Buffer,
  signal: Option<AbortSignal>,
) -> AsyncTask<ConvertWOFF2ToTTFTask> {
  AsyncTask::with_optional_signal(ConvertWOFF2ToTTFTask { input }, signal)
}
