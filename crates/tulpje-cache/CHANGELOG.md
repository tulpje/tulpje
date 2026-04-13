# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.5.2] - 2026-10-04

### Fixed

 - Clear old `guild_channels` when caching a guild

### Commit Details

<details><summary>view details</summary>

 * fix(tulpje-cache): clear old `guild_channels` when caching a guild ([`6aab3cb`](https://github.com/tulpje/tulpje/commit/6aab3cb02f0c4ee3cfaa5d991e633ed626a7f9ad))
</details>

## [cache-v0.5.1-rc.1] - 2026-10-04

### Changed

 - Use `wild` linker
 - Set `publish` in each crate's `Cargo.toml`

### Commit Details

<details><summary>view details</summary>

 * chore(build): use `wild` linker ([`be4dd98`](https://github.com/tulpje/tulpje/commit/be4dd98c6090e3ac7ae0b0e24dac850f193031d5))
 * chore: set `publish` in each crate's `Cargo.toml` ([`e2d986c`](https://github.com/tulpje/tulpje/commit/e2d986c8f9560959dec509f114687dd3023e5c5f))
</details>

## [cache-v0.5.0-rc.1] - 2026-10-04

### Changed

 - Front change notifications
 - Switch to using `mod.rs` files

### Commit Details

<details><summary>view details</summary>

 * feat(pluralkit): front change notifications ([`bd16e3d`](https://github.com/tulpje/tulpje/commit/bd16e3dc421311b85b68058fd9f7467c890ff284))
 * refactor: switch to using `mod.rs` files ([`e7614fa`](https://github.com/tulpje/tulpje/commit/e7614fa7ae6bed2ad1b5113d4d5efd3e351b9b15))
</details>

## [cache-v0.4.1] - 2026-10-04

### Changed

 - Update `twilight-cache-inmemory`, `twilight-gateway`, `twilight-http` and `twilight-model` from 0.17.0 to 0.17.1
 - Make `serde` and `serde_json` workspace dependencies

### Removed

 - Remove catch-all for events, add missing ignored events

### Commit Details

<details><summary>view details</summary>

 * chore(deps): update `twilight-cache-inmemory`, `twilight-gateway`, `twilight-http` and `twilight-model` from 0.17.0 to 0.17.1 ([`e3b1c33`](https://github.com/tulpje/tulpje/commit/e3b1c33fb5adb0e92cd1c161d0e317ab31dada4e))
 * fix(cache): remove catch-all for events, add missing ignored events ([`b5d4948`](https://github.com/tulpje/tulpje/commit/b5d4948ca697edc8ff8cefda570bb37c84a2320a))
 * chore(deps): make `serde` and `serde_json` workspace dependencies ([`52f8711`](https://github.com/tulpje/tulpje/commit/52f8711b9f31a4edf8fbe3c4b5de9f84f55d309f))
</details>

## [cache-v0.4.0-rc.1] - 2026-10-04

### Added

 - Add `avatar_decoration_data` and `banner` fields to user

### Changed

 - Bump `serde_json` from 1.0.140 to 1.0.145
 - Bump `serde` from 1.0.219 to 1.0.228
 - Move redis crate to workspace deps
 - Update to rust 1.90.0, fix lint warnings, `cargo fmt`
 - `clippy::collapsible_if`
 - `cargo fmt`
 - Specify edition on workspace level
 - Move twilight-* crates to workspace deps

### Fixed

 - Update User on MemberUpdate
 - Hanging connections

### Removed

 - Remove deref

### Commit Details

<details><summary>view details</summary>

 * fix(cache): add `avatar_decoration_data` and `banner` fields to user ([`7fee98c`](https://github.com/tulpje/tulpje/commit/7fee98c34df25d4c9b18cdc070351e4ab84b7744))
 * fix(cache): remove deref ([`35be22e`](https://github.com/tulpje/tulpje/commit/35be22e26db2a093752616b8881cf0445ddadb86))
 * fix(cache): update User on MemberUpdate ([`4eac92f`](https://github.com/tulpje/tulpje/commit/4eac92f2344f46b1db32d729f6ec942189c45635))
 * chore(deps): bump `serde_json` from 1.0.140 to 1.0.145 ([`7e2baa3`](https://github.com/tulpje/tulpje/commit/7e2baa398dae6719dd07d53875f6e1639875d1df))
 * chore(deps): bump `serde` from 1.0.219 to 1.0.228 ([`5511665`](https://github.com/tulpje/tulpje/commit/55116655b37504981043e098367cfd8df29e8691))
 * chore(deps): move redis crate to workspace deps ([`0299f80`](https://github.com/tulpje/tulpje/commit/0299f80d7e8a099677875d2cc6c9cf96b880508e))
 * chore(build): update to rust 1.90.0, fix lint warnings, `cargo fmt` ([`b850737`](https://github.com/tulpje/tulpje/commit/b8507378e832c0dee7bfedc74ebc72d3ad250235))
 * chore(lint): `clippy::collapsible_if` ([`3280fa2`](https://github.com/tulpje/tulpje/commit/3280fa2aff1e50057231438153466c6839807201))
 * chore: `cargo fmt` ([`a08aa81`](https://github.com/tulpje/tulpje/commit/a08aa8152bc0422ea2a1c3740cfd59a098e26e58))
 * refactor(build): specify edition on workspace level ([`751c335`](https://github.com/tulpje/tulpje/commit/751c335316d1e9b4440e98e7435fa38ed1ea8c09))
 * chore(deps): move twilight-* crates to workspace deps ([`d36e8d5`](https://github.com/tulpje/tulpje/commit/d36e8d565d66b066284387a721bc3cc013e5365d))
 * fix(gateway): hanging connections ([`5635aa6`](https://github.com/tulpje/tulpje/commit/5635aa6aa56de9cbecbafaefcaaeed0ef75fc0a2))
</details>

## [cache-v0.3.0] - 2026-10-04

### Changed

 - Bump serde_json from 1.0.138 to 1.0.140

### Fixed

 - Redis should have feature `tokio-comp` not `aio`

### Commit Details

<details><summary>view details</summary>

 * fix(handler): redis should have feature `tokio-comp` not `aio` ([`5a478cb`](https://github.com/tulpje/tulpje/commit/5a478cb694650f5d1b22d8265a40f2a409f3d60e))
 * build(deps): bump serde_json from 1.0.138 to 1.0.140 ([`b92e976`](https://github.com/tulpje/tulpje/commit/b92e9766acfafb5444bdbb591256170f36790723))
</details>

## [cache-v0.2.0] - 2026-10-04

### Breaking Changes

 - Use redis-rs directly instead of through bb8 pool

### Changed

 - Bump redis from 0.28.2 to 0.29.1
 - Bump serde from 1.0.216 to 1.0.219
 - Bump serde_json from 1.0.133 to 1.0.138

### Commit Details

<details><summary>view details</summary>

 * build(deps): bump redis from 0.28.2 to 0.29.1 ([`a65610b`](https://github.com/tulpje/tulpje/commit/a65610b39b50e851ef96207d4c4da63f544f94f2))
 * build(deps): bump serde from 1.0.216 to 1.0.219 ([`cc80138`](https://github.com/tulpje/tulpje/commit/cc8013804aac926c4104e61d4196c72c2ba3faa9))
 * build(deps): bump serde_json from 1.0.133 to 1.0.138 ([`78203bb`](https://github.com/tulpje/tulpje/commit/78203bb7396b63807cc1103dece994c06202ef03))
 * refactor!: use redis-rs directly instead of through bb8 pool ([`12add45`](https://github.com/tulpje/tulpje/commit/12add4574d435e2c8f86ba139502f042847e9111))
</details>

## [cache-v0.1.0] - 2026-10-04

### Changed

 - Implemented tulpje-cache, a redis based caching library

### Commit Details

<details><summary>view details</summary>

 * feat: implemented tulpje-cache, a redis based caching library ([`d11b12e`](https://github.com/tulpje/tulpje/commit/d11b12ed73a8fec4c80368e62e324a3b69536002))
</details>
<!-- generated by git-cliff -->
