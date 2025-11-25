<div align="center">

# 📊 Sorahk Performance Monitor 📊

### ✨ *A Microsecond-Precision Keyboard Event Recorder* ✨

<p align="center">
  <img src="https://img.shields.io/badge/Platform-Windows-blue?style=flat-square&logo=windows" alt="Platform"/>
  <img src="https://img.shields.io/badge/Language-Rust-orange?style=flat-square&logo=rust" alt="Language"/>
  <img src="https://img.shields.io/badge/Precision-Microsecond-purple?style=flat-square" alt="Precision"/>
  <img src="https://img.shields.io/badge/License-MIT-green?style=flat-square" alt="License"/>
</p>

---

</div>

## 📖 Overview

**Sorahk Performance Monitor** is a specialized keyboard event recording tool designed for performance testing and analysis. Built with Rust and utilizing RDTSC (Time Stamp Counter), it provides microsecond-precision timing measurements for evaluating auto-key press tools, measuring input latency, and analyzing keyboard performance.

The tool is particularly useful for benchmarking [Sorahk](https://github.com/llnut/Sorahk) against other automation tools like AutoHotkey. 🎯

> ⚠️ **Platform Requirement**: This application requires Windows 10 or later and x86_64 architecture for RDTSC support.

---

## ✨ Features

- **Microsecond-Precision Timing** – Hardware-level RDTSC timestamps with sub-millisecond accuracy
- **Low-overhead Recording** – Lock-free ring buffer with minimal CPU usage (< 1%)
- **Automatic Statistics** – Per-key interval analysis with mean, stddev, and precision metrics
- **CSV Export** – Full event log with timestamps, key codes, and intervals
- **Configurable Event Limit** – Record specific number of events or unlimited
- **Simple Controls** – Press ESC to stop and save results

---

## 🚀 Getting Started

### ▶️ Quick Start

1. **Download** or build `sorahk-perf-monitor.exe`
2. **Run** the executable from command line or use provided batch files
3. **Perform** your keyboard actions (or run test scripts)
4. **Press** ESC to stop recording
5. **Review** performance statistics in the terminal output and analyze the generated `key_events.csv` file

### 📝 Basic Usage

```bash
# Record unlimited events (press ESC to stop)
sorahk-perf-monitor.exe

# Record specific number of events
sorahk-perf-monitor.exe -n 1000
sorahk-perf-monitor.exe --max-events 5000
```

### 🎮 Command Line Options

```
OPTIONS:
    -n, --max-events <COUNT>    Limit maximum number of events to record
    -h, --help                  Print help message

CONTROLS:
    ESC                         Stop recording and save results

OUTPUT:
    Terminal                    Real-time statistics and analysis summary
    key_events.csv              CSV file with all recorded events
```

### 🔨 Building from Source

**Prerequisites:**
- [Rust](https://rustup.rs/) (stable channel via rustup)
- Windows 10 or later
- x86_64 architecture

**Build Steps:**

```bash
# Clone the repository
git clone https://github.com/llnut/sorahk-perf-monitor.git
cd sorahk-perf-monitor

# Release build (recommended)
cargo build --release

# With native CPU optimizations
set RUSTFLAGS=-C target-cpu=native
cargo build --release

# The executable will be at: target\release\sorahk-perf-monitor.exe
```

---

## 📊 Output Format

### 📈 Terminal Statistics

For keys with 5 or more repeated events, the tool displays real-time analysis:

```
╔════════════════════════════════════════════════════════════╗
║                    Final Report                            ║
╠════════════════════════════════════════════════════════════╣
║  Total Events:        1000                                 ║
║  Press Events:        500                                  ║
║  Release Events:      500                                  ║
║  Total Duration:      5023 ms                              ║
║  Average Rate:        199.08 events/sec                    ║
╠════════════════════════════════════════════════════════════╣
║  Output File:         key_events.csv                       ║
╚════════════════════════════════════════════════════════════╝

╔════════════════════════════════════════════════════════════╗
║              Per-Key Interval Analysis                     ║
╚════════════════════════════════════════════════════════════╝

Key: A (100 repeat events)
  Average Interval: 5023.45 μs (5.02 ms)
  Min Interval:     4890 μs (4.890 ms)
  Max Interval:     5210 μs (5.210 ms)
  Std Deviation:    78.23 μs (0.078 ms)
  Frequency:        199.07 Hz
  Within ±1ms:      92.0%
  Within ±2ms:      100.0%
```

### 📄 CSV File Format

The tool generates `key_events.csv` with the following columns:

| Column | Description |
|--------|-------------|
| **EventID** | Sequential event number (0, 1, 2, ...) |
| **Timestamp(us)** | Microseconds since first event |
| **KeyCode** | Windows virtual key code (decimal) |
| **KeyName** | Human-readable key name |
| **Action** | "down" or "up" |
| **Interval(us)** | Microseconds since previous event |

**Example CSV Output:**

```csv
EventID,Timestamp(us),KeyCode,KeyName,Action,Interval(us)
0,0,65,A,down,0
1,5234,65,A,up,5234
2,10456,65,A,down,5222
3,15678,65,A,up,5222
```

---

## ⚠️ Limitations

- **Platform**: Windows only (uses Windows API)
- **Architecture**: x86_64 only (requires RDTSC instruction)
- **Buffer Size**: Events beyond 65,536 will overwrite oldest events
- **TSC Stability**: May experience drift on systems with dynamic frequency scaling

---

## 🛠️ Troubleshooting

### TSC Calibration Issues

If timing appears inconsistent:

1. Ensure stable CPU frequency (disable power saving modes)
2. Run as administrator for reliable timing
3. Close other high-priority applications

### Missing Events

If events are not recorded:

1. Verify keyboard hook was installed successfully
2. Check if other hooks are blocking events
3. Try running as administrator

### Unexpected Results

- Disable other key remapping software during testing
- Check Windows keyboard repeat settings
- Verify CSV file encoding (UTF-8)

---

## 📄 License

**MIT License** – see the [LICENSE](LICENSE) file for details.

---

## 🙌 Acknowledgements

This tool is built using:

- 🦀 **[Rust](https://www.rust-lang.org/)** – Memory safety and zero-cost abstractions
- 🪟 **[windows-rs](https://crates.io/crates/windows)** – Native Windows API access
- 📊 **[csv](https://crates.io/crates/csv)** – CSV file generation

---

## 🔗 References

- [Sorahk Project](https://github.com/llnut/Sorahk)
- [RDTSC Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html)
- [Windows Keyboard Hooks](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-hooks)
- [QueryPerformanceCounter](https://docs.microsoft.com/en-us/windows/win32/api/profileapi/nf-profileapi-queryperformancecounter)
