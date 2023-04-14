
use crate::client_server_pair::Packet;
use crate::packet_processor::PacketProcessorTrait;
use crate::{client_server_pair::ClientServerPair};
use crate::{RUNNING};
use crossbeam_channel::{unbounded, Receiver, Sender};
use pcap::{Capture, Device, Active};
use std::io::{stdin, stdout, Write};
use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering;
use std::thread::JoinHandle;


pub struct Evergreen{
    pub client_server_pair: ClientServerPair,

}

impl Evergreen{
    pub fn new()->Self{
        Evergreen{
            client_server_pair: ClientServerPair::new(),
        }
    }


    pub fn run(&mut self, sender: Sender<Packet>){

        
        let (mut cap, device_name) = get_capture().unwrap();

        println!("evergreen is listening on {}...", device_name);

        while RUNNING.load(Ordering::SeqCst) {
            if let Ok(packet) = cap.next_packet() {
                let pktdata = packet.data.to_vec();
                let (data, port) = remove_headers(pktdata, cap.get_datalink().eq(&pcap::Linktype::ETHERNET));
                let is_client = port != 22101 && port != 22102;
                if data.len() == 20{
                    let magic = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
                    
                    match magic{
                        0x00000145 => {
                            //server sends connect
                            let conv = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
                            let token = u32::from_be_bytes([data[8], data[9], data[10], data[11]]);
                            self.client_server_pair.init_kcp(conv, token);
                            // println!("omg handshake")
                            
                        },
                        0x00000194 =>{
                            //disconnect lol 
                            //todo: handle
                            println!("{} disconnected", if is_client {"CLIENT"} else{"SERVER"});

                        },
                        _ => {
                            //unknown?
                            println!("unknown magic: {:x?}", magic)
                        }
                    }
                }
                self.client_server_pair.add_data(&data, is_client);

                    //are these clones ideal? no. but it works
                let p1 = self.client_server_pair.recv_kcp(true);
                let p2 = self.client_server_pair.recv_kcp(false);
                

                if let Some(p) = p1{
                    sender.send(p).unwrap();
                }
                if let Some(p) = p2{
                    sender.send(p).unwrap();
                }

            }
        }

        
    }
    
}





fn remove_headers(mut data: Vec<u8>, is_ether: bool)->(Vec<u8>, u16){
    // todo: maybe investigate if cloning the data is necessary
    if is_ether {
        data = (&data[14..]).to_vec();
    }

    let source_port = u16::from_be_bytes([data[20], data[21]]);


    // //remove ipv4 header and udp header
    let data = &data[20+8..];
    (Vec::from(data), source_port)
}


fn get_capture() -> std::option::Option<(Capture<Active>, String)>{

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

    let main_device = devices[choice].clone();
    

    let device_name = format!(
        "{} {}",
        main_device.name.clone(),
        main_device.desc.as_ref().unwrap_or(&"No Description".to_owned()));

    let mut cap = Capture::from_device(main_device)
        .unwrap()
        .promisc(true).timeout(1)
        //   .snaplen(5000)
        .open()
        .unwrap();

    _ = cap.filter("udp portrange 22101-22102", true);
    Some((cap, device_name))

}