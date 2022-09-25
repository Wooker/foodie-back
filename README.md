# foodie-back

## How to build
### Rust installation
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Database setup
Create the database on local postgres server and provide a link to it in `.env` file in the root directory of the project, like so:
```env
DATABASE_URL=postgres://{username}:{password}@localhost/{database_name}
```
where you replace `{username}`, `{password}`, and `{database_name}` with your own parameters.


### Diesel setup
Diesel is an ORM tool with its own CLI for creating schemas and migration scripts.
```sh
cargo install diesel_cli --no-default-features --features postgres
```

All of the migration scripts present in `migrations` directory.
To apply them run:
```sh
diesel migration run
```
Diesel will use the link you provided in `.env` file to connect to the database.
If you want to reset the database and apply the migrations again, use:
```sh
diesel database reset
```

### Build the project
```sh
cargo build
```
