# Legal & Regulatory Risk Register

**Product:** Suda Design  
**Document Ref:** RISK-REG-2026-02  
**Last Updated:** 2026-11-01  
**Status:** DRAFT — RISK MONITORING REGISTER (REQUIRES ONGOING REVIEW)  

---

## 1. Risk Scoring Methodology

- **Severity (S):** 1 (Low) to 5 (Catastrophic / Severe Financial Penalty or Injunction)
- **Likelihood (L):** 1 (Rare) to 5 (Almost Certain / Inherent to Structure)
- **Risk Score (R):** Severity × Likelihood (1 to 25)

---

## 2. Priority Risk Register

| ID | Risk Category | Risk Description | S | L | R | Mitigation Strategy & Safeguards | Responsible Party |
| :--- | :--- | :--- | :---: | :---: | :---: | :--- | :--- |
| **RSK-01** | **US Tax Compliance (Form 5472)** | For a foreign-owned US single-member LLC, failure to timely file IRS Form 5472 when reportable transactions exist carries a statutory penalty of $25,000 per violation under IRC § 6038A. | 5 | 3 | **15** | Engage a qualified US international tax CPA upon formation to evaluate capital contributions, formation fees, and file Form 5472 with pro forma 1120 if reportable transactions occur. | US CPA / LLC Manager |
| **RSK-02** | **Saudi Management & PE** | Managing US LLC operations while residing in Saudi Arabia could trigger central management (POEM) or Permanent Establishment (PE) scrutiny under ZATCA rules. | 4 | 3 | **12** | Consult a licensed Saudi tax advisor to review management structures, board resolutions, and local commercial licensing prior to commercial launch. | Saudi Tax Advisor / Owner |
| **RSK-03** | **Google Play Address Verification** | Google Play Organization verification frequently rejects commercial registered agent addresses or mail forwarding services, demanding utility bills or physical lease agreements in the company's name. | 4 | 3 | **12** | Prepare for verification challenges; obtain corporate bank statements showing legal address, and evaluate physical business address solutions if required by Google. | Corporate Organizer |
| **RSK-04** | **Trademark Clearance** | Launching commercially as "Suda Design" without a professional clearance search risks conflicting with prior registrations in Class 9 or 42. | 4 | 3 | **12** | Conduct comprehensive USPTO and SAIP trademark clearance searches through a licensed trademark attorney before public commercial launch. | Trademark Attorney |
| **RSK-05** | **Contributor IP Contamination** | An external contractor or contributor introducing copyleft (GPL) code or proprietary third-party code without proper authorization. | 4 | 2 | **8** | Require signed Developer IP Agreements (`DEVELOPER_IP_AGREEMENT.md`) for all paid work and enforce automated SBOM screening in CI. | Lead Architect |
| **RSK-06** | **FinCEN BOI Reporting** | Non-compliance with the Corporate Transparency Act beneficial ownership reporting within 30 days of LLC creation (subject to ongoing court challenges). | 3 | 2 | **6** | File BOI report directly via FinCEN portal upon formation, monitoring active court rulings. | LLC Manager |
| **RSK-07** | **Open Source Attribution Breach** | Omitting required copyright notices or licenses for one of the 359 compiled Rust crates or bundled fonts. | 2 | 2 | **4** | Automatically bundle `legal/open-source/THIRD_PARTY_NOTICES.md` into the in-app About / Legal screen. | Android Build Engineer |

---

## 3. Official References & Authorities
- **Internal Revenue Code § 6038A (Information with respect to certain foreign-owned corporations):** https://www.irs.gov/
- **Wyoming Limited Liability Company Act (W.S. 17-29-101 et seq.):** https://wyoleg.gov/
- **ZATCA Saudi Income Tax Law:** https://zatca.gov.sa/
- **Google Play Developer Account Verification Guidelines:** https://support.google.com/googleplay/android-developer/answer/13962645
