mod gen_cooked;
mod normal;
mod rng;
mod source;
mod zipf;

unsafe impl<S: source::Source> Send for Rand<S> where S: Send {}
unsafe impl<S: source::Source> Sync for Rand<S> where S: Sync {}

pub struct Rand<S>
where
    S: source::Source,
{
    src: S,
}

impl<S: Sized + source::Source> Clone for Rand<S>
where
    S: Clone,
{
    fn clone(&self) -> Self {
        Rand {
            src: self.src.clone(),
        }
    }
}

impl<S> Rand<S>
where
    S: source::Source,
{
    pub fn new(src: S) -> Rand<S> {
        Rand { src }
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
        let mut r: super::Rand<_> = super::Rand::new(super::rng::RngSource::new(1));
        println!("{}", r.i64());
        println!("{}", r.i64());
        println!("{}", r.i64());
        println!("{}", r.i64());
        println!("{}", r.i64());
    }
    #[test]
    fn examples2() {
        let r: super::Rand<_> = super::Rand::new(super::rng::LockedSource::new(1));
        let mut handles = vec![];
        for _i in 0..4 {
            let mut r = r.clone();
            let h = std::thread::spawn(move || {
                println!("{}", r.i64());
            });
            handles.push(h);
        }
        for h in handles {
            h.join().unwrap();
        }
    }
}
