fn main() {
    let width1 = 30;
    let height1 = 50;
    let rect_area = area(width1, height1);
    println!("{}", rect_area);
}


fn area(width: u32, height: u32) -> u32 {
    width * height
}