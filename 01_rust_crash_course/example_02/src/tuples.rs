pub fn run() {
    // Tuples
    println!("--- Tuples ---");

    let person = ("Brad", "Mass", 37);
    let person_again: (&str, &str, i8) = ("Brad", "Mass", 37);
    println!("{:?}", person);
    println!("{:?}", person_again);
    println!("{} | {} | {}", person.0, person.1, person.2); // indexing

    // Arrays
    println!("--- Arrays ---");

    let mut arr1 = [1, 2, 3, 4, 5, 6];
    arr1[0] = 42; // Assignment
    println!("{:?}", arr1);

    let arr2: [i32; 5] = [1, 2, 3, 4, 5]; // Same element N times
    let arr3: [i32; 3] = [10; 3];
    println!("{:?}", arr1);
    println!("{:?}", arr2);
    println!("{:?}", arr3);

    let arr4: [[i32; 3]; 3] = [[1; 3]; 3]; // 2D array
    println!("{:?}", arr4);
    println!("{:?}", arr4[2][2]); // Indexing (&arr4[2][2] also works)
    println!("{:?}", arr4.len()); // Length
    println!(
        "[[i32; 3] is {:?} bytes (i32*3*3 = 4bytes*3*3)",
        std::mem::size_of_val(&arr4)
    );

    let slice: &[i32] = &arr1[1..3]; // Slice (reference)
    println!("{:?}", slice);

    // Vectors
    println!("--- Vectors ---");

    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
    println!("{:?}, {}", numbers, numbers.len());
    numbers.push(1337);
    println!("{:?}", numbers);
    numbers.pop();
    println!("{:?}", numbers);
    // Iteration
    for x in numbers.iter() {
        println!("Number: {}", x);
    }
    // Mutable iteration
    for x in numbers.iter_mut() {
        *x *= 2
    }
    println!("{:?}", numbers);
}
