#![allow(unreachable_code)]

// 'label: 로 n 중 루프를 컨트롤 할 수 있음!
// 불필요한 변수 사용안해도됨!
fn nesting_and_labels() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // inner loop만 break
            // break;

            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}
