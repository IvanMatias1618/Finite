use axum::{
    extract::{State, Path},
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

use crate::dominio::categoria::{Categoria, Subcategoria};

#[axum::debug_handler]
pub async fn listar_categorias(
    State(estado): State<Arc<EstadoApp>>,
) -> Result<Json<Vec<Categoria>>, AppError> {
    match estado.listar_categorias.ejecutar().await {
        Ok(categorias) => Ok(Json(categorias)),
        Err(e) => Err(AppError(e.to_string())),
    }
}

#[axum::debug_handler]
pub async fn listar_subcategorias(
    State(estado): State<Arc<EstadoApp>>,
    Path(id): Path<i32>,
) -> Result<Json<Vec<Subcategoria>>, AppError> {
    match estado.listar_subcategorias.ejecutar(id).await {
        Ok(subcategorias) => Ok(Json(subcategorias)),
        Err(e) => Err(AppError(e.to_string())),
    }
}

use crate::aplicacion::servicios::listar_colaboradores_marketplace::ColaboradorMarketplace;

use crate::dominio::mensaje::MensajeSolicitud;

#[derive(Deserialize)]
pub struct DatosEnviarMensaje {
    pub emisor_id: i32,
    pub contenido: String,
}

#[axum::debug_handler]
pub async fn enviar_mensaje(
    State(estado): State<Arc<EstadoApp>>,
    Path(id): Path<i32>,
    Json(datos): Json<DatosEnviarMensaje>,
) -> Result<Json<MensajeSolicitud>, AppError> {
    match estado.gestionar_mensajes.enviar_mensaje(id, datos.emisor_id, datos.contenido).await {
        Ok(mensaje) => Ok(Json(mensaje)),
        Err(e) => Err(AppError(e.to_string())),
    }
}

#[axum::debug_handler]
pub async fn listar_mensajes(
    State(estado): State<Arc<EstadoApp>>,
    Path(id): Path<i32>,
) -> Result<Json<Vec<MensajeSolicitud>>, AppError> {
    match estado.gestionar_mensajes.listar_mensajes(id).await {
        Ok(mensajes) => Ok(Json(mensajes)),
        Err(e) => Err(AppError(e.to_string())),
    }
}

#[derive(Deserialize)]
pub struct QueryMarketplace {
    pub latitud: Decimal,
    pub longitud: Decimal,
}

#[axum::debug_handler]
pub async fn listar_colaboradores_marketplace(
    State(estado): State<Arc<EstadoApp>>,
    Path(id): Path<i32>,
    axum::extract::Query(query): axum::extract::Query<QueryMarketplace>,
) -> Result<Json<Vec<ColaboradorMarketplace>>, AppError> {
    match estado.listar_colaboradores_marketplace.ejecutar(id, query.latitud, query.longitud).await {
        Ok(colaboradores) => Ok(Json(colaboradores)),
        Err(e) => Err(AppError(e.to_string())),
    }
}

#[derive(Deserialize)]
pub struct DatosLogin {
    pub correo: String,
    pub contrasenna: String,
}

#[axum::debug_handler]
pub async fn login_usuario(
    State(estado): State<Arc<EstadoApp>>,
    Json(datos): Json<DatosLogin>,
) -> Result<Json<String>, AppError> {
    match estado.login_usuario
        .ejecutar(datos.correo, datos.contrasenna)
        .await
    {
        Ok(token) => Ok(Json(token)),
        Err(e) => Err(AppError(e.to_string())),
    }
}

use crate::dominio::colaborador::PerfilColaborador;

#[axum::debug_handler]
pub async fn consultar_perfil_colaborador(
    State(estado): State<Arc<EstadoApp>>,
    Path(id): Path<i32>,
) -> Result<Json<PerfilColaborador>, AppError> {
    match estado.consultar_perfil_colaborador.ejecutar(id).await {
        Ok(Some(perfil)) => Ok(Json(perfil)),
        Ok(None) => Err(AppError("Colaborador no encontrado".into())),
        Err(e) => Err(AppError(e.to_string())),
    }
}

use crate::dominio::solicitud::SolicitudServicio;
use crate::dominio::urgencia::Urgencia;
use rust_decimal::Decimal;

#[derive(Deserialize)]
pub struct DatosCrearSolicitud {
    pub usuario_id: i32,
    pub colaborador_id: i32,
    pub subcategoria_id: i32,
    pub urgencia: Urgencia,
    pub descripcion_detallada: String,
    pub fotos_evidencia_inicial: Option<String>,
    pub latitud: Decimal,
    pub longitud: Decimal,
}

#[axum::debug_handler]
pub async fn crear_solicitud(
    State(estado): State<Arc<EstadoApp>>,
    Json(datos): Json<DatosCrearSolicitud>,
) -> Result<Json<SolicitudServicio>, AppError> {
    match estado.solicitud_servicio
        .crear_solicitud_directa(
            datos.usuario_id, 
            datos.colaborador_id, 
            datos.subcategoria_id, 
            datos.urgencia, 
            datos.descripcion_detallada, 
            datos.fotos_evidencia_inicial, 
            datos.latitud, 
            datos.longitud
        )
        .await
    {
        Ok(solicitud) => Ok(Json(solicitud)),
        Err(e) => Err(AppError(e.to_string())),
    }
}

#[derive(Deserialize)]
pub struct FiltroSolicitudes {
    pub usuario_id: Option<i32>,
}

#[axum::debug_handler]
pub async fn listar_solicitudes(
    State(estado): State<Arc<EstadoApp>>,
    axum::extract::Query(filtro): axum::extract::Query<FiltroSolicitudes>,
) -> Result<Json<Vec<SolicitudServicio>>, AppError> {
    match estado.listar_solicitudes.ejecutar(filtro.usuario_id).await {
        Ok(solicitudes) => Ok(Json(solicitudes)),
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

#[derive(Deserialize)]
pub struct DatosDocumentacion {
    pub ine_frontal: String,
    pub ine_trasera: String,
    pub comprobante_domicilio: String,
    pub foto_selfie_ine: String,
}

#[axum::debug_handler]
pub async fn actualizar_documentacion(
    State(estado): State<Arc<EstadoApp>>,
    Path(id): Path<i32>,
    Json(datos): Json<DatosDocumentacion>,
) -> Result<StatusCode, AppError> {
    match estado.actualizar_documentacion
        .ejecutar(id, datos.ine_frontal, datos.ine_trasera, datos.comprobante_domicilio, datos.foto_selfie_ine)
        .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err(AppError(e.to_string())),
    }
}

#[derive(Deserialize)]
pub struct DatosPreciosDinamicos {
    pub precio_por_kilometro: Decimal,
    pub recargo_lluvia: Decimal,
    pub recargo_domingo: Decimal,
    pub recargo_nocturno: Decimal,
}

#[axum::debug_handler]
pub async fn configurar_precios_dinamicos(
    State(estado): State<Arc<EstadoApp>>,
    Path(id): Path<i32>,
    Json(datos): Json<DatosPreciosDinamicos>,
) -> Result<StatusCode, AppError> {
    match estado.configurar_precios_dinamicos
        .ejecutar(id, datos.precio_por_kilometro, datos.recargo_lluvia, datos.recargo_domingo, datos.recargo_nocturno)
        .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err(AppError(e.to_string())),
    }
}

use crate::dominio::disponibilidad::Disponibilidad;

#[axum::debug_handler]
pub async fn configurar_horarios(
    State(estado): State<Arc<EstadoApp>>,
    Path(id): Path<i32>,
    Json(horarios): Json<Vec<Disponibilidad>>,
) -> Result<StatusCode, AppError> {
    match estado.configurar_horarios.ejecutar(id, horarios).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err(AppError(e.to_string())),
    }
}
