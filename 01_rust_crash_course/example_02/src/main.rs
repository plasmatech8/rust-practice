mod print;
mod vars;

fn main() {
    println!("Hello, world!");
    println!();

    // Use module functions

    print::run();
    println!();

    vars::run();
    println!();
}
