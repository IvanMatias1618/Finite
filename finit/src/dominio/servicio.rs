use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Servicio {
    pub id: Option<i32>,
    pub colaborador_id: i32,
    pub categoria_id: i32,
    pub descripcion: String,
    pub distancia_maxima_kilometros: Decimal,
    pub precio_por_kilometro: Decimal,
    pub latitud: Decimal,
    pub longitud: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrecioServicioUrgencia {
    pub id: Option<i32>,
    pub servicio_id: i32,
    pub urgencia: super::urgencia::Urgencia,
    pub precio: Decimal,
}
