use crate::ipv4;
use anyhow::Result;
use log::info;
use std::net::{SocketAddr, TcpStream};
use std::thread;
use std::thread::JoinHandle;

fn tcp_scan(mut target: ipv4::IPv4, target_port: u16) -> bool {
    let dest = target.to_socketaddr(target_port).unwrap();

    if let Ok(res) = TcpStream::connect(dest) {
        info!("* Got TCP ack from: {:?} | {:?}", dest, res);
        return true;
    }
    false
}

fn create_scan_thread(
    ip_list: Vec<ipv4::IPv4>,
    thread_id: u32,
    ips_per_thread: u32,
    target_port: u16,
) -> JoinHandle<Vec<bool>> {
    thread::spawn(move || {
        info!("Starting thread worker #{}", thread_id);
        let mut results: Vec<bool> = Vec::new();

        // do the scan thing
        for i in 0..ips_per_thread {
            let id = (thread_id * ips_per_thread) + i;
            let ref target = ip_list[id as usize];
            let result = tcp_scan(*target, target_port);
            results.push(result);
        }

        results
    })
}

pub fn start_scan(target_port: u16, num_threads: u32, ignorelist: Option<Vec<u64>>) -> Result<()> {
    let ip_list = ipv4::get_all(ignorelist)?;

    let ips_per_thread = ((ip_list.len() as f32) / num_threads as f32) as u32;
    let ips_left = num_threads * ips_per_thread; // how many ips we have left after the first threads

    // container for all of our threads
    let mut threads: Vec<JoinHandle<Vec<bool>>> = Vec::new();
    let result_list: Vec<bool> = Vec::new();

    // create all of our threads
    for thread_id in 0..num_threads {
        threads.push(create_scan_thread(
            ip_list.clone(),
            thread_id,
            ips_per_thread,
            target_port,
        ));
    }

    // create the last thread to do the job
    if ips_left > 0 {
        threads.push(create_scan_thread(
            ip_list,
            num_threads,
            ips_left,
            target_port,
        ));
    }

    Ok(())
}
