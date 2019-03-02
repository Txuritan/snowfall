# TODO

- [ ] PostgreSQL uses UUID v0.5 we need it to be v0.7. Until then collections, bookmarks, and tags will use a normal `INTEGER` id. This will be a breaking change when its fixed.
  - [ ] `INTEGER` to UUID database updater.
- [ ] SSL/TLS:
  - [ ] PostgreSQL uses OpenSSL v0.9 I'll either have to use a rev or master, wait until the crate updates, or never implement it for the backend.
  - [ ] MySQL doesn't support SSL/TLS on Windows, I'll either wait, or never implement it for the backend.
  - [ ] Actix Web and PostgresSQL both support NativeTLS, MySQL doesn't, use either OpenSSL or NativeTLS when MySQL implements it.
- [ ] Implement multi-database support:
  - [ ] MySQL
  - [ ] PostgreSQL
  - [X] SQLite
- [ ] Setup a system to make sure all dependencies are up to date.
- [ ] Release build system (GitLab CI or Drone CI).
- [ ] Tests...