use protobuf::Message;
use rsa::pkcs1::DecodeRsaPrivateKey;

use crate::cmdids::CmdIds;
use crate::protos::{PacketHead, GetPlayerTokenRsp};
use crate::random_cs::bruteforce;
use crate::{proto_decode};
use crate::mtkey::{MTKey, get_dispatch_keys};
use std::any::Any;
use std::sync::mpsc::{Receiver, Sender};
use std::io::{Write, self};
use std::thread::JoinHandle;


pub fn processing_thread(reciever: Receiver<(Vec<u8>, u16)>){
    let mut pair : Option<ClientServerPair> = None;

    loop {
        match reciever.recv(){
            Ok((data, port)) =>{
                // println!("got data! {}", data.len());
                //handle data
                let is_client = port != 22101 && port != 22102;

                if let Some(pair) = &mut pair{
                    pair.add_data(&data, is_client);
                    pair.recv_kcp(true);
                    pair.recv_kcp(false);
                    
                }else{
                    if data.len() == 20{
                        let magic = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
                        
                        match magic{
                            0x00000145 => {
                                //server sends connect
                                let conv = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
                                let token = u32::from_be_bytes([data[8], data[9], data[10], data[11]]);
                                pair = Some(ClientServerPair::new(conv, token));
                                // println!("omg handshake")
                                
                            },
                            0x00000194 =>{
                                //disconnect lol 
                                //todo: handle
                                if pair.is_some(){
                                    println!("{} disconnected", if is_client {"CLIENT"} else{"SERVER"});
                                    pair = None;
                                }
                            },
                            _ => {
                                //unknown?
                            }
                        }
                    }
                }
        }
            Err(_) => {
                // println!("sender closed...");
                if pair.is_some(){
                    //again, not sure if i have to explicitly do this
                    //drop(json_sender) prob not, its been moved
                }
                break;
            },
        }
    }
    

}

pub struct ClientServerPair{
    client: kcp::Kcp<Source>,
    server: kcp::Kcp<Source>,
    dispatch_key: Option<MTKey>,
    session_key: Option<MTKey>,


    tokenrspsendtime: Option<u64>,
    tokenrspserverseed: Option<u64>,

    rsa_key: rsa::RsaPrivateKey,

    count: i32,

}

pub struct Packet{
    pub cmdid: u16,
    header_size: u16,
    data_size: u32,
    is_client: bool,
    pub header: Vec<u8>,
    pub data: Vec<u8>,
}

impl Packet{

}

impl ClientServerPair{
    pub fn new(conv: u32, token:u32)->ClientServerPair{
        
        let rsakey = rsa::RsaPrivateKey::from_pkcs1_pem(include_str!("../private.pem")).unwrap();

        let mut p = ClientServerPair{
            client: kcp::Kcp::new(conv, token, Source {  is_client: true}),
            server: kcp::Kcp::new(conv, token, Source {  is_client: false}),
            dispatch_key: None,
            session_key: None,
            tokenrspsendtime: None,
            tokenrspserverseed: None,
            rsa_key: rsakey,
            count: 0,
        };
        p.client.set_nodelay(true, 10, 2, false);
        p.client.set_wndsize(128, 128);
        // p.client.set_mtu(512);

        p.server.set_nodelay(true, 10, 2, false);
        p.server.set_wndsize(128, 128);
        // p.server.set_mtu(512);

        // register packet handlers

        p
    }

    
    fn decode_base64_rsa(&self, data: String)->Vec<u8>{
        let d = base64::decode(data).unwrap();
        // let mut buf = [0; 4096];
        // self.rsa_key.private_decrypt(&d, &mut buf, openssl::rsa::Padding::PKCS1).unwrap();
        let res = self.rsa_key.decrypt(rsa::Pkcs1v15Encrypt, &d);
        res.unwrap()
    }


    fn recv_kcp(&mut self, is_client: bool){
        let kcp = if is_client{
            &mut self.client
        }else{
            &mut self.server
        };
        let size = match kcp.peeksize(){
            Ok(size) => size,
            Err(_) => return,
        };
        let mut buf = vec![0; size];
        let _ = kcp.recv(&mut buf);
        self.decrypt_packet(&mut buf, is_client);
        // copy data
    }

    pub fn add_data(&mut self, data: &[u8], is_client: bool){
        if is_client{
            _ = self.client.input(data);
        }else{
            _ = self.server.input(data);
        }
    }

    fn is_valid(data: &[u8])->bool{
        if data.len() <= 2{
            data[0] == 0x45 && data[1] == 0x67
        }else{
            data[0] == 0x45 && data[1] == 0x67 && data[data.len()-2] == 0x89 && data[data.len()-1] == 0xAB
        }
    }

    fn decrypt_packet(&mut self, data: &mut Vec<u8>, is_client: bool){
        //probably xor.
        self.count += 1;
        // let mut contents = String::new();
        // for byte in &mut *data{
        //     contents.push_str(&format!("{:02x}", byte))
        // }

        // println!("packet count {} from client: {} , {}", self.count, is_client, contents);
        if let Some(session_key) = &self.session_key{
            session_key.xor(data);

        } else if let Some(dispatch_key) = &self.dispatch_key{
            //test dispatch key xor
            let mut testbuf = [data[0].clone(), data[1].clone()].to_vec();
            dispatch_key.xor(&mut testbuf);
            if Self::is_valid(&testbuf){
                dispatch_key.xor(data);
            } else{
                println!("attempting to bf!");
                //bruteforce session key!
                if let Some(sent_time) = self.tokenrspsendtime{
                    if let Some(server_seed) = self.tokenrspserverseed{
                        if let Some(seed) = bruteforce(sent_time, server_seed, data){
                            self.session_key = Some(MTKey::from_seed(seed));
                        }else{
                            println!("honestly we just failed");
                        }
                    }else{
                        println!("missing server seed");
                    }
                }else{
                    println!("missing sendtime");
                }

                if let None = self.session_key{
                    //crying screaming 
                    // println!("im rlly sad rn bc bruteforce failed");
                    return;
                }
                
            }
        } else{
            //find dispatch key
            let keys = get_dispatch_keys();
            let first_bytes = u16::from_be_bytes([data[0] ^ 0x45, data[1] ^ 0x67]);
            if keys.contains_key(&first_bytes){
                self.dispatch_key = Some(
                    MTKey { keybuf: keys[&first_bytes].to_vec() }
                );
                self.dispatch_key.as_mut().unwrap().xor(data);
                println!("found key starting with {}",  first_bytes);

            }else{
                println!("cant find key starting with {}", first_bytes);
            }
        }

        if !Self::is_valid(data){
            //crying screaming 
            println!("invalid packet.... encryption probably fucked up");
            return;
        }


        // let magic = u16::from_be_bytes([data[0], data[1]]);
        let cmdid = u16::from_be_bytes([data[2], data[3]]);
        let headsize = u16::from_be_bytes([data[4], data[5]]);
        let bodysize = u32::from_be_bytes([data[6], data[7], data[8], data[9]]);


        let head = &data[10..10+headsize as usize];
        let body = &data[10+headsize as usize..data.len()-2];

        let mut p = Packet{
            cmdid,
            header_size: headsize,
            data_size: bodysize,
            header: head.to_vec(),
            data: body.to_vec(),
            is_client,
        };

        self.handle_parsed_packet(&mut p);

        // send data to ws etc
    }

    fn handle_parsed_packet(&mut self, p: &mut Packet)-> Option<String>{
        let cmd = CmdIds::from_u16(p.cmdid);
        if let None = cmd{
            println!("unknown cmdid: {}", p.cmdid);
            return None;
        }

        let cmd = cmd.unwrap();
        println!("{}: {:?}", if p.is_client {"CLIENT"} else{"SERVER"},cmd);
        
        let packethead = if p.header.len() > 0 {PacketHead::PacketHead::parse_from_bytes(&p.header).ok()}else{None};
        
        let data: Option<String> = match cmd{   
            CmdIds::GetPlayerTokenRsp=>{
                if let Some(x) = packethead{
                    self.tokenrspsendtime = Some(x.sent_ms);
                }

                let x = GetPlayerTokenRsp::GetPlayerTokenRsp::parse_from_bytes(&p.data).ok();
                if let Some(v) = x{
                    
                    // self.tokenrspserverseed = Some();
                    //v.serverRandKey
                    let x = self.decode_base64_rsa(v.serverRandKey.clone());
                    self.tokenrspserverseed = Some(u64::from_be_bytes([x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7]]));
                    // lol!
                    let options = protobuf_json_mapping::PrintOptions{
                        enum_values_int :false,
                        proto_field_name: false,
                        always_output_default_values: true,
                        _future_options: (),
                    };
                    Some(protobuf_json_mapping::print_to_string_with_options(&v,&options).unwrap())
                }else{
                    None
                }

            },
            CmdIds::UnionCmdNotify=>{
                let x = crate::protos::UnionCmdNotify::UnionCmdNotify::parse_from_bytes(p.data.as_slice());
                if let Some(v) = x.ok(){
                    for unioncmd in v.cmdList{
                        let op = self.handle_parsed_packet(&mut Packet{
                            cmdid: unioncmd.messageId as u16,
                            header_size: 0,
                            data_size: unioncmd.body.len() as u32,
                            is_client: true,
                            header: vec![],
                            data: unioncmd.body,
                        });
                    }
                }
                Some(String::new())
            },
            _ =>{
                proto_decode::default_decode_proto(p, cmd)
            },
        };

        if data.is_some(){
            println!("{}", data.clone().unwrap())
        }

        data
        // up
    }
}

struct WSHandle{
    join_handle:Option<JoinHandle<()>>,
    sender: Sender<String>
}
impl WSHandle{
    fn new()->Self{
        let (json_sender, ws_receiver) = std::sync::mpsc::channel();
        let ws = std::thread::spawn(move ||{
            crate::ws_thread::run(ws_receiver);
        });

        WSHandle{
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


// stupid stuff for kcp, ignore please
pub struct Source
{
    is_client: bool,
}

impl Write for Source {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        Ok(data.len())
        //ignore lol!
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}