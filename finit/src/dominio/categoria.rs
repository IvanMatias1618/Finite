use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Categoria {
    pub id: Option<i32>,
    pub nombre: String,
}
