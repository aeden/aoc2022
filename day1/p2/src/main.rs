use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let bin_name = &args[0];
        println!("Usage: {bin_name} file_path");
        return;
    }
    let file_path = &args[1];

    // a vector of all elves
    let mut elves = Vec::new();

    // the counter of calotires for the current elf
    let mut current = 0;

    // fill elves with summed calorie counts
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

    // sort the elves (lowest to highest)
    elves.sort();

    // reverse the elves (highest to lowest)
    let mut elves_rev: Vec<_> = elves.into_iter().rev().collect();

    // split off the top 3 elves and throw away rest
    let _ = elves_rev.split_off(3);

    // sum the total of the top 3 elves
    let sum: i32 = elves_rev.iter().sum();

    println!("Sum of top 3: {sum}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
