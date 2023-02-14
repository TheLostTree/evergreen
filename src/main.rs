#![allow(dead_code)]
mod sniffer;
mod client_server_pair;
mod random_cs;
mod mtkey;
mod packet_processor;
mod ws_thread;


use ctrlc;
use std::{sync::atomic::{AtomicBool, Ordering}};

pub static RUNNING: AtomicBool = AtomicBool::new(true);

mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos_target/mod.rs"));
}
mod cmdids{
    include!(concat!(env!("OUT_DIR"), "/cmdids_target/cmdids.rs"));
}
// mod proto_decode{
//     include!(concat!(env!("OUT_DIR"), "/proto_decode.rs"));
// }



fn main() {
    ctrlc::set_handler(move || {
        println!("got signal");
        RUNNING.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // let start = std::time::Instant::now();
    // let protos = crate::packet_processor::load_dyn_protos();
    // let end = std::time::Instant::now();
    // println!("loaded {} protos in {:?}", protos.len(),end - start);

    let sniffing = std::thread::spawn(sniffer::run);

    _ = sniffing.join();
    println!("closing...")
}

