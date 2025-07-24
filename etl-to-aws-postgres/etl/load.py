import os
import psycopg2

def load(df, table_name):
    conn = psycopg2.connect(
        host=os.getenv("DB_HOST"),
        port=os.getenv("DB_PORT"),
        user=os.getenv("DB_USER"),
        password=os.getenv("DB_PASS"),
        dbname=os.getenv("DB_NAME")
    )
    cur = conn.cursor()
    for _, row in df.iterrows():
        cur.execute(f"INSERT INTO {table_name} (name, age) VALUES (%s, %s)", tuple(row))
    conn.commit()
    cur.close()
    conn.close()
