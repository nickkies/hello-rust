use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;

fn convert_file(file_path: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let mut json_file = File::open(file_path)?;
    let mut json_data = String::new();
    json_file.read_to_string(&mut json_data)?;

    let data: Value = serde_json::from_str(&json_data)?;

    let array = data["array"]
        .as_array()
        .ok_or("Failed to get array from JSON")?;

    let res = array
        .iter()
        .flat_map(|arr| arr.as_array().unwrap().iter().cloned())
        .map(|v| {
            if let Value::Number(num) = v {
                num.as_f64().unwrap_or_default() as f32
            } else {
                0.0
            }
        })
        .collect::<Vec<f32>>();

    Ok(res)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arr = ["u", "v"];

    for d in arr {
        let x = convert_file(&format!("data/{}.json", d))?;
        let mut bin_file = File::create(format!("data/{}.bin", d))?;
        bin_file.write_all(bytemuck::cast_slice(&x))?;
    }

    Ok(())
}
