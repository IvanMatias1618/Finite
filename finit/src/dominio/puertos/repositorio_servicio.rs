use crate::dominio::servicio::{Servicio, PrecioServicioUrgencia};
use crate::dominio::urgencia::Urgencia;
use std::error::Error;
use rust_decimal::Decimal;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioServicio: Send + Sync {
    async fn guardar(&self, servicio: Servicio) -> Result<Servicio, Box<dyn Error + Send + Sync>>;
    async fn guardar_precio_urgencia(&self, precio: PrecioServicioUrgencia) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn buscar_por_id(&self, id: i32) -> Result<Option<Servicio>, Box<dyn Error + Send + Sync>>;
    async fn buscar_por_categoria_y_cercania(&self, categoria_id: i32, latitud: f64, longitud: f64) -> Result<Vec<Servicio>, Box<dyn Error + Send + Sync>>;
    async fn buscar_precio_por_servicio_y_urgencia(&self, servicio_id: i32, urgencia: Urgencia) -> Result<Option<Decimal>, Box<dyn Error + Send + Sync>>;
}
