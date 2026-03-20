# Toll Optimizer

Toll Optimizer is a high-performance Rust-based system designed to analyze 407 ETR (Electronic Toll Route) statements. it identifies patterns in your travel and suggests optimizations based on **time** (shifting trips to cheaper timeslots) and **distance** (adjusting entry/exit points) to reduce your total toll charges.

## Project Structure

- **`crates/core`**: Core logic for parsing CSVs, calculating tolls (including 2026 rates), and clustering trips using K-Means.
- **`crates/api`**: Axum-based REST API for multi-user analysis, history tracking, and authentication.
- **`crates/cli`**: Command-line interface for local CSV analysis.
- **`frontend`**: React (TypeScript) + Vite dashboard for visualizing savings and managing uploads.

## Prerequisites

- **Rust**: Latest stable version.
- **Node.js & npm**: For the frontend.
- **Docker**: For the PostgreSQL database.
- **PostgreSQL Client** (optional): For manual schema inspection.

## Quick Start (Production / Docker)

The easiest way to run the full application is using Docker Compose.

### 1. Setup Environment
Copy `env.txt` to `.env` in the root directory and set the values.

### 2. Start the System
```bash
docker-compose up --build -d
```
The application will automatically:
- Start a PostgreSQL database.
- Run database migrations.
- Build the React frontend.
- Start the Axum API serving both the backend and frontend.

### 2. Access the Dashboard
- **Toll Optimizer UI**: [http://localhost:3000](http://localhost:3000)
- **Grafana Dashboard**: [http://localhost:3001](http://localhost:3001) (Default: `admin` / `admin`)
- **Prometheus Metrics**: [http://localhost:9090](http://localhost:9090)
- **Alertmanager**: [http://localhost:9093](http://localhost:9093)

---

## Monitoring & Observability

The system includes a full monitoring stack (Loki stack) for metrics, logs, and alerting:

- **Grafana**: The central dashboard for visualizing metrics and logs. Pre-configured with Prometheus and Loki datasources.
- **Prometheus**: Automatically scrapes quantitative metrics from the API every 15s.
- **Loki**: Aggregates structured JSON logs from all containers.
- **Promtail**: Automatically discovers and ships Docker container logs to Loki.
- **Alertmanager**: Handles alerts triggered by Prometheus metrics or Loki log patterns.

### Querying Logs in Grafana
1. Open Grafana at [http://localhost:3001](http://localhost:3001).
2. Go to **Explore**.
3. Select **Loki** as the datasource.
4. Use LogQL to filter logs, e.g., `{container="toll_optimizer_app"}`.

### Alerting
Alerts are configured for:
- **High Error Rates**: Triggered when "ERROR" appears frequently in logs.
- **Unknown Entry/Exit Points**: Triggered when the CSV parser encounters unrecognized locations, including the specific point name in the alert description.

---

## Development Setup

If you want to run the components separately for development:

### 1. Start the Database
```bash
docker-compose up db -d
```

### 2. Configure Environment
Copy `env.txt` to `.env` in the root directory (or `crates/api`) and set the values.

### 3. Start the Backend API
```bash
cargo run -p toll-optimizer-api
```
The API will be available at `http://localhost:3000`.

### 4. Start the Frontend
```bash
cd frontend
npm install
npm run dev
```
Open [http://localhost:5173](http://localhost:5173) in your browser. Note: In dev mode, the frontend connects to `http://localhost:3000` via Vite's proxy or environment configuration.

## Using the CLI Tool
If you prefer to analyze local files without a database:
1. Run the analyzer, providing the path to your CSV statement as an argument:
```bash
cargo run -p toll-optimizer-cli -- <csv_file_path>
```
**Example:**
```bash
cargo run -p toll-optimizer-cli -- csv/"2025-12-28 - 573522284 Statement.csv"
```

## Features
- **Time-Based Analysis**: Identifies trip clusters and calculates potential savings if you were to leave in a cheaper timeslot.
- **Distance-Based Analysis**: Suggests alternate entry or exit points that could lower your toll for the same route.
- **Savings Persistence**: Automatically saves the best optimization strategy (Time or Distance) to your account history.
- **Interactive Dashboard**: Grouped analysis by **transponder**, collapsible cards, and detailed trip tables with suggested route mappings.

## License
MIT
