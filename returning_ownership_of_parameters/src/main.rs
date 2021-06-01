fn main() {
    let mut s1 = String::from("hello");

    let r1 = &s1; // Immutable reference- OK
    let r2 = &s1; // Immtable reference - OK
    let r3 = &mut s1; // Mutable referecne - NOT OK

    // println!("{}, {}", r1, r2);
    println!("{}, {}, {}", r1, r2, r3);

    // {
    //     let r1 = &mut s1; // mutable borrow in different scope
    // } // r1 goes out of scope so new reference can be made.

    // let r2 = &mut s1; // second mutable borrow in different scope -OK

    // let r1 = &mut s1; // first mutable borrow - OK
    // let r2 = &mut s1; // second mutable borrow - NOT OK
    // println!("{}, {}", r1, r2);

    // change(&mut s1);

    // let len = calculate_length(&s1);
    // println!("The length of {} is {}", s1, len)
}

// fn change(str: &mut String) {
//     str.push_str(", world!");
// }

// fn calculate_length(str: &String) -> usize { // s is a reference to String
//     str.len()
// }
// s goes out of scope but because ownership wasn't transferred nothing happens

// fn calculate_length(str: String) -> (String, usize) {
//     let length = str.len();
//     (str, length)
// }
