//! Keyboard event monitoring tool for performance analysis.

use csv::Writer;
use serde::Serialize;
use std::arch::x86_64::_rdtsc;
use std::sync::atomic::{AtomicU64, Ordering};
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_ESCAPE};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageA, KBDLLHOOKSTRUCT, MSG, PM_REMOVE, PeekMessageA,
    SetWindowsHookExA, TranslateMessage, UnhookWindowsHookEx, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP,
    WM_SYSKEYDOWN, WM_SYSKEYUP,
};

/// Processed keyboard event with timing information.
#[derive(Debug, Clone, Serialize)]
struct KeyEvent {
    event_id: u64,
    timestamp_us: u64,
    key_code: u32,
    key_name: String,
    action: String,
    interval_us: u64,
}

/// Raw event data captured by the keyboard hook.
/// Uses 16-byte alignment for efficient memory access.
#[derive(Clone, Copy)]
#[repr(C, align(16))]
struct RawEvent {
    tsc: u64,
    vk_code: u32,
    action: u8,
    _padding: [u8; 3],
}

const RING_BUFFER_SIZE: usize = 65536;

/// 64-byte aligned atomic to avoid cache line contention.
#[repr(align(64))]
struct AlignedAtomicU64(AtomicU64);

static mut RING_BUFFER: [RawEvent; RING_BUFFER_SIZE] = [RawEvent {
    tsc: 0,
    vk_code: 0,
    action: 0,
    _padding: [0; 3],
}; RING_BUFFER_SIZE];

static WRITE_INDEX: AlignedAtomicU64 = AlignedAtomicU64(AtomicU64::new(0));
static TSC_FREQUENCY: AtomicU64 = AtomicU64::new(0);

/// Maps virtual key code to human-readable name.
#[inline(always)]
fn get_key_name(vk_code: u32) -> String {
    match vk_code {
        0x41..=0x5A => format!("{}", (vk_code as u8) as char),
        0x30..=0x39 => format!("{}", (vk_code as u8) as char),
        0x60..=0x69 => format!("NUM{}", vk_code - 0x60),
        0x70..=0x87 => format!("F{}", vk_code - 0x6F),
        0x10 => "SHIFT".to_string(),
        0x11 => "CTRL".to_string(),
        0x12 => "ALT".to_string(),
        0xA0 => "LSHIFT".to_string(),
        0xA1 => "RSHIFT".to_string(),
        0xA2 => "LCTRL".to_string(),
        0xA3 => "RCTRL".to_string(),
        0xA4 => "LALT".to_string(),
        0xA5 => "RALT".to_string(),
        0x20 => "SPACE".to_string(),
        0x0D => "ENTER".to_string(),
        0x08 => "BACKSPACE".to_string(),
        0x09 => "TAB".to_string(),
        0x1B => "ESC".to_string(),
        0x01 => "LBUTTON".to_string(),
        0x02 => "RBUTTON".to_string(),
        0x04 => "MBUTTON".to_string(),
        _ => format!("VK_{:02X}", vk_code),
    }
}

/// Low-level keyboard hook procedure.
/// Records timestamp and event data with minimal processing.
#[allow(non_snake_case)]
#[inline(always)]
extern "system" fn keyboard_hook_proc(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if code >= 0 {
        unsafe {
            let kb_struct = *(l_param.0 as *const KBDLLHOOKSTRUCT);
            let vk_code = kb_struct.vkCode;

            let action = match w_param.0 as u32 {
                WM_KEYDOWN | WM_SYSKEYDOWN => 0u8,
                WM_KEYUP | WM_SYSKEYUP => 1u8,
                _ => 2u8,
            };

            if action < 2 {
                let tsc = _rdtsc();
                let idx =
                    WRITE_INDEX.0.fetch_add(1, Ordering::Relaxed) as usize & (RING_BUFFER_SIZE - 1);

                std::ptr::write_volatile(
                    &mut RING_BUFFER[idx],
                    RawEvent {
                        tsc,
                        vk_code,
                        action,
                        _padding: [0; 3],
                    },
                );
            }
        }
    }

    unsafe { CallNextHookEx(None, code, w_param, l_param) }
}

/// Checks if ESC key is currently pressed.
#[inline(always)]
fn check_esc_key() -> bool {
    unsafe { GetAsyncKeyState(VK_ESCAPE.0 as i32) as u16 & 0x8000 != 0 }
}

/// Calibrates TSC frequency using QueryPerformanceCounter.
fn calibrate_tsc() -> f64 {
    unsafe {
        let mut qpc_start: i64 = 0;
        let mut qpc_end: i64 = 0;
        let mut qpc_freq: i64 = 0;

        QueryPerformanceFrequency(&mut qpc_freq as *mut i64).ok();
        QueryPerformanceCounter(&mut qpc_start as *mut i64).ok();
        let tsc_start = _rdtsc();

        std::thread::sleep(std::time::Duration::from_millis(100));

        QueryPerformanceCounter(&mut qpc_end as *mut i64).ok();
        let tsc_end = _rdtsc();

        let qpc_delta = (qpc_end - qpc_start) as f64;
        let tsc_delta = (tsc_end - tsc_start) as f64;

        (tsc_delta * qpc_freq as f64) / qpc_delta
    }
}

fn print_help() {
    println!("Keyboard Performance Monitor - Event Recording Tool");
    println!();
    println!("USAGE:");
    println!("    sorahk-perf-monitor [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    -n, --max-events <COUNT>    Limit maximum number of events to record");
    println!("                                (default: unlimited)");
    println!("    -h, --help                  Print this help message");
    println!();
    println!("CONTROLS:");
    println!("    ESC                         Stop recording and save results");
    println!();
    println!("OUTPUT:");
    println!("    key_events.csv              CSV file with all recorded events");
    println!();
    println!("EXAMPLES:");
    println!("    sorahk-perf-monitor                    # Unlimited recording");
    println!("    sorahk-perf-monitor -n 1000            # Record 1000 events");
    println!("    sorahk-perf-monitor --max-events 5000  # Record 5000 events");
}

fn parse_max_events() -> Option<u64> {
    let args: Vec<String> = std::env::args().collect();

    for i in 0..args.len() {
        if args[i] == "-h" || args[i] == "--help" {
            print_help();
            std::process::exit(0);
        }

        if (args[i] == "-n" || args[i] == "--max-events") && i + 1 < args.len() {
            match args[i + 1].parse::<u64>() {
                Ok(max) if max > 0 => return Some(max),
                Ok(_) => {
                    eprintln!("Error: max-events must be greater than 0");
                    std::process::exit(1);
                }
                Err(_) => {
                    eprintln!("Error: invalid number for max-events: {}", args[i + 1]);
                    std::process::exit(1);
                }
            }
        }
    }

    None
}

fn main() {
    let max_events = parse_max_events();

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║                Keyboard Performance Monitor                ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();

    if let Some(max) = max_events {
        println!("Max Events Limit: {}", max);
    } else {
        println!("Max Events Limit: Unlimited");
    }

    println!("Calibrating TSC frequency...");

    let tsc_freq = calibrate_tsc();
    TSC_FREQUENCY.store(tsc_freq as u64, Ordering::Relaxed);
    println!("TSC Frequency: {:.2} MHz", tsc_freq / 1_000_000.0);
    println!();
    println!("Press ESC to stop and save results");
    println!("Output file: key_events.csv");
    println!();

    let output_file = "key_events.csv";

    unsafe {
        let hook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(keyboard_hook_proc), None, 0)
            .expect("Failed to install hook");

        println!("Recording...");

        let mut msg = MSG::default();

        loop {
            if check_esc_key() {
                break;
            }

            if let Some(max) = max_events
                && WRITE_INDEX.0.load(Ordering::Relaxed) >= max
            {
                println!("\nReached max events limit ({}), stopping...", max);
                break;
            }

            while PeekMessageA(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                let _ = TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }

            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        UnhookWindowsHookEx(hook).ok();
        println!("\nProcessing events...");
    }

    let total_written = WRITE_INDEX.0.load(Ordering::Acquire);
    let tsc_freq = TSC_FREQUENCY.load(Ordering::Relaxed) as f64;

    if total_written == 0 {
        println!("No events recorded.");
        return;
    }

    let event_count = std::cmp::min(total_written as usize, RING_BUFFER_SIZE);
    let mut raw_events: Vec<RawEvent> = Vec::with_capacity(event_count);

    unsafe {
        if total_written as usize <= RING_BUFFER_SIZE {
            raw_events.extend_from_slice(&RING_BUFFER[..event_count]);
        } else {
            let start_idx = (total_written as usize) & (RING_BUFFER_SIZE - 1);
            raw_events.extend_from_slice(&RING_BUFFER[start_idx..]);
            raw_events.extend_from_slice(&RING_BUFFER[..start_idx]);
        }
    }

    raw_events.sort_unstable_by_key(|e| e.tsc);
    raw_events.retain(|e| e.tsc > 0);

    if raw_events.is_empty() {
        println!("No valid events recorded.");
        return;
    }

    let start_tsc = raw_events[0].tsc;
    let mut events: Vec<KeyEvent> = Vec::with_capacity(raw_events.len());
    let mut last_tsc = start_tsc;
    let mut press_events = 0u64;
    let mut release_events = 0u64;

    let tsc_to_us = 1_000_000.0 / tsc_freq;

    for (event_id, raw_event) in raw_events.iter().enumerate() {
        let timestamp_us = ((raw_event.tsc - start_tsc) as f64 * tsc_to_us) as u64;
        let interval_us = ((raw_event.tsc - last_tsc) as f64 * tsc_to_us) as u64;

        let event = KeyEvent {
            event_id: event_id as u64,
            timestamp_us,
            key_code: raw_event.vk_code,
            key_name: get_key_name(raw_event.vk_code),
            action: if raw_event.action == 0 { "down" } else { "up" }.to_string(),
            interval_us,
        };

        if raw_event.action == 0 {
            press_events += 1;
        } else {
            release_events += 1;
        }

        last_tsc = raw_event.tsc;
        events.push(event);
    }

    let total_events = events.len() as u64;
    let last_tsc = raw_events.last().unwrap().tsc;
    let total_duration_ms = ((last_tsc - start_tsc) as f64 * tsc_to_us / 1000.0) as u64;

    let file = std::fs::File::create(output_file).expect("Failed to create CSV file");
    let buf_writer = std::io::BufWriter::with_capacity(8192 * 16, file);
    let mut csv_writer = Writer::from_writer(buf_writer);

    csv_writer
        .write_record([
            "EventID",
            "Timestamp(us)",
            "KeyCode",
            "KeyName",
            "Action",
            "Interval(us)",
        ])
        .expect("Failed to write CSV header");
    for event in &events {
        csv_writer.serialize(event).expect("Failed to write event");
    }
    csv_writer.flush().expect("Failed to flush CSV");

    let rate = if total_duration_ms > 0 {
        total_events as f64 / (total_duration_ms as f64 / 1000.0)
    } else {
        0.0
    };

    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║                    Final Report                            ║");
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║  Total Events:        {:<37}║", total_events);
    println!("║  Press Events:        {:<37}║", press_events);
    println!("║  Release Events:      {:<37}║", release_events);
    println!("║  Total Duration:      {:<34} ms║", total_duration_ms);
    println!("║  Average Rate:        {:<26.2} events/sec║", rate);
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║  Output File:         {:<37}║", output_file);
    println!("╚════════════════════════════════════════════════════════════╝");

    if total_events >= 10 {
        generate_analysis(&events, output_file);
    }
}

/// Generates per-key interval analysis for repeated key events.
fn generate_analysis(events: &[KeyEvent], csv_file: &str) {
    use std::collections::HashMap;

    let mut key_intervals: HashMap<&str, Vec<u64>> = HashMap::new();
    let mut last_down: HashMap<&str, u64> = HashMap::new();

    for event in events {
        if event.action == "down" {
            let key = event.key_name.as_str();

            if let Some(&prev_ts) = last_down.get(key) {
                let interval = event.timestamp_us.saturating_sub(prev_ts);
                key_intervals.entry(key).or_default().push(interval);
            }

            last_down.insert(key, event.timestamp_us);
        }
    }

    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║              Per-Key Interval Analysis                     ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    for (key, intervals) in key_intervals.iter() {
        if intervals.len() >= 5 {
            let len = intervals.len();
            let len_f64 = len as f64;

            let sum: u64 = intervals.iter().sum();
            let avg = sum as f64 / len_f64;

            let (min, max) = intervals
                .iter()
                .fold((u64::MAX, u64::MIN), |(min, max), &val| {
                    (min.min(val), max.max(val))
                });

            let variance = intervals
                .iter()
                .map(|&x| {
                    let diff = x as f64 - avg;
                    diff * diff
                })
                .sum::<f64>()
                / len_f64;
            let std_dev = variance.sqrt();

            println!("\nKey: {} ({} repeat events)", key, len);
            println!("  Average Interval: {:.2} μs ({:.2} ms)", avg, avg / 1000.0);
            println!(
                "  Min Interval:     {} μs ({:.3} ms)",
                min,
                min as f64 / 1000.0
            );
            println!(
                "  Max Interval:     {} μs ({:.3} ms)",
                max,
                max as f64 / 1000.0
            );
            println!(
                "  Std Deviation:    {:.2} μs ({:.3} ms)",
                std_dev,
                std_dev / 1000.0
            );
            println!("  Frequency:        {:.2} Hz", 1_000_000.0 / avg);

            let (within_1ms, within_2ms) = intervals.iter().fold((0, 0), |(w1, w2), &x| {
                let diff = (x as f64 - avg).abs();
                (
                    w1 + (diff <= 1000.0) as usize,
                    w2 + (diff <= 2000.0) as usize,
                )
            });

            let precision_1ms = within_1ms as f64 / len_f64 * 100.0;
            let precision_2ms = within_2ms as f64 / len_f64 * 100.0;

            println!("  Within ±1ms:      {:.1}%", precision_1ms);
            println!("  Within ±2ms:      {:.1}%", precision_2ms);
        }
    }

    println!("\nData saved to: {}", csv_file);
}
