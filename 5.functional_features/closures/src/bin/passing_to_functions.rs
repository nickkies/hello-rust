fn average<T, U>(nums: &[i32], add: T, div: U) -> f32
where
    T: Fn(i32, i32) -> i32,
    U: Fn(i32, i32) -> f32,
{
    let mut sum = 0;
    for num in nums {
        sum = add(sum, *num);
    }
    div(sum, nums.len() as i32)
}

fn main() {
    let add = |x, y| x + y;
    let div = |x, y| x as f32 / y as f32;
    let my_nums = [1, 2, 3, 4, 5];
    let res = average(&my_nums, add, div);
    println!("Average of {my_nums:?} = {res}");
}
