use crate::dominio::usuario::Usuario;
use crate::dominio::colaborador::Colaborador;
use crate::dominio::servicio::{Servicio, PrecioServicioUrgencia};
use crate::dominio::puertos::repositorio_usuario::RepositorioUsuario;
use crate::dominio::puertos::repositorio_colaborador::RepositorioColaborador;
use crate::dominio::puertos::repositorio_servicio::RepositorioServicio;
use std::error::Error;
use std::sync::Arc;

pub struct CasoUsoRegistroColaborador {
    repositorio_usuario: Arc<dyn RepositorioUsuario>,
    repositorio_colaborador: Arc<dyn RepositorioColaborador>,
    repositorio_servicio: Arc<dyn RepositorioServicio>,
}

impl CasoUsoRegistroColaborador {
    pub fn nuevo(
        repositorio_usuario: Arc<dyn RepositorioUsuario>,
        repositorio_colaborador: Arc<dyn RepositorioColaborador>,
        repositorio_servicio: Arc<dyn RepositorioServicio>,
    ) -> Self {
        Self {
            repositorio_usuario,
            repositorio_colaborador,
            repositorio_servicio,
        }
    }

    pub async fn ejecutar(
        &self,
        nombre: String,
        correo: String,
        telefono: String,
        sitio_web: Option<String>,
        servicios: Vec<(Servicio, Vec<PrecioServicioUrgencia>)>,
    ) -> Result<i32, Box<dyn Error + Send + Sync>> {
        // Buscar o crear usuario
        let usuario = match self.repositorio_usuario.buscar_por_correo(&correo).await? {
            Some(u) => u,
            None => {
                self.repositorio_usuario
                    .guardar(Usuario {
                        id: None,
                        nombre,
                        correo,
                    })
                    .await?
            }
        };

        // Crear colaborador
        let colaborador = self.repositorio_colaborador.guardar(Colaborador {
            id: None,
            usuario_id: usuario.id.unwrap(),
            telefono,
            sitio_web,
        }).await?;

        // Registrar servicios y sus precios
        for (mut servicio, precios) in servicios {
            servicio.colaborador_id = colaborador.id.unwrap();
            let servicio_guardado = self.repositorio_servicio.guardar(servicio).await?;
            
            for mut precio in precios {
                precio.servicio_id = servicio_guardado.id.unwrap();
                self.repositorio_servicio.guardar_precio_urgencia(precio).await?;
            }
        }

        Ok(colaborador.id.unwrap())
    }
}
