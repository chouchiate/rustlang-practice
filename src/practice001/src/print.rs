pub fn run() {
    //print to console
    println!("Hello World from print.rs file!");
    // Basic Formatting
    println!("Number: {}", 2);

    println!("{} is from {}", "Meow", "Cat");
    //Positional Arguments\
    println!("{0} is from {1} and {0} likes to {2}", "Meow", "Cat", "eat");
    //Named arguments
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "Baseball"
    );
    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octals {:o}", 10, 10, 10);
    
    // Placeholder for debug trait
    println!("{:?}",(12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10)
}
