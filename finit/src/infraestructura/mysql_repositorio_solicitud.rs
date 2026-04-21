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
            EstadoSolicitud::PendienteDeRevision => "pendiente_de_revision",
            EstadoSolicitud::AceptadoPorColaborador => "aceptado_por_colaborador",
            EstadoSolicitud::CitaProgramada => "cita_programada",
            EstadoSolicitud::Terminado => "terminado",
            EstadoSolicitud::Cancelado => "cancelado",
            EstadoSolicitud::EnEsperaDePago => "en_espera_de_pago",
        };

        let resultado = sqlx::query(
            "INSERT INTO solicitud_servicio (usuario_id, colaborador_id, subcategoria_id, servicio_id, urgencia, precio_final, estado, descripcion_detallada, fotos_evidencia_inicial, latitud_usuario, longitud_usuario) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(solicitud.usuario_id)
        .bind(solicitud.colaborador_id)
        .bind(solicitud.subcategoria_id)
        .bind(solicitud.servicio_id)
        .bind(urgencia_cadena)
        .bind(solicitud.precio_final)
        .bind(estado_cadena)
        .bind(&solicitud.descripcion_detallada)
        .bind(&solicitud.fotos_evidencia_inicial)
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
            "SELECT id, usuario_id, colaborador_id, subcategoria_id, servicio_id, urgencia, precio_final, estado, descripcion_detallada, fotos_evidencia_inicial, latitud_usuario, longitud_usuario, fecha_creacion FROM solicitud_servicio WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(registro)
    }

    async fn listar_por_usuario(&self, usuario_id: i32) -> Result<Vec<SolicitudServicio>, Box<dyn Error + Send + Sync>> {
        let registros = sqlx::query_as::<MySql, SolicitudServicio>(
            "SELECT id, usuario_id, colaborador_id, subcategoria_id, servicio_id, urgencia, precio_final, estado, descripcion_detallada, fotos_evidencia_inicial, latitud_usuario, longitud_usuario, fecha_creacion FROM solicitud_servicio WHERE usuario_id = ?"
        )
        .bind(usuario_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(registros)
    }

    async fn listar_todas(&self) -> Result<Vec<SolicitudServicio>, Box<dyn Error + Send + Sync>> {
        let registros = sqlx::query_as::<MySql, SolicitudServicio>(
            "SELECT id, usuario_id, colaborador_id, subcategoria_id, servicio_id, urgencia, precio_final, estado, descripcion_detallada, fotos_evidencia_inicial, latitud_usuario, longitud_usuario, fecha_creacion FROM solicitud_servicio"
        )
        .fetch_all(&self.pool).await?;

        Ok(registros)
    }

    async fn actualizar_estado(&self, id: i32, estado: EstadoSolicitud) -> Result<(), Box<dyn Error + Send + Sync>> {
        let estado_cadena = match estado {
            EstadoSolicitud::PendienteDeRevision => "pendiente_de_revision",
            EstadoSolicitud::AceptadoPorColaborador => "aceptado_por_colaborador",
            EstadoSolicitud::CitaProgramada => "cita_programada",
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
