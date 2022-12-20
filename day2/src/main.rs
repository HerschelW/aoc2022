// A = Rock X = Rock (1)
// B = Paper Y = Paper (2)
// C = Scissors Z = Scissors (3)
// 0 for loss
// 3 for draw
// 6 for win

fn main() {
    rochambeau();
}

fn rochambeau() {
    // Read input file
    let input = std::fs::read_to_string("src/input.txt").unwrap();

    // split lines into vector
    let lines: Vec<&str> = input.lines().collect();
    // print lines
    println!("lines: {:?}", lines);

    // compare a to b in lines vector
    // if a == b, add 3 to score
    // if a == rock and b == scissors, add 6 to score
    // if a == paper and b == rock, add 6 to score
    // if a == scissors and b == paper, add 6 to score
    // if a == rock and b == paper, add 0 to score
    // if a == paper and b == scissors, add 0 to score
    // if a == scissors and b == rock, add 0 to score

    // create score variable
    let mut score = 0;
    let mut part2_score = 0;
    let mut choices: Vec<String> = Vec::new();
    // loop through lines vector
    for i in &lines {
        // get a and b
        let a = i.chars().next().unwrap();
        let b = i.chars().nth(2).unwrap();
        // match b and a as string
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
        // print score
        // println!("New score: {}", score);

        // part 2
        // create empty char variable
        let mut choice = ' ';
        // create empty string variable
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

        // make single str from a and choice
        let choice_str = format!("{} {}", a, choice);

        // push choice_str to choices vector
        choices.push(choice_str)
    }

    // loop through choices vector
    for i in &choices {
        // get a and b
        let a = i.chars().next().unwrap();
        // print a
        println!("a: {}", a);
        let b = i.chars().nth(2).unwrap();
        // print b
        println!("b: {}", b);
        // match b and a as string
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

    // print score
    println!("Final score: {}", score);
    println!("Part 2 score: {}", part2_score);
}
