
mod generic_types;
mod traits;
mod lifetimes;

fn main() {
    println!("Generic Types");
    generic_types::run();
    println!("Traits");
    traits::run();
    println!("Lifetimes");
    lifetimes::run();
}
