#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct User {
    id: u32,
}

fn main() {
    sort_vector_of_integers();
    sort_vector_of_floats();
    sort_vector_of_structs();
}

fn sort_vector_of_integers() {
    println!("\nSort vector of integers");
    let mut vec = vec![6, 3, 1, 4, 9, -6, 22, 4];
    println!("Before: {:?}", vec);
    vec.sort();
    println!("After: {:?}", vec);
    assert_eq!(vec, [-6, 1, 3, 4, 4, 6, 9, 22]);
}

fn sort_vector_of_floats() {
    println!("\nSort vector of floats");
    let mut vec = vec![6.0, 3.0, 1.0, 4.0, 9.0, -6.0, 22.0, 4.0];
    println!("Before: {:?}", vec);
    vec.sort_by(|i, j| i.partial_cmp(j).unwrap());
    println!("After: {:?}", vec);
    assert_eq!(vec, [-6.0, 1.0, 3.0, 4.0, 4.0, 6.0, 9.0, 22.0]);
}

fn sort_vector_of_structs() {
    println!("\nSort vector of structs");
    let mut vec = vec![User {id: 6}, User {id: 2}, User {id: 5}, User {id: 1}];
    println!("Before: {:?}", vec);
    vec.sort_by_key(|u| u.id);
    println!("After: {:?}", vec);
    assert_eq!(vec, [User {id: 1}, User {id: 2}, User {id: 5}, User {id: 6}]);
}
