use crate::dominio::puertos::repositorio_soporte::RepositorioSoporte;
use crate::infraestructura::RepositorioMySQL;
use std::error::Error;
use async_trait::async_trait;

#[async_trait]
impl RepositorioSoporte for RepositorioMySQL {
    async fn guardar_reporte(&self, usuario_id: i32, descripcion: String, fotos: Option<String>) -> Result<i32, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query(
            "INSERT INTO reporte_soporte (usuario_id, descripcion, fotos_evidencia) VALUES (?, ?, ?)"
        )
        .bind(usuario_id)
        .bind(descripcion)
        .bind(fotos)
        .execute(&self.pool)
        .await?;

        Ok(resultado.last_insert_id() as i32)
    }
}
