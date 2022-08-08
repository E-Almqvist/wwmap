use crate::ipv4;
use log::info;
use std::thread;
use std::{
    net::{SocketAddr, TcpStream},
    thread::JoinHandle,
};

async fn check_ack(dest: &SocketAddr) -> bool {
    if let Ok(res) = TcpStream::connect(dest) {
        info!("Got TCP ack from: {:?}", dest);
        return true;
    }
    false
}

pub fn start_scan(depth: u32) {
    let ip_list = ipv4::get_all(None);

    let mut threads: Vec<JoinHandle<()>> = Vec::new();

    for i in 0..depth {
        let thread = thread::spawn(|| {
            println!("hi");
            // do scan
        });
        threads.push(thread);
    }
}

/*

depth: u32
blacklist_ips: Vec

pre:
    # generate IPs
    ALL_IPS: Vec<Vecu8>> = ...

*/
