struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    let rect_area = area(width1, height1);
    println!("{}", rect_area);

    let rect1 = (30,50);
    println!("{}", area_tuple(rect1));

    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect3_area = area_struct(&rect3);
    println!("{}", rect3_area);
}


fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) ->  u32{
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
