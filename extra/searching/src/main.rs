fn main() {
    const TEST_ARR_ORDERED: [i32; 8] = [1, 2, 3, 4, 5, 6, 9, 11];
    const TEST_ARR_UNORDERED: [i32; 8] = [5, 3, 2, 4, 9, 8, 1, 0];
    const FOUND_TARGET: i32 = 4;
    const UNFOUND_TARGET: i32 = 10;

    println!("{} in {:?}: {}", FOUND_TARGET, TEST_ARR_ORDERED, iter_binary_search(&TEST_ARR_ORDERED[..], &FOUND_TARGET));
    println!("{} in {:?}: {}", UNFOUND_TARGET, TEST_ARR_ORDERED, iter_binary_search(&TEST_ARR_ORDERED[..], &UNFOUND_TARGET));
    println!("{} in {:?}: {}", FOUND_TARGET, TEST_ARR_ORDERED, rec_binary_search(&TEST_ARR_ORDERED[..], &FOUND_TARGET));
    println!("{} in {:?}: {}", UNFOUND_TARGET, TEST_ARR_ORDERED, rec_binary_search(&TEST_ARR_ORDERED[..], &UNFOUND_TARGET));
    println!("{} in {:?}: {}", FOUND_TARGET, TEST_ARR_UNORDERED, linear_search(&TEST_ARR_UNORDERED[..], &FOUND_TARGET));
    println!("{} in {:?}: {}", UNFOUND_TARGET, TEST_ARR_UNORDERED, linear_search(&TEST_ARR_UNORDERED[..], &UNFOUND_TARGET));
}

fn iter_binary_search(arr: &[i32], target: &i32) -> bool{
    let mut low: usize = 0;
    let mut high: usize = arr.len();

    while low <= high {
        let mid: usize = ((high - low) / 2) + low;

        if *target == arr[mid] {
            return true
        }

        if *target < arr[mid] {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    false
}

fn rec_binary_search(arr: &[i32], target: &i32) -> bool{
    if arr.len() <= 1 {
        return arr.len() == 0 || arr[0] == *target
    }

    let mid: usize = arr.len() / 2;
    
    if *target < arr[mid] {
        rec_binary_search(&arr[..mid], target)
    } else {
        rec_binary_search(&arr[mid..], target)
    }
}

fn linear_search(arr: &[i32], target: &i32) -> bool {
    match arr.iter().find(|&x| x == target) {
        Some(_) => true,
        None => false
    }
}
