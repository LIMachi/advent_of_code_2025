use partser::prelude::*;

fn parse(input: &str) -> Vec<(isize, isize)> {
    let mut out = Vec::new();
    for line in input.lines() {
        out.push(line.parse_with(true, swizzle!((isize, ',', isize); 0, 2)).unwrap());
    }
    out
}

//~500 inputs, seems like bruteforce should be enough
fn a(input: &[(isize, isize)]) -> usize {
    let mut best = 0;
    for a in 0..input.len() - 1 {
        for b in a + 1..input.len() {
            let dx = input[a].0.abs_diff(input[b].0) + 1;
            let dy = input[a].1.abs_diff(input[b].1) + 1;
            let area = dx * dy;
            if area > best {
                // println!("best: {area} > {best} ({a}: {} {} * {b} : {} {})", input[a].0, input[a].1, input[b].0, input[b].1);
                best = area;
            }
        }
    }
    best
}

//ok, so now we have to test using the shape produced by those points:
/* ex:
..............
.......#XXX#..
.......XXXXX..
..#XXXX#XXXX..
..XXXXXXXXXX..
..#XXXXXX#XX..
.........XXX..
.........#X#..
..............
 */

//the issue is to guess when a point is inside or outside the shape. the shape is so big that using pseudo image processing, it would take 8.6 terrabites (I calculated about 9285035856 bytes based on the minimum and maximum on the x and y axis), so we need another way
//another (costly) solution for a single point would be to use a ray cast technique where we count the number of lines that intersect the vector 0,0 to our point (the number of intersection would give use the number of times we entered/leaved the shape and ultimately if we are inside or out)
//but this would be too costly
/* let's try something
..............
.......#XXX#..
.......XXXXX..
..#XXXX#XX##..
..XXXXXXXXX...
..#XXXXXX###..
.........XXX..
.........#X#..
..............
 */
//new idea: we test all the possible rectangles (sorted by biggest area so we stop once we have a valid one instead of checking them all), which is just a variation of part a
//and instead of checking each point in the rectangle we can test if there is an invalid vertex inside the rectangle (should be as easy as testing if a point is strictly inside the rectangle, as points on the border would not intersect the shape)

fn intersect(aabb: ((isize, isize), (isize, isize)), (start, end): ((isize, isize), (isize, isize))) -> bool {
    let left = start.0.max(end.0) <= aabb.0.0.min(aabb.1.0);
    let right = start.0.min(end.0) >= aabb.0.0.max(aabb.1.0);
    let top = start.1.max(end.1) <= aabb.0.1.min(aabb.1.1);
    let bottom = start.1.min(end.1) >= aabb.0.1.max(aabb.1.1);
    !(left || right || top || bottom)
}

//1396494456
fn b(input: &[(isize, isize)]) -> usize {
    let mut rectangles: Vec<((isize, isize), (isize, isize), usize)> = Vec::new();
    let mut lines: Vec<((isize, isize), (isize, isize))> = Vec::new();
    for a in 0..input.len() - 1 {
        for b in a + 1..input.len() {
            let a = input[a];
            let b = input[b];
            let dx = a.0.abs_diff(b.0) + 1;
            let dy = a.1.abs_diff(b.1) + 1;
            rectangles.push((a, b, dx * dy));
        }
    }
    rectangles.sort_by(|a, b| {
        b.2.cmp(&a.2)
    });
    // dbg!(rectangles[0]); //should be the same output as part A
    for i in 0..input.len() {
        let a = input[i];
        let b = input[if i + 1 < input.len() { i + 1 } else { 0 }];
        lines.push((a, b));
    }
    'main: for &(c1, c2, area) in &rectangles {
        for &line in &lines {
            if intersect((c1, c2), line) {
                continue 'main;
            }
        }
        return area;
    }
    0
}

pub(crate) fn day09() {
    // let input = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";
    let input = include_str!("day09.txt");
    let input = parse(input);
    println!("\nDay 09");
    println!("a: {}", a(&input));
    println!("b: {}", b(&input));
}