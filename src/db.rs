use diesel::PgConnection;
use diesel::Connection;
use fang::run_migrations_postgres;
use postgres_native_tls::MakeTlsConnector;
use native_tls::TlsConnector;

pub fn do_migrations(database_url: String) {
    let mut connection = PgConnection::establish(&database_url).unwrap();

    run_migrations_postgres(&mut connection).unwrap();
}

pub fn create_db_connector(database_url: String) -> MakeTlsConnector {
    let mut connection = PgConnection::establish(&database_url).unwrap();

    log::info!("Running migrations...");

    run_migrations_postgres(&mut connection).unwrap();

    log::info!("Migrations done...");

    drop(connection);

    log::info!("Starting...");

    let connector = TlsConnector::builder()
        .build()
        .expect("Failed to build TLS connector");

    MakeTlsConnector::new(connector)
}
