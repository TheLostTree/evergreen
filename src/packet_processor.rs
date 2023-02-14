// this code is intended to use the data from parsed packets to calculate dps or whatnot
use std::{collections::HashMap, str::FromStr, thread::JoinHandle};
use protobuf::{MessageDyn, reflect::FileDescriptor};
use protobuf_json_mapping::{print_to_string_with_options, PrintOptions};
use protobuf_parse::Parser;

use crate::{cmdids::CmdIds::{*, self}, ws_thread::IDKMan};

pub struct PacketProcessor{
    handlers: HashMap<CmdIds, Handler>,
    descriptors: Option<HashMap<CmdIds, FileDescriptor>>,
    ws: IDKMan,

    descriptor_load: Option<JoinHandle<HashMap<CmdIds, FileDescriptor>>>
}
type Handler = fn(&mut PacketProcessor, &[u8])-> Option<Box<dyn MessageDyn>>;
type Message = Box<dyn MessageDyn>;
pub fn load_dyn_protos()->HashMap<CmdIds, FileDescriptor>{
    let x = Parser::new().pure()
    .inputs(std::fs::read_dir("./all_protos").unwrap().map(|v|v.unwrap().path()))
    .include("./all_protos")
    .parse_and_typecheck().unwrap();
    // x.file_descriptors
    //haha.... clone....
    println!("therers only {} descriptors actually", x.file_descriptors.len());
    let mut map = HashMap::new();
    for descriptor in FileDescriptor::new_dynamic_fds(x.file_descriptors, &[]).expect("oopsie!"){
        let cmd = CmdIds::from_str(descriptor.name());
        if let Ok(c) = cmd{
            map.insert(c, descriptor);
        }
    }

    map
    
}

#[allow(unused_variables)]
impl PacketProcessor{
    pub fn new() -> Self{
        let mut handlers: HashMap<CmdIds, Handler> = HashMap::new();
        // handlers.insert(SceneEntityAppearNotify, Self::scene_entity_appear);
        // handlers.insert(GetPlayerTokenReq, Self::get_player_token);
        Self{
            handlers,
            ws: IDKMan::new(),
            descriptors: None,
            descriptor_load: Some(std::thread::spawn(||{
                load_dyn_protos()
            }))
        }
    }

    

    pub fn process(&mut self, cmdid: CmdIds, bytes: &[u8]){
        match self.handlers.get(&cmdid){
            Some(handler) => {
                let msg = handler(self, bytes);
                if let Some(message) = msg{
                    self.send_protobuf(message.as_ref())
                }
            },
            None => {
                if let Some(x) = &self.descriptors{
                    let msg = x.get(&cmdid).unwrap().message_by_full_name(&format!("{}",cmdid)).unwrap();
                    if let Ok(b) = msg.parse_from_bytes(bytes){
                        self.send_protobuf(b.as_ref());
                    }
                } else{
                    // oh god....
                    self.descriptors = Some(self.descriptor_load.take().map(|f|JoinHandle::join(f)).unwrap().unwrap());
                    if let Some(fdesc) = self.descriptors.as_ref().expect("hurr durr").get(&cmdid){

                        match fdesc.message_by_full_name(&cmdid.to_string()){
                            Some(msg) => {
                                if let Ok(b) = msg.parse_from_bytes(bytes){
                                    self.send_protobuf(b.as_ref());
                                };
                            },
                            None => {
                                println!("cmdid: {}", cmdid.to_string());
                                println!("messages expected: {:?}", fdesc.messages().map(|v|v.name().to_string()).collect::<Vec<_>>());
                                
                            }
                        }
                    }
                    
                }
            },
        };
    }

    fn send_protobuf(&self, message: &dyn MessageDyn){
        let print_options = PrintOptions{
            always_output_default_values : true,
            ..PrintOptions::default()
        };
        if let Ok(st) = print_to_string_with_options(message, &print_options){
            if let Some(sender) = &self.ws.sender{
                _ = sender.send(st);
            }
        }
    }

    fn scene_entity_appear(&mut self, bytes: &[u8])-> Option<Message>{

        None
    }


    fn get_player_token(&mut self, bytes: &[u8])-> Option<Message>{
        None
    }
}


