use crate::dominio::puertos::repositorio_servicio::RepositorioServicio;
use crate::dominio::puertos::repositorio_colaborador::RepositorioColaborador;
use crate::dominio::puertos::repositorio_usuario::RepositorioUsuario;
use crate::dominio::urgencia::Urgencia;
use serde::Serialize;
use std::sync::Arc;
use std::error::Error;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

#[derive(Serialize)]
pub struct ColaboradorMarketplace {
    pub colaborador_id: i32,
    pub nombre: String,
    pub descripcion_servicio: String,
    pub precio_base: Decimal,
    pub distancia_km: Decimal,
}

pub struct CasoUsoListarColaboradoresMarketplace {
    repo_servicio: Arc<dyn RepositorioServicio>,
    repo_colaborador: Arc<dyn RepositorioColaborador>,
    repo_usuario: Arc<dyn RepositorioUsuario>,
}

impl CasoUsoListarColaboradoresMarketplace {
    pub fn nuevo(
        repo_servicio: Arc<dyn RepositorioServicio>,
        repo_colaborador: Arc<dyn RepositorioColaborador>,
        repo_usuario: Arc<dyn RepositorioUsuario>,
    ) -> Self {
        Self { repo_servicio, repo_colaborador, repo_usuario }
    }

    pub async fn ejecutar(
        &self,
        subcategoria_id: i32,
        latitud: Decimal,
        longitud: Decimal,
    ) -> Result<Vec<ColaboradorMarketplace>, Box<dyn Error + Send + Sync>> {
        let lat_f = latitud.to_f64().unwrap_or(0.0);
        let lon_f = longitud.to_f64().unwrap_or(0.0);

        let servicios = self.repo_servicio
            .buscar_por_categoria_y_cercania(subcategoria_id, lat_f, lon_f)
            .await?;

        let mut lista = Vec::new();

        for s in servicios {
            if let Some(colab) = self.repo_colaborador.buscar_por_id(s.colaborador_id).await? {
                if let Some(user) = self.repo_usuario.buscar_por_id(colab.usuario_id).await? {
                    // Obtenemos el precio para urgencia baja como "precio desde"
                    let precio = self.repo_servicio
                        .buscar_precio_por_servicio_y_urgencia(s.id.unwrap(), Urgencia::Baja)
                        .await?
                        .unwrap_or(Decimal::ZERO);

                    // Calculamos distancia simple (el repo ya filtró por cercanía, pero aquí podríamos refinar)
                    // Por ahora usamos la distancia que el motor de matching ya calcula o una estimación.
                    
                    lista.push(ColaboradorMarketplace {
                        colaborador_id: colab.id.unwrap(),
                        nombre: user.nombre,
                        descripcion_servicio: s.descripcion,
                        precio_base: precio,
                        distancia_km: Decimal::ZERO, // TODO: Implementar cálculo real aquí o en el repo
                    });
                }
            }
        }

        // Ordenar por precio base ascendente (simplificado)
        lista.sort_by(|a, b| a.precio_base.cmp(&b.precio_base));

        Ok(lista)
    }
}
