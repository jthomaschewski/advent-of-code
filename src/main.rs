use aoc2022::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// execute only specified days solution
    #[clap(short, long)]
    day: Option<usize>,

    /// execute only specified part of selected day(s)
    #[clap(short, long)]
    part: Option<u8>,
}

fn main() {
    let args = Args::parse();
    let parts_to_run = Part::from_arg(args.part);

    let mut days_to_run = Vec::from([
        day01::run,
        day02::run,
        day03::run,
        day04::run,
        day05::run,
        day06::run,
    ]);

    if let Some(day) = args.day {
        if day > days_to_run.len() {
            println!("Day {} not implemented yet", day);
            std::process::exit(1);
        }
        days_to_run = vec![days_to_run[day - 1]]
    }

    for (day, run) in days_to_run.iter().enumerate() {
        println!("\nSolution day {}", day + 1);

        for part in &parts_to_run {
            println!("Part {part}: {}", run(*part));
        }
    }
}
