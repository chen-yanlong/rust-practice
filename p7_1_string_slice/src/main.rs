fn main() {
    let s = String::from("hello world");
    // `str` stands for string slice, its store in stack in bin
    // let hello = &s[0..5];
    // let world = &s[6..11];

    let s2 = "hello world";

    // let word = first_word(&s);
    let word = first_word(s2);
    println!("{}", word);
    


}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..]
}
