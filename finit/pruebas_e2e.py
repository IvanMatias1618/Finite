import requests
import json
import random
import time
import sys

# --- CONFIGURACIÓN ---
URL_BASE = "http://localhost:3000"

def generar_datos_aleatorios():
    id_unico = random.randint(1000, 9999)
    return {
        "nombre": f"Usuario Probeta {id_unico}",
        "correo": f"test_{id_unico}@ejemplo.com",
        "contrasenna": "password123"
    }

def ejecutar_pruebas():
    print("🚀 Iniciando Pruebas de Integracion End-to-End para finit...\n")
    
    datos_usuario = generar_datos_aleatorios()
    token = None
    usuario_id = None
    colaborador_id = None
    solicitud_id = None

    try:
        # 1. REGISTRO DE USUARIO
        print(f"1. Registrando usuario: {datos_usuario['correo']}...")
        res = requests.post(f"{URL_BASE}/usuarios", json=datos_usuario)
        if res.status_code not in [200, 201]:
            print(f"❌ Error al registrar usuario: {res.text}")
            sys.exit(1)
        usuario_id = res.json()
        print(f"✅ Usuario registrado con ID: {usuario_id}")

        # 2. LOGIN
        print("2. Iniciando sesion para obtener Token JWT...")
        res = requests.post(f"{URL_BASE}/login", json={
            "correo": datos_usuario["correo"],
            "contrasenna": datos_usuario["contrasenna"]
        })
        if res.status_code != 200:
            print(f"❌ Error en login: {res.text}")
            sys.exit(1)
        token = res.text.strip('"')
        print(f"✅ Login exitoso. Token: {token[:20]}...")

        # 3. REGISTRO DE COLABORADOR
        print("3. Promoviendo usuario a Colaborador...")
        # Corregido: Usamos los campos reales de la estructura 'Servicio' en Rust
        servicios_col = [
            (
                {
                    "colaborador_id": 0,
                    "subcategoria_id": 1,
                    "descripcion": "Servicio de Cerrajería Express",
                    "distancia_maxima_kilometros": "20.0",
                    "precio_por_kilometro": "2.5",
                    "latitud": "19.4326",
                    "longitud": "-99.1332"
                },
                [
                    { "servicio_id": 0, "urgencia": "media", "precio": "75.50" },
                    { "servicio_id": 0, "urgencia": "alta", "precio": "120.00" }
                ]
            )
        ]
        
        datos_col = {
            "token_usuario": token,
            "telefono": "5512345678",
            "sitio_web": "http://probeta.com",
            "servicios": servicios_col
        }
        
        res = requests.post(f"{URL_BASE}/colaboradores", json=datos_col)
        if res.status_code not in [200, 201]:
            print(f"❌ Error al registrar colaborador: {res.text}")
            sys.exit(1)
        colaborador_id = res.json()
        print(f"✅ Colaborador registrado con ID: {colaborador_id}")

        # 4. VERIFICACIÓN EN MARKETPLACE
        print("4. Verificando presencia en el Marketplace (Cerrajería)...")
        res = requests.get(f"{URL_BASE}/subcategorias/1/colaboradores?latitud=19.4326&longitud=-99.1332")
        colaboradores = res.json()
        encontrado = any(c['colaborador_id'] == colaborador_id for c in colaboradores)
        
        if not encontrado:
            print("❌ El colaborador no aparece en el Marketplace.")
            sys.exit(1)
        print("✅ Colaborador encontrado en el Marketplace con exito.")

        # 5. CREACIÓN DE SOLICITUD DE SERVICIO
        print("5. Creando una solicitud de servicio...")
        datos_solicitud = {
            "usuario_id": usuario_id,
            "colaborador_id": colaborador_id,
            "subcategoria_id": 1,
            "urgencia": "alta",
            "descripcion_detallada": "Me quede fuera de casa sin llaves, es urgente.",
            "fotos_evidencia_inicial": "llave_rota.jpg",
            "latitud": 19.4326,
            "longitud": -99.1332
        }
        res = requests.post(f"{URL_BASE}/solicitudes", json=datos_solicitud)
        if res.status_code not in [200, 201]:
            print(f"❌ Error al crear solicitud: {res.text}")
            sys.exit(1)
        solicitud_id = res.json()['id']
        print(f"✅ Solicitud creada con ID: {solicitud_id}")

        # 6. PRUEBA DE CHAT
        print("6. Enviando mensaje de chat...")
        mensaje = {
            "emisor_id": usuario_id,
            "contenido": "¿En cuanto tiempo podria llegar?"
        }
        res = requests.post(f"{URL_BASE}/solicitudes/{solicitud_id}/mensajes", json=mensaje)
        if res.status_code not in [200, 201]:
            print(f"❌ Error al enviar mensaje: {res.text}")
            sys.exit(1)
        
        print("7. Verificando historial de mensajes...")
        res = requests.get(f"{URL_BASE}/solicitudes/{solicitud_id}/mensajes")
        mensajes = res.json()
        if len(mensajes) == 0 or mensajes[-1]['contenido'] != mensaje['contenido']:
            print("❌ El mensaje no se guardo correctamente.")
            sys.exit(1)
        print("✅ Mensaje recibido y verificado en el historial.")

        print("\n✨ TODAS LAS PRUEBAS PASARON CON EXITO ✨")
        print("El flujo completo de negocio esta estabilizado.")

    except Exception as e:
        print(f"\n☢️ ERROR CRITICO DURANTE LAS PRUEBAS: {e}")
        sys.exit(1)

if __name__ == "__main__":
    ejecutar_pruebas()
