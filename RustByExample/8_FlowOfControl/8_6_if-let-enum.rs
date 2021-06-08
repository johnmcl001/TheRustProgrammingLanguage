enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn main() {
    // create simple variables
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // Variable matches foobarr
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // variable b does not match foobar, this will print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // variable c matchs fooqux which has a value
    // similar to some()
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // binding also works
    if let Foo::Qux(_value @ 100) = c {
        println!("c is 100");
    }
}