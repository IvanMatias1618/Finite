use std::error::Error;
use async_trait::async_trait;
use serde_json::Value;

use super::repositorio_usuario::RepositorioUsuario;
use super::repositorio_colaborador::RepositorioColaborador;
use super::repositorio_servicio::RepositorioServicio;
use super::repositorio_solicitud::RepositorioSolicitud;
use super::repositorio_categoria::RepositorioCategoria;
use super::repositorio_mensaje::RepositorioMensaje;
use super::repositorio_disponibilidad::RepositorioDisponibilidad;
use super::repositorio_configuracion_precio::RepositorioConfiguracionPrecio;
use super::repositorio_resennia::RepositorioResennia;
use super::repositorio_cotizacion_especial::RepositorioCotizacionEspecial;
use super::repositorio_soporte::RepositorioSoporte;

#[async_trait]
pub trait RepositorioMotor: 
    RepositorioUsuario + 
    RepositorioColaborador + 
    RepositorioServicio + 
    RepositorioSolicitud + 
    RepositorioCategoria + 
    RepositorioMensaje + 
    RepositorioDisponibilidad + 
    RepositorioConfiguracionPrecio + 
    RepositorioResennia + 
    RepositorioCotizacionEspecial + 
    RepositorioSoporte +
    Send + Sync 
{
    async fn inicializar_tablas(&self) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn limpiar_y_sembrar(&self, admin_password: &str) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn ejecutar_query(&self, sql: &str) -> Result<Value, Box<dyn Error + Send + Sync>>;
}
