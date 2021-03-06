#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
use reqwest::Url;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let str = "Hello, world!".to_string();
    
    let url = "https://github.com/JJins/jjins.github.io/raw/master/mugger.json".to_string();

    println!("{}", &str);
    
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
    
    //let res = reqwest::get(url).await?;
}
