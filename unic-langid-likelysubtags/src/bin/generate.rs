use serde_json::{Result, Value};
use std::fs;

fn main() {
    let contents = fs::read_to_string("./data/likelySubtags.json")
        .expect("Something went wrong reading the file");
    let data = "[0, 1]";
    let v: Value = serde_json::from_str(&contents).unwrap();
    let values = v["supplemental"]["likelySubtags"].as_object().unwrap();

    println!("pub const LANG_ONLY: &[(&str, &str)] = &[");
    for (k, v) in values {
        if k.contains("-") {
            continue;
        }
        println!("   (\"{}\", \"{}\"),", k, v.as_str().unwrap());
    }
    println!("];");
}
