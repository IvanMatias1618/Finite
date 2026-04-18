pub mod dominio;
pub mod aplicacion;
pub mod infraestructura;

use std::sync::Arc;
use crate::infraestructura::api::rutas as ax_routing;
use crate::aplicacion::servicios::registro_colaborador::CasoUsoRegistroColaborador;
use crate::infraestructura::sqlite_repositorio::RepositorioSQLite;
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;
use axum::{Router, routing::get};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Iniciando motor finit con SQLite para pruebas locales...");
    
    // Usar SQLite en memoria para máxima velocidad y cero dependencias externas en la prueba
    let pool = SqlitePool::connect("sqlite::memory:").await?;

    let repositorio = Arc::new(RepositorioSQLite::nuevo(pool));
    
    // Inicializar tablas en SQLite
    repositorio.inicializar_tablas().await?;

    // Inicializar Casos de Uso
    let caso_uso_registro = Arc::new(CasoUsoRegistroColaborador::nuevo(
        repositorio.clone(),
        repositorio.clone(),
        repositorio.clone(),
    ));

    // Configurar Rutas
    let app = Router::new()
        .route("/", get(|| async { "Bienvenido al motor finit - API activa (Modo Prueba SQLite)" }))
        .merge(ax_routing::crear_rutas(caso_uso_registro))
        .layer(CorsLayer::permissive());

    // Iniciar Servidor
    let puerto = "3000";
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", puerto)).await?;
    println!("Servidor finit iniciado en http://localhost:{}", puerto);
    
    axum::serve(listener, app).await?;

    Ok(())
}
