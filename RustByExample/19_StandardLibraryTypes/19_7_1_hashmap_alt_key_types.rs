use std::collections::HashMap;

// Eq requires taht you derivce PartialEq on the type
#[derive(PartialEq, Eq, Hash)]
struct Account<'a> {
	username: &'a str,
	password: &'a str,
}

struct AccountInfo<'a> {
	name: &'a str,
	email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
	println!("Username: {}", username);
	println!("Password: {}", password);
	println!("Attempting logon...");

	let logon = Account {
		username,
		password,
	};

	match accounts.get(&logon) {
		Some(account_info) => {
			println!("Successful logon");
			println!("Name: {}", account_info.name);
			println!("Email: {}", account_info.email);
		},
		_ => println!("Login failed"),
	}
} 

fn main() {
	let mut accounts: Accounts = HashMap::new();

	let account = Account {
		username: "john",
		password: "password",
	};

	let account_info = AccountInfo {
		name: "john",
		email: "email",
	};

	accounts.insert(account, account_info);

	try_logon(&accounts, "john", "password");
	try_logon(&accounts, "john", "wrong");
}