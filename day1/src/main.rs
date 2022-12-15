fn main() {
    solve_day_1();
}

fn solve_day_1() {
    let input = include_str!("../input.txt");
    let input: Vec<&str> = input.split_whitespace().collect();
    let mut input: Vec<i32> = input.iter().map(|x| x.parse().unwrap()).collect();
    input.sort();
    let largest = input[input.len() - 1];
    println!("{:?}", largest);
}
