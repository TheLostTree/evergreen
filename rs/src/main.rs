use ctrlc;
use pcap::{Capture, Device};
use std::io::{stdin, stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tungstenite::{connect, Message};
use url::Url;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    //Url::parse("ws://localhost:3012/socket").unwrap()

    let ws_url = Url::parse("ws://localhost:3012").unwrap();


    let (mut socket, response) =
        connect(&ws_url).expect("Can't connect");


    println!("Connected to the server");


    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

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

    _ = cap.filter("udp portrange 22101-22102", false);

    println!("listening on {}...", device_name);

    let mut count = 0;

    while running.load(Ordering::SeqCst) {
        if let Ok(packet) = cap.next_packet() {
            println!("received packet! {} size: {}", count, packet.len());
            count += 1;
            let res = socket
                .write_message(Message::Binary(packet.data.to_vec()));
            if let Err(err) = res{
                match err{
                    tungstenite::Error::ConnectionClosed => {
                        //todo: something where i attempt to reconnect every second if im not connected
                        connect(&ws_url).expect("Can't connect");
                    },
                    tungstenite::Error::AlreadyClosed => {
                    },
                    tungstenite::Error::Io(_) => todo!(),
                    tungstenite::Error::Tls(_) => todo!(),
                    tungstenite::Error::Capacity(_) => todo!(),
                    tungstenite::Error::Protocol(_) => todo!(),
                    tungstenite::Error::SendQueueFull(_) => todo!(),
                    tungstenite::Error::Utf8 => todo!(),
                    tungstenite::Error::Url(_) => todo!(),
                    tungstenite::Error::Http(_) => todo!(),
                    tungstenite::Error::HttpFormat(_) => todo!(),
                }
            }
                
                //todo: it may fail panic here if i close ws
        }
    }

    println!("closing...")
}

fn get_device(default: bool) -> Option<Device> {
    if default {
        return Device::lookup().unwrap();
    }
    let devices = Device::list().unwrap();

    for (i, device) in devices.iter().enumerate() {
        //print devices
        println!(
            "{}: Name: {}, Description: {}, Connected: {:?}",
            i,
            device.name,
            device.desc.as_ref().unwrap_or(&"None".to_string()),
            device.flags.connection_status
        );
    }
    println!("Choose the device to listen to.");

    let mut inpt = String::new();
    let _ = stdout().flush();
    if let Some(err) = stdin().read_line(&mut inpt).err() {
        println!("{:?}", err);
        return None;
    }

    let choice = loop {
        match inpt.trim().parse::<usize>() {
            Ok(x) => {
                if x > devices.len() {
                    println!("please give me a number thats IN BOUNDS.: ");
                    continue;
                }
                break x;
            }
            Err(_) => {
                println!("please give me a number: ")
            }
        }
    };
    Some(devices[choice].clone())

    // let main_device = (&devices[choice]).clone();
}
