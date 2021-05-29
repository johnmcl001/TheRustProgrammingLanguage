fn main() {
    let fahr = 22;
    let celsius = 44;
    println!("{:.2} fahrenheit is {:.2} celsius", fahr, fahrenheit_to_celsius(fahr));
    println!("{:.2} celsius is {:.2} fahrenheit", celsius, celsius_to_fahrenheit(celsius));
}

fn fahrenheit_to_celsius(fahrenheit: i32) -> f64 {
    let fahrenheit: f64 = fahrenheit.into();
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: i32) -> f64 {
    let celsius: f64 = celsius.into();
    celsius * 9.0 / 5.0 + 32.0
}
