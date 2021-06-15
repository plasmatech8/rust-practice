mod print;
mod types;
mod vars;

fn main() {
    println!("Hello, world!");
    println!();

    // Use module functions

    print::run();
    println!();

    vars::run();
    println!();

    types::run();
    println!();
}
