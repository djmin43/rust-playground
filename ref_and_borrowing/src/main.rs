fn main() {
    let s1 = String::from("hello");
    let mut s2 = String::from("hello");

    let len = calculate_string_length(&s1);
    change(&mut s2);

    println!("The length of '{}' is {}.", s1, len);
    println!("{s2}");

//     dangling
    let reference_to_nothing = dangle();


}

fn calculate_string_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String){
    some_string.push_str(", world");
}

fn dangle() -> &String {
    let s= String::from("hello");

    &s
}