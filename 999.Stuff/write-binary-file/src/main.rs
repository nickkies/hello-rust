use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let arr = ["u", "v"];

    arr.iter().for_each(|d| {
        let mut json_file =
            File::open(format!("data/{}.json", d)).expect("Failed to open JSON file");
        let mut json_data = String::new();
        json_file
            .read_to_string(&mut json_data)
            .expect("Failed to read JSON file");
        let data: Value = serde_json::from_str(&json_data).expect("Failed to parse JSON");

        let array = data["array"]
            .as_array()
            .expect("Failed to get array from JSON");

        // TODO
        let x: Vec<f64> = array
            .iter()
            .map(|v| {
                if let Value::Number(num) = v {
                    println!("{num}");
                    num.as_f64().unwrap_or_default()
                } else {
                    0.0
                }
            })
            .collect();

        println!("{x:?}");

        let mut bin_file =
            File::create(format!("{}.bin", d)).expect("Failed to create binary file");
        bin_file
            .write_all(bytemuck::cast_slice(&x))
            .expect("Failed to write binary file");
    });
}
