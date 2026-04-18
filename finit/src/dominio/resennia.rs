use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Resennia {
    pub id: Option<i32>,
    pub solicitud_id: i32,
    pub calificacion: i8,
    pub comentario: Option<String>,
    pub fecha_creacion: Option<DateTime<Utc>>,
}
