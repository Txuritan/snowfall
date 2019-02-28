//! # r2d2-mysql
//! MySQL support for the r2d2 connection pool (Rust) . see [`r2d2`](http://github.com/sfackler/r2d2.git)  .
//!
//! #### Install
//! Just include another `[dependencies.*]` section into your Cargo.toml:
//!
//! ```toml
//! [dependencies.r2d2_mysql]
//! git = "https://github.com/outersky/r2d2-mysql"
//! version="*"
//! ```
//! #### Sample
//!
//! ```
//! extern crate mysql;
//! extern crate r2d2_mysql;
//! extern crate r2d2;
//! use std::env;
//! use std::sync::Arc;
//! use std::thread;
//! use mysql::{Opts,OptsBuilder};
//! use r2d2_mysql::MysqlConnectionManager;
//!
//! fn main() {
//! 	let db_url =  env::var("DATABASE_URL").unwrap();
//!     let opts = Opts::from_url(&db_url).unwrap();
//!     let builder = OptsBuilder::from_opts(opts);
//!     let manager = MysqlConnectionManager::new(builder);
//!     let pool = Arc::new(r2d2::Pool::builder().max_size(4).build(manager).unwrap());
//!
//!     let mut tasks = vec![];
//!
//!     for i in 0..3 {
//!         let pool = pool.clone();
//!         let th = thread::spawn(move || {
//!             let mut conn = pool.get().unwrap();
//!             conn.query("select user()").unwrap();
//!             println!("thread {} end!" , i );
//!         });
//!         tasks.push(th);
//!     }
//!
//!     for th in tasks {
//!         let _ = th.join();
//!     }
//! }
//! ```
//!

use mysql::{error::Error, Conn, Opts, OptsBuilder};

#[derive(Clone, Debug)]
pub struct MysqlConnectionManager {
    params: Opts,
}

impl MysqlConnectionManager {
    pub fn new(params: OptsBuilder) -> MysqlConnectionManager {
        MysqlConnectionManager {
            params: Opts::from(params),
        }
    }
}

impl r2d2::ManageConnection for MysqlConnectionManager {
    type Connection = Conn;
    type Error = Error;

    fn connect(&self) -> Result<Conn, Error> {
        Conn::new(self.params.clone())
    }

    fn is_valid(&self, conn: &mut Conn) -> Result<(), Error> {
        conn.query("SELECT version()").map(|_| ())
    }

    fn has_broken(&self, conn: &mut Conn) -> bool {
        self.is_valid(conn).is_err()
    }
}
