fn main() {
    let v = vec!['a', 'z', 'b'];
    println!("Largest value in {:?} is {}", v, find_largest(&v));
}

fn find_largest<T>(v: &Vec<T>) -> &T 
    where T: Ord
{
    v.iter().max().unwrap() 
}
