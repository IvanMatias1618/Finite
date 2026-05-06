use crate::dominio::colaborador::{Colaborador, TrabajoPortafolio, ResumenEstadisticasColaborador};
use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioColaborador: Send + Sync {
    async fn guardar(&self, colaborador: Colaborador) -> Result<Colaborador, Box<dyn Error + Send + Sync>>;
    async fn actualizar(&self, colaborador: Colaborador) -> Result<Colaborador, Box<dyn Error + Send + Sync>>;
    async fn buscar_por_id(&self, id: i32) -> Result<Option<Colaborador>, Box<dyn Error + Send + Sync>>;
    async fn guardar_trabajo_portafolio(&self, trabajo: TrabajoPortafolio) -> Result<TrabajoPortafolio, Box<dyn Error + Send + Sync>>;
    async fn buscar_portafolio_por_colaborador(&self, colaborador_id: i32) -> Result<Vec<TrabajoPortafolio>, Box<dyn Error + Send + Sync>>;
    async fn eliminar_trabajo_portafolio(&self, trabajo_id: i32) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn obtener_estadisticas(&self, colaborador_id: i32) -> Result<ResumenEstadisticasColaborador, Box<dyn Error + Send + Sync>>;
    async fn listar_todos(&self) -> Result<Vec<Colaborador>, Box<dyn Error + Send + Sync>>;
}
