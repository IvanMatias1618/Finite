use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
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
pub struct DatosRegistroUsuario {
    pub nombre: String,
    pub correo: String,
    pub contrasenna: String,
}

use crate::infraestructura::api::rutas::EstadoApp;

#[axum::debug_handler]
pub async fn registrar_usuario(
    State(estado): State<Arc<EstadoApp>>,
    Json(datos): Json<DatosRegistroUsuario>,
) -> Result<Json<i32>, AppError> {
    match estado.registro_usuario
        .ejecutar(datos.nombre, datos.correo, datos.contrasenna)
        .await
    {
        Ok(id) => Ok(Json(id)),
        Err(e) => Err(AppError(e.to_string())),
    }
}

#[derive(Deserialize)]
pub struct DatosRegistro {
    pub token_usuario: String,
    pub telefono: String,
    pub sitio_web: Option<String>,
    pub servicios: Vec<(Servicio, Vec<PrecioServicioUrgencia>)>,
}

#[axum::debug_handler]
pub async fn registrar_colaborador(
    State(estado): State<Arc<EstadoApp>>,
    Json(datos): Json<DatosRegistro>,
) -> Result<Json<i32>, AppError> {
    match estado.registro_colaborador
        .ejecutar(datos.token_usuario, datos.telefono, datos.sitio_web, datos.servicios)
        .await
    {
        Ok(id) => Ok(Json(id)),
        Err(e) => Err(AppError(e.to_string())),
    }
}
