use dotenv::dotenv;
use std::env;
use actix_files::NamedFile;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpServer, Responder, Result};
use env_logger;
use log::info;
use serde::Serialize;

#[derive(Serialize)]
struct DemandCurveEntry {
    start: String,
    value: u32
}

#[get("/entries")]
async fn get_curve_entries()  -> impl Responder {
    let mut vec:Vec<DemandCurveEntry> = Vec::new();

    vec.push(DemandCurveEntry{start: "2022-08-10T10:00:00.000".to_string(), value: 5});
    vec.push(DemandCurveEntry{start: "2022-08-10T12:00:00.000".to_string(), value: 10});
    vec.push(DemandCurveEntry{start: "2022-08-10T13:05:14.135".to_string(), value: 25});

    return web::Json(vec);
}

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("index.html")?)
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port:u16 = std::env::var("PORT").map_or(8080, |p| p.parse().expect("Could not parse PORT property"));
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    info!("Starting web server on port {} ...", port);
    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(Logger::default())
            .service(greet)
            .service(get_curve_entries)
            .route("/", web::get().to(index))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
