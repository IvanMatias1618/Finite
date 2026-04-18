#[cfg(test)]
mod tests {
    use sqlx::MySqlPool;
    use std::env;

    #[tokio::test]
    async fn prueba_tabla_temporal() -> Result<(), Box<dyn std::error::Error>> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL debe estar configurada");
        let pool = MySqlPool::connect(&database_url).await?;

        // Crear una tabla temporal para pruebas
        sqlx::query("CREATE TEMPORARY TABLE prueba_finit (id INT PRIMARY KEY, valor VARCHAR(255))")
            .execute(&pool)
            .await?;

        // Insertar datos de prueba
        sqlx::query("INSERT INTO prueba_finit (id, valor) VALUES (?, ?)")
            .bind(1)
            .bind("Prueba de concepto")
            .execute(&pool)
            .await?;

        // Verificar datos
        let row: (i32, String) = sqlx::query_as("SELECT id, valor FROM prueba_finit WHERE id = ?")
            .bind(1)
            .fetch_one(&pool)
            .await?;

        assert_eq!(row.0, 1);
        assert_eq!(row.1, "Prueba de concepto");

        println!("Prueba con tabla temporal exitosa");
        Ok(())
    }
}
