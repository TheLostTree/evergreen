#![allow(dead_code)]
mod sniffer;
mod client_server_pair;
mod random_cs;
mod mtkey;
mod key_bruteforce;

use ctrlc;
use protobuf::Message;
use protobuf_json_mapping::print_to_string;
use protos::BattlePassAllDataNotify;

use std::sync::atomic::{AtomicBool, Ordering};
pub static RUNNING: AtomicBool = AtomicBool::new(true);
mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos_target/mod.rs"));
}
mod cmdids{
    include!(concat!(env!("OUT_DIR"), "/cmdids_target/cmdids.rs"));
}

mod proto_decode{
    include!(concat!(env!("OUT_DIR"), "/proto_decode.rs"));
}



fn main() {
    ctrlc::set_handler(move || {
        println!("got signal");
        RUNNING.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // println!("{:?}",protos::GetPlayerTokenReq::file_descriptor());
    // testparse();

    // testbf();

    let sniffing = std::thread::spawn(sniffer::run);


    _ = sniffing.join();
    // mtkey::get_dispatch_keys();
    // key_bruteforce::bruteforce(1658814410247, 4502709363913224634, &[0x0b, 0xb9]);
    println!("closing...")
}

fn testparse(){
    let hexstr = "1a0d30b1ba04480150c203681470031a0d30ccd904480150dc0b680170021a1130b9ba04480350a30558e01268e80770031a0c30c4b204480350e1016896011a0d30d1d904480150dc0b680170021a0d30baba04480150c203680a70031a0f30abba04480150c2035805680a70031a0d30cdd904480150dc0b680170021a0d30b2ba04480150e802681470031a1230afba04480150c20358981168a0c21e70031a0d30cad904480350ca11680c70021a0f30bcba04480150c60a5801680370031a0b30c2b204480350960168041a0f30b3ba04480350e8025814681470031a0e30ced904480150dc0b68b81770021a0d30cfd904480150dc0b680170021a0d30b4ba04480150c203680370031a0b30c3b2044803509601680a1a0d30bdba04480150a305680270031a0d30d0d904480150dc0b680170021a0d30b5ba04480150c203680370031a0a30c1b2044803507868011a0d30cbd904480150dc0b680170021a0f30b0ba04480350c2035818680f70031a1130aeba04480150a30558c60568b00970031a0d30c9d904480350dc0b683270021a0d30bbba04480150c2036802700330017ae8021a084004680178ac8c3d1a084008680178b08c3d1a08400f680178b78c3d1a084014680178bc8c3d1a084005680178ad8c3d1a08400b680178b38c3d1a084009680178b18c3d1a084012680178ba8c3d1a084010680178b88c3d1a084011680178b98c3d1a08400d680178b58c3d1a084016680178be8c3d1a08400a680178b28c3d1a08400e680178b68c3d1a084018680178c08c3d1a084017680178bf8c3d1a084003680178ab8c3d1a084001680178a98c3d1a084013680178bb8c3d1a084015680178bd8c3d1a084007680178af8c3d1a084006680178ae8c3d1a084002680178aa8c3d1a08400c680178b48c3d20012a0e10908ede9e064003589083839f06300138c9044090c7999e0648cb1e501858c81a7090e2f19f067a4b221779735f676c625f62705f6e6f726d616c5f7469657231303a1879735f676c625f62705f757067726164655f746965723132721679735f676c625f62705f65787472615f746965723230";

    let mut datavec = vec![];
    for i in (0..hexstr.len()-1).step_by(2){
        let slice = &hexstr[i..i+2];
        let thing = u8::from_str_radix(slice, 16).unwrap();
        datavec.push(thing);
    }

    let x = BattlePassAllDataNotify::BattlePassAllDataNotify::parse_from_bytes(datavec.as_slice());
    match x{
        Ok(v) => {
            println!("{}", print_to_string(&v).unwrap())
        },
        Err(err) => {
            println!("{:?}", err)
        },
    }
}

