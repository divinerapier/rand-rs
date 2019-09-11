use crate::rand::Rand;
use crate::source::RngSource;

thread_local!(
    pub static THREAD_RAND: std::cell::RefCell<Rand<RngSource>> =
        std::cell::RefCell::new(Rand::new(RngSource::new(1)));
);

pub fn seed(seed: i64) {
    THREAD_RAND.with(|x| {
        let x: &std::cell::RefCell<Rand<RngSource>> = x;
        let mut x = x.borrow_mut();
        x.seed(seed);
    })
}

pub fn i32() -> i32 {
    THREAD_RAND.with(|x| {
        let x: &std::cell::RefCell<Rand<RngSource>> = x;
        let mut x = x.borrow_mut();
        x.i32()
    })
}

pub fn u32() -> u32 {
    THREAD_RAND.with(|x| {
        let x: &std::cell::RefCell<Rand<RngSource>> = x;
        let mut x = x.borrow_mut();
        x.u32()
    })
}

pub fn i32n(n: i32) -> i32 {
    THREAD_RAND.with(|x| {
        let x: &std::cell::RefCell<Rand<RngSource>> = x;
        let mut x = x.borrow_mut();
        x.i32n(n)
    })
}

pub fn i64() -> i64 {
    THREAD_RAND.with(|x| {
        let x: &std::cell::RefCell<Rand<RngSource>> = x;
        let mut x = x.borrow_mut();
        x.i64()
    })
}

pub fn u64() -> u64 {
    THREAD_RAND.with(|x| {
        let x: &std::cell::RefCell<Rand<RngSource>> = x;
        let mut x = x.borrow_mut();
        x.u64()
    })
}

pub fn i64n(n: i64) -> i64 {
    THREAD_RAND.with(|x| {
        let x: &std::cell::RefCell<Rand<RngSource>> = x;
        let mut x = x.borrow_mut();
        x.i64n(n)
    })
}

pub fn shuffle<T>(array: &mut Vec<T>) {
    let mut i = array.len() - 1;
    while i > 1 << 31 - 2 {
        let j = i64n((i + 1) as i64);
        array.swap(i as usize, j as usize);
        i -= 1;
    }
    while i > 0 {
        let j = i32n((i + 1) as i32);
        array.swap(i as usize, j as usize);
        i -= 1;
    }
}

#[cfg(test)]
mod test {
    use super::i64;
    use std::thread;
    #[test]
    fn example_thread_local() {
        let mut handles = vec![];
        for i in 0..4 {
            let h = thread::spawn(move || {
                for j in 0..3 {
                    println!("thread local: {}, index: {}, {}", i, j, i64());
                }
            });
            handles.push(h);
        }
        for h in handles {
            h.join().unwrap();
        }
    }
    #[test]
    fn shuffle() {
        let mut v: Vec<i32> = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];
        super::shuffle(&mut v);
        println!("shuffle: {:?}", v);
    }
}
