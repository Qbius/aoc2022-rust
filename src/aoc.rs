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
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use std::collections::HashMap;
use std::fs;
type AocF = fn(&String) -> Option<String>;

pub fn day(n: u8, example: bool) {
    let mut days = HashMap::<u8, (AocF, AocF)>::new();
    days.insert(1, (day01::first, day01::second));
    days.insert(2, (day02::first, day02::second));
    days.insert(3, (day03::first, day03::second));
    days.insert(4, (day04::first, day04::second));
    days.insert(5, (day05::first, day05::second));
    days.insert(6, (day06::first, day06::second));
    days.insert(7, (day07::first, day07::second));
    days.insert(8, (day08::first, day08::second));
    days.insert(9, (day09::first, day09::second));
    days.insert(10, (day10::first, day10::second));
    days.insert(11, (day11::first, day11::second));
    days.insert(12, (day12::first, day12::second));
    days.insert(13, (day13::first, day13::second));
    days.insert(14, (day14::first, day14::second));
    days.insert(15, (day15::first, day15::second));
    days.insert(16, (day16::first, day16::second));
    days.insert(17, (day17::first, day17::second));
    days.insert(18, (day18::first, day18::second));
    days.insert(19, (day19::first, day19::second));
    days.insert(20, (day20::first, day20::second));
    days.insert(21, (day21::first, day21::second));
    days.insert(22, (day22::first, day22::second));
    days.insert(23, (day23::first, day23::second));
    days.insert(24, (day24::first, day24::second));
    days.insert(25, (day25::first, day25::second));

    match days.get(&n) {
        Some((first, second)) => {
            let input_directory = if example {"examples"} else {"inputs"};
            let input = str::replace(&fs::read_to_string(format!("{input_directory}/{n}.txt")).unwrap(), "\r\n", "\n");
            println!("Day {}", n);
            println!("First: {}", match first(&input) {Some(v) => v, None => "None".to_string()});
            println!("Second: {}", match second(&input) {Some(v) => v, None => "None".to_string()});
        }
        None => {
            println!("wtf");
        }
    };
}