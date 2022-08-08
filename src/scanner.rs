use crate::ipv4;
use anyhow::{Result, anyhow};
use log::info;
use std::thread;
use std::{
    net::{SocketAddr, TcpStream},
    thread::JoinHandle,
};

async fn tcp_scan(dest: &SocketAddr) -> bool {
    if let Ok(res) = TcpStream::connect(dest) {
        info!("Got TCP ack from: {:?}", dest);
        return true;
    }
    false
}

pub fn start_scan(target_port: u16, num_threads: u32, ignorelist: Vec<u64>) -> Result<()> {
    let ip_list = ipv4::get_all(None)?;

    // casting hell
    // Get the amount of needed threads
    let needed_threads = ((ip_list.len() as f32) / num_threads as f32).ceil() as u32;

    // Container for all of our threads
    let mut threads: Vec<JoinHandle<()>> = Vec::new();

    // Create all of our threads
    for i in 0..needed_threads {
        let thread = thread::spawn(|| {
            // do scan
            let target = ip_list[i as usize].to_socketaddr(target_port)?;
        });
        threads.push(thread);
    }

    Ok(())
}

/*

depth: u32
blacklist_ips: Vec

pre:
    # generate IPs
    ALL_IPS: Vec<Vecu8>> = ...

*/
