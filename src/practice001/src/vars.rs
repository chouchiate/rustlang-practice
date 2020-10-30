// variables hold primitive data or reference to data
// variables are immutable by default
// rust is a block-scoped language

pub fn run (){
    let name = "Brad";
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);
    age = 38;
    println!("My name is {} and I am {}", name, age);

    // define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);
    
}