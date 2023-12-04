enum Operation {
    Add,
    Mul,
}

fn get_implementation(operation: Operation, operand: i32) -> Box<dyn Fn(i32) -> i32> {
    match operation {
        Operation::Add => Box::new(move |x| x + operand),
        Operation::Mul => Box::new(move |x| x * operand),
    }
}

fn main() {
    const OPERNAD: i32 = 5;
    let adder = get_implementation(Operation::Add, OPERNAD);
    let muliplier = get_implementation(Operation::Mul, OPERNAD);
    assert_eq!(adder(10), 15);
    assert_eq!(muliplier(10), 50);
}
