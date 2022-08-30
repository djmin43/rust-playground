use std::io;

fn main() {
//     Scalar(Primitive?) and Compound(Reference)
//     Scalar: integer, floating-point numbers, booleans, characters

    // let x= 1.0;
    // let x = x + 9.5;
    // let y: f32 = 3.0;
    //
    // let sum = 5 + 10;
    //
    // let difference = 95.5 - 4.3;

    let floored = 2.0/3.0;
    println!("the value is {}", floored);


//     Compound Types: two types

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is {x}, {y}, {z}");
    println!("{floored}");


    let a = [1,2,3,4,5 ];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");


}
