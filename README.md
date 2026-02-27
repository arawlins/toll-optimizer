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

## Quick Start

### 1. Start the Database
The application requires PostgreSQL. Use the provided Docker Compose configuration:
```bash
docker-compose up -d
```

### 2. Configure Environment
Create a `.env` file in the root directory (or `crates/api`):
```env
DATABASE_URL=
JWT_SECRET=
RUST_LOG=info
```

### 3. Initialize the Schema
Run the initial migration (ensure the DB is up first):
```bash
docker exec -i toll_optimizer_db psql -U admin -d toll_optimizer < crates/api/migrations/20260126000000_init_schema.sql
```

### 4. Start the Backend API
```bash
cargo run -p toll-optimizer-api
```
The API will be available at `http://localhost:3000`.

### 5. Start the Frontend
```bash
cd frontend
npm install
npm run dev
```
Open [http://localhost:5173](http://localhost:5173) in your browser.

## Using the CLI Tool
If you prefer to analyze local files without a database:
1. Place your CSV statements in a `csv/` directory.
2. Run the analyzer:
```bash
cargo run -p toll-optimizer-cli
```

## Features
- **Time-Based Analysis**: Identifies trip clusters and calculates potential savings if you were to leave in a cheaper timeslot.
- **Distance-Based Analysis**: Suggests alternate entry or exit points that could lower your toll for the same route.
- **Savings Persistence**: Automatically saves the best optimization strategy (Time or Distance) to your account history.
- **Interactive Dashboard**: Drill down into specific trips and see exactly how much each adjustment saves.

## License
MIT
