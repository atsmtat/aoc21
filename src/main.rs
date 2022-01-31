use clap::{App, Arg};
use std::fs::File;
use std::io::{BufRead, BufReader};

mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("aoc21")
        .version("1.0")
        .author("Smit Soni")
        .about("Advent of Code 2021")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .takes_value(true)
                .required(true)
                .help("Day of the calendar between 1-8"),
        )
        .arg(
            Arg::with_name("part")
                .short("p")
                .long("part")
                .takes_value(true)
                .required(true)
                .help("Which part of the puzzle, 1 or 2"),
        )
        .arg(
            Arg::with_name("INPUT")
                .index(1)
                .help("File with puzzle input"),
        )
        .get_matches();

    let day = matches
        .value_of("day")
        .map_or(1, |d| d.parse::<u8>().unwrap());
    let part = matches
        .value_of("part")
        .map_or(1, |d| d.parse::<u8>().unwrap());

    match part {
        1 | 2 => {}
        _ => {
            clap::Error::with_description(
                "Invalid part, must be 1 or 2",
                clap::ErrorKind::InvalidValue,
            )
            .exit();
        }
    }

    let def_file_name = format!("data/day{}.txt", day);
    let file_name = matches.value_of("INPUT").unwrap_or(&def_file_name);

    let file = File::open(file_name)?;
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());

    match day {
        1 => day1::solve(lines, part),
        2 => day2::solve(lines, part),
        3 => day3::solve(lines, part),
        4 => day4::solve(lines, part),
        5 => day5::solve(lines, part),
        6 => day6::solve(lines, part),
        7 => day7::solve(lines, part),
        8 => day8::solve(lines, part),
        9 => day9::solve(lines, part),
        10 => day10::solve(lines, part),
        11 => day11::solve(lines, part),
        _ => {
            println!("Unsolved");
        }
    }

    Ok(())
}
