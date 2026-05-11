use crate::dominio::cotizacion_especial::CotizacionEspecial;
use crate::dominio::puertos::repositorio_cotizacion_especial::RepositorioCotizacionEspecial;
use crate::infraestructura::RepositorioMySQL;
use std::error::Error;
use async_trait::async_trait;
use sqlx::{MySql, Row};

#[async_trait]
impl RepositorioCotizacionEspecial for RepositorioMySQL {
    async fn guardar(&self, cotizacion: CotizacionEspecial) -> Result<CotizacionEspecial, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query(
            "INSERT INTO cotizacion_especial (usuario_id, descripcion_trabajo, fotos_evidencia, presupuesto_estimado, nivel_urgencia) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(cotizacion.usuario_id)
        .bind(&cotizacion.descripcion_trabajo)
        .bind(&cotizacion.fotos_evidencia)
        .bind(cotizacion.presupuesto_estimado)
        .bind(cotizacion.nivel_urgencia.a_cadena())
        .execute(&self.pool)
        .await?;

        let id = resultado.last_insert_id() as i32;
        Ok(CotizacionEspecial {
            id: Some(id),
            ..cotizacion
        })
    }

    async fn listar_por_usuario(&self, usuario_id: i32) -> Result<Vec<CotizacionEspecial>, Box<dyn Error + Send + Sync>> {
        let registros = sqlx::query_as::<MySql, CotizacionEspecial>(
            "SELECT id, usuario_id, descripcion_trabajo, fotos_evidencia, presupuesto_estimado, nivel_urgencia, fecha_creacion FROM cotizacion_especial WHERE usuario_id = ? ORDER BY fecha_creacion DESC"
        )
        .bind(usuario_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(registros)
    }

    async fn buscar_por_id(&self, id: i32) -> Result<Option<CotizacionEspecial>, Box<dyn Error + Send + Sync>> {
        let registro = sqlx::query_as::<MySql, CotizacionEspecial>(
            "SELECT id, usuario_id, descripcion_trabajo, fotos_evidencia, presupuesto_estimado, nivel_urgencia, fecha_creacion FROM cotizacion_especial WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(registro)
    }
}
