use chrono::prelude::*;
use json5;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
struct J5 {
    unquoted: String,
    singleQuotes: String,
    lineBreaks: String,
    hexadecimal: i32,
    leadingDecimalPoint: f64,
    andTrailing: f64,
    positiveSign: i32,
    trailingComma: String,
    andIn: Vec<String>,
    backwardsCompatible: String,
    inf: f64,
    ninf: f64,
    nan: f64,
    created: DateTime<Local>,
}
fn main() {
    if let Ok(content) = std::fs::read_to_string("./test.json5") {
        let deserialized: J5 = json5::from_str(&content).unwrap();
        println!("deserialized = {:?}", deserialized);

        let serialized = json5::to_string(&deserialized).unwrap();
        println!("serialized = {}", serialized);
        std::fs::write("./test.write.json5", serialized);
        /*
        if let Ok(mut file) = std::fs::File::create("./test.write.json5") {
//            file.write_all(serialized.as_bytes()).unwrap();
            file.write(serialized.as_bytes()).unwrap();
            std::fs::write("./test.write.json5", serialized);
        }
        */
    } else { println!("ファイル読み込みエラー。"); }
}
