use std::{collections::HashMap, path::Path, str::FromStr};

use sqlx::{ConnectOptions, Error, Result, SqliteConnection, sqlite::SqliteConnectOptions};
use tokio::fs;

pub async fn seed_db() {
    let mut conn = connection().await.unwrap();

    let table_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM sqlite_master WHERE type='table' AND name=?)",
    )
    .bind("freshservice")
    .fetch_one(&mut conn)
    .await
    .unwrap();

    if table_exists {
        return println!("Database setup.");
    }

    sqlx::raw_sql(
        "CREATE TABLE freshservice (
            file_name VARCHAR,
            file_ext VARCHAR,
            file_length INT,
            is_default INT,
            is_selected INT
        )",
    )
    .execute(&mut conn)
    .await
    .expect("failed to create the table.");

    let default_files_directory = Path::new("assets/media/fs/default");
    let mut entries = fs::read_dir(default_files_directory).await.unwrap();

    let mut v: Vec<HashMap<String, String>> = Vec::new();
    while let Some(entry) = entries.next_entry().await.unwrap() {
        let path = entry.path();

        let file_name = path.file_stem().unwrap();
        let file_ext = path.extension().unwrap();

        let mut map: HashMap<String, String> = HashMap::new();

        map.insert(
            "filename".to_string(),
            file_name.to_string_lossy().into_owned(),
        );
        map.insert(
            "fileext".to_string(),
            file_ext.to_string_lossy().into_owned(),
        );

        v.push(map);
    }

    for i in v {
        if i.get("filename").unwrap() == "alert" {
            sqlx::query(
                "INSERT INTO freshservice (file_name, file_ext, file_length, is_default, is_selected) VALUES (?, ?, ?, ?, ?)"
            )
            .bind(i.get("filename"))
            .bind(i.get("fileext"))
            .bind(2)
            .bind(1)
            .bind(1)
            .execute(&mut conn)
            .await
            .expect("failed to seed the table.");

            continue;
        }

        sqlx::query(
            "INSERT INTO freshservice (file_name, file_ext, file_length, is_default, is_selected) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(i.get("filename"))
        .bind(i.get("fileext"))
        .bind(2)
        .bind(1)
        .bind(0)
        .execute(&mut conn)
        .await
        .expect("failed to seed the table.");
    }

    println!("Seeded the database.");
    println!("Database setup.");
}

pub async fn connection() -> Result<SqliteConnection, Error> {
    let init = SqliteConnectOptions::from_str("sqlite://database.db")?
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .create_if_missing(true);

    Ok(init.connect().await?)
}
