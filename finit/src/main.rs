use std::sync::Arc;
use finit::infraestructura::api::rutas::{self as ax_routing, EstadoApp};
use finit::aplicacion::servicios::registro_colaborador::CasoUsoRegistroColaborador;
use finit::aplicacion::servicios::registro_usuario::CasoUsoRegistroUsuario;
use finit::aplicacion::servicios::login_usuario::CasoUsoLoginUsuario;
use finit::aplicacion::servicios::listar_categorias::CasoUsoListarCategorias;
use finit::aplicacion::servicios::listar_subcategorias::CasoUsoListarSubcategorias;
use finit::aplicacion::servicios::consultar_perfil_colaborador::CasoUsoConsultarPerfilColaborador;
use finit::aplicacion::servicios::solicitud_servicio::CasoUsoSolicitudServicio;
use finit::aplicacion::servicios::listar_solicitudes::CasoUsoListarSolicitudes;
use finit::aplicacion::servicios::listar_colaboradores_marketplace::CasoUsoListarColaboradoresMarketplace;
use finit::aplicacion::servicios::gestionar_mensajes::CasoUsoGestionarMensajes;
use finit::aplicacion::servicios::actualizar_documentacion::CasoUsoActualizarDocumentacion;
use finit::aplicacion::servicios::configurar_precios_dinamicos::CasoUsoConfigurarPreciosDinamicos;
use finit::aplicacion::servicios::configurar_horarios::CasoUsoConfigurarHorarios;
use finit::infraestructura::RepositorioMySQL;
use sqlx::MySqlPool;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok(); // Carga las variables de entorno

    println!("Iniciando motor finit (Versión MySQL)...");
    
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL no definida en .env");
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secreto_finit".to_string());
    let puerto = std::env::var("PUERTO").unwrap_or_else(|_| "3000".to_string());

    println!("Conectando a MySQL...");
    let pool = MySqlPool::connect(&db_url).await?;

    let repositorio = Arc::new(RepositorioMySQL::nuevo(pool));
    
    // Inicializar tablas y datos semilla
    repositorio.inicializar_tablas().await?;

    // Inicializar Casos de Uso con secretos
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
        jwt_secret.clone(),
    ));

    let listar_categorias = Arc::new(CasoUsoListarCategorias::nuevo(
        repositorio.clone(),
    ));

    let listar_subcategorias = Arc::new(CasoUsoListarSubcategorias::nuevo(
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
    ));

    let gestionar_mensajes = Arc::new(CasoUsoGestionarMensajes::nuevo(
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

    let estado = Arc::new(EstadoApp {
        registro_colaborador,
        registro_usuario,
        login_usuario,
        listar_categorias,
        listar_subcategorias,
        consultar_perfil_colaborador,
        solicitud_servicio,
        listar_solicitudes,
        listar_colaboradores_marketplace,
        gestionar_mensajes,
        actualizar_documentacion,
        configurar_precios_dinamicos,
        configurar_horarios,
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
