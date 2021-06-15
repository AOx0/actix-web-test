
use diesel::dsl::max;
use crate::actix_stuff::models::CC;
use actix_web::HttpResponse;
use actix_web::web::{self, Form};

use serde::{Serialize, Deserialize};

use crate::diesel_stuff::models::*;
use crate::diesel_stuff::schema::*;
use crate::prelude::*;


fn ccgen_post(query: Form<CC>)  -> HttpResponse  {
    let query = query.into_inner();
    let CC {cc1, cc2} = query;
    println!("{}\n{}", cc1, cc2);
    //Do something with your data


    let max_id = ExtraInfo::insert(&cc1, &cc2);

    HttpResponse::Ok()
        .body("jasda")
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/extrapolar").route(web::post().to(ccgen_post)));
}