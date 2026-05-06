use axum::{
    extract::{State, Path, Multipart},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
    Extension,
};
use std::path::PathBuf;
use tokio::fs;
use uuid::Uuid;

#[axum::debug_handler]
pub async fn subir_archivo(
    mut multipart: Multipart,
) -> Result<Json<String>, AppError> {
    if let Some(field) = multipart.next_field().await.map_err(|e| AppError(e.to_string()))? {
        let nombre_original = field.file_name().unwrap_or("archivo").to_string();
        let extension = std::path::Path::new(&nombre_original)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("bin");
        
        let nombre_unico = format!("{}.{}", Uuid::new_v4(), extension);
        let ruta = PathBuf::from("uploads").join(&nombre_unico);

        let datos = field.bytes().await.map_err(|e| AppError(e.to_string()))?;
        fs::write(&ruta, datos).await.map_err(|e| AppError(e.to_string()))?;

        // Devolvemos la URL relativa que Okupo o Tauri usaran
        Ok(Json(format!("/archivos/{}", nombre_unico)))
    } else {
        Err(AppError("No se recibio ningun archivo".into()))
    }
}
use crate::dominio::token::Claims;

#[axum::debug_handler]
pub async fn aceptar_solicitud(
    State(estado): State<Arc<EstadoApp>>,
    Path(id): Path<i32>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, AppError> {
    let colaborador_id = claims.sub.parse::<i32>().map_err(|_| AppError("Token invalido".into()))?;
    match estado.gestionar_estado_solicitud.aceptar_solicitud(id, colaborador_id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err(AppError(e.to_string())),
    }
}

#[axum::debug_handler]
pub async fn finalizar_solicitud(
    State(estado): State<Arc<EstadoApp>>,
    Path(id): Path<i32>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, AppError> {
    let colaborador_id = claims.sub.parse::<i32>().map_err(|_| AppError("Token invalido".into()))?;
    match estado.gestionar_estado_solicitud.finalizar_solicitud(id, colaborador_id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err(AppError(e.to_string())),
    }
}

#[axum::debug_handler]
pub async fn cancelar_solicitud(
    State(estado): State<Arc<EstadoApp>>,
    Path(id): Path<i32>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, AppError> {
    let usuario_id = claims.sub.parse::<i32>().map_err(|_| AppError("Token invalido".into()))?;
    match estado.gestionar_estado_solicitud.cancelar_solicitud(id, usuario_id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err(AppError(e.to_string())),
    }
}

use crate::aplicacion::servicios::cotizar_servicio::Cotizacion;

#[derive(Deserialize)]
pub struct DatosCotizar {
    pub colaborador_id: i32,
    pub subcategoria_id: i32,
    pub urgencia: Urgencia,
    pub latitud: Decimal,
    pub longitud: Decimal,
}

#[axum::debug_handler]
pub async fn cotizar_servicio(
    State(estado): State<Arc<EstadoApp>>,
    Json(datos): Json<DatosCotizar>,
) -> Result<Json<Cotizacion>, AppError> {
    match estado.cotizar_servicio.ejecutar(
        datos.colaborador_id,
        datos.subcategoria_id,
        datos.urgencia,
        datos.latitud,
        datos.longitud
    ).await {
        Ok(cotizacion) => Ok(Json(cotizacion)),
        Err(e) => Err(AppError(e.to_string())),
    }
}

use crate::dominio::colaborador::{Colaborador, EstadoVerificacion};

#[axum::debug_handler]
pub async fn listar_colaboradores_pendientes(
    State(estado): State<Arc<EstadoApp>>,
) -> Result<Json<Vec<Colaborador>>, AppError> {
    match estado.gestionar_verificacion.listar_pendientes().await {
        Ok(pendientes) => Ok(Json(pendientes)),
        Err(e) => Err(AppError(e.to_string())),
    }
}

#[derive(Deserialize)]
pub struct DatosProcesarVerificacion {
    pub estado: EstadoVerificacion,
    pub comentario: Option<String>,
}

#[axum::debug_handler]
pub async fn procesar_verificacion(
    State(estado): State<Arc<EstadoApp>>,
    Path(id): Path<i32>,
    Json(datos): Json<DatosProcesarVerificacion>,
) -> Result<StatusCode, AppError> {
    match estado.gestionar_verificacion.procesar_verificacion(id, datos.estado, datos.comentario).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err(AppError(e.to_string())),
    }
}
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

use crate::dominio::colaborador::{PerfilColaborador, ResumenEstadisticasColaborador, TrabajoPortafolio};
use crate::dominio::resennia::Resennia;
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

#[axum::debug_handler]
pub async fn consultar_subcategoria(
    State(estado): State<Arc<EstadoApp>>,
    Path(id): Path<i32>,
) -> Result<Json<Subcategoria>, AppError> {
    match estado.consultar_subcategoria.ejecutar(id).await {
        Ok(Some(subcategoria)) => Ok(Json(subcategoria)),
        Ok(None) => Err(AppError("Subcategoria no encontrada".into())),
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

#[axum::debug_handler]
pub async fn consultar_estadisticas_colaborador(
    State(estado): State<Arc<EstadoApp>>,
    Path(id): Path<i32>,
) -> Result<Json<ResumenEstadisticasColaborador>, AppError> {
    match estado.consultar_estadisticas_colaborador.ejecutar(id).await {
        Ok(stats) => Ok(Json(stats)),
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

#[derive(Deserialize)]
pub struct DatosCalificacion {
    pub solicitud_id: i32,
    pub calificacion: i8,
    pub comentario: Option<String>,
}

#[derive(Deserialize)]
pub struct DatosRegistrarServicio {
    pub colaborador_id: i32,
    pub subcategoria_id: i32,
    pub descripcion: String,
    pub distancia_maxima_kilometros: Decimal,
    pub precio_por_kilometro: Decimal,
    pub latitud: Decimal,
    pub longitud: Decimal,
    pub precios_urgencia: Vec<PrecioServicioUrgencia>,
}

#[derive(Deserialize)]
pub struct DatosTrabajoPortafolio {
    pub foto_antes: String,
    pub foto_despues: String,
    pub descripcion: String,
}

#[axum::debug_handler]
pub async fn annadir_trabajo_portafolio(
    State(estado): State<Arc<EstadoApp>>,
    Path(id): Path<i32>,
    Json(datos): Json<DatosTrabajoPortafolio>,
) -> Result<Json<TrabajoPortafolio>, AppError> {
    let trabajo = TrabajoPortafolio {
        id: None,
        colaborador_id: id,
        foto_antes: datos.foto_antes,
        foto_despues: datos.foto_despues,
        descripcion: datos.descripcion,
    };

    match estado.gestionar_portafolio.annadir_trabajo(id, trabajo).await {
        Ok(t) => Ok(Json(t)),
        Err(e) => Err(AppError(e.to_string())),
    }
}

#[axum::debug_handler]
pub async fn eliminar_trabajo_portafolio(
    State(estado): State<Arc<EstadoApp>>,
    Path((_id, trabajo_id)): Path<(i32, i32)>,
) -> Result<StatusCode, AppError> {
    match estado.gestionar_portafolio.eliminar_trabajo(trabajo_id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err(AppError(e.to_string())),
    }
}

#[axum::debug_handler]
pub async fn registrar_servicio_tecnico(
    State(estado): State<Arc<EstadoApp>>,
    Json(datos): Json<DatosRegistrarServicio>,
) -> Result<Json<i32>, AppError> {
    let servicio = Servicio {
        id: None,
        colaborador_id: datos.colaborador_id,
        subcategoria_id: datos.subcategoria_id,
        descripcion: datos.descripcion,
        distancia_maxima_kilometros: datos.distancia_maxima_kilometros,
        precio_por_kilometro: datos.precio_por_kilometro,
        latitud: datos.latitud,
        longitud: datos.longitud,
    };

    match estado.registrar_servicio_tecnico.ejecutar(datos.colaborador_id, servicio, datos.precios_urgencia).await {
        Ok(id) => Ok(Json(id)),
        Err(e) => Err(AppError(e.to_string())),
    }
}

#[axum::debug_handler]
pub async fn calificar_servicio(
    State(estado): State<Arc<EstadoApp>>,
    Json(datos): Json<DatosCalificacion>,
) -> Result<Json<Resennia>, AppError> {
    match estado.calificar_servicio.ejecutar(datos.solicitud_id, datos.calificacion, datos.comentario).await {
        Ok(resennia) => Ok(Json(resennia)),
        Err(e) => Err(AppError(e.to_string())),
    }
}
