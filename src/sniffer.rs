use crate::RUNNING;
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

    println!("Successfully connected to ws");

    let mut cap = Capture::from_device(main_device)
        .unwrap()
        .promisc(true).timeout(1)
        //   .snaplen(5000)
        .open()
        .unwrap();

    _ = cap.filter("udp portrange 22101-22102", true);
    println!("listening on {}...", device_name);

    let mut count = 0;

    

    while RUNNING.load(Ordering::SeqCst) {
        if let Ok(packet) = cap.next_packet() {
            println!("received packet! {} size: {}", count, packet.len());
            count += 1;
            let pktdata = packet.data.to_vec();
            println!("{:?}", pktdata);
            // handle data
            
        }
    }
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