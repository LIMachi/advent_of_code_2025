use partser::{Parseable, Repeatable};
use partser::prelude::*;
use partser::number::i64;

#[derive(Default, Debug, Copy, Clone)]
enum Operation {
    #[default]
    Add,
    Multiply,
}

impl Operation {
    pub fn apply(&self, x: i64, y: i64) -> i64 {
        match self {
            Operation::Add => x + y,
            Operation::Multiply => x * y,
        }
    }
}

#[derive(Default, Debug)]
struct Problem(Vec<i64>, Operation);

fn parse_a(input: &str) -> Vec<Problem> {
    let mut lines = input.lines().rev();
    let operations = lines.next().unwrap();
    let ops = operations.parse_with(false, "*+".any().map_ok(|c| if c == '*' { Operation::Multiply } else { Operation::Add }).rep_separated(' '.rep(1.., true), .., true)).unwrap();
    let mut problems = Vec::with_capacity(ops.len());
    for op in ops {
        problems.push(Problem(Default::default(), op));
    }
    for lnums in lines.rev() {
        let nums = lnums.parse_with(true, i64.rep_separated(' '.rep(1.., true), .., true)).unwrap();
        for (i, num) in nums.iter().enumerate() {
            problems[i].0.push(*num);
        }
    }
    problems
}

fn parse_b(input: &str) -> Vec<Problem> {
    let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().rev().collect()).collect();
    let mut problem = Problem(Vec::with_capacity(input.len()), Operation::Add);
    let mut problems = Vec::new();
    for x in 0..input[0].len() {
        let mut val = 0;
        let mut end = false;
        for y in 0..input.len() {
            match input[y][x] {
                o @ '*' | o @ '+' => {
                    end = true;
                    problem.1 = if o == '*' { Operation::Multiply } else { Operation::Add };
                }
                c @ '0'..='9' => {
                    val = val * 10 + c as i64 - '0' as i64;
                }
                _ => {}
            }
        }
        if val != 0 {
            problem.0.push(val);
        }
        if end {
            problems.push(problem);
            problem = Problem(Vec::new(), Operation::Add);
        }
    }
    problems
}

fn a(input: Vec<Problem>) -> i64 {
    let mut acc = 0;
    for Problem(nums, op) in &input {
        let op = *op;
        let res = nums.iter().copied().reduce(|acc, num| op.apply(acc, num)).unwrap();
        acc += res;
    }
    acc
}

pub(crate) fn day06() {
    // let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
    let input = include_str!("day06.txt");
    let input_a = parse_a(input);
    let input_b = parse_b(input);
    println!("\nDay 06");
    println!("a: {}", a(input_a));
    println!("b: {}", a(input_b));
}