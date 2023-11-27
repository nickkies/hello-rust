fn get_last(nums: &mut Vec<i32>) -> Result<i32, &str> {
    if nums.len() == 0 {
        return Err("Empty vector");
    }
    Ok(nums.pop().unwrap())
}

fn main() {
    let mut vec1 = vec![1, 2, 3];
    let mut vec2 = vec![];

    assert!(matches!(get_last(&mut vec1), Ok(3)));
    assert!(matches!(get_last(&mut vec2), Err("Empty vector")));
}
