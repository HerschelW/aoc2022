// import find priorities
mod find_priorities;

use find_priorities::find_priorities;

fn main() {
    solve_day_3_part_1();
    solve_day_3_part_2();
}

// fn find_priorities(matching_items: Vec<char>) -> i32 {
//     let mut alphabet: Vec<char> = Vec::new();

//     for i in 1..27 {
//         alphabet.push((i + 96) as u8 as char);
//     }

//     for i in 1..27 {
//         alphabet.push((i + 64) as u8 as char);
//     }

//     let mut matching_values: Vec<i32> = Vec::new();
//     for item in matching_items {
//         for j in 0..alphabet.len() {
//             if item == alphabet[j] {
//                 matching_values.push(j as i32 + 1);
//             }
//         }
//     }

//     let mut sum = 0;
//     for value in matching_values {
//         sum += value;
//     }

//     sum
// }

fn solve_day_3_part_1() {
    let input = include_str!("input.txt");
    let lines = input.lines();
    let mut matching_items: Vec<char> = Vec::new();

    for line in lines {
        let (left, right) = line.split_at(line.len() / 2);
        let mut left: Vec<char> = left.chars().collect();
        let mut right: Vec<char> = right.chars().collect();

        left.sort();
        left.dedup();
        right.sort();
        right.dedup();

        for item in left {
            if right.contains(&item) {
                matching_items.push(item);
            }
        }
    }

    let part_1_sum = find_priorities(matching_items);
    println!("Part 1 Sum is {}", part_1_sum);
}

fn solve_day_3_part_2() {
    let input = include_str!("input.txt");
    let lines = input.lines();
    let mut lines_length = 0;

    for _ in lines {
        lines_length += 1;
    }

    let mut matching_items: Vec<char> = Vec::new();
    let lines = input.lines();
    let lines: Vec<&str> = lines.collect();

    for i in (0..lines_length - 1).step_by(3) {
        let line1 = lines[i];
        let line2 = lines[i + 1];
        let line3 = lines[i + 2];

        let mut line1: Vec<char> = line1.chars().collect();
        let mut line2: Vec<char> = line2.chars().collect();
        let mut line3: Vec<char> = line3.chars().collect();

        line1.sort();
        line1.dedup();
        line2.sort();
        line2.dedup();
        line3.sort();
        line3.dedup();

        for item in line1 {
            if line2.contains(&item) && line3.contains(&item) {
                matching_items.push(item);
            }
        }
    }

    let part_2_sum = find_priorities(matching_items);
    println!("Part 2 Sum is {}", part_2_sum);
}
