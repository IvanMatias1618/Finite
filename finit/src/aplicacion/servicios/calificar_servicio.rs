use crate::dominio::resennia::Resennia;
use crate::dominio::puertos::repositorio_resennia::RepositorioResennia;
use crate::dominio::puertos::repositorio_solicitud::RepositorioSolicitud;
use crate::dominio::solicitud::EstadoSolicitud;
use std::error::Error;
use std::sync::Arc;

pub struct CasoUsoCalificarServicio {
    repositorio_resennia: Arc<dyn RepositorioResennia>,
    repositorio_solicitud: Arc<dyn RepositorioSolicitud>,
}

impl CasoUsoCalificarServicio {
    pub fn nuevo(
        repositorio_resennia: Arc<dyn RepositorioResennia>,
        repositorio_solicitud: Arc<dyn RepositorioSolicitud>,
    ) -> Self {
        Self {
            repositorio_resennia,
            repositorio_solicitud,
        }
    }

    pub async fn ejecutar(
        &self,
        solicitud_id: i32,
        calificacion: i8,
        comentario: Option<String>,
    ) -> Result<Resennia, Box<dyn Error + Send + Sync>> {
        // 1. Verificar que la solicitud exista y esté terminada
        let solicitud = self.repositorio_solicitud.buscar_por_id(solicitud_id).await?
            .ok_or("La solicitud no existe")?;

        if solicitud.estado != EstadoSolicitud::Terminado {
            return Err("Solo se pueden calificar servicios terminados".into());
        }

        // 2. Verificar que no haya sido calificada ya
        if let Some(_) = self.repositorio_resennia.buscar_por_solicitud(solicitud_id).await? {
            return Err("Esta solicitud ya ha sido calificada".into());
        }

        // 3. Guardar la reseña
        let resennia = Resennia {
            id: None,
            solicitud_id,
            calificacion,
            comentario,
            fecha_creacion: None,
        };

        self.repositorio_resennia.guardar(resennia).await
    }
}
