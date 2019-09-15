# Srand

Random number generators and other randomness functionality inspired by golang standard library with simple apis to use. 

## A simple rng random generator

``` rust
let mut r: Rand<_> = Rand::new(RngSource::new(1));
let mut get = vec![];
for _i in 0..50 {
    get.push(r.int32n(100));
}
```

## A threads safe random generator

``` rust
let r: Rand<_> = Rand::new(LockedSource::new(1));
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
```

## Thread local apis

``` rust
srand::ThreadLocal::seed(1234567);
srand::ThreadLocal::int32();
srand::ThreadLocal::uint32();
srand::ThreadLocal::int64();
srand::ThreadLocal::uint64();
```

## Random data

``` rust
let mut buffer = Vec::with_capacity(16);
buffer.resize(16, 0u8);
srand::read(&mut buffer);
```
