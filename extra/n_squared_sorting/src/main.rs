fn main() {
    println!("Bubble Sort Functional");
    let arr = [6, 7, 2, 3, 2, 1];
    println!("Before: {:?}", arr);
    println!("After: {:?}", bubble_sort_functional(arr));

    println!("\nSelection Sort Functional");
    println!("Before: {:?}", arr);
    println!("After: {:?}", selection_sort_functional(arr));

    println!("\nInsertion Sort Functional");
    println!("Before: {:?}", arr);
    println!("After: {:?}", insertion_sort_functional(arr));

    println!("\nBubble Sort In Place");
    let mut arr = [6, 7, 2, 3, 2, 1];
    println!("Before: {:?}", arr);
    bubble_sort(&mut arr);
    println!("After: {:?}", arr);

    arr = [6, 7, 2, 3, 2, 1];
    println!("\nSelection Sort In Place");
    println!("Before: {:?}", arr);
    selection_sort(&mut arr);
    println!("After: {:?}", arr);

    arr = [6, 7, 2, 3, 2, 1];
    println!("\nInsertion Sort");
    println!("Before: {:?}", arr);
    insertion_sort(&mut arr);
    println!("After: {:?}", arr);
}


fn bubble_sort_functional(arr: [i32; 6]) -> [i32; 6]{
    let mut swap: bool;
    let mut result: [i32; 6] = arr;
    for i in (0..result.len()).rev() {
        swap = false;
        for j in 0..i {
            if result[j] > result[j + 1] {
                result.swap(j, j + 1);
                swap = true;
            }
        }
        if !swap {
            break;
        }
    }
    result
}

fn selection_sort_functional(arr: [i32; 6]) -> [i32; 6] {
    let mut result: [i32; 6] = arr;
    for i in 0..result.len() {
        let mut smallest = i;
        for j in i + 1..result.len() {
            if result[j] < result[smallest] {
                smallest = j;
            }
        }
        result.swap(i, smallest);
    }
    result
}

fn insertion_sort_functional(arr: [i32; 6]) -> [i32; 6] {
    let mut result = arr;
    for i in 1..result.len() {
        let mut j = i;
        while j > 0 && result[j] < result[j - 1] {
            result.swap(j, j - 1);
            j -= 1;
        }
    }
    result
}

fn bubble_sort(arr: &mut [i32]) {
    let mut swap: bool;
    for i in (0..arr.len()).rev() {
        swap = false;
        for j in 0..i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swap = true;
            }
        }
        if !swap {
            return;
        }
    }
}

fn selection_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        let mut smallest = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[smallest] {
                smallest = j;
            }
        }
        arr.swap(i, smallest);
    }
}


fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}
