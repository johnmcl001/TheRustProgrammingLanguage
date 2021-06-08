enum Foo {
    Bar,
}

fn main() {
    // create simple variables
    let a = Foo::Bar;

    // Variable matches foobarr
    if let Foo::Bar = a {
        println!("a is foobar");
    }
}