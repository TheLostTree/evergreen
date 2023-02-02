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
    pub fn default() -> Random {
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
        let mut ii = 0;
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

        for k in 1..5 {
            for i in 1..56 {
                rand.seed_array[i] -= rand.seed_array[1 + (i + 30) % 55];
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
