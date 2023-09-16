use crate::db::{get_db_manager, DBError};
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

pub type AccountIDList = Vec<Uuid>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, GraphQLObject)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A Kide account")]
pub struct KideAccount {
    pub uuid: Uuid,
    pub name: String,
    pub token: String,
}

impl<'a> TryFrom<&'a Row> for KideAccount {
    type Error = DBError;

    fn try_from(row: &'a Row) -> Result<Self, Self::Error> {
        Ok(Self {
            uuid: row.try_get("uuid")?,
            name: row.try_get("name")?,
            token: row.try_get("token")?,
        })
    }
}

impl KideAccount {
    pub fn new(uuid: Uuid, name: String, token: String) -> Self {
        Self { uuid, name, token }
    }

    pub async fn create(name: String, token: String) -> Result<Self, DBError> {
        let db_manager = get_db_manager();
        let conn = db_manager.connection().await?;

        let statement = conn
            .prepare("INSERT INTO kideaccounts (name, token) VALUES ($1, $2) RETURNING uuid")
            .await?;
        let row = conn.query_one(&statement, &[&name, &token]).await?;

        let uuid: Uuid = row.get(0);
        Ok(Self { uuid, name, token })
    }

    pub async fn delete(uuid: Uuid) -> Result<(), DBError> {
        let db_manager = get_db_manager();

        db_manager
            .execute("DELETE FROM kideaccounts WHERE uuid = $1", &[&uuid])
            .await?;

        Ok(())
    }

    pub async fn save(&self) -> Result<(), DBError> {
        let db_manager = get_db_manager();

        db_manager
            .execute(
                "UPDATE kideaccounts SET name = $1, token = $2 WHERE uuid = $3",
                &[&self.name, &self.token, &self.uuid],
            )
            .await?;

        Ok(())
    }

    pub async fn from_uuid(uuid: Uuid) -> Result<Option<KideAccount>, DBError> {
        let db_manager = get_db_manager();
        let row = db_manager
            .query_opt("SELECT * FROM kideaccounts WHERE uuid = $1", &[&uuid])
            .await?;

        match row {
            Some(row) => Ok(Some(Self::try_from(&row)?)),
            None => Ok(None),
        }
    }

    pub async fn from_uuids(account_uuids: AccountIDList) -> Result<Vec<KideAccount>, DBError> {
        let mut accounts = Vec::new();

        for account_uuid in account_uuids {
            log::trace!("Fetching account {}...", account_uuid);

            if let Some(account) = Self::from_uuid(account_uuid).await? {
                accounts.push(account);
            }
        }

        Ok(accounts)
    }

    pub async fn all() -> Result<Vec<KideAccount>, DBError> {
        let db_manager = get_db_manager();
        let rows = db_manager
            .query("SELECT uuid, name, token FROM kideaccounts", &[])
            .await?;

        let mut accounts = Vec::new();
        for row in rows {
            let account = KideAccount::try_from(&row)?;
            accounts.push(account);
        }

        Ok(accounts)
    }
}
