#![feature(generic_const_exprs)]

pub mod vec3;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

use partser::{ParserOut, StringReader};
use partser::multi::take_while;

use crate::day01::day01;
use crate::day02::day02;
use crate::day03::day03;
use crate::day04::day04;
use crate::day05::day05;
use crate::day06::day06;
use crate::day07::day07;
use crate::day08::day08;
use crate::day09::day09;
use crate::day10::day10;
use crate::day11::day11;
use crate::day12::day12;

pub fn ident(reader: StringReader) -> ParserOut<String> {
    take_while(|c| c.is_alphabetic())(reader)
}

fn main() {
    // day01();
    // day02();
    // day03();
    // day04();
    // day05();
    // day06();
    // day07();
    // day08();
    // day09();
    day10();
    day11();
    day12();
}