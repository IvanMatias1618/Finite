use crate::dominio::servicio::{Servicio, PrecioServicioUrgencia};
use crate::dominio::puertos::repositorio_servicio::RepositorioServicio;
use std::error::Error;
use std::sync::Arc;

pub struct CasoUsoRegistrarServicioTecnico {
    repositorio_servicio: Arc<dyn RepositorioServicio>,
}

impl CasoUsoRegistrarServicioTecnico {
    pub fn nuevo(repositorio_servicio: Arc<dyn RepositorioServicio>) -> Self {
        Self { repositorio_servicio }
    }

    pub async fn ejecutar(
        &self,
        colaborador_id: i32,
        mut servicio: Servicio,
        precios_urgencia: Vec<PrecioServicioUrgencia>,
    ) -> Result<i32, Box<dyn Error + Send + Sync>> {
        servicio.colaborador_id = colaborador_id;
        let servicio_guardado = self.repositorio_servicio.guardar(servicio).await?;
        let servicio_id = servicio_guardado.id.unwrap();

        for mut p in precios_urgencia {
            p.servicio_id = servicio_id;
            self.repositorio_servicio.guardar_precio_urgencia(p).await?;
        }

        Ok(servicio_id)
    }
}
