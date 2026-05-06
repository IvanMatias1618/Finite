use finit::aplicacion::servicios::listar_colaboradores_marketplace::CasoUsoListarColaboradoresMarketplace;
use finit::infraestructura::sqlite_repositorio::RepositorioSQLite;
use sqlx::SqlitePool;
use std::sync::Arc;
use rust_decimal::Decimal;

#[tokio::test]
async fn test_marketplace_colaboradores() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    let repositorio = Arc::new(RepositorioSQLite::nuevo(pool.clone()));
    repositorio.inicializar_tablas().await.unwrap();

    // 1. Setup Datos
    sqlx::query("INSERT INTO categoria (id, nombre) VALUES (1, 'Hogar')").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO subcategoria (id, categoria_id, nombre) VALUES (1, 1, 'Fontaneria')").execute(&pool).await.unwrap();
    
    // Colaborador 1
    sqlx::query("INSERT INTO usuario (id, nombre, correo, contrasenna) VALUES (1, 'Colab Barato', 'b@t.com', 'p')").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO colaborador (id, usuario_id, telefono) VALUES (1, 1, '1')").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO servicio (id, colaborador_id, subcategoria_id, descripcion, distancia_maxima_kilometros, precio_por_kilometro, latitud, longitud) 
                 VALUES (1, 1, 1, 'Servicio Barato', '10.0', '1.0', '19.0', '-99.0')").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO precio_servicio_urgencia (servicio_id, urgencia, precio) VALUES (1, 'baja', '50.0')").execute(&pool).await.unwrap();

    // Colaborador 2
    sqlx::query("INSERT INTO usuario (id, nombre, correo, contrasenna) VALUES (2, 'Colab Caro', 'c@t.com', 'p')").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO colaborador (id, usuario_id, telefono) VALUES (2, 2, '2')").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO servicio (id, colaborador_id, subcategoria_id, descripcion, distancia_maxima_kilometros, precio_por_kilometro, latitud, longitud) 
                 VALUES (2, 2, 1, 'Servicio Caro', '10.0', '1.0', '19.0', '-99.0')").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO precio_servicio_urgencia (servicio_id, urgencia, precio) VALUES (2, 'baja', '150.0')").execute(&pool).await.unwrap();

    let caso_uso = CasoUsoListarColaboradoresMarketplace::nuevo(
        repositorio.clone(),
        repositorio.clone(),
        repositorio.clone(),
    );

    let lat = Decimal::new(190, 1); // 19.0
    let lon = Decimal::new(-990, 1); // -99.0

    let resultados = caso_uso.ejecutar(1, lat, lon).await.unwrap();

    assert_eq!(resultados.len(), 2);
    assert_eq!(resultados[0].nombre, "Colab Barato");
    assert_eq!(resultados[0].precio_base, Decimal::new(50, 0));
    assert_eq!(resultados[1].nombre, "Colab Caro");
}
