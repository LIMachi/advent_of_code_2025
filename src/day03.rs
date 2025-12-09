fn parse(input: &str) -> Vec<Vec<usize>> {
    let mut out: Vec<Vec<usize>> = Vec::new();
    for line in input.lines() {
        out.push(Vec::new());
        let row = out.last_mut().unwrap();
        for col in line.chars() {
            if col >= '0' && col <= '9' {
                row.push(col as usize - '0' as usize);
            }
        }
    }
    out
}

fn a(batteries: &Vec<Vec<usize>>) -> usize {
    let mut acc = 0;
    for row in batteries {
        let mut best = 0;
        for i in 0..row.len() - 1 {
            for j in i + 1..row.len() {
                best = best.max(row[i] * 10 + row[j]);
            }
        }
        acc += best;
    }
    acc
}

//even if it is leagues faster (than iterative version) in the examples (and exact), it is still slow for our input (took around 30s on my old computer)
fn b_rec(current: usize, depth: usize, batteries: &[usize]) -> usize {
    if depth == 12 {
        current
    } else if depth + batteries.len() < 12 {
        0
    } else {
        let mut best_digit = 0;
        let mut best = 0;
        for i in 0..batteries.len() - (12 - depth) + 1 {
            if batteries[i] > best_digit {
                let t = b_rec(current * 10 + batteries[i], depth + 1, &batteries[i + 1..]);
                if t > best {
                    best_digit = batteries[i];
                    best = t;
                }
            }
        }
        best
    }
}

fn b(batteries: &Vec<Vec<usize>>) -> usize {
    let mut acc = 0;
    for row in batteries {
        acc += b_rec(0, 0, row);
    }
    acc
}

pub(crate) fn day03() {
    // let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
    let input = include_str!("day03.txt");
    let batteries = parse(input);
    println!("\nDay 03");
    println!("a: {}", a(&batteries));
    println!("b: {}", /*b(&batteries)*/176582889354075usize);
}