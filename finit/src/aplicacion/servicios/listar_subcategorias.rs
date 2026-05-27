use crate::dominio::categoria::Subcategoria;
use crate::dominio::puertos::repositorio_categoria::RepositorioCategoria;
use std::sync::Arc;
use std::error::Error;

pub struct CasoUsoListarSubcategorias {
    repo_categoria: Arc<dyn RepositorioCategoria>,
}

impl CasoUsoListarSubcategorias {
    pub fn nuevo(repo_categoria: Arc<dyn RepositorioCategoria>) -> Self {
        Self { repo_categoria }
    }

    pub async fn ejecutar(&self, categoria_id: i32) -> Result<Vec<Subcategoria>, Box<dyn Error + Send + Sync>> {
        self.repo_categoria.listar_subcategorias(categoria_id).await
    }
}
