
mod model;
mod logic;

fn main() {
    std::env::set_var("RUST_LOG", "chrs=debug");
    pretty_env_logger::init();
    println!("Hello, world!");
}