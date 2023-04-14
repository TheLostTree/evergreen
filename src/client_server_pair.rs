use crossbeam_channel::Sender;
use protobuf::Message;
use rsa::pkcs1::DecodeRsaPrivateKey;

use common::cmdids::CmdIds;
use common::protos::{PacketHead, GetPlayerTokenRsp};
use crate::key_bruteforce::KeyBruteforce;
use crate::mtkey::{MTKey, get_dispatch_keys};
use std::io::{Write, self};




pub struct ClientServerPair{
    client: Option<kcp::Kcp<Source>>,
    server: Option<kcp::Kcp<Source>>,
    dispatch_key: Option<MTKey>,
    session_key: Option<MTKey>,


    tokenrspsendtime: Option<u64>,
    tokenrspserverseed: Option<u64>,

    rsa_key: rsa::RsaPrivateKey,
    key_bf: KeyBruteforce,

    // sender: Sender<Packet>,
}

#[derive(Debug,Clone)]
pub struct Packet{
    pub cmdid: u16,
    header_size: u16,
    data_size: u32,
    pub is_client: bool,
    pub header: Vec<u8>,
    pub data: Vec<u8>,

}
impl ClientServerPair{

    pub fn new()->ClientServerPair{
        
        let rsakey = rsa::RsaPrivateKey::from_pkcs1_pem(include_str!("../private.pem")).unwrap();

        let mut p = ClientServerPair{
            client: None,
            server: None,
            dispatch_key: None,
            session_key: None,
            tokenrspsendtime: None,
            tokenrspserverseed: None,
            rsa_key: rsakey,
            key_bf: KeyBruteforce::new(),
            // sender: sender,
        };
        
        // p.server.set_mtu(512);

        // register packet handlers

        p
    }


    pub fn init_kcp(&mut self, conv: u32, token:u32){
        let mut client = kcp::Kcp::new(conv, token, Source {  is_client: true});
        let mut server = kcp::Kcp::new(conv, token, Source {  is_client: true});

        client.set_nodelay(true, 10, 2, false);
        client.set_wndsize(128, 128);
        // p.client.set_mtu(512);

        server.set_nodelay(true, 10, 2, false);
        server.set_wndsize(128, 128);

        _ = self.client.insert(client);
        _ = self.server.insert(server);
    }
    
    fn decode_base64_rsa(&self, data: String)->Vec<u8>{
        let d = base64::decode(data).unwrap();
        // let mut buf = [0; 4096];
        // self.rsa_key.private_decrypt(&d, &mut buf, openssl::rsa::Padding::PKCS1).unwrap();
        let res = self.rsa_key.decrypt(rsa::Pkcs1v15Encrypt, &d);
        res.unwrap()
    }

    pub fn recv_kcp(&mut self, is_client: bool)->Option<Packet>{
        let kcp = if is_client{
            &mut self.client
        }else{
            &mut self.server
        };
        if kcp.is_none() { return None; }
        let kcp = kcp.as_mut().unwrap();

        let size = match kcp.peeksize(){
            Ok(size) => size,
            Err(_) => return None,
        };
        let mut buf = vec![0; size];
        let _ = kcp.recv(&mut buf);
        self.decrypt_packet(&mut buf, is_client)
        // copy data
    }

    pub fn add_data(&mut self, data: &[u8], is_client: bool){
        let kcp = if is_client{
            &mut self.client
        }else{
            &mut self.server
        };
        if kcp.is_none() { return; }
        let kcp = kcp.as_mut().unwrap();
        _ = kcp.input(data);
    }

    fn is_valid(data: &[u8])->bool{
        if data.len() <= 2{
            data[0] == 0x45 && data[1] == 0x67
        }else{
            data[0] == 0x45 && data[1] == 0x67 && data[data.len()-2] == 0x89 && data[data.len()-1] == 0xAB
        }
    }

    fn decrypt_packet(&mut self, data: &mut Vec<u8>, is_client: bool) -> Option<Packet>{
        if let Some(session_key) = &self.session_key{
            session_key.xor(data);

            if !Self::is_valid(data){
                //invalidate session key
                self.session_key = None;
                println!("invalidated session key");
            }

        } else if let Some(dispatch_key) = &self.dispatch_key{
            //test dispatch key xor
            let mut testbuf = [data[0].clone(), data[1].clone()].to_vec();
            dispatch_key.xor(&mut testbuf);
            if Self::is_valid(&testbuf){
                dispatch_key.xor(data);
            } else{
                println!("attempting to bf!");
                //bruteforce session key!
                //also theres definitely a better way to do this than the 3 nested if let Some's
                if let Some(sent_time) = self.tokenrspsendtime{
                    if let Some(server_seed) = self.tokenrspserverseed{
                        if let Some(seed) = self.key_bf.bruteforce(sent_time, server_seed, data){
                            let key = MTKey::from_seed(seed);
                            key.xor(data);

                            self.session_key = Some(key);
                            if !Self::is_valid(data){
                                //invalidate session key
                                self.session_key = None;
                                println!("invalidated session key");
                            }
                            // return self.decrypt_packet(data, is_client);
                            
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
                    // crying screaming 
                    // println!("im rlly sad rn bc bruteforce failed");
                    return None;
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
            return None;
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
        Some(p)

        // send data to ws etc
    }

    fn handle_parsed_packet(&mut self, p: &mut Packet){
        let cmd = CmdIds::from_u16(p.cmdid);
        if let None = cmd{
            println!("unknown cmdid: {}", p.cmdid);
            return;
        }

        let cmd = cmd.unwrap();
        // println!("{}: {:?}", if p.is_client {"CLIENT"} else{"SERVER"},cmd);
        
        let packethead = if p.header.len() > 0 {PacketHead::PacketHead::parse_from_bytes(&p.header).ok()}else{None};
        
        //this really is the only one that matters for the packet parsing
        _ = match cmd{   
            CmdIds::GetPlayerTokenRsp=>{
                if let Some(x) = packethead{
                    self.tokenrspsendtime = Some(x.sent_ms);
                }

                let x = GetPlayerTokenRsp::GetPlayerTokenRsp::parse_from_bytes(&p.data).ok();
                if let Some(v) = x{
                    
                    // self.tokenrspserverseed = Some();
                    //v.serverRandKey
                    let x = self.decode_base64_rsa(v.server_rand_key.clone());
                    self.tokenrspserverseed = Some(u64::from_be_bytes([x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7]]));
                    // Some(protobuf_json_mapping::print_to_string_with_options(&v,&options).unwrap())
                }else{
                    // None
                }

            },
            _ => {}
        };

        // todo: pass to whatever handles the packets

        return;
        // up
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