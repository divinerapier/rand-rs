use crate::rand::Rand;
use crate::source::RngSource;

thread_local!(
    pub static THREAD_RAND: std::cell::RefCell<Rand<RngSource>> =
        std::cell::RefCell::new(Rand::new(RngSource::new(1)));
);

pub struct ThreadLocal;

impl ThreadLocal {
    pub fn seed(seed: i64) {
        THREAD_RAND.with(|x| {
            let x: &std::cell::RefCell<Rand<RngSource>> = x;
            let mut x = x.borrow_mut();
            x.seed(seed);
        })
    }

    pub fn int32() -> i32 {
        THREAD_RAND.with(|x| {
            let x: &std::cell::RefCell<Rand<RngSource>> = x;
            let mut x = x.borrow_mut();
            x.int32()
        })
    }

    pub fn uint32() -> u32 {
        THREAD_RAND.with(|x| {
            let x: &std::cell::RefCell<Rand<RngSource>> = x;
            let mut x = x.borrow_mut();
            x.uint32()
        })
    }

    pub fn int32n(n: i32) -> i32 {
        THREAD_RAND.with(|x| {
            let x: &std::cell::RefCell<Rand<RngSource>> = x;
            let mut x = x.borrow_mut();
            x.int32n(n)
        })
    }

    pub fn int64() -> i64 {
        THREAD_RAND.with(|x| {
            let x: &std::cell::RefCell<Rand<RngSource>> = x;
            let mut x = x.borrow_mut();
            x.int64()
        })
    }

    pub fn uint64() -> u64 {
        THREAD_RAND.with(|x| {
            let x: &std::cell::RefCell<Rand<RngSource>> = x;
            let mut x = x.borrow_mut();
            x.uint64()
        })
    }

    pub fn int64n(n: i64) -> i64 {
        THREAD_RAND.with(|x| {
            let x: &std::cell::RefCell<Rand<RngSource>> = x;
            let mut x = x.borrow_mut();
            x.int64n(n)
        })
    }

    pub fn shuffle<T>(array: &mut Vec<T>) {
        THREAD_RAND.with(|x| {
            let x: &std::cell::RefCell<Rand<RngSource>> = x;
            let mut x = x.borrow_mut();
            x.shuffle(array);
        })
    }
}

#[cfg(test)]
mod test {
    use super::ThreadLocal;
    use std::thread;
    #[test]
    fn example_thread_local() {
        let mut handles = vec![];
        for i in 0..4 {
            let h = thread::spawn(move || {
                for j in 0..3 {
                    println!(
                        "thread local: {}, index: {}, {}",
                        i,
                        j,
                        ThreadLocal::int64()
                    );
                }
            });
            handles.push(h);
        }
        for h in handles {
            h.join().unwrap();
        }
    }
}
