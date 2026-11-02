# Privacy Policy
## Suda Design (Android Edition)

**Document Status:** DRAFT — PENDING TECHNICAL VERIFICATION AGAINST PRODUCTION RELEASE ARTIFACT  
**Effective Date:** `[PROVISIONAL DRAFT — 2026-11-01]`  
**Last Updated:** 2026-11-01  
**Publisher:** Suda Design (`INDIVIDUAL_OWNER`; designated for transfer to future operating entity)  
**Official Contact:** `[PENDING PUBLISHER CONTACT SETUP — e.g., privacy@sudadesign.app upon domain activation]`  
**Official Corporate Address:** `[PENDING OFFICIAL REGISTERED ADDRESS]`  
**Hosted Policy URL:** `[PENDING PUBLIC POLICY HOSTING URL]`  

---

## 1. Overview and Privacy Posture

Suda Design is designed as an offline-first graphic and image editing tool for Android devices. This Privacy Policy discloses the data handling practices of the application based on an architectural review of the codebase.

> [!NOTE]
> **Provisional Audit Note:** The statements below reflect the verified source code in this repository. Final verification of runtime network activity and data transmission must be performed on the compiled production release artifact (`.aab`) prior to distribution.

---

## 2. Technical Audit of Data Handling

An inspection of the application manifest (`apps/photocraft-android/android/app/src/main/AndroidManifest.xml`) and dependency graph indicates:

| Data Category | Technical Finding (Source Audit) | Verified Behavior |
| :--- | :--- | :--- |
| **Personal Identifiers** | No account or registration code exists in the repository. | No name, email, phone number, or ID is collected. |
| **Account Creation** | Zero authentication or user login systems. | Operates without user credentials or profiles. |
| **Analytics & Telemetry** | Zero third-party telemetry SDKs (no Firebase, Mixpanel, Flurry). | No user interaction or behavioral tracking. |
| **Crash Reporting SDKs** | Zero third-party crash reporters (no Sentry, Crashlytics, Bugsnag). | Diagnostic logs stay on the local device. |
| **Advertising Networks** | Zero advertising SDKs (no AdMob, Unity, AppLovin). | No advertisements are displayed; no ad IDs accessed. |
| **Network Communication** | The Android manifest omits `android.permission.INTERNET`. | Core editing operates without internet connectivity. |

---

## 3. Local Handling of User Files & Content

1. **Local Processing:** All image processing, layer compositing, raster filtering, brush rendering, and font parsing are performed locally on your device's CPU and GPU.
2. **Storage Access Framework (SAF):**
   - The application uses the Android native Storage Access Framework (`ACTION_OPEN_DOCUMENT`, `ACTION_CREATE_DOCUMENT`).
   - Suda Design only accesses the specific image or font file(s) that you explicitly select via the system picker.
   - The application does not scan, index, or access your broader photo gallery or internal file storage.
3. **No Cloud Uploads:** Your creative artwork, imported images, and exported files are saved to your local storage and are never uploaded to any remote server by the application.

---

## 4. International Privacy Standards Alignment

### 4.1. European Union (GDPR)
Under Regulation (EU) 2016/679 (GDPR), because Suda Design does not transmit personal data to external servers, no remote personal data processing or controller/processor transfer is conducted by the publisher.

### 4.2. California Consumer Privacy Act (CCPA / CPRA)
Suda Design does not collect, sell, or share personal information as defined under the California Consumer Privacy Act (Cal. Civ. Code § 1798.100 et seq.).

### 4.3. Saudi Arabian Personal Data Protection Law (PDPL)
Under Royal Decree No. M/19 and its implementing regulations, personal data sovereignty is maintained because creative content and files remain exclusively within the user's local device control.

### 4.4. Children's Privacy (COPPA)
Suda Design does not knowingly collect personal information from children under the age of 13.

---

## 5. Security & User Responsibilities

Because files are stored locally on your device, their security depends on your device-level security (PIN, biometrics, screen lock, and OS security patches). Users are encouraged to maintain independent backups of critical creative work.

---

## 6. Updates to this Policy

This policy will be updated before introducing any network-enabled capabilities, account features, or cloud synchronization.

---

## 7. Official References
- **Google Play Developer Policy on Privacy, Security and Deception:** https://play.google.com/about/privacy-security-deception/
- **European Data Protection Board (EDPB) Guidelines:** https://edpb.europa.eu/
- **Saudi Authority for Data and Artificial Intelligence (SDAIA) — PDPL:** https://sdaia.gov.sa/
