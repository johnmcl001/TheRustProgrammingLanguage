#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: &str, email: &str) -> User {
    User {
        username: String::from(username),
        email: String::from(email),
        sign_in_count: 0,
        active: true,
    }
}

fn main() {
    let user1 = User {
        username: String::from("username"),
        email: String::from("email"),
        sign_in_count: 0,
        active: true,
    };
    println!("User 1: {:?}", user1);
    println!("User 1 username: {}", user1.username);
    let mut user2 = User {
        username: String::from("username2"),
        email: String::from("email2"),
        sign_in_count: 0,
        active: true,
    };
    user2.username = String::from("this is a test");
    println!("User 2: {:?}", user2);

    let user3 = build_user("built user", "email email");
    println!("User 3: {:?}", user3);
}
