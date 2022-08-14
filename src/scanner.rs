use crate::ipv4::{IPv4, IPv4Range};
use core::time::Duration;
use log::warn;
use std::net::TcpStream;
use std::thread::JoinHandle;
use std::{panic, thread};

fn tcp_scan(mut target: IPv4, target_port: u16) -> bool {
    let dest = target
        .to_socketaddr(target_port)
        .unwrap_or_else(|e| panic!("{}", e));

    let timeout = Duration::new(1, 0);

    if let Ok(_res) = TcpStream::connect_timeout(&dest, timeout) {
        println!("* {:?}", dest);
        true
    } else {
        false
    }
}

fn create_scan_thread(ip_range: IPv4Range, target_port: u16) -> JoinHandle<Vec<(u32, bool)>> {
    thread::spawn(move || {
        let mut results: Vec<(u32, bool)> = Vec::new();

        // do the scan thing
        ip_range.into_iter().for_each(|id| {
            let target = IPv4::new(id as u64);
            let result = tcp_scan(target, target_port);
            results.push((id, result));
        });

        results
    })
}

fn create_scan_worker(
    thread_id: u64,
    ips_per_thread: u64,
    target_port: u16,
    ignorelist: Vec<u32>,
) -> JoinHandle<Vec<(u32, bool)>> {
    let (f, t) = (
        (thread_id * ips_per_thread),
        ((thread_id + 1) * ips_per_thread - 1),
    );
    let range = IPv4Range::new(f as u32, t as u32, Some(ignorelist));
    create_scan_thread(range, target_port)
}

fn get_scan_workers(
    range: IPv4Range,
    target_port: u16,
    num_threads: u64,
) -> Vec<JoinHandle<Vec<(u32, bool)>>> {
    let (from, to) = (range.id_start, range.id_end);

    let ip_amount: u64 = (to as u64 - from as u64) + 1;
    let ips_per_thread: u64 = ((ip_amount as f32) / num_threads as f32).floor() as u64;

    // container for all of our threads
    let mut threads: Vec<JoinHandle<Vec<(u32, bool)>>> = Vec::new();

    // TODO: make last thread do the "ips_left" work
    for thread_id in 0..num_threads {
        let id_ignorelist = range.id_ignore.clone();

        // Create a worker
        let worker = create_scan_worker(thread_id, ips_per_thread, target_port, id_ignorelist);

        threads.push(worker);
    }

    // how many ips we have left after the first threads
    if (ip_amount as u64 % num_threads) != 0 {
        let completed_ips = (num_threads * ips_per_thread) as u64;
        let ips_left: u64 = ip_amount as u64 - completed_ips;

        // Clean up the rest
        warn!("Number of IPv4 addresses is not divisible by the amount of threads! Creating extra thread...");
        let id_ignorelist = range.id_ignore.clone();
        let worker = create_scan_worker(
            threads.len() as u64 + 1,
            ips_left,
            target_port,
            id_ignorelist,
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

pub fn start_scan(range: IPv4Range, target_port: u16, num_threads: u64) -> Vec<ScanResult> {
    println!("Starting wwmap scan...");

    // Get the workers
    let scan_workers = get_scan_workers(range, target_port, num_threads);
    println!("Loaded {} scan worker(s).", scan_workers.len());

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

    results
}
