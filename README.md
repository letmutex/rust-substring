# Effective substring in Rust

View code [substring.rs](./lib/substring.rs)

View blog [Effective substring in Rust](https://letmutex.com/article/effective-substring-in-rust)

# Get Started

Run benchmark

```bash
cargo bench
```

Run tests

```bash
cargo test
```

# Benchmark Results

substring(1, 20) on `Hello✨, 🎈 this 🎉 is a text.`:

```
skip take and collect      time:   [313.51 ns 315.61 ns 318.00 ns]
manual byte indices        time:   [43.970 ns 44.178 ns 44.407 ns]
2 char_indices()           time:   [30.878 ns 31.016 ns 31.160 ns]
1 char_indices()           time:   [31.925 ns 32.055 ns 32.189 ns]
left-right char skipping   time:   [30.525 ns 30.667 ns 30.817 ns]
```

substring(1, 20000) on `"Hello✨, 🎈 this 🎉 is a text.".repeat(1000)`:

```
skip take and collect      time:   [50.435 µs 50.770 µs 51.132 µs]
manual byte indices        time:   [29.578 µs 29.742 µs 29.923 µs]
2 char_indices()           time:   [16.551 µs 16.659 µs 16.780 µs]
1 char_indices()           time:   [16.583 µs 16.664 µs 16.751 µs]
left-right char skipping   time:   [14.096 µs 14.338 µs 14.584 µs]
```
