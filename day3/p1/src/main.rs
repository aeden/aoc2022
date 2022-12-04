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

    if let Ok(lines) = read_lines(file_path) {
        let mut line_count = 0;
        let mut total = 0;
        for line in lines {
            line_count += 1;
            if let Ok(contents) = line {
                let (left, right) = contents.split_at(contents.len() / 2);
                total += priority(left, right, line_count);
            }
        }
        println!("Processed {line_count} lines");
        println!("Total: {total}");
    }
}

fn priority(left: &str, right: &str, _line_number: i32) -> u128 {
    for cl in left.chars() {
        for cr in right.chars() {
            if cl == cr {
                // println!("line {}: {} == {}, add {}", line_number, cl, cr, u128::from(cl) - get_offset(cl));
                return u128::from(cl) - get_offset(cl);
            }
        }
    }
    panic!("Did not find common item between the two containers");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_offset(c: char) -> u128 {
    if c.is_ascii_lowercase() {
        return 96;
    } else if c.is_ascii_uppercase() {
        return 38;
    } else {
        return 0; // TODO: error case
    }
}