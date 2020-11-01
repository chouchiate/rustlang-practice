// primitive str = immutable fixed-length string somewhere in memory
// string = growable, heap-allocated data structure - use when you need to modify own string data

pub fn run(){
    let mut hello = String::from("Hello");
    // Get length
    println!("Length {}", hello.len());
    println!("{}", hello);
    // push a char
    hello.push('W');
    println!("{}", hello);
    // push a string
    hello.push_str("orld");
    println!("{}", hello);
    // capacity in bytes
    println!("Capacity: {}", hello.capacity());
    // Check if empty
    println!("Is Empty: {}", hello.is_empty());
    // contains
    println!("Contains 'World' {}", hello.contains("World!"));
    // replace
    println!("Replace: {}", hello.replace("World", "There"));
    // loop through string by whitespace
    hello.push_str(", Mate!");
    for word in hello.split_whitespace() {
        println!("{}", word);
    }
    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);
    // assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());


}
