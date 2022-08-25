use crate::ipv4::{IPv4, IPv4Range};
use core::time::Duration;
use log::{debug, info, warn};
use std::net::TcpStream;
use std::thread::JoinHandle;
use std::{panic, thread};

static mut TCP_SCANS_ISSUED: u64 = 0;
static mut TARGET_SCAN_TOTAL: u32 = 0;

fn tcp_scan(mut target: IPv4, target_port: u16, timeout: Duration) -> bool {
    debug!("Starting scan on {:?}", target);
    let dest = target.to_socketaddr(target_port);

    unsafe {
        TCP_SCANS_ISSUED += 1;
    }

    // TODO: Fix closed port viewed as "open"
    if let Ok(_res) = TcpStream::connect_timeout(&dest, timeout) {
        println!("{:?}", dest);
        true
    } else {
        debug!("Timeout * {:?}", dest);
        false
    }
}

fn create_scan_thread(
    ip_range: IPv4Range,
    target_port: u16,
    timeout: Duration,
) -> JoinHandle<Vec<(u32, bool)>> {
    thread::spawn(move || {
        let mut results: Vec<(u32, bool)> = Vec::new();
        debug!("Created scan worker for IPv4 range: {:?}", ip_range);

        // do the scan thing
        ip_range.into_iter().for_each(|id| {
            let target = IPv4::new(id as u64);
            let result = tcp_scan(target, target_port, timeout);
            if result {
                results.push((id, result));
            }
        });

        results
    })
}

fn create_scan_worker(
    range: IPv4Range,
    thread_id: u64,
    ips_per_thread: u64,
    target_port: u16,
    timeout: Duration,
) -> JoinHandle<Vec<(u32, bool)>> {
    let ignorelist = range.id_ignore;

    let (f, t) = (
        (thread_id * ips_per_thread) + (range.id_start as u64),
        ((thread_id + 1) * ips_per_thread - 1_u64) + range.id_start as u64,
    );
    let range = IPv4Range::new(f as u32, t as u32, Some(ignorelist));
    create_scan_thread(range, target_port, timeout)
}

fn get_scan_workers(
    range: IPv4Range,
    target_port: u16,
    num_threads: u64,
    timeout: Duration,
) -> Vec<JoinHandle<Vec<(u32, bool)>>> {
    let (from, to) = (range.id_start, range.id_end);

    let ip_amount: u64 = (to as u64 - from as u64) + 1;
    let ips_per_thread: u64 = ((ip_amount as f32) / num_threads as f32).floor() as u64;

    // container for all of our threads
    let mut threads: Vec<JoinHandle<Vec<(u32, bool)>>> = Vec::new();

    // TODO: make last thread do the "ips_left" work
    debug!("Creating scan workers...");
    for thread_id in 0..num_threads {
        let range_copy = range.clone();

        // Create a worker
        let worker =
            create_scan_worker(range_copy, thread_id, ips_per_thread, target_port, timeout);

        threads.push(worker);
    }

    // how many ips we have left after the first threads
    if (ip_amount as u64 % num_threads) != 0 {
        let completed_ips = (num_threads * ips_per_thread) as u64;
        let ips_left: u64 = ip_amount as u64 - completed_ips;

        // Clean up the rest
        warn!("Number of IPv4 addresses is not divisible by the amount of threads! Creating extra thread...");
        let worker = create_scan_worker(
            range,
            threads.len() as u64 + 1,
            ips_left,
            target_port,
            timeout,
        );
        threads.push(worker);
    };

    threads
}

#[derive(Debug)]
pub struct ScanResult {
    pub target: IPv4,
    pub result: bool,
}

impl ScanResult {
    fn from_tuple(result_tuple: (u32, bool)) -> Self {
        let (id, result) = result_tuple;
        let target = IPv4::new(id as u64);
        Self { target, result }
    }
}

unsafe fn print_progress() {
    let percent_done = (TCP_SCANS_ISSUED as f64 / TARGET_SCAN_TOTAL as f64) * 100_f64;
    println!(
        "* Progress: {}/{} [{:.2}%]",
        TCP_SCANS_ISSUED, TARGET_SCAN_TOTAL, percent_done
    );
}

fn create_progress_worker() -> JoinHandle<()> {
    thread::spawn(move || loop {
        unsafe {
            print_progress();
        }

        let dur = Duration::new(5, 0);
        thread::sleep(dur);
    })
}

pub fn start_scan(
    range: IPv4Range,
    target_port: u16,
    num_threads: u64,
    timeout: Duration,
    show_progress: bool,
) -> Vec<ScanResult> {
    println!("Starting wwmap scan...");

    // Create progress worker
    unsafe {
        if show_progress {
            TARGET_SCAN_TOTAL = range.length();
            create_progress_worker();
        }
    }

    // Get the workers
    let scan_workers = get_scan_workers(range, target_port, num_threads, timeout);
    info!("Loaded {} scan worker(s).", scan_workers.len());

    let mut results: Vec<ScanResult> = Vec::new();

    // Run all the workers
    for worker in scan_workers {
        let result_tuples = match worker.join() {
            Ok(r) => r,
            Err(e) => panic!("{:?}", e),
        };

        let mut worker_results = result_tuples
            .iter()
            .map(|res| ScanResult::from_tuple(*res))
            .collect();

        results.append(&mut worker_results);
    }

    unsafe {
        println!(
            "Scan finished with {} result(s) with a total of {} scans.",
            results.len(),
            TCP_SCANS_ISSUED
        );
    }
    results
}
