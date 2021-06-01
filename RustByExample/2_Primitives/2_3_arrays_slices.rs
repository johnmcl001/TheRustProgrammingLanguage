use std::mem;

fn main() {
    // Fized size array
    let xs: [i32; 5] = [1, 2, 3, 4 , 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // len returns the count of the elements in the array
    println!("number of elements in array: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("Borrow whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [start..end]
    // start is the first position of the slice
    // end is one more than the last position
    println!("borrow section of array as slice");
    analyze_slice(&ys[1..4]);

}

// This functin borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}