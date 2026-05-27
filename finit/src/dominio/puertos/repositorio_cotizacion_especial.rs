use crate::dominio::cotizacion_especial::CotizacionEspecial;
use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioCotizacionEspecial: Send + Sync {
    async fn guardar(&self, cotizacion: CotizacionEspecial) -> Result<CotizacionEspecial, Box<dyn Error + Send + Sync>>;
    async fn listar_por_usuario(&self, usuario_id: i32) -> Result<Vec<CotizacionEspecial>, Box<dyn Error + Send + Sync>>;
    async fn buscar_por_id(&self, id: i32) -> Result<Option<CotizacionEspecial>, Box<dyn Error + Send + Sync>>;
}
