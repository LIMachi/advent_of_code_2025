use std::ops::RangeInclusive;
use partser::prelude::*;
use partser::number::usize;

fn parse(input: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    for line in input.lines() {
        if let Ok((start, _, finish)) = line.parse_with(true, (usize, '-', usize)) {
            ranges.push(RangeInclusive::new(start, finish));
        } else if let Ok(id) = line.parse_with(true, usize) {
            ids.push(id);
        }
    }
    (ranges, ids)
}

fn a(ranges: &[RangeInclusive<usize>], ids: &[usize]) -> usize {
    let mut count = 0;
    'ids: for id in ids {
        for range in ranges {
            if range.contains(id) {
                count += 1;
                continue 'ids;
            }
        }
    }
    count
}

#[derive(Debug, Default)]
struct MultiRange(Vec<RangeInclusive<usize>>);

impl MultiRange {
    pub fn insert(&mut self, range: &RangeInclusive<usize>) {
        let start = *range.start();
        let end = *range.end();
        for i in 0..self.0.len() {
            let inner = &self.0[i];
            let cs = inner.contains(&start);
            let ce = inner.contains(&end);
            if cs && ce {
                return;
            }
            if ce {
                self.0[i] = RangeInclusive::new(start, *inner.end());
                return;
            }
            if cs {
                if self.0.get(i + 1).is_some_and(|nr| nr.contains(&end)) {
                    self.0[i] = RangeInclusive::new(*inner.start(), *self.0[i + 1].end());
                    self.0.remove(i + 1);
                } else {
                    self.0[i] = RangeInclusive::new(*inner.start(), end);
                }
                return;
            }
            if range.contains(inner.start()) && range.contains(inner.end()) {
                if self.0.get(i + 1).is_some_and(|nr| nr.contains(&end)) {
                    self.0[i] = RangeInclusive::new(start, *self.0[i + 1].end());
                    self.0.remove(i + 1);
                } else {
                    self.0[i] = range.clone();
                }
                return;
            }
            if end < *inner.start() {
                self.0.insert(i, range.clone());
                return;
            }
        }
        self.0.push(range.clone());
    }

    pub fn len_included(&self) -> usize {
        let mut acc = 0;
        for inner in &self.0 {
            acc += inner.end() - inner.start() + 1;
        }
        acc
    }
}

fn b(ranges: &[RangeInclusive<usize>]) -> usize {
    let mut multi = MultiRange(Vec::new());
    for range in ranges {
        multi.insert(range);
    }
    multi.len_included()
}

pub(crate) fn day05() {
    // let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
    let input = include_str!("day05.txt");
    let (ranges, ids) = parse(input);
    println!("\nDay 05");
    println!("a: {}", a(&ranges, &ids));
    println!("b: {}", b(&ranges));
}