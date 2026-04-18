use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Urgencia {
    Baja,
    Media,
    Alta,
    Critica,
}

impl Urgencia {
    pub fn a_cadena(&self) -> String {
        match self {
            Urgencia::Baja => "baja".to_string(),
            Urgencia::Media => "media".to_string(),
            Urgencia::Alta => "alta".to_string(),
            Urgencia::Critica => "critica".to_string(),
        }
    }
}
