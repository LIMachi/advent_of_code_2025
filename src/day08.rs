use std::collections::HashMap;
use std::usize;
use partser::prelude::*;
use partser::number::i64;
use crate::vec3::Vec3;

/*
#[derive(Debug, Copy, Clone)]
struct Box {
    position: Vec3,
    circuit: Option<usize>
}

impl Into<Vec3> for Box {
    fn into(self) -> Vec3 {
        self.position
    }
}

#[derive(Debug, Default)]
struct SparseWorld {
    chunks: HashMap<Vec3, Vec<Box>>,
}

fn best_different_pairs<'a, T: Into<Vec3> + Clone, I: Iterator<Item = &'a T> + Clone>(view: I, discriminant: impl Fn(&T, &T) -> bool) -> Vec<(&'a T, &'a T)> {
    let mut best_dist = f64::MAX;
    let mut bests = Vec::with_capacity(1);
    let start = view.clone();
    for (ia, a) in view.enumerate() {
        for (ib, b) in start.clone().enumerate() {
            if ia == ib || discriminant(a, b) {
                continue;
            }
            let va: Vec3 = a.clone().into();
            let vb: Vec3 = b.clone().into();
            let dist = va.dist_squared(&vb);
            if dist <= best_dist {
                if dist < best_dist {
                    bests.clear();
                }
                best_dist = dist;
                bests.push((a, b));
            }
        }
    }
    bests
}

impl SparseWorld {
    const CHUNK_SIZE: i64 = 512;

    fn insert(&mut self, position: Vec3) {
        let chunk = position.scale(-Self::CHUNK_SIZE);
        if !self.chunks.contains_key(&chunk) {
            self.chunks.insert(chunk, Vec::with_capacity(1));
        }
        self.chunks.get_mut(&chunk).unwrap().push(Box { position, circuit: None });
    }

    fn best_pair(&self) -> Option<(&Box, &Box)> {
        let mut proximity = 1;
        loop {
            best_different_pairs(chunks.iter().flat_map(|c| self.chunks[]), |b1, b2| match (b1.circuit, b2.circuit) {
                (Some(c1), Some(c2)) => c1 == c2,
                (None, _) | (_, None) => false,
            })
        }
    }
}
*/
fn parse(input: &str) -> Vec<Vec3> {
    // let mut out = SparseWorld::default();
    let mut out = Vec::new();
    for line in input.lines() {
        out.push(line.parse_with(true, i64.rep_separated(',', 3..=3, true).map_ok(|v| Vec3 { x: v[0], y: v[1], z: v[2] })).unwrap());
    }
    out
}

/*
fn best_pair(boxes: &Vec<Box>) -> Option<(usize, usize)> {
    let mut best = f64::MAX;
    let mut pair = None;
    for a in 0..boxes.len() - 1 {
        for b in a + 1..boxes.len() {
            if match (boxes[a].circuit, boxes[b].circuit) {
                (Some(c1), Some(c2)) => c1 != c2,
                (None, _) | (_, None) => true,
            } {
                let dist = boxes[a].position.dist_squared(&boxes[b].position);
                if dist < best {
                    best = dist;
                    pair = Some((a, b));
                }
            }
        }
    }
    pair
}
*/

//first try got 1000 after 30+ seconds (invalid result, too low, and too slow anyway)
/*
fn _a(mut boxes: Vec<Box>, mut limit: usize) -> usize {
    let mut circuits = Vec::new();
    while let Some((a, b)) = best_pair(&boxes) {
        limit -= 1;
        if limit == 0 {
            break;
        }
        // dbg!(boxes[a], boxes[b]);
        match (boxes[a].circuit, boxes[b].circuit) {
            (None, None) => {
                //new circuit
                let id = circuits.len();
                circuits.push(vec![a, b]);
                //1 allocation!
                boxes[a].circuit = Some(id);
                boxes[b].circuit = Some(id);
            }
            (Some(c), None) => {
                //link b to a
                circuits[c].push(b);
                boxes[b].circuit = Some(c);
            }
            (None, Some(c)) => {
                //link a to b
                circuits[c].push(a);
                boxes[a].circuit = Some(c);
            }
            (Some(c1), Some(c2)) => {
                //merge a inside b
                //2 allocations!
                let t: Vec<usize> = circuits[c2].iter().map(|i| {boxes[*i].circuit = Some(c1); *i}).collect();
                circuits[c1].extend(t);
                circuits[c2].clear();
            }
        }
    }
    // dbg!(&circuits);
    let mut circuit_lengths: Vec<usize> = circuits.iter().filter_map(|c| if c.len() > 0 { Some(c.len()) } else { None }).collect();
    circuit_lengths.sort_by(|a, b| b.cmp(a));
    // dbg!(&circuit_lengths);
    circuit_lengths.iter().take(3).copied().reduce(|acc, v| acc * v).unwrap()
}
*/

//after some research (I usually try to do advent of code without searching on the internet or talking with an ai, but this problem was annoying to debug when each try took 40s)
//and ended up founding Kruskal's algorithm (which is a carbon copy of what is asked in the problem, except the limitation on 1000 iterations)
// https://en.wikipedia.org/wiki/Kruskal%27s_algorithm
// https://en.wikipedia.org/wiki/Disjoint-set_data_structure
#[derive(Debug)]
struct DSU {
    parents: Vec<usize>,
    rank: Vec<u8>,
}

impl DSU {
    fn new(len: usize) -> Self {
        Self {
            parents: (0..len).collect(),
            rank: vec![0; len],
        }
    }

    fn find(&mut self, link: usize) -> usize {
        if self.parents[link] != link {
            self.parents[link] = self.find(self.parents[link]);
        }
        self.parents[link]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut a = self.find(a);
        let mut b = self.find(b);
        if a != b {
            if self.rank[a] < self.rank[b] {
                let t = a;
                a = b;
                b = t;
            }
            self.parents[b] = a;
            if self.rank[a] == self.rank[b] {
                self.rank[a] += 1;
            }
        }
    }
}

//now takes about 1 second to get 54180
fn a(boxes: Vec<Vec3>, mut limit: usize) -> usize {
    let mut distances: Vec<(usize, usize, f64)> = Vec::with_capacity(boxes.len() * boxes.len());
    let mut dsu = DSU::new(boxes.len());
    for a in 0..boxes.len() - 1 {
        for b in a + 1..boxes.len() {
            distances.push((a, b, boxes[a].dist_squared(&boxes[b])));
        }
    }
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    for (a, b, _) in &distances {
        if dsu.find(*a) != dsu.find(*b) {
            dsu.union(*a, *b);
        }
        limit -= 1;
        if limit == 0 {
            break;
        }
    }
    for i in 0..boxes.len() {
        dsu.find(i);
    }
    let mut groups = vec![0; boxes.len()];
    for root in &dsu.parents {
        groups[*root] += 1;
    }
    groups = groups.into_iter().filter(|&group| group > 1).collect::<Vec<_>>();
    groups.sort_by(|&a, &b| b.cmp(&a));
    groups.iter().take(3).copied().reduce(|acc, v| acc * v).unwrap()
}

fn b(boxes: Vec<Vec3>) -> i64 {
    let mut distances: Vec<(usize, usize, f64)> = Vec::with_capacity(boxes.len() * boxes.len());
    let mut dsu = DSU::new(boxes.len());
    for a in 0..boxes.len() - 1 {
        for b in a + 1..boxes.len() {
            distances.push((a, b, boxes[a].dist_squared(&boxes[b])));
        }
    }
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    let mut unions = 0;
    for (a, b, _) in &distances {
        if dsu.find(*a) != dsu.find(*b) {
            dsu.union(*a, *b);
            unions += 1;
            if unions >= boxes.len() - 1 {
                return boxes[*a].x * boxes[*b].x;
            }
        }
    }
    0
}

pub(crate) fn day08() {
    // let input = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";
    let input = include_str!("day08.txt");
    let input = parse(input);
    println!("\nDay 08");
    println!("a: {}", a(input.clone(), 1000));
    println!("b: {}", b(input));
}