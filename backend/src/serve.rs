use actix_files::NamedFile;
use actix_web::{get, HttpRequest, Responder};

#[get("/{file}")]
pub(crate) async fn serve_frontend(req: HttpRequest) -> impl Responder {
    NamedFile::open("../dist/index.html")
        .unwrap()
        .into_response(&req)
}
