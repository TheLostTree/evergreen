#![allow(dead_code)]
mod client_server_pair;
mod key_bruteforce;
mod mtkey;
mod packet_processor;
mod ws_thread;
mod evergreen;
mod protobuf_decoder;


use crossbeam_channel::unbounded;
use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::evergreen::Evergreen;
pub static RUNNING: AtomicBool = AtomicBool::new(true);


// mod proto_decode{
//     include!(concat!(env!("OUT_DIR"), "/proto_decode.rs"));
// }

use pcap::Device;

fn test(){
    let devices = Device::list().unwrap();
    for dev in devices{
        if dev.flags.connection_status != pcap::ConnectionStatus::Connected{
            continue;
        }



        println!("name: {}", dev.name);
        println!("desc: {:?}", dev.desc);
        println!("addresses: {:?}", dev.addresses);
        println!("flags: {:?}", dev.flags);

        let cap = dev.open().unwrap();

        println!("datalinks: {:?}", cap.list_datalinks().unwrap().iter().map(|x|x.get_description()).collect::<Vec<_>>());
        println!("");

    }
}

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

    let (s, r) = unbounded();
    let mut evergreen = Evergreen::new();

    evergreen.run(s);






    // evergreen

    println!("closing...")
}

