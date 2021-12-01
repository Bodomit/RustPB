fn main() {
    let s = String::from("I'm just a poor boy, nobody loves me");

    let word = first_word(&s[0..6]);
    let word = first_word(&s[..]);
    let word = first_word(&s);

    let sl = "hello world";
    let word = first_word(&sl[0..6]);
    let word = first_word(&sl[..]);
    let word = first_word(sl);
    println!("word: {}", word)
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
