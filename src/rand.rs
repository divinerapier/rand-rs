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
    /// Creates a new Rand that uses random values from src to
    /// generate other random values.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use srand::Rand;
    /// use srand::RngSource;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let mut r: Rand<_> = Rand::new(RngSource::new(1));
    ///     Ok(())
    /// }
    /// ```
    pub fn new(src: S) -> Rand<S> {
        Rand { src }
    }

    /// Seed uses the provided seed value to initialize the generator to a
    /// deterministic state.
    pub fn seed(&mut self, seed: i64) {
        self.src.seed(seed);
    }

    /// Returns a non-negative pseudo-random 63-bit integer as an i64.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use srand::{Rand,RngSource};
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let mut r: Rand<_> = Rand::new(RngSource::new(1));
    ///     println!("n: {}", r.int64());
    ///     Ok(())
    /// }
    /// ```
    pub fn int64(&mut self) -> i64 {
        self.src.int64()
    }

    /// Returns a pseudo-random 64-bit value as a u64.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use srand::{Rand, RngSource};
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let mut r: Rand<_> = Rand::new(RngSource::new(1));
    ///     println!("n: {}", r.uint64());
    ///     Ok(())
    /// }
    /// ```
    pub fn uint64(&mut self) -> u64 {
        self.src.uint64()
    }

    /// i32 returns a non-negative pseudo-random 31-bit integer as an i32.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use srand::{Rand, RngSource};
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let mut r: Rand<_> = Rand::new(RngSource::new(1));
    ///     println!("n: {}", r.int32());
    ///     Ok(())
    /// }
    /// ```
    pub fn int32(&mut self) -> i32 {
        (self.int64() >> 32) as i32
    }

    /// Returns a pseudo-random 32-bit value as a u32.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use srand::{Rand, RngSource};
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let mut r: Rand<_> = Rand::new(RngSource::new(1));
    ///     println!("n: {}", r.uint32());
    ///     Ok(())
    /// }
    /// ```
    pub fn uint32(&mut self) -> u32 {
        (self.int64() >> 31) as u32
    }

    /// Returns an i32, a non-negative pseudo-random number in [0,n).
    /// It panics if n <= 0.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use srand::{Rand, RngSource};
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let mut r: Rand<_> = Rand::new(RngSource::new(1));
    ///     println!("n: {}", r.int32n(90000001));
    ///     Ok(())
    /// }
    /// ```
    pub fn int32n(&mut self, n: i32) -> i32 {
        assert!(n > 0);
        if n & (n - 1) == 0 {
            return self.int32() & (n - 1);
        }
        let max: i64 = (1 << 31) - 1 - ((1 << 31) % n as i64);
        let mut v = self.int32();
        loop {
            if v as i64 <= max {
                break;
            }
            v = self.int32();
        }
        v % n
    }

    /// Returns an i64, a non-negative pseudo-random number in [0,n).
    /// It panics if n <= 0.
    /// # Examples
    ///
    /// ```no_run
    /// use srand::{Rand, RngSource};
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let mut r: Rand<_> = Rand::new(RngSource::new(1));
    ///     println!("n: {}", r.int64n(90000001));
    ///     Ok(())
    /// }
    /// ```
    pub fn int64n(&mut self, n: i64) -> i64 {
        assert!(n > 0);
        if n & (n - 1) == 0 {
            return self.int64() & (n - 1);
        }
        let max: u64 = 1 << 63;
        let max: i64 = ((max - 1) - (max % n as u64)) as i64;
        let mut v = self.int64();
        loop {
            if v <= max {
                break;
            }
            v = self.int64();
        }
        v % n
    }

    /// Returns a f32, a pseudo-random number in [0.0,1.0).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use srand::{Rand, RngSource};
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let mut r: Rand<_> = Rand::new(RngSource::new(1));
    ///     println!("n: {}", r.float32());
    ///     Ok(())
    /// }
    /// ```
    pub fn float32(&mut self) -> f32 {
        loop {
            let f = self.float64() as f32;
            if f != 1f32 {
                return f;
            }
        }
    }

    /// Returns a f64, a pseudo-random number in [0.0,1.0).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use srand::{Rand, RngSource};
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let mut r: Rand<_> = Rand::new(RngSource::new(1));
    ///     println!("n: {}", r.float64());
    ///     Ok(())
    /// }
    /// ```
    pub fn float64(&mut self) -> f64 {
        loop {
            let f: f64 = self.int64() as f64 / (1 << 63) as f64;
            if f != 1f64 {
                return f;
            }
        }
    }

    /// Shuffle pseudo-randomizes the order of elements.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use srand::{Rand, RngSource};
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let mut r: Rand<_> = Rand::new(RngSource::new(1));
    ///     let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    ///     println!("result: {:?}", r.shuffle(&mut v));
    ///     println!("shuffled array: {:?}", v);
    ///     Ok(())
    /// }
    /// ```
    pub fn shuffle<T>(&mut self, array: &mut Vec<T>) {
        let mut i = array.len() - 1;
        while i > 1 << 31 - 2 {
            let j = self.int64n((i + 1) as i64);
            array.swap(i as usize, j as usize);
            i -= 1;
        }
        while i > 0 {
            let j = self.int32n((i + 1) as i32);
            array.swap(i as usize, j as usize);
            i -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    pub use crate::source::{LockedSource, RngSource};
    #[test]
    fn examples() {
        println!("custom locked source");
        let r: super::Rand<_> = super::Rand::new(LockedSource::new(1));
        let mut handles = vec![];
        for i in 0..4 {
            let mut r: super::Rand<_> = r.clone();
            let h = std::thread::spawn(move || {
                for j in 0..3 {
                    println!("thread: {}, index: {}, {}", i, j, r.int64());
                }
            });
            handles.push(h);
        }
        for h in handles {
            h.join().unwrap();
        }

        println!("custom rng source: i32n(100)");
        let expected: Vec<i32> = vec![
            81, 87, 47, 59, 81, 18, 25, 40, 56, 0, 94, 11, 62, 89, 28, 74, 11, 45, 37, 6, 95, 66,
            28, 58, 47, 47, 87, 88, 90, 15, 41, 8, 87, 31, 29, 56, 37, 31, 85, 26, 13, 90, 94, 63,
            33, 47, 78, 24, 59, 53,
        ];
        let mut r: super::Rand<_> = super::Rand::new(RngSource::new(1));
        let mut get = vec![];
        for _i in 0..50 {
            get.push(r.int32n(100));
        }
        assert_eq!(expected, get);

        let mut r: super::Rand<_> = super::Rand::new(RngSource::new(1));
        for _i in 0..50 {
            // get.push(r.i32n(20));
            println!("r.i32n(20): {}", r.int32n(20))
        }

        println!("custom rng source: i64n(100)");
        let expected: Vec<i64> = vec![
            10, 51, 21, 51, 37, 20, 58, 48, 16, 49, 84, 87, 74, 36, 15, 73, 68, 91, 90, 31, 73, 56,
            11, 37, 78, 9, 72, 50, 88, 71, 44, 43, 23, 59, 3, 39, 83, 7, 32, 80, 15, 16, 20, 71,
            52, 7, 19, 62, 10, 97,
        ];
        let mut r: super::Rand<_> = super::Rand::new(RngSource::new(1));
        let mut get = vec![];
        for _i in 0..50 {
            get.push(r.int64n(100));
        }
        assert_eq!(expected, get);

        println!("custom rng source: i32()");
        let expected: Vec<i32> = vec![
            1298498081, 2019727887, 1427131847, 939984059, 911902081, 1474941318, 140954425,
            336122540, 208240456, 646203300, 1106410694, 1747278511, 460128162, 817455089,
            683024728, 1006933274, 607811211, 629431445, 1458323237, 469339106, 436340495,
            774965466, 1225511528, 1852186258, 629458047, 637979947, 1616138287, 443632888,
            1858292790, 1496193015, 1124895541, 60780408, 340007387, 1304066831, 2094315429,
            170625356, 1277341737, 126960631, 1486111485, 647515026, 372086413, 1162003090,
            1168565194, 598090563, 908712433, 1139424147, 544474078, 605764324, 1693516159,
            776971353,
        ];
        let mut r: super::Rand<_> = super::Rand::new(RngSource::new(1));
        let mut get = vec![];
        for _i in 0..50 {
            get.push(r.int32());
        }
        assert_eq!(expected, get);

        println!("custom rng source: i64()");
        let expected: Vec<i64> = vec![
            5577006791947779410,
            8674665223082153551,
            6129484611666145821,
            4037200794235010051,
            3916589616287113937,
            6334824724549167320,
            605394647632969758,
            1443635317331776148,
            894385949183117216,
            2775422040480279449,
            4751997750760398084,
            7504504064263669287,
            1976235410884491574,
            3510942875414458836,
            2933568871211445515,
            4324745483838182873,
            2610529275472644968,
            2703387474910584091,
            6263450610539110790,
            2015796113853353331,
            1874068156324778273,
            3328451335138149956,
            5263531936693774911,
            7955079406183515637,
            2703501726821866378,
            2740103009342231109,
            6941261091797652072,
            1905388747193831650,
            7981306761429961588,
            6426100070888298971,
            4831389563158288344,
            261049867304784443,
            1460320609597786623,
            5600924393587988459,
            8995016276575641803,
            732830328053361739,
            5486140987150761883,
            545291762129038907,
            6382800227808658932,
            2781055864473387780,
            1598098976185383115,
            4990765271833742716,
            5018949295715050020,
            2568779411109623071,
            3902890183311134652,
            4893789450120281907,
            2338498362660772719,
            2601737961087659062,
            7273596521315663110,
            3337066551442961397,
        ];
        let mut r: super::Rand<_> = super::Rand::new(RngSource::new(1));
        let mut get = vec![];
        for _i in 0..50 {
            get.push(r.int64());
        }
        assert_eq!(expected, get);

        let mut r: super::Rand<_> = super::Rand::new(RngSource::new(1));
        // r.seed(1);
        let mut v = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];
        r.shuffle(&mut v);
        println!("shuffle: {:?}", v);
    }
}
