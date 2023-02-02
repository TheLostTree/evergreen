#![allow(dead_code)]
mod sniffer;
mod client_server_pair;
mod random_cs;
mod mtkey;
mod key_bruteforce;

use ctrlc;

use std::sync::atomic::{AtomicBool, Ordering};
pub static RUNNING: AtomicBool = AtomicBool::new(true);
mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos_target/mod.rs"));
}
mod cmdids{
    include!(concat!(env!("OUT_DIR"), "/cmdids_target/cmdids.rs"));
}


fn main() {
    ctrlc::set_handler(move || {
        RUNNING.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // println!("{:?}",protos::GetPlayerTokenReq::file_descriptor());

    // let sniffing = std::thread::spawn(sniffer::run);





    // _ = sniffing.join();
    println!("{:?}",mtkey::get_dispatch_keys());
    // key_bruteforce::bruteforce(1658814410247, 4502709363913224634, &[0x0b, 0xb9]);
    println!("closing...")
}

