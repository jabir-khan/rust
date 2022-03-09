// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run(){
    let name = "Jabir";
    let mut age = 37;
    println!("My name is {} and I am  {}", name, age);
    age = 38;
    println!("My name is {} and I am  {}", name, age);

    // Define constant
    // while using const we do need to specify the data type of const
    const ID: i32 = 001;
    println!("ID:{}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Jabir",37);
    println!("{} is {}", my_name, my_age);

    
 
}