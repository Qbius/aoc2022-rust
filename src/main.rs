mod aoc;

use clap::Parser;

/// aoc2022 runner
#[derive(Parser)]
struct Args {
   /// Which aoc2022 day to run, from 1 to 25
   day: u8,

   /// If set, run the day's example
   #[arg(short, long, action)]
   example: bool,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.example);
    aoc::day(args.day, args.example);
}
