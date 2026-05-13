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
        nombre_completo: String,
        telefono: String,
        telefono_verificacion: Option<String>,
        zona_trabajo: Option<String>,
        sitio_web: Option<String>,
        foto_perfil: Option<String>,
        medio_transporte: Option<String>,
        especialidad_resumen: Option<String>,
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
        let mut usuario = self.repositorio_usuario.buscar_por_id(usuario_id).await?
            .ok_or("Usuario no encontrado")?;

        // Actualizar el nombre y el rol del usuario
        let mut usuario_actualizado = false;
        if usuario.nombre != nombre_completo {
            usuario.nombre = nombre_completo;
            usuario_actualizado = true;
        }
        
        if usuario.rol != "colaborador" {
            usuario.rol = "colaborador".to_string();
            usuario_actualizado = true;
        }

        if usuario_actualizado {
            self.repositorio_usuario.actualizar(usuario).await?;
        }

        // Crear colaborador
        let colaborador = self.repositorio_colaborador.guardar(Colaborador {
            id: None,
            usuario_id,
            telefono,
            telefono_verificacion,
            zona_trabajo,
            sitio_web,
            foto_perfil,
            especialidad_resumen,
            es_verificado: false,
            estado_verificacion: EstadoVerificacion::Pendiente,
            ine_frontal: None,
            ine_trasera: None,
            comprobante_domicilio: None,
            foto_selfie_ine: None,
            medio_transporte,
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
