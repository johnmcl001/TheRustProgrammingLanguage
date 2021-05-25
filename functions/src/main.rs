fn main() {
    let x = 5;

    another_function(x, 32);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
