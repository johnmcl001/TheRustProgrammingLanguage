fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // expression assigned to y
        x_cube + x_squared + x
    };

    let z = {
        // semicolon supresses this expression and () is assigned to z
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}