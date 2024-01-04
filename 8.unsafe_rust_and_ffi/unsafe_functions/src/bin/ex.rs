unsafe fn increment(a: *mut i32) {
    *a += 1;
}

unsafe fn get_val(a: *const i32) -> i32 {
    *a
}

fn main() {
    let mut x = 0;
    let ptr = &mut x as *mut i32;

    unsafe {
        increment(ptr);
        increment(ptr);
        increment(ptr);
        assert_eq!(get_val(ptr), 3);
    }
}
