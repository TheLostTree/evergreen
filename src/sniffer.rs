use crate::{RUNNING, client_server_pair, ws_thread};
use pcap::{Capture, Device};
use std::io::{stdin, stdout, Write};
use std::sync::atomic::Ordering;



pub fn run(){
    let main_device = get_device(false).unwrap();

    let device_name = format!(
        "{} {}",
        main_device.name.clone(),
        main_device
            .desc
            .as_ref()
            .unwrap_or(&"No Description".to_owned())
    );

    let mut cap = Capture::from_device(main_device)
        .unwrap()
        .promisc(true).timeout(1)
        //   .snaplen(5000)
        .open()
        .unwrap();

    _ = cap.filter("udp portrange 22101-22102", true);
    println!("listening on {}...", device_name);

    let (packet_sender, packet_receiver) = std::sync::mpsc::channel();

    let processing_thread = std::thread::spawn(move||{
        client_server_pair::processing_thread(packet_receiver)
    });


    

    while RUNNING.load(Ordering::SeqCst) {
        if let Ok(packet) = cap.next_packet() {
            let pktdata = packet.data.to_vec();
            _ = packet_sender.send(remove_headers(pktdata));
        }
    }

    // not *quite* sure i have to do this explicitly
    drop(packet_sender);
    // drop(json_sender);

    _ = processing_thread.join();
}


fn remove_headers(data: Vec<u8>)->(Vec<u8>, u16){
    // let source_addr = u32::from_be_bytes([data[12], data[13], data[14], data[15]]);

    //remove ethernet header
    // if using a vpn, ignore this lol
    // let data = &data[14..];
    //beginning of udp header
    let source_port = u16::from_be_bytes([data[20], data[21]]);

    // //remove ipv4 header and udp header
    let data = &data[20+8..];
    (Vec::from(data), source_port)
}


fn get_device(default: bool) -> Option<Device> {
    if default {
        return Device::lookup().unwrap();
    }
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

    let choice = loop{
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
    

    Some(devices[choice].clone())
}