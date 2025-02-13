use actix_files::NamedFile;
use actix_identity::Identity;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

#[get("/{file}")]
pub(crate) async fn serve_frontend(req: HttpRequest, id: Option<Identity>, auth_url: web::Data<String>) -> impl Responder {
    if id.is_some() {
        NamedFile::open("../dist/index.html").unwrap().into_response(&req)
    } else {
        HttpResponse::TemporaryRedirect().insert_header(("location", auth_url.to_string())).finish()
    }
}
