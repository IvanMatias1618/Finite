use finit::aplicacion::servicios::consultar_perfil_colaborador::CasoUsoConsultarPerfilColaborador;
use finit::infraestructura::sqlite_repositorio::RepositorioSQLite;
use sqlx::SqlitePool;
use std::sync::Arc;
use rust_decimal::Decimal;

#[tokio::test]
async fn test_consultar_perfil_colaborador() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    let repositorio = Arc::new(RepositorioSQLite::nuevo(pool.clone()));
    repositorio.inicializar_tablas().await.unwrap();

    // 1. Insertar Usuario
    sqlx::query("INSERT INTO usuario (id, nombre, correo, contrasenna) VALUES (1, 'Ivan', 'ivan@test.com', 'password')")
        .execute(&pool).await.unwrap();

    // 2. Insertar Colaborador
    sqlx::query("INSERT INTO colaborador (id, usuario_id, telefono, sitio_web) VALUES (1, 1, '123456789', 'http://test.com')")
        .execute(&pool).await.unwrap();

    // 3. Insertar Servicios
    sqlx::query("INSERT INTO servicio (id, colaborador_id, categoria_id, descripcion, distancia_maxima_kilometros, precio_por_kilometro, latitud, longitud) VALUES (1, 1, 1, 'Servicio de prueba', '10.0', '5.5', '19.4326', '-99.1332')")
        .execute(&pool).await.unwrap();

    let caso_uso = CasoUsoConsultarPerfilColaborador::nuevo(
        repositorio.clone(),
        repositorio.clone(),
        repositorio.clone(),
    );

    let perfil = caso_uso.ejecutar(1).await.unwrap().expect("Perfil debería existir");

    assert_eq!(perfil.id, 1);
    assert_eq!(perfil.nombre, "Ivan");
    assert_eq!(perfil.telefono, "123456789");
    assert_eq!(perfil.sitio_web, Some("http://test.com".to_string()));
    assert_eq!(perfil.servicios.len(), 1);
    assert_eq!(perfil.servicios[0].descripcion, "Servicio de prueba");
    assert_eq!(perfil.servicios[0].distancia_maxima_kilometros, Decimal::new(100, 1)); // 10.0
}
