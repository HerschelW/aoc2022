fn main() {
    rochambeau();
}

fn rochambeau() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut score = 0;
    let mut part2_score = 0;
    let mut choices: Vec<String> = Vec::new();

    for i in &lines {
        let a = i.chars().next().unwrap();
        let b = i.chars().nth(2).unwrap();

        match (b, a) {
            ('X', 'B') => score += 1,
            ('Y', 'C') => score += 2,
            ('Z', 'A') => score += 3,
            ('X', 'A') => score += 4,
            ('Y', 'B') => score += 5,
            ('Z', 'C') => score += 6,
            ('X', 'C') => score += 7,
            ('Y', 'A') => score += 8,
            ('Z', 'B') => score += 9,
            _ => println!("Error"),
        }

        let mut choice = ' ';

        match b {
            'X' => match a {
                'A' => choice = 'Z',
                'B' => choice = 'X',
                'C' => choice = 'Y',
                _ => (),
            },
            'Y' => match a {
                'A' => choice = 'X',
                'B' => choice = 'Y',
                'C' => choice = 'Z',
                _ => (),
            },
            'Z' => match a {
                'A' => choice = 'Y',
                'B' => choice = 'Z',
                'C' => choice = 'X',
                _ => (),
            },
            _ => println!("Error"),
        }

        let choice_str = format!("{} {}", a, choice);
        choices.push(choice_str)
    }

    for i in &choices {
        let a = i.chars().next().unwrap();
        let b = i.chars().nth(2).unwrap();

        match (b, a) {
            ('X', 'B') => part2_score += 1,
            ('Y', 'C') => part2_score += 2,
            ('Z', 'A') => part2_score += 3,
            ('X', 'A') => part2_score += 4,
            ('Y', 'B') => part2_score += 5,
            ('Z', 'C') => part2_score += 6,
            ('X', 'C') => part2_score += 7,
            ('Y', 'A') => part2_score += 8,
            ('Z', 'B') => part2_score += 9,
            _ => println!("Error"),
        }
    }

    println!("Final score: {}", score);
    println!("Part 2 score: {}", part2_score);
}
