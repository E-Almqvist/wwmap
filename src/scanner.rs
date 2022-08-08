use std::{net::{TcpStream, SocketAddr}, thread::JoinHandle};
use log::info;
use std::thread;
use crate::permutations;

async fn check_ack(dest: &SocketAddr) -> bool {
    if let Ok(res) = TcpStream::connect(dest) {
        info!("Got TCP ack from: {:?}", dest);
        return true;
    }
    false
}

pub fn start_scan(depth: u32) {
    let ip_list = permutations::ipv4(None);

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

