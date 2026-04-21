use crate::dominio::colaborador::{Colaborador, TrabajoPortafolio};
use crate::dominio::puertos::repositorio_colaborador::RepositorioColaborador;
use crate::infraestructura::RepositorioMySQL;
use std::error::Error;
use async_trait::async_trait;
use sqlx::MySql;
use rust_decimal::Decimal;

#[async_trait]
impl RepositorioColaborador for RepositorioMySQL {
    async fn guardar(&self, colaborador: Colaborador) -> Result<Colaborador, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query(
            "INSERT INTO colaborador (usuario_id, telefono, sitio_web, foto_perfil, especialidad_resumen, es_verificado, medio_transporte, rating_promedio, total_servicios) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(colaborador.usuario_id)
        .bind(&colaborador.telefono)
        .bind(&colaborador.sitio_web)
        .bind(&colaborador.foto_perfil)
        .bind(&colaborador.especialidad_resumen)
        .bind(colaborador.es_verificado)
        .bind(&colaborador.medio_transporte)
        .bind(colaborador.rating_promedio)
        .bind(colaborador.total_servicios)
        .execute(&self.pool)
        .await?;

        let id = resultado.last_insert_id() as i32;
        Ok(Colaborador {
            id: Some(id),
            ..colaborador
        })
    }

    async fn buscar_por_id(&self, id: i32) -> Result<Option<Colaborador>, Box<dyn Error + Send + Sync>> {
        let registro = sqlx::query_as::<MySql, Colaborador>(
            "SELECT id, usuario_id, telefono, sitio_web, foto_perfil, especialidad_resumen, es_verificado, medio_transporte, rating_promedio, total_servicios FROM colaborador WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(registro)
    }

    async fn guardar_trabajo_portafolio(&self, trabajo: TrabajoPortafolio) -> Result<TrabajoPortafolio, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query(
            "INSERT INTO portafolio_colaborador (colaborador_id, foto_antes, foto_despues, descripcion) VALUES (?, ?, ?, ?)"
        )
        .bind(trabajo.colaborador_id)
        .bind(&trabajo.foto_antes)
        .bind(&trabajo.foto_despues)
        .bind(&trabajo.descripcion)
        .execute(&self.pool)
        .await?;

        let id = resultado.last_insert_id() as i32;
        Ok(TrabajoPortafolio {
            id: Some(id),
            ..trabajo
        })
    }

    async fn buscar_portafolio_por_colaborador(&self, colaborador_id: i32) -> Result<Vec<TrabajoPortafolio>, Box<dyn Error + Send + Sync>> {
        let registros = sqlx::query_as::<MySql, TrabajoPortafolio>(
            "SELECT id, colaborador_id, foto_antes, foto_despues, descripcion FROM portafolio_colaborador WHERE colaborador_id = ?"
        )
        .bind(colaborador_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(registros)
    }
}
