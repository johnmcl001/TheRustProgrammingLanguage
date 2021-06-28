use std::collections::HashMap;

fn main() {
    let mut v = vec![1, 3, 3, 3, 4, 5, 5, 2, 22, 100];
    println!("Mean: {}", mean(&v));
    println!("Median: {}", median(&mut v));
    println!("Mode: {}", mode(&v));
}

fn mean(v: &Vec<u32>) -> f32 {
    v.iter().sum::<u32>() as f32 / v.len() as f32
}

fn median(v: &mut Vec<u32>) -> u32 {
    v.sort();
    v[v.len() / 2]
}

fn mode(v: & Vec<u32>) -> u32 {
    let mut occurences = HashMap::new();
    for &i in v {
        *occurences.entry(i).or_insert(0) += 1;
    }
    let mut mode = 0;
    for key in occurences.keys() {
        if occurences.get(key) > occurences.get(&mode) {
            mode = *key;
        }
    }
    occurences.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| value)
        .expect("Zero returned")
}