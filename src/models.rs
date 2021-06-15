use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CC {
    pub cc1: String,
    pub cc2: String
}
