use crate::ipv4;
use anyhow::Result;
use log::info;
use std::thread;
use std::thread::JoinHandle;
use std::net::{SocketAddr, TcpStream};

fn tcp_scan(dest: &SocketAddr) -> bool {
    if let Ok(res) = TcpStream::connect(dest) {
        info!("Got TCP ack from: {:?}", dest);
        return true;
    }
    false
}

// TODO: do thread optimization

pub fn scan(target: &ipv4::IPv4) -> bool {
    true
}

fn create_scan_thread(
    ip_list: Vec<ipv4::IPv4>,
    thread_id: u32,
    ips_per_thread: u32,
) -> JoinHandle<Vec<bool>> {
    thread::spawn(move || {
        let results: Vec<bool> = Vec::new();

        // do the scan thing
        for i in 0..ips_per_thread {
            let id = (thread_id * ips_per_thread) + i;
            let ref target = ip_list[id as usize];
            let result = scan(&target);
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
            ip_list,
            thread_id,
            ips_per_thread,
        ));
    }

    // create the last thread to do the job

    Ok(())
}

/*

depth: u32
blacklist_ips: Vec

pre:
    # generate IPs
    ALL_IPS: Vec<Vecu8>> = ...

*/
