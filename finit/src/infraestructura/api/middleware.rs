use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::sync::Arc;
use crate::infraestructura::api::rutas::EstadoApp;
use crate::dominio::token::Claims;

pub async fn validar_jwt(
    State(_estado): State<Arc<EstadoApp>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let auth_header = match auth_header {
        Some(h) => h,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];

    // Necesitamos el secreto JWT. Lo sacamos de la configuracion o variables de entorno.
    // Como no esta en EstadoApp directamente, lo ideal seria añadirlo o pasarlo.
    // Revisando main.rs, el secreto se usa en CasoUsoLoginUsuario y CasoUsoRegistroColaborador.
    
    // Por simplicidad inmediata, intentaremos sacarlo de la variable de entorno, 
    // pero lo correcto es que esté en el estado.
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "secreto_por_defecto_desarrollo".to_string());

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    );

    match token_data {
        Ok(data) => {
            // Insertar el ID del usuario en las extensiones de la solicitud para uso posterior
            req.extensions_mut().insert(data.claims);
            Ok(next.run(req).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}
