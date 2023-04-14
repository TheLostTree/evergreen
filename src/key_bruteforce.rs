use std::fs;


pub struct KeyBruteforce{
    previous_seeds: Vec<i64>
}

impl KeyBruteforce{
    fn guess(&self, test_buf: &[u8], ts:i64, server_seed: u64, depth:i32)->Option<u64>{
        let key_prefix = [test_buf[0]^0x45, test_buf[1]^0x67];
        let key_suffix = [test_buf[test_buf.len()-2]^0x89, test_buf[test_buf.len()-1]^0xAB];
    
        //ts = sent_time as i64 + offset
        let rand_seed :i32 = (ts) as i32;
        let mut rand = Random::with_seed(rand_seed);
    
        for _ in 0..depth{
            let client_seed = rand.next_safe_uint64();
            let seed = client_seed ^ server_seed;
            let key = crate::mtkey::MTKey::from_seed(seed);
    
            let is_valid_prefix = key.keybuf[0] == key_prefix[0] && key.keybuf[1] == key_prefix[1];
            let is_valid_suffix = key.keybuf[(test_buf.len()-2) % key.keybuf.len()] == key_suffix[0] && key.keybuf[(test_buf.len()-1) % key.keybuf.len()] == key_suffix[1];
            //
    
            if is_valid_prefix && is_valid_suffix{
                // found it!
                return Some(seed)
            }
    
        }
    
        None
    
    
    } 
    //todo: do the guess/ prevkey thing to prevent this from only working once per game launch
    pub fn bruteforce(&mut self, sent_time: u64, server_seed: u64, bytes: &[u8])->Option<u64>{
       
        for oldseed in self.previous_seeds.iter(){
            if let Some(key) = self.guess(bytes, *oldseed, server_seed, 1000){
                println!("found from old seeds!");
                return Some(key)
            }
        }
    
        for i in 0..3000{
            let offset = if i % 2 == 0 {i / 2}else{-(i as i64 - 1) / 2}; 
            let ts = sent_time as i64 + offset;
    
            if let Some(key) = self.guess(bytes, ts, server_seed, 1000){
                self.previous_seeds.push(ts); //save static random seed
                return Some(key)
            }
        }
        // sad!
        println!("unfortunate...");
    
        None
    }

    pub fn new()->KeyBruteforce{
        //load previous seeds from file
        let contents = fs::read_to_string("./prev_seeds.txt").unwrap_or("".to_string());
        let mut prev_seeds = Vec::new();
        for line in contents.lines(){
            prev_seeds.push(line.parse::<i64>().unwrap());
        }

        if prev_seeds.len() > 0{
            println!("loaded {} previous seeds", prev_seeds.len());
        }
        


        KeyBruteforce{
            previous_seeds: Vec::new()
        }
    }


    pub fn save(&self){
        let mut contents = String::new();
        //o nly save the last 1000
        let n = if self.previous_seeds.len() > 1000{self.previous_seeds.len() - 1000}else{0};
        
        for seed in self.previous_seeds.iter().skip(n){
            contents.push_str(&seed.to_string());
            contents.push_str("\n");
        }
        fs::write("./prev_seeds.txt", contents).unwrap();
    }
}

pub struct Random {
    //
    // Private Constants
    //
    m_big: i32,
    m_seed: i32,
    inext: i32,
    inextp: i32,
    seed_array: [i32; 56],
}

impl Random {
    // public Random()
    //   : this(Environment.TickCount) {
    // }
    
    fn default() -> Random {
        Random {
            m_big: i32::MAX,
            m_seed: 161803398,
            inext: 0,
            inextp: 0,
            seed_array: [0; 56],
        }
    }

    pub fn new()->Random{
        //shitty af but oh well
        Random::with_seed(std::time::Instant::now().elapsed().as_millis() as i32)
    }

    pub fn with_seed(seed: i32) -> Random {
        let mut ii ;
        let mut rand = Random::default();

        let subtraction = if seed == i32::MIN {
            i32::MAX
        } else {
            i32::abs(seed)
        };
        let mut mj = rand.m_seed - subtraction;
        rand.seed_array[55] = mj;

        let mut mk = 1;

        for i in 1..55 {
            ii = 21 * i % 55;
            rand.seed_array[ii] = mk;
            mk = mj - mk;
            if mk < 0 {
                mk += rand.m_big
            }
            mj = rand.seed_array[ii]
        }

        for _ in 1..5 {
            for i in 1..56 {
                rand.seed_array[i] = rand.seed_array[i].wrapping_sub(rand.seed_array[1 + (i + 30) % 55]);
                if rand.seed_array[i] < 0 {
                    rand.seed_array[i] += rand.m_big
                };
            }
        }

        rand.inext = 0;
        rand.inextp = 21;
        // seed = 1; lol why?

        rand
    }

    pub fn next_double(&mut self)->f64{
        (self.internal_sample() as f64)*(1.0/(self.m_big as f64))
    }

    fn internal_sample(&mut self)->i32{
        let mut ret_val: i32;
        let mut loc_inext = self.inext;
        let mut loc_inextp = self.inextp;

        if (loc_inext += 1, loc_inext).1 >= 56{
            loc_inext = 1;
        }
        if (loc_inextp += 1, loc_inextp).1 >= 56{
            loc_inextp = 1;
        }

        ret_val = self.seed_array[loc_inext as usize] - self.seed_array[loc_inextp as usize];
        if ret_val == self.m_big {ret_val -= 1};
        if ret_val < 0 {ret_val += self.m_big};

        self.inext = loc_inext;
        self.inextp = loc_inextp;

        ret_val
    }
    
    pub fn next_safe_uint64(&mut self) -> u64{
        (self.next_double() * (u64::MAX as f64)) as u64
    }
}
