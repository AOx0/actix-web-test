use crate::useful::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name= "extra_info"]
pub struct ExtraInfo {
    pub cc1: String,
    pub cc2: String
}

impl ExtraInfo {
    pub fn insert(in_cc1: &str, in_cc2: &str) {
        diesel::insert_into(extra_info::dsl::extra_info)
            .values(ExtraInfo { cc1: in_cc1.to_string(), cc2: in_cc2.to_string()  })
            .execute(&connect()).unwrap()
        ;
    }
}
