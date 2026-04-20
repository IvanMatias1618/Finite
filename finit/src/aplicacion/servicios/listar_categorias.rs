use crate::dominio::categoria::Categoria;
use crate::dominio::puertos::repositorio_categoria::RepositorioCategoria;
use std::error::Error;
use std::sync::Arc;

pub struct CasoUsoListarCategorias {
    repositorio_categoria: Arc<dyn RepositorioCategoria>,
}

impl CasoUsoListarCategorias {
    pub fn nuevo(repositorio_categoria: Arc<dyn RepositorioCategoria>) -> Self {
        Self { repositorio_categoria }
    }

    pub async fn ejecutar(&self) -> Result<Vec<Categoria>, Box<dyn Error + Send + Sync>> {
        self.repositorio_categoria.listar().await
    }
}
