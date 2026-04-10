# Toll Optimizer: Architecture Analysis

## Executive Summary

The toll-optimizer is a full-stack web application with a Rust backend (Axum REST API, CLI), React/TypeScript frontend, PostgreSQL database, and a full observability stack (Prometheus, Loki, Grafana). The architecture documentation in `web_app_architecture.md` is largely accurate, but the implementation has **one critical compilation blocker**, several **security vulnerabilities**, and notable gaps in test coverage and robustness.

---

## 1. Architecture Conformance

The documented architecture is well-realized overall:

| Component | Documented | Implemented | Status |
|---|---|---|---|
| Rust Axum backend | Yes | Yes | Conforms |
| JWT authentication | Yes | Yes | Conforms |
| Argon2 password hashing | Yes | Yes | Conforms |
| sqlx async DB client | Yes | Yes | Conforms |
| React + Vite + TypeScript | Yes | Yes | Conforms |
| React Query | Yes | Yes | Conforms |
| Tailwind + shadcn/ui | Yes | Yes | Conforms |
| Recharts | Yes | Yes | Conforms |
| Prometheus metrics (`/metrics`) | Yes | Yes | Conforms |
| Metrics endpoint basic auth | Yes | Yes | Conforms |
| Structured JSON logging (tracing) | Yes | Yes | Conforms |
| Loki/Promtail log shipping | Yes | Yes | Conforms |
| Alertmanager → Slack | Yes | Yes | Conforms |
| Multi-stage Docker build | Yes | Yes | Conforms |
| Minimal state (no trip persistence) | Yes | Yes | Conforms |
| K-means clustering | Yes | Yes | Conforms |
| In-memory CSV processing | Yes | Yes | Conforms |

The main discrepancy: the doc states the system is "production-ready," but the issues below contradict that.

---

## 2. Rust Edition

**File:** All `Cargo.toml` files

All crates declare `edition = "2024"`, which is valid — the Rust 2024 edition was stabilized in Rust 1.85 (February 2025). No action required.

---

## 3. Security Vulnerabilities

### 3.1 CORS Too Permissive
**File:** `crates/api/src/main.rs`

`CorsLayer::permissive()` accepts requests from any origin. This enables cross-site request forgery and should never be used in production.

**Fix:** Replace with `CorsLayer::new().allow_origin("https://yourdomain.com".parse::<HeaderValue>()?)`.

### 3.2 Auth Token Stored in localStorage
**File:** `frontend/src/store.ts`

The JWT and user object are persisted in `localStorage`, which is accessible to any JavaScript on the page. A single XSS vulnerability anywhere in the frontend gives an attacker full account access.

**Fix:** Either use `HttpOnly` cookies (requires backend change to `Set-Cookie` on login) or, at minimum, store only in session storage and never persist to disk.

### 3.3 No Rate Limiting on Auth Endpoints
**Files:** `crates/api/src/handlers/auth.rs`, `crates/api/src/main.rs`

`POST /auth/login` and `POST /auth/register` have no rate limiting. Unlimited brute-force attempts are possible.

**Fix:** Add a rate-limiting middleware (e.g., `tower-governor`) on the auth router.

### 3.4 User Enumeration
**File:** `crates/api/src/handlers/auth.rs`

Registration returns "User already exists" for duplicate emails, and login likely returns different errors for unknown users vs. wrong passwords. This allows enumeration of registered email addresses.

**Fix:** Return a generic `"Invalid credentials"` message for all login failures. Registration can acknowledge duplicate email (it's a UX choice), but should be documented as a trade-off.

### 3.5 Default Credentials for Metrics Endpoint
**File:** `crates/api/src/auth/mod.rs` (lines 24–25)

If `METRICS_USERNAME` or `METRICS_PASSWORD` env vars are not set, the application falls back to hardcoded credentials (`"admin"` / `"secret"`).

**Fix:** Replace the fallback with a hard failure: `std::env::var("METRICS_USERNAME").expect("METRICS_USERNAME must be set")`.

### 3.6 No HTTPS Enforcement
The Docker Compose setup has no TLS termination. Credentials and tokens are transmitted in plaintext if deployed without a reverse proxy.

**Fix:** Add a TLS-terminating reverse proxy (nginx/Traefik) to the Compose stack, or document this as a required external dependency.

### 3.7 Loki Unauthenticated
**File:** `monitoring/loki/loki-config.yml`

`auth_enabled: false` with Loki exposed on port 3100. Anyone on the network can read all application logs.

**Fix:** Enable Loki authentication or ensure the port is not externally exposed in production.

---

## 4. Core Logic Bugs

### 4.1 Time Parsing Rejects 12 AM
**File:** `crates/core/src/trip_analyzer.rs`

The time parser condition `if hour == 0 || hour > 12` treats hour `0` as invalid. `00:xx` is valid midnight (12 AM), so trips occurring at midnight are silently dropped.

**Fix:** Change the condition to `if hour > 12` (hour `0` in 24-hour format is valid midnight).

### 4.2 CSV Parser is Fragile
**File:** `crates/core/src/csv_parser.rs` (line 44)

Parsing uses a simple `split("\",\"")` instead of a proper CSV library. This breaks on any field that contains a comma, an escaped quote, or inconsistent quoting.

Additionally, parts are accessed by magic index (`parts[4]`, `parts[5]`, etc.) with no bounds check. A row with fewer than 10 fields will panic.

**Fix:** Introduce the `csv` crate. Replace index access with named field extraction and add a length guard before indexing.

### 4.3 Potential Off-by-One in Westbound Cost Calculation
**File:** `crates/core/src/trip_analyzer.rs`

The westbound cost path accesses `ACCESS_POINTS[i+1]` in a loop without checking if `i+1` is within bounds. A trip exiting at the last access point could cause an out-of-bounds panic.

**Fix:** Add an explicit bounds check before accessing `ACCESS_POINTS[i+1]`.

### 4.4 Holiday List Expires After 2026
**File:** `crates/core/src/trip_analyzer.rs`

Holiday classification has hardcoded dates for 2024–2026 only. Data from 2023 or 2027+ will be misclassified as weekdays.

**Fix:** Extend the list or, ideally, replace it with a configurable source.

### 4.5 Typo in Constant Name
**File:** `crates/core/src/constants.rs`

`OLD_ACCESSS_POINTS` has a double `S`. While a minor cosmetic issue, it makes grep/search unreliable.

**Fix:** Rename to `OLD_ACCESS_POINTS`.

---

## 5. Database Issues

### 5.1 DECIMAL Precision May Overflow
**File:** `crates/api/migrations/20260126000000_init_schema.sql`

`cost_actual`, `cost_optimized`, and `savings` are `DECIMAL(10, 2)`, allowing a maximum of `$99,999.99`. A user with thousands of high-cost trips could exceed this and receive a database error.

**Fix:** Increase to `DECIMAL(12, 2)` or `DECIMAL(15, 2)`.

### 5.2 No Pagination on History Endpoint
**File:** `crates/api/src/db/summary.rs`

`get_summaries_by_user()` returns every row for a user with no `LIMIT`. A user with thousands of uploads causes an unbounded query and a large JSON response.

**Fix:** Add `LIMIT` and `OFFSET` parameters to the query and expose them as query params on `GET /api/history`.

### 5.3 No Constraints on Numeric Values
**File:** `crates/api/migrations/20260126000000_init_schema.sql`

The schema has no `CHECK` constraints to enforce that `cost_actual`, `cost_optimized`, and `total_trips` are non-negative. Malformed analysis results could insert nonsensical data.

**Fix:** Add `CHECK (cost_actual >= 0)`, `CHECK (cost_optimized >= 0)`, `CHECK (total_trips >= 0)`.

---

## 6. Frontend Issues

### 6.1 Expired Token Not Detected by AuthGuard
**File:** `frontend/src/components/AuthGuard.tsx`

The guard only checks that a token string exists in the store — it does not verify the JWT expiration claim (`exp`). A user with an expired token is admitted to the dashboard and will only discover the failure when an API call returns 401.

**Fix:** Decode the JWT payload client-side (no signature verification needed here) and redirect to login if `exp` is in the past.

### 6.2 No File Size Validation on Upload
**File:** `frontend/src/components/UploadDropzone.tsx`

The client validates file extension (`.csv`) but not file size. Selecting a large file provides no feedback until the server's 10 MB body limit rejects it.

**Fix:** Add a client-side size check (e.g., `file.size > 10 * 1024 * 1024`) before submission.

### 6.3 API Base URL Falls Back to localhost
**File:** `frontend/src/lib/api.ts`

`import.meta.env.VITE_API_URL || 'http://localhost:3000'` silently falls back to `localhost` if the env var is not set at build time, causing production builds to fail in non-obvious ways.

**Fix:** Remove the fallback and let the build fail fast with a clear message if `VITE_API_URL` is not set.

### 6.4 Loose TypeScript Typing
**File:** `frontend/src/pages/Dashboard.tsx`

`analysis.time_analysis` and `distance_analysis` are typed as `any[]`. This eliminates type safety for the most important data structure in the application.

**Fix:** Define explicit TypeScript interfaces matching the `AnalysisResponse` schema from the backend.

---

## 7. Infrastructure Issues

### 7.1 Docker Container Runs as Root
**File:** `Dockerfile`

The runtime stage does not create or switch to a non-root user. If the application is compromised, the attacker has root inside the container.

**Fix:** Add `RUN useradd -m -u 1000 appuser` and `USER appuser` before the `CMD` in the runtime stage.

### 7.2 No Health Check in Dockerfile
**File:** `Dockerfile`

The `Dockerfile` has no `HEALTHCHECK` instruction. Docker (and orchestrators like ECS/Kubernetes) cannot detect if the application inside the container has crashed.

**Fix:** Add `HEALTHCHECK CMD curl -f http://localhost:3000/health || exit 1`.

### 7.3 No Prometheus Alert Rules
**File:** `monitoring/prometheus/prometheus.yml`

The Prometheus config has no `rule_files` section. The architecture doc references metric-based alerting, but no alert rules are defined. Alertmanager is configured for routing but has nothing to route.

**Fix:** Create a `monitoring/prometheus/rules/` directory with alert rule files (e.g., high error rate, unknown access point warnings) and reference them with `rule_files`.

### 7.4 Environment Variable Substitution via sed is Fragile
**File:** `docker-compose.yml`

The `entrypoint` for Prometheus and Alertmanager uses `sed` to substitute `__PLACEHOLDER__` strings. If any environment variable value contains a forward slash or special regex character, the substitution silently corrupts the config.

**Fix:** Use Docker secrets or a proper templating tool (e.g., `envsubst`) instead of `sed`.

---

## 8. Test Coverage Gaps

The existing tests (`crates/api/tests/`, `crates/core/tests/`) cover happy paths and a few security smoke tests. The following scenarios are not tested:

**Core logic (high priority):**
- Midnight trip time parsing (the 12 AM bug above is undetected by tests)
- Weekday vs. weekend pricing differences
- Year boundary (2025 pricing vs. 2026 pricing)
- Holiday vs. non-holiday classification
- Westbound zone boundary trips (off-by-one risk)
- Malformed CSV with fewer than 10 columns

**API (high priority):**
- Requests with expired JWTs
- Requests with tampered JWT signatures
- Requests missing the `Authorization` header on protected endpoints
- Duplicate email registration
- Upload of non-CSV content despite `.csv` extension

**Database:**
- `get_summaries_by_user` isolation (user A cannot see user B's data)
- Cost value at DECIMAL boundary

---

## 9. Code Quality Observations

- **`expect()` on startup config** (`crates/api/src/main.rs`): Panicking on missing env vars is acceptable at startup, but the error messages should be descriptive enough to guide operators.
- **`_claims` parameter** (`crates/api/src/handlers/analyze.rs:16`): The claims are extracted for authentication but never used for authorization logic (e.g., associating the upload with the authenticated user). The user ID for the DB insert comes from a separate extraction. This is fine, but the underscore naming convention is misleading — it implies unused when the extraction itself is the side effect.
- **SQLX_OFFLINE=true in Dockerfile**: Compile-time query verification is one of sqlx's main features. Building with `SQLX_OFFLINE=true` without the accompanying `.sqlx` metadata cache disables that verification entirely. Either generate `.sqlx` files and commit them, or document this as a known gap.
- **`max_connections: 5`** (`crates/api/src/main.rs`): Appropriate for development. Should be made configurable via environment variable for production tuning.

---

## 10. Priority Action List (Implementation Status)

**P1 — Fix before production deployment:**
1. [x] Implement rate limiting on `/auth/login` and `/auth/register`.
2. [x] Replace `CorsLayer::permissive()` with explicit allowed origins.
4. [ ] Move JWT storage from `localStorage` to `HttpOnly` cookie. (Currently in `sessionStorage`).
5. [x] Fix the midnight time parsing bug (`hour == 0` rejection). Verified with tests.
6. [x] Add bounds check to the westbound off-by-one risk.
7. [x] Replace the fragile CSV `split()` with the `csv` crate.

**P2 — Fix before public release:**
8. [x] Add pagination to `GET /api/history`.
9. [x] Increase `DECIMAL(10, 2)` to `DECIMAL(15, 2)` in the schema.
10. [x] Add `HEALTHCHECK` to `Dockerfile` and run as non-root user.
11. [x] Define Prometheus alert rules.
12. [x] Enable Loki authentication.
13. [x] Fix expired-token detection in `AuthGuard.tsx`.
14. [x] Document SQLX verification gap.

**P3 — Improve robustness:**
15. [x] Add test coverage for edge cases listed in Section 8 (Midnight, Duplicate Email, Isolation, etc).
16. [x] Define TypeScript interfaces for `AnalysisResponse`.
17. [x] Extend the holiday list beyond 2026 (Extended to 2028).
18. [x] Add database `CHECK` constraints for non-negative financial values.
