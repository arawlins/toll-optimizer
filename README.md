# Toll Optimizer CLI

Toll Optimizer is a high-performance Rust-based tool designed to analyze 407 ETR (Electronic Toll Route) statements. It identifies patterns in your travel and suggests optimizations based on **time** (shifting trips to cheaper timeslots) and **distance** (adjusting entry/exit points) to reduce your total toll charges.

## Features
- **Time-Based Analysis**: Identifies trip clusters and calculates potential savings if you were to leave in a cheaper timeslot.
- **Distance-Based Analysis**: Suggests alternate entry or exit points that could lower your toll for the same route.
- **Live Pricing**: Real-time 407 ETR rate lookup and optimization tips for the current and upcoming timeslots.
- **2026 Rate Projections**: Includes the latest 2026 toll rates for accurate savings estimates.
- **JSON Output**: Fully structured machine-readable output optimized for LLMs and data pipelines.
- **Markdown Output**: Professional report format with tables, ideal for sharing and documentation.
- **LLM Ready**: Includes a native "Skill" definition for seamless use with AI agents.
- **Standalone Binaries**: Run the tool without needing to install Rust or Cargo.

## Installation

### 1. Download Pre-compiled Binaries
You can download the latest standalone binary for your operating system from the [Releases](https://github.com/arawlins/toll-optimizer/releases) page.

- **Linux**: `toll-optimizer-linux-x86_64.tar.gz`
- **macOS (Intel)**: `toll-optimizer-macos-x86_64.tar.gz`
- **macOS (Apple Silicon)**: `toll-optimizer-macos-aarch64.tar.gz`
- **Windows**: `toll-optimizer-windows-x86_64.zip`

Extract the archive and run the `toll-optimizer` executable.

### 2. Build from Source
If you have Rust installed, you can build and install the tool directly:
```bash
cargo install --path .
```

## Usage

### Analyze a Statement
Analyze a 407 ETR CSV statement file:

```bash
toll-optimizer <path-to-csv>
```

### Check Current Pricing
Get the current and next timeslot prices:

```bash
toll-optimizer --current-price
```

You can also override the date and time:
```bash
toll-optimizer --current-price --date 2026-05-12 --time "07:30 AM"
```

### Options:
- `-v, --verbose`: Show detailed trip listings and validation.
- `-j, --json`: Output results in JSON format (for programmatic use).
- `-m, --markdown`: Output results in Markdown format (for reports).
- `--current-price`: Display pricing info for the current timeslot.
- `--date <DATE>`: Override date for pricing (YYYY-MM-DD).
- `--time <TIME>`: Override time for pricing (HH:MM AM/PM or HH:MM).
- `-h, --help`: Show help information.
- `-V, --version`: Show version information.

### Example:
```bash
toll-optimizer csv/2025-12-28-Statement.csv --verbose
```

## CSV Format
The tool expects the standard CSV export format from the 407 ETR website. Ensure your file contains headers like `Date`, `Entry Time`, `Entry Point`, `Exit Point`, etc.

## Development

Run tests:
```bash
cargo test
```

## License
MIT
