use evergreen::{packet_processor::PacketConsumer, protobuf_decoder};
use common::cmdids::CmdIds;
use protobuf::MessageDyn;
use protobuf_json_mapping::{print_to_string_with_options, PrintOptions};

pub struct Iridium {
    broadcast: ws::Sender,
    decoder: protobuf_decoder::DynProtoHandler,
}

impl Iridium {
    pub fn new() -> Self {
        let server_addr = "127.0.0.1:40510";

        let ws = ws::Builder::new().build(|_| |_| Ok(())).unwrap();

        let bs = ws.broadcaster();
        std::thread::spawn(move || {
            ws.listen(server_addr).unwrap();
        });
        Self {
            broadcast: bs,
            decoder: protobuf_decoder::DynProtoHandler::new(),
        }
    }
    fn send(&self, msg: String) {
        _ = self.broadcast.send(msg);
    }

    fn send_protobuf(&self, message: &dyn MessageDyn, cmdid: CmdIds, is_server: bool) {
        let print_options = PrintOptions {
            always_output_default_values: true,
            ..PrintOptions::default()
        };
        if let Ok(st) = print_to_string_with_options(message, &print_options) {
            let mut jsonstr = String::new();
            jsonstr.push_str(
                r#"{
    "cmd": "PacketNotify",
    "data": [{
        "packetID": "#,
            );
            jsonstr.push_str(&format!("{}", cmdid.clone() as u16));
            jsonstr.push_str(
                r#",
    "protoName": ""#,
            );
            jsonstr.push_str(&format!("{:?}", cmdid));
            jsonstr.push_str(
                r#"",
    "object": "#,
            );
            jsonstr.push_str(&st);
            jsonstr.push_str(
                r#",
    "packet": """#,
            );
            jsonstr.push_str(
                r#",
    "source": "#,
            );
            jsonstr.push_str(&format!("{}", if is_server { 0 } else { 1 }));
            jsonstr.push_str(
                "  }]
}",
            );

            self.send(jsonstr)
        }
    }
}

impl PacketConsumer for Iridium {
    fn process(&mut self, cmdid: CmdIds, bytes: &[u8], is_server: bool) {
        let message = self.decoder.decode(cmdid.clone(), bytes);
        if let Some(message) = message {
            self.send_protobuf(message.as_ref(), cmdid, is_server);
        }
    }
}
