use crate::random_cs::Random;




pub fn bruteforce(sent_time: u64, server_seed: u64, bytes: &[u8])->Option<u64>{
    println!("Sent time: {}", sent_time);
    println!("Server seed: {}", server_seed);
    let key_prefix = [bytes[0]^0x45, bytes[1]^0x67];

    println!("Key prefix: {:?}", key_prefix);

    let bf_bounds = 1000;
    for i in 0..bf_bounds{
        let offset = if i % 2 == 0 {i / 2}else{-(i as i64 - 1) / 2}; 
        // println!("Trying offset: {}", offset);

        //:vomit:
        let rand_seed :i32 = (sent_time as i64 + offset) as i32;
        let mut rand = Random::with_seed(rand_seed);
        let client_seed = rand.next_safe_uint64();

        let seed = client_seed ^ server_seed;

        //todo: partial key generation? might be faster
        let key = crate::mtkey::MTKey::from_seed(seed);
        if key.keybuf[0] == key_prefix[0] && key.keybuf[1] == key_prefix[1]{
            println!("Found key with seed: {} with offset {}", seed, offset);
            return Some(seed);
        }
    }
    None

}