use serde::{Deserialize, Serialize};
//use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::types::Json;
use std::collections::HashMap;
//use std::str::FromStr;

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct Image {
    id: i64,
    file_path: String,
    file_name: String,
    digitized_at: chrono::NaiveDateTime,
    #[serde(flatten)]
    props: Json<HashMap<String, String>>,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

#[derive(Debug)]
pub struct RegistImage {
    pub file_path: String,
    pub file_name: String,
    pub digitized_at: i64,
    pub props: HashMap<String, String>,
}

impl RegistImage {
    pub async fn db_regist_image(&self) -> anyhow::Result<Image> {
        let pool = crate::db::get_pool();

        if let Ok(select_image) = sqlx::query_as::<_, Image>(
            r#"
                SELECT * FROM images WHERE file_path = $1 AND file_name = $2
            "#,
        )
        .bind(&self.file_path)
        .bind(&self.file_name)
        .fetch_one(pool)
        .await
        {
            return Ok(select_image);
        }

        let image = sqlx::query_as::<_, Image>(
            r#"
        INSERT INTO images (
            file_path, file_name, digitized_at, props
        ) VALUES (
            $1,$2,$3,$4
        ) returning *, props as "props: Json<Props>"
        "#,
        )
        .bind(&self.file_path)
        .bind(&self.file_name)
        .bind(&self.digitized_at)
        .bind(Json(&self.props))
        .fetch_one(pool)
        .await?;

        Ok(image)
    }
}
