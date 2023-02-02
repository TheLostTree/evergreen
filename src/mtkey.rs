use std::{num::Wrapping, collections::HashMap};
use bytes::BufMut;

const MHYKEYS : &str = include_str!("../dispatch_keys.bin");


pub struct MTKey{
    pub keybuf: Vec<u8>,
}

impl MTKey{
    pub fn from_seed(seed: u64)->MTKey{
        let mut mt = MT19937_64::default();
        mt.seed(seed);
        let newseed = mt.next_ulong();
        mt.seed(newseed);
        _ = mt.next_ulong(); //discard
        let mut keybuf = vec![];
        for _ in (0..4096).step_by(8){
            //write to keybuf as big endian
            let val = mt.next_ulong();
            keybuf.put_u64(val);
        }
        //:(
        MTKey { keybuf: keybuf }
    }
    pub fn xor(&self, data: &mut Vec<u8>){
        for i in 0..data.len(){
            data[i] ^= self.keybuf[i % self.keybuf.len()];
        }
    }
}

pub struct MT19937_64 {
    mt: [u64; 312],
    mti: u32,
}

impl MT19937_64 {
    pub fn default() -> MT19937_64 {
        MT19937_64 {
            // these are used in c# for some reason but not here? 
            // N: 0x138, // 312
            // M: 0x9C, // 156
            // matrix_a: 0xB5026F5AA96619E9, //13043109905998158313
            mt: [0; 312],
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
                if k < (312 - 156) {
                    self.mt[k] = self.mt[k + 156]
                        ^ (y >> 1)
                        ^ (if (y & 1) == 0 { 0 } else { 0xb5026f5aa96619e9 });
                } else {
                    self.mt[k] = self.mt[(Wrapping(k + 156 + self.mt.len()) - Wrapping(624)).0]
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
    }
}


//i'm at most calling this once per run so i'm not going to bother saving it anywhere
pub fn get_dispatch_keys()->HashMap<u16, [u8;4096]>{
    // let f = std::fs::read_to_string("./MHYKeys.bin").unwrap();
    let mut x = HashMap::new();
    for line in MHYKEYS.split("\n"){
        let parts = line.split(": ").collect::<Vec<&str>>();
        //todo: check if its actually a i32 or a u32?
        let firstbytes = parts[0].parse::<u16>();
        // parse hex
        let mut keybytes = [0u8; 4096];

        let mut index = 0;
        for b in parts[1].chars().step_by(2){
            let byte = u8::from_str_radix(&b.to_string(), 16).unwrap();
            keybytes[index] = byte;
            index += 1;
        }
        x.insert(firstbytes.unwrap(), keybytes);
    }
    x
}