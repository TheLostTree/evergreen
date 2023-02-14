// this code is intended to use the data from parsed packets to calculate dps or whatnot
use std::collections::HashMap;
use protobuf::{MessageDyn, descriptor::FileDescriptorProto};
use protobuf_parse::Parser;

use crate::cmdids::CmdIds::{*, self};

pub struct PacketProcessor{
    handlers: HashMap<CmdIds, Handler>,
}
type Handler = fn(&mut PacketProcessor, &[u8])-> Option<Box<dyn MessageDyn>>;
type Message = Box<dyn MessageDyn>;
pub fn load_dyn_protos()->Vec<FileDescriptorProto>{
    let x = Parser::new().pure()
    .inputs(std::fs::read_dir("./all_protos").unwrap().map(|v|v.unwrap().path()))
    .include("./all_protos")
    .parse_and_typecheck().unwrap();
    // x.file_descriptors
    //haha.... clone....
    println!("therers only {} descriptors actually", x.file_descriptors.len());

    x.file_descriptors
}

#[allow(unused_variables)]
impl PacketProcessor{
    pub fn new() -> Self{
        let mut handlers: HashMap<CmdIds, Handler> = HashMap::new();
        handlers.insert(SceneEntityAppearNotify, Self::scene_entity_appear);
        handlers.insert(GetPlayerTokenReq, Self::get_player_token);
        Self{
            handlers,
        }
    }

    

    pub fn process(&mut self, cmdid: CmdIds, bytes: &[u8]){
        match self.handlers.get(&cmdid){
            Some(handler) => {
                handler(self, bytes);
            },
            None => {

            },
        };
    }

    fn scene_entity_appear(&mut self, bytes: &[u8])-> Option<Message>{

        None
    }


    fn get_player_token(&mut self, bytes: &[u8])-> Option<Message>{
        None
    }
}


