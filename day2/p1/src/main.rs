use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

const ROCK: i16 = 1;
const PAPER: i16 = 2;
const SCISSORS: i16 = 3;

#[derive(Debug, Clone)]
struct ParseError;

#[derive(Debug)]
struct RoundStrategy {
    them: i16,
    us: i16,
}

impl FromStr for RoundStrategy {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut letters = s.split_whitespace();
        let them = match letters.next().unwrap() {
            "A" => ROCK,
            "B" => PAPER,
            "C" => SCISSORS,
            _ => todo!(),
        };

        let us = match letters.next().unwrap() {
            "X" => ROCK,
            "Y" => PAPER,
            "Z" => SCISSORS,
            _ => todo!(),
        };

        Ok(RoundStrategy { them, us })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let bin_name = &args[0];

    let args_length = args.len();
    if args_length < 2 {
        println!("Usage: {bin_name} file_path");
        return;
    }
    println!("Args length {args_length}");
    let file_path = &args[1];

    println!("In file {}", file_path);

    if let Ok(lines) = read_lines(file_path) {
        let mut total = 0;
        let mut line_count = 0;
        for line in lines {
            line_count += 1;
            if let Ok(match_strategy) = line {
                // println!("{}", match_strategy);
                let rs = match RoundStrategy::from_str(&match_strategy) {
                    Ok(rs) => rs,
                    Err(_) => todo!(),
                };

                if (rs.them == ROCK && rs.us == SCISSORS)
                    || (rs.them == PAPER && rs.us == ROCK)
                    || (rs.them == SCISSORS && rs.us == PAPER)
                {
                    // println!("They won, add {}", rs.us);
                } else if rs.them == rs.us {
                    // println!("Tie, add {}", rs.us + 3);
                    total += 3;
                } else {
                    // println!("We won, add {}", rs.us + 6);
                    total += 6
                }

                total += rs.us;

                // println!("Running total: {total}");
            }
        }
        println!("Processed {line_count} lines");
        println!("Total: {total}");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
