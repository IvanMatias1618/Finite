use std::sync::Arc;
use std::error::Error;
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
use finit::aplicacion::servicios::gestionar_soporte::CasoUsoGestionarSoporte;
use finit::aplicacion::servicios::actualizar_documentacion::CasoUsoActualizarDocumentacion;
use finit::aplicacion::servicios::configurar_precios_dinamicos::CasoUsoConfigurarPreciosDinamicos;
use finit::aplicacion::servicios::configurar_horarios::CasoUsoConfigurarHorarios;
use finit::aplicacion::servicios::calificar_servicio::CasoUsoCalificarServicio;
use finit::aplicacion::servicios::registrar_servicio_tecnico::CasoUsoRegistrarServicioTecnico;
use finit::aplicacion::servicios::consultar_estadisticas_colaborador::CasoUsoConsultarEstadisticasColaborador;
use finit::aplicacion::servicios::gestionar_estado_solicitud::CasoUsoGestionarEstadoSolicitud;
use finit::aplicacion::servicios::cotizar_servicio::CasoUsoCotizarServicio;
use finit::aplicacion::servicios::gestionar_verificacion::CasoUsoGestionarVerificacion;
use finit::aplicacion::servicios::cotizacion_especial::CasoUsoCotizacionEspecial;
use finit::aplicacion::servicios::login_social::CasoUsoLoginSocial;
use finit::infraestructura::social::google::GoogleProvider;
use finit::infraestructura::social::facebook::FacebookProvider;
use finit::infraestructura::{RepositorioMySQL, sqlite_repositorio::RepositorioSQLite};
use finit::infraestructura::api::rutas as ax_routing;
use finit::infraestructura::api::rutas::EstadoApp;
use finit::dominio::puertos::repositorio_motor::RepositorioMotor;
use sqlx::{MySqlPool, SqlitePool};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenvy::dotenv().ok();

    let args: Vec<String> = std::env::args().collect();
    
    // Si no hay argumentos (solo el nombre del binario), mostramos el help
    if args.len() == 1 {
        mostrar_ayuda();
        return Ok(());
    }

    match args[1].as_str() {
        "run" => iniciar_servidor().await,
        "--reset-db" => resetear_base_de_datos().await,
        "--help" | "-h" => {
            mostrar_ayuda();
            Ok(())
        },
        _ => {
            println!("❌ Comando no reconocido: {}", args[1]);
            mostrar_ayuda();
            Ok(())
        }
    }
}

fn mostrar_ayuda() {
    println!("--- 🚀 MOTOR FINIT CLI ---");
    println!("Uso: cargo run [comando]");
    println!("");
    println!("Comandos disponibles:");
    println!("  run         Inicia el servidor API");
    println!("  --reset-db  Limpia la base de datos y crea un usuario administrador");
    println!("  --help      Muestra este resumen de comandos");
    println!("");
    println!("Configuración:");
    println!("  Asegúrate de configurar el archivo .env con DATABASE_URL.");
    println!("  Soporta MySQL (mysql://...) y SQLite (sqlite:...)");
}

async fn resetear_base_de_datos() -> Result<(), Box<dyn Error + Send + Sync>> {
    let repo = obtener_repositorio().await?;
    let admin_pass = std::env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin".to_string());
    
    println!("🧹 Iniciando reset de base de datos...");
    repo.limpiar_y_sembrar(&admin_pass).await?;
    println!("👋 Proceso de reset finalizado.");
    Ok(())
}

async fn obtener_repositorio() -> Result<Arc<dyn RepositorioMotor>, Box<dyn Error + Send + Sync>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL debe estar configurada");
    
    if database_url.starts_with("mysql://") {
        println!("🗄️ Usando motor MySQL");
        let pool = MySqlPool::connect(&database_url).await?;
        Ok(Arc::new(RepositorioMySQL::nuevo(pool)))
    } else if database_url.starts_with("sqlite:") {
        println!("🗄️ Usando motor SQLite");
        let pool = SqlitePool::connect(&database_url).await?;
        let repo = RepositorioSQLite::nuevo(pool);
        Ok(Arc::new(repo))
    } else {
        Err("Protocolo de base de datos no soportado. Use mysql:// o sqlite:".into())
    }
}

async fn iniciar_servidor() -> Result<(), Box<dyn Error + Send + Sync>> {
    let repositorio = obtener_repositorio().await?;
    
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "secreto_por_defecto_desarrollo".to_string());
    let puerto = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string());

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
        jwt_secret.clone(),
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

    let gestionar_soporte = Arc::new(CasoUsoGestionarSoporte::nuevo(
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

    let cotizacion_especial = Arc::new(CasoUsoCotizacionEspecial::nuevo(
        repositorio.clone(),
    ));

    let proveedor_google = Arc::new(GoogleProvider);
    let proveedor_facebook = Arc::new(FacebookProvider);

    let login_social = Arc::new(CasoUsoLoginSocial::nuevo(
        repositorio.clone(),
        proveedor_google,
        proveedor_facebook,
        jwt_secret.clone(),
    ));

    let conekta_api_key = std::env::var("CONEKTA_PRIVATE_KEY").unwrap_or_else(|_| "key_sandbox_default".to_string());
    let gestion_pagos = Arc::new(finit::aplicacion::servicios::gestion_pagos::CasoUsoGestionPagos::nuevo(
        repositorio.clone(),
        repositorio.clone(),
        repositorio.clone(),
        repositorio.clone(),
        conekta_api_key,
    ));

    let estado = Arc::new(EstadoApp {
        repositorio: repositorio.clone(),
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
        gestionar_soporte,
        actualizar_documentacion,
        configurar_precios_dinamicos,
        configurar_horarios,
        calificar_servicio,
        registrar_servicio_tecnico,
        consultar_estadisticas_colaborador,
        gestionar_estado_solicitud,
        cotizar_servicio,
        gestionar_verificacion,
        cotizacion_especial,
        login_social,
        gestion_pagos,
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
