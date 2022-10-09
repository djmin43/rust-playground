fn main() {
    let config_max = Some(100u32);
    match config_max {

        Some(max) => println!("the maximum is configured to be {}", max),
        _ => (),
    }
    if let Some(test) = config_max {
        println!("the maximum is {}", test);
    }

}
