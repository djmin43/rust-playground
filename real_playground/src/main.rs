fn main() {
    let mut s = vec![1, 2, 3];
    {
        let v = s
            .drain(2..)
            .collect::<Vec<i32>>();
        println!("{:?}", v);
    }

    println!("{:?}", s);

}
