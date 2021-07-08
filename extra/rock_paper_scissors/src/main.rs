use rand::Rng;
use std::io::{self, Write};
use std::error;

#[derive(Debug)]
enum GuessType {
    Rock,
    Paper,
    Scissors,
}

impl GuessType {
    fn derive(guess: &str) -> Result<GuessType, String> {
        match guess {
            "rock" => Ok(GuessType::Rock),
            "paper" => Ok(GuessType::Paper),
            "scissors" => Ok(GuessType::Scissors),
            e => Err(e.to_string()),
        }
    }
}

#[derive(Debug)]
struct Guess {
    guess: GuessType,
}

impl Guess {
    fn new(guess: GuessType) -> Guess {
        Guess {
            guess,
        }
    }

    fn get_guess(&self) -> &GuessType {
        &self.guess
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
	loop {
        let computer_guess = get_computer_guess();
        let user_guess = match get_user_guess() {
            Ok(guess) => guess,
            Err(e) => {
                println!("{} is not one of rock, paper, scissors\n", e);
                continue;
            }
        };
        check_result(user_guess, computer_guess);
        match &check_quit()[..] {
            "y" => return Ok(()),
            "n" => continue,
            &_ => panic!("Invalid input"),
        };
	}
}

fn check_quit() -> String {
    print!("Quit? (Y/N): ");
    io::stdout().flush().unwrap();
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read");
    input.to_lowercase().trim().to_string()
}

fn get_user_guess() -> Result<Guess, String> {
    print!("\nInput guess: ");
    io::stdout().flush().unwrap();
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read");
    match GuessType::derive(input.to_lowercase().trim()) {
        Ok(guess) => Ok(Guess::new(guess)),
        Err(e) => Err(e.to_string()), 
    }
}

fn get_computer_guess() -> Guess {
	let guess = match rand::thread_rng().gen_range(1..101) {
        1..=33 => GuessType::derive("rock"),
        34..=66 => GuessType::derive("paper"),
        _ => GuessType::derive("scissors")
    };
    Guess::new(guess.unwrap())
}

fn check_result(user_guess: Guess, computer_guess: Guess) {
    println!("User: {:?}", user_guess.get_guess());
    println!("Computer: {:?}", computer_guess.get_guess());
    println!("{}\n", get_result(computer_guess, user_guess));
}

fn get_result(user_guess: Guess, computer_guess: Guess) -> String {
	let result = match user_guess.get_guess() {
		GuessType::Rock => {
			match computer_guess.get_guess() {
				GuessType::Rock => "Tie",
				GuessType::Paper => "Computer wins",
				GuessType::Scissors => "User wins",
			}
		},
		GuessType::Paper => {
			match computer_guess.get_guess() {
				GuessType::Rock => "User wins",
				GuessType::Paper => "Tie",
				GuessType::Scissors => "Computer wins",
			}
		},
		GuessType::Scissors => {
			match computer_guess.get_guess() {
				GuessType::Rock => "Computer wins",
				GuessType::Paper => "User wins",
				GuessType::Scissors => "Tie",
			}
		}
	};
    result.to_string()
}
