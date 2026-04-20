use axum::{routing::{get, post}, Router};
use crate::aplicacion::servicios::registro_colaborador::CasoUsoRegistroColaborador;
use crate::aplicacion::servicios::registro_usuario::CasoUsoRegistroUsuario;
use crate::aplicacion::servicios::login_usuario::CasoUsoLoginUsuario;
use crate::aplicacion::servicios::listar_categorias::CasoUsoListarCategorias;
use crate::aplicacion::servicios::consultar_perfil_colaborador::CasoUsoConsultarPerfilColaborador;
use crate::infraestructura::api::manejadores;
use std::sync::Arc;

pub struct EstadoApp {
    pub registro_colaborador: Arc<CasoUsoRegistroColaborador>,
    pub registro_usuario: Arc<CasoUsoRegistroUsuario>,
    pub login_usuario: Arc<CasoUsoLoginUsuario>,
    pub listar_categorias: Arc<CasoUsoListarCategorias>,
    pub consultar_perfil_colaborador: Arc<CasoUsoConsultarPerfilColaborador>,
}

pub fn crear_rutas(estado: Arc<EstadoApp>) -> Router {
    Router::new()
        .route("/", get(|| async { "Bienvenido al motor finit - API activa (okupo.db)" }))
        .route("/colaboradores", post(manejadores::registrar_colaborador))
        .route("/colaboradores/:id", get(manejadores::consultar_perfil_colaborador))
        .route("/usuarios", post(manejadores::registrar_usuario))
        .route("/login", post(manejadores::login_usuario))
        .route("/categorias", get(manejadores::listar_categorias))
        .with_state(estado)
}
