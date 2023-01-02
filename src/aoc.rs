mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
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
    days.insert(1, (day1::first, day1::second));
    days.insert(2, (day2::first, day2::second));
    days.insert(3, (day3::first, day3::second));
    days.insert(4, (day4::first, day4::second));
    days.insert(5, (day5::first, day5::second));
    days.insert(6, (day6::first, day6::second));
    days.insert(7, (day7::first, day7::second));
    days.insert(8, (day8::first, day8::second));
    days.insert(9, (day9::first, day9::second));
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