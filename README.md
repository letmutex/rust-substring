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
skip take and collect      time:   [304.61 ns 305.72 ns 306.83 ns]
manual byte indices        time:   [95.303 ns 95.676 ns 96.064 ns]
2 char_indices()           time:   [85.484 ns 85.835 ns 86.201 ns]
1 char_indices()           time:   [85.672 ns 86.142 ns 86.641 ns]
left-right char skipping   time:   [85.784 ns 86.215 ns 86.650 ns]
```

substring(1, 2000) on `"Hello✨, 🎈 this 🎉 is a text.".repeat(100)`:

```
skip take and collect      time:   [4.5254 µs 4.5599 µs 4.5986 µs]
manual byte indices        time:   [2.5376 µs 2.5508 µs 2.5639 µs]
2 char_indices()           time:   [1.7670 µs 1.7749 µs 1.7841 µs]
1 char_indices()           time:   [1.7653 µs 1.7750 µs 1.7853 µs]
left-right char skipping   time:   [1.4175 µs 1.4250 µs 1.4330 µs]
```
