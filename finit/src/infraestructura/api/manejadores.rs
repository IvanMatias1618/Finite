use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use crate::aplicacion::servicios::registro_colaborador::CasoUsoRegistroColaborador;
use crate::dominio::servicio::{Servicio, PrecioServicioUrgencia};
use serde::Deserialize;
use std::sync::Arc;

// Error personalizado para cumplir con los requerimientos de Axum
pub struct AppError(String);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error del sistema: {}", self.0),
        )
            .into_response()
    }
}

#[derive(Deserialize)]
pub struct DatosRegistro {
    pub nombre: String,
    pub correo: String,
    pub telefono: String,
    pub sitio_web: Option<String>,
    pub servicios: Vec<(Servicio, Vec<PrecioServicioUrgencia>)>,
}

#[axum::debug_handler]
pub async fn registrar_colaborador(
    State(caso_uso): State<Arc<CasoUsoRegistroColaborador>>,
    Json(datos): Json<DatosRegistro>,
) -> Result<Json<i32>, AppError> {
    match caso_uso
        .ejecutar(datos.nombre, datos.correo, datos.telefono, datos.sitio_web, datos.servicios)
        .await
    {
        Ok(id) => Ok(Json(id)),
        Err(e) => Err(AppError(e.to_string())),
    }
}
