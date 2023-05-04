// this code is intended to use the data from parsed packets to calculate dps or whatnot
use crossbeam_channel::Receiver;
use std::{collections::HashMap, sync::Arc};
use common::cmdids::CmdIds;
use crate::client_server_pair::Packet;


// use (*arc_t).clone() to get the data as mutable (or a copy of it anyhow)
// otherwise i'm just using one reference everywhere to one packet
// since packets can get pretty big
pub trait PacketConsumer {
    fn process(&mut self, cmdid: CmdIds, bytes: &[u8], is_server: bool);
    fn run(&mut self, rx: Receiver<Arc<Packet>>) {
        for packet in rx {
            self.process(
                CmdIds::from_u16(packet.cmdid).unwrap(),
                &packet.data,
                !packet.is_client,
            );
        }
        println!("PacketConsumer::run() finished");
    }
}



//example

pub struct PacketProcessor {
    handlers: HashMap<CmdIds, Handler>,
    count: u32,
}

type Handler = fn(&mut PacketProcessor, &[u8]) -> ();
// type Message = Box<dyn MessageDyn>;

#[allow(unused_variables)]
impl PacketProcessor {
    pub fn new() -> Self {
        let mut handlers: HashMap<CmdIds, Handler> = HashMap::new();
        _= handlers.insert(CmdIds::SceneEntityAppearNotify, Self::scene_entity_appear);
        // handlers.insert(GetPlayerTokenReq, Self::get_player_token);
        Self { handlers, count: 0 }
    }
    fn scene_entity_appear(&mut self, bytes: &[u8]) {
        println!("scene_entity_appear: {:?}", bytes);
    }
}

impl PacketConsumer for PacketProcessor {
    fn process(&mut self, cmdid: CmdIds, bytes: &[u8], is_server: bool) {
        println!(
            "#{} {}: {:?}",
            {
                self.count += 1;
                self.count
            },
            if is_server { "S2C" } else { "C2S" },
            cmdid
        );
        match self.handlers.get(&cmdid) {
            Some(handler) => {
                let _ = handler(self, bytes);
            }
            None => {
            }
        };
    }

}

