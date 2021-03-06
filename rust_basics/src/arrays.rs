pub fn run(){
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Get single value
    println!("Single Value: {}", numbers[0]);

    // Get array length
    println!("Array length: {}", numbers.len());

    // Array stack allocation
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);




}