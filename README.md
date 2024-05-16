# multimedia-management-service

![Project Status: Aborted](https://img.shields.io/badge/Project_Status-Aborted-lightgrey.svg)

## Table of Contents

- [Summary](#summary)
- [Requirements](#requirements)
- [Rust essentials](#rust-essentials)

### Summary

The Mutli Media Management Service enables a shared multi-media platform and use cases like live streaming static (e.g. Youtube, Netflix) or dynamic time-series (e.g. video, audio, subtitles) content (e.g. Twitch, Kick).

## Requirements

The platform is exposing HTTP endpoints in which to

- [ ] Upload multimedia container file formats containing video, audio or subtitles (e.g. MP4, MOV, WEBM) to a Storage Account Container (consider tags, name of the video, description) and generate meta information for the uploaded multimedia container file in a Sql database table
- [ ] Download multimedia container file formats by id from a Storage Account Container
- [ ] Retrieve list of meta information with or without query (query options: by date time, tags, name of the video, free-text search of the description)
- [ ] Retrieve meta information by id from a Sql database table
- [ ] Update meta informations (tags, name of the video, description except unique ids)
- [ ] Delete multimedia container file formats containing video, audio or subtitles (e.g. MP4, MOV, WEBM) in a Storage Account Container and delete associated meta information by id

### Rust essentials

#### Testing crates

```rust
# Navigate to the internal crate to test, e.g. domain models
cd lib/domain/models/
cargo test
```

#### Debugging

Setup in VS Code a [launch.json](.vscode\launch.json). Set breakpoints in all the internal rust files of interest and start debugging.
