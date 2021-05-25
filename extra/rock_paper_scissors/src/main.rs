use rand::Rng;
use std::io;

fn main() {
	loop {
        let mut user_guess = String::new();
	    io::stdin()
			.read_line(&mut user_guess)
			.expect("Failed to read line");

		match user_guess.to_lowercase().trim() {
			"rock" | "paper" | "scissors" => play_round(&user_guess),
			"quit" => break,
			_ => {
				println!("Invalid input: {}", user_guess.to_lowercase().trim());
				continue;
			},
		};
	}
}

fn play_round(user_guess: &String) {
	let comp_guess = match rand::thread_rng().gen_range(1, 101) {
        1..=33 => "rock",
        34..=66 => "paper",
        _ => "scissors"
    };

	let result = match user_guess.to_lowercase().trim() {
		"rock" => {
			match comp_guess {
				"rock" => "Tie",
				"paper" => "Computer wins",
				_ => "User wins",
			}
		},
		"paper" => {
			match comp_guess {
				"rock" => "User wins",
				"paper" => "Tie",
				_ => "Computer wins",
			}
		},
		_ => {
			match comp_guess {
				"rock" => "Computer wins",
				"paper" => "User wins",
				_ => "Tie",
			}
		}
	};

	println!("User: {}", user_guess.to_lowercase().trim());
	println!("Comp: {}", comp_guess);
	println!("{}", result);
}
