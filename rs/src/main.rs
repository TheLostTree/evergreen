
use std::{alloc::System, ptr::read};
use std::io::{stdin, stdout, Write};
use pcap::{Device,Capture};

fn main() {
    let devices = Device::list().unwrap();
    let def = Device::lookup().unwrap();
    println!("Default device: {:?}", def);

    for (i, device) in devices.iter().enumerate(){
        //print devices
        println!("{}: {:?}", i, device);
    }
    println!("Choose the device to listen to.");

    let mut inpt = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut inpt).unwrap();
    let choice : usize= inpt.trim().parse().unwrap();





    let main_device = (&devices[choice]).clone();


    let mut cap = Capture::from_device(main_device).unwrap()
                      .promisc(true)
                    //   .snaplen(5000)
                      .open().unwrap();

    println!("listening on {}...", &devices[choice].name);
    let mut count = 0;
    while let Ok(packet) = cap.next_packet() {
        println!("received packet! {:?}", count);
        count += 1;
    }
}