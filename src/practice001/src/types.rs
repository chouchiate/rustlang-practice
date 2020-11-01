    /*
    primitive types --
    integars: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, (number of bits they take in memory)
    floats: f32, f64
    Boolean (bool)
    Charaters (char)
    Tuples
    Arrays
    Vectors
    */

    //Rust is a statically typed language, which means that it must know the types of all variables at compile time, 
    //however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run(){
    // default is "i32"
    let _x = 1;

    // default is "f64"
    let _y = 2.5;

    // Add explicit type
    let _z: i64 = 4477272932;

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i32: {}", std::i64::MAX);

    // Get boolean from expression
    let is_greater : bool = 10 < 5;

    // Boolean
    let is_active: bool = true;

    let a1 = 'a';

    let face = '\u{1F600}';

    println!("{:?}", (_x,_y,_z, is_active, is_greater, a1, face));
}