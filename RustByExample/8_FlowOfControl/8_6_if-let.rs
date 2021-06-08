fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // if let reads if let destructures number into Some(i), evaluate block
    if let Some(i) = number {
        println!("Matched: {:?}", i);
    }

    // if failues needs specification use else
    if let Some(i) = letter {
        println!("Matched: {:?}", i);
    } else {
        println!("Didn't match a number, use a letter");
    }

    // provide an altered failing condition
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched: {:?}", i);
    } else if i_like_letters {
        println!("Didn't match a number, use a letter");
    } else {
        println!("Didn't match a letter, use an emoticon");
    }
}