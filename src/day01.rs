use partser::prelude::*;
use partser::number::i32;

fn instruction(reader: StringReader) -> ParserOut<i32> {
    (("LR".any().map_ok(|c| if c == 'L' { -1 } else { 1 }), i32).map_ok(|(m, v)| m * v))(reader)
}

fn parse(input: &str) -> Vec<i32> {
    let mut out = Vec::new();
    for line in input.lines() {
        out.push(line.parse_with(true, instruction).unwrap());
    }
    out
}

fn a(mut cursor: i32, instructions: &[i32], modulus: i32) -> usize {
    let mut count = 0;
    for instruction in instructions {
        cursor += instruction;
        while cursor < 0 {
            cursor += modulus;
        }
        cursor %= modulus;
        if cursor == 0 {
            count += 1;
        }
    }
    count
}

fn b(mut cursor: i32, instructions: &[i32], modulus: i32) -> usize {
    let mut count = 0;
    for instruction in instructions {
        let direction = instruction.signum();
        let mut steps = instruction.abs();
        for _ in 0..steps {
            cursor += direction;
            if cursor < 0 {
                cursor += modulus;
            }
            if cursor >= modulus {
                cursor -= modulus;
            }
            if cursor == 0 {
                count += 1;
            }
        }
    }
    count
}

pub(crate) fn day01() {
    // let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
    let input = include_str!("day01.txt");
    let instructions = parse(input);
    println!("\nDay 01");
    println!("a: {}", a(50, &instructions, 100));
    println!("b: {}", b(50, &instructions, 100));
}