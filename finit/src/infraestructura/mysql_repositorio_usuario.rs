use crate::dominio::usuario::Usuario;
use crate::dominio::puertos::repositorio_usuario::RepositorioUsuario;
use crate::infraestructura::RepositorioMySQL;
use std::error::Error;
use async_trait::async_trait;
use sqlx::MySql;

#[async_trait]
impl RepositorioUsuario for RepositorioMySQL {
    async fn guardar(&self, usuario: Usuario) -> Result<Usuario, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query(
            "INSERT INTO usuario (nombre, correo, contrasenna) VALUES (?, ?, ?)"
        )
        .bind(&usuario.nombre)
        .bind(&usuario.correo)
        .bind(&usuario.contrasenna)
        .execute(&self.pool)
        .await?;

        let id = resultado.last_insert_id() as i32;
        Ok(Usuario {
            id: Some(id),
            ..usuario
        })
    }

    async fn buscar_por_id(&self, id: i32) -> Result<Option<Usuario>, Box<dyn Error + Send + Sync>> {
        let registro = sqlx::query_as::<MySql, Usuario>(
            "SELECT id, nombre, correo, contrasenna FROM usuario WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(registro)
    }

    async fn buscar_por_correo(&self, correo: &str) -> Result<Option<Usuario>, Box<dyn Error + Send + Sync>> {
        let registro = sqlx::query_as::<MySql, Usuario>(
            "SELECT id, nombre, correo, contrasenna FROM usuario WHERE correo = ?"
        )
        .bind(correo)
        .fetch_optional(&self.pool)
        .await?;

        Ok(registro)
    }
}
