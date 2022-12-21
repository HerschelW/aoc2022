pub fn find_priorities(matching_items: Vec<char>) -> i32 {
    let mut alphabet: Vec<char> = Vec::new();

    for i in 1..27 {
        alphabet.push((i + 96) as u8 as char);
    }

    for i in 1..27 {
        alphabet.push((i + 64) as u8 as char);
    }

    let mut matching_values: Vec<i32> = Vec::new();
    for item in matching_items {
        for j in 0..alphabet.len() {
            if item == alphabet[j] {
                matching_values.push(j as i32 + 1);
            }
        }
    }

    let mut sum = 0;
    for value in matching_values {
        sum += value;
    }

    sum
}
