// tuples group together vaules of different types
// max 12 elements\


pub fn run(){
    let person: (&str, &str, i8) = ("Derick", "Cool", 40);
    println!("{} is from {} and is {}", person.0, person.1, person.2);
}