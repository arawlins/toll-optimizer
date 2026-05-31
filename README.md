# Toll Optimizer CLI

Toll Optimizer is a high-performance Rust-based tool designed to analyze 407 ETR (Electronic Toll Route) statements. It identifies patterns in your travel and suggests optimizations based on **time** (shifting trips to cheaper timeslots) and **distance** (adjusting entry/exit points) to reduce your total toll charges.

### This application is not affiliated with 407 ETR in any way. It is intended for personal use only. ###

## Features
- **Time-Based Analysis**: Identifies trip clusters and calculates potential savings if you were to leave in a cheaper timeslot.
- **Distance-Based Analysis**: Suggests alternate entry or exit points that could lower your toll for the same route.
- **Live Pricing**: Real-time 407 ETR rate lookup and optimization tips for the current and upcoming timeslots.
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

#### Note for macOS Users
When running a downloaded binary on macOS, you may see a message saying "Apple could not verify 'toll-optimizer' is free of malware." To resolve this, you can remove the "quarantine" flag from the binary by running the following command in your terminal:

```bash
xattr -d com.apple.quarantine /path/to/toll-optimizer
```

Replace `/path/to/toll-optimizer` with the actual path to the downloaded file (e.g., `~/Downloads/toll-optimizer`).

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
#### Options:
- `-j, --json`: Output results in JSON format (for agentic or programmatic use).
- `-m, --markdown`: Output results in Markdown format (for reports).
- `--show-summary`: Suppresses individual trip details in the output. Only the processing summary and cluster-level analysis (averages, totals, and optimization advice) will be shown. This works for standard, Markdown, and JSON outputs.
#### Example:
```bash
toll-optimizer csv/2026-01-28-Statement.csv
```
#### CSV Format
The tool expects the standard CSV export format from the 407 ETR website. Ensure your file contains headers like `Date`, `Entry Time`, `Entry Point`, `Exit Point`, etc.

#### Errors
If you see any unrecognized access points or vehicle classes after the Processing Summary in the output, you can open an [issue](https://github.com/arawlins/toll-optimizer/issues) and I'll fix it. Example output:
```
--- Processing Summary ---
Trips Processed: 6
Trips Skipped:   3
Total Bill Cost: $114.51
Potential Time-Based Savings:     $19.40
Potential Distance-Based Savings: $7.14

Unrecognized Access Points:
  - Doober | NOT RECOGNIZED
  - Goober | NOT RECOGNIZED

Unrecognized Vehicle Classes:
  - Space Shuttle | NOT RECOGNIZED
```

### Check Current Pricing
Get the current and next timeslot average prices:

```bash
toll-optimizer --current-price
```
#### Options:
- `-j, --json`: Output results in JSON format (for agentic or programmatic use).
- `-m, --markdown`: Output results in Markdown format (for reports).
- `--date <DATE>`: Override date for pricing (YYYY-MM-DD).
- `--time <TIME>`: Override time for pricing (HH:MM AM/PM or HH:MM).
- `--vehicle-class <CLASS>`: Vehicle class for pricing (e.g., "Light vehicle", "Heavy Single Unit", "Heavy Multiple Unit", "Medium Vehicle", "Motorcycle"). Default: "Light vehicle".
#### Example:
```bash
toll-optimizer --current-price --date 2026-05-12 --time "07:30 AM"
```

### Single Trip Calculation
Calculate the cost for a single trip between two points.

```bash
toll-optimizer --entry "McCowan" --exit "Hwy400"
```
#### Options:
- `-j, --json`: Output results in JSON format (for agentic or programmatic use).
- `-m, --markdown`: Output results in Markdown format (for reports).
- `--date <DATE>`: Override date for pricing (YYYY-MM-DD).
- `--time <TIME>`: Override time for pricing (HH:MM AM/PM or HH:MM).
- `--vehicle-class <CLASS>`: Vehicle class for pricing (e.g., "Light vehicle", "Heavy Single Unit", "Heavy Multiple Unit", "Medium Vehicle", "Motorcycle"). Default: "Light vehicle".
#### Example:
```bash
toll-optimizer --entry "McCowan" --exit "Hwy400" --date 2026-05-12 --time "07:30 AM"
```

### List Access Points
List all recognized 407 ETR access points.

```bash
toll-optimizer --list-access-points
```
#### Options:
- `-j, --json`: Output results in JSON format (for agentic or programmatic use).
- `-m, --markdown`: Output results in Markdown format (for reports).

### List Pricing Timeslots
List all 407 ETR pricing timeslots for weekdays and weekends/holidays.

```bash
toll-optimizer --list-timeslots
```

### Help and Version
Print help or version information.

```bash
toll-optimizer --help
toll-optimizer --version
```

## LLM Integration (Skill)

Toll Optimizer includes a [SKILL.md](https://github.com/arawlins/toll-optimizer/blob/main/SKILL.md) file that allows LLMs (like Gemini, ChatGPT, or Claude) to autonomously analyze your statements and provide financial advice.

### Adding to PATH

To use the tool with an LLM, the `toll-optimizer` executable must be accessible via your system's `PATH`.

#### macOS / Linux

1. Move the extracted `toll-optimizer` binary to a standard system directory, such as `/usr/local/bin`:
   ```bash
   sudo mv toll-optimizer /usr/local/bin/
   ```
   *(Alternatively, if you prefer a user-local directory like `~/.local/bin`, make sure that directory is added to your shell's configuration file, e.g., `~/.bashrc` or `~/.zshrc`: `export PATH="$HOME/.local/bin:$PATH"`)*

2. Verify that it works by opening a new terminal and running:
   ```bash
   toll-optimizer --version
   ```

#### Windows

1. Move the `toll-optimizer.exe` to a dedicated folder (e.g., `C:\tools\toll-optimizer` or `C:\Program Files\toll-optimizer`).
2. Add that folder to your User or System `PATH` environment variable:
   - Press `Win + R`, type `sysdm.cpl`, and press **Enter**.
   - Go to the **Advanced** tab and click **Environment Variables...**.
   - Under **User variables** or **System variables**, select the `Path` variable and click **Edit...**.
   - Click **New** and paste the path to the folder where you placed the executable (e.g., `C:\tools\toll-optimizer`).
   - Click **OK** on all windows to save the changes.
3. Open a new Command Prompt or PowerShell window and verify the installation:
   ```cmd
   toll-optimizer.exe --version
   ```

### Using with Gemini CLI
If you use the [Gemini CLI](https://github.com/google/gemini-cli), you can install this tool as a skill:

1. Install the skill using gemini cli:
   ```bash
   gemini install skill <path to SKILL.md>
   ```
2. Ensure the `toll-optimizer` binary is in your `PATH`.

### Using with Claude Code
If you use [Claude Code](https://claude.com/product/claude-code), you can install this tool as a skill:

1. Copy the SKILL.md file into your ~/.claude/skills/ directory:
   ```
   cp <path to SKILL.md> ~/.claude/skills/
   ```
2. Ensure the `toll-optimizer` binary is in your `PATH`.

### Using with Codex
If you use [Codex](https://developers.openai.com/codex), you can install this tool as a skill:

1. Copy the SKILL.md file into your ~/.codex/skills/ directory:
   ```
   cp <path to SKILL.md> ~/.codex/skills/
   ```
2. Ensure the `toll-optimizer` binary is in your `PATH`.

### Once installed, you can ask your agent questions like:
#### "Is it cheaper for me to get on the 407 now or wait 30 minutes?"
Sample Output:
```
The price is the same whether you get on now (5:55 PM) or in 30 minutes (6:25 PM), as both fall within the weekend daytime rate (10:00 AM – 7:00 PM).

  However, if you can wait until 7:00 PM (1 hour and 5 minutes from now), you will save approximately 20% on your toll.

  Pricing Comparison (Weekend)
   * Now (5:55 PM): ~66.3¢/km (Daytime Rate)
   * In 30 mins (6:25 PM): ~66.3¢/km (Daytime Rate)
   * At 7:00 PM: ~52.3¢/km (Evening Rate) — Saves ~14¢/km

  If your trip is long, waiting until 7:00 PM is highly recommended. If you must leave within the next 30 minutes, there is no financial advantage to waiting.
```

#### "I'm going to drive from Markville Mall to the Cambridge Centre on Wednesday afternoon leaving around 4pm. How much will that trip cost?"
Sample Output:
```
 The estimated cost for your trip from Markville Mall to Cambridge Centre on Wednesday, May 20, 2026, leaving around 4:00 PM, is approximately $54.79 for the 407 ETR portion.

  Trip Details:
   * 407 Entry: McCowan Rd (closest to Markville Mall)
   * 407 Exit: Hwy 401 West (where the 407 connects to the 401 toward Cambridge)
   * Distance on 407: ~56.26 km
   * Base Toll: $53.79 (Peak Weekday Rate)
   * Trip Charge: $1.00
   * Total Estimated 407 Cost: $54.79

  Note: This estimate only covers the 407 ETR toll. The remainder of your journey from the Hwy 401/407 interchange to the Cambridge Centre via Hwy 401 is toll-free. Since you are leaving at 4:00 PM on a
  weekday, you will be traveling during Peak Period rates.
```

#### "Help me find ways to reduce my 407 bill, here is a statement: \<path to statement.csv\>"
Sample Output:
```
 Based on your 407 ETR statement from January 28 to February 27, 2026, here is an analysis of potential savings for your account.

  1. Statement Summary
  ┌──────────────────────────────────┬─────────┐
  │ Metric                           │ Value   │
  ├──────────────────────────────────┼─────────┤
  │ Total Cost of Bill               │ $420.16 │
  │ Total Trips Processed            │ 26      │
  │ Potential Distance-Based Savings │ $19.65  │
  │ Potential Time-Based Savings     │ $8.94   │
  └──────────────────────────────────┴─────────┘

  2. Distance-Based Optimization Advice
  You can reduce your costs by making small adjustments to where you enter and exit the highway:
   - Westbound (Transponder1): Your frequent entry at McCowan for trips to Hwy 404 is a major expense. Entering at Kennedy or Warden instead would have saved you $10.32 this month.
   - Eastbound (Transponder1): For your long trips from Hwy 401 to Kennedy, exiting slightly earlier at Warden or Woodbine would have saved $3.33.
   - Westbound (Transponder1): Entering at Warden instead of Kennedy for your trips toward Hwy 401 would have saved $2.29.
   - Eastbound (Transponder2): Entering at Markham instead of McCowan for trips to Brock Road (Hwy 7) would have saved $1.54.

  3. Time-Based Optimization Advice
  Adjusting your departure times by just a few minutes can help you avoid peak rates:
   - Eastbound (Transponder2): For your trips around 7:08 PM, waiting until after 9:00 PM to enter the highway would have saved you $4.14.
   - Westbound (Transponder2): For trips around 6:52 PM, waiting just 8 minutes until after 7:00 PM would have saved $2.63.
   - Eastbound (Transponder1): For your afternoon trips around 3:44 PM, leaving after 6:00 PM would have yielded $2.17 in savings.

  By combining these strategies—particularly switching your westbound entry from McCowan to Kennedy and waiting until after 7:00 PM for your evening trips—you could reduce your monthly bill by nearly $30.
```

## Development

Run tests:
```bash
cargo test
```

## License
MIT
