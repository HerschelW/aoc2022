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

        // compare a and b as string
        match a {
            'A' => match b {
                'X' => score += 4,
                'Y' => score += 1,
                'Z' => score += 7,
                _ => (),
            },
            'B' => match b {
                'X' => score += 8,
                'Y' => score += 5,
                'Z' => score += 2,
                _ => (),
            },
            'C' => match b {
                'X' => score += 3,
                'Y' => score += 9,
                'Z' => score += 6,
                _ => (),
            },
            // if no matches print 'error: no matches'
            _ => (),
        }
        //     if a == b {
        //         score += 3;
        //     } else if a == 'A' && b == 'Z' {
        //         score += 6;
        //     } else if a == 'B' && b == 'X' {
        //         score += 6;
        //     } else if a == 'C' && b == 'Y' {
        //         score += 6;
        //     } else if a == 'A' && b == 'Y' {
        //         score += 0;
        //     } else if a == 'B' && b == 'Z' {
        //         score += 0;
        //     } else if a == 'C' && b == 'X' {
        //         score += 0;
        //     }
        // }
    }
    // print score
    println!("score: {}", score);
}
