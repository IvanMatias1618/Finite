use serde::{Deserialize, Serialize};
use crate::dominio::servicio::Servicio;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Colaborador {
    pub id: Option<i32>,
    pub usuario_id: i32,
    pub telefono: String,
    pub sitio_web: Option<String>,
    pub foto_perfil: Option<String>,
    pub especialidad_resumen: Option<String>,
    pub es_verificado: bool,
    pub medio_transporte: Option<String>,
    #[sqlx(default)]
    pub rating_promedio: Decimal,
    pub total_servicios: i32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TrabajoPortafolio {
    pub id: Option<i32>,
    pub colaborador_id: i32,
    pub foto_antes: String,
    pub foto_despues: String,
    pub descripcion: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerfilColaborador {
    pub id: i32,
    pub nombre: String,
    pub telefono: String,
    pub sitio_web: Option<String>,
    pub foto_perfil: Option<String>,
    pub especialidad_resumen: Option<String>,
    pub es_verificado: bool,
    pub medio_transporte: Option<String>,
    pub rating_promedio: Decimal,
    pub total_servicios: i32,
    pub servicios: Vec<Servicio>,
    pub portafolio: Vec<TrabajoPortafolio>,
}
