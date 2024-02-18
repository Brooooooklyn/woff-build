fn main() {
  println!("cargo:rerun-if-changed=src/woff2_c.cpp");
  println!("cargo:rerun-if-changed=src/woff2_c.hpp");

  let compile_target = std::env::var("TARGET").expect("TARGET");
  let compile_target_os = std::env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS");
  let compile_target_env = std::env::var("CARGO_CFG_TARGET_ENV").expect("CARGO_CFG_TARGET_ENV");
  let compile_target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH");

  match compile_target_os.as_str() {
    "windows" => {
      std::env::set_var("CC", "clang-cl");
      std::env::set_var("CXX", "clang-cl");
    }
    "wasm32-wasi-preview1-threads" => {}
    _ => {
      std::env::set_var("CC", "clang");
      std::env::set_var("CXX", "clang++");
    }
  }

  let prefix = if compile_target.contains("windows") {
    ""
  } else {
    "lib"
  };

  let ext = if compile_target.contains("windows") {
    "dll"
  } else if compile_target.contains("apple") {
    "dylib"
  } else {
    "so"
  };

  std::fs::remove_file(format!(
    "brotli/c/target/release/{compile_target}/{prefix}brotli_ffi.{ext}"
  ))
  .unwrap_or(());

  if compile_target_os == "macos" {
    println!("cargo:rustc-link-lib=brotli_ffi");
  } else {
    println!("cargo:rustc-link-lib=static=brotli_ffi");
  }
  println!("cargo:rustc-link-search=brotli/c/target/{compile_target}/release");

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
      let gcc_version = String::from_utf8(
        std::process::Command::new("ls")
          .arg("/aarch64-linux-musl-cross/aarch64-linux-musl/include/c++")
          .output()
          .unwrap()
          .stdout,
      )
      .unwrap();
      let gcc_version_trim = gcc_version.trim();
      builder
        .include("/aarch64-linux-musl-cross/aarch64-linux-musl/include")
        .include(format!(
          "/aarch64-linux-musl-cross/aarch64-linux-musl/include/c++/{gcc_version_trim}"
        ))
        .include(format!(
          "/aarch64-linux-musl-cross/aarch64-linux-musl/include/c++/{gcc_version_trim}/aarch64-linux-musl"
        ));
    }
    "x86_64-unknown-linux-musl" => {
      let gcc_version = String::from_utf8(
        std::process::Command::new("ls")
          .arg("/usr/include/c++")
          .output()
          .unwrap()
          .stdout,
      )
      .unwrap();
      let gcc_version_trim = gcc_version.trim();
      builder
        .static_flag(true)
        .include("/usr/include")
        .include(format!("/usr/include/c++/{gcc_version_trim}"))
        .include(format!(
          "/usr/include/c++/{gcc_version_trim}/x86_64-alpine-linux-musl"
        ));
    }
    "wasm32-wasi-preview1-threads" => {
      builder.cpp_set_stdlib("c++");
      if let Ok(sdk) = std::env::var("WASI_SDK_PATH") {
        builder
          .compiler(&format!("{sdk}/bin/clang++"))
          .flag("-nostdlib")
          .archiver(&format!("{sdk}/bin/llvm-ar"));
        println!("cargo:rustc-link-search={sdk}/share/wasi-sysroot/lib/wasm32-wasi-threads");
        println!("cargo:rustc-link-lib=static=c++");
        println!("cargo:rustc-link-lib=static=c++abi");
      }
    }
    _ => {}
  }

  if compile_target_os == "linux" {
    if compile_target_env != "gnu" {
      builder.cpp_set_stdlib("stdc++");
    } else {
      builder.cpp_set_stdlib("c++").flag("-static");
      println!("cargo:rustc-link-lib=static=c++");
      match compile_target_arch.as_str() {
        "aarch64" => {
          builder
            .include("/usr/aarch64-unknown-linux-gnu/aarch64-unknown-linux-gnu/sysroot/usr/include")
            .flag("--sysroot=/usr/aarch64-unknown-linux-gnu/aarch64-unknown-linux-gnu/sysroot");
          println!("cargo:rustc-link-search=/usr/aarch64-unknown-linux-gnu/lib/llvm-18/lib");
          println!("cargo:rustc-link-search=/usr/aarch64-unknown-linux-gnu/lib");
          println!("cargo:rustc-link-search=/usr/aarch64-unknown-linux-gnu/aarch64-unknown-linux-gnu/sysroot/lib");
          println!("cargo:rustc-link-search=/usr/aarch64-unknown-linux-gnu/lib/gcc/aarch64-unknown-linux-gnu/4.8.5");
        }
        "x86_64" => {
          builder.include("/usr/lib/llvm-18/include/c++/v1");
          println!("cargo:rustc-link-search=/usr/lib/llvm-18/lib");
        }
        "arm" => {
          let gcc_version = String::from_utf8(
            std::process::Command::new("ls")
              .arg("/usr/arm-linux-gnueabihf/include/c++")
              .output()
              .unwrap()
              .stdout,
          )
          .unwrap();
          let gcc_version_trim = gcc_version.trim();
          builder
            .include("/usr/arm-linux-gnueabihf/include")
            .include(format!(
              "/usr/arm-linux-gnueabihf/include/c++/${gcc_version_trim}/arm-linux-gnueabihf"
            ));
          println!("cargo:rustc-link-search=/usr/arm-linux-gnueabihf/lib");
          println!("cargo:rustc-link-search=/usr/arm-linux-gnueabihf/lib/llvm-14/lib");
        }
        _ => {}
      }
    }
  }

  builder.file("src/woff2_c.cpp").compile("woff2_encode");

  napi_build::setup();
}
