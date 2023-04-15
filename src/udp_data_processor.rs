use crate::client_server_pair::ClientServerPair;
use crate::client_server_pair::Packet;
use crossbeam_channel::{Sender};
use pcap::{Capture, Device};
use std::sync::Arc;
use std::sync::Mutex;

pub struct UdpDataProcessor {
    pub client_server_pair: ClientServerPair,
}

impl UdpDataProcessor {
    pub fn new() -> Self {
        UdpDataProcessor {
            client_server_pair: ClientServerPair::new(),
        }
    }

    pub fn run(&mut self, sender: Sender<Packet>, main_device: Device,running: Arc<Mutex<bool>>) {
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
        .promisc(true)
        .timeout(1)
        //   .snaplen(5000)
        .open()
        .unwrap();

        _ = cap.filter("udp portrange 22101-22102", true);


        println!("evergreen is listening on {}...", device_name);

        while running.lock().unwrap().eq(&true){
            if let Ok(packet) = cap.next_packet() {
                let pktdata = packet.data.to_vec();
                let linktype = cap.get_datalink();
                let (data, port) =
                    remove_headers(pktdata, linktype.eq(&pcap::Linktype::ETHERNET));
                let is_client = port != 22101 && port != 22102;
                if data.len() == 20 {
                    let magic = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);

                    match magic {
                        0x00000145 => {
                            //server sends connect
                            let conv = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
                            let token = u32::from_be_bytes([data[8], data[9], data[10], data[11]]);
                            self.client_server_pair.init_kcp(conv, token);
                            // println!("omg handshake")
                        }
                        0x00000194 => {
                            println!(
                                "{} disconnected",
                                if is_client { "CLIENT" } else { "SERVER" }
                            );
                        }
                        _ => {
                            //unknown?
                            println!("unknown magic: {:x?}", magic)
                        }
                    }
                    continue;
                }


                self.client_server_pair.add_data(&data, is_client);

                //are these clones ideal? no. but it works
                let p1 = self.client_server_pair.recv_kcp(true);
                let p2 = self.client_server_pair.recv_kcp(false);

                if let Some(p) = p1 {
                    sender.send(p).unwrap();
                }
                if let Some(p) = p2 {
                    sender.send(p).unwrap();
                }
            }
        }

        //save 
        self.client_server_pair.key_bf.save();
    }
}

fn remove_headers(mut data: Vec<u8>, is_ether: bool) -> (Vec<u8>, u16) {
    // todo: maybe investigate if cloning the data is necessary
    if is_ether {
        data = (&data[14..]).to_vec();
    }

    let source_port = u16::from_be_bytes([data[20], data[21]]);

    // //remove ipv4 header and udp header
    let data = &data[20 + 8..];
    (Vec::from(data), source_port)
}
