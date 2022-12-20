fn main() {
    solve_day_3();
}

fn solve_day_3() {
    // parse input.txt
    let input = include_str!("sample_input.txt");
    let lines = input.lines();
    // print first 5 lines
    let mut matching_item = ' ';
    // empty vector
    let mut matching_items: Vec<char> = Vec::new();
    for line in lines {
        // print length of line
        println!("{}: {}", line.len(), line);
        // split line into two halves
        let (left, right) = line.split_at(line.len() / 2);
        // print left and right halves
        println!("{} {}", left, right);
        // find matching letters in halves
        let mut matches = 0;

        for l in left.chars() {
            for r in right.chars() {
                if l == r {
                    matches += 1;
                    matching_item = l;
                    matching_items.push(matching_item);
                }
            }
        }
        // print number of matches
        println!("{} matches", matches);
    }
    // remove duplicate items
    matching_items.sort();
    matching_items.dedup();
    // print number of unique matches
    println!("{} unique matches", matching_items.len());

    // print matching_items
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
    // print alphabet
    println!("{:?}", alphabet);

    // find value for matching items
    let mut matching_values: Vec<i32> = Vec::new();
    for i in 0..matching_items.len() {
        for j in 0..alphabet.len() {
            if matching_items[i] == alphabet[j] {
                matching_values.push(j as i32 + 1);
            }
        }
    }
    // print matching values
    println!("{:?}", matching_values);

    // find sum of matching values
    let mut sum = 0;
    for i in 0..matching_values.len() {
        sum += matching_values[i];
    }
    // print sum
    println!("{}", sum);
}
