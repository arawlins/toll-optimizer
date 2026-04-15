# Legal Analysis: Toll Optimizer & 407 ETR Terms of Service

This document analyzes the compatibility of the **Toll Optimizer** application with the legal terms and conditions provided by 407 ETR at [https://www.407etr.com/en/legal](https://www.407etr.com/en/legal).

## Executive Summary

Based on the current architecture and functionality of Toll Optimizer (as of April 2026), the application **does not appear to violate** the core legal terms for personal, local use. By avoiding automated web scraping, credential sharing, and centralized data hosting, the tool operates as a local analytical calculator for a user's own data.

---

## 1. Automated Access & Web Scraping
**Relevant Term:** 407 ETR's *Website Terms of Use* prohibit the use of "robots, spiders, scrapers, or other automated means" to access the website or retrieve data.

**Analysis:**
*   **Toll Optimizer Approach:** The application does not connect to the 407 ETR website or API. It requires the user to manually log in to their account, download their statement as a CSV file, and provide that file to the application.
*   **Conclusion:** **No Violation.** The tool does not automate the acquisition of data from 407 ETR’s infrastructure.

## 2. Account Credentials & Security
**Relevant Term:** The *My Account Agreement* strictly prohibits users from sharing their login credentials (username and password) with any third-party application or service.

**Analysis:**
*   **Toll Optimizer Approach:** The application never requests, stores, or utilizes 407 ETR login credentials. It operates entirely on the data contained within the user-provided CSV file.
*   **Conclusion:** **No Violation.** The security of the user's 407 ETR account remains intact and is never compromised by the tool.

## 3. Data Privacy & Custodianship
**Relevant Term:** The *Privacy Notice* states that 407 ETR is not responsible for the security or privacy of data once it is moved to a third-party platform by the user.

**Analysis:**
*   **Toll Optimizer Approach:**
    *   **CLI Mode:** Data is processed entirely in the user's local memory.
    *   **API/Docker Mode:** Data is stored in a local PostgreSQL database within the user's own environment.
*   **Conclusion:** **Compliance via Self-Hosting.** Because the application is designed to be run locally or self-hosted, the user retains full custody of their sensitive trip data. As long as the tool is not hosted as a public multi-tenant service, the developer avoids the liabilities associated with being a third-party data custodian.

## 4. Commercial Use & Derivative Works
**Relevant Term:** The *Terms of Use* prohibit using the website's content for "commercial purposes" without express written consent. They also restrict the creation of "derivative works" based on 407 ETR's proprietary information.

**Analysis:**
*   **Current State:** Toll Optimizer is an open-source (MIT licensed) tool intended for personal analytics. It is not a commercial venture.
*   **Calculation Logic:** The application recreates 407 ETR's tolling rates (e.g., `calculate_cost_2026()`) to suggest savings. While these rates are public knowledge, 407 ETR might argue that a systematic recreation of their pricing models constitutes an unauthorized derivative work if used commercially.
*   **Conclusion:** **Low Risk for Personal Use.** Analyzing one's own billing data against public rates for personal optimization is generally considered fair use. However, commercializing this logic would likely require a formal agreement.

## 5. Accuracy & Official Records
**Relevant Term:** 407 ETR maintains that their systems are the only official record of trips and tolls.

**Analysis:**
*   **Toll Optimizer Approach:** The tool provides comparisons (e.g., `[Calc: $X] [Actual: $Y]`) for informational purposes.
*   **Conclusion:** Users must be aware that the output of Toll Optimizer has no legal standing in disputes with 407 ETR. The tool should be treated as a "best-effort" estimation and optimization guide.

---

## Future Risk Warnings

If the development of Toll Optimizer shifts toward a public SaaS model, the following risks would immediately apply:

1.  **Commercial Violation:** Charging for access or displaying ads would likely violate the "No Commercial Use" clause regarding 407 ETR's data formats and pricing models.
2.  **Privacy Liability:** Hosting other users' trip data (which reveals life patterns and home/work locations) would trigger significant data protection requirements (e.g., PIPEDA in Canada).
3.  **Intellectual Property Claims:** 407 ETR could issue a Cease and Desist (C&D) regarding the unauthorized reproduction of their proprietary tolling algorithms in a public-facing application.

**Recommendation:** Maintain the current "Local-First / Self-Hosted" philosophy to ensure compliance with 407 ETR's legal framework.
