use crate::models::CC;
use actix_web::HttpResponse;
use actix_web::web::{self, Form};

use serde::{Serialize, Deserialize};

fn ccgen_post(query: Form<CC>)  -> HttpResponse  {
    let query = query.into_inner();
    let CC {cc1, cc2} = query;
    println!("{}\n{}", cc1, cc2);
    //Do something with your data

    #[derive(Serialize, Deserialize)]
    struct Respuesta<'a> {
        pub l : &'a str
    }

    HttpResponse::Ok()
        .body("jasda")
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/extrapolar").route(web::post().to(ccgen_post)));
}