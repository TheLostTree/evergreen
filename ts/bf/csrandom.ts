export class WasmMult {
    static wasm_base64 = 'AGFzbQEAAAABBwFgAnx+AX4DAgEABxcBE2llZWU3NTJfdWludDY0X211bHQAAAoLAQkAIAG6IACisQs=';
    static wasm_buffer = Buffer.from(this.wasm_base64, 'base64');
    static wasm: WebAssembly.Module;
    static wasm_instance: WebAssembly.Instance;

    constructor() {}

    static async init() {
        WasmMult.wasm = await WebAssembly.compile(WasmMult.wasm_buffer);
        WasmMult.wasm_instance = new WebAssembly.Instance(WasmMult.wasm);
    }

    public static mult(a: number, b: BigInt) {
        let bign_s = (WasmMult.wasm_instance.exports.ieee752_uint64_mult as CallableFunction)(a, b);
        // result in BigInt:
        return BigInt.asUintN(64, bign_s);
    }
}

export default class Random {
    private readonly MBIG = 2147483647;
    private readonly MSEED = 161803398;
    private readonly MZ = 0;

    private inext: number;
    private inextp: number;
    private SeedArray: Array<number> = new Array();

    // process.uptime is not accurate to the Environment.TickCount that the c# one uses
    // so provide a seed otherwise.
    constructor(Seed: number = process.uptime()) {
        {
            let ii: number;
            let mj: number, mk: number;

            //Initialize our Seed array.
            //This algorithm comes from Numerical Recipes in C (2nd Ed.)
            let subtraction: number = Seed == -2147483648 ? 2147483647 : Math.abs(Seed);
            mj = this.MSEED - subtraction;
            this.SeedArray[55] = mj;
            mk = 1;
            for (let i = 1; i < 55; i++) {
                //Apparently the range [1..55] is special (Knuth) and so we're wasting the 0'th position.
                ii = (21 * i) % 55;
                this.SeedArray[ii] = mk;
                mk = mj - mk;
                if (mk < 0) mk += this.MBIG;
                mj = this.SeedArray[ii];
            }
            for (let k = 1; k < 5; k++) {
                for (let i = 1; i < 56; i++) {
                    this.SeedArray[i] -= this.SeedArray[1 + ((i + 30) % 55)];
                    if (this.SeedArray[i] < 0) this.SeedArray[i] += this.MBIG;
                }
            }
            this.inext = 0;
            this.inextp = 21;
            Seed = 1;
        }
    }

    public InternalSample() {
        let retVal: number;
        let locINext: number = this.inext;
        let locINextp: number = this.inextp;

        if (++locINext >= 56) locINext = 1;
        if (++locINextp >= 56) locINextp = 1;

        retVal = this.SeedArray[locINext] - this.SeedArray[locINextp];

        if (retVal == this.MBIG) retVal--;
        if (retVal < 0) retVal += this.MBIG;

        this.SeedArray[locINext] = retVal;

        this.inext = locINext;
        this.inextp = locINextp;

        return retVal;
    }

    /*====================================Sample====================================
    **Action: Return a new random number [0..1) and reSeed the Seed array.
    **Returns: A double [0..1)
    **Arguments: None
    **Exceptions: None
    ==============================================================================*/
    protected Sample() {
        //Including this division at the end gives us significantly improved
        //random number distribution.
        return this.InternalSample() * (1.0 / this.MBIG);
    }

    /*=====================================Next=====================================
    **Returns: An int [0..Int32.MaxValue)
    **Arguments: None
    **Exceptions: None.
    ==============================================================================*/
    public Next() {
        return this.InternalSample();
    }

    // double in c# == number in js
    public NextDouble() {
        return this.Sample();
    }

    public NextSafeUint64() {
        //this bigint is just ulong.maxvalue
        return WasmMult.mult(this.NextDouble(), BigInt('18446744073709551615'));
    }

    public NextBytes(buffer: Buffer) {
        for (let i = 0; i < buffer.length; i++) {
            buffer[i] = this.InternalSample() % 0xff;
        }
    }
}