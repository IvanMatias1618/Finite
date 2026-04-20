use finit::aplicacion::servicios::listar_categorias::CasoUsoListarCategorias;
use finit::infraestructura::sqlite_repositorio::RepositorioSQLite;
use sqlx::SqlitePool;
use std::sync::Arc;

#[tokio::test]
async fn test_listar_categorias() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    let repositorio = Arc::new(RepositorioSQLite::nuevo(pool.clone()));
    repositorio.inicializar_tablas().await.unwrap();

    // Insertar categorias de prueba
    sqlx::query("INSERT INTO categoria (nombre) VALUES ('Fontaneria'), ('Electricidad')")
        .execute(&pool).await.unwrap();

    let caso_uso = CasoUsoListarCategorias::nuevo(repositorio.clone());
    let categorias = caso_uso.ejecutar().await.unwrap();

    assert_eq!(categorias.len(), 2);
    assert_eq!(categorias[0].nombre, "Fontaneria");
    assert_eq!(categorias[1].nombre, "Electricidad");
}
