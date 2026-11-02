# Commercial & Technical Release Compliance Checklist
## Suda Design (Android Edition)

**Document Reference:** REL-CHK-2026-02  
**Status:** DRAFT — OPERATIONAL CHECKLIST PENDING FINAL PRODUCTION SIGN-OFF  
**Applicability:** Protocol to be executed for every production release (`.aab` / `.apk`) submitted to Google Play Console or published on GitHub.

---

## 1. Open-Source & Intellectual Property Verification

- [ ] **Dual-License Upstream Integrity:** Verify that PhotoCraft copyright headers, `LICENSE-MIT`, and `LICENSE-APACHE` in the workspace root remain intact.
- [ ] **Third-Party Attribution Sync:** Run `python scripts/generate_sbom_and_notices.py` to ensure `SBOM.json` and `THIRD_PARTY_NOTICES.md` match the exact dependency tree.
- [ ] **Zero Copyleft Verification:** Inspect the target graph (`cargo tree --target aarch64-linux-android -p photocraft-android`) to confirm zero GPL/AGPL packages are compiled into `libphotocraft_android.so`.
- [ ] **Dual-License Branch Confirmation:** Verify that dual-licensed packages (e.g., `self_cell`) continue to be elected under permissive terms (`Apache-2.0`).
- [ ] **Proprietary Asset Isolation:** Confirm that proprietary brand assets ("Sd" logo, splash artwork) are excluded from open-source MIT/Apache distribution.
- [ ] **In-App Legal Notice Access:** Confirm that the application's About/Legal screen provides readable access to `THIRD_PARTY_NOTICES.md`.

---

## 2. Privacy, Telemetry & Network Verification

- [ ] **Zero Telemetry Verification:** Confirm that no analytics, advertising, or crash reporting SDKs have been added to `Cargo.toml` or `build.gradle.kts`.
- [ ] **Zero Network Transmission Check:** Verify via network packet inspection that the application initiates zero HTTP/HTTPS socket connections upon launch and during offline editing.
- [ ] **Manifest Permission Check:** Verify that `AndroidManifest.xml` contains zero unauthorized permissions.
- [ ] **Storage Access Framework (SAF):** Confirm that all document, photo, and font imports execute exclusively through native Android SAF picker intents (`ACTION_OPEN_DOCUMENT`).
- [ ] **Policy Synchronicity:** Confirm that the live Privacy Policy URL displays the verified text of `legal/users/PRIVACY_POLICY.md`.

---

## 3. Google Play Store Policy Verification

- [ ] **Data Safety Form Matching:** Ensure every answer in Google Play Console matches `legal/users/GOOGLE_PLAY_DATA_SAFETY.md`.
- [ ] **Target SDK Level:** Ensure `compileSdk` and `targetSdk` comply with the current Google Play minimum API level requirements (API level 34 or higher; current project is level 35).
- [ ] **App Bundle Packaging:** Verify that production builds are exported as signed Android App Bundles (`.aab`).
- [ ] **Play App Signing:** Ensure Google Play App Signing is configured securely and upload keystores are stored in secure offline vaults.
- [ ] **Testing Requirements:** Comply with Google Play closed testing requirements applicable to the publisher account type.
- [ ] **Content Rating:** Ensure the IARC content rating questionnaire is completed and confirmed.

---

## 4. Git Repository & Secret Hygiene

- [ ] **No Keystores in Git:** Verify that `*.keystore`, `*.jks`, and `signing.properties` are listed in `.gitignore` and not tracked.
- [ ] **No Personal Credentials:** Verify that passwords, tokens, and personal identity documents are excluded from Git commits.
- [ ] **Clean Working Tree:** Ensure build artifacts (`target/`, `.gradle/`, `build/`) are excluded.

---

## 5. Release Authorization Protocol (Draft Sign-Off)

| Milestone Area | Reviewer Role | Status | Date |
| :--- | :--- | :---: | :--- |
| **Legal & IP Audit** | Legal / Compliance Lead | `DRAFT — REQUIRES REVIEW` | `[PENDING REVIEW]` |
| **Privacy & Data Safety** | Technical Architect | `PENDING TECHNICAL VERIFICATION` | `[PENDING VERIFICATION]` |
| **Store Packaging & Build** | Android Release Engineer | `PENDING RELEASE BUILD` | `[RELEASE DATE]` |
| **Executive Sign-off** | `INDIVIDUAL_OWNER` / Manager | `PENDING RELEASE BUILD` | `[RELEASE DATE]` |

---

## 6. Official References
- **Google Play Console — App Submission Checklist:** https://support.google.com/googleplay/android-developer/answer/9859152
- **Android Developers — Publishing Overview:** https://developer.android.com/studio/publish
