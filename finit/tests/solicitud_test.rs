use finit::aplicacion::servicios::listar_solicitudes::CasoUsoListarSolicitudes;
use finit::infraestructura::sqlite_repositorio::RepositorioSQLite;
use finit::dominio::solicitud::{SolicitudServicio, EstadoSolicitud};
use finit::dominio::urgencia::Urgencia;
use sqlx::SqlitePool;
use std::sync::Arc;
use rust_decimal::Decimal;

#[tokio::test]
async fn test_listar_solicitudes() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    let repositorio = Arc::new(RepositorioSQLite::nuevo(pool.clone()));
    repositorio.inicializar_tablas().await.unwrap();

    // 1. Insertar Datos de prueba
    sqlx::query("INSERT INTO usuario (id, nombre, correo) VALUES (1, 'User 1', 'u1@t.com'), (2, 'User 2', 'u2@t.com')")
        .execute(&pool).await.unwrap();
    
    // Insertar un par de solicitudes para el usuario 1
    sqlx::query("INSERT INTO solicitud_servicio (usuario_id, servicio_id, urgencia, precio_final, estado, fecha_creacion) 
                 VALUES (1, 10, 'alta', '500.0', 'en_espera_de_pago', '2023-10-27 10:00:00')")
        .execute(&pool).await.unwrap();
    
    sqlx::query("INSERT INTO solicitud_servicio (usuario_id, servicio_id, urgencia, precio_final, estado, fecha_creacion) 
                 VALUES (1, 11, 'baja', '200.0', 'en_espera_de_pago', '2023-10-27 11:00:00')")
        .execute(&pool).await.unwrap();

    // Insertar una para el usuario 2
    sqlx::query("INSERT INTO solicitud_servicio (usuario_id, servicio_id, urgencia, precio_final, estado, fecha_creacion) 
                 VALUES (2, 20, 'critica', '1000.0', 'en_espera_de_pago', '2023-10-27 12:00:00')")
        .execute(&pool).await.unwrap();

    let caso_uso = CasoUsoListarSolicitudes::nuevo(repositorio.clone());

    // Probar listar todas
    let todas = caso_uso.ejecutar(None).await.unwrap();
    assert_eq!(todas.len(), 3);

    // Probar listar por usuario 1
    let user1 = caso_uso.ejecutar(Some(1)).await.unwrap();
    assert_eq!(user1.len(), 2);
    assert_eq!(user1[0].usuario_id, 1);
    assert_eq!(user1[0].urgencia, Urgencia::Alta);

    // Probar listar por usuario 2
    let user2 = caso_uso.ejecutar(Some(2)).await.unwrap();
    assert_eq!(user2.len(), 1);
    assert_eq!(user2[0].usuario_id, 2);
    assert_eq!(user2[0].urgencia, Urgencia::Critica);
}
