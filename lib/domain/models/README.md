# Models

## How to create diesel migrations

0. Create the `migrations folder` and `diesel.toml` diesel setup
1. In the `migrations folder` manually create a new migration, e.g. `2023-10-08-174636_models` 
2. In the `2023-10-08-174636_models` folder add an `down.sql` and `up.sql`
3. The `up.sql` will be applied on the targeted Sql database server trough `diesel migration run` and will additionaly generate a `src/schema.rs` file which can be included in the defined database model structs, e.g. `src/container_meta.rs` or `src/file_meta.rs`
4. The `down.sql` will be applied on the targeted Sql database server trough `diesel migration redo`