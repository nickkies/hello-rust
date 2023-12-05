fn decrement(x: u32) -> u32 {
    x - 1
}

fn multiply(x: u32, y: u32) -> u32 {
    x * y
}

fn factorial(num: u32, dec: fn(u32) -> u32, mul: fn(u32, u32) -> u32) -> u32 {
    let mut res = 1;
    let mut tmp = num;
    while tmp > 1 {
        res = mul(res, tmp);
        tmp = dec(tmp);
    }
    res
}

fn main() {
    let num = 6;
    let fact = factorial(num, decrement, multiply);
    println!("{num}! = {fact}");
}
