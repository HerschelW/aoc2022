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
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut score = 0;
    for i in &lines {
        let a = i.chars().next().unwrap();
        let b = i.chars().nth(2).unwrap();
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
            _ => (),
        }
    }
    println!("score: {}", score);
}
