// use Lines
use std::str::Lines;

fn main() {
    // read input file
    let input = include_str!("sample_input.txt");
    // split input into lines
    let lines = input.lines();

    // solve day 4 part 1
    solve_day_4_part_1(lines);
}

// create part 1 function with Lines as parameter
fn solve_day_4_part_1(lines: Lines) {
    // vector of vectors
    let mut list_of_lists = Vec::new();
    for line in lines {
        // split lines at comma
        let split_line = line.split(",");
        for split in split_line {
            // populate numerical list, first value is beginning, last value is ending
            let numerical_list: Vec<i32> = split.split("-").map(|s| s.parse().unwrap()).collect();
            // make new list that spreads numerical list[0]...numerical_list[1]
            let mut new_list = Vec::new();
            for i in numerical_list[0]..numerical_list[1] + 1 {
                new_list.push(i);
            }
            // print new list
            println!("{:?}", new_list);
            // push new list to list of lists
            list_of_lists.push(new_list);
        }
    }
    // print list of lists
    println!("{:?}", list_of_lists);
}
