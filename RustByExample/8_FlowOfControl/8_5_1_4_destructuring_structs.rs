fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo{ x: (2, 2), y: 2};

    match foo {
        Foo { x: (1, b), y  } => println!("first of x is 1, b = {}, y = {}", b, y),

        // can destructure structs and rename the variables, order not important
        Foo { y:2, x: i } => println!("y is 2, i = {:?}", i),

        // can also ignore some variables
        Foo{ y, .. } => println!("y is {}, don't care about x", y),
    }
}

