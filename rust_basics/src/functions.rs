// Functions - Used to store blocks of code for re-use

pub fn run(){
    greeting("Hello", "Jabir");

    // Bind function values to variable
    let sum = add(5, 5);
    println!("Sum is: {}", sum);

    println!("Sum: {}", add(5,6));

    // Closure
    let n3: i32 = 5;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure Sum: {}", add_nums(3, 4));

}

fn greeting(greet: &str, name: &str){
    println!("{} {} , nice to meet you,", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}