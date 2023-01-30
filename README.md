Learning to build Todo list API with Rust + Postgresql + Actix.

Checklist:
- [x] start http server -> using [actix web](https://actix.rs/docs/application/)
- [x] create pool for db connection -> using [r2d2](https://github.com/sfackler/r2d2)
- [x] implement request handler for health module
- [ ] implement request handler for account module
- [ ] implement request handler for todo module
- [ ] create user sessions
- [ ] add test cases for each module
- and more

---

Pre-requisites:

- [rust](https://www.rust-lang.org/tools/install)
- [docker](https://docs.docker.com/get-docker/)
- [diesel_cli](https://diesel.rs/guides/getting-started)

Run locally:

```
// setup .env
DATABASE_URL=postgres://username:password@localhost/todo
```

```
docker-compose up
diesel migration run
cargo run
```