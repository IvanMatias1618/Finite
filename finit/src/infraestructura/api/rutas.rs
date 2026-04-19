use axum::{routing::post, Router};
use crate::aplicacion::servicios::registro_colaborador::CasoUsoRegistroColaborador;
use crate::aplicacion::servicios::registro_usuario::CasoUsoRegistroUsuario;
use crate::infraestructura::api::manejadores;
use std::sync::Arc;

pub struct EstadoApp {
    pub registro_colaborador: Arc<CasoUsoRegistroColaborador>,
    pub registro_usuario: Arc<CasoUsoRegistroUsuario>,
}

pub fn crear_rutas(estado: Arc<EstadoApp>) -> Router {
    Router::new()
        .route("/colaboradores", post(manejadores::registrar_colaborador))
        .route("/usuarios", post(manejadores::registrar_usuario))
        .with_state(estado)
}
