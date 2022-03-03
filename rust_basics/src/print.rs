pub fn run(){
    // normal print
    println!("hello from print");

    // Basic Formatting
    println!("{}  is from  {}", "Jabir", "Nepal");

    // Positional Argument
    println!("{0} is from {1} and {0} likes to {2}.", "Jabir" , "Nepal", "code");

    // Named Arguments
    println!("{name} likes to play {activity}.", name="Jabir", activity="football");

    // Placeholder traits
    println!("Binary: {:b} Hex {:x} Octal: {:0}", 10,10,10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!(" 10 + 10 = {} ", 10+10);


}