fn main() {
  println!("cargo:rerun-if-changed=src/woff2_c.cpp");
  println!("cargo:rerun-if-changed=src/woff2_c.hpp");

  let compile_target = std::env::var("TARGET").expect("TARGET");
  let compile_target_os = std::env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS");
  let compile_target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH");

  match compile_target_os.as_str() {
    "windows" => {
      std::env::set_var("CC", "clang-cl");
      std::env::set_var("CXX", "clang-cl");
    }
    "wasm32-wasip1-threads" => {}
    _ => {
      std::env::set_var("CC", "clang");
      std::env::set_var("CXX", "clang++");
    }
  }

  if compile_target_os == "macos" || compile_target_os == "wasi" {
    println!("cargo:rustc-link-lib=brotli_ffi");
  } else {
    println!("cargo:rustc-link-lib=static=brotli_ffi");
  }
  println!("cargo:rustc-link-search=rust-brotli/target/{compile_target}/release");

  let mut builder = cc::Build::new();
  builder
    .cpp(true)
    .include("rust-brotli")
    .include("woff2/include")
    .files([
      "woff2/src/variable_length.cc",
      "woff2/src/table_tags.cc",
      "woff2/src/font.cc",
      "woff2/src/glyph.cc",
      "woff2/src/normalize.cc",
      "woff2/src/woff2_common.cc",
      "woff2/src/transform.cc",
      "woff2/src/woff2_out.cc",
      "woff2/src/woff2_enc.cc",
      "woff2/src/woff2_dec.cc",
    ]);

  if compile_target.contains("windows") {
    builder
      .flag("/std:c++17")
      .flag("-Wno-unused-function")
      .flag("-Wno-unused-parameter")
      .static_crt(true);
  } else {
    builder
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
      .flag("-Wno-unused-parameter");
  }

  match compile_target.as_str() {
    "aarch64-unknown-linux-musl" => {
      builder
        .static_flag(true)
        .compiler("clang++")
        .include("/usr/lib/llvm-18/include/c++/v1");
    }
    "x86_64-unknown-linux-gnu" | "aarch64-unknown-linux-gnu" => {
      builder.include("/usr/lib/llvm-18/include/c++/v1");
    }
    "x86_64-unknown-linux-musl" => {
      builder
        .static_flag(true)
        .compiler("clang++")
        .include("/usr/lib/llvm-18/include/c++/v1");
    }
    "wasm32-wasip1-threads" => {
      builder.cpp_set_stdlib("c++");
      if let Ok(sdk) = std::env::var("WASI_SDK_PATH") {
        builder
          .compiler(&format!("{sdk}/bin/clang++"))
          .flag("-nostdlib");
        println!("cargo:rustc-link-search={sdk}/share/wasi-sysroot/lib/wasm32-wasi-threads");
        println!("cargo:rustc-link-lib=static=c++");
        println!("cargo:rustc-link-lib=static=c++abi");
      }
    }
    _ => {}
  }

  if compile_target_os == "linux" {
    match compile_target_arch.as_str() {
      "aarch64" | "x86_64" => {
        builder.cpp_set_stdlib("c++").flag("-static");
        println!("cargo:rustc-link-lib=static=c++");
        println!("cargo:rustc-link-search=/usr/lib/llvm-18/lib");
      }
      _ => {}
    }
  }

  builder.file("src/woff2_c.cpp").compile("woff2_encode");

  napi_build::setup();
}
