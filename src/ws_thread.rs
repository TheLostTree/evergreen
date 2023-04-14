use std::{sync::mpsc::Receiver, thread::JoinHandle, any::Any};

use serde::{Serialize, Deserialize};
use ws::{Sender, Handler};

pub struct IDKMan{
    join_handle:Option<JoinHandle<()>>,
    pub sender: Option<std::sync::mpsc::Sender<String>>
}
impl IDKMan{
    pub fn new()->Self{
        let (json_sender, ws_receiver) = std::sync::mpsc::channel();
        let ws = std::thread::spawn(move ||{
            crate::ws_thread::run(ws_receiver);
        });

        IDKMan{
            sender: Some(json_sender),
            join_handle:Some(ws)
        }
        // _ = ws.join();
    }

    pub fn join(&mut self)->Option<Result<(), Box<dyn Any + Send>>>{
        //this is kinda smart https://stackoverflow.com/questions/57670145/how-to-store-joinhandle-of-a-thread-to-close-it-later
        self.sender.take().map(|x|drop(x));
        self.join_handle.take().map(JoinHandle::join)
    }
}

struct WSHandle{
    broadcast: Sender,
    handler: Option<JoinHandle<()>>
}

pub fn run(reciever: Receiver<String>){
    let handle = WSHandle::new();
    loop{
        if let Ok(msg) = reciever.recv(){
            // println!("Received: {}", msg);
            handle.broadcast.send(msg).unwrap();
        }else{
            //error
            break;
        }
    }
}

#[derive(Serialize, Deserialize)]
enum WSCmd{
    ProcessFileReq, // lol!
    ConnectReq,
    ConnectRsp, 
    PacketNotify
}



#[derive(Serialize, Deserialize)]
struct PacketNotify{
    cmd: String,
    data: Vec::<IridiumPacket>
}


#[derive(Serialize, Deserialize)]
struct IridiumPacket{
    cmd: WSCmd,
    data: String
}
#[derive(Serialize, Deserialize)]
struct Retcode{
    retcode:i32
}

struct WSMessageHandler{
    out: Sender,
}

impl Handler for WSMessageHandler{
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let iridium_msg :IridiumPacket = serde_json::from_str(&msg.to_string()).unwrap();
        println!("Received: {}", iridium_msg.data);
        match iridium_msg.cmd{
            WSCmd::ConnectReq=>{
                self.out.send(serde_json::to_string(&IridiumPacket{
                    cmd: WSCmd::ConnectRsp,
                    data: "{\"retcode\":0}".to_string()
                }).unwrap()).unwrap();
            },
            _=>{}
        }
        Ok(())
    }
}

impl WSHandle{
    fn new()->Self{
        let server_addr = "127.0.0.1:40510";


        let ws = ws::Builder::new().build(|x|{
            WSMessageHandler{
                out: x
            }}).unwrap();

        let bs = ws.broadcaster();

        let t = std::thread::spawn(move || {
            //this blocks
            _ = ws.listen(server_addr); // idk if i need to do anything with this
        });

        WSHandle { 
            broadcast: bs,
            handler: Some(t)
        }
        // let mut connections = vec![];
    }

    fn join(&mut self){
        self.handler.take().map(JoinHandle::join);
    }

    fn send(&self, msg: String){
        self.broadcast.send(msg).unwrap();
    }
}
