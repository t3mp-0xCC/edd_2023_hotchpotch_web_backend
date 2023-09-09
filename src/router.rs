use actix_web::{
    get,
    Error,
    HttpResponse,
    HttpRequest,
};

#[get("/")]
async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body("0w0"))
}
