use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    scores.entry(String::from("Blue")).or_insert(30);
    scores.entry(String::from("Yellow")).or_insert(30);

    println!("{:?}", scores);

}
