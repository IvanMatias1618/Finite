use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Colaborador {
    pub id: Option<i32>,
    pub usuario_id: i32,
    pub telefono: String,
    pub sitio_web: Option<String>,
}
