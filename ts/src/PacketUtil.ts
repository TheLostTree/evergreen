class IPv4Packet{
    version!: number;
    ihl!: number;
    dscp!: number;
    ecn!: number;
    totalLength!: number;
    identification!: number;
    flags!: number;
    fragmentOffset!: number;
    ttl!: number;
    protocol!: number;
    headerChecksum!: number;
    sourceAddress!: number;
    destinationAddress!: number;
    options!: Buffer;
    payload!: Buffer;
}

class UDPPacket{
    sourcePort!: number;
    destinationPort!: number;
    length!: number;
    checksum!: number;
    payload!: Buffer;
}

class EthernetPacket{
    destinationAddress!: Buffer;
    sourceAddress!: Buffer;
    tag: Buffer | null = null;
    type!: number;
    payload!: Buffer;
}

class TcpPacket{
    sourcePort!: number;
    destinationPort!: number;
    sequenceNumber!: number;
    acknowledgementNumber!: number;
    dataOffset!: number;
    reserved!: number;
    ns!: number;
    cwr!: number;
    ece!: number;
    urg!: number;
    ack!: number;
    psh!: number;
    rst!: number;
    syn!: number;
    fin!: number;
    window!: number;
    checksum!: number;
    urgentPointer!: number;
    options!: Buffer;
    payload!: Buffer;
}

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
    let packet = new IPv4Packet();
    packet.version = version;
    packet.ihl = ihl;
    packet.dscp = dscp;
    packet.ecn = ecn;
    packet.totalLength = totalLength;
    packet.identification = identification;
    packet.flags = flags;
    packet.fragmentOffset = fragmentOffset;
    packet.ttl = ttl;
    packet.protocol = protocol;
    packet.headerChecksum = headerChecksum;
    packet.sourceAddress = sourceAddress;
    packet.destinationAddress = destinationAddress;
    packet.options = options;
    packet.payload = payload;
    return packet;
}

function decodeUDPHeader(data: Buffer) {
    let sourcePort = data.readUInt16BE(0);
    let destinationPort = data.readUInt16BE(2);
    let length = data.readUInt16BE(4);
    let checksum = data.readUInt16BE(6);
    let payload = data.slice(8);
    let packet = new UDPPacket();
    packet.sourcePort = sourcePort;
    packet.destinationPort = destinationPort;
    packet.length = length;
    packet.checksum = checksum;
    packet.payload = payload;
    return packet;
}
// 02 00 00 00 45 00 
// 00 34 00 00 40 00
// 40 06 45 e0 
// 0a 72 
// c1e8a29f85eae40501
function decodeEthernetHeader(data: Buffer) {
    let destinationAddress = data.slice(0, 6);
    let sourceAddress = data.slice(6, 12);
    let tag: Buffer | null = null;
    let type = data.readUInt16BE(12);
    if (type == 0x8100) {
        tag = data.slice(12, 18);
        type = data.readUInt16BE(18);
    }
    let payload = data.slice(14);
    let packet = new EthernetPacket();
    packet.destinationAddress = destinationAddress;
    packet.sourceAddress = sourceAddress;
    packet.tag = tag;
    packet.type = type;
    packet.payload = payload;
    return packet;
}

function decodeTcpHeader(data: Buffer){
    let sourcePort = data.readUInt16BE(0);
    let destinationPort = data.readUInt16BE(2);
    let sequenceNumber = data.readUInt32BE(4);
    let acknowledgementNumber = data.readUInt32BE(8);
    let dataOffset = data[12] >> 4;
    let reserved = data[12] & 0x0f;
    let ns = data[13] >> 7;
    let cwr = (data[13] >> 6) & 0x01;
    let ece = (data[13] >> 5) & 0x01;
    let urg = (data[13] >> 4) & 0x01;
    let ack = (data[13] >> 3) & 0x01;
    let psh = (data[13] >> 2) & 0x01;
    let rst = (data[13] >> 1) & 0x01;
    let syn = data[13] & 0x01;
    let fin = data[14] >> 7;
    let window = data.readUInt16BE(14);
    let checksum = data.readUInt16BE(16);
    let urgentPointer = data.readUInt16BE(18);
    let options = data.slice(20, dataOffset * 4);
    let payload = data.slice(dataOffset * 4);
    let packet = new TcpPacket();
    packet.sourcePort = sourcePort;
    packet.destinationPort = destinationPort;
    packet.sequenceNumber = sequenceNumber;
    packet.acknowledgementNumber = acknowledgementNumber;
    packet.dataOffset = dataOffset;
    packet.reserved = reserved
    packet.ns = ns;
    packet.cwr = cwr;
    packet.ece = ece;
    packet.urg = urg;
    packet.ack = ack;
    packet.psh = psh;
    packet.rst = rst;
    packet.syn = syn;
    packet.fin = fin;
    packet.window = window;
    packet.checksum = checksum;
    packet.urgentPointer = urgentPointer;
    packet.options = options;
    packet.payload = payload;
    return packet;
}

class NetworkPacket {
    sourceAddress!: number;
    destinationAddress!: number;
    sourcePort!: number;
    destinationPort!: number;
    payload!: Buffer;
    type: number = 0;
}

function decodePacket(data: Buffer){
    //figure out what type of packet it is
    let ethHeader = decodeEthernetHeader(data);
    if(ethHeader.type == 0x0800){
        let ipHeader = decodeIPv4Header(ethHeader.payload);
        if(ipHeader.protocol == 0x11){
            let udpHeader = decodeUDPHeader(ipHeader.payload);
            let packet = new NetworkPacket();
            packet.sourcePort = udpHeader.sourcePort;
            packet.destinationPort = udpHeader.destinationPort;

            packet.destinationAddress = ipHeader.destinationAddress;
            packet.sourceAddress = ipHeader.sourceAddress;

            packet.payload = udpHeader.payload;
            packet.type = 0x11;
            return packet;
        }
        if(ipHeader.protocol == 0x06){
            //tcp
            let tcpHeader = decodeTcpHeader(ipHeader.payload);
            let packet = new NetworkPacket();
            packet.sourcePort = tcpHeader.sourcePort;
            packet.destinationPort = tcpHeader.destinationPort;

            packet.destinationAddress = ipHeader.destinationAddress;
            packet.sourceAddress = ipHeader.sourceAddress;
            packet.payload = tcpHeader.payload;
            packet.type = 0x06;
            return packet;
        }
        console.log("Unknown protocol: " + ipHeader.protocol);
        return;
    }
    if(ethHeader.type == 0x86DD){
        //ipv6
        return;
    }
    // console.log("Unknown type: " + ethHeader.type);
    // console.log("0x"+data.toString('hex'));

}

export {
    decodePacket,
    decodeEthernetHeader,
    decodeUDPHeader,
    decodeIPv4Header
}