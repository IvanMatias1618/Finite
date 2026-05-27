use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MensajeSolicitud {
    pub id: Option<i32>,
    pub solicitud_id: i32,
    pub emisor_id: i32, // ID del usuario (ya sea el cliente o el usuario del colaborador)
    pub contenido: String,
    pub fecha_envio: Option<DateTime<Utc>>,
}
