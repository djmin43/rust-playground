fn main() {

    let mut s = String::from("hello world");

    let word = first_word(&s);

    let hello = &s[0..5];
    let world = &s[6..11];

    let hello_world = &s[..];


    println!("{word}");

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

