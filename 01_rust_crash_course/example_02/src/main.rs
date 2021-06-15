mod print;
mod tuples;
mod types;
mod vars;

fn main() {
    println!("Hello, world!");
    println!();

    // Use module functions

    println!("========== Print ==========");
    print::run();

    println!("========== Vars ==========");
    vars::run();

    println!("========== Types ==========");
    types::run();

    println!("========== Tuples/Array ==========");
    tuples::run();
}
