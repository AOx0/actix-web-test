use crate::prelude::*;
use crate::diesel_stuff::schema::*;
use diesel::{Insertable, Queryable};

use crate::diesel::*;

#[derive(Insertable, Queryable, Debug)]
#[table_name= "extra_info"]
pub struct ExtraInfo {
    pub id_request: i32,
    pub cc1: String,
    pub cc2: String
}


impl ExtraInfo {
    pub fn insert(in_cc1: &str, in_cc2: &str) {
        use crate::diesel_stuff::schema::extra_info::dsl::*;
        use crate::diesel_stuff::schema::extra_info as e_i;
        let db = connect();

        let other_id_request = e_i::table.order(id_request.desc()).select(id_request).first::<i32>(&db).unwrap() + 1;

        diesel::insert_into(extra_info)
            .values(ExtraInfo { id_request: other_id_request, cc1: in_cc1.to_string(), cc2: in_cc2.to_string()  })
            .execute(&db).unwrap()
        ;
    }
}

