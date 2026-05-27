use crate::dominio::solicitud::EstadoSolicitud;
use crate::dominio::puertos::repositorio_solicitud::RepositorioSolicitud;
use crate::dominio::puertos::repositorio_colaborador::RepositorioColaborador;
use std::error::Error;
use std::sync::Arc;
use std::str::FromStr;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
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

#[derive(Debug, Serialize, Deserialize)]
pub struct DesglosePago {
    pub total: Decimal,
    pub gasto_conekta: Decimal,
    pub base_reparto: Decimal,
    pub reparto_tecnico: Decimal,
    pub reparto_empresa: Decimal,
    pub reparto_mayoral: Decimal,
    pub reparto_socio: Decimal,
    pub impuesto_isr: Decimal,
    pub impuesto_iva: Decimal,
    pub impuesto_imss: Decimal,
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

        // 2. Calcular Desglose (Reglas Mayoral v2)
        let subcat_id = request.subcategoria_id.unwrap_or(solicitud.subcategoria_id);
        let subcategoria = self.repositorio_categoria.buscar_subcategoria_por_id(subcat_id).await?
            .ok_or("Subcategoría no encontrada")?;
        
        let es_flete = subcategoria.nombre.to_uppercase().contains("FLETE");
        let desglose = self.calcular_desglose(solicitud.precio_final, "tarjeta", es_flete);
        
        let monto_centavos = (desglose.total * Decimal::from(100)).to_u64().unwrap_or(0);

        // 3. Obtener Datos del Colaborador para el split
        let colaborador = self.repositorio_colaborador.buscar_por_id(solicitud.colaborador_id).await?
            .ok_or("Colaborador no encontrado")?;
        
        let rec_tecnico = colaborador.conekta_receptor_id.ok_or("El colaborador no tiene ID de receptor de Conekta configurado")?;
        
        // IDs de receptores fijos desde variables de entorno
        let rec_okupo = std::env::var("CONEKTA_RECEPTOR_OKUPO_ID").unwrap_or_else(|_| "rec_default_okupo".to_string());
        let rec_duenno = std::env::var("CONEKTA_RECEPTOR_DUENNO_ID").unwrap_or_else(|_| "rec_default_duenno".to_string());
        let rec_socio = std::env::var("CONEKTA_RECEPTOR_SOCIO_ID").unwrap_or_else(|_| "rec_default_socio".to_string());

        // 4. Distribución de Fondos (Split Payments)
        let monto_tecnico = (desglose.reparto_tecnico * Decimal::from(100)).to_u64().unwrap_or(0);
        let monto_okupo = (desglose.reparto_empresa * Decimal::from(100)).to_u64().unwrap_or(0);
        let monto_duenno = (desglose.reparto_mayoral * Decimal::from(100)).to_u64().unwrap_or(0);
        let monto_socio = (desglose.reparto_socio * Decimal::from(100)).to_u64().unwrap_or(0);

        // 5. Obtener datos del usuario para customer_info
        let usuario = self.repositorio_usuario.buscar_por_id(solicitud.usuario_id).await?
            .ok_or("Usuario no encontrado")?;

        // 6. Crear orden en Conekta
        let payload = serde_json::json!({
            "currency": "MXN",
            "customer_info": {
                "name": usuario.nombre,
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

    pub fn calcular_desglose(&self, precio_total: Decimal, metodo_pago: &str, es_flete: bool) -> DesglosePago {
        let total = precio_total;
        
        // 1. Gasto Conekta
        let gasto_conekta = match metodo_pago {
            "tarjeta" => {
                let comision_base = (total * Decimal::from_str("0.029").unwrap()) + Decimal::from_str("2.50").unwrap();
                comision_base * Decimal::from_str("1.16").unwrap()
            },
            "oxxo" | "efectivo" => Decimal::from_str("13.92").unwrap(),
            "spei" => Decimal::from_str("5.80").unwrap(),
            _ => Decimal::ZERO,
        };

        // 2. Impuestos
        let isr_p = if es_flete { Decimal::from_str("0.021").unwrap() } else { Decimal::from_str("0.01").unwrap() };
        let iva_p = Decimal::from_str("0.08").unwrap();
        let imss_p = Decimal::from_str("0.015").unwrap();

        let impuesto_isr = total * isr_p;
        let impuesto_iva = total * iva_p;
        let impuesto_imss = total * imss_p;
        let total_impuestos = impuesto_isr + impuesto_iva + impuesto_imss;

        // 3. Base Reparto
        let base_reparto = total - gasto_conekta - total_impuestos;

        // 4. Repartos (75/15/5/5)
        let reparto_tecnico = base_reparto * Decimal::from_str("0.75").unwrap();
        let reparto_empresa = base_reparto * Decimal::from_str("0.15").unwrap();
        let reparto_mayoral = base_reparto * Decimal::from_str("0.05").unwrap();
        let reparto_socio = base_reparto * Decimal::from_str("0.05").unwrap();

        DesglosePago {
            total,
            gasto_conekta,
            base_reparto,
            reparto_tecnico,
            reparto_empresa,
            reparto_mayoral,
            reparto_socio,
            impuesto_isr,
            impuesto_iva,
            impuesto_imss,
        }
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
                    println!("💰 PAGO CONFIRMADO: La solicitud #{} ha sido activada. Notificando al técnico #{}...", solicitud.id.unwrap(), solicitud.colaborador_id);
                    // Aquí se dispararía la notificación push/SMS al técnico
                } else {
                    println!("⚠️ WEBHOOK ERROR: No se encontró solicitud para la orden de Conekta: {}", order_id);
                }
            },
            "charge.refunded" => {
                let order_id = evento["data"]["object"]["order_id"].as_str().unwrap_or("");
                if let Some(solicitud) = self.repositorio_solicitud.buscar_por_orden_conekta(order_id).await? {
                    self.repositorio_solicitud.actualizar_estado(solicitud.id.unwrap(), EstadoSolicitud::Cancelado).await?;
                    println!("↩️ REEMBOLSO PROCESADO: La solicitud #{} ha sido cancelada. Notificando al usuario #{} y al técnico #{}...", 
                             solicitud.id.unwrap(), solicitud.usuario_id, solicitud.colaborador_id);
                    // Aquí se dispararían las notificaciones de cancelación
                }
            },
            _ => {
                println!("🔔 Evento de webhook no manejado: {}", tipo);
            }
        }
        
        Ok(())
    }
}
