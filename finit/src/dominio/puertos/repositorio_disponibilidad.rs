use crate::dominio::disponibilidad::Disponibilidad;
use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioDisponibilidad: Send + Sync {
    async fn guardar_disponibilidad(&self, disponibilidad: Disponibilidad) -> Result<Disponibilidad, Box<dyn Error + Send + Sync>>;
    async fn buscar_por_colaborador(&self, colaborador_id: i32) -> Result<Vec<Disponibilidad>, Box<dyn Error + Send + Sync>>;
    async fn eliminar_por_colaborador(&self, colaborador_id: i32) -> Result<(), Box<dyn Error + Send + Sync>>;
}
