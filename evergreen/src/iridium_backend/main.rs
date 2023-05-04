use pcap::Device;
use std::io::{stdin, stdout, Write};
mod iridium_backend;

use evergreen::{
    evergreen::Evergreen,
    packet_processor::{self},
};

fn main() {
    let main_device = get_device().unwrap();
    let mut main = Evergreen::new(main_device);
    main.add_consumer(|| Box::new(packet_processor::PacketProcessor::new()));
    main.add_consumer(|| Box::new(iridium_backend::Iridium::new()));

    main.do_loop();
    // evergreen

    println!("closing...")
}

fn get_device() -> std::option::Option<Device> {
    let devices = Device::list().unwrap();

    for (i, device) in devices.iter().enumerate() {
        //print devices
        println!(
            "{}: Name: {}, Description: {}, Status: {:?}",
            i,
            device.name,
            device.desc.as_ref().unwrap_or(&"None".to_string()),
            device.flags.connection_status
        );
    }
    println!("Choose the device to listen to.");

    let choice = loop {
        let mut inpt = String::new();
        let _ = stdout().flush();
        if let Some(err) = stdin().read_line(&mut inpt).err() {
            println!("{:?}", err);
            return None;
        }
        match inpt.trim().parse::<usize>() {
            Ok(x) => {
                if x >= devices.len() {
                    println!("please give me a number thats IN BOUNDS");
                    continue;
                }
                break x;
            }
            Err(_) => {
                println!("please give me a number... :( ")
            }
        }
    };

    let main_device = devices[choice].clone();

    Some(main_device)
}
