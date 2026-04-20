use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use super::urgencia::Urgencia;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "lowercase")]
pub enum EstadoSolicitud {
    Pendiente,
    Aceptado,
    Terminado,
    Cancelado,
    EnEsperaDePago,
}

impl EstadoSolicitud {
    pub fn desde_cadena(cadena: &str) -> Option<Self> {
        match cadena.to_lowercase().as_str() {
            "pendiente" => Some(Self::Pendiente),
            "aceptado" => Some(Self::Aceptado),
            "terminado" => Some(Self::Terminado),
            "cancelado" => Some(Self::Cancelado),
            "en_espera_de_pago" => Some(Self::EnEsperaDePago),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SolicitudServicio {
    pub id: Option<i32>,
    pub usuario_id: i32,
    pub servicio_id: i32,
    pub urgencia: Urgencia,
    pub precio_final: Decimal,
    pub estado: EstadoSolicitud,
    pub latitud_usuario: Option<Decimal>,
    pub longitud_usuario: Option<Decimal>,
    pub fecha_creacion: Option<DateTime<Utc>>,
}
