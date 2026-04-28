use crate::dominio::configuracion_precio::ConfiguracionPrecio;
use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioConfiguracionPrecio: Send + Sync {
    async fn guardar(&self, configuracion: ConfiguracionPrecio) -> Result<ConfiguracionPrecio, Box<dyn Error + Send + Sync>>;
    async fn buscar_por_colaborador(&self, colaborador_id: i32) -> Result<Option<ConfiguracionPrecio>, Box<dyn Error + Send + Sync>>;
    async fn actualizar(&self, configuracion: ConfiguracionPrecio) -> Result<ConfiguracionPrecio, Box<dyn Error + Send + Sync>>;
}
