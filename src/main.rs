use std::env;
mod aoc;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_str = if args.len() > 1 {&args[1]} else {""};
    let example = args.len() > 2 && (args[2] == "-e" || args[2] == "--example");
    match day_str.parse::<i32>() {
        Ok(day) => {
            aoc::day(day, example);
        },
        Err(_) => {
            println!("Please specify day as the first argument");
        }  
    };

}
