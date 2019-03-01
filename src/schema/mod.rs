pub mod types;

use juniper::{graphql_object, FieldResult, RootNode};
use url::Url;

use chrono::{DateTime, Utc};

use rusqlite::{types::ToSql, OptionalExtension};

use self::types::*;
use crate::database::Database;

pub struct QueryRoot;
pub struct MutationRoot;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}

graphql_object!(QueryRoot: Database |&self| {
    field bookmark(&executor, id: i32) -> FieldResult<Bookmark> {
        executor.context().get_bookmark_by_id(id)
    }

    field collection(&executor, id: i32) -> FieldResult<Collection> {
        match executor.context() {
            //#region[rgba(241,153,31,0.1)] MySQL
            Database::MySQL { pool } => {
                Ok(Collection {
                    id,
                    title: "4".to_string(),
                })
            },
            //#endregion

            //#region[rgba(51,103,145,0.1)] PostgreSQL
            Database::PostgreSQL { pool } => {
                Ok(Collection {
                    id,
                    title: "4".to_string(),
                })
            },
            //#endregion

            //#region[rgba(1,52,76,0.3)] SQLite
            Database::SQLite { pool } => {
                Ok(Collection {
                    id,
                    title: "4".to_string(),
                })
            },
            //#endregion
        }
    }

    field tag(&executor, id: i32) -> FieldResult<Tag> {
        match executor.context() {
            //#region[rgba(241,153,31,0.1)] MySQL
            Database::MySQL { pool } => {
                Ok(Tag {
                    id,
                    name: "4".to_string(),
                })
            },
            //#endregion

            //#region[rgba(51,103,145,0.1)] PostgreSQL
            Database::PostgreSQL { pool } => {
                Ok(Tag {
                    id,
                    name: "4".to_string(),
                })
            },
            //#endregion

            //#region[rgba(1,52,76,0.3)] SQLite
            Database::SQLite { pool } => {
                Ok(Tag {
                    id,
                    name: "4".to_string(),
                })
            },
            //#endregion
        }
    }
});

graphql_object!(MutationRoot: Database |&self| {
    field createBookmark(&executor, title: String, url: String, tags: Option<Vec<String>>) -> FieldResult<Bookmark> {
        executor.context().create_bookmark(title, url, tags)
    }
});

impl Database {
    fn create_bookmark(&self, title: String, url: String, tags: Option<Vec<String>>) -> FieldResult<Bookmark> {
        match self {
            //#region[rgba(241,153,31,0.1)] MySQL
            Database::MySQL { .. } => {
                Ok(Bookmark {
                    id: 0,
                    title: "".to_string(),
                    domain: "".to_string(),
                    url: "".to_string(),
                    tags: Vec::new(),
                })
            },
            //#endregion

            //#region[rgba(51,103,145,0.1)] PostgreSQL
            Database::PostgreSQL { .. } => {
                Ok(Bookmark {
                    id: 0,
                    title: "".to_string(),
                    domain: "".to_string(),
                    url: "".to_string(),
                    tags: Vec::new(),
                })
            },
            //#endregion

            //#region[rgba(1,52,76,0.3)] SQLite
            Database::SQLite { pool } => {
                let mut conn = pool.get()?;

                let tx = conn.transaction()?;

                let parsed = Url::parse(&url)?;
                let host = parsed.host();

                let date_time: DateTime<Utc> = Utc::now();
                let naive = date_time.naive_utc();

                let mut bookmark_id = 0i32;

                {
                    tx.execute(
                        "INSERT INTO bookmarks(title, domain, url, created, last_updated) VALUES (?1, ?2, ?3, ?4, ?5);",
                        &[
                            &title as &ToSql,
                            &if let Some(host) = host {
                                format!("{}", host) // TODO: another way of doing this?
                            } else {
                                "UNKNOWN".to_string()
                            } as &ToSql,
                            &url as &ToSql,
                            &naive as &ToSql,
                            &naive as &ToSql,
                        ],
                    )?;

                    bookmark_id = tx.last_insert_rowid() as i32;

                    if let Some(tags) = tags {
                        let mut check_stmt = tx.prepare(
                            "SELECT id FROM tags WHERE name = ?1;"
                        )?;

                        let mut inst_tag_stmt = tx.prepare(
                            "INSERT INTO tags (name, created, last_updated) VALUES (?1, ?2, ?3);"
                        )?;

                        let mut inst_bk_tag_stmt = tx.prepare(
                            "INSERT OR IGNORE INTO bookmark_tags (bookmark_id, tag_id, created, last_updated) VALUES (?1, ?2, ?3, ?4);"
                        )?;

                        for tag in tags {
                            let tag = tag.to_lowercase();
                            let tag = tag.trim();

                            let id = if let Some(id) = check_stmt.query_row(&[&tag as &ToSql], |row| -> i32 { row.get("id") }).optional()? {
                                id
                            } else {
                                inst_tag_stmt.execute(&[
                                    &tag as &ToSql,
                                    &naive as &ToSql,
                                    &naive as &ToSql,
                                ])?;
                                tx.last_insert_rowid() as i32
                            };

                            inst_bk_tag_stmt.execute(&[
                                &bookmark_id as &ToSql,
                                &id as &ToSql,
                                &naive as &ToSql,
                                &naive as &ToSql,
                            ])?;
                        }
                    }
                }

                tx.commit()?;

                self.get_bookmark_by_id(bookmark_id)
            }
            //#endregion
        }
    }

    fn get_bookmark_by_id(&self, id: i32) -> FieldResult<Bookmark> {
        match self {
            //#region[rgba(241,153,31,0.1)] MySQL
            Database::MySQL { .. } => Ok(Bookmark {
                id: id,
                title: "".to_string(),
                domain: "".to_string(),
                url: "".to_string(),
                tags: Vec::new(),
            }),
            //#endregion

            //#region[rgba(51,103,145,0.1)] PostgreSQL
            Database::PostgreSQL { .. } => Ok(Bookmark {
                id: id,
                title: "".to_string(),
                domain: "".to_string(),
                url: "".to_string(),
                tags: Vec::new(),
            }),
            //#endregion

            //#region[rgba(1,52,76,0.3)] SQLite
            Database::SQLite { pool } => {
                let conn = pool.get()?;

                let mut bookmark = conn.query_row(
                    "SELECT title, domain, url FROM bookmarks WHERE id = ?;",
                    &[id],
                    |row| Bookmark {
                        id: id,
                        title: row.get("title"),
                        domain: row.get("domain"),
                        url: row.get("url"),
                        tags: Vec::new(),
                    },
                )?;

                let mut stmt = conn.prepare(
                    "SELECT tag.id, tag.name FROM bookmark_tags bookmark_tag LEFT JOIN tags tag ON bookmark_tag.tag_id = tag.id WHERE bookmark_tag.bookmark_id = ?1 ORDER BY tag.name;"
                )?;

                for tag in stmt.query_map(&[&bookmark.id as &ToSql], |row| -> Tag {
                    Tag {
                        id: row.get(0),
                        name: row.get(1),
                    }
                })? {
                    bookmark.tags.push(tag?);
                }

                Ok(bookmark)
            }
            //#endregion
        }
    }
}
