/*
  Source: https://github.com/pawitp/protobuf-decoder by pawitp
  Modified to adapt common-js
 */
// import JSBI from "jsbi";
import { bufferLeToBeHex } from "./hexUtils";
import { interpretAsSignedType } from "./varintUtils";

function decodeFixed32(value: Buffer) {
  const floatValue = value.readFloatLE(0);
  const intValue = value.readInt32LE(0);
  const uintValue = value.readUInt32LE(0);

  const result = [];

  result.push({ type: "Int", value: intValue });

  if (intValue !== uintValue) {
    result.push({ type: "Unsigned Int", value: uintValue });
  }

  result.push({ type: "Float", value: floatValue });

  return result;
}

function decodeFixed64(value: Buffer) {
  const floatValue = value.readDoubleLE(0);
  // const uintValue = JSBI.BigInt("0x" + bufferLeToBeHex(value));
  const uintValue = value.readBigUInt64LE(0);
  const intValue = twoComplements(uintValue);

  const result = [];

  result.push({ type: "Int", value: intValue.toString() });

  if (intValue !== uintValue) {
    result.push({ type: "Unsigned Int", value: uintValue.toString() });
  }

  result.push({ type: "Double", value: floatValue });

  return result;
}

function decodeVarintParts(value: string | number | bigint | boolean) {
  const result = [];
  const intVal = BigInt(value);
  result.push({ type: "Int", value: intVal.toString() });

  const signedIntVal = interpretAsSignedType(intVal);
  if (signedIntVal !== intVal) {
    result.push({ type: "Signed Int", value: signedIntVal.toString() });
  }
  return result;
}

const maxLong = 0x7fffffffffffffffn
const longForComplement = 0x10000000000000000n

function twoComplements(uintValue: any) {
  if (uintValue > maxLong) {
    return uintValue-longForComplement
  } else {
    return uintValue;
  }
}

export { decodeFixed32, decodeFixed64, decodeVarintParts };