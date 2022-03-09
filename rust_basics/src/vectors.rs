pub fn run(){
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // 

    println!("{:?}", numbers);

    // Get single value
    println!("Single Value: {}", numbers[0]);

    // Get array length
    println!("Vector length: {}", numbers.len());

    // Vector stack allocation
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter(){
        println!("Numbers in vec: {}", x );
    }

    // Loop through vector values
    for x in numbers.iter_mut(){
       *x *=2; //multiply each numbers in vector with 2
    }
    println!("Numbers Vec: {:?}", numbers);





}