use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

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
