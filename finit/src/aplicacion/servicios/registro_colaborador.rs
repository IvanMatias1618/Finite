use crate::dominio::colaborador::{Colaborador, EstadoVerificacion};
use crate::dominio::servicio::{Servicio, PrecioServicioUrgencia};
use crate::dominio::puertos::repositorio_usuario::RepositorioUsuario;
use crate::dominio::puertos::repositorio_colaborador::RepositorioColaborador;
use crate::dominio::puertos::repositorio_servicio::RepositorioServicio;
use crate::dominio::token::Claims;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::error::Error;
use std::sync::Arc;
use rust_decimal::Decimal;

pub struct CasoUsoRegistroColaborador {
    repositorio_usuario: Arc<dyn RepositorioUsuario>,
    repositorio_colaborador: Arc<dyn RepositorioColaborador>,
    repositorio_servicio: Arc<dyn RepositorioServicio>,
    jwt_secret: String,
}

impl CasoUsoRegistroColaborador {
    pub fn nuevo(
        repositorio_usuario: Arc<dyn RepositorioUsuario>,
        repositorio_colaborador: Arc<dyn RepositorioColaborador>,
        repositorio_servicio: Arc<dyn RepositorioServicio>,
        jwt_secret: String,
    ) -> Self {
        Self {
            repositorio_usuario,
            repositorio_colaborador,
            repositorio_servicio,
            jwt_secret,
        }
    }

    pub async fn ejecutar(
        &self,
        token_usuario: String,
        telefono: String,
        sitio_web: Option<String>,
        servicios: Vec<(Servicio, Vec<PrecioServicioUrgencia>)>,
    ) -> Result<i32, Box<dyn Error + Send + Sync>> {
        // Decodificar el token JWT
        let token_data = decode::<Claims>(
            &token_usuario,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::default(),
        ).map_err(|_| "Token de usuario invalido o expirado")?;

        let usuario_id = token_data.claims.sub.parse::<i32>()?;

        // Validar que el usuario existe
        let usuario = self.repositorio_usuario.buscar_por_id(usuario_id).await?
            .ok_or("Usuario no encontrado")?;

        // Crear colaborador
        let colaborador = self.repositorio_colaborador.guardar(Colaborador {
            id: None,
            usuario_id: usuario.id.unwrap(),
            telefono,
            sitio_web,
            foto_perfil: None,
            especialidad_resumen: None,
            es_verificado: false,
            estado_verificacion: EstadoVerificacion::Pendiente,
            ine_frontal: None,
            ine_trasera: None,
            comprobante_domicilio: None,
            foto_selfie_ine: None,
            medio_transporte: None,
            rating_promedio: Decimal::ZERO,
            total_servicios: 0,
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
