pub mod dominio;
pub mod aplicacion;
pub mod infraestructura;

use std::sync::Arc;
use crate::infraestructura::api::rutas::{self as ax_routing, EstadoApp};
use crate::aplicacion::servicios::registro_colaborador::CasoUsoRegistroColaborador;
use crate::aplicacion::servicios::registro_usuario::CasoUsoRegistroUsuario;
use crate::infraestructura::sqlite_repositorio::RepositorioSQLite;
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;
use axum::{Router, routing::get};

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

    let estado = Arc::new(EstadoApp {
        registro_colaborador,
        registro_usuario,
    });

    // Configurar Rutas
    let app = Router::new()
        .route("/", get(|| async { "Bienvenido al motor finit - API activa (okupo.db)" }))
        .merge(ax_routing::crear_rutas(estado))
        .layer(CorsLayer::permissive());
    // Iniciar Servidor
    let puerto = "3000";
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", puerto)).await?;
    println!("Servidor finit iniciado en http://localhost:{}", puerto);
    
    axum::serve(listener, app).await?;

    Ok(())
}
