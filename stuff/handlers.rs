use crate::useful::*;
use actix_web::HttpResponse;
use actix_web::web::{self, Form};

fn ccgen_post(query: Form<models::ExtraInfo>)  -> HttpResponse  {
    let models::ExtraInfo {cc1, cc2} = query.into_inner();
    models::ExtraInfo::insert(&cc1, &cc2);

    HttpResponse::Ok().body("Success")
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/extrapolar").route(web::post().to(ccgen_post)));
}