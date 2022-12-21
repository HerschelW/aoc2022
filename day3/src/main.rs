fn main() {
    solve_day_3_part_1();
    solve_day_3_part_2();
}

// find_priorities function that takes matching_item vector as parameter
fn find_priorities(matching_items: Vec<char>) {
    // assign values 1 through 26 to each lowercase letter of the alphabet
    let mut alphabet: Vec<char> = Vec::new();
    for i in 1..27 {
        alphabet.push((i + 96) as u8 as char);
    }
    // assign values 27 through 52 to each uppercase letter of the alphabet
    for i in 1..27 {
        alphabet.push((i + 64) as u8 as char);
    }

    // find value for matching items
    let mut matching_values: Vec<i32> = Vec::new();
    for item in matching_items {
        for j in 0..alphabet.len() {
            if item == alphabet[j] {
                matching_values.push(j as i32 + 1);
            }
        }
    }
    // print matching values
    println!("{:?}", matching_values);

    // find sum of matching values
    let mut sum = 0;
    for value in matching_values {
        sum += value;
    }

    // print sum
    println!("{}", sum);
}

fn solve_day_3_part_1() {
    // parse input.txt
    let input = include_str!("part2_sample_input.txt");
    let lines = input.lines();

    // Part 1
    let mut matching_items: Vec<char> = Vec::new();
    for line in lines {
        // split line into two halves
        let (left, right) = line.split_at(line.len() / 2);

        // remove duplicates in left and right
        let left: String = left.chars().collect();
        let right: String = right.chars().collect();

        // turn to vectors
        let mut left: Vec<char> = left.chars().collect();
        let mut right: Vec<char> = right.chars().collect();

        left.sort();
        left.dedup();
        right.sort();
        right.dedup();

        // find matching items
        for item in left {
            if right.contains(&item) {
                matching_items.push(item);
            }
        }
    }

    find_priorities(matching_items);
}

fn solve_day_3_part_2() {
    let input = include_str!("part2_sample_input.txt");
    // print input length
    println!("{}", input.len());
    let lines = input.lines();
    // get lines length
    let mut lines_length = 0;
    for _ in lines {
        lines_length += 1;
    }
    let mut matching_items: Vec<char> = Vec::new();
    let mut lines = input.lines();
    // convert lines to vector
    let mut lines: Vec<&str> = lines.collect();
    // print lines
    println!("{:?}", lines);

    for i in (0..lines_length - 1).step_by(3) {
        // get lines
        let line1 = lines[i];
        let line2 = lines[i + 1];
        let line3 = lines[i + 2];

        // turn lines into vectors
        let mut line1: Vec<char> = line1.chars().collect();
        let mut line2: Vec<char> = line2.chars().collect();
        let mut line3: Vec<char> = line3.chars().collect();

        // print line1
        println!("{:?}", line1);

        // sort and remove duplicates
        line1.sort();
        line1.dedup();
        line2.sort();
        line2.dedup();
        line3.sort();
        line3.dedup();

        // find matching items
        for item in line1 {
            if line2.contains(&item) && line3.contains(&item) {
                matching_items.push(item);
            }
        }
    }
    find_priorities(matching_items);
}
