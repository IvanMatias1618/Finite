use crate::dominio::colaborador::PerfilColaborador;
use crate::dominio::puertos::repositorio_colaborador::RepositorioColaborador;
use crate::dominio::puertos::repositorio_usuario::RepositorioUsuario;
use crate::dominio::puertos::repositorio_servicio::RepositorioServicio;
use std::sync::Arc;
use std::error::Error;

pub struct CasoUsoConsultarPerfilColaborador {
    repo_colaborador: Arc<dyn RepositorioColaborador>,
    repo_usuario: Arc<dyn RepositorioUsuario>,
    repo_servicio: Arc<dyn RepositorioServicio>,
}

impl CasoUsoConsultarPerfilColaborador {
    pub fn nuevo(
        repo_colaborador: Arc<dyn RepositorioColaborador>,
        repo_usuario: Arc<dyn RepositorioUsuario>,
        repo_servicio: Arc<dyn RepositorioServicio>,
    ) -> Self {
        Self {
            repo_colaborador,
            repo_usuario,
            repo_servicio,
        }
    }

    pub async fn ejecutar(&self, colaborador_id: i32) -> Result<Option<PerfilColaborador>, Box<dyn Error + Send + Sync>> {
        let colaborador = match self.repo_colaborador.buscar_por_id(colaborador_id).await? {
            Some(c) => c,
            None => return Ok(None),
        };

        let usuario = match self.repo_usuario.buscar_por_id(colaborador.usuario_id).await? {
            Some(u) => u,
            None => return Err("Usuario asociado no encontrado".into()),
        };

        let servicios = self.repo_servicio.buscar_por_colaborador(colaborador_id).await?;
        let portafolio = self.repo_colaborador.buscar_portafolio_por_colaborador(colaborador_id).await?;

        Ok(Some(PerfilColaborador {
            id: colaborador.id.unwrap_or(colaborador_id),
            nombre: usuario.nombre,
            telefono: colaborador.telefono,
            sitio_web: colaborador.sitio_web,
            foto_perfil: colaborador.foto_perfil,
            especialidad_resumen: colaborador.especialidad_resumen,
            es_verificado: colaborador.es_verificado,
            medio_transporte: colaborador.medio_transporte,
            rating_promedio: colaborador.rating_promedio,
            total_servicios: colaborador.total_servicios,
            servicios,
            portafolio,
        }))
    }
}
