use std::sync::Arc;
use axum::{routing::{get, post}, Router};
use crate::aplicacion::servicios::registro_colaborador::CasoUsoRegistroColaborador;
use crate::aplicacion::servicios::registro_usuario::CasoUsoRegistroUsuario;
use crate::aplicacion::servicios::login_usuario::CasoUsoLoginUsuario;
use crate::aplicacion::servicios::listar_categorias::CasoUsoListarCategorias;
use crate::aplicacion::servicios::listar_subcategorias::CasoUsoListarSubcategorias;
use crate::aplicacion::servicios::consultar_perfil_colaborador::CasoUsoConsultarPerfilColaborador;
use crate::aplicacion::servicios::solicitud_servicio::CasoUsoSolicitudServicio;
use crate::aplicacion::servicios::listar_solicitudes::CasoUsoListarSolicitudes;
use crate::aplicacion::servicios::listar_colaboradores_marketplace::CasoUsoListarColaboradoresMarketplace;
use crate::aplicacion::servicios::gestionar_mensajes::CasoUsoGestionarMensajes;
use super::manejadores;

pub struct EstadoApp {
    pub registro_colaborador: Arc<CasoUsoRegistroColaborador>,
    pub registro_usuario: Arc<CasoUsoRegistroUsuario>,
    pub login_usuario: Arc<CasoUsoLoginUsuario>,
    pub listar_categorias: Arc<CasoUsoListarCategorias>,
    pub listar_subcategorias: Arc<CasoUsoListarSubcategorias>,
    pub consultar_perfil_colaborador: Arc<CasoUsoConsultarPerfilColaborador>,
    pub solicitud_servicio: Arc<CasoUsoSolicitudServicio>,
    pub listar_solicitudes: Arc<CasoUsoListarSolicitudes>,
    pub listar_colaboradores_marketplace: Arc<CasoUsoListarColaboradoresMarketplace>,
    pub gestionar_mensajes: Arc<CasoUsoGestionarMensajes>,
}

pub fn crear_rutas(estado: Arc<EstadoApp>) -> Router {
    Router::new()
        .route("/", get(|| async { "Bienvenido al motor finit - API activa (okupo.db)" }))
        .route("/colaboradores", post(manejadores::registrar_colaborador))
        .route("/colaboradores/:id", get(manejadores::consultar_perfil_colaborador))
        .route("/usuarios", post(manejadores::registrar_usuario))
        .route("/login", post(manejadores::login_usuario))
        .route("/categorias", get(manejadores::listar_categorias))
        .route("/categorias/:id/subcategorias", get(manejadores::listar_subcategorias))
        .route("/subcategorias/:id/colaboradores", get(manejadores::listar_colaboradores_marketplace))
        .route("/solicitudes", post(manejadores::crear_solicitud))
        .route("/solicitudes", get(manejadores::listar_solicitudes))
        .route("/solicitudes/:id/mensajes", post(manejadores::enviar_mensaje))
        .route("/solicitudes/:id/mensajes", get(manejadores::listar_mensajes))
        .with_state(estado)
}
