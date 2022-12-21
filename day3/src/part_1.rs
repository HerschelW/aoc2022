// // use crate

// pub fn solve_day_3_part_1() {
//     let input = include_str!("input.txt");
//     let lines = input.lines();
//     let mut matching_items: Vec<char> = Vec::new();

//     for line in lines {
//         let (left, right) = line.split_at(line.len() / 2);
//         let mut left: Vec<char> = left.chars().collect();
//         let mut right: Vec<char> = right.chars().collect();

//         left.sort();
//         left.dedup();
//         right.sort();
//         right.dedup();

//         for item in left {
//             if right.contains(&item) {
//                 matching_items.push(item);
//             }
//         }
//     }

//     let part_1_sum = find_priorities(matching_items);
//     println!("Part 1 Sum is {}", part_1_sum);
// }
