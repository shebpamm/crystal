use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use juniper::GraphQLObject;
use crate::db::{get_db_manager, DBError};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, GraphQLObject)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A Kide account")]
pub struct KideAccount {
    pub id: i32,
    pub name: String,
    pub token: String,
}

impl <'a> TryFrom<&'a Row> for KideAccount {
    type Error = DBError;

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

pub async fn fetch_kide_accounts(account_ids: Vec<String>) -> Result<Vec<KideAccount>, DBError> {
    let mut accounts = Vec::new();
    let db_manager = get_db_manager();

    for account_id in account_ids {
        // TODO: Change to using UUIDs and not serials, so just cast to int for now...
        let account_id: i32 = account_id.parse().unwrap();

        log::trace!("Fetching account {}...", account_id);
        let row = db_manager
            .query_one("SELECT * FROM kideaccounts WHERE id = $1", &[&account_id])
            .await?;
        log::trace!("Fetched row {:#?}", row);
        let account = KideAccount::try_from(&row)?;
        accounts.push(account);
    }

    Ok(accounts)
}
