use actix_session::Session;
use actix_web::{get, HttpResponse, Responder, web};
use crate::add_player;
use crate::players::Players;

pub(crate) fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(me)
            .service(online)
            .service(rename)
    );
}

#[get("/me")]
async fn me(session: Session, data: web::Data<Players>) -> HttpResponse {
    let players = data.get_ref();
    let session_id = session.get::<u64>("id");
    match session_id {
        Ok(Some(id)) => if players.contains(id) {
            HttpResponse::Ok().body(format!("User id: {id}"))
        } else {
            add_player(session, players)
        },
        Ok(None) => add_player(session, players),
        Err(e) => HttpResponse::InternalServerError().
            body(format!("Failed to set user id {e}"))
    }
}

#[get("/online")]
async fn online(data: web::Data<Players>) -> HttpResponse {
    HttpResponse::Ok().json(data.get_ref().player_list())
}

#[get("/rename/{name}")]
async fn rename(session: Session, data: web::Data<Players>, name: web::Path<String>) -> impl Responder {
    let players = data.get_ref();
    let session_id = session.get::<u64>("id");
    match session_id {
        Ok(Some(id)) => if players.contains(id) {
            match players.rename(id, name.into_inner()) {
                Ok(_) => HttpResponse::Ok().body("Name updated"),
                Err(e) => HttpResponse::InternalServerError().body(
                    format!("Failed to update name: {:?}", e)
                )
            }
        } else {
            HttpResponse::Unauthorized().body("User not logged in yet")
        },
        Ok(None) => HttpResponse::Unauthorized().body("User not logged in yet"),
        Err(e) => HttpResponse::InternalServerError().body(
            format!("Failed to set user id: {e}")
        )
    }
}
