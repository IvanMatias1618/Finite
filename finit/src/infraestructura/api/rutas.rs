use std::sync::Arc;
use axum::{routing::{get, post}, Router};
use axum::extract::DefaultBodyLimit;
use crate::aplicacion::servicios::registro_colaborador::CasoUsoRegistroColaborador;
use crate::aplicacion::servicios::registro_usuario::CasoUsoRegistroUsuario;
use crate::aplicacion::servicios::login_usuario::CasoUsoLoginUsuario;
use crate::aplicacion::servicios::listar_categorias::CasoUsoListarCategorias;
use crate::aplicacion::servicios::listar_subcategorias::CasoUsoListarSubcategorias;
use crate::aplicacion::servicios::consultar_subcategoria::CasoUsoConsultarSubcategoria;
use crate::aplicacion::servicios::consultar_perfil_colaborador::CasoUsoConsultarPerfilColaborador;
use crate::aplicacion::servicios::gestionar_portafolio::CasoUsoGestionarPortafolio;
use crate::aplicacion::servicios::solicitud_servicio::CasoUsoSolicitudServicio;
use crate::aplicacion::servicios::listar_solicitudes::CasoUsoListarSolicitudes;
use crate::aplicacion::servicios::listar_colaboradores_marketplace::CasoUsoListarColaboradoresMarketplace;
use crate::aplicacion::servicios::gestionar_mensajes::CasoUsoGestionarMensajes;
use super::manejadores;

use crate::aplicacion::servicios::actualizar_documentacion::CasoUsoActualizarDocumentacion;
use crate::aplicacion::servicios::configurar_precios_dinamicos::CasoUsoConfigurarPreciosDinamicos;
use crate::aplicacion::servicios::configurar_horarios::CasoUsoConfigurarHorarios;
use crate::aplicacion::servicios::calificar_servicio::CasoUsoCalificarServicio;
use crate::aplicacion::servicios::registrar_servicio_tecnico::CasoUsoRegistrarServicioTecnico;
use crate::aplicacion::servicios::consultar_estadisticas_colaborador::CasoUsoConsultarEstadisticasColaborador;

use super::middleware;
use axum::middleware::{from_fn_with_state, from_fn};

use crate::aplicacion::servicios::gestionar_estado_solicitud::CasoUsoGestionarEstadoSolicitud;
use crate::aplicacion::servicios::cotizar_servicio::CasoUsoCotizarServicio;
use crate::aplicacion::servicios::gestionar_verificacion::CasoUsoGestionarVerificacion;
use crate::aplicacion::servicios::cotizacion_especial::CasoUsoCotizacionEspecial;
use crate::aplicacion::servicios::login_social::CasoUsoLoginSocial;
use crate::aplicacion::servicios::gestion_pagos::CasoUsoGestionPagos;

use crate::infraestructura::RepositorioMySQL;

pub struct EstadoApp {
    pub repositorio: Arc<RepositorioMySQL>,
    pub registro_colaborador: Arc<CasoUsoRegistroColaborador>,
    pub registro_usuario: Arc<CasoUsoRegistroUsuario>,
    pub login_usuario: Arc<CasoUsoLoginUsuario>,
    pub listar_categorias: Arc<CasoUsoListarCategorias>,
    pub listar_subcategorias: Arc<CasoUsoListarSubcategorias>,
    pub consultar_subcategoria: Arc<CasoUsoConsultarSubcategoria>,
    pub consultar_perfil_colaborador: Arc<CasoUsoConsultarPerfilColaborador>,
    pub solicitud_servicio: Arc<CasoUsoSolicitudServicio>,
    pub listar_solicitudes: Arc<CasoUsoListarSolicitudes>,
    pub listar_colaboradores_marketplace: Arc<CasoUsoListarColaboradoresMarketplace>,
    pub gestionar_mensajes: Arc<CasoUsoGestionarMensajes>,
    pub gestionar_portafolio: Arc<CasoUsoGestionarPortafolio>,
    pub actualizar_documentacion: Arc<CasoUsoActualizarDocumentacion>,
    pub configurar_precios_dinamicos: Arc<CasoUsoConfigurarPreciosDinamicos>,
    pub configurar_horarios: Arc<CasoUsoConfigurarHorarios>,
    pub calificar_servicio: Arc<CasoUsoCalificarServicio>,
    pub registrar_servicio_tecnico: Arc<CasoUsoRegistrarServicioTecnico>,
    pub consultar_estadisticas_colaborador: Arc<CasoUsoConsultarEstadisticasColaborador>,
    pub gestionar_estado_solicitud: Arc<CasoUsoGestionarEstadoSolicitud>,
    pub cotizar_servicio: Arc<CasoUsoCotizarServicio>,
    pub gestionar_verificacion: Arc<CasoUsoGestionarVerificacion>,
    pub cotizacion_especial: Arc<CasoUsoCotizacionEspecial>,
    pub login_social: Arc<CasoUsoLoginSocial>,
    pub gestion_pagos: Arc<CasoUsoGestionPagos>,
}

use tower_http::services::ServeDir;

pub fn crear_rutas(estado: Arc<EstadoApp>) -> Router {
    let rutas_protegidas = Router::new()
        .route("/colaboradores", post(manejadores::registrar_colaborador))
        .route("/colaboradores/:id/documentacion", post(manejadores::actualizar_documentacion))
        .route("/colaboradores/:id/precios-dinamicos", post(manejadores::configurar_precios_dinamicos))
        .route("/colaboradores/:id/horarios", post(manejadores::configurar_horarios))
        .route("/colaboradores/:id/portafolio", post(manejadores::annadir_trabajo_portafolio))
        .route("/colaboradores/:id/portafolio/:trabajo_id", axum::routing::delete(manejadores::eliminar_trabajo_portafolio))
        .route("/tecnico/servicios", post(manejadores::registrar_servicio_tecnico))
        .route("/solicitudes", post(manejadores::crear_solicitud))
        .route("/solicitudes", get(manejadores::listar_solicitudes))
        .route("/solicitudes/:id/aceptar", post(manejadores::aceptar_solicitud))
        .route("/solicitudes/:id/finalizar", post(manejadores::finalizar_solicitud))
        .route("/solicitudes/:id/cancelar", post(manejadores::cancelar_solicitud))
        .route("/solicitudes/:id/mensajes", post(manejadores::enviar_mensaje))
        .route("/solicitudes/:id/mensajes", get(manejadores::listar_mensajes))
        .route("/calificaciones", post(manejadores::calificar_servicio))
        .route("/cotizar", post(manejadores::cotizar_servicio))
        .route("/cotizaciones-especiales", post(manejadores::solicitar_cotizacion_especial))
        .route("/pagos/confirmar", post(manejadores::confirmar_pago))
        .nest("/admin", Router::new()
            .route("/colaboradores/pendientes", get(manejadores::listar_colaboradores_pendientes))
            .route("/query", post(manejadores::ejecutar_consulta_sql))
            .layer(from_fn(middleware::requiere_admin))
        )
        .route("/colaboradores/:id/verificar", post(manejadores::procesar_verificacion))
        .layer(from_fn_with_state(estado.clone(), middleware::validar_jwt));

    Router::new()
        .route("/", get(|| async { "Bienvenido al motor finit - API activa (okupo.db)" }))
        .nest_service("/archivos", ServeDir::new("uploads"))
        .route("/subidas", post(manejadores::subir_archivo))
        .route("/usuarios", post(manejadores::registrar_usuario))
        .route("/login", post(manejadores::login_usuario))
        .route("/auth/google", post(manejadores::login_google))
        .route("/auth/facebook", post(manejadores::login_facebook))
        .route("/categorias", get(manejadores::listar_categorias))
        .route("/categorias/:id/subcategorias", get(manejadores::listar_subcategorias))
        .route("/subcategorias/:id", get(manejadores::consultar_subcategoria))
        .route("/subcategorias/:id/colaboradores", get(manejadores::listar_colaboradores_marketplace))
        .route("/colaboradores/:id", get(manejadores::consultar_perfil_colaborador))
        .route("/colaboradores/:id/estadisticas", get(manejadores::consultar_estadisticas_colaborador))
        .route("/api/conekta/webhook", post(manejadores::conekta_webhook))
        .merge(rutas_protegidas)
        .layer(DefaultBodyLimit::max(20 * 1024 * 1024)) // Limite de 20MB para subida de documentos base64
        .with_state(estado)
}
