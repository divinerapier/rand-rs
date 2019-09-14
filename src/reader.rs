use std::fs::File;
use std::io::Read;
use std::sync::Mutex;

lazy_static::lazy_static! {
    /// Easy implementation: read from /dev/urandom.
    /// This is sufficient on Linux, OS X, and FreeBSD.
    static ref RANDOM_READER: Mutex<Reader<File>> = {
        #[cfg(any(linux,unix,macos))]
        let f : File = File::open("/dev/urandom").unwrap();
        Mutex::new(Reader::new(f))
    };
}

pub struct Reader<R>
where
    R: Read,
{
    r: R,
}

impl<R: Read> Reader<R> {
    pub fn new(r: R) -> Reader<R> {
        Reader { r }
    }
}

impl<R: Read> Read for Reader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::result::Result<usize, std::io::Error> {
        self.r.read_exact(buf)?;
        Ok(buf.len())
    }
}

pub fn read(buf: &mut [u8]) -> std::result::Result<usize, std::io::Error> {
    let mut r = RANDOM_READER.lock().unwrap();
    r.read(buf)
}

#[cfg(test)]
mod test {
    #[test]
    fn random_read_examples() {
        let mut buffer = Vec::with_capacity(16);
        buffer.resize(16, 0u8);
        println!("{:?}", super::read(&mut buffer));
        println!("buffer: {:?}", buffer);
    }
}
