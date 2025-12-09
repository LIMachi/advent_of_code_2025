fn parse(input: &str) -> Vec<Vec<bool>> {
    let mut out: Vec<Vec<bool>> = Vec::new();
    for line in input.lines() {
        out.push(Vec::new());
        let row = out.last_mut().unwrap();
        for col in line.chars() {
            if col == '@' || col == '.' {
                row.push(col == '@');
            }
        }
    }
    out
}

fn has_roll(map: &[Vec<bool>], x: isize, y: isize) -> bool {
    if x < 0 || y < 0 {
        false
    } else {
        *map.get(y as usize).and_then(|l| l.get(x as usize)).unwrap_or(&false)
    }
}

fn a(map: &Vec<Vec<bool>>) -> usize {
    let mut count = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] {
                let mut adjacent = 0;
                for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1), (1, 1), (-1, 1), (-1, -1), (1, -1)] {
                    let tx = x as isize + dx;
                    let ty = y as isize + dy;
                    if has_roll(&map, tx, ty) {
                        adjacent += 1;
                    }
                }
                if adjacent < 4 {
                    count += 1;
                }
            }
        }
    }
    count
}

fn b(map: &mut Vec<Vec<bool>>) -> usize {
    let mut count = 0;
    let mut run = true;
    while run {
        run = false;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] {
                    let mut adjacent = 0;
                    for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1), (1, 1), (-1, 1), (-1, -1), (1, -1)] {
                        let tx = x as isize + dx;
                        let ty = y as isize + dy;
                        if has_roll(&map, tx, ty) {
                            adjacent += 1;
                        }
                    }
                    if adjacent < 4 {
                        run = true;
                        map[y][x] = false;
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

pub(crate) fn day04() {
    // let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
    let input = include_str!("day04.txt");
    let mut map = parse(input);
    println!("\nDay 04");
    println!("a: {}", a(&map));
    println!("b: {}", b(&mut map));
}