use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

    let mut elves = Vec::new();
    let mut current = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(calories) = line {
                if calories.is_empty() {
                    elves.push(current);
                    current = 0;
                } else {
                    let parsed: i32 = calories.parse().unwrap();
                    current += parsed;
                }
            }
        }
    }

    elves.sort();

    dbg!(&elves);

    let largest = match elves.pop() {
        Some(x) => x,
        None => 0,
    };

    println!("Largest value: {}", largest);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
