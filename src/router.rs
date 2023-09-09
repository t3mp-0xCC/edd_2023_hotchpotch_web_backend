use actix_web::{
    web,
    get, post, delete,
    Error,
    HttpResponse,
    HttpRequest,
};
use serde::Deserialize;

use crate::{auth, cruds};

#[get("/")]
async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body("0w0"))
}

// Structs
#[derive(Deserialize)]
struct CreateEventReqBody {
    name: String,
    desc: String,
    url: String,
}

#[derive(Deserialize)]
struct CreateSoloReqBody {
    event_id: String,
}

#[derive(Deserialize)]
struct CreateTeamReqBody {
    event_id: String,
    name: String,
    desc: String,
}

#[derive(Deserialize)]
struct CreateJoinReqBody {
    team_id: String,
}

#[derive(Deserialize)]
struct CreateRequestReqBody {
    team_id: String,
    user_id: String,
    message: String,
}

#[derive(Deserialize)]
struct UserIdQuery {
    user_id: Option<String>,
}

#[derive(Deserialize)]
struct EventIdQuery {
    event_id: String,
}

#[derive(Deserialize)]
struct TeamIdQuery {
    team_id: String,
}

// API
#[post("/api/users")]
async fn create_user(req: HttpRequest) -> Result<HttpResponse, Error> {
    match auth::parse_token(req) {
        Some(token) => {
            match auth::verification(token).await {
                Ok(user_data) => {
                    println!("GitHub User Data: {:?}", user_data);
                    let empty = String::new();
                    match cruds::create_user(&user_data.login, &user_data.avatar_url, &empty) {
                        Ok(_) => return Ok(HttpResponse::Created().content_type("text/html").body("0w0")),
                        Err(_) => return Ok(HttpResponse::InternalServerError().finish())
                    };
                },
                Err(_) => return Ok(HttpResponse::Unauthorized().finish())
            }
        },
        None => return Ok(HttpResponse::Unauthorized().finish())
    };
}

#[get("api/users")]
async fn get_user(query: web::Query<UserIdQuery>) -> Result<HttpResponse, Error> {
    if query.user_id.is_none() {
        // queryなし
        return Ok(HttpResponse::Ok().content_type("text/html").body("0u0"));
    } else {
        // queryあり
        return Ok(HttpResponse::Ok().content_type("text/html").body("0w0"))
    }
}

#[post("/api/events")]
async fn create_event(body: web::Json<CreateEventReqBody>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body("0w0"))
}

#[get("/api/events")]
async fn get_event() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body("0w0"))
}

#[delete("/api/events")]
async fn delete_event(query: web::Query<EventIdQuery>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body("0w0"))
}

#[post("/api/solos")]
async fn create_solo(body: web::Query<CreateSoloReqBody>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body("0w0"))
}

#[get("/api/solos")]
async fn get_solo(query: web::Query<EventIdQuery>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body("0w0"))
}

#[post("/api/teams")]
async fn create_team(body: web::Json<CreateTeamReqBody>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body("0w0"))
}

#[get("/api/teams")]
async fn get_team(query: web::Query<TeamIdQuery>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body("0w0"))
}

#[get("/api/teams/event")]
async fn get_team_by_event(query: web::Query<EventIdQuery>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body("0w0"))
}

#[post("/api/joins")]
async fn create_join(body: web::Query<CreateJoinReqBody>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body("0w0"))
}

#[post("/api/requests")]
async fn create_request(body: web::Json<CreateRequestReqBody>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body("0w0"))
}

#[get("/api/requests")]
async fn get_request() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body("0w0"))
}