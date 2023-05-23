# Microservice Template using Axum
Microservice template using Axum, Sqlx, and utoipa that attempts to be flexible but not overly 
complicated. Developed using Sqlite, but should work for other database types.

Controller, model, and all code related to each entity type are contained in
the same module. It feels to me like that makes it easier to reason about all the
code related to one specific domain.

I was inspired by:
* https://carlosmv.hashnode.dev/creating-a-rest-api-with-axum-sqlx-rust
* https://github.com/launchbadge/sqlx/blob/v0.6.2/README.md
* https://tms-dev-blog.com/rust-sqlx-basics-with-sqlite/
* https://cargo-generate.github.io/cargo-generate/index.html
* https://github.com/juhaku/utoipa/blob/master/examples/todo-axum/src/main.rs