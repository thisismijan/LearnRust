use actix_files::Files; 
use actix_web::{dev::Server, middleware, web, App, HttpResponse, HttpServer};
use std::net::TcpListener;
use tera::Tera;

pub mod constants;
pub mod handlers;
pub mod models;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::new("templates/**/*.html").expect("Unable to unwrap the tera template");
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

pub fn start_blog(listener: TcpListener) -> Result<Server, std::io::Error> {
    // move keyword converts any variables captures by reference or mutable reference to variables captures by value
    let srv = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(TEMPLATES.clone()))
            .wrap(middleware::Logger::default())
            .service(Files::new("/static", "static/").use_last_modified(true)) // new line
            .route("/health", web::get().to(HttpResponse::Ok))
            .service(handlers::index)
            .service(handlers::post) 
    })
    .listen(listener)?
    .run();

    Ok(srv)
}