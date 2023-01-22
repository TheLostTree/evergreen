use ctrlc;
use pcap::{Capture, Device};
use std::io::{stdin, stdout, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use tungstenite::{connect, Message, WebSocket};
use tungstenite::stream::MaybeTlsStream;
use url::Url;


static RUNNING: AtomicBool = AtomicBool::new(true);

fn main() {
    // let running = Arc::new(AtomicBool::new(true));
    //Url::parse("ws://localhost:3012/socket").unwrap()

    let ws_url = Url::parse("ws://localhost:3012").unwrap();
    ctrlc::set_handler(move || {
        RUNNING.store(false, Ordering::SeqCst);
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

    let mut socket = connect_to_ws(&ws_url).unwrap();
    println!("Successfully connected to ws");

    let mut cap = Capture::from_device(main_device)
        .unwrap()
        .promisc(true).timeout(1)
        //   .snaplen(5000)
        .open()
        .unwrap();

        // _ = cap.filter("udp portrange 22101-22102", false);
    _ = cap.filter("udp", false);

    println!("listening on {}...", device_name);

    let mut count = 0;

    while RUNNING.load(Ordering::SeqCst) {
        if let Ok(packet) = cap.next_packet() {
            println!("received packet! {} size: {}", count, packet.len());
            count += 1;
            let res = socket
                .write_message(Message::Binary(packet.data.to_vec()));
                
            if let Err(err) = res{
                println!("error sending packet: {}", err);
                match err{
                    tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed | tungstenite::Error::Io(_)=> {
                        //todo: something where i attempt to reconnect every second if im not connected
                        socket = match connect_to_ws(&ws_url){
                            Some(x) => x,
                            None => {
                                println!("canceling ig");
                                continue;
                            }
                        };
                    },
                    _ => {
                        // println!("error sending packet: {}", err);
                    }
                }
            }
                
                //todo: it may fail panic here if i close ws
        }
    }

    println!("closing...")
}


fn connect_to_ws(req: &Url)-> Option<WebSocket<MaybeTlsStream<TcpStream>>>{
    let socket = loop {
        if RUNNING.load(Ordering::SeqCst) == false{
            break None;
        }
        match connect(req) {
            Ok((socket, _)) => {
                println!("Connected to the server");
                break Some(socket);
            }
            Err(err) => {
                println!("Error connecting to server, waiting 1 second to reconnect: {}", err);
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    };
    socket
    
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
