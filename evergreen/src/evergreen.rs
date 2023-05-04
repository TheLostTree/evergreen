use crate::client_server_pair::Packet;
use crate::packet_processor::PacketConsumer;
use crate::udp_data_processor::UdpDataProcessor;
use crossbeam_channel::{unbounded, Receiver, Sender};
use pcap::Device;
use std::{sync::{Arc, Mutex}};

pub struct Evergreen {
    running: Arc<Mutex<bool>>,
    senders: Vec<Sender<Arc<Packet>>>,

    packet_rx: Receiver<Packet>,
}

impl Evergreen {
    pub fn new(d: Device) -> Self {
        let running: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
        {
            let rclone = running.clone();
            _ = ctrlc::set_handler(move || {
                println!("got signal");
                *rclone.lock().unwrap() = false;
            });  // lol!
        }
        let (s, r) = unbounded();

        let rclone = running.clone();
        _ = std::thread::spawn(move || {
            let mut udpdataprocessor = UdpDataProcessor::new();
            udpdataprocessor.run(s, d, rclone);
        });

        Self {
            running: running,
            senders: vec![],
            packet_rx: r,
        }
    }

    fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }

    
    pub fn add_consumer(&mut self, f: fn() -> Box<dyn PacketConsumer>) {
        let (pptx, pprx) = unbounded();
        _ = std::thread::spawn(move || {
            f().run(pprx);
        });
        self.senders.push(pptx);
    }

    pub fn do_loop(&mut self) {
        loop {
            match self.packet_rx.recv() {
                Ok(x) => {
                    let rc_pkt = Arc::new(x);
                    for sender in self.senders.iter() {
                        sender.send(rc_pkt.clone()).unwrap();
                    }
                }
                Err(_) => {
                    println!("channel closed");
                    break;
                }
            };
            if !self.is_running() {
                break;
            }
        }
    }
}


impl Drop for Evergreen {
    fn drop(&mut self) {
        println!("dropping evergreen");
        *self.running.lock().unwrap() = false;
    }
}