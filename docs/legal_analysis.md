# Legal Analysis: Toll Optimizer CLI & 407 ETR Terms of Service

This document analyzes the compatibility of the **Toll Optimizer CLI** with the legal terms and conditions provided by 407 ETR.

## Executive Summary

The pivot to a **standalone CLI tool** significantly strengthens the project's legal position compared to a hosted web application. By operating as a local-only analytical calculator for a user's own data, the tool avoids the primary risks associated with automated access, data custodianship, and commercialization.

---

## 1. Automated Access & Web Scraping
**Relevant Term:** 407 ETR's terms prohibit "robots, spiders, scrapers, or other automated means" to access their infrastructure.

**Analysis:**
- **Status:** **Fully Compliant.**
- **Details:** The CLI does not connect to 407 ETR servers. It requires a manually downloaded CSV file provided by the user.

## 2. Account Credentials & Security
**Relevant Term:** The *My Account Agreement* prohibits sharing credentials with third-party services.

**Analysis:**
- **Status:** **Fully Compliant.**
- **Details:** The CLI never asks for or handles usernames or passwords. All analysis is derived from billing data already in the user's possession.

## 3. Data Privacy & Custodianship (The CLI Advantage)
**Relevant Term:** PIPEDA and 407 ETR's privacy policies govern the handling of Personally Identifiable Information (PII).

**Analysis:**
- **Status:** **Optimal Safety.**
- **Details:** By removing the web app and database, the developer is **never a data custodian.** Trip data (which reveals sensitive home/work locations and patterns) stays entirely on the user's local machine. This eliminates the legal liability associated with data breaches or unauthorized aggregation of 407 ETR customer data.

## 4. Commercial Use
**Relevant Term:** Prohibits using content for commercial purposes without consent.

**Analysis:**
- **Status:** **Low Risk.**
- **Details:** The tool is open-source (MIT) and intended for personal use. As long as it is not sold as a service, analyzing one's own data against public toll rates is generally considered fair use.

## 5. Accuracy & Official Records
**Relevant Term:** 407 ETR systems are the only official record.

**Analysis:**
- **Status:** **Information Only.**
- **Details:** The tool provides "best-effort" estimates. It includes a disclaimer in the output (especially in `--verbose` mode) that results are informational and have no legal standing in disputes.

---

## Conclusion

The **Local-First CLI** model is the safest architecture for this project. It respects 407 ETR's intellectual property while empowering users to analyze their own data without compromising their privacy or violating their account agreements.
