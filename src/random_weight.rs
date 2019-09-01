use super::Weight;
use rand::prelude::{Rng, ThreadRng};
use std::{collections::HashMap, hash::Hash};

#[derive(Copy, Clone, Debug)]
struct RandWeightItem<T> {
    item: T,
    weight: isize,
}

// Use the random algorithm to select next item.
#[derive(Default)]
pub struct RandWeight<T> {
    items: Vec<RandWeightItem<T>>,
    n: usize,
    sum_of_weights: isize,
    r: ThreadRng,
}

impl<T: Copy + PartialEq + Eq + Hash> RandWeight<T> {
    pub fn new() -> Self {
        RandWeight {
            items: Vec::new(),
            n: 0,
            sum_of_weights: 0,
            r: rand::thread_rng(),
        }
    }
}

impl<T: Copy + PartialEq + Eq + Hash> Weight for RandWeight<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.n == 0 {
            return None;
        }
        if self.n == 1 {
            return Some(self.items[0].item);
        }

        let mut index = self.r.gen_range(0, self.sum_of_weights);
        for item in &self.items {
            index -= item.weight;
            if index <= 0 {
                return Some(item.item);
            }
        }

        Some(self.items[self.n - 1].item)
    }
    // add adds a weighted item for selection.
    fn add(&mut self, item: T, weight: isize) {
        let weight_item = RandWeightItem { item, weight };

        self.items.push(weight_item);
        self.n += 1;
        self.sum_of_weights += weight;
    }

    // all returns all items.
    fn all(&self) -> HashMap<T, isize> {
        let mut rt: HashMap<T, isize> = HashMap::new();
        for w in &self.items {
            rt.insert(w.item, w.weight);
        }
        rt
    }

    // remove_all removes all weighted items.
    fn remove_all(&mut self) {
        self.items.clear();
        self.n = 0;
        self.r = rand::thread_rng();
    }

    // reset resets the balancing algorithm.
    fn reset(&mut self) {
        self.r = rand::thread_rng();
    }
}

#[cfg(test)]
mod tests {
    use crate::{RandWeight, Weight};
    use std::collections::HashMap;

    #[test]
    fn test_smooth_weight() {
        let mut sw: RandWeight<&str> = RandWeight::new();
        sw.add("server1", 5);
        sw.add("server2", 2);
        sw.add("server3", 3);

        let mut results: HashMap<&str, usize> = HashMap::new();

        for _ in 0..10000 {
            let s = sw.next().unwrap();
            // *results.get_mut(s).unwrap() += 1;
            *results.entry(s).or_insert(0) += 1;
        }

        println!("{:?}", results);
        // assert!(results["server1"] > 4000 && results["server1"] < 6000);
        // assert!(results["server2"] > 1000 && results["server1"] < 3000);
        // assert!(results["server3"] > 2000 && results["server1"] < 4000);
    }
}
