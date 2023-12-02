use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CakeModel {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCakeModel {
    pub name: String,
}

#[derive(Debug, Deserialize)]

pub struct UpdateCakeModel {
    pub name: String,
}
