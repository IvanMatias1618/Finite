use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Categoria {
    pub id: Option<i32>,
    pub nombre: String,
}
