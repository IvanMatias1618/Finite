use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Usuario {
    pub id: Option<i32>,
    pub nombre: String,
    pub correo: String,
    pub contrasenna: String,
}
