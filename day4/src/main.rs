fn main() {
    solve_day_4_part_1()
}

fn solve_day_4_part_1() {
    let input = include_str!("sample_input.txt");

    // get assignment ranges
    let ranges: Vec<&str> = input.split("\n").collect();

    // create contains variable
    let mut contains = 0;

    // create partial overlaps variable
    let mut partial_overlaps = 0;

    // loop through ranges
    for range in ranges {
        // split range into two assignments
        let assignments: Vec<&str> = range.split(",").collect();

        // split assignments into two ranges
        let a1: Vec<i32> = assignments[0]
            .split("-")
            .map(|x| x.parse().expect("Error parsing range"))
            .collect();

        let a2: Vec<i32> = assignments[1]
            .split("-")
            .map(|x| x.parse().expect("Error parsing range"))
            .collect();

        // check if a1 contains a2
        if (a1[0] <= a2[0]) && (a1[1] >= a2[1]) {
            contains += 1;
        } else if (a2[0] <= a1[0]) && (a2[1] >= a1[1]) {
            contains += 1;
        }

        // check if a1 and a2 partially overlap
        if (a1[0] <= a2[1]) && (a1[1] >= a2[0]) {
            partial_overlaps += 1;
        }
    }

    // print contains
    println!("{}", contains);

    // print partial overlaps
    println!("{}", partial_overlaps);
}
