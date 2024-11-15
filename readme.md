# rust-open-addressing-linear-probing

benchmarks
```sh
hyperfine \
  --warmup 3 \
  --prepare 'cargo clean && cargo build --release' \
  'target/release/rust-open-addresing-linear-probing --capacity 10000' \
  'target/release/rust-open-addresing-linear-probing --custom --capacity 10000'
```

output:
```
flox [default] ➜  rust-open-addresing-linear-probing git:(master) ✗ hyperfine \
  --warmup 3 \
  --prepare 'cargo clean && cargo build --release' \
  'target/release/rust-open-addresing-linear-probing --capacity 1000000' \
  'target/release/rust-open-addresing-linear-probing --custom --capacity 1000000'
Benchmark 1: target/release/rust-open-addresing-linear-probing --capacity 1000000
  Time (mean ± σ):     142.5 ms ±   2.6 ms    [User: 1.8 ms, System: 3.3 ms]
  Range (min … max):   139.0 ms … 146.7 ms    10 runs
 
Benchmark 2: target/release/rust-open-addresing-linear-probing --custom --capacity 1000000
  Time (mean ± σ):     140.8 ms ±   3.4 ms    [User: 1.7 ms, System: 3.1 ms]
  Range (min … max):   134.4 ms … 146.9 ms    10 runs
 
Summary
  target/release/rust-open-addresing-linear-probing --custom --capacity 1000000 ran
    1.01 ± 0.03 times faster than target/release/rust-open-addresing-linear-probing --capacity 1000000
```
