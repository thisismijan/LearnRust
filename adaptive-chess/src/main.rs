
mod app;
mod logic;
mod model;
mod cache;

fn main() {
    std::env::set_var("RUST_LOG", "adaptive-chess=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    pretty_env_logger::init();
        app::AppRunner::run();

}