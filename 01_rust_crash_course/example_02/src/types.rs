fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn run() {
    // Primitive types
    /*
    Integers:   u8, i8, ... , u128, i128
    Floats:     f32, f64
    Boolean:    bool
    Character:  char
    Tuples:     (,)
    Arrays:     [,]
    */
    // Integers
    println!("Max for i32: {}", std::i32::MAX);
    let x: i64 = 2147483648;
    print_type_of(&x);

    // Strings and Chars
    let string = "Hello World!";
    let character = 'c';
    let unicode_char = '\u{1F600}';
    println!("{} {} {}", string, character, unicode_char);
    let mut string_builder = String::from("Hello"); // alloc::string::String
    println!("{}", string_builder);
    string_builder.push_str(" World!");
    println!("{}", string_builder);
    for word in string.split_whitespace() {
        println!("{}", word);
    }
}
