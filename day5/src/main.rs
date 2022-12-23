// use fs
use std::fs;

fn main() {
    // import input using fs
    let input = fs::read_to_string("sample_input.txt").expect("Error reading input file");

    // solve day 5
    solve_day_5(input);
}

fn solve_day_5(input: String) {
    // split into two lines at \n\n
    let mut lines = input.split("\n\n");
    // get first line
    let crates = lines.next().unwrap();
    // get second line
    let instructions = lines.next().unwrap();

    // print lines
    println!("Line 1: {}", crates);
    println!("Line 2: {}", instructions);
}
