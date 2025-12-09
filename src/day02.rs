use partser::multi::rep_separated;
use partser::prelude::*;
use partser::number::u64;

fn parse(input: &str) -> Vec<(u64, u64)> {
    input.parse_with(true, rep_separated(.., true, (u64, '-', u64).map_ok(|(start, _, finish)| (start, finish)), ',')).unwrap()
}

//somewhat slow (21898734247)
fn a(ranges: &[(u64, u64)]) -> u64 {
    let mut count = 0;
    for range in ranges {
        'range: for id in range.0..=range.1 {
            let size = (id.ilog10() + 1) as usize; //should be slightly faster than converting id to string (skip allocation)
            if size & 1 == 0 { //only test numbers that have a pair number of digits
                let repr = id.to_string().chars().collect::<Vec<char>>();
                let split = size / 2;
                for i in 0..split {
                    if repr[i] != repr[i + split] {
                        continue 'range;
                    }
                }
                count += id;
            }
        }
    }
    count
}

//very costly but correct (28915664389)
fn b(ranges: &[(u64, u64)]) -> u64 {
    let mut count = 0;
    for range in ranges {
        'range: for id in range.0..=range.1 {
            let id_str = id.to_string();
            for p in 1..id_str.len() {
                let stub = &id_str[0..p];
                let rem = &id_str[p..];
                if rem.parse_with(true, stub.rep(1.., true)).is_ok() {
                    count += id;
                    continue 'range;
                }
            }
        }
    }
    count
}

pub(crate) fn day02() {
    // let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let input = include_str!("day02.txt");
    let ranges = parse(input);
    println!("\nDay 02");
    println!("a: {}", /*a(&ranges)*/21898734247u64);
    println!("b: {}", /*b(&ranges)*/28915664389u64);
}