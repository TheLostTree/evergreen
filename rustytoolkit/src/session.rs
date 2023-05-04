use std::{collections::HashMap, sync::Arc};

use common::cmdids::CmdIds;
use crossbeam_channel::Receiver;
use protobuf::Message;
use evergreen::{packet_processor::PacketConsumer, client_server_pair::Packet};
use common::protos::*;


pub struct Session{
    handlers: HashMap<CmdIds, Handler>,

}


type Handler = fn(&mut Session, &[u8]) -> ();

impl PacketConsumer for Session{
    fn run(&mut self, rx: Receiver<Arc<Packet>>) {
        for packet in rx {
            if let Some(cmdid) = common::cmdids::CmdIds::from_u16(packet.cmdid) {
                self.process(cmdid, &packet.data, !packet.is_client);
            }
        }
    }

    fn process(&mut self, cmdid: common::cmdids::CmdIds, bytes: &[u8], _is_server: bool) {
        if let Some(handler) = self.handlers.get(&cmdid) {
            handler(self, bytes);
        }
    }
}


impl Session{
    pub fn  new() -> Self{
        let mut handlers: HashMap<CmdIds, Handler> = HashMap::new();
        handlers.insert(CmdIds::SceneEntityAppearNotify, Self::on_scene_entity_appear);
        Self{handlers}
    }



    fn on_scene_entity_appear(&mut self, bytes: &[u8]){
        let msg = SceneEntityAppearNotify::SceneEntityAppearNotify::parse_from_bytes(bytes).unwrap();



        for entity in msg.entity_list.into_iter(){
            match msg.appear_type.enum_value_or(VisionType::VisionType::VISION_NONE){
                VisionType::VisionType::VISION_BORN =>{

                },
                VisionType::VisionType::VISION_NONE => todo!(),
                VisionType::VisionType::VISION_MEET => todo!(),
                VisionType::VisionType::VISION_REBORN => todo!(),
                VisionType::VisionType::VISION_REPLACE => todo!(),
                VisionType::VisionType::VISION_WAYPOINT_REBORN => todo!(),
                VisionType::VisionType::VISION_MISS => todo!(),
                VisionType::VisionType::VISION_DIE => todo!(),
                VisionType::VisionType::VISION_GATHER_ESCAPE => todo!(),
                VisionType::VisionType::VISION_REFRESH => todo!(),
                VisionType::VisionType::VISION_TRANSPORT => todo!(),
                VisionType::VisionType::VISION_REPLACE_DIE => todo!(),
                VisionType::VisionType::VISION_REPLACE_NO_NOTIFY => todo!(),
                VisionType::VisionType::VISION_PICKUP => todo!(),
                VisionType::VisionType::VISION_REMOVE => todo!(),
                VisionType::VisionType::VISION_CHANGE_COSTUME => todo!(),
                VisionType::VisionType::VISION_FISH_REFRESH => todo!(),
                VisionType::VisionType::VISION_FISH_BIG_SHOCK => todo!(),
                VisionType::VisionType::VISION_FISH_QTE_SUCC => todo!(),
                VisionType::VisionType::VISION_CAPTURE_DISAPPEAR => todo!(),

            }
        }
        
    }

}

