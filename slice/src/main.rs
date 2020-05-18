fn main() {
    let mut s = String::from("Hello, World!");
    let first_word = find_first_word(&s);
    let second_word = find_first_word(&s[7..]);

    println!("first word: {}", first_word);
    println!("second word: {}", second_word);

    s.clear();

    println!("str: {}", s);

    let x: [u8; 256] = [0; 256];
    let y = &x[..128];

    println!("{:?}", y.len())
}

fn find_first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }

    s
}
