import mysql.connector

def check_colaborador():
    try:
        conn = mysql.connector.connect(
            host="localhost",
            user="ivan",
            password="password",
            database="finit"
        )
        cursor = conn.cursor(dictionary=True)
        
        cursor.execute("SELECT * FROM colaborador")
        cols = cursor.fetchall()
        
        for col in cols:
            print(f"--- Colaborador ID: {col['id']} ---")
            print(f"   INE Frontal: {col['ine_frontal'][:50] if col['ine_frontal'] else 'null'}")
            print(f"   INE Trasera: {col['ine_trasera'][:50] if col['ine_trasera'] else 'null'}")
            print(f"   Comprobante: {col['comprobante_domicilio'][:50] if col['comprobante_domicilio'] else 'null'}")
            print(f"   Selfie:      {col['foto_selfie_ine'][:50] if col['foto_selfie_ine'] else 'null'}")
            
        cursor.close()
        conn.close()
    except Exception as e:
        print(f"🔥 Error: {e}")

if __name__ == "__main__":
    check_colaborador()
