# Global Legal & Regulatory Compliance Matrix
## Suda Design (Android Edition)

**Document Reference:** COMP-MAT-2026-02  
**Status:** DRAFT — REGULATORY CLASSIFICATION MATRIX (REQUIRES PROFESSIONAL JURISDICTIONAL REVIEW)  
**Target Markets:** Global Distribution via Google Play Store (United States, European Union, United Kingdom, Kingdom of Saudi Arabia, and International)  

---

## 1. Compliance Classification Legend

- **Applicable (APP):** Currently applies to software distribution; actions implemented or in progress.
- **Not Applicable (N/A):** Does not apply under current architecture (e.g., offline-first, no personal data collection, zero financial transactions).
- **Conditional (CND):** Will apply upon activation of future capabilities (e.g., cloud sync, monetization, user accounts).
- **Requires Professional Review (RPR):** Mandates jurisdiction-specific legal or tax counsel review prior to commercial launch or corporate transition.

---

## 2. Regulatory Compliance Matrix

| Regulatory Domain & Law | Target Jurisdiction | Status | Technical Architecture Context & Current Posture | Required Action / Responsible Entity |
| :--- | :--- | :---: | :--- | :--- |
| **Open-Source Copyright & Dual Licensing** | Global | **APP** | Underlying PhotoCraft engine dual-licensed (MIT/Apache 2.0). 359 permissive Android crates. | Notices and SBOM compiled in `legal/open-source/`. Mandatory inclusion in builds. |
| **Export Administration Regulations (EAR) / Cryptography** | United States | **APP** | Standard hashing / compression algorithms (zlib, sha256). Mass market / standard encryption exemption (ECCN 5D992.c). | Complete self-classification record for Google Play export declarations. |
| **US Sanctions & Relocation Compliance** | United States | **RPR** | Founder currently in Sudan, planning transition to Saudi Arabia prior to official corporate launch. | Ensure corporate formation and banking strictly follow US OFAC and export regulations. |
| **GDPR (General Data Protection Regulation)** | European Union / EEA | **APP** | Zero personal data collection, zero remote telemetry in current source code. | Maintain offline privacy disclosures (`legal/users/PRIVACY_POLICY.md`). |
| **European Accessibility Act (EAA)** | European Union | **CND / APP** | EAA requires digital creation tools to support accessibility interfaces (screen readers, high contrast). | AccessKit integration in UI shell (`crates/ui-egui`) addresses accessibility. |
| **Saudi Personal Data Protection Law (PDPL)** | Saudi Arabia | **APP** | Royal Decree M/19 compliance. Data sovereignty maintained locally on device. | Maintain local privacy disclosures. |
| **Saudi ZATCA Tax & Permanent Establishment (PE)** | Saudi Arabia | **RPR** | Managing US LLC operations while residing in Saudi Arabia presents POEM / PE questions under Saudi tax law. | Retain licensed Saudi tax advisor (`legal/company/SAUDI_RESIDENCY_AND_MANAGEMENT_REVIEW.md`). |
| **Wyoming Limited Liability Company Act** | Wyoming, USA | **RPR** | Statutory compliance for `FUTURE_WYOMING_LLC`. Registered agent, annual report ($60 min). | Appoint licensed Registered Agent, adopt Operating Agreement. |
| **US Federal Tax (IRS Form 5472 / 1120)** | United States | **RPR** | 25% foreign-owned US LLC reporting rules apply when reportable transactions exist. | Retain US CPA with international tax expertise. |
| **FinCEN Beneficial Ownership Information (BOI)** | United States | **RPR** | Corporate Transparency Act compliance upon formation of LLC (30 days from creation in 2025+). | File BOI report via FinCEN portal within statutory deadline; monitor court litigation. |
| **Google Play Developer Program Policies** | Global / Google | **APP** | Data Safety declaration, Target API Level 35, Scoped Storage (SAF), testing requirements. | Execute `legal/compliance/RELEASE_COMPLIANCE_CHECKLIST.md`. |
| **Children's Privacy (COPPA / GDPR-K)** | USA / EU | **N/A** | App targets general audience; zero personal data collected. | Declare general audience in Google Play Console. |
| **Payment Services / PCI-DSS** | Global | **N/A** | Current version is 100% free; no payments processed. | Re-evaluate before activating in-app purchases. |

---

## 3. Official References & Regulatory Authorities
- **US Bureau of Industry and Security (BIS) — Export Controls:** https://www.bis.doc.gov/
- **IRS Publication 542 (Corporations & Foreign-Owned Entities):** https://www.irs.gov/
- **FinCEN Beneficial Ownership Information (BOI):** https://www.fincen.gov/boi
- **Zakat, Tax and Customs Authority (ZATCA):** https://zatca.gov.sa/
- **European Commission — GDPR Official Text:** https://eur-lex.europa.eu/eli/reg/2016/679/oj
