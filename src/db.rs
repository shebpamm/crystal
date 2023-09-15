use diesel::pg::Pg;
use diesel::Connection;
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use tokio_postgres;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Row, ToStatement};

use bb8_postgres::bb8::{Pool, PooledConnection, RunError};
use bb8_postgres::PostgresConnectionManager;

use fang::run_migrations_postgres;
use fang::{FangError,ToFangError};

use native_tls::TlsConnector;
use once_cell::sync::OnceCell;
use postgres_native_tls::MakeTlsConnector;
use std::fmt::Debug;

static DB_MANAGER_INSTANCE: OnceCell<DBManager> = OnceCell::new();

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

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
    run_crystal_migrations(&mut connection);
}

fn run_crystal_migrations(connection: &mut impl MigrationHarness<Pg>) {
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}

pub fn create_db_connector() -> MakeTlsConnector {
    let connector = TlsConnector::builder()
        .build()
        .expect("Failed to build TLS connector");

    MakeTlsConnector::new(connector)
}

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
#[derive(thiserror::Error, Debug, ToFangError)]
pub enum DBError {
    #[error("Postgres connection error")]
    ConnectionError(#[from] PostgresConnectionError),

    #[error("Postgres error")]
    PostgresError(#[from] tokio_postgres::error::Error),
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
    pub async fn new(config: DBOptions) -> Result<Self, DBError> {
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
            .await?;

        Ok(Self { pool })
    }

    // Helper to get a connection from the bb8 pool
    pub async fn connection(&self) -> Result<DBConnection<'_>, DBError> {
        let conn = self.pool.get().await?;
        Ok(conn)
    }

    // Perform a query from a fetched bb8 connection
    pub async fn query<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, DBError>
    where
        T: ?Sized + ToStatement,
    {
        let conn = self.connection().await?;
        let rows = conn.query(statement, params).await?;
        Ok(rows)
    }

    // Perform a query_one from a fetched bb8 connection
    pub async fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, DBError>
    where
        T: ?Sized + ToStatement,
    {
        let conn = self.connection().await?;
        let row = conn
            .query_one(statement, params)
            .await?;
        Ok(row)
    }
}
