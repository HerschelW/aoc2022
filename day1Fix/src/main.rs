fn main() {
    // turn input.txt into an array
    let input = include_str!("../input.txt");
    let input: Vec<&str> = input.split_whitespace().collect();
    // turn array into a vector of integers
    let mut input: Vec<i32> = input.iter().map(|x| x.parse().unwrap()).collect();
    // sort vector
    input.sort();
    // assign the largest integer to a variable 'largest'
    let largest = input[input.len() - 1];
    // print largest
    println!("{:?}", largest);
}
