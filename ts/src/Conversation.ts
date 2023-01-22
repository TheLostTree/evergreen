import node_kcp from 'node-kcp-token';

export class UdpSniffer {
    server?: node_kcp.KCP;
    client?: node_kcp.KCP;

    dispatchKey?: Buffer;
    sessionKey?: Buffer;

    constructor(){}
    public addData(data: Buffer, sourcePort: number, destinationPort: number) {
        if(data.length == 20){
            //handshake
            let magic = data.readUInt32BE(0);
            let conv = data.readUInt32BE(4);
            let token = data.readUInt32BE(8);
            switch (magic) {
                case 0x00000145:
                    this.server = new node_kcp.KCP(conv, token, {address: "", port: sourcePort, family: "IPv4"});
                    this.client = new node_kcp.KCP(conv, token, {address: "", port: destinationPort, family: "IPv4"});
                    this.initKcp(this.client);
                    this.initKcp(this.server);
                    break;
                case 0x00000194:
                    // disconnect
                    break;
                default:
                    break;
            } 
            return;   
        }
        if(this.server && this.client){
            if(sourcePort == this.server.context().port){
                this.server.input(data);
            }else if(sourcePort == this.client.context().port){
                this.client.input(data);
            }else{
                console.log("unknown port %s", sourcePort);
            }
            this.recvAll();
        }

    }
    private recv(kcp: node_kcp.KCP){
        let buffer = kcp.recv();
        if (!buffer) return false;
        this.handleIncomingPacket(buffer);
        return true;

    }

    private recvAll(){
        while(this.recv(this.server!) || this.recv(this.client!)){
            //loop
        };
    }
    private initKcp(kcp?: node_kcp.KCP){
        //default callbacks
        kcp?.output((buffer: Buffer, size: number) => {
        })
        kcp?.nodelay(1, 10, 2, 0);
        kcp?.wndsize(128, 128);
    }
    handleIncomingPacket(buffer: Buffer) {
        console.log(buffer.toString("hex"));
        // xor
        
    }
    
}


