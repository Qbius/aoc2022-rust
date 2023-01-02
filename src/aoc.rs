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
type VoidFnPtr = Box<dyn Fn(&String, bool) -> Option<String>>;

pub fn day(n: u8, example: bool) {
    let mut days = HashMap::<u8, (VoidFnPtr, VoidFnPtr)>::new();
    days.insert(1, (Box::new(day1::first), Box::new(day1::second)));
    days.insert(2, (Box::new(day2::first), Box::new(day2::second)));
    days.insert(3, (Box::new(day3::first), Box::new(day3::second)));
    days.insert(4, (Box::new(day4::first), Box::new(day4::second)));
    days.insert(5, (Box::new(day5::first), Box::new(day5::second)));
    days.insert(6, (Box::new(day6::first), Box::new(day6::second)));
    days.insert(7, (Box::new(day7::first), Box::new(day7::second)));
    days.insert(8, (Box::new(day8::first), Box::new(day8::second)));
    days.insert(9, (Box::new(day9::first), Box::new(day9::second)));
    days.insert(10, (Box::new(day10::first), Box::new(day10::second)));
    days.insert(11, (Box::new(day11::first), Box::new(day11::second)));
    days.insert(12, (Box::new(day12::first), Box::new(day12::second)));
    days.insert(13, (Box::new(day13::first), Box::new(day13::second)));
    days.insert(14, (Box::new(day14::first), Box::new(day14::second)));
    days.insert(15, (Box::new(day15::first), Box::new(day15::second)));
    days.insert(16, (Box::new(day16::first), Box::new(day16::second)));
    days.insert(17, (Box::new(day17::first), Box::new(day17::second)));
    days.insert(18, (Box::new(day18::first), Box::new(day18::second)));
    days.insert(19, (Box::new(day19::first), Box::new(day19::second)));
    days.insert(20, (Box::new(day20::first), Box::new(day20::second)));
    days.insert(21, (Box::new(day21::first), Box::new(day21::second)));
    days.insert(22, (Box::new(day22::first), Box::new(day22::second)));
    days.insert(23, (Box::new(day23::first), Box::new(day23::second)));
    days.insert(24, (Box::new(day24::first), Box::new(day24::second)));
    days.insert(25, (Box::new(day25::first), Box::new(day25::second)));

    match days.get(&n) {
        Some((first, second)) => {
            let input = str::replace(&fs::read_to_string(format!("inputs/{n}.txt")).unwrap(), "\r\n", "\n");
            println!("Day {}", n);
            println!("First: {}", match first(&input, example) {Some(v) => v, None => "None".to_string()});
            println!("Second: {}", match second(&input, example) {Some(v) => v, None => "None".to_string()});
        }
        None => {
            println!("wtf");
        }
    };
}