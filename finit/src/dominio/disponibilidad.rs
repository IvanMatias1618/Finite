use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Disponibilidad {
    pub id: Option<i32>,
    pub colaborador_id: i32,
    pub dia_semana: i8, // 0 = Domingo, 1 = Lunes, ..., 6 = Sabado
    pub hora_inicio: String, // Formato HH:MM
    pub hora_fin: String,    // Formato HH:MM
    pub activo: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HorarioSemanal {
    pub colaborador_id: i32,
    pub dias: Vec<Disponibilidad>,
}
