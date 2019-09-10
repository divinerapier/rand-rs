mod gen_cooked;
mod normal;
mod rng;
mod source;
mod zipf;

pub struct Rand<S>
where
    S: source::Source,
{
    src: S,
    read_val: i64,
    read_pos: i8,
}

impl<S> Rand<S>
where
    S: source::Source,
{
    pub fn new(src: S) -> Rand<S> {
        Rand {
            src,
            read_val: 0,
            read_pos: 0,
        }
    }

    pub fn seed(&mut self, seed: i64) {
        self.src.seed(seed);
    }

    pub fn i64(&mut self) -> i64 {
        self.src.i64()
    }

    pub fn u64(&mut self) -> u64 {
        self.src.u64()
    }

    pub fn i32(&mut self) -> i32 {
        (self.i64() >> 32) as i32
    }

    pub fn u32(&mut self) -> u32 {
        (self.i64() >> 31) as u32
    }

    pub fn i32n(&mut self, n: i32) -> i32 {
        assert!(n > 0);
        if n & (n - 1) == 0 {
            return self.i32() % (n - 1);
        }
        let max = (1 << 31) - 1 - ((1 << 31) % n);
        let mut v = self.i32();
        loop {
            if v <= max {
                break;
            }
            v = self.i32();
        }
        v % n
    }

    pub fn i64n(&mut self, n: i64) -> i64 {
        assert!(n > 0);
        if n & (n - 1) == 0 {
            return self.i64() % (n - 1);
        }
        let max = (1 << 63) - 1 - ((1 << 63) % n);
        let mut v = self.i64();
        loop {
            if v <= max {
                break;
            }
            v = self.i64();
        }
        v % n
    }

    pub fn f32(&mut self) -> f32 {
        loop {
            let f = self.f64() as f32;
            if f != 1f32 {
                return f;
            }
        }
    }

    pub fn f64(&mut self) -> f64 {
        loop {
            let f: f64 = self.i64() as f64 / (1 << 63) as f64;
            if f != 1f64 {
                return f;
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn examples() {
        let mut r: super::Rand<_> = super::Rand::new(super::rng::RngSource::new());
        println!("{}", r.i64());
        println!("{}", r.i64());
        println!("{}", r.i64());
        println!("{}", r.i64());
        println!("{}", r.i64());
    }
}
