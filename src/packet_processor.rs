// this code is intended to use the data from parsed packets to calculate dps or whatnot
use std::{collections::HashMap, sync::{Mutex, Arc}, os::unix::thread};
use crossbeam_channel::Receiver;
use protobuf::MessageDyn;
use protobuf_json_mapping::{print_to_string_with_options, PrintOptions};


pub trait PacketProcessorTrait{
    fn process(&mut self, cmdid: CmdIds, bytes: &[u8], is_server: bool);
}


use common::cmdids::CmdIds;

use crate::client_server_pair::Packet;

pub struct PacketProcessor{
    handlers: HashMap<CmdIds, Handler>,
}

type Handler = fn(&mut PacketProcessor, &[u8])-> Option<Box<dyn MessageDyn>>;
type Message = Box<dyn MessageDyn>;

#[allow(unused_variables)]
impl PacketProcessor{
    pub fn new() -> Self{
        let handlers: HashMap<CmdIds, Handler> = HashMap::new();
        // handlers.insert(SceneEntityAppearNotify, Self::scene_entity_appear);
        // handlers.insert(GetPlayerTokenReq, Self::get_player_token);
        Self{
            handlers,
            
        }
    }



    fn send_protobuf(&self, message: &dyn MessageDyn, cmdid: CmdIds, is_server: bool){
        let print_options = PrintOptions{
            always_output_default_values : true,
            ..PrintOptions::default()
        };
        if let Ok(st) = print_to_string_with_options(message, &print_options){
            let mut jsonstr = String::new();
                jsonstr.push_str(r#"{
    "cmd": "PacketNotify",
    "data": [{
        "packetID": "#);
                jsonstr.push_str(&format!("{}", cmdid.clone() as u16));
                jsonstr.push_str(r#",
    "protoName": ""#);
                jsonstr.push_str(&format!("{:?}", cmdid));
                jsonstr.push_str(r#"",
    "object": "#);
                jsonstr.push_str(&st);
                jsonstr.push_str(r#",
    "packet": """#);
                jsonstr.push_str(r#",
    "source": "#);
                jsonstr.push_str(&format!("{}", if is_server{0}else{1}));
                jsonstr.push_str("  }]
}");

            println!("{}", jsonstr);
        }
    }

    fn scene_entity_appear(&mut self, bytes: &[u8])-> Option<Message>{

        None
    }


    fn get_player_token(&mut self, bytes: &[u8])-> Option<Message>{
        None
    }
}


impl PacketProcessorTrait for PacketProcessor{
    fn process(&mut self, cmdid: CmdIds, bytes: &[u8], is_server: bool){
        match self.handlers.get(&cmdid){
            Some(handler) => {
                let msg = handler(self, bytes);
                if let Some(message) = msg{
                    self.send_protobuf(message.as_ref(), cmdid, is_server);
                }
            },
            None => {
                println!("no handler for {:?}", cmdid);
            },
        };
    }
}


