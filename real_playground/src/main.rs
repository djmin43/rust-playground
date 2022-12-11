fn main() {
    let mut s = vec![1, 2, 3];
    let v = s
        .drain(1..)
        .collect::<Vec<i32>>();

    println!("{:?}", s);
    println!("{:?}", v);

}
