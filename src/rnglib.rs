use std::time::Instant;

#[derive(Debug)]
pub struct XorRng64{
    initial_seed: u64,
    state: u64
}
impl XorRng64{
    pub fn new(initial_seed: u64) -> Self{
        XorRng64{
            initial_seed,
            state: initial_seed
        }
    }

    pub fn next(&mut self, range: u64) -> u64{
        let mut new_state = self.state;
        new_state ^= new_state << 13;
        new_state ^= new_state >> 17;
        new_state ^= new_state << 5;

        self.state = new_state;
        return new_state % range
    }
}

// use a different xorshift to conveniently generate rngs
#[derive(Debug)]
pub struct RngGenerator64{
    initial_seed: u64,
    state: u64
}
impl RngGenerator64{

    pub fn new_time_seeded() -> Self{
        let trand = Instant::now();
        loop{
            if trand.elapsed().as_secs_f64() > 0.0{
                break;
            }
        }

        let mut mushed = unsafe{
            let float_ptr = &trand.elapsed().as_secs_f64() as *const f64;
            let int_ptr = float_ptr as *const u64;
            *int_ptr
        };

        return RngGenerator64::new(mushed);
    }

    pub fn new(initial_state: u64) -> Self{
        Self{
            initial_seed: initial_state,
            state: initial_state
        }
    }

    pub fn next(&mut self) -> XorRng64{
        let mut next_state = self.state;
        next_state ^= next_state >> 12;
        next_state ^= next_state << 25;
        next_state ^= next_state >> 27;
        
        self.state = next_state;
        return XorRng64::new(next_state);
    }
}