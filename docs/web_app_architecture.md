# Toll Optimizer: Web Application Architecture

## 1. Overview
The goal is to transform the `toll-optimizer` CLI tool into a distributed web application. The system will allow users to upload toll statement CSVs, receive an optimization analysis in real-time, and track their historical savings.

## 2. Technology Stack

### Backend (Rust)
*   **Web Framework**: `axum` (Modular, async-first, Tokio ecosystem).
*   **Database Client**: `sqlx` (Async, compile-time SQL verification).
*   **Async Runtime**: `tokio`.
*   **Auth**: `argon2` (Password hashing) and `jsonwebtoken` (JWT for session management).
*   **Serialization**: `serde` (JSON).
*   **Observability**: `tracing`, `tracing-subscriber`, `opentelemetry` (OTLP exporter).

### Database (PostgreSQL)
*   Standard relational storage for user accounts and historical summary metadata.
*   Minimal state strategy: Individual trip records are **not** persisted.

### Frontend (React)
*   **Build Tool**: `Vite` with `TypeScript`.
*   **State/Data Fetching**: `React Query` (TanStack Query).
*   **Styling**: `Tailwind CSS` + `shadcn/ui`.
*   **Charts**: `Recharts` for visualizing savings and trip clusters.

---

## 3. System Architecture

### Cargo Workspace Structure
To share logic between the CLI and the Web API, the project will be refactored into a workspace:

```text
toll-optimizer/
├── Cargo.toml (Workspace Root)
├── crates/
│   ├── core/           # Shared logic (lib)
│   │   ├── src/lib.rs
│   │   ├── parser.rs   # Generic stream parsing
│   │   ├── analyzer.rs # Pure calculation logic
│   │   └── models.rs   # Shared data structures
│   ├── cli/            # Refactored CLI tool (bin)
│   └── api/            # Axum REST API (bin)
├── frontend/           # React SPA
└── docker/             # Dockerfiles and config
```

### Database Schema
Minimalist design to support privacy and performance.

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
The API will be instrumented to provide visibility into request latency, error rates, and database performance.
*   **Instrumentation**: All handlers and core logic will be instrumented with `#[tracing::instrument]`.
*   **Infrastructure**: `tracing-subscriber` will be configured to export traces via OTLP (OpenTelemetry Protocol).
*   **Middleware**: `tower_http::trace::TraceLayer` will automatically record HTTP request/response lifecycles.
*   **Correlation**: Request IDs will be propagated to log entries for easier debugging.

### Authentication
*   `POST /auth/register`: Create user.
*   `POST /auth/login`: Returns JWT and user profile.

### Analysis & History
All endpoints below require a valid `Authorization: Bearer <token>` header.

*   **`GET /api/history`**: Returns a list of `upload_summaries` for the authenticated user.
*   **`POST /api/analyze`**:
    *   Accepts `multipart/form-data` (CSV file).
    *   Server parses CSV using `core` library in-memory.
    *   Performs K-means clustering and pricing optimization.
    *   Saves a summary row to `upload_summaries`.
    *   Returns full detailed JSON analysis (detailed trips, clusters, and savings).

---

## 5. Deployment Strategy (Docker/Linux)

### Containerization
*   **Multi-stage Dockerfile**:
    1.  `rust:bookworm` for compiling the backend binaries.
    2.  `node:20` for building the React frontend.
    3.  `debian:bookworm-slim` for the final runtime image.
*   The Rust binary will serve the static React files from the `/dist` directory.

### Orchestration
*   `docker-compose.yml` defining:
    *   `app`: The integrated Rust/React container.
    *   `db`: Official PostgreSQL image with persistent volume.

---

## 6. Implementation Phases

1.  **Phase 1 (Workspace & Core)**: Extract `src/` logic into `crates/core`. Refactor parser to handle `BufRead` instead of just local files.
2.  **Phase 2 (Database Setup)**: Initialize `sqlx` migrations and schema.
3.  **Phase 3 (Backend API)**: Implement Axum routes, JWT middleware, and the `/analyze` endpoint using the `core` library.
4.  **Phase 4 (Frontend Scaffold)**: Create Vite project, set up API clients, and implement the Auth flow.
5.  **Phase 5 (Analysis UI)**: Build the upload dashboard and visualization charts for the analysis results.
6.  **Phase 6 (Production)**: Configure Docker and test the deployment.
