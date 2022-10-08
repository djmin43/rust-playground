fn main() {
    let value = value_in_cents(Coin::Dime);
    println!("{}", value);
}

fn value_in_cents (coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}