
import MT19937_64 from './MT64';

export default class MTKey {
    keybytes!: Buffer;

    static mtgen: MT19937_64 = new MT19937_64();

    constructor(bytes: Buffer) {
        this.keybytes = bytes;
    }

    static fromSeed(seed: bigint) {
        this.mtgen.seed(seed);

        let newseed = this.mtgen.int64();
        this.mtgen.seed(newseed);
        this.mtgen.int64();
        let key = Buffer.alloc(4096);
        for (let i = 0; i < 4096; i += 8) {
            let val = this.mtgen.int64();
            key.writeBigUInt64BE(val, i);
        }
        return new MTKey(key);
    }

    static getFirstBytes(seed: bigint) {
        this.mtgen.seed(seed);

        let newseed = this.mtgen.int64();
        this.mtgen.seed(newseed);
        this.mtgen.int64();
        let key = Buffer.alloc(8);
        let val = this.mtgen.int64();
        key.writeBigUInt64BE(val, 0);
        return key;
    }

    XOR(bytes: Buffer) {
        let result = Buffer.alloc(bytes.length);
        for (let i = 0; i < bytes.length; i++) {
            result[i] = this.keybytes[i % 4096] ^ bytes[i];
        }
        console.assert(result.length == bytes.length);
        return result;
    }
}