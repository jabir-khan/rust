// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run(){
    // Primitive str
    // let hello = "Hello";
    // String
    let mut hello  = String::from("Hello ");

    // Get length of string
    println!("length of string is: {}", hello.len());

    // Push char
    hello.push('W');
    
    // Push multiple string
    hello.push_str("orld");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Is empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace(){
        println!("{}", word);
    }
    
    // Create string with capacity 
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //Asssertion testting
    assert_eq!(2, s.len()); //check if the length of s is 2
    assert_eq!(10, s.capacity()); // check id the s capacity is 10
   

    println!("{}", hello);

}