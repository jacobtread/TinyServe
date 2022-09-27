use std::path::Path;
use actix_files::NamedFile;
use actix_web::{get, App, Error, HttpServer, Responder};
use actix_web::web::Path as WebPath;

#[get("/{path:.*}")]
async fn index(path: WebPath<String>) -> Result<impl Responder, Error> {
    let public_path = Path::new("public");
    // Join the public path to the user provided path
    let file_path = public_path.join(path.into_inner());

    // Handle 404 Page
    if !file_path.exists() {
        let _404_page = public_path.join("404.html");
        if _404_page.exists() {
            return Ok(NamedFile::open(_404_page));
        }
    }

    // Directory index serving
    if file_path.is_dir() {
        let index_path = file_path.join("index.html");
        // If index exists serve instead.
        if index_path.exists() {
            return Ok(NamedFile::open(index_path));
        }
    }
    Ok(NamedFile::open(file_path))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Started serving on http://localhost:8080");
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}