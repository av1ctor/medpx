use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Default)]
pub struct Xoshiro256ss {
    state: [u64; 4],
} 
    
impl Xoshiro256ss {    
    pub fn new (
        seed: u64
    ) -> Self {
        let tmp1 = Self::splitmix64(seed);
        let tmp2 = Self::splitmix64(tmp1.1);
        let tmp3 = Self::splitmix64(tmp2.1);
        let tmp4 = Self::splitmix64(tmp3.1);
        
        Self {
            state: [tmp1.0, tmp2.0, tmp3.0, tmp4.0]
        }
    }
    
    pub fn next(
        &mut self
    ) -> u64 {
        let result = self.state[0] + self.state[3];
        let t = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;
        self.state[3] = (self.state[3] >> 45) | (self.state[3] << (64 - 45));

        result
    }

    fn splitmix64(
        state: u64
    ) -> (u64, u64) {
        let s = state + 0x9E3779B97f4A7C15;
        let mut result = s;
        result = (result ^ (result >> 30)) * 0xBF58476D1CE4E5B9;
        result = (result ^ (result >> 27)) * 0x94D049BB133111EB;
        (result ^ (result >> 31), s)
    }
}

