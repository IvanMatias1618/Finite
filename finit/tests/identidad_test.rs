use finit::dominio::puertos::repositorio_usuario::RepositorioUsuario;
use finit::aplicacion::servicios::registro_usuario::CasoUsoRegistroUsuario;
use finit::aplicacion::servicios::login_usuario::CasoUsoLoginUsuario;
use finit::infraestructura::sqlite_repositorio::RepositorioSQLite;
use sqlx::SqlitePool;
use std::sync::Arc;

#[tokio::test]
async fn test_flujo_completo_identidad() {
    // Usar base de datos en memoria para el test
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    let repositorio = Arc::new(RepositorioSQLite::nuevo(pool));
    repositorio.inicializar_tablas().await.unwrap();

    let registro = CasoUsoRegistroUsuario::nuevo(repositorio.clone());
    let login = CasoUsoLoginUsuario::nuevo(repositorio.clone());

    // 1. Registrar usuario
    let nombre = "Test User".to_string();
    let correo = "test@finit.com".to_string();
    let contrasenna = "secreto123".to_string();

    let id = registro.ejecutar(nombre.clone(), correo.clone(), contrasenna.clone()).await.unwrap();
    assert!(id > 0);

    // 2. Intentar registrar mismo correo (debe fallar)
    let fallo = registro.ejecutar("Otro".into(), correo.clone(), "456".into()).await;
    assert!(fallo.is_err());

    // 3. Login correcto (debe devolver JWT)
    let token = login.ejecutar(correo.clone(), contrasenna.clone()).await.unwrap();
    assert!(!token.is_empty());
    assert!(token.split('.').count() == 3); // Formato JWT: header.payload.signature

    // 4. Login con contrasenna incorrecta (debe fallar)
    let fallo_login = login.ejecutar(correo.clone(), "incorrecta".into()).await;
    assert!(fallo_login.is_err());
}
