# TODO

- [ ] PostgreSQL uses UUID v0.5 we need it to be v0.7. Until then collections, bookmarks, and tags will use a normal `INTEGER` id. This will be a breaking change when its fixed.
  - [ ] `INTEGER` to UUID database updater.
- [ ] Implement multi-database support:
  - [ ] MySQL
  - [ ] PostgreSQL
  - [X] SQLite
- [ ] Setup a system to make sure all dependencies are up to date.
- [ ] Release build system (GitLab CI or Drone CI).
- [ ] Tests...