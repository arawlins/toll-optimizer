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

### Monitoring & Observability (Loki Stack)
-   **Grafana**: Visualization and dashboarding ([localhost:3001](http://localhost:3001)).
-   **Prometheus**: Metrics storage ([localhost:9090](http://localhost:9090)).
-   **Loki**: Log aggregation and log-based alerting ([localhost:3100](http://localhost:3100)).
-   **Promtail**: Log shipper (agent that ships Docker logs to Loki).
-   **Alertmanager**: Alert routing to external services (Slack) ([localhost:9093](http://localhost:9093)).
-   Read [monitoring.md](monitoring.md) for more information.

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
A comprehensive strategy combining metrics, structured logs, and automated alerting.
-   **Metrics (Prometheus)**: `axum-prometheus` exposes a `/metrics` endpoint on port `3000`.
    -   Requires **Basic Authentication** (via `METRICS_USERNAME` and `METRICS_PASSWORD`).
    -   Scraped every 15s by the dedicated `prometheus` container.
-   **Structured Logging (Loki)**: `tracing-subscriber` outputs **JSON** to `stdout`.
    -   `promtail` ships these logs from all containers to `loki`.
    -   All handlers instrumented with `#[tracing::instrument]`.
    -   Logs include request IDs, user IDs, and durations.
-   **Alerting (Alertmanager)**:
    -   Triggered by both metric thresholds (Prometheus) and log patterns (Loki).
    -   Sends notifications to a Slack webhook (configured via `SLACK_WEBHOOK_URL`).
    -   Example Alert: "Unknown entry/exit point found in CSV."

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
-   **Monitoring Stack**: `prom/prometheus`, `grafana/loki`, `grafana/promtail`, `prom/alertmanager`, and `grafana/grafana`.
-   **Database Container**: `postgres:16-alpine`.
-   **Frontend Service**: Built via `node:20` and served by the Axum backend.
-   **Orchestration**: Managed via `docker-compose.yml`.
-   **Persistence**: Named volumes for `postgres_data`, `prometheus_data`, `grafana_data`, and `loki_data`.
-   **Healthchecks**: The `db` container includes a healthcheck (`pg_isready`). `loki` and `promtail` use `restart: always` to ensure connectivity.

## 6. Implementation Status
All core modules (Core, CLI, API, Frontend) are fully implemented. The system is production-ready with:
-   **Usage Analysis**: Transponder-centric dashboard with time and distance-based optimizations.
-   **Full Observability Stack**: Metrics (Prometheus), Logs (Loki), and Alerting (Alertmanager).
-   **Persistent Data**: Named volumes for database and monitoring tools.
-   **Automated Tests**: Comprehensive integration testing in `crates/api/tests/` and `crates/core/tests/`.
-   **Documentation**: Detailed API references, architectural guides, and monitoring instructions in `docs/`.

