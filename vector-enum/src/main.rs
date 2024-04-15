fn main() {
    println!("Using enum to hold multiple types using vector");

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell :: Int(3),
        SpreadsheetCell :: Text(String::from("blue")),
        SpreadsheetCell :: Float(10.78),
    ];

    println!("vec {:?}", row);

}
