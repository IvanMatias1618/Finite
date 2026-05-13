use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use super::urgencia::Urgencia;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq)]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "lowercase")]
pub enum EstadoSolicitud {
    PendienteDeRevision,
    AceptadoPorColaborador,
    CitaProgramada,
    Terminado,
    Cancelado,
    EnEsperaDePago,
    Pagado,
}

impl EstadoSolicitud {
    pub fn desde_cadena(cadena: &str) -> Option<Self> {
        match cadena.to_lowercase().as_str() {
            "pendiente_de_revision" => Some(Self::PendienteDeRevision),
            "aceptado_por_colaborador" => Some(Self::AceptadoPorColaborador),
            "cita_programada" => Some(Self::CitaProgramada),
            "terminado" => Some(Self::Terminado),
            "cancelado" => Some(Self::Cancelado),
            "en_espera_de_pago" => Some(Self::EnEsperaDePago),
            "pagado" => Some(Self::Pagado),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SolicitudServicio {
    pub id: Option<i32>,
    pub usuario_id: i32,
    pub colaborador_id: i32,
    pub subcategoria_id: i32,
    pub servicio_id: i32,
    pub urgencia: Urgencia,
    pub precio_final: Decimal,
    pub estado: EstadoSolicitud,
    pub descripcion_detallada: String,
    pub fotos_evidencia_inicial: Option<String>,
    pub latitud_usuario: Option<Decimal>,
    pub longitud_usuario: Option<Decimal>,
    pub conekta_order_id: Option<String>,
    pub fecha_creacion: Option<DateTime<Utc>>,
}
