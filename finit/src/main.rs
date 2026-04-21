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
use finit::infraestructura::sqlite_repositorio::RepositorioSQLite;
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Iniciando motor finit con SQLite (okupo.db)...");
    
    let pool = SqlitePool::connect("sqlite:infraestructura/okupo.db").await?;

    let repositorio = Arc::new(RepositorioSQLite::nuevo(pool));
    
    // Inicializar tablas en SQLite
    repositorio.inicializar_tablas().await?;

    // Inicializar Casos de Uso
    let registro_colaborador = Arc::new(CasoUsoRegistroColaborador::nuevo(
        repositorio.clone(),
        repositorio.clone(),
        repositorio.clone(),
    ));

    let registro_usuario = Arc::new(CasoUsoRegistroUsuario::nuevo(
        repositorio.clone(),
    ));

    let login_usuario = Arc::new(CasoUsoLoginUsuario::nuevo(
        repositorio.clone(),
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
    });

    // Configurar Rutas
    let app = ax_routing::crear_rutas(estado)
        .layer(CorsLayer::permissive());

    // Iniciar Servidor
    let puerto = "3000";
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", puerto)).await?;
    println!("Servidor finit iniciado en http://localhost:{}", puerto);
    
    axum::serve(listener, app).await?;

    Ok(())
}
