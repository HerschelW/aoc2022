fn main() {
    solve_day_3();
}

fn solve_day_3() {
    // parse input.txt
    let input = include_str!("input.txt");
    let lines = input.lines();
    // empty vector
    let mut matching_items: Vec<char> = Vec::new();
    for line in lines {
        // split line into two halves
        let (left, right) = line.split_at(line.len() / 2);

        for l in left.chars() {
            for r in right.chars() {
                if l == r {
                    matching_items.push(l);
                }
            }
        }
    }

    // print matching items
    println!("{:?}", matching_items);

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
