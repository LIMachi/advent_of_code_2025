//first optimisations visible (since lights are the only mutable thing, and the maximum number of lights seem to be around 10):
//use u16 to represent the state of the lights instead of Vec<bool>
//since the lights (in the diagram) are the target, we can also see the reverse operation (starting from the light diagram and trying to turn them all off) as a valid solution
//now to apply a connection on the light we can use a simple xor
//proof: target 6 (.##.) using button connections 5 (2, 0) and 3 (1, 0)
//6 ^ 5 -> 0110 ^ 0101 -> 0011 -> 3
//3 ^ 3 -> 0011 ^ 0011 -> 0000 -> 0
//and in reverse (the order of the xor don't matter)
//0 ^ 5 -> 0000 ^ 0101 -> 0101 -> 5
//5 ^ 3 -> 0101 ^ 0011 -> 0110 -> 6

//sadly we can't use this trick for the second part (as some values are in the 100 so 7 bits, by 10 positions, that's over 64 bits but could still fit in 128 bits, and we need a logic to stop a branch if we overflow one counter)
//I feel I've already have done something similar in another AoC
//my guess is to find wich buttons are unique (as in, the only button that affect a specific joltage) and press it as many time as necessary to set the unique joltage.
//by doing so it will eliminate one variable and start the other buttons a lot higher (and potentially singeling other buttons that could use the same trick)

//lets try this with (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
//0 is present 2 times
//1 is present 2 times
//2 is present 3 times
//3 is present 3 times
//no obvious outlier, but we know that 0 need to be incremented 3 times, so that leaves those permutations (A: (0,1), B: (0,2))
//AAA {3,3,0,0}, AAB {3,2,1,0}, ABB {3,1,2,0} and BBB {3,0,3,0}
//let's look the status of 1 and 2:
//1 is linked to 3
//2 is linked to 3 or alone
//1 is more bounded, so use BBB instead
//let's call C (1,3)
//BBBCCCCC {3,5,3,5}
//we only need to add one instance of (2, 3) and one of (3) to finish the sequence, so 10 presses
//if we bruteforced this, it could have tested 30-50 permutations just for this simple input

//lets try the second one (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
//0 is present 3 times
//1 is present 2 times
//3 is present 3 times
//4 is present 3 times
//the lowest target is 4 that should be present 2 times
//lets try with permutations of A: (0,2,3,4) or B: (1,2,3,4)
//AA {2,0,2,2,2}, AB {1,1,2,2,2}, BB {0,2,2,2,2}
//now we are left with (2,3) (0,4) (0,1,2)
//0 is present 2 times (target 7 from 0-2)
//1 is present 1 times (target 5 from 0-2)
//2 is present 2 times (target 12 from 2)
//3 is present 1 times (target 7 from 2)
//4 is already reached
//here we can solve 3 (only one valid input and target), so we know that we need to insert 5 times (2,3), which would give us {0-2,0-2, 7, 7, 2}
//now we are left with (0,4) (0,1,2)
//0 is present 2 times (target 7 from 0-2)
//1 is present 1 times (target 5 from 0-2)
//2 is present 1 times (target 12 from 7)
//3 is already reached
//4 is already reached
//solve 2 as 5 times (0, 1, 2) which would give us {5-7,5-7, 12, 7, 2}
//we finally can conclude that the correct selection was 2 times (0,2,3,4)
//2 (0,2,3,4) + 5 (2,3) + 5 (0,1,2) 12 presses (actually the same result as the example)

//finally with (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
//0 3
//1 3
//2 3
//3 2
//4 3
//5 1
//5 * (0,1,2,4,5) {5,5,5,0,5,5}
//(0,1,2,3,4) (0,3,4) (1,2)
//0 2 from 5 to 10
//1 2 from 5 to 11
//2 2 from 5 to 11
//3 2 from 0 to 5
//4 2 from 5 to 10
//5 * (0,1,2,3,4) {10,10,10,5,10} (we reached the target for 0, 3 and 4 at the same time)
//1 * (1,2)
//11 total moves

//test with our first real input:
//[.##.#..#.] (2,3,4,6) (0,1,3,4,5,6) (1,2,3,7,8) (0,1,3,5,6,7,8) (0,1,3,4,5,6,8) (0,1,4,7,8) (2,4,6,7,8) {41,56,34,37,46,22,41,67,71}
//0 4
//1 5
//2 3
//3 5
//4 5
//5 3
//6 5
//7 4
//8 5

//just realised something: this is like a matrix multiplying a vector resulting in another vector

//   0  1  2  3  4  5  6  7  8
//A [0, 0, 1, 1, 1, 0, 1, 0, 0] (2,3,4,6)
//B [1, 1, 0, 1, 1, 1, 1, 0, 0] (0,1,3,4,5,6)
//C [0, 1, 1, 1, 0, 0, 0, 1, 1] (1,2,3,7,8)
//D [1, 1, 0, 1, 0, 1, 1, 1, 1] (0,1,3,5,6,7,8)
//E [1, 1, 0, 1, 1, 1, 1, 0, 1] (0,1,3,4,5,6,8)
//F [1, 1, 0, 0, 1, 0, 0, 1, 1] (0,1,4,7,8)
//G [0, 0, 1, 0, 1, 0, 1, 1, 1] (2,4,6,7,8)
//= {41,56,34,37,46,22,41,67,71}

//for once I won't be writing my matrice struct and use nalgebra instead and do vector and matrices calculations

use std::collections::HashSet;
use std::mem;
use nalgebra::{DMatrix, DVector, RowDVector};
use partser::number::isize;

type Lsize = u16;

#[derive(Debug, Clone)]
struct Diagram {
    lights: Lsize,
    connections: Vec<Lsize>,
    joltage: Vec<isize>,
}

fn parse(input: &str) -> Vec<Diagram> {
    use partser::prelude::*;

    fn partser(reader: StringReader) -> ParserOut<Diagram> {
        let lights = delimited('[', rep(1.., true, ".#".any().map_ok(|c| c == '#')), ']');
        let connection = delimited('(', rep_separated(1.., true, usize, ','), ')');
        let connections = rep_separated(1.., true, connection, ' ');
        let joltage = delimited('{', rep_separated(1.., true, isize, ','), '}');
        (lights, ' ', connections, ' ', joltage).map_ok(|(l, _, b, _, joltage)| {
            let mut lights: Lsize = 0;
            for (i, &light) in l.iter().enumerate() {
                if light {
                    lights |= 1 << i;
                }
            }
            let mut connections = Vec::with_capacity(b.len());
            for button in &b {
                let mut connection = 0u16;
                for s in button {
                    connection |= 1 << s;
                }
                connections.push(connection);
            }
            Diagram { lights, connections, joltage }
        })(reader)
    }

    input.lines().map(|line| line.parse_with(true, partser).unwrap()).collect()
}

fn a(diagrams: &[Diagram]) -> usize {
    let mut acc = 0;
    for diagram in diagrams {
        let mut visited = HashSet::new();
        let mut batch = vec![diagram.lights];
        let mut next = Vec::new();
        let mut cycles = 0;
        'main: loop {
            for &state in &batch {
                if state == 0 {
                    break 'main;
                }
                visited.insert(state);
                for &xor in &diagram.connections {
                    let xored = state ^ xor;
                    if visited.contains(&xored) {
                        continue;
                    }
                    next.push(xored);
                }
            }
            cycles += 1;
            mem::swap(&mut batch, &mut next);
            next.clear();
        }
        acc += cycles;
    }
    acc
}

fn b(diagrams: &[Diagram]) -> usize {
    let mut acc = 0;
    for diagram in diagrams.iter().take(1) {
        let mut rows = vec![0isize; diagram.joltage.len() * diagram.connections.len()];
        for y in 0..diagram.connections.len() {
            for x in 0..diagram.joltage.len() {
                if diagram.connections[y] & 1 << x != 0 {
                    rows[x + y * diagram.joltage.len()] = 1;
                }
            }
        }

        let matrix = DMatrix::from_row_slice(diagram.connections.len(), diagram.joltage.len(), &rows);
        println!("mat: {matrix}");
        let target = RowDVector::from_row_slice(&diagram.joltage);
        println!("target: {target}");
        let test: RowDVector<isize> = RowDVector::from_row_slice(&[1, 3, 0, 3, 1, 2]);
        println!("test: {test}");

        dbg!(test * matrix == target);
    }
    acc
}

pub(crate) fn day10() {
    let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    // let input = include_str!("day10.txt");
    let input = parse(input);
    // dbg!(&input);
    println!("\nDay 10");
    // println!("a: {}", a(&input));
    println!("b: {}", b(&input));
}