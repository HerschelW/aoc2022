use std::io::{self};

pub struct RockPaperScissors {
    should_quit: bool,
}

impl RockPaperScissors {
    pub fn new() -> RockPaperScissors {
        RockPaperScissors { should_quit: false }
    }

    pub fn run(&mut self) {
        pub fn get_choice() {
            let mut player_choice = String::new();
            io::stdin()
                .read_line(&mut player_choice)
                .expect("Failed to read line");
            let player_choice_uppercase = player_choice.to_uppercase();
            player_choice = player_choice_uppercase.clone();
            let _a: &str = "A";
            let _b: &str = "B";
            let _c: &str = "C";
            if player_choice == _a || player_choice == _b || player_choice == _c {
                player_choice_uppercase;
            } else {
                println!("Invalid input");
                get_choice();
            }
        }

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
            let mut player1_choice = get_choice();

            let mut player2_choice = String::new();
            io::stdin()
                .read_line(&mut player2_choice)
                .expect("Failed to read line");
            player2_choice = player2_choice.to_uppercase();
            // print player 2 choice
            println!("{} chose {}", player2, player2_choice);

            if player1_choice == 'A' && player2_choice == "B" {
                player2_score += 3;
            } else if player1_choice == 'A' && player2_choice == "C" {
                player1_score += 3;
            } else if player1_choice == "B" && player2_choice == 'A' {
                player1_score += 3;
            } else if player1_choice == "B" && player2_choice == "C" {
                player2_score += 3;
            } else if player1_choice == "C" && player2_choice == 'A' {
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
                // clone player1
                winner = player1.clone();
                game = false;
            } else if player2_score >= 21 {
                // clone player2
                winner = player2.clone();
                game = false;
            }
        }

        println!("{} wins!", winner);
    }
}
