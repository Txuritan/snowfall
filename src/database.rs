use juniper::Context;

use r2d2::Pool;

use crate::{
    backend::{
        mysql::MysqlConnectionManager, postgres::PostgresConnectionManager,
        sqlite::SqliteConnectionManager,
    },
    config::Database as CDB,
    CONFIG,
};

const SQLITE_SCHEMA: &str = include_str!("sql.sqlite");

#[derive(Clone)]
pub enum Database {
    MySQL {
        pool: Pool<MysqlConnectionManager>,
    },
    PostgreSQL {
        pool: Pool<PostgresConnectionManager>,
    },
    SQLite {
        pool: Pool<SqliteConnectionManager>,
    },
}

impl Database {
    pub fn new() -> Database {
        Default::default()
    }
}

impl Default for Database {
    fn default() -> Database {
        match &CONFIG.database {
            CDB::SQLite { file } => {
                let manager = SqliteConnectionManager::file(file);
                let pool = r2d2::Pool::new(manager).unwrap();

                {
                    let conn = pool.get().expect("unable to get connection");

                    conn.execute_batch(format!("BEGIN;\n{}\nCOMMIT;", SQLITE_SCHEMA).as_str())
                        .expect("unable to setup database");
                }

                Database::SQLite { pool }
            }
        }
    }
}

impl Context for Database {}
