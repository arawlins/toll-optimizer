# Toll Optimizer: Web Application Architecture

## 1. Overview
The `toll-optimizer` web application transforms the core analysis logic into a distributed system. It allows users to:
-   Securely upload toll statement CSVs.
-   Receive real-time optimization analysis (time-based and distance-based).
-   Track and visualize historical savings over time.
-   Manage their personal account with JWT authentication.

## 2. Technology Stack

### Backend (Rust)
-   **Web Framework**: `axum` (Modular, async-first, Tokio ecosystem).
-   **Database Client**: `sqlx` (Async, compile-time SQL verification).
-   **Async Runtime**: `tokio`.
-   **Auth**: `argon2` (Password hashing) and `jsonwebtoken` (JWT for session management).
-   **Serialization**: `serde` (JSON).
-   **Observability**: `axum-prometheus` (Metrics), `tracing`, `tracing-subscriber` (Structured JSON Logging).

### Database (PostgreSQL)
-   Standard relational storage for user accounts and historical summary metadata.
-   Minimal state strategy: Individual trip records are **not** persisted.

### Frontend (React)
-   **Build Tool**: `Vite` with `TypeScript`.
-   **State/Data Fetching**: `React Query` (TanStack Query).
-   **Styling**: `Tailwind CSS` + `shadcn/ui`.
-   **Charts**: `Recharts` for visualizing savings and trip clusters.

---

## 3. System Architecture

### Cargo Workspace Structure
The project uses a standard Rust workspace:

```text
toll-optimizer/
├── Cargo.toml (Workspace Root)
├── crates/
│   ├── core/           # Shared logic (lib)
│   ├── cli/            # Local command-line tool (bin)
│   └── api/            # Axum REST API (bin)
├── frontend/           # React SPA
└── docker-compose.yml  # Orchestration
```

### Database Schema
#### Table: `users`
| Column | Type | Constraints |
| :--- | :--- | :--- |
| `id` | UUID | Primary Key |
| `email` | VARCHAR | Unique, Indexed |
| `password_hash`| VARCHAR | Argon2 hash |
| `created_at` | TIMESTAMPTZ | Default now() |

#### Table: `upload_summaries`
| Column | Type | Constraints |
| :--- | :--- | :--- |
| `id` | UUID | Primary Key |
| `user_id` | UUID | Foreign Key -> users.id |
| `filename` | VARCHAR | User reference |
| `total_trips` | INTEGER | |
| `cost_actual` | DECIMAL | Total recorded cost |
| `cost_optimized`| DECIMAL | Total optimized cost |
| `savings` | DECIMAL | cost_actual - cost_optimized |
| `uploaded_at` | TIMESTAMPTZ | Default now() |

---

## 4. API Design

### Observability & Monitoring
A hybrid strategy combining metrics and structured logs.
-   **Metrics (Prometheus)**: `axum-prometheus` exposes a `/metrics` endpoint on port `3000`.
    -   Requires **Basic Authentication** (via `METRICS_USERNAME` and `METRICS_PASSWORD`).
-   **Structured Logging**: `tracing-subscriber` outputs **JSON** to `stdout`.
    -   All handlers instrumented with `#[tracing::instrument]`.
    -   Logs include request IDs, user IDs, and durations.

### Authentication
-   `POST /auth/register`: User registration.
-   `POST /auth/login`: Authentication, returns JWT and user profile.

### Analysis & History
Endpoints below require `Authorization: Bearer <token>`.

-   **`GET /api/history`**: Returns `upload_summaries` for the user.
-   **`POST /api/analyze`**:
    -   Accepts `multipart/form-data` (CSV file).
    -   Processes CSV in-memory using `core` library.
    -   Performs K-means clustering and pricing optimization.
    -   Returns detailed JSON analysis:
        -   Detailed trips (with suggested entry/exit).
        -   Trip clusters (time and distance).
        -   Potential savings.

---

## 5. Deployment Strategy (Docker)

-   **Backend Container**: Multi-stage `rust:bookworm` -> `debian:bookworm-slim`.
-   **Database Container**: `postgres:16-alpine`.
-   **Frontend Service**: Served via the Axum backend or dedicated web server.
-   **Orchestration**: Managed via `docker-compose.yml`.

## 6. Implementation Status
All core modules (Core, CLI, API, Frontend) are fully implemented. The system is production-ready with structured logging, metrics protection, and a transponder-centric UI dashboard.
