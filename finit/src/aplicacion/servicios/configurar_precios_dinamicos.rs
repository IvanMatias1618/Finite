use crate::dominio::configuracion_precio::ConfiguracionPrecio;
use crate::dominio::puertos::repositorio_configuracion_precio::RepositorioConfiguracionPrecio;
use std::error::Error;
use std::sync::Arc;
use rust_decimal::Decimal;

pub struct CasoUsoConfigurarPreciosDinamicos {
    repositorio_configuracion: Arc<dyn RepositorioConfiguracionPrecio>,
}

impl CasoUsoConfigurarPreciosDinamicos {
    pub fn nuevo(repositorio_configuracion: Arc<dyn RepositorioConfiguracionPrecio>) -> Self {
        Self { repositorio_configuracion }
    }

    pub async fn ejecutar(
        &self,
        colaborador_id: i32,
        precio_por_kilometro: Decimal,
        recargo_lluvia: Decimal,
        recargo_domingo: Decimal,
        recargo_nocturno: Decimal,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let configuracion_existente = self.repositorio_configuracion.buscar_por_colaborador(colaborador_id).await?;

        let configuracion = ConfiguracionPrecio {
            id: configuracion_existente.as_ref().and_then(|c| c.id),
            colaborador_id,
            precio_por_kilometro,
            recargo_lluvia,
            recargo_domingo,
            recargo_nocturno,
        };

        if configuracion_existente.is_some() {
            self.repositorio_configuracion.actualizar(configuracion).await?;
        } else {
            self.repositorio_configuracion.guardar(configuracion).await?;
        }

        Ok(())
    }
}
