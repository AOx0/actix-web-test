use crate::useful::*;
use actix_web::HttpResponse;
use actix_web::web::{self, Form};

fn handle_post(query: Form<models::ExtraInfo>) -> HttpResponse  {
    let models::ExtraInfo {cc1, cc2} = query.into_inner();
    models::ExtraInfo::insert(&cc1, &cc2);

    HttpResponse::Ok().body("Success")
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/somepost").route(web::post().to(handle_post)));
}