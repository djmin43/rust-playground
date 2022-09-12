fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
