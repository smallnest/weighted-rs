use super::Weight;
use std::{collections::HashMap, hash::Hash};

#[derive(Copy, Clone, Debug)]
struct SmoothWeightItem<T> {
    item: T,
    weight: isize,
    current_weight: isize,
    effective_weight: isize,
}

// SW (Smooth Weighted) is a struct that contains weighted items and provides methods to select a
// weighted item. It is used for the smooth weighted round-robin balancing algorithm. This algorithm
// is implemented in Nginx: https://github.com/phusion/nginx/commit/27e94984486058d73157038f7950a0a36ecc6e35.
// Algorithm is as follows: on each peer selection we increase current_weight
// of each eligible peer by its weight, select peer with greatest current_weight
// and reduce its current_weight by total number of weight points distributed
// among peers.
// In case of { 5, 1, 1 } weights this gives the following sequence of
// current_weight's: (a, a, b, a, c, a, a)
#[derive(Default)]
pub struct SmoothWeight<T> {
    items: Vec<SmoothWeightItem<T>>,
    n: isize,
}

impl<T: Copy + PartialEq + Eq + Hash> SmoothWeight<T> {
    pub fn new() -> Self {
        SmoothWeight {
            items: Vec::new(),
            n: 0,
        }
    }

    //https://github.com/phusion/nginx/commit/27e94984486058d73157038f7950a0a36ecc6e35
    fn next_smooth_weighted(&mut self) -> Option<SmoothWeightItem<T>> {
        let mut total = 0;

        let mut best = self.items[0];
        let mut best_index = 0;
        let mut found = false;

        let items_len = self.items.len();
        for i in 0..items_len {
            self.items[i].current_weight += self.items[i].effective_weight;
            total += self.items[i].effective_weight;
            if self.items[i].effective_weight < self.items[i].weight {
                self.items[i].effective_weight += 1;
            }

            if !found || self.items[i].current_weight > best.current_weight {
                best = self.items[i];
                found = true;
                best_index = i;
            }
        }

        if !found {
            return None;
        }

        self.items[best_index].current_weight -= total;
        Some(best)
    }
}

impl<T: Copy + PartialEq + Eq + Hash> Weight for SmoothWeight<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.n == 0 {
            return None;
        }
        if self.n == 1 {
            return Some(self.items[0].item);
        }

        let rt = self.next_smooth_weighted()?;
        Some(rt.item)
    }
    // add adds a weighted item for selection.
    fn add(&mut self, item: T, weight: isize) {
        let weight_item = SmoothWeightItem {
            item,
            weight,
            current_weight: 0,
            effective_weight: weight,
        };

        self.items.push(weight_item);
        self.n += 1;
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
    }

    // reset resets the balancing algorithm.
    fn reset(&mut self) {
        for w in &mut self.items {
            w.current_weight = 0;
            w.effective_weight = w.weight;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{SmoothWeight, Weight};
    use std::collections::HashMap;

    #[test]
    fn test_smooth_weight() {
        let mut sw: SmoothWeight<&str> = SmoothWeight::new();
        sw.add("server1", 5);
        sw.add("server2", 2);
        sw.add("server3", 3);

        let mut results: HashMap<&str, usize> = HashMap::new();

        for _ in 0..100 {
            let s = sw.next().unwrap();
            // *results.get_mut(s).unwrap() += 1;
            *results.entry(s).or_insert(0) += 1;
        }

        assert_eq!(results["server1"], 50);
        assert_eq!(results["server2"], 20);
        assert_eq!(results["server3"], 30);
    }
}
