use crate::dominio::solicitud::{SolicitudServicio, EstadoSolicitud};
use crate::dominio::puertos::repositorio_solicitud::RepositorioSolicitud;
use crate::infraestructura::RepositorioMySQL;
use std::error::Error;
use async_trait::async_trait;
use sqlx::MySql;

#[async_trait]
impl RepositorioSolicitud for RepositorioMySQL {
    async fn crear(&self, solicitud: SolicitudServicio) -> Result<SolicitudServicio, Box<dyn Error + Send + Sync>> {
        let urgencia_cadena = solicitud.urgencia.a_cadena();
        let estado_cadena = match solicitud.estado {
            EstadoSolicitud::Pendiente => "pendiente",
            EstadoSolicitud::Aceptado => "aceptado",
            EstadoSolicitud::Terminado => "terminado",
            EstadoSolicitud::Cancelado => "cancelado",
            EstadoSolicitud::EnEsperaDePago => "en_espera_de_pago",
        };

        let resultado = sqlx::query(
            "INSERT INTO solicitud_servicio (usuario_id, servicio_id, urgencia, precio_final, estado, latitud_usuario, longitud_usuario) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(solicitud.usuario_id)
        .bind(solicitud.servicio_id)
        .bind(urgencia_cadena)
        .bind(solicitud.precio_final)
        .bind(estado_cadena)
        .bind(solicitud.latitud_usuario)
        .bind(solicitud.longitud_usuario)
        .execute(&self.pool)
        .await?;

        let id = resultado.last_insert_id() as i32;
        Ok(SolicitudServicio {
            id: Some(id),
            ..solicitud
        })
    }

    async fn buscar_por_id(&self, id: i32) -> Result<Option<SolicitudServicio>, Box<dyn Error + Send + Sync>> {
        let registro = sqlx::query_as::<MySql, SolicitudServicio>(
            "SELECT id, usuario_id, servicio_id, urgencia, precio_final, estado, latitud_usuario, longitud_usuario, fecha_creacion FROM solicitud_servicio WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(registro)
    }

    async fn actualizar_estado(&self, id: i32, estado: EstadoSolicitud) -> Result<(), Box<dyn Error + Send + Sync>> {
        let estado_cadena = match estado {
            EstadoSolicitud::Pendiente => "pendiente",
            EstadoSolicitud::Aceptado => "aceptado",
            EstadoSolicitud::Terminado => "terminado",
            EstadoSolicitud::Cancelado => "cancelado",
            EstadoSolicitud::EnEsperaDePago => "en_espera_de_pago",
        };

        sqlx::query(
            "UPDATE solicitud_servicio SET estado = ? WHERE id = ?"
        )
        .bind(estado_cadena)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
