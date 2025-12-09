fn a(mut input: Vec<Vec<char>>) -> usize {
    let mut splits = 0;
    for y in 0..input.len() - 1 {
        for x in 0..input[y].len() {
            if input[y][x] == '|' {
                if input[y + 1][x] == '^' {
                    if let Some(t) = input[y + 1].get_mut(x - 1) {
                        *t = '|';
                    }
                    if let Some(t) = input[y + 1].get_mut(x + 1) {
                        *t = '|';
                    }
                    splits += 1;
                } else {
                    input[y + 1][x] = '|';
                }
            }
        }
    }
    splits
}

fn b(mut input: Vec<Vec<char>>) -> usize {
    let mut paths = vec![0; input[0].len()];
    let mut prev = Vec::new();
    for y in 0..input.len() - 1 {
        for x in 0..input[y].len() {
            if input[y][x] == '|' {
                let path = if y == 0 {
                    1
                } else {
                    prev[x]
                };
                if input[y + 1][x] == '^' {
                    if let Some(t) = input[y + 1].get_mut(x - 1) {
                        *t = '|';
                        paths[x - 1] += path;
                    }
                    if let Some(t) = input[y + 1].get_mut(x + 1) {
                        *t = '|';
                        paths[x + 1] += path;
                    }
                } else {
                    input[y + 1][x] = '|';
                    paths[x] += path;
                }
            }
        }
        prev = paths;
        paths = vec![0; input[0].len()];
    }
    prev.iter().copied().reduce(|acc, v| acc + v).unwrap()
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().map(|c| if c == 'S' { '|' } else { c }).collect()).collect()
}

pub(crate) fn day07() {
    // let input = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............";
    let input = include_str!("day07.txt");
    let input = parse(input);
    println!("\nDay 07");
    println!("a: {}", a(input.clone()));
    println!("b: {}", b(input));
}

/*
had to visually test it first
.......S.......
.......1.......
......1^1......
......1.1......
.....1^2^1.....
.....1.2.1.....
....1^3^3^1....
....1.3.3.1....
...1^4^331^1...
...1.4.331.1...
..1^5^434^2^1..
..1.5.434.2.1..
.1^154^74.21^1.
.1.154.74.21.1.
1^2^A^B^B^211^1
1.2.A.B.B.211.1

1+2+10+11+11+2+1+1+1 = 40

seem's like after 2 tries (including a recursive one that results in 30min+ of calculation), I finally got it in iterative that is so simple that it can be done manually
 */