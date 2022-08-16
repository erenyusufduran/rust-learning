fn main() {
    let s = String::from("Hi There!");
    let word = first_word(&s);
    let second_word_ = second_word(&s);
    println!("{word}");
    println!("{second_word_}");

    // string slices
    let strn = String::from("Hello World");
    let hello = &strn[0..5]; // we can write like ..5
    let world = &strn[6..11]; // 6..
    println!("{hello} {world}");

    // other slices
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }
    &s[..]
}
