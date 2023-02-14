use std::{sync::mpsc::Receiver, str::FromStr, thread::JoinHandle, any::Any};

use serde::{Serialize, Deserialize};
use ws::{Sender, Handler};

struct WSHandle1{
    join_handle:Option<JoinHandle<()>>,
    sender: std::sync::mpsc::Sender<String>
}
impl WSHandle1{
    fn new()->Self{
        let (json_sender, ws_receiver) = std::sync::mpsc::channel();
        let ws = std::thread::spawn(move ||{
            crate::ws_thread::run(ws_receiver);
        });

        WSHandle1{
            sender: json_sender,
            join_handle:Some(ws)
        }
        // _ = ws.join();
    }

    fn join(&mut self)->Option<Result<(), Box<dyn Any + Send>>>{
        //this is kinda smart https://stackoverflow.com/questions/57670145/how-to-store-joinhandle-of-a-thread-to-close-it-later
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
            println!("Received: {}", msg);
            handle.broadcast.send(msg).unwrap();
        }else{
            //error

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
                    data: "lol".to_string()
                }).unwrap()).unwrap();
            },
            _=>{}
        }

        Err(ws::Error::new(ws::ErrorKind::Internal, "Not implemented"))
    }
}

impl WSHandle{
    fn new()->Self{
        let server_addr = "ws://127.0.0.1:40510";
        // let recievers = vec![];
        let mut ws = ws::WebSocket::new(|x|{
            WSMessageHandler{
                out: x
            }
        }).unwrap();

        ws.connect(url::Url::from_str(server_addr).unwrap()).unwrap();

        let mut x = WSHandle { 
            broadcast:  ws.broadcaster(),
            handler: None
        };
        let t = std::thread::spawn(move || {
            _ = ws.run(); // idk if i need to do anything with this
        });

        //this is kinda dumb tbh
        x.handler = Some(t);

        x

        // let mut connections = vec![];
    }

    fn join(&mut self){
        self.handler.take().map(JoinHandle::join);
    }

    fn send(&self, msg: String){
        self.broadcast.send(msg).unwrap();
    }
}
