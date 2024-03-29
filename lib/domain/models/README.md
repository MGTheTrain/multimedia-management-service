# Models

## How to create manual diesel migrations

**Precondition**: `export DATABASE_URL=postgres://user:password@localhost:5432/diesel-demo` in terminal process in which to also utilize `diesel` cli tool

0. Automatically create the `migrations folder` and `diesel.toml` trough `diesel setup` command in this folder
1. In the `migrations folder` manually create a new migration, e.g. `2023-10-08-174636_models` folder 
2. In the `2023-10-08-174636_models` folder add an `down.sql` and `up.sql`
3. The `up.sql` will be applied on the targeted Sql database server trough `diesel migration run` and will additionaly generate a `src/schema.rs` file which need to be included in the defined database model structs, e.g. `src/container_meta.rs` or `src/track.rs`
4. The `down.sql` will be applied on the targeted Sql database server trough `diesel migration redo`

**NOTE**: Ensure updated `up.sql` and `down.sql` files for each migration are synced with `src/container_meta.rs`, `src/track.rs` and `src/schema.rs`. Always check results in the `src/schema.rs` after executing `diesel migration run` or `diesel migration redo`. Check if results in the `src/schema.rs` align with the domain model structs defined in `src/container_meta.rs` or `src/track.rs` (e.g. `cargo test` or trough `pre-compiler` red underlined lines in the code). 

## Automatic diesel migrations in code at compile time

The [psql_data_access_async.rs](../../persistence/data_access/src/psql_data_access_async.rs) and [psql_data_access.rs](../../persistence/data_access/src/psql_data_access.rs) shows in the tests how to migrate with the `run_pending_migrations(...)` method during compile time (e.g. `cargo build`). The steps in the previous section **How to create manual diesel migrations** are not required.