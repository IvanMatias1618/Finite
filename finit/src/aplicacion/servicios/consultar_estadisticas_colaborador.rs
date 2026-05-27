use crate::dominio::colaborador::ResumenEstadisticasColaborador;
use crate::dominio::puertos::repositorio_colaborador::RepositorioColaborador;
use std::error::Error;
use std::sync::Arc;

pub struct CasoUsoConsultarEstadisticasColaborador {
    repositorio_colaborador: Arc<dyn RepositorioColaborador>,
}

impl CasoUsoConsultarEstadisticasColaborador {
    pub fn nuevo(repositorio_colaborador: Arc<dyn RepositorioColaborador>) -> Self {
        Self {
            repositorio_colaborador,
        }
    }

    pub async fn ejecutar(
        &self,
        colaborador_id: i32,
    ) -> Result<ResumenEstadisticasColaborador, Box<dyn Error + Send + Sync>> {
        // Verificar que el colaborador exista
        self.repositorio_colaborador.buscar_por_id(colaborador_id).await?
            .ok_or("El colaborador no existe")?;

        self.repositorio_colaborador.obtener_estadisticas(colaborador_id).await
    }
}
