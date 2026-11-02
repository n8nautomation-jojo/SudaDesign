# Third-Party Software, Library & Asset Notices

**Product:** Suda Design (Android Edition)  
**Audit & Generation Date:** 2026-11-01  
**Target Architecture:** `aarch64-linux-android` | **Compile SDK:** `35` | **Min SDK:** `26`  
**Toolchain:** `cargo 1.99.1 (5f94df478 2026-08-27)` | `rustc 1.99.1 (58cf8f9c3 2026-10-13)` | Gradle `8.14.3` | AGP `8.7.3`  
**Document Status:** DRAFT — PENDING TECHNICAL VERIFICATION AGAINST PRODUCTION RELEASE ARTIFACT  

---

## 1. Scope and Inventory Boundaries

This inventory covers components linked, packaged, or bundled into the Android application artifact:  
1. **Android Gradle Dependencies:** Core AndroidX and Android GameActivity runtimes.  
2. **Bundled Non-Code Assets:** Typography (fonts), iconography, and neural network weights.  
3. **Native Rust Shared Library:** All 359 distinct crates resolved in the target dependency graph for `aarch64-linux-android`.

*Exclusions:* Build-time host developer tools (e.g. desktop CLI utilities, test fixtures, formatting linters) not included in the release `.apk`/`.aab` package are excluded from this distribution notice.

---

## 2. Upstream Core Project

### PhotoCraft Core Engine
- **Copyright:** (c) 2026 ArtCraft Team and PhotoCraft contributors
- **License:** Dual MIT OR Apache-2.0
- **Upstream Source:** https://github.com/storytold/photocraft
- **Description:** Clean-room, native Photoshop-comparable image editing engine written in Rust.

PhotoCraft components are utilized under the permissive terms of either the MIT License or the Apache License, Version 2.0.

---

## 3. Android Gradle Dependencies (JVM / Jetpack)

| Dependency Name | Version | Declared License | Origin / Repository |
| :--- | :--- | :--- | :--- |
| `androidx.core:core-ktx` | `1.15.0` | `Apache-2.0` | https://android.googlesource.com/platform/frameworks/support |
| `androidx.appcompat:appcompat` | `1.7.0` | `Apache-2.0` | https://android.googlesource.com/platform/frameworks/support |
| `androidx.games:games-activity` | `3.0.5` | `Apache-2.0` | https://android.googlesource.com/platform/frameworks/support |

---

## 4. Bundled Fonts, Icons & Model Weights

| Asset Name | Version | License | Author / Copyright Holder | Source |
| :--- | :--- | :--- | :--- | :--- |
| **Inter Font Family** | 4.0 | `OFL-1.1` | Rasmus Andersson | https://github.com/rsms/inter |
| **JetBrains Mono** | 2.304 | `OFL-1.1` | JetBrains s.r.o. | https://github.com/JetBrains/JetBrainsMono |
| **BIZ UDPGothic / BIZ UDMincho / Shippori Mincho** | unspecified | `OFL-1.1` | Morisawa Inc. / Fontworks / Osamu Omiya | https://fonts.google.com |
| **Lucide Icons** | 0.300+ | `ISC` | Lucide Contributors | https://github.com/lucide-icons/lucide |
| **Magnetic Lasso Icon** | N/A | `CC-BY-3.0` | Arthur Shlain from Noun Project | https://thenounproject.com |
| **libfacedetection.bin** | weights-v1 | `BSD-3-Clause` | Shiqi Yu | https://github.com/ShiqiYu/libfacedetection |

---

## 5. Non-Standard SPDX & Flagged Expressions Analysis

The following packages declare legacy slash syntax (e.g., `LicenseA/LicenseB`), exceptions, or non-standard identifiers in Cargo metadata. Each has been reviewed for permissibility:

| Package Name | Version | Raw Declared License | Evaluated Permissibility |
| :--- | :--- | :--- | :--- |
| `bit_field` | 0.10.3 | `Apache-2.0/MIT` | Permissive dual-license (standard slash syntax representing OR) |
| `blake3` | 1.8.7 | `CC0-1.0 OR Apache-2.0 OR Apache-2.0 WITH LLVM-exception` | Permissive Apache 2.0 with LLVM Exception |
| `brotli-decompressor` | 5.0.3 | `BSD-3-Clause/MIT` | Permissive dual-license (standard slash syntax representing OR) |
| `filetime` | 0.2.29 | `MIT/Apache-2.0` | Permissive dual-license (standard slash syntax representing OR) |
| `guillotiere` | 0.7.0 | `MIT/Apache-2.0` | Permissive dual-license (standard slash syntax representing OR) |
| `khronos-egl` | 6.0.0 | `MIT/Apache-2.0` | Permissive dual-license (standard slash syntax representing OR) |
| `linux-raw-sys` | 0.12.1 | `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT` | Permissive Apache 2.0 with LLVM Exception |
| `pollster` | 1.0.1 | `Apache-2.0/MIT` | Permissive dual-license (standard slash syntax representing OR) |
| `qoi` | 0.4.1 | `MIT/Apache-2.0` | Permissive dual-license (standard slash syntax representing OR) |
| `quick-error` | 2.0.1 | `MIT/Apache-2.0` | Permissive dual-license (standard slash syntax representing OR) |
| `rustix` | 1.1.5 | `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT` | Permissive Apache 2.0 with LLVM Exception |
| `same-file` | 1.0.6 | `Unlicense/MIT` | Permissive dual-license (standard slash syntax representing OR) |
| `type-map` | 0.5.1 | `MIT/Apache-2.0` | Permissive dual-license (standard slash syntax representing OR) |
| `unicode-bidi-mirroring` | 0.4.0 | `MIT/Apache-2.0` | Permissive dual-license (standard slash syntax representing OR) |
| `unicode-ccc` | 0.4.0 | `MIT/Apache-2.0` | Permissive dual-license (standard slash syntax representing OR) |
| `unicode-properties` | 0.1.4 | `MIT/Apache-2.0` | Permissive dual-license (standard slash syntax representing OR) |
| `unicode-vo` | 0.1.0 | `MIT/Apache-2.0` | Permissive dual-license (standard slash syntax representing OR) |
| `version_check` | 0.9.5 | `MIT/Apache-2.0` | Permissive dual-license (standard slash syntax representing OR) |
| `walkdir` | 2.5.0 | `Unlicense/MIT` | Permissive dual-license (standard slash syntax representing OR) |
| `wasmi` | 2.0.0 | `MIT/Apache-2.0` | Permissive dual-license (standard slash syntax representing OR) |
| `wasmi_collections` | 2.0.0 | `MIT/Apache-2.0` | Permissive dual-license (standard slash syntax representing OR) |
| `wasmi_core` | 2.0.0 | `MIT/Apache-2.0` | Permissive dual-license (standard slash syntax representing OR) |
| `wasmi_ir` | 2.0.0 | `MIT/Apache-2.0` | Permissive dual-license (standard slash syntax representing OR) |
| `wasmparser` | 0.262.0 | `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT` | Permissive Apache 2.0 with LLVM Exception |

---

## 6. Complete Inventory of Compiled Native Rust Crates

The following 359 crates are compiled into the native shared library (`libphotocraft_android.so` for `aarch64-linux-android`):

| Crate Name | Version | Declared License | Repository / Homepage |
| :--- | :--- | :--- | :--- |
| `accesskit` | 0.25.1 | `MIT OR Apache-2.0` | https://github.com/AccessKit/accesskit |
| `accesskit_winit` | 0.34.1 | `Apache-2.0` | https://github.com/AccessKit/accesskit |
| `adler2` | 2.0.1 | `0BSD OR MIT OR Apache-2.0` | https://github.com/oyvindln/adler2 |
| `ahash` | 0.8.12 | `MIT OR Apache-2.0` | https://github.com/tkaitchuck/ahash |
| `aho-corasick` | 1.1.5 | `Unlicense OR MIT` | https://github.com/BurntSushi/aho-corasick |
| `alloc-no-stdlib` | 2.0.4 | `BSD-3-Clause` | https://github.com/dropbox/rust-alloc-no-stdlib |
| `alloc-stdlib` | 0.2.4 | `BSD-3-Clause` | https://github.com/dropbox/rust-alloc-no-stdlib |
| `allocator-api2` | 0.2.21 | `MIT OR Apache-2.0` | https://github.com/zakarumych/allocator-api2 |
| `android-activity` | 0.6.1 | `MIT OR Apache-2.0` | https://github.com/rust-mobile/android-activity |
| `android-properties` | 0.2.2 | `MIT` | https://github.com/miklelappo/android-properties |
| `android_log-sys` | 0.3.2 | `MIT OR Apache-2.0` | https://github.com/rust-mobile/android_log-sys-rs |
| `android_logger` | 0.15.1 | `MIT OR Apache-2.0` | https://github.com/rust-mobile/android_logger-rs |
| `android_system_properties` | 0.1.6 | `MIT OR Apache-2.0` | https://github.com/nical/android_system_properties |
| `arrayref` | 0.3.9 | `BSD-2-Clause` | https://github.com/droundy/arrayref |
| `arrayvec` | 0.7.8 | `MIT OR Apache-2.0` | https://github.com/bluss/arrayvec |
| `ash` | 0.38.0+1.3.281 | `MIT OR Apache-2.0` | https://github.com/ash-rs/ash |
| `autocfg` | 1.5.1 | `Apache-2.0 OR MIT` | https://github.com/cuviper/autocfg |
| `base64` | 0.23.1 | `MIT OR Apache-2.0` | https://github.com/marshallpierce/rust-base64 |
| `bit-set` | 0.11.1 | `Apache-2.0 OR MIT` | https://github.com/contain-rs/bit-vec |
| `bit-vec` | 0.10.1 | `Apache-2.0 OR MIT` | https://github.com/contain-rs/bit-vec |
| `bit_field` | 0.10.3 | `Apache-2.0/MIT` | https://github.com/phil-opp/rust-bit-field |
| `bitflags` | 2.13.2 | `MIT OR Apache-2.0` | https://github.com/bitflags/bitflags |
| `blake3` | 1.8.7 | `CC0-1.0 OR Apache-2.0 OR Apache-2.0 WITH LLVM-exception` | https://github.com/BLAKE3-team/BLAKE3 |
| `block-buffer` | 0.10.4 | `MIT OR Apache-2.0` | https://github.com/RustCrypto/utils |
| `brotli-decompressor` | 5.0.3 | `BSD-3-Clause/MIT` | https://github.com/dropbox/rust-brotli-decompressor |
| `bytemuck` | 1.25.2 | `Zlib OR Apache-2.0 OR MIT` | https://github.com/Lokathor/bytemuck |
| `bytemuck_derive` | 1.12.1 | `Zlib OR Apache-2.0 OR MIT` | https://github.com/Lokathor/bytemuck |
| `byteorder-lite` | 0.1.0 | `Unlicense OR MIT` | https://github.com/image-rs/byteorder-lite |
| `bytes` | 1.12.1 | `MIT` | https://github.com/tokio-rs/bytes |
| `cc` | 1.5.1 | `MIT OR Apache-2.0` | https://github.com/rust-lang/cc-rs |
| `cfg-if` | 1.0.5 | `MIT OR Apache-2.0` | https://github.com/rust-lang/cfg-if |
| `cfg_aliases` | 0.2.2 | `MIT` | https://github.com/katharostech/cfg_aliases |
| `chrono` | 0.4.45 | `MIT OR Apache-2.0` | https://github.com/chronotope/chrono |
| `cobs` | 0.3.0 | `MIT OR Apache-2.0` | https://github.com/jamesmunns/cobs.rs |
| `codespan-reporting` | 0.13.1 | `Apache-2.0` | https://github.com/brendanzab/codespan |
| `color` | 0.3.3 | `Apache-2.0 OR MIT` | https://github.com/linebender/color |
| `color_quant` | 1.1.0 | `MIT` | https://github.com/image-rs/color_quant.git |
| `combine` | 4.6.8 | `MIT` | https://github.com/Marwes/combine |
| `constant_time_eq` | 0.4.2 | `CC0-1.0 OR MIT-0 OR Apache-2.0` | https://github.com/cesarb/constant_time_eq |
| `core_maths` | 0.1.1 | `MIT` | https://github.com/robertbastian/core_maths |
| `cpufeatures` | 0.3.1 | `MIT OR Apache-2.0` | https://github.com/RustCrypto/utils |
| `craft-docpath` | 0.1.0 | `MIT OR Apache-2.0` | https://github.com/storytold/craft-libs |
| `craft-fonts-build` | 0.1.0 | `MIT OR Apache-2.0` | https://github.com/storytold/craft-libs |
| `craft-i18n` | 0.1.0 | `MIT OR Apache-2.0` | https://github.com/storytold/craft-libs |
| `craft-jpeg` | 0.1.0 | `MIT OR Apache-2.0` | https://github.com/storytold/craft-libs |
| `craft-print` | 0.1.0 | `MIT OR Apache-2.0` | https://github.com/storytold/craft-libs |
| `craft-ui` | 0.1.0 | `MIT OR Apache-2.0` | https://github.com/storytold/craft-libs |
| `craft-update` | 0.1.0 | `MIT OR Apache-2.0` | https://github.com/storytold/craft-libs |
| `craft-winprint-gdi` | 0.1.0 | `MIT OR Apache-2.0` | N/A |
| `crc32fast` | 1.5.3 | `MIT OR Apache-2.0` | https://github.com/srijs/rust-crc32fast |
| `crossbeam-deque` | 0.8.8 | `MIT OR Apache-2.0` | https://github.com/crossbeam-rs/crossbeam |
| `crossbeam-epoch` | 0.9.21 | `MIT OR Apache-2.0` | https://github.com/crossbeam-rs/crossbeam |
| `crossbeam-utils` | 0.8.23 | `MIT OR Apache-2.0` | https://github.com/crossbeam-rs/crossbeam |
| `crypto-common` | 0.1.7 | `MIT OR Apache-2.0` | https://github.com/RustCrypto/traits |
| `cursor-icon` | 1.2.0 | `MIT OR Apache-2.0 OR Zlib` | https://github.com/rust-windowing/cursor-icon |
| `data-url` | 0.3.2 | `MIT OR Apache-2.0` | https://github.com/servo/rust-url |
| `digest` | 0.10.7 | `MIT OR Apache-2.0` | https://github.com/RustCrypto/traits |
| `displaydoc` | 0.2.7 | `MIT OR Apache-2.0` | https://github.com/yaahc/displaydoc |
| `dlib` | 0.5.3 | `MIT` | https://github.com/elinorbgr/dlib |
| `document-features` | 0.2.12 | `MIT OR Apache-2.0` | https://github.com/slint-ui/document-features |
| `dpi` | 0.1.2 | `Apache-2.0 AND MIT` | https://github.com/rust-windowing/winit |
| `ecolor` | 0.37.0 | `MIT OR Apache-2.0` | https://github.com/emilk/egui |
| `eframe` | 0.37.0 | `MIT OR Apache-2.0` | https://github.com/emilk/egui/tree/main/crates/eframe |
| `egui` | 0.37.0 | `MIT OR Apache-2.0` | https://github.com/emilk/egui |
| `egui-wgpu` | 0.37.0 | `MIT OR Apache-2.0` | https://github.com/emilk/egui/tree/main/crates/egui-wgpu |
| `egui-winit` | 0.37.0 | `MIT OR Apache-2.0` | https://github.com/emilk/egui/tree/main/crates/egui-winit |
| `egui_extras` | 0.37.0 | `MIT OR Apache-2.0` | https://github.com/emilk/egui |
| `either` | 1.18.0 | `MIT OR Apache-2.0` | https://github.com/rayon-rs/either |
| `emath` | 0.37.0 | `MIT OR Apache-2.0` | https://github.com/emilk/egui/tree/main/crates/emath |
| `enum-map` | 3.1.0 | `MIT OR Apache-2.0` | https://codeberg.org/sugar700/enum-map |
| `enum-map-derive` | 1.0.1 | `MIT OR Apache-2.0` | https://codeberg.org/sugar700/enum-map |
| `env_filter` | 0.1.4 | `MIT OR Apache-2.0` | https://github.com/rust-cli/env_logger |
| `epaint` | 0.37.0 | `MIT OR Apache-2.0` | https://github.com/emilk/egui/tree/main/crates/epaint |
| `epaint_default_fonts` | 0.37.0 | `(MIT OR Apache-2.0) AND OFL-1.1 AND Ubuntu-font-1.0` | https://github.com/emilk/egui/tree/main/crates/epaint_default_fonts |
| `equivalent` | 1.0.2 | `Apache-2.0 OR MIT` | https://github.com/indexmap-rs/equivalent |
| `errno` | 0.3.14 | `MIT OR Apache-2.0` | https://github.com/lambda-fairy/rust-errno |
| `euclid` | 0.22.14 | `MIT OR Apache-2.0` | https://github.com/servo/euclid |
| `exr` | 1.74.2 | `BSD-3-Clause` | https://github.com/johannesvollmer/exrs |
| `fastrand` | 2.5.0 | `Apache-2.0 OR MIT` | https://github.com/smol-rs/fastrand |
| `fax` | 0.2.6 | `MIT` | https://github.com/pdf-rs/fax |
| `fax_derive` | 0.2.0 | `MIT` | https://github.com/pdf-rs/fax |
| `fdeflate` | 0.3.7 | `MIT OR Apache-2.0` | https://github.com/image-rs/fdeflate |
| `fearless_simd` | 0.4.1 | `Apache-2.0 OR MIT` | https://github.com/linebender/fearless_simd |
| `filetime` | 0.2.29 | `MIT/Apache-2.0` | https://github.com/alexcrichton/filetime |
| `find-msvc-tools` | 0.1.14 | `MIT OR Apache-2.0` | https://github.com/rust-lang/cc-rs |
| `fixed_decimal` | 0.7.3 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `flate2` | 1.1.10 | `MIT OR Apache-2.0` | https://github.com/rust-lang/flate2-rs |
| `float-cmp` | 0.9.0 | `MIT` | https://github.com/mikedilger/float-cmp |
| `foldhash` | 0.2.0 | `Zlib` | https://github.com/orlp/foldhash |
| `font-types` | 0.12.5 | `MIT OR Apache-2.0` | https://github.com/googlefonts/fontations |
| `fontdb` | 0.23.0 | `MIT` | https://github.com/RazrFalcon/fontdb |
| `fontique` | 0.11.1 | `Apache-2.0 OR MIT` | https://github.com/linebender/parley |
| `generic-array` | 0.14.7 | `MIT` | https://github.com/fizyk20/generic-array.git |
| `getrandom` | 0.4.3 | `MIT OR Apache-2.0` | https://github.com/rust-random/getrandom |
| `gif` | 0.14.2 | `MIT OR Apache-2.0` | https://github.com/image-rs/image-gif |
| `glow` | 0.17.0 | `MIT OR Apache-2.0 OR Zlib` | https://github.com/grovesNL/glow |
| `gpu-allocator` | 0.28.0 | `MIT OR Apache-2.0` | https://github.com/Traverse-Research/gpu-allocator |
| `guillotiere` | 0.7.0 | `MIT/Apache-2.0` | https://github.com/nical/guillotiere |
| `half` | 2.7.1 | `MIT OR Apache-2.0` | https://github.com/VoidStarKat/half-rs |
| `harfrust` | 0.12.0 | `MIT` | https://github.com/harfbuzz/harfrust |
| `hashbrown` | 0.17.1 | `MIT OR Apache-2.0` | https://github.com/rust-lang/hashbrown |
| `http` | 1.5.0 | `MIT OR Apache-2.0` | https://github.com/hyperium/http |
| `httparse` | 1.10.1 | `MIT OR Apache-2.0` | https://github.com/seanmonstar/httparse |
| `icu_collections` | 2.3.0 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `icu_decimal` | 2.3.0 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `icu_locale_core` | 2.3.0 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `icu_locale_fallback` | 2.3.0 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `icu_locale_fallback_data` | 2.3.0 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `icu_normalizer` | 2.3.0 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `icu_normalizer_data` | 2.3.0 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `icu_plurals` | 2.3.0 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `icu_properties` | 2.3.0 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `icu_properties_data` | 2.3.0 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `icu_provider` | 2.3.1 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `icu_provider_adapters` | 2.3.0 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `icu_provider_blob` | 2.3.0 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `icu_segmenter` | 2.3.0 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `icu_segmenter_data` | 2.3.0 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `image` | 0.25.10 | `MIT OR Apache-2.0` | https://github.com/image-rs/image |
| `image-webp` | 0.2.4 | `MIT OR Apache-2.0` | https://github.com/image-rs/image-webp |
| `imagesize` | 0.13.0 | `MIT` | https://github.com/Roughsketch/imagesize |
| `indexmap` | 2.14.2 | `Apache-2.0 OR MIT` | https://github.com/indexmap-rs/indexmap |
| `itertools` | 0.15.0 | `MIT OR Apache-2.0` | https://github.com/rust-itertools/itertools |
| `itoa` | 1.0.18 | `MIT OR Apache-2.0` | https://github.com/dtolnay/itoa |
| `jni` | 0.22.4 | `MIT OR Apache-2.0` | https://github.com/jni-rs/jni-rs |
| `jni-macros` | 0.22.4 | `MIT OR Apache-2.0` | https://github.com/jni-rs/jni-rs |
| `jni-sys` | 0.4.1 | `MIT OR Apache-2.0` | https://github.com/jni-rs/jni-sys |
| `jni-sys-macros` | 0.4.1 | `MIT OR Apache-2.0` | https://github.com/jni-rs/jni-sys |
| `jobserver` | 0.1.35 | `MIT OR Apache-2.0` | https://github.com/rust-lang/jobserver-rs |
| `jpeg-encoder` | 0.6.1 | `(MIT OR Apache-2.0) AND IJG` | https://github.com/vstroebel/jpeg-encoder |
| `jxl-bitstream` | 1.1.0 | `MIT OR Apache-2.0` | https://github.com/tirr-c/jxl-oxide.git |
| `jxl-coding` | 1.0.1 | `MIT OR Apache-2.0` | https://github.com/tirr-c/jxl-oxide.git |
| `jxl-color` | 0.11.0 | `MIT OR Apache-2.0` | https://github.com/tirr-c/jxl-oxide.git |
| `jxl-frame` | 0.13.3 | `MIT OR Apache-2.0` | https://github.com/tirr-c/jxl-oxide.git |
| `jxl-grid` | 0.6.2 | `MIT OR Apache-2.0` | https://github.com/tirr-c/jxl-oxide.git |
| `jxl-image` | 0.13.0 | `MIT OR Apache-2.0` | https://github.com/tirr-c/jxl-oxide.git |
| `jxl-jbr` | 0.2.1 | `MIT OR Apache-2.0` | https://github.com/tirr-c/jxl-oxide.git |
| `jxl-modular` | 0.11.3 | `MIT OR Apache-2.0` | https://github.com/tirr-c/jxl-oxide.git |
| `jxl-oxide` | 0.12.6 | `MIT OR Apache-2.0` | https://github.com/tirr-c/jxl-oxide.git |
| `jxl-oxide-common` | 1.0.0 | `MIT OR Apache-2.0` | https://github.com/tirr-c/jxl-oxide.git |
| `jxl-render` | 0.12.4 | `MIT OR Apache-2.0` | https://github.com/tirr-c/jxl-oxide.git |
| `jxl-threadpool` | 1.0.0 | `MIT OR Apache-2.0` | https://github.com/tirr-c/jxl-oxide.git |
| `jxl-vardct` | 0.11.1 | `MIT OR Apache-2.0` | https://github.com/tirr-c/jxl-oxide.git |
| `khronos-egl` | 6.0.0 | `MIT/Apache-2.0` | https://github.com/timothee-haudebourg/khronos-egl |
| `kurbo` | 0.13.1 | `Apache-2.0 OR MIT` | https://github.com/linebender/kurbo |
| `lebe` | 0.5.3 | `BSD-3-Clause` | https://github.com/johannesvollmer/lebe |
| `libc` | 0.2.191 | `MIT OR Apache-2.0` | https://github.com/rust-lang/libc |
| `libloading` | 0.8.9 | `ISC` | https://github.com/nagisa/rust_libloading/ |
| `libm` | 0.2.16 | `MIT` | https://github.com/rust-lang/compiler-builtins |
| `linebender_resource_handle` | 0.1.1 | `Apache-2.0 OR MIT` | https://github.com/linebender/raw_resource_handle |
| `linux-raw-sys` | 0.12.1 | `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT` | https://github.com/sunfishcode/linux-raw-sys |
| `litemap` | 0.8.3 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `litrs` | 1.0.0 | `MIT OR Apache-2.0` | https://github.com/LukasKalbertodt/litrs |
| `lock_api` | 0.4.14 | `MIT OR Apache-2.0` | https://github.com/Amanieu/parking_lot |
| `log` | 0.4.34 | `MIT OR Apache-2.0` | https://github.com/rust-lang/log |
| `memchr` | 2.8.3 | `Unlicense OR MIT` | https://github.com/BurntSushi/memchr |
| `memmap2` | 0.9.11 | `MIT OR Apache-2.0` | https://github.com/RazrFalcon/memmap2-rs |
| `miniz_oxide` | 0.9.1 | `MIT OR Zlib OR Apache-2.0` | https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide |
| `moxcms` | 0.8.1 | `BSD-3-Clause OR Apache-2.0` | https://github.com/awxkee/moxcms.git |
| `naga` | 30.0.1 | `MIT OR Apache-2.0` | https://github.com/gfx-rs/wgpu |
| `naga-types` | 30.0.1 | `MIT OR Apache-2.0` | https://github.com/gfx-rs/wgpu |
| `ndk` | 0.9.0 | `MIT OR Apache-2.0` | https://github.com/rust-mobile/ndk |
| `ndk-context` | 0.1.1 | `MIT OR Apache-2.0` | https://github.com/rust-windowing/android-ndk-rs |
| `ndk-sys` | 0.6.0+11769913 | `MIT OR Apache-2.0` | https://github.com/rust-mobile/ndk |
| `nohash-hasher` | 0.2.0 | `Apache-2.0 OR MIT` | https://github.com/paritytech/nohash-hasher |
| `num-complex` | 0.4.6 | `MIT OR Apache-2.0` | https://github.com/rust-num/num-complex |
| `num-integer` | 0.1.47 | `MIT OR Apache-2.0` | https://github.com/rust-num/num-integer |
| `num-traits` | 0.2.19 | `MIT OR Apache-2.0` | https://github.com/rust-num/num-traits |
| `num_enum` | 0.7.6 | `BSD-3-Clause OR MIT OR Apache-2.0` | https://github.com/illicitonion/num_enum |
| `num_enum_derive` | 0.7.6 | `BSD-3-Clause OR MIT OR Apache-2.0` | https://github.com/illicitonion/num_enum |
| `once_cell` | 1.21.4 | `MIT OR Apache-2.0` | https://github.com/matklad/once_cell |
| `ordered-float` | 5.5.0 | `MIT` | https://github.com/reem/rust-ordered-float |
| `parking_lot` | 0.12.5 | `MIT OR Apache-2.0` | https://github.com/Amanieu/parking_lot |
| `parking_lot_core` | 0.9.12 | `MIT OR Apache-2.0` | https://github.com/Amanieu/parking_lot |
| `parlance` | 0.1.0 | `Apache-2.0 OR MIT` | https://github.com/linebender/parley |
| `parley` | 0.11.1 | `Apache-2.0 OR MIT` | https://github.com/linebender/parley |
| `parley_data` | 0.11.1 | `Apache-2.0 OR MIT` | https://github.com/linebender/parley |
| `paste` | 1.0.15 | `MIT OR Apache-2.0` | https://github.com/dtolnay/paste |
| `peniko` | 0.6.1 | `Apache-2.0 OR MIT` | https://github.com/linebender/peniko |
| `percent-encoding` | 2.3.2 | `MIT OR Apache-2.0` | https://github.com/servo/rust-url/ |
| `photocraft-affinity` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `photocraft-algo` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `photocraft-android` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `photocraft-cms` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `photocraft-codecs` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `photocraft-color` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `photocraft-compose` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `photocraft-doc` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `photocraft-engine` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `photocraft-format` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `photocraft-geom` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `photocraft-gpu` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `photocraft-io` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `photocraft-ipp` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `photocraft-ops` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `photocraft-paint` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `photocraft-plugins` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `photocraft-psd` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `photocraft-raster` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `photocraft-raw` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `photocraft-text` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `photocraft-ui-egui` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `photocraft-vector` | 0.18.0 | `MIT OR Apache-2.0` | N/A |
| `pico-args` | 0.5.0 | `MIT` | https://github.com/RazrFalcon/pico-args |
| `pin-project-lite` | 0.2.17 | `Apache-2.0 OR MIT` | https://github.com/taiki-e/pin-project-lite |
| `pkg-config` | 0.3.34 | `MIT OR Apache-2.0` | https://github.com/rust-lang/pkg-config-rs |
| `png` | 0.18.1 | `MIT OR Apache-2.0` | https://github.com/image-rs/image-png |
| `pollster` | 1.0.1 | `Apache-2.0/MIT` | https://github.com/zesterer/pollster |
| `polycool` | 0.4.0 | `MIT OR Apache-2.0` | https://github.com/linebender/kurbo |
| `postcard` | 1.1.3 | `MIT OR Apache-2.0` | https://github.com/jamesmunns/postcard |
| `potential_utf` | 0.1.6 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `presser` | 0.3.1 | `MIT OR Apache-2.0` | https://github.com/EmbarkStudios/presser |
| `primal-check` | 0.3.4 | `MIT OR Apache-2.0` | https://github.com/huonw/primal |
| `proc-macro-crate` | 3.5.0 | `MIT OR Apache-2.0` | https://github.com/bkchr/proc-macro-crate |
| `proc-macro2` | 1.0.107 | `MIT OR Apache-2.0` | https://github.com/dtolnay/proc-macro2 |
| `profiling` | 1.0.18 | `MIT OR Apache-2.0` | https://github.com/aclysma/profiling |
| `pulp` | 0.22.3 | `MIT` | https://github.com/sarah-quinones/pulp/ |
| `pulp-wasm-simd-flag` | 0.1.1 | `MIT` | https://github.com/sarah-quinones/pulp/ |
| `pxfm` | 0.1.30 | `BSD-3-Clause OR Apache-2.0` | https://github.com/awxkee/pxfm |
| `qoi` | 0.4.1 | `MIT/Apache-2.0` | https://github.com/aldanor/qoi-rust |
| `quick-error` | 2.0.1 | `MIT/Apache-2.0` | http://github.com/tailhook/quick-error |
| `quote` | 1.0.47 | `MIT OR Apache-2.0` | https://github.com/dtolnay/quote |
| `raw-window-handle` | 0.6.2 | `MIT OR Apache-2.0 OR Zlib` | https://github.com/rust-windowing/raw-window-handle |
| `rayon` | 1.12.0 | `MIT OR Apache-2.0` | https://github.com/rayon-rs/rayon |
| `rayon-core` | 1.13.0 | `MIT OR Apache-2.0` | https://github.com/rayon-rs/rayon |
| `read-fonts` | 0.41.0 | `MIT OR Apache-2.0` | https://github.com/googlefonts/fontations |
| `reborrow` | 0.5.5 | `MIT` | https://github.com/sarah-ek/reborrow/ |
| `regex` | 1.13.1 | `MIT OR Apache-2.0` | https://github.com/rust-lang/regex |
| `regex-automata` | 0.4.18 | `MIT OR Apache-2.0` | https://github.com/rust-lang/regex |
| `regex-syntax` | 0.8.11 | `MIT OR Apache-2.0` | https://github.com/rust-lang/regex |
| `renderdoc-sys` | 1.1.0 | `MIT OR Apache-2.0` | https://github.com/ebkalderon/renderdoc-rs |
| `resvg` | 0.45.1 | `Apache-2.0 OR MIT` | https://github.com/linebender/resvg |
| `rgb` | 0.8.53 | `MIT` | https://github.com/kornelski/rust-rgb |
| `ring` | 0.17.14 | `Apache-2.0 AND ISC` | https://github.com/briansmith/ring |
| `roxmltree` | 0.21.1 | `MIT OR Apache-2.0` | https://github.com/RazrFalcon/roxmltree |
| `rustc-hash` | 2.1.3 | `Apache-2.0 OR MIT` | https://github.com/rust-lang/rustc-hash |
| `rustc_version` | 0.4.1 | `MIT OR Apache-2.0` | https://github.com/djc/rustc-version-rs |
| `rustfft` | 6.4.1 | `MIT OR Apache-2.0` | https://github.com/ejmahler/RustFFT |
| `rustix` | 1.1.5 | `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT` | https://github.com/bytecodealliance/rustix |
| `rustls` | 0.23.46 | `Apache-2.0 OR ISC OR MIT` | https://github.com/rustls/rustls |
| `rustls-pki-types` | 1.15.1 | `MIT OR Apache-2.0` | https://github.com/rustls/pki-types |
| `rustls-platform-verifier` | 0.7.1 | `MIT OR Apache-2.0` | https://github.com/rustls/rustls-platform-verifier |
| `rustls-platform-verifier-android` | 0.2.0 | `MIT OR Apache-2.0` | https://github.com/rustls/rustls-platform-verifier |
| `rustls-webpki` | 0.103.15 | `ISC` | https://github.com/rustls/webpki |
| `rustversion` | 1.0.23 | `MIT OR Apache-2.0` | https://github.com/dtolnay/rustversion |
| `rustybuzz` | 0.20.1 | `MIT` | https://github.com/harfbuzz/rustybuzz |
| `ruzstd` | 0.9.1 | `MIT` | https://github.com/KillingSpark/zstd-rs |
| `same-file` | 1.0.6 | `Unlicense/MIT` | https://github.com/BurntSushi/same-file |
| `scopeguard` | 1.2.0 | `MIT OR Apache-2.0` | https://github.com/bluss/scopeguard |
| `self_cell` | 1.3.0 | `Apache-2.0 OR GPL-2.0-only` | https://github.com/Voultapher/self_cell |
| `semver` | 1.0.28 | `MIT OR Apache-2.0` | https://github.com/dtolnay/semver |
| `serde` | 1.0.229 | `MIT OR Apache-2.0` | https://github.com/serde-rs/serde |
| `serde_core` | 1.0.229 | `MIT OR Apache-2.0` | https://github.com/serde-rs/serde |
| `serde_derive` | 1.0.229 | `MIT OR Apache-2.0` | https://github.com/serde-rs/serde |
| `serde_json` | 1.0.154 | `MIT OR Apache-2.0` | https://github.com/serde-rs/json |
| `serde_spanned` | 1.1.1 | `MIT OR Apache-2.0` | https://github.com/toml-rs/toml |
| `sha2` | 0.10.9 | `MIT OR Apache-2.0` | https://github.com/RustCrypto/hashes |
| `shlex` | 2.0.1 | `MIT OR Apache-2.0` | https://github.com/comex/rust-shlex |
| `simd-adler32` | 0.3.10 | `MIT` | https://github.com/mcountryman/simd-adler32 |
| `simd_cesu8` | 1.2.0 | `Apache-2.0 OR MIT` | https://github.com/seancroach/simd_cesu8 |
| `simdutf8` | 0.1.5 | `MIT OR Apache-2.0` | https://github.com/rusticstuff/simdutf8 |
| `simplecss` | 0.2.2 | `Apache-2.0 OR MIT` | https://github.com/linebender/simplecss |
| `siphasher` | 1.0.4 | `MIT OR Apache-2.0` | https://github.com/jedisct1/rust-siphash |
| `skrifa` | 0.44.0 | `MIT OR Apache-2.0` | https://github.com/googlefonts/fontations |
| `slotmap` | 1.1.1 | `Zlib` | https://github.com/orlp/slotmap |
| `smallvec` | 1.16.2 | `MIT OR Apache-2.0` | https://github.com/servo/rust-smallvec |
| `smol_str` | 0.2.2 | `MIT OR Apache-2.0` | https://github.com/rust-analyzer/smol_str |
| `spin` | 0.9.9 | `MIT` | https://github.com/mvdnes/spin-rs.git |
| `spirv` | 0.4.0+sdk-1.4.341.0 | `Apache-2.0` | https://github.com/gfx-rs/rspirv |
| `stable_deref_trait` | 1.2.1 | `MIT OR Apache-2.0` | https://github.com/storyyeller/stable_deref_trait |
| `static_assertions` | 1.1.0 | `MIT OR Apache-2.0` | https://github.com/nvzqz/static-assertions-rs |
| `strength_reduce` | 0.2.4 | `MIT OR Apache-2.0` | http://github.com/ejmahler/strength_reduce |
| `strict-num` | 0.1.1 | `MIT` | https://github.com/RazrFalcon/strict-num |
| `subtle` | 2.6.1 | `BSD-3-Clause` | https://github.com/dalek-cryptography/subtle |
| `svgtypes` | 0.15.3 | `Apache-2.0 OR MIT` | https://github.com/linebender/svgtypes |
| `syn` | 3.0.6 | `MIT OR Apache-2.0` | https://github.com/dtolnay/syn |
| `synstructure` | 0.14.0 | `MIT` | https://github.com/mystor/synstructure |
| `sys-locale` | 0.3.2 | `MIT OR Apache-2.0` | https://github.com/1Password/sys-locale |
| `tar` | 0.4.46 | `MIT OR Apache-2.0` | https://github.com/composefs/tar-rs |
| `tempfile` | 3.27.0 | `MIT OR Apache-2.0` | https://github.com/Stebalien/tempfile |
| `thiserror` | 2.0.21 | `MIT OR Apache-2.0` | https://github.com/dtolnay/thiserror |
| `thiserror-impl` | 2.0.21 | `MIT OR Apache-2.0` | https://github.com/dtolnay/thiserror |
| `tiff` | 0.11.3 | `MIT` | https://github.com/image-rs/image-tiff |
| `tiny-skia` | 0.11.4 | `BSD-3-Clause` | https://github.com/RazrFalcon/tiny-skia |
| `tiny-skia-path` | 0.11.4 | `BSD-3-Clause` | https://github.com/RazrFalcon/tiny-skia/tree/master/path |
| `tinystr` | 0.8.4 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `tinyvec` | 1.13.3 | `Zlib OR Apache-2.0 OR MIT` | https://github.com/Lokathor/tinyvec |
| `toml` | 1.1.6+spec-1.1.0 | `MIT OR Apache-2.0` | https://github.com/toml-rs/toml |
| `toml_datetime` | 1.1.1+spec-1.1.0 | `MIT OR Apache-2.0` | https://github.com/toml-rs/toml |
| `toml_edit` | 0.25.15+spec-1.1.0 | `MIT OR Apache-2.0` | https://github.com/toml-rs/toml |
| `toml_parser` | 1.1.3+spec-1.1.0 | `MIT OR Apache-2.0` | https://github.com/toml-rs/toml |
| `toml_writer` | 1.1.2+spec-1.1.0 | `MIT OR Apache-2.0` | https://github.com/toml-rs/toml |
| `tracing` | 0.1.44 | `MIT` | https://github.com/tokio-rs/tracing |
| `tracing-core` | 0.1.36 | `MIT` | https://github.com/tokio-rs/tracing |
| `transpose` | 0.2.3 | `MIT OR Apache-2.0` | https://github.com/ejmahler/transpose |
| `ttf-parser` | 0.25.1 | `MIT OR Apache-2.0` | https://github.com/harfbuzz/ttf-parser |
| `twox-hash` | 2.1.4 | `MIT` | https://github.com/shepmaster/twox-hash |
| `type-map` | 0.5.1 | `MIT/Apache-2.0` | https://github.com/kardeiz/type-map |
| `typenum` | 1.20.1 | `MIT OR Apache-2.0` | https://github.com/paholg/typenum |
| `unicode-bidi` | 0.3.18 | `MIT OR Apache-2.0` | https://github.com/servo/unicode-bidi |
| `unicode-bidi-mirroring` | 0.4.0 | `MIT/Apache-2.0` | https://github.com/RazrFalcon/unicode-bidi-mirroring |
| `unicode-ccc` | 0.4.0 | `MIT/Apache-2.0` | https://github.com/RazrFalcon/unicode-ccc |
| `unicode-general-category` | 1.1.0 | `Apache-2.0` | https://github.com/yeslogic/unicode-general-category |
| `unicode-ident` | 1.0.26 | `(MIT OR Apache-2.0) AND Unicode-3.0` | https://github.com/dtolnay/unicode-ident |
| `unicode-properties` | 0.1.4 | `MIT/Apache-2.0` | https://github.com/unicode-rs/unicode-properties |
| `unicode-script` | 0.5.8 | `MIT OR Apache-2.0` | https://github.com/unicode-rs/unicode-script |
| `unicode-segmentation` | 1.13.3 | `MIT OR Apache-2.0` | https://github.com/unicode-rs/unicode-segmentation |
| `unicode-vo` | 0.1.0 | `MIT/Apache-2.0` | https://github.com/RazrFalcon/unicode-vo |
| `unicode-width` | 0.2.2 | `MIT OR Apache-2.0` | https://github.com/unicode-rs/unicode-width |
| `untrusted` | 0.9.0 | `ISC` | https://github.com/briansmith/untrusted |
| `ureq` | 3.5.0 | `MIT OR Apache-2.0` | https://github.com/algesten/ureq |
| `ureq-proto` | 0.6.5 | `MIT OR Apache-2.0` | https://github.com/algesten/ureq-proto |
| `usvg` | 0.45.1 | `Apache-2.0 OR MIT` | https://github.com/linebender/resvg |
| `utf16_iter` | 1.0.5 | `Apache-2.0 OR MIT` | https://github.com/hsivonen/utf16_iter |
| `utf8-zero` | 0.8.1 | `MIT OR Apache-2.0` | https://github.com/algesten/utf8-zero |
| `utf8_iter` | 1.0.4 | `Apache-2.0 OR MIT` | https://github.com/hsivonen/utf8_iter |
| `uuid` | 1.26.1 | `Apache-2.0 OR MIT` | https://github.com/uuid-rs/uuid |
| `vello_common` | 0.2.0 | `Apache-2.0 OR MIT` | https://github.com/linebender/vello |
| `vello_cpu` | 0.2.0 | `Apache-2.0 OR MIT` | https://github.com/linebender/vello |
| `version_check` | 0.9.5 | `MIT/Apache-2.0` | https://github.com/SergioBenitez/version_check |
| `walkdir` | 2.5.0 | `Unlicense/MIT` | https://github.com/BurntSushi/walkdir |
| `wasmi` | 2.0.0 | `MIT/Apache-2.0` | https://github.com/wasmi-labs/wasmi |
| `wasmi_collections` | 2.0.0 | `MIT/Apache-2.0` | https://github.com/wasmi-labs/wasmi |
| `wasmi_core` | 2.0.0 | `MIT/Apache-2.0` | https://github.com/wasmi-labs/wasmi |
| `wasmi_ir` | 2.0.0 | `MIT/Apache-2.0` | https://github.com/wasmi-labs/wasmi |
| `wasmparser` | 0.262.0 | `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT` | https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wasmparser |
| `wayland-sys` | 0.31.11 | `MIT` | https://github.com/smithay/wayland-rs |
| `web-time` | 1.1.0 | `MIT OR Apache-2.0` | https://github.com/daxpedda/web-time |
| `weezl` | 0.1.12 | `MIT OR Apache-2.0` | https://github.com/image-rs/weezl |
| `wgpu` | 30.0.1 | `MIT OR Apache-2.0` | https://github.com/gfx-rs/wgpu |
| `wgpu-core` | 30.0.1 | `MIT OR Apache-2.0` | https://github.com/gfx-rs/wgpu |
| `wgpu-core-deps-windows-linux-android` | 30.0.1 | `MIT OR Apache-2.0` | https://github.com/gfx-rs/wgpu |
| `wgpu-hal` | 30.0.1 | `MIT OR Apache-2.0` | https://github.com/gfx-rs/wgpu |
| `wgpu-naga-bridge` | 30.0.1 | `MIT OR Apache-2.0` | https://github.com/gfx-rs/wgpu |
| `wgpu-types` | 30.0.1 | `MIT OR Apache-2.0` | https://github.com/gfx-rs/wgpu |
| `winapi-util` | 0.1.11 | `Unlicense OR MIT` | https://github.com/BurntSushi/winapi-util |
| `windows-link` | 0.2.1 | `MIT OR Apache-2.0` | https://github.com/microsoft/windows-rs |
| `windows-sys` | 0.61.2 | `MIT OR Apache-2.0` | https://github.com/microsoft/windows-rs |
| `winit` | 0.30.13 | `Apache-2.0` | https://github.com/rust-windowing/winit |
| `winnow` | 1.0.4 | `MIT` | https://github.com/winnow-rs/winnow |
| `write16` | 1.0.0 | `Apache-2.0 OR MIT` | https://github.com/hsivonen/write16 |
| `writeable` | 0.6.4 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `xmlwriter` | 0.1.0 | `MIT` | https://github.com/RazrFalcon/xmlwriter |
| `yoke` | 0.8.3 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `yoke-derive` | 0.8.3 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `zerocopy` | 0.8.59 | `BSD-2-Clause OR Apache-2.0 OR MIT` | https://github.com/google/zerocopy |
| `zerocopy-derive` | 0.8.59 | `BSD-2-Clause OR Apache-2.0 OR MIT` | https://github.com/google/zerocopy |
| `zerofrom` | 0.1.8 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `zerofrom-derive` | 0.1.8 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `zeroize` | 1.9.1 | `Apache-2.0 OR MIT` | https://github.com/RustCrypto/utils |
| `zerotrie` | 0.2.5 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `zerovec` | 0.11.8 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `zerovec-derive` | 0.11.6 | `Unicode-3.0` | https://github.com/unicode-org/icu4x |
| `zlib-rs` | 0.6.8 | `Zlib` | https://github.com/trifectatechfoundation/zlib-rs |
| `zmij` | 1.0.23 | `MIT` | https://github.com/dtolnay/zmij |
| `zune-core` | 0.5.3 | `MIT OR Apache-2.0 OR Zlib` | https://github.com/etemesi254/zune-image |
| `zune-inflate` | 0.2.54 | `MIT OR Apache-2.0 OR Zlib` | https://github.com/etemesi254/zune-image/tree/main/zune-inflate |
| `zune-jpeg` | 0.5.15 | `MIT OR Apache-2.0 OR Zlib` | https://github.com/etemesi254/zune-image/tree/dev/crates/zune-jpeg |
| `zune-jpegxl` | 0.5.2 | `MIT OR Apache-2.0 OR Zlib` | https://github.com/etemesi254/zune-image/tree/dev/crates/zune-jpegxl |

---

## 7. Official Sources & Reference Standards

- **SPDX License List 3.25:** https://spdx.org/licenses/
- **Apache Software Foundation Licensing FAQ:** https://www.apache.org/foundation/license-faq.html
- **Open Source Initiative (OSI):** https://opensource.org/licenses
- **SIL Open Font License 1.1:** https://openfontlicense.org/
