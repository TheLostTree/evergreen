const copyProtos = ()=>{
    const fs = require('fs')
    const protolist = [
        "PacketHead.proto",
        "GetPlayerTokenReq.proto",
        "GetPlayerTokenRsp.proto",
        "AvatarDataNotify.proto",
        "AvatarFightPropNotify.proto",
        "AvatarFightPropUpdateNotify.proto",
        "CombatInvocationsNotify.proto",
        "PlayerEnterSceneInfoNotify.proto",
        "UnionCmdNotify.proto",
        "AbilityInvocationsNotify.proto",
        "SceneTeamUpdateNotify.proto",
        "SceneEntityDisappearNotify.proto",
        "SceneEntityAppearNotify.proto",
        "PlayerEnterSceneNotify.proto",
    ]

    const includes = []

    const proto_source = `./all_protos`
    //copy dependencies + protolist to ./protos

    while(protolist.length > 0){
        const proto = protolist.shift()
        includes.push(proto)
        const proto_path = `${proto_source}\\${proto}`
        const proto_content = fs.readFileSync(proto_path, 'utf8')
        const proto_imports = proto_content.match(/import\s+"(.*)";/g)
        if(proto_imports){
            proto_imports.forEach(proto_import => {
                const import_proto = proto_import.match(/import\s+"(.*)";/)[1]
                if(!includes.includes(import_proto)){
                    protolist.push(import_proto)
                }
            })
        }
    }
    //copy dependencies + protolist to ./protos
    includes.forEach(proto => {
        fs.writeFileSync("./protos/"+proto, fs.readFileSync(`${proto_source}\\${proto}`, 'utf8'))
    })
}

const generateCmdIdsCsv = ()=>{
    const cmids = require('../packetIds.json');
    const fs = require('fs')
    const csv = Object.entries(cmids).map(([num, name]) => {
        return `${name},${num}`
    }).join('\n')

    fs.writeFileSync('./CmdIds.csv', csv)
}

copyProtos()
generateCmdIdsCsv();