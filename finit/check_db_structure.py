import mysql.connector
import os

db_url = "mysql://ivan:password@localhost:3306/finit"

def check_db():
    try:
        conn = mysql.connector.connect(
            host="localhost",
            user="ivan",
            password="password",
            database="finit"
        )
        cursor = conn.cursor()
        
        tables = ['usuarios', 'colaborador', 'disponibilidad_colaborador', 'configuracion_precios_colaborador']
        
        for table in tables:
            print(f"\n--- Estructura de la tabla: {table} ---")
            try:
                cursor.execute(f"DESCRIBE {table}")
                for row in cursor.fetchall():
                    print(row)
            except Exception as e:
                print(f"❌ La tabla '{table}' no existe o tiene errores: {e}")
        
        cursor.close()
        conn.close()
    except Exception as e:
        print(f"🔥 Error conectando a la base de datos: {e}")

if __name__ == "__main__":
    check_db()
