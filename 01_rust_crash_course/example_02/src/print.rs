pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Print Formatting
    println!("Number: {}", 1);
    println!("Number: {} {} {}", 0, 1, 2);
    println!("Number: {1} {0} {1}", 0, 1);
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "Basketball"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
    println!("Pi: {:.3}", 3.14159265359);

    // Print an object
    println!("{:?}", (12, true, "hello"))
}
