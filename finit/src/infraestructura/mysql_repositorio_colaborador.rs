use crate::dominio::colaborador::Colaborador;
use crate::dominio::puertos::repositorio_colaborador::RepositorioColaborador;
use crate::infraestructura::RepositorioMySQL;
use std::error::Error;
use async_trait::async_trait;
use sqlx::MySql;

#[async_trait]
impl RepositorioColaborador for RepositorioMySQL {
    async fn guardar(&self, colaborador: Colaborador) -> Result<Colaborador, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query(
            "INSERT INTO colaborador (usuario_id, telefono, sitio_web) VALUES (?, ?, ?)"
        )
        .bind(colaborador.usuario_id)
        .bind(&colaborador.telefono)
        .bind(&colaborador.sitio_web)
        .execute(&self.pool)
        .await?;

        let id = resultado.last_insert_id() as i32;
        Ok(Colaborador {
            id: Some(id),
            usuario_id: colaborador.usuario_id,
            telefono: colaborador.telefono,
            sitio_web: colaborador.sitio_web,
        })
    }

    async fn buscar_por_id(&self, id: i32) -> Result<Option<Colaborador>, Box<dyn Error + Send + Sync>> {
        let registro = sqlx::query_as::<MySql, Colaborador>(
            "SELECT id, usuario_id, telefono, sitio_web FROM colaborador WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(registro)
    }
}
