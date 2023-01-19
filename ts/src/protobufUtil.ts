// @ts-ignore
import protobuf from "./protobuf.js"


export default class ProtobufUtil {
    static unknownDecode(buf_data: Buffer, settings = {
        protobufDefinition: "",
        showUnknownFields: true,
        showTypes: true
    }) {
        return protobuf.decode(buf_data, [settings.protobufDefinition, settings.showUnknownFields, settings.showTypes])
    }
}

let getWireType = (type: string) => {
    switch (type) {
        case "uint32":
        case "int32":
        case "bool":
        case "uint64":
        case "int64":
            return "VarInt"
        case "fixed64":
        case "double":
            return "64-Bit"
        case "float":
        case "fixed32":
            return "32-Bit"
        case "string":
        default:
            //messages
            return "L-delim"
    }
}