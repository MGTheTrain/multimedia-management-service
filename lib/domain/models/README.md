# Models

## How to create diesel migrations

0. Automatically create the `migrations folder` and `diesel.toml` trough `diesel setup` command in this folder
1. In the `migrations folder` manually create a new migration, e.g. `2023-10-08-174636_models` 
2. In the `2023-10-08-174636_models` folder add an `down.sql` and `up.sql`
3. The `up.sql` will be applied on the targeted Sql database server trough `diesel migration run` and will additionaly generate a `src/schema.rs` file which can be included in the defined database model structs, e.g. `src/container_meta.rs` or `src/file_meta.rs`
4. The `down.sql` will be applied on the targeted Sql database server trough `diesel migration redo`

**NOTE**: Ensure updated `up.sql` and `down.sql` files for each migration is synced with `src/container_meta.rs`, `src/file_meta.rs` and `src/schema.rs`. Always check results in the `src/schema.rs` after executing `diesel migration run` or `diesel migration redo`. Check if results in the `src/schema.rs` align with the domain model structs defined in `src/container_meta.rs` or `src/file_meta.rs` (e.g. `cargo test` or trough `pre-compiler` red underlined lines in the code). 