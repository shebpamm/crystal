use diesel::Connection;
use diesel::PgConnection;
use fang::run_migrations_postgres;
use native_tls::TlsConnector;
use once_cell::sync::OnceCell;
use postgres_native_tls::MakeTlsConnector;
use snafu::{ResultExt, Snafu};
use tokio_postgres;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Row, ToStatement};

use bb8_postgres::bb8::{Pool, PooledConnection, RunError};
use bb8_postgres::PostgresConnectionManager;

pub async fn initialize_db_manager(database_url: String) {
    let options = DBOptions {
        pg_params: database_url.clone(),
        pool_max_size: 10,
    };

    let db_manager = DBManager::new(options).await.unwrap();
    set_db_manager(db_manager);
}

pub fn do_migrations(database_url: String) {
    let mut connection = PgConnection::establish(&database_url).unwrap();

    run_migrations_postgres(&mut connection).unwrap();
}

pub fn create_db_connector() -> MakeTlsConnector {
    let connector = TlsConnector::builder()
        .build()
        .expect("Failed to build TLS connector");

    MakeTlsConnector::new(connector)
}

static DB_MANAGER_INSTANCE: OnceCell<DBManager> = OnceCell::new();

pub fn get_db_manager() -> &'static DBManager {
    DB_MANAGER_INSTANCE.get().unwrap()
}

fn set_db_manager(db_manager: DBManager) {
    let _ = DB_MANAGER_INSTANCE.set(db_manager);
}

pub type DBConnection<'a> = PooledConnection<'a, PostgresConnectionManager<MakeTlsConnector>>;

pub type DBPool = Pool<PostgresConnectionManager<MakeTlsConnector>>;

pub type PostgresConnectionError = RunError<tokio_postgres::error::Error>;

// Provide a contexts for better error handling
#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("ConnectionError: {}", source))]
    ConnectionError { source: PostgresConnectionError },

    #[snafu(display("PostgresError: {}", source))]
    PostgresError { source: tokio_postgres::Error },
}

pub struct DBOptions {
    // see https://docs.rs/tokio-postgres/latest/tokio_postgres/config/struct.Config.html"
    pub pg_params: String,
    pub pool_max_size: u32,
}

pub struct DBManager {
    pool: DBPool,
}

impl DBManager {
    // Get an instance of DBManager
    pub async fn get() -> &'static DBManager {
        DB_MANAGER_INSTANCE.get().unwrap()
    }

    // Create the DBManager instance using DBOptions
    pub async fn new(config: DBOptions) -> Result<Self, Error> {
        let DBOptions {
            pg_params,
            pool_max_size,
        } = config;

        let manager =
            PostgresConnectionManager::new_from_stringlike(pg_params, create_db_connector())
                .expect("unable build PostgresConnectionManager");

        let pool = Pool::builder()
            .max_size(pool_max_size)
            .build(manager)
            .await
            .context(PostgresSnafu)?;

        Ok(Self { pool })
    }

    // Helper to get a connection from the bb8 pool
    pub async fn connection(&self) -> Result<DBConnection<'_>, Error> {
        let conn = self.pool.get().await.context(ConnectionSnafu)?;
        Ok(conn)
    }

    // Perform a query from a fetched bb8 connection
    pub async fn query<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, Error>
    where
        T: ?Sized + ToStatement,
    {
        let conn = self.connection().await?;
        let rows = conn.query(statement, params).await.context(PostgresSnafu)?;
        Ok(rows)
    }

    // Perform a query_one from a fetched bb8 connection
    pub async fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, Error>
    where
        T: ?Sized + ToStatement,
    {
        let conn = self.connection().await?;
        let row = conn
            .query_one(statement, params)
            .await
            .context(PostgresSnafu)?;
        Ok(row)
    }
}
