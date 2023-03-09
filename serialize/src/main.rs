use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();

    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    println!("deserialized = {:?}", deserialized);

    let full_name = "John Doe";
    let age_last_year = 42;

    let john = serde_json::json!({
        "name": full_name,
        "age": age_last_year + 1,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    println!("first phone number: {}", john["phones"][0]);
    println!("{}", john.to_string());
}
