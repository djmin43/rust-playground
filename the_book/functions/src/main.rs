fn main() {
    let y = plus_one(100);
    println!("the value of y is : {y}");
}

fn plus_one (x: i32) -> i32 {
    x + 1
}