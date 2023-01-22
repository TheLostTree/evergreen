/*
  Source: https://github.com/pawitp/protobuf-decoder by pawitp
  Modified to adapt common-js
 */
// const JSBI = require("jsbi");


function interpretAsSignedType(n:bigint) {
  // see https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/wire_format_lite.h#L857-L876
  // however, this is a simpler equivalent formula
  // const isEven = JSBI.equal(JSBI.bitwiseAnd(n, JSBI.BigInt(1)), JSBI.BigInt(0));
  const isEven = n % 2n === 0n;
  if (isEven) {
    return n/2n
  } else {

    return -1n * (n + 1n) / 2n;
    // return JSBI.multiply(
    //   JSBI.BigInt(-1),
    //   JSBI.divide(JSBI.add(n, BIGINT_1), BIGINT_2)
    // );
  }
}
type ret = { value: bigint, length: number }

function decodeVarint(buffer: Buffer, offset:number) : ret
{
  let res = 0n
  let shift = 0;
  let byte = 0;

  do {
    if (offset >= buffer.length) {
      throw new RangeError("Index out of bound decoding varint");
    }

    byte = buffer[offset++];

    // const multiplier = JSBI.exponentiate(BIGINT_2, JSBI.BigInt(shift));
    const multiplier = 2n ** BigInt(shift);

    // const thisByteValue = JSBI.multiply(JSBI.BigInt(byte & 0x7f), multiplier);
    const thisByteValue = BigInt(byte & 0x7f) * multiplier;
    shift += 7;
    // res = JSBI.add(res, thisByteValue);
    res += thisByteValue;

  } while (byte >= 0x80);

  return {
    value: res,
    length: shift / 7
  };
}

export { decodeVarint, interpretAsSignedType };
