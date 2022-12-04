use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

const ROCK: i16 = 1;
const PAPER: i16 = 2;
const SCISSORS: i16 = 3;

const WIN: i16 = 1;
const TIE: i16 = 2;
const LOSE: i16 = 3;

#[derive(Debug, Clone)]
struct ParseError;

#[derive(Debug)]
struct RoundStrategy {
    them: i16,
    end: i16,
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

        let end = match letters.next().unwrap() {
            "X" => WIN,
            "Y" => TIE,
            "Z" => LOSE,
            _ => todo!(),
        };

        Ok(RoundStrategy { them, end })
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

                if rs.end == WIN {
                    // println!("They they need to win");
                    if rs.them == ROCK {
                        total += SCISSORS;
                    } else if rs.them == PAPER {
                        total += ROCK;
                    } else {
                        total += PAPER;
                    }
                } else if rs.end == TIE {
                    // println!("Tie");
                    total += rs.them + 3;
                } else {
                    // println!("We need to win");
                    if rs.them == ROCK {
                        total += PAPER;
                    } else if rs.them == PAPER {
                        total += SCISSORS;
                    } else {
                        total += ROCK;
                    }
                    total += 6;
                }

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
