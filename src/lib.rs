//! A libray for weighted balancing algorithm.
//! It provides three weighted balancing (elect) algorithm.
//! One is random algorithm.
//! Another is weighted balancing algorithm used by LVS.
//! The third is smooth weighted balancing algorithm used by Nginx.
//!
//! The LVS weighted round-robin scheduling is introduced at http://kb.linuxvirtualserver.org/wiki/Weighted_Round-Robin_Scheduling.
//! The Nginx smooth weighted round-robin balancing algorithm is introduced at https://github.com/phusion/nginx/commit/27e94984486058d73157038f7950a0a36ecc6e35.
//! The random algorithm is not smooth although it follows weight configuration.
//! Using it is simple:
//! ```rust
//!     use weighted_rs::{SmoothWeight, Weight};
//!     use std::collections::HashMap;
//!
//!     let mut sw: SmoothWeight<&str> = SmoothWeight::new();
//!     sw.add("server1", 5);
//!     sw.add("server2", 2);
//!     sw.add("server3", 3);
//!
//!     for _ in 0..100 {
//!         let s = sw.next().unwrap();
//!         println!("{}", s);
//!     }
//! ```

pub mod random_weight;
pub mod roundrobin_weight;
pub mod smooth_weight;

pub use random_weight::*;
pub use roundrobin_weight::*;
pub use smooth_weight::*;

/// A common trait for weight algorithm.
pub trait Weight: Iterator {
    /// adds a weighted item for selection.
    fn add(&mut self, item: Self::Item, weight: isize);

    /// returns all items.
    fn all(&self) -> impl Iterator<Item = (Self::Item, isize)> + '_;

    /// removes all weighted items.
    fn remove_all(&mut self);

    /// resets the balancing algorithm.
    fn reset(&mut self);
}
