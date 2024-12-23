use actix_files::NamedFile;
use actix_web::{get, Responder};

#[get("/{page}")]
pub(crate) async fn serve_frontend() -> impl Responder {
    NamedFile::open("../dist/index.html")
}
