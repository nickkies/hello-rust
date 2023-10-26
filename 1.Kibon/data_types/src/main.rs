fn main() {
    // boolean
    let _b1: bool = true;

    // unsigned integer

    let _i1: u8 = 255;
    let _i2: u16 = 65_535;
    let _i3: u32 = 4_294_967_295;
    let _i4: u64 = 18_446_744_073_709_551_615;
    let i5: u128 = u128::MAX;

    // signed integer
    let _i6: i8 = -128;
    let _i7: i16 = -32_768;
    let _i8: i32 = -2_147_483_648;
    let _i9: i64 = -9_223_372_036_854_775_808;
    let i10: i128 = i128::MIN;

    // floating point numbers
    let f1: f32 = f32::MAX;
    let f2: f64 = f64::MIN;

    println!("i5: {i5}\ni10: {i10}");
    println!("f1: {f1}\nf2: {f2}");

    // platforms specific number
    let _p1: usize = usize::MAX;
    let _p2: isize = isize::MIN;

    // characters, &str and String
    let _c1: char = 'a';
    let _s1: &str = "hello";
    let _s2: String = String::from("world");

    // arrays
    let _a1: [i32; 5] = [1, 2, 3, 4, 5];

    let _i1 = _a1[4];

    // tuples
    let t1: (u8, u8, u8) = (1, 2, 3);
    let _t2: (u8, f32, char) = (1, 2.0, '3');

    let _s1 = t1.2;
    let (_i1, _f1, _c1) = t1;

    let _unit = ();

    // Type aliases
    type Age = u8;
    let _a1: Age = 32;
}
