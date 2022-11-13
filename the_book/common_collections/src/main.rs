
fn main() {

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Float(3.3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Int(2342)
    ];


    let first = &row[0];

    println!("{:?}", first);

}
