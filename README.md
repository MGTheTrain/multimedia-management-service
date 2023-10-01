# multimedia-management-service

## Table of Contents

- [Summary](#summary)
- [Rust essentials](#rust-essentials)

### Summary

The Mutli Media Management Service enables a shared video platform and use cases like live streaming static (e.g. Youtube, Netflix) or dynamic time-series (e.g. video, audio, subtitles) content (e.g. Twitch, Kick). The platform is exposing HTTP endpoints in which to

- upload multimedia container file formats containing video, audio or subtitles (e.g. MP4, MOV, WEBM) to a Storage Account Container (consider tags, name of the video, description) and generate meta information for the uploaded multimedia container file in a Sql database table
- download multimedia container file formats by id from a Storage Account Container
- retrieve list of meta information with or without query (query options: by date time, tags, name of the video, free-text search of the description)
- retrieve meta information by id from a Sql database table
- update meta informations (tags, name of the video, description)
- delete multimedia container file formats containing video, audio or subtitles (e.g. MP4, MOV, WEBM) in a Storage Account Container and delete associated meta information by id

### Rust essentials

0. Creating crates (Already considered here. So no actions required):

```rust
# libs
cargo new lib/domain/models --lib
cargo new lib/domain/interfaces --lib
cargo new lib/web/controllers --lib
cargo new lib/web/dtos --lib
cargo new lib/application/services --lib
cargo new lib/infrastructure/connectors --lib
cargo new lib/infrastructure/settings --lib
cargo new lib/persistence/data_access --lib
cargo new lib/persistence/migrations --lib

# For src/main.rs manually create the file and update the Cargo.toml to include internal and external crates
```

1. Testing crates

```rust
# Navigate to the internal crate to test, e.g. domain models
cd lib/domain/models/
cargo test
```

2. Debugging

```rust
# Set entrypoints in all the internal rust files of interest
cargo run
```
