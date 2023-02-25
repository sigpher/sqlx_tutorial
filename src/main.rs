use std::error::Error;

use sqlx::{Row, SqlitePool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = SqlitePool::connect("test.db").await?;
    // add_todo(&pool, String::from("dominate the world")).await?;
    list_todos(&pool).await?;
    Ok(())
}

async fn add_todo(pool: &SqlitePool, description: String) -> Result<i64, Box<dyn Error>> {
    let mut conn = pool.acquire().await?;

    let sql = r#"
   INSERT INTO todos ( description )
   VALUES ( ?1 )
           "#;
    let id = sqlx::query(sql)
        .bind(description)
        .execute(&mut conn)
        .await?
        .last_insert_rowid();
    Ok(id)
}

async fn list_todos(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
    let sql = "SELECT * FROM todos;";
    let ret = sqlx::query(sql).fetch_all(pool).await?;

    for (idx, row) in ret.iter().enumerate() {
        println!(
            "[{}]: {} , {}, {}",
            idx,
            row.get::<u32, &str>("id"),
            row.get::<String, &str>("description"),
            row.get::<bool, &str>("done"),
        );
    }

    Ok(())
}

#[derive(Debug)]
pub struct Todos {
    pub id: u32,
    pub description: String,
    pub done: bool,
}

impl Todos {
    pub fn new(id: u32, description: String, done: bool) -> Self {
        Self {
            id,
            description,
            done,
        }
    }
}
