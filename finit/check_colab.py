import mysql.connector
try:
    conn = mysql.connector.connect(host="localhost", user="ivan", password="password", database="finit")
    cursor = conn.cursor()
    cursor.execute("DESCRIBE colaborador")
    for row in cursor.fetchall():
        print(row)
    conn.close()
except Exception as e:
    print(e)
