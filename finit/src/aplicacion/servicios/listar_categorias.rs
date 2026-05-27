use crate::dominio::categoria::Categoria;
use crate::dominio::puertos::repositorio_categoria::RepositorioCategoria;
use std::sync::Arc;
use std::error::Error;

pub struct CasoUsoListarCategorias {
    repo_categoria: Arc<dyn RepositorioCategoria>,
}

impl CasoUsoListarCategorias {
    pub fn nuevo(repo_categoria: Arc<dyn RepositorioCategoria>) -> Self {
        Self { repo_categoria }
    }

    pub async fn ejecutar(&self) -> Result<Vec<Categoria>, Box<dyn Error + Send + Sync>> {
        // Ahora solo devolvemos las categorias padres, sin anidar subcategorias (Lazy Load)
        self.repo_categoria.listar().await
    }
}
