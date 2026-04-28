use serde::{Deserialize, Serialize};
use crate::dominio::servicio::Servicio;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EstadoVerificacion {
    Pendiente,
    Verificado,
    Rechazado,
}

impl EstadoVerificacion {
    pub fn a_cadena_sqlite(&self) -> String {
        match self {
            EstadoVerificacion::Pendiente => "pendiente".to_string(),
            EstadoVerificacion::Verificado => "verificado".to_string(),
            EstadoVerificacion::Rechazado => "rechazado".to_string(),
        }
    }

    pub fn desde_cadena_sqlite(cadena: &str) -> Self {
        match cadena {
            "verificado" => EstadoVerificacion::Verificado,
            "rechazado" => EstadoVerificacion::Rechazado,
            _ => EstadoVerificacion::Pendiente,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Colaborador {
    pub id: Option<i32>,
    pub usuario_id: i32,
    pub telefono: String,
    pub sitio_web: Option<String>,
    pub foto_perfil: Option<String>,
    pub especialidad_resumen: Option<String>,
    pub es_verificado: bool,
    pub estado_verificacion: EstadoVerificacion,
    pub ine_frontal: Option<String>,
    pub ine_trasera: Option<String>,
    pub comprobante_domicilio: Option<String>,
    pub foto_selfie_ine: Option<String>,
    pub medio_transporte: Option<String>,
    #[sqlx(default)]
    pub rating_promedio: Decimal,
    pub total_servicios: i32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TrabajoPortafolio {
    pub id: Option<i32>,
    pub colaborador_id: i32,
    pub foto_antes: String,
    pub foto_despues: String,
    pub descripcion: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerfilColaborador {
    pub id: i32,
    pub nombre: String,
    pub telefono: String,
    pub sitio_web: Option<String>,
    pub foto_perfil: Option<String>,
    pub especialidad_resumen: Option<String>,
    pub es_verificado: bool,
    pub medio_transporte: Option<String>,
    pub rating_promedio: Decimal,
    pub total_servicios: i32,
    pub servicios: Vec<Servicio>,
    pub portafolio: Vec<TrabajoPortafolio>,
}
