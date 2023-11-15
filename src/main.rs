use std::net::TcpListener;

use dev_log::start_blog;

// this is a macro, specifically a proc macro which makes the blog run asynchronously
#[actix_web::main]
// the main method which denotes the start point of the app, it returns a Result type
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    // if method return type, no need for return statement if don't use semi-colon
    start_blog(listener)?.await
}