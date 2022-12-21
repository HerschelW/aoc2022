// use Lines
use std::str::Lines;

fn main() {
    // read input file
    let input = include_str!("input.txt");
    // split input into lines
    let lines = input.lines();

    // solve day 4 part 1
    solve_day_4_part_1(lines);
}

// create part 1 function with Lines as parameter
fn solve_day_4_part_1(lines: Lines) {
    // print lines
    for line in lines {
        println!("{}", line);
    }
}
