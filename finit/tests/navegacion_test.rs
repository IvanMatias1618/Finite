use finit::aplicacion::servicios::listar_categorias::CasoUsoListarCategorias;
use finit::aplicacion::servicios::listar_subcategorias::CasoUsoListarSubcategorias;
use finit::infraestructura::sqlite_repositorio::RepositorioSQLite;
use sqlx::SqlitePool;
use std::sync::Arc;

#[tokio::test]
async fn test_listar_categorias() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    let repositorio = Arc::new(RepositorioSQLite::nuevo(pool.clone()));
    repositorio.inicializar_tablas().await.unwrap();

    // Insertar categorias y subcategorias de prueba
    sqlx::query("INSERT INTO categoria (id, nombre) VALUES (1, 'Fontaneria'), (2, 'Electricidad')")
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO subcategoria (categoria_id, nombre) VALUES (1, 'Fugas'), (1, 'Instalaciones')")
        .execute(&pool).await.unwrap();

    let caso_uso = CasoUsoListarCategorias::nuevo(repositorio.clone());
    let categorias = caso_uso.ejecutar().await.unwrap();

    assert_eq!(categorias.len(), 2);
    assert_eq!(categorias[0].nombre, "Fontaneria");
    // Verificamos que no trae subcategorias (Lazy Load)
    assert!(categorias[0].subcategorias.is_none());
    assert_eq!(categorias[1].nombre, "Electricidad");
}

#[tokio::test]
async fn test_listar_subcategorias() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    let repositorio = Arc::new(RepositorioSQLite::nuevo(pool.clone()));
    repositorio.inicializar_tablas().await.unwrap();

    sqlx::query("INSERT INTO categoria (id, nombre) VALUES (1, 'Fontaneria')")
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO subcategoria (categoria_id, nombre) VALUES (1, 'Fugas'), (1, 'Instalaciones')")
        .execute(&pool).await.unwrap();

    let caso_uso = CasoUsoListarSubcategorias::nuevo(repositorio.clone());
    let subcategorias = caso_uso.ejecutar(1).await.unwrap();

    assert_eq!(subcategorias.len(), 2);
    assert_eq!(subcategorias[0].nombre, "Fugas");
    assert_eq!(subcategorias[1].nombre, "Instalaciones");
}
