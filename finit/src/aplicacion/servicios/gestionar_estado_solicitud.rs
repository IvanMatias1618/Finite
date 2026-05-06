use crate::dominio::puertos::repositorio_solicitud::RepositorioSolicitud;
use crate::dominio::solicitud::EstadoSolicitud;
use std::sync::Arc;
use std::error::Error;

pub struct CasoUsoGestionarEstadoSolicitud {
    repo_solicitud: Arc<dyn RepositorioSolicitud>,
}

impl CasoUsoGestionarEstadoSolicitud {
    pub fn nuevo(repo_solicitud: Arc<dyn RepositorioSolicitud>) -> Self {
        Self { repo_solicitud }
    }

    pub async fn aceptar_solicitud(
        &self,
        solicitud_id: i32,
        colaborador_id: i32,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let solicitud = self.repo_solicitud.buscar_por_id(solicitud_id).await?
            .ok_or("Solicitud no encontrada")?;

        if solicitud.colaborador_id != colaborador_id {
            return Err("No tienes permiso para aceptar esta solicitud".into());
        }

        if solicitud.estado != EstadoSolicitud::PendienteDeRevision {
            return Err("La solicitud no se encuentra en estado pendiente".into());
        }

        self.repo_solicitud.actualizar_estado(solicitud_id, EstadoSolicitud::AceptadoPorColaborador).await
    }

    pub async fn finalizar_solicitud(
        &self,
        solicitud_id: i32,
        colaborador_id: i32,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let solicitud = self.repo_solicitud.buscar_por_id(solicitud_id).await?
            .ok_or("Solicitud no encontrada")?;

        if solicitud.colaborador_id != colaborador_id {
            return Err("No tienes permiso para finalizar esta solicitud".into());
        }

        self.repo_solicitud.actualizar_estado(solicitud_id, EstadoSolicitud::Terminado).await
    }

    pub async fn cancelar_solicitud(
        &self,
        solicitud_id: i32,
        usuario_id: i32,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let solicitud = self.repo_solicitud.buscar_por_id(solicitud_id).await?
            .ok_or("Solicitud no encontrada")?;

        // Validar que el que cancela sea el usuario o el colaborador involucrado
        // (Para simplificar, por ahora solo el usuario_id que recibimos, 
        // pero podriamos recibir un 'solicitante_id' y validar contra ambos)
        if solicitud.usuario_id != usuario_id && solicitud.colaborador_id != usuario_id {
            return Err("No tienes permiso para cancelar esta solicitud".into());
        }

        self.repo_solicitud.actualizar_estado(solicitud_id, EstadoSolicitud::Cancelado).await
    }
}
