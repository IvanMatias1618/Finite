use crate::dominio::disponibilidad::Disponibilidad;
use crate::dominio::puertos::repositorio_disponibilidad::RepositorioDisponibilidad;
use crate::infraestructura::RepositorioMySQL;
use std::error::Error;
use async_trait::async_trait;
use sqlx::MySql;

#[async_trait]
impl RepositorioDisponibilidad for RepositorioMySQL {
    async fn guardar_disponibilidad(&self, disponibilidad: Disponibilidad) -> Result<Disponibilidad, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query(
            "INSERT INTO disponibilidad_colaborador (colaborador_id, dia_semana, hora_inicio, hora_fin, activo) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(disponibilidad.colaborador_id)
        .bind(disponibilidad.dia_semana)
        .bind(&disponibilidad.hora_inicio)
        .bind(&disponibilidad.hora_fin)
        .bind(disponibilidad.activo)
        .execute(&self.pool)
        .await?;

        let id = resultado.last_insert_id() as i32;
        Ok(Disponibilidad {
            id: Some(id),
            ..disponibilidad
        })
    }

    async fn buscar_por_colaborador(&self, colaborador_id: i32) -> Result<Vec<Disponibilidad>, Box<dyn Error + Send + Sync>> {
        let registros = sqlx::query_as::<MySql, Disponibilidad>(
            "SELECT id, colaborador_id, dia_semana, hora_inicio, hora_fin, activo FROM disponibilidad_colaborador WHERE colaborador_id = ?"
        )
        .bind(colaborador_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(registros)
    }

    async fn eliminar_por_colaborador(&self, colaborador_id: i32) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query("DELETE FROM disponibilidad_colaborador WHERE colaborador_id = ?")
            .bind(colaborador_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
