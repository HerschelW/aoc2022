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
    let mut round = 1;
    // loop through lines vector
    for i in &lines {
        // print i
        println!("\nRound {}", round);
        // get a and b
        let a = i.chars().next().unwrap();
        // print a
        println!("a: {}", a);
        let b = i.chars().nth(2).unwrap();
        // print b
        println!("b: {}", b);

        // increment round
        round += 1;
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
        println!("New score: {}", score);
    }
    // print lines length
    println!("lines length: {}", lines.len());
}
