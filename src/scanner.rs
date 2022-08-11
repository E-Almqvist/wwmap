use crate::ipv4::{IPv4, IPv4Range};
use anyhow::Result;
use log::info;
use std::net::TcpStream;
use std::thread;
use std::thread::JoinHandle;

fn tcp_scan(mut target: IPv4, target_port: u16) -> bool {
    let dest = target.to_socketaddr(target_port).unwrap();

    false
    // TODO: FIX
    //     if let Ok(res) = TcpStream::connect(dest) {
    //         info!("* Got TCP ack from: {:?} | {:?}", dest, res);
    //         return true;
    //     }
    //     false
}

fn create_scan_thread(
    thread_id: u32,
    ip_range: IPv4Range,
    target_port: u16,
) -> JoinHandle<Vec<bool>> {
    thread::spawn(move || {
        info!("Starting thread worker #{}", thread_id);
        let mut results: Vec<bool> = Vec::new();

        // do the scan thing
        ip_range.into_iter().for_each(|id| {
            let target = IPv4::new(id as u64);
            let result = tcp_scan(target, target_port);
            results.insert(id as usize, result);
        });

        results
    })
}

pub fn start_scan(
    from: u32,
    to: u32,
    target_port: u16,
    num_threads: u32,
    ignorelist: Option<Vec<u32>>,
) -> Result<()> {
    println!("Starting wwmap...");
    //let ip_list = get_all(ignorelist)?;

    let ips_per_thread = (((to - from) as f32) / num_threads as f32) as u32;
    let ips_left = num_threads * ips_per_thread; // how many ips we have left after the first threads

    // container for all of our threads
    let mut threads: Vec<JoinHandle<Vec<bool>>> = Vec::new();

    for thread_id in 0..num_threads {
        // Calculate ranges & stuff
        let (f, t) = (
            (thread_id * ips_per_thread),
            ((thread_id + 1) * ips_per_thread),
        );
        let range = IPv4Range::new(f, t, ignorelist);

        // Create a worker
        let worker = create_scan_thread(thread_id, range, target_port);
        threads.push(worker);
    }

    // container for all the results
    let mut result_list: Vec<bool> = Vec::new();

    Ok(())
}
