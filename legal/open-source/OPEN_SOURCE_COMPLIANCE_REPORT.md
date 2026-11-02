# Open-Source Compliance & Dependency Audit Report

**Product:** Suda Design (Android Edition)  
**Document Ref:** OS-COMP-2026-02  
**Audit Date:** 2026-11-01  
**Status:** DRAFT — PENDING TECHNICAL VERIFICATION AGAINST PRODUCTION ARTIFACT  
**Target Architecture:** `aarch64-linux-android`  
**Toolchain Environment:** Cargo 1.99.1, Rustc 1.99.1, Gradle 8.14.3, AGP 8.7.3, Kotlin 2.0.21  

---

## 1. Scope and Inventory Boundaries

This compliance report details the technical analysis of open-source dependencies and third-party assets incorporated into the Suda Design Android build.

### Inventory Boundaries:
1. **Target Native Dependencies:** Evaluated strictly for the `aarch64-linux-android` target of crate `photocraft-android`, which compiles into the release shared object (`libphotocraft_android.so`). (359 distinct Cargo crates).
2. **Android Runtime Dependencies:** JVM / AAR libraries declared in `build.gradle.kts` (3 AndroidX packages).
3. **Bundled Non-Code Assets:** Font families, vector icons, and pre-trained neural network weights packaged into application assets.
4. **Exclusions:** Host-only build tools, formatting utilities (e.g. `rustfmt`, `clippy`), documentation generators, and desktop-only CLI crates (e.g. `colored` under MPL-2.0, `r-efi`) are **excluded** from the mobile distribution and do not enter the Android runtime binary.

---

## 2. Technical Findings by Category

### 2.1. Native Rust Dependencies (`aarch64-linux-android`)
- **Total Packages:** 359 crates resolved via `cargo tree --target aarch64-linux-android -p photocraft-android --prefix none`.
- **Copyleft Assessment:** Zero packages in the target graph are governed by reciprocal copyleft licenses (GPL, AGPL, or LGPL).
- **License Distribution (SPDX Expressions):**
  - Permissive Dual MIT / Apache-2.0: 238 packages (including legacy slash syntax `MIT/Apache-2.0`).
  - MIT License: 35 packages.
  - Unicode-3.0 License: 27 packages.
  - BSD-3-Clause / BSD-2-Clause: 11 packages.
  - Apache License 2.0: 8 packages (including packages with LLVM exception).
  - Zlib License: 6 packages.
  - Public Domain / Unlicense: 4 packages.
  - ISC License: 3 packages.
  - Dual Permissive combinations (Zlib/MIT, BSD/MIT): 27 packages.

### 2.2. Dual-Licensed Packages (`self_cell`)
- `self_cell` v1.3.0 declares `Apache-2.0 OR GPL-2.0-only`.
- **Finding:** Under the doctrine of alternative licensing, Suda Design elects the **Apache-2.0** branch. This election is documented in `legal/open-source/THIRD_PARTY_NOTICES.md`. No source code disclosure obligation is triggered.

### 2.3. Non-SPDX & Legacy Slash Syntax
- 25 packages in Cargo metadata declare legacy forward-slash delimiters (e.g., `MIT/Apache-2.0`, `Apache-2.0/MIT`, `Unlicense/MIT`) or `WITH LLVM-exception`.
- **Finding:** These represent standard permissive dual-licensing adopted before modern SPDX 2.0 expression standardization in Cargo. Each has been individually verified as permissive.

### 2.4. Android Gradle Dependencies
- `androidx.core:core-ktx:1.15.0` (Apache-2.0)
- `androidx.appcompat:appcompat:1.7.0` (Apache-2.0)
- `androidx.games:games-activity:3.0.5` (Apache-2.0)
- **Finding:** All runtime JVM libraries originate from the Android Open Source Project (AOSP) under Apache-2.0.

### 2.5. Bundled Assets
- **Fonts:** Inter, JetBrains Mono, BIZ UDPGothic, BIZ UDMincho, Shippori Mincho are licensed under the **SIL Open Font License 1.1 (OFL-1.1)**. None are sold standalone; all are embedded within software.
- **Icons:** Lucide icons (ISC), Magnetic Lasso (CC BY 3.0, credited to Arthur Shlain).
- **Neural Network Weights:** `libfacedetection.bin` (BSD-3-Clause, credited to Shiqi Yu).

---

## 3. Legal Interpretations vs. Technical Facts

| Aspect | Technical Finding (Verified Fact) | Legal Interpretation (Requires Legal Review) |
| :--- | :--- | :--- |
| **GPL / Copyleft Exposure** | Target build tree contains 0 GPL/AGPL packages. | Suda Design is not legally obligated to publish original proprietary modifications under copyleft rules. |
| **Apache 2.0 Section 6** | Upstream PhotoCraft is Apache-2.0/MIT; brand assets are separate SVG/PNG files. | Trademark rights in "Suda Design" may be asserted independently from software copyright. |
| **Dual License Election** | `self_cell` metadata offers Apache-2.0 as an explicit option. | Election of Apache-2.0 insulates downstream binaries from GPL reciprocal conditions. |

---

## 4. Unresolved Assumptions & Ongoing Verification Steps

1. **Production Artifact Verification:** A post-build inspection of the final signed `.aab` / `.apk` (using tools like `apkanalyzer`) must verify that no unanticipated third-party classes or assets were injected during Gradle minification/merging.
2. **Third-Party Attribution in UI:** An automated UI test should confirm that `THIRD_PARTY_NOTICES.md` or equivalent license texts are accessible from the in-app About / Legal screen.
3. **Continuous CI Gating:** Future dependency updates in `Cargo.lock` must be screened automatically for non-permissive licenses.

---

## 5. Official Reference Standards
- **SPDX Specification 2.3:** https://spdx.dev/specifications/
- **Apache Software Foundation 3rd Party Licensing Policy:** https://www.apache.org/legal/resolved.html
- **Open Source Initiative (OSI) Compliance:** https://opensource.org/compliance
