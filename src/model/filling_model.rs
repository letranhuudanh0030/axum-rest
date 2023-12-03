use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FillingModel {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateFillingModel {
    pub name: String,
}

#[derive(Debug, Deserialize)]

pub struct UpdateFillingModel {
    pub name: String,
}
