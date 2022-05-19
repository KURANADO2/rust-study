fn main() {
    let mut s = String::from("Hello world");
    let len = first_word(&s);
    println!("{}", len);
    s.clear();

    let s2 = String::from("Hello world");
    let hello = &s2[0..5];
    let world = &s2[6..11];
    println!("{}, {}", hello, world);
    let hello = first_word2(&s2);
    println!("{}", hello);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3])
}

fn first_word(some_str: &String) -> usize {
    let bytes = some_str.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    some_str.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}