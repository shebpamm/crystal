use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use crate::db::{get_db_manager, DBError};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KideAccount {
    pub id: i32,
    pub name: String,
    pub token: String,
}

impl <'a> TryFrom<&'a Row> for KideAccount {
    type Error = tokio_postgres::Error;

    fn try_from(row: &'a Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            token: row.try_get("token")?,
        })
    }
}

impl KideAccount {
    pub async fn create(name: String, token: String) -> Result<Self, DBError> {
        let db_manager = get_db_manager();
        let conn = db_manager.connection().await?;

        let statement = conn.prepare("INSERT INTO kideaccounts (name, token) VALUES ($1, $2) RETURNING id").await?;
        let row = conn.query_one(&statement, &[&name, &token]).await?;

        let id: i32 = row.get(0);
        Ok(Self {
            id,
            name,
            token,
        })
    }
}
