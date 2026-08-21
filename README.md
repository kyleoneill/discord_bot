# Setup
## Env
This project requires a `.env` file which has `DISCORD_TOKEN` and `DATABASE_URL` keys. The databse URL is a path to a db file or a database server,
an example value for a sqlite database looks like `DATABASE_URL="sqlite://bot_db.db"`.

## Database
This project uses SQLx and requires [SQLx CLI](https://crates.io/crates/sqlx-cli) to create the database file and run migrations. Installing it and
setting up the database looks like:

```sh
$ cargo install sqlx-cli
$ sqlx database create
$ sqlx migrate run
```

A new migration can be added with
```sh
$ sqlx migrate add <name>
```
which will create a new migration in `migrations/<timestamp>-<name>.sql` which will run when the migrate command is run.
