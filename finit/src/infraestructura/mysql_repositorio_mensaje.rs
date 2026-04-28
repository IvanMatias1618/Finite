use crate::dominio::mensaje::MensajeSolicitud;
use crate::dominio::puertos::repositorio_mensaje::RepositorioMensaje;
use crate::infraestructura::RepositorioMySQL;
use std::error::Error;
use async_trait::async_trait;
use sqlx::MySql;

#[async_trait]
impl RepositorioMensaje for RepositorioMySQL {
    async fn guardar(&self, mensaje: MensajeSolicitud) -> Result<MensajeSolicitud, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query(
            "INSERT INTO mensaje_solicitud (solicitud_id, emisor_id, contenido) VALUES (?, ?, ?)"
        )
        .bind(mensaje.solicitud_id)
        .bind(mensaje.emisor_id)
        .bind(&mensaje.contenido)
        .execute(&self.pool)
        .await?;

        let id = resultado.last_insert_id() as i32;
        Ok(MensajeSolicitud {
            id: Some(id),
            ..mensaje
        })
    }

    async fn listar_por_solicitud(&self, solicitud_id: i32) -> Result<Vec<MensajeSolicitud>, Box<dyn Error + Send + Sync>> {
        let registros = sqlx::query_as::<MySql, MensajeSolicitud>(
            "SELECT id, solicitud_id, emisor_id, contenido, fecha_envio FROM mensaje_solicitud WHERE solicitud_id = ? ORDER BY fecha_envio ASC"
        )
        .bind(solicitud_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(registros)
    }
}
