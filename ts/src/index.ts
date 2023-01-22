import * as ws from "ws";
import { decodePacket } from "./PacketUtil";
import { UdpSniffer as Conversation } from "./Conversation";
const wss = new ws.WebSocketServer({ port: 3012 });

wss.on('connection', function connection(ws) {
    ws.on('message', function message(data:Buffer) {
        handle(data);
    });
  
    ws.send('something');
});
const serverClientConv = new Conversation();

function handle(data: Buffer) {
    let udp_data = decodePacket(data);

    console.log(udp_data?.payload.toString("hex"));
    if(udp_data){
        serverClientConv.addData(udp_data.payload, udp_data.sourcePort, udp_data.destinationPort);
    }
}

// import ProtobufUtil from "./protobufUtil";
// import {protoRawDecode} from "./test/proto_raw_decoder"
// let test = Buffer.from("0d1c0000001203596f751a024d65202b2a0a0a066162633132331200", "hex");
// const s = protoRawDecode(test)
// console.log(s)
