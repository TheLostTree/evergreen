import ProtobufUtil from "./protobufUtil";

let test = Buffer.from("0d1c0000001203596f751a024d65202b2a0a0a066162633132331200", "hex");
console.log(ProtobufUtil.unknownDecode(test, {
    protobufDefinition: "",
    showUnknownFields: true,
    showTypes: true
}));
// const WebSocket = require('ws');
import * as ws from "ws";
const wss = new ws.WebSocketServer({ port: 3012 });

// wss.on('connection', function connection(ws) {
//     ws.on('message', function message(data:Buffer) {
//         // console.log(data);
//         handle(data);
//     });
  
//     ws.send('something');
// });

function decodeIPv4Header(data: Buffer) {
    let version = data[0] >> 4;
    let ihl = data[0] & 0x0f;
    let dscp = data[1] >> 2;
    let ecn = data[1] & 0x03;
    let totalLength = data.readUInt16BE(2);
    let identification = data.readUInt16BE(4);
    let flags = data[6] >> 5;
    let fragmentOffset = data.readUInt16BE(6) & 0x1fff;
    let ttl = data[8];
    let protocol = data[9];
    let headerChecksum = data.readUInt16BE(10);
    let sourceAddress = data.readUInt32BE(12);
    let destinationAddress = data.readUInt32BE(16);
    let options = data.slice(20, ihl * 4);
    let payload = data.slice(ihl * 4);
    return {
        version,
        ihl,
        dscp,
        ecn,
        totalLength,
        identification,
        flags,
        fragmentOffset,
        ttl,
        protocol,
        headerChecksum,
        sourceAddress,
        destinationAddress,
        options,
        payload
    }
}

function decodeUDPHeader(data: Buffer) {
    let sourcePort = data.readUInt16BE(0);
    let destinationPort = data.readUInt16BE(2);
    let length = data.readUInt16BE(4);
    let checksum = data.readUInt16BE(6);
    let payload = data.slice(8);
    return {
        sourcePort,
        destinationPort,
        length,
        checksum,
        payload
    }
}

function decodeEthernetHeader(data: Buffer) {
    let destinationAddress = data.slice(0, 6);
    let sourceAddress = data.slice(6, 12);
    let type = data.readUInt16BE(12);
    let payload = data.slice(14);
    return {
        destinationAddress,
        sourceAddress,
        type,
        payload
    }
}


let buf = Buffer.from("45000030519800008011dfe4c0a80b882ffc0d14", "hex")
console.log(buf.toString("hex"))
let header = decodeIPv4Header(buf);
console.log(header);



function handle(data: Buffer) {
    console.log(data.toString("hex"));
    let ethernetHeader = decodeEthernetHeader(data);
    console.log(ethernetHeader);
    let ipv4Header = decodeIPv4Header(ethernetHeader.payload);
    console.log(ipv4Header);
    let udpHeader = decodeUDPHeader(ipv4Header.payload);
    console.log(udpHeader);
}