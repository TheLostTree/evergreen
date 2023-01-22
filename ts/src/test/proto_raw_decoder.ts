import { decodeProto, TYPES } from "./protobuf_decoder/protobufDecoder";
import { decodeFixed32, decodeFixed64, decodeVarintParts } from "./protobuf_decoder/protobufPartDecoder";
import { parseInput } from "./protobuf_decoder/hexUtils";
import { decodeVarint, interpretAsSignedType } from "./protobuf_decoder/varintUtils";

//todo: use this function to return objects rather than strings for the values
function parseType(decodeList:  {
    type: string;
    value: number|string|bigint;
}[]){
    console.log(decodeList)
    return decodeList.reduce((pre: any, cur: { type: any; value: any; }, index: number, arr: any) => {
        if (index === 0) {
          return `${cur.type}:${cur.value}`;
        } else {
        } return `${pre}, ${cur.type}:${cur.value}`;
      }, '');
}

function getNextIndex(key: string, repeatedMap: { [x: string]: number; }){
    if(!repeatedMap[key]){
        repeatedMap[key] = 0;
    }
    repeatedMap[key]++;
    return repeatedMap[key] < 2 ? key : `${key}#${repeatedMap[key].toString()}`;
}

function processProtoPart(raw: Buffer){
    const data = decodeProto(raw);
    const result :{[key:string]: any}= {};
    const repeatedMap = {};
    data.parts.forEach(e => {
        let key;
        let res: any;
        switch(e.type){
            case TYPES.FIXED32:
                const fixed32 = decodeFixed32(e.value);
                key = getNextIndex(`${e.index}: 32b`, repeatedMap);
                res = parseType(fixed32);
                break;

            case TYPES.FIXED64:
                const fixed64 = decodeFixed64(e.value);
                key = getNextIndex(`${e.index}:64b`, repeatedMap);
                res = parseType(fixed64);
                break;

            case TYPES.VARINT:
                const varint = decodeVarintParts(e.value);
                key = getNextIndex(`${e.index}: varint`, repeatedMap);
                res = parseType(varint);
                break;

            case TYPES.STRING:
                const str = processProtoPart(e.value);
                key = getNextIndex(`${e.index}: l-delim`, repeatedMap);
 
                if (e.value.length > 0 && !str.leftOver) {
                    res = str;
                } else {
                    res = [];
                    decodeRepeated(e, res);

                    res.push(`String ${e.value.toString()}, Raw: 0x${e.value.toString("hex")}`);
                }

                break;
        }
        result[key as string] = res;
    });

    if(data.leftOver && data.leftOver.length > 0){
        result.leftOver = data.leftOver.toString("base64");
    }

    return result;
}

function decodeRepeated(e: { index?: number; type?: number; value: any; }, res: any[]) {
    try {
        let list = [];
        let len = 0;
        while (len < e.value.length) {
            const reslove = decodeVarint(e.value, len);
            len += reslove.length;
            list.push(reslove.value);
        }

        if (list.length > 0) {
            res.push(`Repeated Int: [${list.toString()}]`);

            if (list[0] !== interpretAsSignedType(list[0])) {
                const newList = list.map(i => interpretAsSignedType(i));
                res.push(`Repeated Signed Int: [${newList.toString()}]`);
            }

        }
    } catch (ex) {
        // it must be not this type
    }
}

function protoRawDecode(raw: Buffer){
    return processProtoPart(raw);
}

export { protoRawDecode };