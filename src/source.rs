pub trait Source {
    fn i64(&mut self) -> i64;
    fn u64(&mut self) -> u64;
    fn seed(&mut self, seed: i64);
}
