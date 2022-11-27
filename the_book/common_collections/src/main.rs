
fn main() {

    let mut v = vec![1,4,110, 55,12, 34, 61, 6,23, 612, 132424, 324,2,3,4,5,6,7,8,9,10];

    v.sort();

    let median = v.len()/2;

    let v_median = v[median];


    println!("{:?}", v);
    println!("{}", v_median);
}
