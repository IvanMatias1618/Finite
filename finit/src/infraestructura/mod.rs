pub mod mysql_repositorio_usuario;
pub mod mysql_repositorio_colaborador;
pub mod mysql_repositorio_servicio;
pub mod mysql_repositorio_solicitud;
pub mod sqlite_repositorio;
pub mod api;

use sqlx::MySqlPool;

pub struct RepositorioMySQL {
    pub pool: MySqlPool,
}

impl RepositorioMySQL {
    pub fn nuevo(pool: MySqlPool) -> Self {
        Self { pool }
    }
}
