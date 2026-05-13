use crate::dominio::solicitud::EstadoSolicitud;
use crate::dominio::puertos::repositorio_solicitud::RepositorioSolicitud;
use crate::dominio::puertos::repositorio_colaborador::RepositorioColaborador;
use std::error::Error;
use std::sync::Arc;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmacionPagoRequest {
    pub solicitud_id: i32,
    pub conekta_token: String,
    pub subcategoria_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmacionPagoResponse {
    pub status: String,
    pub message: String,
    pub conekta_order_id: Option<String>,
}

pub struct CasoUsoGestionPagos {
    repositorio_solicitud: Arc<dyn RepositorioSolicitud>,
    repositorio_colaborador: Arc<dyn RepositorioColaborador>,
    repositorio_categoria: Arc<dyn crate::dominio::puertos::repositorio_categoria::RepositorioCategoria>,
    repositorio_usuario: Arc<dyn crate::dominio::puertos::repositorio_usuario::RepositorioUsuario>,
    cliente_http: Client,
    conekta_api_key: String,
}

impl CasoUsoGestionPagos {
    pub fn nuevo(
        repositorio_solicitud: Arc<dyn RepositorioSolicitud>,
        repositorio_colaborador: Arc<dyn RepositorioColaborador>,
        repositorio_categoria: Arc<dyn crate::dominio::puertos::repositorio_categoria::RepositorioCategoria>,
        repositorio_usuario: Arc<dyn crate::dominio::puertos::repositorio_usuario::RepositorioUsuario>,
        conekta_api_key: String,
    ) -> Self {
        Self {
            repositorio_solicitud,
            repositorio_colaborador,
            repositorio_categoria,
            repositorio_usuario,
            cliente_http: Client::new(),
            conekta_api_key,
        }
    }

    pub async fn confirmar_pago(
        &self,
        request: ConfirmacionPagoRequest,
    ) -> Result<ConfirmacionPagoResponse, Box<dyn Error + Send + Sync>> {
        // 1. Obtener solicitud
        let solicitud = self.repositorio_solicitud.buscar_por_id(request.solicitud_id).await?
            .ok_or("Solicitud no encontrada")?;

        // 2. Validar Precio (Requerimiento 1.A.1)
        let subcat_id = request.subcategoria_id.unwrap_or(solicitud.subcategoria_id);
        let subcategoria = self.repositorio_categoria.buscar_subcategoria_por_id(subcat_id).await?
            .ok_or("Subcategoría no encontrada")?;
        
        let precio_total = subcategoria.precio_base;
        let monto_centavos = (precio_total * Decimal::from(100)).to_string().parse::<u64>().unwrap_or(0);

        // 3. Obtener Datos del Colaborador para el split
        let colaborador = self.repositorio_colaborador.buscar_por_id(solicitud.colaborador_id).await?
            .ok_or("Colaborador no encontrado")?;
        
        let rec_tecnico = colaborador.conekta_receptor_id.ok_or("El colaborador no tiene ID de receptor de Conekta configurado")?;
        
        // IDs de receptores fijos desde variables de entorno
        let rec_okupo = std::env::var("CONEKTA_RECEPTOR_OKUPO_ID").unwrap_or_else(|_| "rec_default_okupo".to_string());
        let rec_duenno = std::env::var("CONEKTA_RECEPTOR_DUENNO_ID").unwrap_or_else(|_| "rec_default_duenno".to_string());
        let rec_socio = std::env::var("CONEKTA_RECEPTOR_SOCIO_ID").unwrap_or_else(|_| "rec_default_socio".to_string());

        // 4. Distribución de Fondos (Split Payments) (Requerimiento 1.A.2)
        // Técnico (75%): Aplicar deducciones: monto = (total * 0.75) * (1 - 0.105)
        let factor_tecnico = Decimal::from_str_radix("0.75", 10).unwrap() * (Decimal::ONE - Decimal::from_str_radix("0.105", 10).unwrap());
        let monto_tecnico = (precio_total * factor_tecnico * Decimal::from(100)).to_string().parse::<u64>().unwrap_or(0);
        
        // Okupo Clic (15%): monto = total * 0.15
        let monto_okupo = (precio_total * Decimal::from_str_radix("0.15", 10).unwrap() * Decimal::from(100)).to_string().parse::<u64>().unwrap_or(0);
        
        // Dueño (5%): monto = total * 0.05
        let monto_duenno = (precio_total * Decimal::from_str_radix("0.05", 10).unwrap() * Decimal::from(100)).to_string().parse::<u64>().unwrap_or(0);
        
        // Socio (5%): monto = total * 0.05
        let monto_socio = (precio_total * Decimal::from_str_radix("0.05", 10).unwrap() * Decimal::from(100)).to_string().parse::<u64>().unwrap_or(0);

        // 5. Obtener datos del usuario para customer_info
        let usuario = self.repositorio_usuario.buscar_por_id(solicitud.usuario_id).await?
            .ok_or("Usuario no encontrado")?;

        // 6. Crear orden en Conekta
        let payload = serde_json::json!({
            "currency": "MXN",
            "customer_info": {
                "name": usuario.nombre_completo,
                "email": usuario.correo,
                "phone": "+525555555555" // Placeholder o usar telefono de usuario si existe
            },
            "line_items": [{
                "name": subcategoria.nombre,
                "unit_price": monto_centavos,
                "quantity": 1
            }],
            "charges": [{
                "payment_method": {
                    "type": "card",
                    "token_id": request.conekta_token
                }
            }],
            "split_rules": [
                { "amount": monto_tecnico, "receiver": rec_tecnico },
                { "amount": monto_okupo, "receiver": rec_okupo },
                { "amount": monto_duenno, "receiver": rec_duenno },
                { "amount": monto_socio, "receiver": rec_socio }
            ]
        });

        let response = self.cliente_http.post("https://api.conekta.io/orders")
            .basic_auth(&self.conekta_api_key, Some(""))
            .header("Accept", "application/vnd.conekta-v2.1.0+json")
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Ok(ConfirmacionPagoResponse {
                status: "error".to_string(),
                message: format!("Error en Conekta: {}", error_text),
                conekta_order_id: None,
            });
        }

        let conekta_res: serde_json::Value = response.json().await?;
        let conekta_order_id = conekta_res["id"].as_str().ok_or("ID de orden no devuelto por Conekta")?.to_string();

        // 7. Actualizar solicitud
        self.repositorio_solicitud.actualizar_orden_conekta(solicitud.id.unwrap(), conekta_order_id.clone()).await?;
        
        // Cambiar a PAGADO si el cargo fue exitoso inmediatamente (aunque el webhook lo confirmara)
        // Pero segun el requerimiento 2: "transicionar automáticamente el pedido de estado PENDIENTE_PAGO a PAGADO"
        self.repositorio_solicitud.actualizar_estado(solicitud.id.unwrap(), EstadoSolicitud::Pagado).await?;

        Ok(ConfirmacionPagoResponse {
            status: "aprobado".to_string(),
            message: "Pago procesado y fondos distribuidos correctamente".to_string(),
            conekta_order_id: Some(conekta_order_id),
        })
    }

    pub async fn procesar_webhook(
        &self,
        evento: serde_json::Value,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let tipo = evento["type"].as_str().unwrap_or("");
        
        match tipo {
            "order.paid" => {
                let order_id = evento["data"]["object"]["id"].as_str().unwrap_or("");
                if let Some(solicitud) = self.repositorio_solicitud.buscar_por_orden_conekta(order_id).await? {
                    self.repositorio_solicitud.actualizar_estado(solicitud.id.unwrap(), EstadoSolicitud::Pagado).await?;
                    println!("💰 Pago confirmado y estado actualizado para la solicitud: {}", solicitud.id.unwrap());
                } else {
                    println!("⚠️ No se encontró solicitud para la orden de Conekta: {}", order_id);
                }
            },
            "charge.refunded" => {
                let order_id = evento["data"]["object"]["order_id"].as_str().unwrap_or("");
                if let Some(solicitud) = self.repositorio_solicitud.buscar_por_orden_conekta(order_id).await? {
                    self.repositorio_solicitud.actualizar_estado(solicitud.id.unwrap(), EstadoSolicitud::Cancelado).await?;
                    println!("↩️ Reembolso procesado y solicitud cancelada: {}", solicitud.id.unwrap());
                }
            },
            _ => {
                println!("🔔 Evento de webhook no manejado: {}", tipo);
            }
        }
        
        Ok(())
    }
}
