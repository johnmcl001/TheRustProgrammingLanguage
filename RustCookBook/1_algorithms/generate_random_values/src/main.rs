use std::iter;
use rand::Rng;
use rand::distributions::Alphanumeric;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::distributions::Standard;

#[derive(Debug)]
struct Rectangle {
    height: u8,
    width: u8,
}

impl Distribution<Rectangle> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rectangle {
        Rectangle{
            width: rng.gen(),
            height: rng.gen(),
        }
    }
}

fn main() {
    generate_random_numbers();
    generate_random_numbers_within_a_range();
    generate_random_numbers_with_a_given_distribution();
    generate_random_values_of_a_custom_type();
    create_random_passwords_from_set_of_alphanumeric_characters();
    create_random_passwords_from_set_of_user_defined_characters();
}

fn generate_random_numbers() {
    println!("Generate Random Numbers\n");
    let mut rng = rand::thread_rng();
    let x: u8 = rng.gen();
    println!("Generated number: {}\n", x);
    let x: u16 = rng.gen();
    println!("Generated number: {}\n", x);
    let x: u32 = rng.gen();
    println!("Generated number: {}\n", x);
    let x: u64 = rng.gen();
    println!("Generated number: {}\n", x);
    let x: i8 = rng.gen();
    println!("Generated number: {}\n", x);
    let x: i16 = rng.gen();
    println!("Generated number: {}\n", x);
    let x: i32 = rng.gen();
    println!("Generated number: {}\n", x);
    let x: i64 = rng.gen();
    println!("Generated number: {}\n", x);
    let x: f32 = rng.gen();
    println!("Generated number: {}\n", x);
    let x: f64 = rng.gen();
    println!("Generated number: {}\n", x);
}

fn generate_random_numbers_within_a_range() {
    println!("Generate Random Numbers Within A Range\n");
    let mut rng = rand::thread_rng();
    let x: u32 = rng.gen_range(1..10);
    println!("Generated number within range (1..10): {}\n", x);
    let x: f32 = rng.gen_range(1.0..10.0);
    println!("Generated number within range (1..10): {}\n", x);
}

fn generate_random_numbers_with_a_given_distribution() {
    println!("Generate Random Numbers With A Given Distribution\n");
    let mut rng = rand::thread_rng();
    let x: u32 = rng.sample(Uniform::new(10u32, 15));
    println!("Generated number with given distribution: {}\n", x);
}

fn generate_random_values_of_a_custom_type() {
    println!("Generate Random Numbers Of A Custom Type\n");
    let mut rng = rand::thread_rng();
    let rectangle: Rectangle = rng.gen();  
    println!("Random rectangle: {:?}\n", rectangle);
}
    
fn create_random_passwords_from_set_of_alphanumeric_characters() {
    println!("Generate Passwords From Set of Alphanumeric Characters\n");
    let mut rng = rand::thread_rng();
    let chars: String = rng.sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    println!("Random Characters: {}\n", chars);
}

fn create_random_passwords_from_set_of_user_defined_characters() {
    println!("Generate Passwords From Set of User Defined Characters\n");
    const CHARS: &[u8] = b"ASDFGHJKLasdfghjkl-=[];'#,./";
    let mut rng = rand::thread_rng();
    let chars: String = iter::repeat(())
        .map(|()| rng.gen_range(0..CHARS.len()))
        .map(|i| CHARS[i])
        .take(7)
        .map(char::from)
        .collect();
    println!("Random Characters: {}\n", chars);
}