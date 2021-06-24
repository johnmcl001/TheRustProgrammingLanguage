fn main() {
    println!("Merge sort using vectors");
    let vec = vec![6, 7, 2, 3, 2, 1];
    println!("Before: {:?}", vec);
    println!("After: {:?}", merge_sort_vec(vec));
}

fn merge_sort_vec(vec: Vec<i32>) -> Vec<i32> {
    if vec.len() <= 1 {
        return vec
    }

    let mut l_vec = merge_sort_vec((&vec[..vec.len() / 2]).to_vec());
    let mut r_vec = merge_sort_vec((&vec[vec.len() / 2..]).to_vec());
    let mut sorted_vec: Vec<i32> = Vec::new();

    while !l_vec.is_empty() && !r_vec.is_empty() {
        if &l_vec[0] < &r_vec[0] {
            sorted_vec.push(l_vec.remove(0));
        } else {
            sorted_vec.push(r_vec.remove(0));
        }
    }

    sorted_vec.append(if !l_vec.is_empty() { &mut l_vec } else { &mut r_vec });

    sorted_vec
} 
