// this code is intended to use the data from parsed packets to calculate dps or whatnot
use crossbeam_channel::Receiver;
use protobuf::MessageDyn;
use std::collections::HashMap;

pub trait PacketConsumer {
    fn process(&mut self, cmdid: CmdIds, bytes: &[u8], is_server: bool);
    fn run(&mut self, rx: Receiver<Packet>);
}

use common::cmdids::CmdIds;

use crate::client_server_pair::Packet;

pub struct PacketProcessor {
    handlers: HashMap<CmdIds, Handler>,
    count: u32,
}

type Handler = fn(&mut PacketProcessor, &[u8]) -> Option<Box<dyn MessageDyn>>;
// type Message = Box<dyn MessageDyn>;

#[allow(unused_variables)]
impl PacketProcessor {
    pub fn new() -> Self {
        let handlers: HashMap<CmdIds, Handler> = HashMap::new();
        // handlers.insert(SceneEntityAppearNotify, Self::scene_entity_appear);
        // handlers.insert(GetPlayerTokenReq, Self::get_player_token);
        Self { handlers , count: 0}
    }


}

impl PacketConsumer for PacketProcessor {
    fn process(&mut self, cmdid: CmdIds, bytes: &[u8], is_server: bool) {
        println!("#{} {}: {:?}", {self.count += 1; self.count}, if is_server { "S2C" } else { "C2S" },cmdid);
        match self.handlers.get(&cmdid) {
            Some(handler) => {
                let _ = handler(self, bytes);
            }
            None => {
                // println!("no handler for {:?}", cmdid);
            }
        };
    }

    fn run(&mut self, rx: Receiver<Packet>) {
        for packet in rx {
            self.process(CmdIds::from_u16(packet.cmdid).unwrap(), &packet.data, !packet.is_client);
        }
    }
}
