use serde::{Deserialize, Serialize};
use crate::dominio::servicio::Servicio;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Colaborador {
    pub id: Option<i32>,
    pub usuario_id: i32,
    pub telefono: String,
    pub sitio_web: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerfilColaborador {
    pub id: i32,
    pub nombre: String,
    pub telefono: String,
    pub sitio_web: Option<String>,
    pub servicios: Vec<Servicio>,
}
