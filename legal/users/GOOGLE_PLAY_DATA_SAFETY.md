# Google Play Console — Data Safety Form Responses
## Suda Design (Android Application)

**Document Reference:** GP-DATA-SAFE-2026-02  
**Status:** DRAFT — PROVISIONAL RESPONSES SUBJECT TO VERIFICATION AGAINST PRODUCTION RELEASE ARTIFACT  
**Package Identifier:** `ai.storyteller.photocraft` (or designated commercial ID)  
**Target API Level:** `35` (Android 15) | **Min API Level:** `26` (Android 8.0)  
**Form Requirement:** Google Play Data Safety Section  

---

> [!IMPORTANT]
> **PROVISIONAL MAPPING NOTICE**  
> The answers below are derived from a source code and dependency audit of the repository as of 2026-11-01. These answers must be re-verified against the compiled production release App Bundle (`.aab`) prior to submitting the Data Safety form in Google Play Console.

---

## 1. High-Level Questionnaire Responses

### Question 1: "Does your app collect or share any of the required user data types?"
- **Provisional Answer:** **No**
- **Technical Basis:** The application contains zero network transmission logic, zero user accounts, zero analytics SDKs, and zero ad networks.

### Question 2: "Is all of the user data collected by your app encrypted in transit?"
- **Provisional Answer:** **Not Applicable (Data is not collected or transmitted off-device)**
- *Note:* If Google Play requires a binary selection, select **"Yes"** and document that no off-device data transfer occurs.

### Question 3: "Do you provide a way for users to request that their data be deleted?"
- **Provisional Answer:** **Not Applicable (No remote user accounts or server databases exist)**
- **Explanation:** User files are stored entirely within local device storage and may be deleted directly by the user using the native Android file manager or by uninstalling the application.

---

## 2. Category-by-Category Data Safety Mapping

| Data Type Category | Collected? | Shared? | Technical Verification Notes |
| :--- | :---: | :---: | :--- |
| **Location** | **No** | **No** | No location APIs requested or linked. |
| **Personal Info** (Name, Email, Phone, ID) | **No** | **No** | No account creation or contact collection code. |
| **Financial Info** | **No** | **No** | No payment or billing libraries integrated. |
| **Health and Fitness** | **No** | **No** | Not applicable. |
| **Messages** (SMS/Email) | **No** | **No** | Not applicable. |
| **Photos and Videos** | **No\*** | **No** | \*Under Google Play guidance, opening a local photo via Storage Access Framework without uploading or sharing off-device is **not** treated as data collection. |
| **Files and Documents** | **No\*** | **No** | \*Opening project files or fonts locally via SAF is not off-device collection. |
| **App Activity** (Interactions/Searches) | **No** | **No** | No user tracking or behavioral telemetry. |
| **App Info & Performance** (Crash logs) | **No** | **No** | No third-party crash reporting SDKs linked. |
| **Device Identifiers** (Android ID/IMEI) | **No** | **No** | No device tracking SDKs included. |

---

## 3. Verified Android Manifest Permissions Check

Inspection of `apps/photocraft-android/android/app/src/main/AndroidManifest.xml` confirms:
- **Zero permissions declared:** No `<uses-permission>` tags are currently present in the manifest.
- Specifically:
  - `android.permission.INTERNET` is **NOT** declared.
  - `android.permission.ACCESS_FINE_LOCATION` is **NOT** declared.
  - `android.permission.READ_EXTERNAL_STORAGE` is **NOT** declared (file access uses SAF).
  - `android.permission.CAMERA` is **NOT** declared.

---

## 4. App Content & Declaration Prerequisites
1. **Target Audience:** General Audience (Age 13+ or 18+).
2. **Ads:** Select **"No, my app does not contain ads"**.
3. **News / Financial / COVID-19:** Select **"No"** for all specialized government and financial categories.

---

## 5. Official References & Google Play Guidelines
- **Google Play Help — Data Safety Section:** https://support.google.com/googleplay/android-developer/answer/10787469
- **Google Play Console — Permissions & APIs:** https://support.google.com/googleplay/android-developer/answer/9214102
