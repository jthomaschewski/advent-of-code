use aoc2022::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    day: u8,

    /// part to execute (0 = all, 1/2 = specified part)
    #[clap(short, long, default_value_t = 0)]
    part: u8,
}

fn main() {
    let args = Args::parse();

    let day: u8;
    let part: u8;
    Args { day, part } = args;

    let run = match day {
        1 => day01::run,
        2 => day02::run,
        3 => day03::run,
        _ => {
            println!("Day {} not implemented yet", day);
            std::process::exit(1);
        }
    };

    let parts = match part {
        1 => vec![Part::One],
        2 => vec![Part::Two],
        _ => vec![Part::One, Part::Two],
    };

    println!("\nSolution day {day}");
    for part in parts {
        println!("Part {part}: {}", run(part));
    }
}
