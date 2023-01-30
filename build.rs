extern crate napi_build;

fn main() {
  let compile_target = std::env::var("TARGET").expect("TARGET");
  assert!(std::process::Command::new("cargo")
    .arg("build")
    .arg("--release")
    .arg("--target")
    .arg(&compile_target)
    .current_dir("brotli/c")
    .status()
    .map(|s| s.success())
    .unwrap_or(false));

  println!("cargo:rustc-link-lib=static=brotli_ffi");
  println!(
    "cargo:rustc-link-search=brotli/c/target/{}/release",
    compile_target
  );

  let mut builder = cc::Build::new();
  builder
    .cpp(true)
    .include("brotli/c")
    .include("woff2/include")
    .files([
      "woff2/src/variable_length.cc",
      "woff2/src/table_tags.cc",
      "woff2/src/font.cc",
      "woff2/src/glyph.cc",
      "woff2/src/normalize.cc",
      "woff2/src/woff2_common.cc",
      "woff2/src/transform.cc",
      "woff2/src/woff2_enc.cc",
    ])
    .flag("-std=c++17")
    .flag("-fno-omit-frame-pointer")
    .flag("-fPIC")
    .flag("-fno-exceptions")
    .flag("-fno-rtti")
    .flag("-fstrict-aliasing")
    .flag("-fvisibility=hidden")
    .flag("-fvisibility-inlines-hidden")
    .flag("-fdata-sections")
    .flag("-ffunction-sections")
    .flag("-Wno-unused-function")
    .flag("-Wno-unused-parameter")
    .cargo_metadata(true);
  builder.compile("woff2");

  let mut builder = cc::Build::new();
  builder
    .cpp(true)
    .include("woff2/include")
    .file("src/encode.cpp")
    .compile("woff2_encode");

  napi_build::setup();
}
