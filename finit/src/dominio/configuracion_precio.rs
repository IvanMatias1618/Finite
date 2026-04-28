use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ConfiguracionPrecio {
    pub id: Option<i32>,
    pub colaborador_id: i32,
    pub precio_por_kilometro: Decimal,
    pub recargo_lluvia: Decimal,   // Porcentaje o monto fijo (segun logica de negocio)
    pub recargo_domingo: Decimal,
    pub recargo_nocturno: Decimal,
}
