# weight-rs

[![Build Status](https://travis-ci.org/smallnest/weighted-rs.svg?branch=master)](https://travis-ci.org/smallnest/weighted-rs)
[![Crate](https://img.shields.io/crates/v/weighted-rs.svg)](https://crates.io/crates/weighted-rs)
[![API](https://docs.rs/weighted-rs/badge.svg)](https://docs.rs/weighted-rs)

A Rust library for weighted balancing algorithm.


It provides three weighted balancing (elect) algorithm.
One is random algorithm.
Another is weighted balancing algorithm used by LVS.
The third is smooth weighted balancing algorithm used by Nginx.

The LVS weighted round-robin scheduling is introduced at http://kb.linuxvirtualserver.org/wiki/Weighted_Round-Robin_Scheduling.
The Nginx smooth weighted round-robin balancing algorithm is introduced at https://github.com/phusion/nginx/commit/27e94984486058d73157038f7950a0a36ecc6e35.
The random algorithm is not smooth although it follows weight configuration.

Using it is simple:
```rust
    use weighted-rs::{SmoothWeight, Weight};
    use std::collections::HashMap;

    let mut sw: SmoothWeight<&str> = SmoothWeight::new();
    sw.add("server1", 5);
    sw.add("server2", 2);
    sw.add("server3", 3);

    for _ in 0..100 {
        let s = sw.next().unwrap();
        println!("{}", s);
    }
```


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
weighted-rs = "0.1.1"
```


# License

weighted-rs is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
