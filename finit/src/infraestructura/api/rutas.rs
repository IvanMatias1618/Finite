use axum::{routing::post, Router};
use crate::aplicacion::servicios::registro_colaborador::CasoUsoRegistroColaborador;
use crate::infraestructura::api::manejadores;
use std::sync::Arc;

pub fn crear_rutas(caso_uso_registro: Arc<CasoUsoRegistroColaborador>) -> Router {
    Router::new()
        .route("/colaboradores", post(manejadores::registrar_colaborador))
        .with_state(caso_uso_registro)
}
