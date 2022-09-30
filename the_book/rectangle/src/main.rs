fn main() {
    let width1 = 30;
    let height1 = 50;
    let rect_area = area(width1, height1);
    println!("{}", rect_area);


    let rect1 = (30,50);
    println!("{}", area_tuple(rect1))
}


fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) ->  u32{
    dimensions.0 * dimensions.1
}