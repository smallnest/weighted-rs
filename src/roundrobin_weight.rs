use super::Weight;

#[derive(Clone, Debug)]
struct RRWeightItem<T> {
    item: T,
    weight: isize,
}

/// RoundrobinWeight is a struct that contains weighted items implement LVS weighted round robin
/// algorithm.
///
/// http://kb.linuxvirtualitem.org/wiki/Weighted_Round-Robin_Scheduling
///
/// http://zh.linuxvirtualitem.org/node/37
#[derive(Debug, Default)]
pub struct RoundrobinWeight<T> {
    items: Vec<RRWeightItem<T>>,
    gcd: isize,
    max_w: isize,
    i: isize,
    cw: isize,
}

impl<T: Clone> RoundrobinWeight<T> {
    pub const fn new() -> Self {
        RoundrobinWeight {
            items: Vec::new(),
            gcd: 0,
            max_w: 0,
            i: 0,
            cw: 0,
        }
    }
}

impl<T: Clone> Weight for RoundrobinWeight<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.items.len() <= 1 {
            return self.items.first().map(|itme| itme.item.clone());
        }

        loop {
            self.i = (self.i + 1) % (self.items.len() as isize);
            if self.i == 0 {
                self.cw -= self.gcd;
                if self.cw <= 0 {
                    self.cw = self.max_w;
                    if self.cw == 0 {
                        return None;
                    }
                }
            }

            if self.items[self.i as usize].weight >= self.cw {
                return Some(self.items[self.i as usize].item.clone());
            }
        }
    }
    // add adds a weighted item for selection.
    fn add(&mut self, item: T, weight: isize) {
        let weight_item = RRWeightItem { item, weight };

        if weight > 0 {
            if self.gcd == 0 {
                self.gcd = weight;
                self.max_w = weight;
                self.i = -1;
                self.cw = 0;
            } else {
                self.gcd = gcd(self.gcd, weight);
                if self.max_w < weight {
                    self.max_w = weight;
                }
            }
        }

        self.items.push(weight_item);
    }

    fn all(&self) -> impl Iterator<Item = (Self::Item, isize)> + '_ {
        self.items
            .iter()
            .map(|item| (item.item.clone(), item.weight))
    }

    fn remove_all(&mut self) {
        self.items.clear();
        self.gcd = 0;
        self.max_w = 0;
        self.i = -1;
        self.cw = 0;
    }

    fn reset(&mut self) {
        self.i = -1;
        self.cw = 0;
    }
}

#[allow(clippy::many_single_char_names)]
fn gcd(x: isize, y: isize) -> isize {
    let mut t: isize;
    let mut a = x;
    let mut b = y;
    loop {
        t = a % b;
        if t > 0 {
            a = b;
            b = t;
        } else {
            return b;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{RoundrobinWeight, Weight};
    use std::collections::HashMap;

    #[test]
    fn test_rr_weight() {
        let mut rrw: RoundrobinWeight<&str> = RoundrobinWeight::new();
        rrw.add("server1", 5);
        rrw.add("server2", 2);
        rrw.add("server3", 3);

        let mut results: HashMap<&str, usize> = HashMap::new();

        for _ in 0..100 {
            let s = rrw.next().unwrap();
            // *results.get_mut(s).unwrap() += 1;
            *results.entry(s).or_insert(0) += 1;
        }

        assert_eq!(results["server1"], 50);
        assert_eq!(results["server2"], 20);
        assert_eq!(results["server3"], 30);
    }
}
