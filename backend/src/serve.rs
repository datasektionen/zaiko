use actix_files::NamedFile;
use actix_web::{get, HttpRequest, HttpResponse, Responder};

#[get("/{file:.*}")]
pub(crate) async fn serve_frontend(req: HttpRequest) -> impl Responder {
    match NamedFile::open("dist/index.html") {
        Ok(file) => file.into_response(&req),
        Err(_) => {
            // In development mode with watch, dist/ may not exist yet
            // Return a temporary message while Vite dev server is starting
            HttpResponse::ServiceUnavailable()
                .body("Frontend is starting up. Please refresh in a moment.")
        }
    }
}
