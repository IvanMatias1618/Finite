use crate::dominio::resennia::Resennia;
use crate::dominio::puertos::repositorio_resennia::RepositorioResennia;
use crate::infraestructura::RepositorioMySQL;
use std::error::Error;
use async_trait::async_trait;
use sqlx::MySql;

#[async_trait]
impl RepositorioResennia for RepositorioMySQL {
    async fn guardar(&self, resennia: Resennia) -> Result<Resennia, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query(
            "INSERT INTO resennia (solicitud_id, calificacion, comentario) VALUES (?, ?, ?)"
        )
        .bind(resennia.solicitud_id)
        .bind(resennia.calificacion)
        .bind(&resennia.comentario)
        .execute(&self.pool)
        .await?;

        let id = resultado.last_insert_id() as i32;
        Ok(Resennia {
            id: Some(id),
            ..resennia
        })
    }

    async fn buscar_por_solicitud(&self, solicitud_id: i32) -> Result<Option<Resennia>, Box<dyn Error + Send + Sync>> {
        let registro = sqlx::query_as::<MySql, Resennia>(
            "SELECT id, solicitud_id, calificacion, comentario, fecha_creacion FROM resennia WHERE solicitud_id = ?"
        )
        .bind(solicitud_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(registro)
    }
}
