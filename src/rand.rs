pub use crate::source::Source;

unsafe impl<S: Source> Send for Rand<S> where S: Send {}
unsafe impl<S: Source> Sync for Rand<S> where S: Sync {}

pub struct Rand<S>
where
    S: Source,
{
    src: S,
}

impl<S: Sized + Source> Clone for Rand<S>
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
    S: Source,
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
    pub use crate::source::{LockedSource, RngSource};
    #[test]
    fn examples() {
        let mut r: super::Rand<_> = super::Rand::new(RngSource::new(1));
        for _i in 0..12 {
            println!("{}", r.i64());
        }
    }
    #[test]
    fn examples_global_thread_safe() {
        let r: super::Rand<_> = super::Rand::new(LockedSource::new(1));
        let mut handles = vec![];
        for i in 0..4 {
            let mut r = r.clone();
            let h = std::thread::spawn(move || {
                for j in 0..3 {
                    println!("thread: {}, index: {}, {}", i, j, r.i64());
                }
            });
            handles.push(h);
        }
        for h in handles {
            h.join().unwrap();
        }
    }
}
