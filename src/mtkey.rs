use std::num::Wrapping;

pub struct Mtkey {
    keybuf: [i32; 4096],
}

pub struct MT19937_64 {
    N: i32,
    M: i32,
    MatrixA: u64,
    mt: [u64; 0x270],
    mti: u32,
}

impl MT19937_64 {
    pub fn default() -> MT19937_64 {
        MT19937_64 {
            N: 0x138,
            M: 0x9C,
            MatrixA: 0xB5026F5AA96619E9,
            mt: [0; 0x270],
            mti: 0x138,
        }
    }

    pub fn seed(&mut self, seed: u64) {
        self.mt[0] = seed & 0xffffffffffffffff;
        for i in 1..312 {
            let value = Wrapping(self.mt[i - 1] ^ (self.mt[i - 1] >> 62));

            self.mt[i] = ((Wrapping(6364136223846793005u64) * value).0
                + (i as u64))
                & 0xffffffffffffffff;
        }
        self.mti = 312;
    }

    pub fn next_ulong(&mut self) -> u64 {
        if self.mti >= 312 {
            if self.mti == 313 {
                self.seed(5489)
            }
            for k in 0..311 {
                let y = (self.mt[k] & 0xffffffff80000000) | (self.mt[k + 1] & 0x7fffffff);
                if k < 312 - 156 {
                    self.mt[k] = self.mt[k + 156]
                        ^ (y >> 1)
                        ^ (if (y & 1) == 0 { 0 } else { 0xb5026f5aa96619e9 });
                } else {
                    self.mt[k] = self.mt[(Wrapping(k + 156+ self.mt.len()) - Wrapping(624)).0]
                        ^ (y >> 1)
                        ^ (if (y & 1) == 0 { 0 } else { 0xb5026f5aa96619e9 });
                }
            }

            let yy = (self.mt[311] & 0xffffffff80000000) | (self.mt[0] & 0x7fffffff);
            self.mt[311] =
                self.mt[155] ^ (yy >> 1) ^ (if yy & 1 == 0 { 0 } else { 0xb5026f5aa96619e9 });
            self.mti = 0;
        }
        let mut x = self.mt[self.mti as usize];
        self.mti += 1;
        x ^= (x >> 29) & 0x5555555555555555;
        x ^= (x << 17) & 0x71d67fffeda60000;
        x ^= (x << 37) & 0xfff7eee000000000;
        x ^= x >> 43;
        x
        // x as i64
    }
}
