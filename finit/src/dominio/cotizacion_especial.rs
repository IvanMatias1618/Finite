use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use super::urgencia::Urgencia;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CotizacionEspecial {
    pub id: Option<i32>,
    pub usuario_id: i32,
    pub descripcion_trabajo: String,
    pub fotos_evidencia: Option<String>, // JSON string de lista de rutas
    pub presupuesto_estimado: Option<Decimal>,
    pub nivel_urgencia: Urgencia,
    pub fecha_creacion: Option<DateTime<Utc>>,
}
