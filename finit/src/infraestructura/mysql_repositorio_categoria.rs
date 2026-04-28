use crate::dominio::categoria::{Categoria, Subcategoria};
use crate::dominio::puertos::repositorio_categoria::RepositorioCategoria;
use crate::infraestructura::RepositorioMySQL;
use std::error::Error;
use async_trait::async_trait;
use sqlx::MySql;

#[async_trait]
impl RepositorioCategoria for RepositorioMySQL {
    async fn listar(&self) -> Result<Vec<Categoria>, Box<dyn Error + Send + Sync>> {
        let registros = sqlx::query_as::<MySql, Categoria>(
            "SELECT id, nombre FROM categoria"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(registros)
    }

    async fn listar_subcategorias(&self, categoria_id: i32) -> Result<Vec<Subcategoria>, Box<dyn Error + Send + Sync>> {
        let registros = sqlx::query_as::<MySql, Subcategoria>(
            "SELECT id, categoria_id, nombre, descripcion FROM subcategoria WHERE categoria_id = ?"
        )
        .bind(categoria_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(registros)
    }
}
