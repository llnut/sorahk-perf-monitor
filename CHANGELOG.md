0.1.0
=====

Initial release

- Microsecond-precision event recording using RDTSC
- Lock-free ring buffer (65,536 events)
- Automatic TSC frequency calibration
- Per-key interval analysis with statistical metrics
- CSV export and terminal statistics output
- Configurable event limit via `-n` / `--max-events`
- ESC key to stop recording
