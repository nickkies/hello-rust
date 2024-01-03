fn main() {
    let num = 123;
    let ptr = &num as *const i32;
    unsafe {
        println!("{} stored at {:p}", *ptr, ptr);
    }
}
