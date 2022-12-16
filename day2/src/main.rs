// A = Rock X = Rock (1)
// B = Paper Y = Paper (2)
// C = Scissors Z = Scissors (3)
// 0 for loss
// 3 for draw
// 6 for win

use std::io;

fn main() {
    rochambeau();
}

fn rochambeau() {
    let mut player1 = String::new();
    let mut player2 = String::new();
    let mut player1_score = 0;
    let mut player2_score = 0;
    let mut round = 0;
    let mut game = true;
    let mut winner = String::new();

    println!("Welcome to Rochambeau!");
    println!("Player 1, please enter your name: ");
    io::stdin()
        .read_line(&mut player1)
        .expect("Failed to read line");
    println!("Player 2, please enter your name: ");
    io::stdin()
        .read_line(&mut player2)
        .expect("Failed to read line");

    while game {
        round += 1;
        println!("Round {}", round);
        let mut player1_choice = String::new();
        io::stdin()
            .read_line(&mut player1_choice)
            .expect("Failed to read line");
        let mut player2_choice = String::new();
        io::stdin()
            .read_line(&mut player2_choice)
            .expect("Failed to read line");

        if player1_choice == "A" && player2_choice == "B" {
            player2_score += 3;
        } else if player1_choice == "A" && player2_choice == "C" {
            player1_score += 3;
        } else if player1_choice == "B" && player2_choice == "A" {
            player1_score += 3;
        } else if player1_choice == "B" && player2_choice == "C" {
            player2_score += 3;
        } else if player1_choice == "C" && player2_choice == "A" {
            player2_score += 3;
        } else if player1_choice == "C" && player2_choice == "B" {
            player1_score += 3;
        } else if player1_choice == player2_choice {
            player1_score += 1;
            player2_score += 1;
        } else {
            println!("Invalid input");
        }

        if player1_score >= 21 {
            winner = player1;
            game = false;
        } else if player2_score >= 21 {
            winner = player2;
            game = false;
        }
    }

    println!("{} wins!", winner);
}
