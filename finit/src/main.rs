use std::sync::Arc;
use finit::aplicacion::servicios::registro_colaborador::CasoUsoRegistroColaborador;
use finit::aplicacion::servicios::registro_usuario::CasoUsoRegistroUsuario;
use finit::aplicacion::servicios::login_usuario::CasoUsoLoginUsuario;
use finit::aplicacion::servicios::listar_categorias::CasoUsoListarCategorias;
use finit::aplicacion::servicios::listar_subcategorias::CasoUsoListarSubcategorias;
use finit::aplicacion::servicios::consultar_subcategoria::CasoUsoConsultarSubcategoria;
use finit::aplicacion::servicios::consultar_perfil_colaborador::CasoUsoConsultarPerfilColaborador;
use finit::aplicacion::servicios::gestionar_portafolio::CasoUsoGestionarPortafolio;
use finit::aplicacion::servicios::solicitud_servicio::CasoUsoSolicitudServicio;
use finit::aplicacion::servicios::listar_solicitudes::CasoUsoListarSolicitudes;
use finit::aplicacion::servicios::listar_colaboradores_marketplace::CasoUsoListarColaboradoresMarketplace;
use finit::aplicacion::servicios::gestionar_mensajes::CasoUsoGestionarMensajes;
use finit::aplicacion::servicios::actualizar_documentacion::CasoUsoActualizarDocumentacion;
use finit::aplicacion::servicios::configurar_precios_dinamicos::CasoUsoConfigurarPreciosDinamicos;
use finit::aplicacion::servicios::configurar_horarios::CasoUsoConfigurarHorarios;
use finit::aplicacion::servicios::calificar_servicio::CasoUsoCalificarServicio;
use finit::aplicacion::servicios::registrar_servicio_tecnico::CasoUsoRegistrarServicioTecnico;
use finit::aplicacion::servicios::consultar_estadisticas_colaborador::CasoUsoConsultarEstadisticasColaborador;
use finit::aplicacion::servicios::gestionar_estado_solicitud::CasoUsoGestionarEstadoSolicitud;
use finit::aplicacion::servicios::cotizar_servicio::CasoUsoCotizarServicio;
use finit::aplicacion::servicios::gestionar_verificacion::CasoUsoGestionarVerificacion;
use finit::infraestructura::RepositorioMySQL;
use finit::infraestructura::api::rutas as ax_routing;
use finit::infraestructura::api::rutas::EstadoApp;
use sqlx::MySqlPool;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL debe estar configurada");
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "secreto_por_defecto_desarrollo".to_string());
    let puerto = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string());

    let pool = MySqlPool::connect(&database_url).await?;
    let repositorio = Arc::new(RepositorioMySQL::nuevo(pool));

    // Inicializar Tablas
    repositorio.inicializar_tablas().await?;

    // Inicializar Casos de Uso
    let registro_colaborador = Arc::new(CasoUsoRegistroColaborador::nuevo(
        repositorio.clone(),
        repositorio.clone(),
        repositorio.clone(),
        jwt_secret.clone(),
    ));

    let registro_usuario = Arc::new(CasoUsoRegistroUsuario::nuevo(
        repositorio.clone(),
    ));

    let login_usuario = Arc::new(CasoUsoLoginUsuario::nuevo(
        repositorio.clone(),
        jwt_secret,
    ));

    let listar_categorias = Arc::new(CasoUsoListarCategorias::nuevo(
        repositorio.clone(),
    ));

    let listar_subcategorias = Arc::new(CasoUsoListarSubcategorias::nuevo(
        repositorio.clone(),
    ));

    let consultar_subcategoria = Arc::new(CasoUsoConsultarSubcategoria::nuevo(
        repositorio.clone(),
    ));

    let consultar_perfil_colaborador = Arc::new(CasoUsoConsultarPerfilColaborador::nuevo(
        repositorio.clone(),
        repositorio.clone(),
        repositorio.clone(),
    ));

    let solicitud_servicio = Arc::new(CasoUsoSolicitudServicio::nuevo(
        repositorio.clone(),
        repositorio.clone(),
    ));

    let listar_solicitudes = Arc::new(CasoUsoListarSolicitudes::nuevo(
        repositorio.clone(),
    ));

    let listar_colaboradores_marketplace = Arc::new(CasoUsoListarColaboradoresMarketplace::nuevo(
        repositorio.clone(),
        repositorio.clone(),
        repositorio.clone(),
        repositorio.clone(),
    ));

    let gestionar_mensajes = Arc::new(CasoUsoGestionarMensajes::nuevo(
        repositorio.clone(),
    ));

    let gestionar_portafolio = Arc::new(CasoUsoGestionarPortafolio::nuevo(
        repositorio.clone(),
    ));

    let actualizar_documentacion = Arc::new(CasoUsoActualizarDocumentacion::nuevo(
        repositorio.clone(),
    ));

    let configurar_precios_dinamicos = Arc::new(CasoUsoConfigurarPreciosDinamicos::nuevo(
        repositorio.clone(),
    ));

    let configurar_horarios = Arc::new(CasoUsoConfigurarHorarios::nuevo(
        repositorio.clone(),
    ));

    let calificar_servicio = Arc::new(CasoUsoCalificarServicio::nuevo(
        repositorio.clone(),
        repositorio.clone(),
    ));

    let registrar_servicio_tecnico = Arc::new(CasoUsoRegistrarServicioTecnico::nuevo(
        repositorio.clone(),
    ));

    let consultar_estadisticas_colaborador = Arc::new(CasoUsoConsultarEstadisticasColaborador::nuevo(
        repositorio.clone(),
    ));

    let gestionar_estado_solicitud = Arc::new(CasoUsoGestionarEstadoSolicitud::nuevo(
        repositorio.clone(),
    ));

    let cotizar_servicio = Arc::new(CasoUsoCotizarServicio::nuevo(
        repositorio.clone(),
        repositorio.clone(),
    ));

    let gestionar_verificacion = Arc::new(CasoUsoGestionarVerificacion::nuevo(
        repositorio.clone(),
    ));

    let estado = Arc::new(EstadoApp {
        registro_colaborador,
        registro_usuario,
        login_usuario,
        listar_categorias,
        listar_subcategorias,
        consultar_subcategoria,
        consultar_perfil_colaborador,
        solicitud_servicio,
        listar_solicitudes,
        listar_colaboradores_marketplace,
        gestionar_mensajes,
        gestionar_portafolio,
        actualizar_documentacion,
        configurar_precios_dinamicos,
        configurar_horarios,
        calificar_servicio,
        registrar_servicio_tecnico,
        consultar_estadisticas_colaborador,
        gestionar_estado_solicitud,
        cotizar_servicio,
        gestionar_verificacion,
    });

    // Configurar Rutas
    let app = ax_routing::crear_rutas(estado)
        .layer(CorsLayer::permissive());

    // Iniciar Servidor
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", puerto)).await?;
    println!("🚀 Servidor finit iniciado en http://localhost:{}", puerto);
    
    axum::serve(listener, app).await?;

    Ok(())
}
