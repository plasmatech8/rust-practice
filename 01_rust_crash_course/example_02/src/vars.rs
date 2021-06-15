pub fn run() {
    // Immutable/mutable variables
    let name = "Brad";
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);
    age = 38;
    println!("My name is {} and I am {}", name, age);

    // Contants
    const ID: i32 = 2;
    println!("ID: {}", ID);

    // Assign multiple vars (unpacking)
    let (my_name, my_age) = ("Brad", 37);
    println!("{}, {}", my_name, my_age)
}
