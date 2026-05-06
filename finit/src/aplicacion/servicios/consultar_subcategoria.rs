use crate::dominio::categoria::Subcategoria;
use crate::dominio::puertos::repositorio_categoria::RepositorioCategoria;
use std::sync::Arc;
use std::error::Error;

pub struct CasoUsoConsultarSubcategoria {
    repo_categoria: Arc<dyn RepositorioCategoria>,
}

impl CasoUsoConsultarSubcategoria {
    pub fn nuevo(repo_categoria: Arc<dyn RepositorioCategoria>) -> Self {
        Self { repo_categoria }
    }

    pub async fn ejecutar(&self, id: i32) -> Result<Option<Subcategoria>, Box<dyn Error + Send + Sync>> {
        self.repo_categoria.buscar_subcategoria_por_id(id).await
    }
}
