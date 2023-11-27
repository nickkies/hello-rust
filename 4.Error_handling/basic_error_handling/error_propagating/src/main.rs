fn factorial(n: u32) -> Result<u32, String> {
    if n == 0 {
        return Ok(1);
    } else if n > 12 {
        return Err("Input too large".to_string());
    }
    let result = n * factorial(n - 1)?;
    Ok(result)
}

fn print_factorial(n: u32) -> Result<(), String> {
    let result = factorial(n)?;
    println!("Factorial of {n} is {result}");
    Ok(())
}

fn main() {
    let n = 13;
    if let Err(err) = print_factorial(n) {
        eprintln!("Error calculating factorial of {n}: {err}");
    }
}
