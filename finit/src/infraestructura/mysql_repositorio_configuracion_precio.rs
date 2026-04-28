use crate::dominio::configuracion_precio::ConfiguracionPrecio;
use crate::dominio::puertos::repositorio_configuracion_precio::RepositorioConfiguracionPrecio;
use crate::infraestructura::RepositorioMySQL;
use std::error::Error;
use async_trait::async_trait;
use sqlx::MySql;

#[async_trait]
impl RepositorioConfiguracionPrecio for RepositorioMySQL {
    async fn guardar(&self, configuracion: ConfiguracionPrecio) -> Result<ConfiguracionPrecio, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query(
            "INSERT INTO configuracion_precio_colaborador (colaborador_id, precio_por_kilometro, recargo_lluvia, recargo_domingo, recargo_nocturno) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(configuracion.colaborador_id)
        .bind(configuracion.precio_por_kilometro)
        .bind(configuracion.recargo_lluvia)
        .bind(configuracion.recargo_domingo)
        .bind(configuracion.recargo_nocturno)
        .execute(&self.pool)
        .await?;

        let id = resultado.last_insert_id() as i32;
        Ok(ConfiguracionPrecio {
            id: Some(id),
            ..configuracion
        })
    }

    async fn buscar_por_colaborador(&self, colaborador_id: i32) -> Result<Option<ConfiguracionPrecio>, Box<dyn Error + Send + Sync>> {
        let registro = sqlx::query_as::<MySql, ConfiguracionPrecio>(
            "SELECT id, colaborador_id, precio_por_kilometro, recargo_lluvia, recargo_domingo, recargo_nocturno FROM configuracion_precio_colaborador WHERE colaborador_id = ?"
        )
        .bind(colaborador_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(registro)
    }

    async fn actualizar(&self, configuracion: ConfiguracionPrecio) -> Result<ConfiguracionPrecio, Box<dyn Error + Send + Sync>> {
        sqlx::query(
            "UPDATE configuracion_precio_colaborador SET precio_por_kilometro = ?, recargo_lluvia = ?, recargo_domingo = ?, recargo_nocturno = ? WHERE id = ?"
        )
        .bind(configuracion.precio_por_kilometro)
        .bind(configuracion.recargo_lluvia)
        .bind(configuracion.recargo_domingo)
        .bind(configuracion.recargo_nocturno)
        .bind(configuracion.id)
        .execute(&self.pool)
        .await?;

        Ok(configuracion)
    }
}
