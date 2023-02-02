mod sniffer;
mod client_server_pair;
mod random_cs;
mod mtkey;
use ctrlc;

use std::sync::atomic::{AtomicBool, Ordering};
pub static RUNNING: AtomicBool = AtomicBool::new(true);
mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos_target/mod.rs"));
}


fn main() {
    ctrlc::set_handler(move || {
        RUNNING.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    println!("{:?}",protos::GetPlayerTokenReq::file_descriptor());

    //is this worth it rather than std::thread::spawn? idk man
    // let sniffing = std::thread::Builder::new().name("Sniffer".into()).spawn(sniffer::run).expect(":(");



    // _ = sniffing.join();

    // let mut rand = random_cs::Random::with_seed(1);
    // // println!("Random(0).next_double() = {}", rand.next_double());
    // //c#: 0.72624326996796
    // //rs: 0.7262432699679598
    // println!("Random(0).next_safe_uint64() = {}", rand.next_safe_uint64());
    //c#: 13396823736352909312
    //rs: 13396823736352909312
    //rs: 4587125731117596160
    //c#: 4587125731117596160

    let mut mt = mtkey::MT19937_64::default();
    mt.seed(10);
    println!("MT19937_64.seed(0).next_i64() = {}", mt.next_ulong());
    //ts: 11091715596963791794n
    //rs: 11091715596963791794

    


    

    println!("closing...")
}

