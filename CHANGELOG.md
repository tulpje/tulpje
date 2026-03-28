# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.22.0] - 2026-10-04

### Fixed

 - Fix tag naming

### Commit Details

<details><summary>view details</summary>

 * fix(tools/release-tulpje): fix tag naming ([`d01e40e`](https://github.com/tulpje/tulpje/commit/d01e40e5932cc85d3a0fd39a1f833fca7fb20655))
</details>

## [0.22.0-rc.1] - 2026-10-04

### Added

 - Add workspace hack with `cargo-hakari`
 - Add a description to tulpje-shared
 - Add `hyperfine` package
 - Add more colored output
 - Add glob support to workspace member resolution in `release.py`
 - Add `--skip-slow` to `release.py` to speed up dry runs

### Changed

 - Rename `db` module to `db_id`
 - Convert `pk_guilds` to use `system_uuid` instead of `system_id` and add foreign key
 - Move migrations to repository root
 - Move Core module into `tulpje-mod-core` crate
 - Move Stats module into `tulpje-mod-stats` crate
 - Move `format_significant_duration` to `tulpje-lib`
 - Move `is_pk_proxy` to `tulpje-lib`
 - Move Emoji module into `tulpje-mod-emoji` crate
 - Split out command calls into separate function in `build.rs`
 - Move PluralKit module into `tulpje-mod-pluralkit` crate
 - Move code shared between modules into `tulpje-lib` crate
 - Rename `tulpje-shared` to `tulpje-common`
 - Use `wild` linker
 - Use `debug = "line-tables-only" to improve debug build times and size
 - Split log formatter into separate module
 - Turn `release.py` into a proper python package
 - Disable dependabot version updates, only use it for security
 - Set `publish` in each crate's `Cargo.toml`
 - Use the Color deref impl
 - Move `Color` struct and role colors into `tulpje-framework`
 - Point to new user guide

### Fixed

 - Properly add foreign keys to all databaes tables
 - Pass shard id to gateway queue
 - Rebuild `tulpje-handler` if migrations change
 - Fix rerun-if-changed paths in build.rs
 - Actually re-use the `cargoArtifacts` derivation
 - Don't use references to Color, it's `Copy` now

### Removed

 - Remove unused dependency `serde-json` from `tulpje-common`
 - Remove unused `tulpje-manager` crate

### Commit Details

<details><summary>view details</summary>

 * refactor(tulpje-lib): rename `db` module to `db_id` ([`d9531bf`](https://github.com/tulpje/tulpje/commit/d9531bf8efe081edf3c8867f10df8505f8b5e357))
 * fix(handler): properly add foreign keys to all databaes tables ([`1921ee9`](https://github.com/tulpje/tulpje/commit/1921ee9ae7aa8be4d64bc47da443cb57c49c6fd5))
 * refactor(handler): convert `pk_guilds` to use `system_uuid` instead of `system_id` and add foreign key ([`53117c1`](https://github.com/tulpje/tulpje/commit/53117c13f0449237dec9b6424fe93a08fdb02402))
 * chore: move migrations to repository root ([`2ac9c1d`](https://github.com/tulpje/tulpje/commit/2ac9c1de832231402b330b6e76a4a28a0738e022))
 * fix(gateway): pass shard id to gateway queue ([`e181030`](https://github.com/tulpje/tulpje/commit/e18103048b2789cd59d80e07e03af590ca4c9427))
 * refactor: move Core module into `tulpje-mod-core` crate ([`4173e28`](https://github.com/tulpje/tulpje/commit/4173e28c136f830c6ac04c9460646410f2a69041))
 * refactor: move Stats module into `tulpje-mod-stats` crate ([`834d8b1`](https://github.com/tulpje/tulpje/commit/834d8b1db38d16337914bff0d95dac5f4d75b4c9))
 * chore(deps): remove unused dependency `serde-json` from `tulpje-common` ([`9b5d172`](https://github.com/tulpje/tulpje/commit/9b5d17226ea3198afa66717e3d5b24aa616eca66))
 * refactor: move `format_significant_duration` to `tulpje-lib` ([`afebce8`](https://github.com/tulpje/tulpje/commit/afebce8c42a5243aba0dda018583bcd8bb0627db))
 * refactor: move `is_pk_proxy` to `tulpje-lib` ([`48934fc`](https://github.com/tulpje/tulpje/commit/48934fc4c19d75c98b821eec8b6f11fa7d928979))
 * refactor: move Emoji module into `tulpje-mod-emoji` crate ([`ba5ecd7`](https://github.com/tulpje/tulpje/commit/ba5ecd7ec4ffd6d27277c3b9d759fbb797fd9ce4))
 * chore(build): add workspace hack with `cargo-hakari` ([`efd258f`](https://github.com/tulpje/tulpje/commit/efd258fca7b67e022a1033f757fe2693cc724e3e))
 * chore: remove unused `tulpje-manager` crate ([`4c3ad99`](https://github.com/tulpje/tulpje/commit/4c3ad9905cd2d908b1a0ca6833d9857993078a7f))
 * fix(build): rebuild `tulpje-handler` if migrations change ([`d92aef2`](https://github.com/tulpje/tulpje/commit/d92aef22b0f34edf793a753275734cdff609dffd))
 * refactor(build): split out command calls into separate function in `build.rs` ([`a274be6`](https://github.com/tulpje/tulpje/commit/a274be648f6f7a9fc898f61b4a95fd9c559cc0dc))
 * fix(build): fix rerun-if-changed paths in build.rs ([`ff69e98`](https://github.com/tulpje/tulpje/commit/ff69e9852446d2d5fe3ba3b6e58db1a3460305b2))
 * refactor: move PluralKit module into `tulpje-mod-pluralkit` crate ([`25f933a`](https://github.com/tulpje/tulpje/commit/25f933a712a9703b7bf7bbc7083f94ae80df3844))
 * refactor: move code shared between modules into `tulpje-lib` crate ([`90dc817`](https://github.com/tulpje/tulpje/commit/90dc8173aef357cadc8394b4ace79108296ca2e8))
 * refactor: rename `tulpje-shared` to `tulpje-common` ([`8503abc`](https://github.com/tulpje/tulpje/commit/8503abc37aef038ff01e5e9e2b68c10f64b54632))
 * chore: add a description to tulpje-shared ([`1d69670`](https://github.com/tulpje/tulpje/commit/1d696706c33e2319e532e0b218559d25f476b4c9))
 * chore(build): use `wild` linker ([`be4dd98`](https://github.com/tulpje/tulpje/commit/be4dd98c6090e3ac7ae0b0e24dac850f193031d5))
 * chore(build): use `debug = "line-tables-only" to improve debug build times and size ([`74233e6`](https://github.com/tulpje/tulpje/commit/74233e6d88b6b428ccb88603a0043b4c97c1e19d))
 * fix(nix): actually re-use the `cargoArtifacts` derivation ([`4f8196e`](https://github.com/tulpje/tulpje/commit/4f8196e1812071a2a54d5b062e880a361046cd4e))
 * chore(nix): add `hyperfine` package ([`3e87ae8`](https://github.com/tulpje/tulpje/commit/3e87ae8639b4caa3591a28e04602c566a67113af))
 * feat(tools/release-tulpje): add more colored output ([`6286254`](https://github.com/tulpje/tulpje/commit/62862549891dda98bfae439e172b4e2a8d3d38c9))
 * refactor(tools/release-tulpje): split log formatter into separate module ([`38ca4d7`](https://github.com/tulpje/tulpje/commit/38ca4d7add0eb7271b752d6f11a748d4342fe01f))
 * refactor(tools/release-tulpje): turn `release.py` into a proper python package ([`d236f5e`](https://github.com/tulpje/tulpje/commit/d236f5e27e1186b9523b1024596551812244053b))
 * fix(build): add glob support to workspace member resolution in `release.py` ([`e773857`](https://github.com/tulpje/tulpje/commit/e7738578fee817cc0139122dcb17abc3773505d8))
 * feat(build): add `--skip-slow` to `release.py` to speed up dry runs ([`1effc2a`](https://github.com/tulpje/tulpje/commit/1effc2af683a7c2512b1fb7bee8048b8325d9ac0))
 * chore(ci): disable dependabot version updates, only use it for security ([`9fbb076`](https://github.com/tulpje/tulpje/commit/9fbb076f2fe18b0a5b8495e629ef1322fa60f4f1))
 * chore: set `publish` in each crate's `Cargo.toml` ([`e2d986c`](https://github.com/tulpje/tulpje/commit/e2d986c8f9560959dec509f114687dd3023e5c5f))
 * chore: use the Color deref impl ([`a8d76cb`](https://github.com/tulpje/tulpje/commit/a8d76cbe9013a8355a3baacf615a7a446d9c97ca))
 * fix(handler): don't use references to Color, it's `Copy` now ([`5eb0851`](https://github.com/tulpje/tulpje/commit/5eb0851b112a8066d5dbcff4c68f429c4bfd55bc))
 * refactor: move `Color` struct and role colors into `tulpje-framework` ([`2c8cf4a`](https://github.com/tulpje/tulpje/commit/2c8cf4adf75ec1e49919be409115fbabecaeb412))
 * chore(docs): point to new user guide ([`fa7730d`](https://github.com/tulpje/tulpje/commit/fa7730d4bf7a3260f849187097da7b96673cc117))
</details>

## [0.21.5-rc.1] - 2026-10-04

### Added

 - Add total system count

### Changed

 - Reduce sensitive information in log messages
 - Improve log message in `update_fronter_category

### Fixed

 - Only try to update fronters for systems that have notifications or a front category

### Commit Details

<details><summary>view details</summary>

 * feat(handler): add total system count ([`c27600e`](https://github.com/tulpje/tulpje/commit/c27600e95a5d55b6bac7bd4cb69cbe320cfef898))
 * fix(handler): only try to update fronters for systems that have notifications or a front category ([`634fc21`](https://github.com/tulpje/tulpje/commit/634fc21c41c6f04b2efea0a4b6ab58bc2a5cc29f))
 * chore(handler): reduce sensitive information in log messages ([`d4223ba`](https://github.com/tulpje/tulpje/commit/d4223ba746482819c8a70f8b0b2c23c73e38721e))
 * chore(handler): improve log message in `update_fronter_category ([`d9a277c`](https://github.com/tulpje/tulpje/commit/d9a277c0d77472e1659bac0bc4076c836f52fddb))
</details>

## [0.21.4] - 2026-10-04

### Fixed

 - Make arguments to `just release` optional

### Commit Details

<details><summary>view details</summary>

 * fix(build): make arguments to `just release` optional ([`6d7ace9`](https://github.com/tulpje/tulpje/commit/6d7ace951b38a9b995b9494683ff20031f1dbb77))
</details>

## [0.21.4-rc.2] - 2026-10-04

### Added

 - Add `just release` shortcut for release script

### Fixed

 - Also store system info in `/pk setup`

### Commit Details

<details><summary>view details</summary>

 * feat(build): add `just release` shortcut for release script ([`106439a`](https://github.com/tulpje/tulpje/commit/106439a0966c0ec9fa8148830b4bac84ee692023))
 * fix(handler/pk): also store system info in `/pk setup` ([`0666d4c`](https://github.com/tulpje/tulpje/commit/0666d4c6b41ac01c21d4ebcf45a2d726e0e60096))
</details>

## [0.21.4-rc.1] - 2026-10-04

### Fixed

 - Don't clean up systems that are still configured in guilds

### Commit Details

<details><summary>view details</summary>

 * fix(handler): don't clean up systems that are still configured in guilds ([`50c5ab2`](https://github.com/tulpje/tulpje/commit/50c5ab268c8d504c932a8de46d97c1a827bf8ad1))
</details>

## [0.21.3] - 2026-10-04

### Fixed

 - Run `guilds:cleanup` daily

### Commit Details

<details><summary>view details</summary>

 * fix(handler): run `guilds:cleanup` daily ([`4006a27`](https://github.com/tulpje/tulpje/commit/4006a27615adbb5411254147404ee8b7d9c349be))
</details>

## [0.21.3-rc.1] - 2026-10-04

### Changed

 - Guild cleanup
 - Register guild commands on `GuildCreate` instead of at start
 - Toggle for MESSAGE_CONTENT intent

### Fixed

 - Only start the database for sqlx-migrate/sqlx-prepare tasks
 - Track more resources we actually want from the cache

### Commit Details

<details><summary>view details</summary>

 * fix(build): only start the database for sqlx-migrate/sqlx-prepare tasks ([`5c9fe2a`](https://github.com/tulpje/tulpje/commit/5c9fe2ad2649a2ca92bf98a047f050ccb3448bf5))
 * feat(handler): guild cleanup ([`48caee4`](https://github.com/tulpje/tulpje/commit/48caee4c07eaca3bcfb62f414922453b64f4c53b))
 * refactor(handler): register guild commands on `GuildCreate` instead of at start ([`9e398cd`](https://github.com/tulpje/tulpje/commit/9e398cde31321cfa997ece2821ce9c51c3d3b8fc))
 * feat: toggle for MESSAGE_CONTENT intent ([`2046cfe`](https://github.com/tulpje/tulpje/commit/2046cfeae31c8958cfd0e0c341c90c3dcacb961f))
 * fix(handler): track more resources we actually want from the cache ([`d3d1501`](https://github.com/tulpje/tulpje/commit/d3d15018a4e78a5b368f665f9153943f783b879f))
</details>

## [0.21.2-rc.1] - 2026-10-04

### Changed

 - Fancier feedback message on `/pk role update`
 - Better `/pk role update` error messages
 - Better `/pk role update` success message

### Fixed

 - Also assign member roles to user
 - Fix `/pk roles update` not notifying user if system is private/not found, also fix using token

### Removed

 - Remove unnecessary GUILD_MEMBERS intent

### Commit Details

<details><summary>view details</summary>

 * chore(gateway): remove unnecessary GUILD_MEMBERS intent ([`bc814cf`](https://github.com/tulpje/tulpje/commit/bc814cf4bd596eab0baa1c0e9d127bf8cc3d48a8))
 * feat(handler/pk): fancier feedback message on `/pk role update` ([`fa11138`](https://github.com/tulpje/tulpje/commit/fa111386dacc320c5d29e662ea31ecff349e0259))
 * fix(handler/pk): also assign member roles to user ([`16d29ad`](https://github.com/tulpje/tulpje/commit/16d29ad31921c7ea7cf281f8c437bce84d99f402))
 * fix(handler/pk): fix `/pk roles update` not notifying user if system is private/not found, also fix using token ([`9ca2342`](https://github.com/tulpje/tulpje/commit/9ca234255baec43a062c6e51a814c310415c171d))
 * feat(handler/pk): better `/pk role update` error messages ([`8a81866`](https://github.com/tulpje/tulpje/commit/8a818660e8333b834d6b7a346ccbcb606c340d2b))
 * feat(handler/pk): better `/pk role update` success message ([`fb1c3e6`](https://github.com/tulpje/tulpje/commit/fb1c3e6858bbadc192c70e8817e9fa6e75e3b63a))
</details>

## [0.21.1-rc.1] - 2026-10-04

### Added

 - Add missing `EMBED_LINKS` permission for to the notification channel
 - Add `Permissions::CONNECT` to the permissions the bot needs on the fronter category

### Changed

 - Update actions/checkout to v6.0.2
 - Update nixbuild/nix-quick-install-action to v34
 - Update cachix/cachix-action to v17
 - Yaml file formatting

### Fixed

 - Correctly bump version on fixes
 - Fix a crash in `update_fronter_channels` and improve logging
 - Leftover reference to http-proxy

### Commit Details

<details><summary>view details</summary>

 * fix(build): correctly bump version on fixes ([`73169ce`](https://github.com/tulpje/tulpje/commit/73169cefefc7db3a8a4243cf340a49085b783dba))
 * fix(handler/pk): fix a crash in `update_fronter_channels` and improve logging ([`bcbe497`](https://github.com/tulpje/tulpje/commit/bcbe4970acc69e80d5a83eb6d86776c119c47ab1))
 * fix(handler/pk): add missing `EMBED_LINKS` permission for to the notification channel ([`97b4949`](https://github.com/tulpje/tulpje/commit/97b4949f4ae11d5b1e2c9edac94a8a06817cbed7))
 * fix(handler/pk): add `Permissions::CONNECT` to the permissions the bot needs on the fronter category ([`f8f2645`](https://github.com/tulpje/tulpje/commit/f8f2645f21e0cc3b5f3bd1bb250e883942ee8513))
 * chore(ci): update actions/checkout to v6.0.2 ([`c99d002`](https://github.com/tulpje/tulpje/commit/c99d002ab01b1481149781d4f948365206b35fbf))
 * chore(ci): update nixbuild/nix-quick-install-action to v34 ([`f8d77eb`](https://github.com/tulpje/tulpje/commit/f8d77ebba62b4b70a09513861926ada49930973a))
 * chore(ci): update cachix/cachix-action to v17 ([`ff2a694`](https://github.com/tulpje/tulpje/commit/ff2a694d1947ebdf614266b096b18366736f609c))
 * chore: yaml file formatting ([`061989e`](https://github.com/tulpje/tulpje/commit/061989eb939c9d547e80c48715088b71c7770aa1))
 * fix(ci): leftover reference to http-proxy ([`594ab31`](https://github.com/tulpje/tulpje/commit/594ab31119c279215a3bfd696ce6e2d398cb194d))
</details>

## [0.21.0-rc.2] - 2026-10-04

### Fixed

 - Correctly update `updated_at` when front hasn't changed

### Commit Details

<details><summary>view details</summary>

 * fix(handler/pk): correctly update `updated_at` when front hasn't changed ([`0ca4b7e`](https://github.com/tulpje/tulpje/commit/0ca4b7e64c35f2777f64adfe480443b61d39ed0f))
</details>

## [0.21.0-rc.1] - 2026-10-04

### Breaking Changes

 - Switch to nirn-proxy instead of twilight-http-proxy

### Changed

 - Always create a new category when `/pk fronters setup` is used
 - Restrict requested intents
 - Document required permissions

### Fixed

 - Continue without loading secrets if directory is missing
 - Use `title` for fronter category name argument to clarify usage

### Commit Details

<details><summary>view details</summary>

 * fix(utils/secret-loader): continue without loading secrets if directory is missing ([`a98a185`](https://github.com/tulpje/tulpje/commit/a98a1852777306155c83b9160f94553e8ddac3cf))
 * feat!: switch to nirn-proxy instead of twilight-http-proxy ([`6cb0e80`](https://github.com/tulpje/tulpje/commit/6cb0e80bc4c952f852e9dccadff63620ad5d3a0b))
 * fix(handler/pk): use `title` for fronter category name argument to clarify usage ([`3edae5a`](https://github.com/tulpje/tulpje/commit/3edae5a47d8d3fbccb31d1593c83fb8ad312316a))
 * refactor(handler/pk): always create a new category when `/pk fronters setup` is used ([`19f8a0c`](https://github.com/tulpje/tulpje/commit/19f8a0ccd5cd88f2a2bb9d6b4c58823a9b546b03))
 * chore(gateway): restrict requested intents ([`9688cba`](https://github.com/tulpje/tulpje/commit/9688cbaedc166a6fd84764b58f6b6a51cc7445a3))
 * chore(docs): document required permissions ([`c50e5d5`](https://github.com/tulpje/tulpje/commit/c50e5d57a4fe248b8d308f823757b954373038d5))
</details>

## [0.20.2-rc.1] - 2026-10-04

### Added

 - Add a user guide
 - Add missing permission overwrites in `/pk notify setup`

### Changed

 - Use `handle_system_ref` in `/pk setup`
 - Move `handle_system_ref` to module wide utils
 - Handle 404 on `/pk setup`
 - Nicer messages in various places
 - Use new channel and permission code for `/pk fronters setup`
 - Rework and split out channel finding and permission checking logic
 - Make permission checking code generic
 - Better permission check code
 - Move reusable responses into dedicated module
 - Move some utility functions to crate root
 - Move `create_or_get_fronter_channel` next to setup command
 - Handle channel references in `/pk notify setup`
 - Error if user haven't set up a notify channel yet but they try to add a system

### Fixed

 - Make `find_channel_by_name` filter on channel type
 - Also release if we go from prerelease to full

### Removed

 - Remove old debug statement

### Commit Details

<details><summary>view details</summary>

 * chore(docs): add a user guide ([`8181830`](https://github.com/tulpje/tulpje/commit/8181830758522f9d33075a3fa7fc72c604af8cee))
 * refactor(handler/pk): use `handle_system_ref` in `/pk setup` ([`8a76788`](https://github.com/tulpje/tulpje/commit/8a76788470097f88a5801dd807d3e49324063d71))
 * refactor(handler/pk): move `handle_system_ref` to module wide utils ([`62abd4b`](https://github.com/tulpje/tulpje/commit/62abd4b62ec908e58b8a3c1872e60987268ec8ad))
 * chore(handler/pk): remove old debug statement ([`0a78884`](https://github.com/tulpje/tulpje/commit/0a788846b45850ac0d9f993096fd19340ae27b1d))
 * feat(handler/pk): handle 404 on `/pk setup` ([`f50668d`](https://github.com/tulpje/tulpje/commit/f50668d839f0f7c129a53fbe9f6e011f0f356048))
 * feat(handler/pk): nicer messages in various places ([`970d07f`](https://github.com/tulpje/tulpje/commit/970d07f9b949c8d6a097870ef11ff8f0048c8b93))
 * fix(handler): make `find_channel_by_name` filter on channel type ([`69a0c04`](https://github.com/tulpje/tulpje/commit/69a0c04d59e3592a7061a1cdfe51fa790ca23485))
 * fix(handler/pk): add missing permission overwrites in `/pk notify setup` ([`9f8be88`](https://github.com/tulpje/tulpje/commit/9f8be889a1728102376b85d158f9109b06f96291))
 * refactor(handler/pk): use new channel and permission code for `/pk fronters setup` ([`0031d6c`](https://github.com/tulpje/tulpje/commit/0031d6c6043336f5bc07296fc3595c25108daa35))
 * refactor(handler/pk): rework and split out channel finding and permission checking logic ([`159e701`](https://github.com/tulpje/tulpje/commit/159e7019497f3f30f18a6d538c62097a66d88815))
 * refactor(handler): make permission checking code generic ([`4bd43cd`](https://github.com/tulpje/tulpje/commit/4bd43cdbf197794bbe4c47cb61ecdaef4a3008d2))
 * refactor(handler): better permission check code ([`74602b1`](https://github.com/tulpje/tulpje/commit/74602b1c711e941a91fcb45a94d15b99b96252ef))
 * refactor(handler): move reusable responses into dedicated module ([`05f5ccb`](https://github.com/tulpje/tulpje/commit/05f5ccb561992e810a180a7312ccac7e26e61f1d))
 * refactor(handler): move some utility functions to crate root ([`c490636`](https://github.com/tulpje/tulpje/commit/c4906367ec942242172ff2b8918c5f4bc3f7763c))
 * refactor(handler/pk): move `create_or_get_fronter_channel` next to setup command ([`03aaba2`](https://github.com/tulpje/tulpje/commit/03aaba21fd07ebc5e3bbf49df5aa1b4b37a25f1c))
 * feat(handler/pk): handle channel references in `/pk notify setup` ([`04884d3`](https://github.com/tulpje/tulpje/commit/04884d3efdff27240eb28c9b0a16ee0c8bd0bbc9))
 * feat(handler/pk): error if user haven't set up a notify channel yet but they try to add a system ([`ee8881d`](https://github.com/tulpje/tulpje/commit/ee8881d7f7fc190f4d4b093dca7eccdb8db3624a))
 * fix(build): also release if we go from prerelease to full ([`8fcbcc2`](https://github.com/tulpje/tulpje/commit/8fcbcc2775a648967aae3082999c6570c482e391))
</details>

## [0.20.1] - 2026-10-04

### Changed

 - Clean shutdown

### Commit Details

<details><summary>view details</summary>

 * feat(handler): clean shutdown ([`eebb951`](https://github.com/tulpje/tulpje/commit/eebb951c00f6202be11e4745cc37d7347176dc2a))
</details>

## [0.20.1-rc.1] - 2026-10-04

### Added

 - Add missing `git-cliff` package to flake
 - Add check in `/pk roles update` for discord role limit
 - Additional comments
 - Additional comments
 - Add env var to enable log source file and line no logging
 - Add user facing errors for incorrect system references

### Changed

 - Let user know if they were/weren't already following a system
 - Implement follow limit
 - Update flake inputs to latest
 - Use a global pk client
 - Implement `Display` for `SystemRef`
 - Check front is public during front category setup
 - Track number of guilds we're in
 - Use fancy response messages in `/pk notify setup`
 - Split up `/pk notify setup` code
 - Use fancy response messages in fronter module
 - Factor out repetitive system ref parsing code
 - Implement `success_response` and `error_response` utility functions
 - Move role commands into subgroup and subcategory
 - Clippy fix
 - Split up fronter submodule more clearly
 - Move fronter command definitions into fronter module

### Fixed

 - Fix feature commit detection
 - Fix clippy warnings
 - Use `PkClient::with_token` to reuse existing client
 - Show system name in `/pk fronters setup` if available
 - Correctly handle private front checking
 - Correctly handle systems without registered switches
 - Correctly use `Span`s with async functions
 - Typo
 - Make `/pk fronters setup` show error if pk module hasn't been setup yet

### Commit Details

<details><summary>view details</summary>

 * fix(build): add missing `git-cliff` package to flake ([`b09db28`](https://github.com/tulpje/tulpje/commit/b09db28c63eedf33ebd60bde3f26f70a59234956))
 * fix(build): fix feature commit detection ([`5bd005e`](https://github.com/tulpje/tulpje/commit/5bd005ee7c642df51fe6453dcb6135eab5f2ea14))
 * feat(handler/pk): let user know if they were/weren't already following a system ([`b39b532`](https://github.com/tulpje/tulpje/commit/b39b5323020c79a2eafa793f5ed96d94495a25e0))
 * feat(handler/pk): implement follow limit ([`7a3001e`](https://github.com/tulpje/tulpje/commit/7a3001eb875bff2c2f1f27880d78881a14660ee8))
 * feat(handler/pk): add check in `/pk roles update` for discord role limit ([`087fc35`](https://github.com/tulpje/tulpje/commit/087fc355ea2ddd91722e8e6c752548a4658617ef))
 * chore(handler/pk): fix clippy warnings ([`64d2c40`](https://github.com/tulpje/tulpje/commit/64d2c40c14351ce6c03774e58719a78789d72129))
 * fix(handler/pk): use `PkClient::with_token` to reuse existing client ([`ca207dc`](https://github.com/tulpje/tulpje/commit/ca207dc80cf1aeb8bb4b4049673c15da27f3362d))
 * chore(deps): update flake inputs to latest ([`838ada6`](https://github.com/tulpje/tulpje/commit/838ada66d2a577cdb395f8e25e6309cec68ae94f))
 * refactor(handler): use a global pk client ([`86644fe`](https://github.com/tulpje/tulpje/commit/86644feb25bbe16e63cee5cd89be03284be2dd49))
 * fix(handler/pk): show system name in `/pk fronters setup` if available ([`e07c732`](https://github.com/tulpje/tulpje/commit/e07c732c6cbe55c7e5c315d04ccfebf5852be27a))
 * fix(handler/pk): correctly handle private front checking ([`74e2094`](https://github.com/tulpje/tulpje/commit/74e2094e41fdc34f2049b5c176f98f5edf5f1c00))
 * feat(handler/pk): implement `Display` for `SystemRef` ([`35598a9`](https://github.com/tulpje/tulpje/commit/35598a95a7fbcfc9355f45dfd5ce965dc343e8d6))
 * chore(handler/pk): additional comments ([`b0c4bb4`](https://github.com/tulpje/tulpje/commit/b0c4bb4f046ba587d9a89398b8b14395407b7adc))
 * fix(handler/pk): correctly handle systems without registered switches ([`721991d`](https://github.com/tulpje/tulpje/commit/721991de62a00b9dcbf901ffb602dcd3f81a9840))
 * chore(handler/pk): additional comments ([`2f7be15`](https://github.com/tulpje/tulpje/commit/2f7be15a9e2461c277ef7487eae9fca36298d23e))
 * feat(handler/pk): check front is public during front category setup ([`9246ed3`](https://github.com/tulpje/tulpje/commit/9246ed37ad6c9d73daab2986bdbfd3e9661daf5c))
 * fix: correctly use `Span`s with async functions ([`1f61330`](https://github.com/tulpje/tulpje/commit/1f61330e614948c0874f8784680f4f1aed14aed0))
 * feat(shared): add env var to enable log source file and line no logging ([`06cff50`](https://github.com/tulpje/tulpje/commit/06cff502926ffe0f5ae43f7162052929ea39082e))
 * fix(gateway): typo ([`dd85e4a`](https://github.com/tulpje/tulpje/commit/dd85e4afc96dcad9d397cbac06c7530597552a7e))
 * feat(gateway): track number of guilds we're in ([`c169c03`](https://github.com/tulpje/tulpje/commit/c169c037b8924bec6de3e33aeab7019b31fcd457))
 * feat(handler/pk): use fancy response messages in `/pk notify setup` ([`a8ea278`](https://github.com/tulpje/tulpje/commit/a8ea278799bc173620f6eef8549f61f79383e789))
 * refactor(handler/pk): split up `/pk notify setup` code ([`f88c479`](https://github.com/tulpje/tulpje/commit/f88c4792a4464c8bd73460b325fe0b13fe3d470b))
 * feat(handler/pk): use fancy response messages in fronter module ([`85d412c`](https://github.com/tulpje/tulpje/commit/85d412cfd6e0e3ec7e822f042b365b549d4bf608))
 * fix(handler/pk): make `/pk fronters setup` show error if pk module hasn't been setup yet ([`8bc783d`](https://github.com/tulpje/tulpje/commit/8bc783d8acc30226ba3050ccb42a6126b517a7c9))
 * refactor(handler/pk): factor out repetitive system ref parsing code ([`28c9b25`](https://github.com/tulpje/tulpje/commit/28c9b2532c00ec74469a361d66e88034c39f1206))
 * refactor(handler/pk): implement `success_response` and `error_response` utility functions ([`8b6c8af`](https://github.com/tulpje/tulpje/commit/8b6c8af246312743af046fc855ceddd8ee642176))
 * refactor(handler/pk): move role commands into subgroup and subcategory ([`badd1fe`](https://github.com/tulpje/tulpje/commit/badd1fe9a1b860930e091a9378abe4f47218ad4e))
 * chore(shared): clippy fix ([`9fb2f6e`](https://github.com/tulpje/tulpje/commit/9fb2f6ee7c9438e874e2191cab5aa9f8b2d13574))
 * refactor(handler/pk): split up fronter submodule more clearly ([`6e1d9b4`](https://github.com/tulpje/tulpje/commit/6e1d9b4628862b8eb5aa7781c5ff6fb8ebb45c62))
 * refactor(handler/pk): move fronter command definitions into fronter module ([`e9d94f3`](https://github.com/tulpje/tulpje/commit/e9d94f3be945248fa222e5e57d953288d4ebc839))
 * fix(handler/pk): add user facing errors for incorrect system references ([`a94b117`](https://github.com/tulpje/tulpje/commit/a94b1173134c7c43a183b2221957cc01f914c5b3))
</details>

## [0.20.0] - 2026-10-04

### Fixed

 - Log message typo

### Commit Details

<details><summary>view details</summary>

 * fix(handler/pk): log message typo ([`7c52141`](https://github.com/tulpje/tulpje/commit/7c521410b4622fe1fd7afb4a6bf3ef650ccf3cc2))
</details>

## [0.20.0-rc.3] - 2026-10-04

### Changed

 - More logging in `process_system`

### Fixed

 - Fix RUST_LOG not being honoured, and spans not showing

### Commit Details

<details><summary>view details</summary>

 * fix(shared/logging): fix RUST_LOG not being honoured, and spans not showing ([`c1ffe79`](https://github.com/tulpje/tulpje/commit/c1ffe797e83ffb7161e3346ee7c2d7e4118b91db))
 * chore(module/pk): more logging in `process_system` ([`1762bfc`](https://github.com/tulpje/tulpje/commit/1762bfcf7396f43bc8d6e63f03b96fd148bcbcfb))
</details>

## [0.20.0-rc.2] - 2026-10-04

### Fixed

 - Mark prereleases correctly
 - Flatten event fields so VictoriaLogs can parse it

### Commit Details

<details><summary>view details</summary>

 * fix(build): mark prereleases correctly ([`fb32652`](https://github.com/tulpje/tulpje/commit/fb32652c3fc665afb95bb3087048f47931c2f24b))
 * fix(shared/logging): flatten event fields so VictoriaLogs can parse it ([`4fa2fdf`](https://github.com/tulpje/tulpje/commit/4fa2fdfccfbacd9d5be835e6585f943a6bf48578))
</details>

## [0.20.0-rc.1] - 2026-10-04

### Added

 - Add extra metrics
 - Add tracing::span to event handling
 - Add `sqlx-migrate` and `sqlx-prepare` tasks

### Changed

 - Enable globally instead of per guild
 - Configurable log format using `RUST_LOG_FORMAT` env var
 - Prettier responses when adding/removing notify systems
 - Better logging for update-member-roles
 - Update vulnerable deps
 - Staggered fronter updates
 - Use cache when updating fronters
 - Use `sqlx::query_as!` instead of manual conversion
 - Front change notifications
 - Update pkrs from 0.4.0 to 0.5.0
 - Don't save token, only optionally use it to update member roles
 - Update vulnerable deps
 - Switch to using `mod.rs` files
 - Update to rust 1.94.0

### Fixed

 - Still set `last_updated` when front is forbidden
 - Also handle systems we can't fetch from PluralKit in add/remove
 - Fix clippy warnings

### Removed

 - Delete fronters before deleting system

### Commit Details

<details><summary>view details</summary>

 * chore(module/pk): enable globally instead of per guild ([`10b60b4`](https://github.com/tulpje/tulpje/commit/10b60b419d123fc938ce55eb8cd6a834fbf7497c))
 * feat(handler/pk): add extra metrics ([`f526948`](https://github.com/tulpje/tulpje/commit/f526948ca29f977da29d030456393dcdf88edcb9))
 * feat: configurable log format using `RUST_LOG_FORMAT` env var ([`dde6a0d`](https://github.com/tulpje/tulpje/commit/dde6a0d69188402eb8918799d98176f3aa78ee90))
 * feat(framework): add tracing::span to event handling ([`4bfa125`](https://github.com/tulpje/tulpje/commit/4bfa125fa567b20115a0a2c09281c154333e9d9d))
 * feat(pk): prettier responses when adding/removing notify systems ([`40adc3d`](https://github.com/tulpje/tulpje/commit/40adc3d9d932441dafb97837cbdc4c3abd4e1e01))
 * feat(pk): better logging for update-member-roles ([`7036c60`](https://github.com/tulpje/tulpje/commit/7036c608571fae67b029cdf236ff40a3569dc3f0))
 * fix(pk): still set `last_updated` when front is forbidden ([`724d254`](https://github.com/tulpje/tulpje/commit/724d254cad07d2cb28eb3bf5fd5db1edae4da433))
 * chore(deps): update vulnerable deps ([`3849ea8`](https://github.com/tulpje/tulpje/commit/3849ea8df1b598d4739fdae900db194ea84ea0c3))
 * fix(pk): also handle systems we can't fetch from PluralKit in add/remove ([`5c2e51b`](https://github.com/tulpje/tulpje/commit/5c2e51b40b144e03ceaa76d77f39711f18ad42c8))
 * fix(pk): delete fronters before deleting system ([`ae2f254`](https://github.com/tulpje/tulpje/commit/ae2f2541006017c3321fe49d20f5a0b01385d036))
 * feat(pk): staggered fronter updates ([`7409016`](https://github.com/tulpje/tulpje/commit/740901666b16f47aa28336d5cd2b284aa66c3ae3))
 * refactor(handler/pk): use cache when updating fronters ([`6c863e8`](https://github.com/tulpje/tulpje/commit/6c863e80ce08a2842e5558b43dc318307e3346fd))
 * refactor(handler/pk): use `sqlx::query_as!` instead of manual conversion ([`91a344f`](https://github.com/tulpje/tulpje/commit/91a344f8d1449551389964202d52f6addc0bd18e))
 * feat(justfile): add `sqlx-migrate` and `sqlx-prepare` tasks ([`88d27d6`](https://github.com/tulpje/tulpje/commit/88d27d625dd034253d3a647542db4c7514f48d30))
 * feat(pluralkit): front change notifications ([`bd16e3d`](https://github.com/tulpje/tulpje/commit/bd16e3dc421311b85b68058fd9f7467c890ff284))
 * chore(deps): update pkrs from 0.4.0 to 0.5.0 ([`45893e9`](https://github.com/tulpje/tulpje/commit/45893e9944e4c446184381cd098cb03217b755c3))
 * refactor(pk): don't save token, only optionally use it to update member roles ([`7fe129f`](https://github.com/tulpje/tulpje/commit/7fe129ffcedfbd5a0d3c2ea85375127247767fed))
 * chore(deps): update vulnerable deps ([`b51b09a`](https://github.com/tulpje/tulpje/commit/b51b09a66e06b5603cd619abbca64a356700e70b))
 * refactor: switch to using `mod.rs` files ([`e7614fa`](https://github.com/tulpje/tulpje/commit/e7614fa7ae6bed2ad1b5113d4d5efd3e351b9b15))
 * chore: fix clippy warnings ([`54920a2`](https://github.com/tulpje/tulpje/commit/54920a2d104aee0c39f012cac86d4115ec937ab1))
 * chore: update to rust 1.94.0 ([`aaf37e6`](https://github.com/tulpje/tulpje/commit/aaf37e6ec1fc2911e898a9e1b0d9948da306b496))
</details>

## [0.19.1] - 2026-10-04

### Breaking Changes

 - Remove musl cross compilation

### Added

 - Add brief setup instructions to the README
 - Additional trace logging for fronter positions
 - Add cachix caching to `build-docker.sh`

### Changed

 - Bump version to 0.19.1-beta.2
 - Dedicated function for handling gateway messages
 - Move shard management into dedicated struct
 - Implement graceful shutdown
 - Use `tokio::select!`'s else branch to break out of loop
 - Move `ParsedEvent` to dedicated file
 - Don't use `TryFrom` for parsing Message into `ParsedEvent`
 - Rename `ShardManager` to `ShardReporter`
 - Return join handle from `ShardManagerHandle::new`
 - Refactor event parsing and processing
 - Enable `zstd` compression in `twilight-gateway`
 - Use `nix run` to run handler/gateway locally
 - Update `twilight-cache-inmemory`, `twilight-gateway`, `twilight-http` and `twilight-model` from 0.17.0 to 0.17.1
 - Make `chrono` a workspace dependency
 - Bump `tracing` from 0.1.43 to 0.1.44
 - Make `tracing` and `tracing-subscriber` workspace dependencies
 - Bump `serde_json` from 1.0.145 to 1.0.149
 - Update `reqwest` from 0.12.25 to 0.13.1
 - Make `serde` and `serde_json` workspace dependencies
 - Update `redis-rs` from 0.32.7 to 1.0.1
 - Bump reqwest from 0.12.24 to 0.12.25
 - Bump tracing-subscriber from 0.3.20 to 0.3.22
 - Bump metrics-exporter-prometheus from 0.18.0 to 0.18.1
 - Bump uuid from 1.18.1 to 1.19.0
 - Bump metrics-exporter-prometheus from 0.17.2 to 0.18.0
 - Bump tracing from 0.1.41 to 0.1.43
 - Bump metrics from 0.24.2 to 0.24.3
 - Bump redis from 0.32.6 to 0.32.7
 - Bump rsa from 0.9.7 to 0.9.10

### Fixed

 - Fix version bumping logic
 - Correctly preserve fronter order
 - Create channels with correct permissions
 - Better error handling on updating fronters
 - Don't clone Latency for every event send to `ShardReporter`
 - Don't panic on join errors, log them
 - Shut down amqp task before joining it
 - Switch from `ring` to aws-lc-rs`

### Removed

 - Remove unnecessary `pub` visibility on `ShardReporter`

### Commit Details

<details><summary>view details</summary>

 * fix(build): fix version bumping logic ([`daea733`](https://github.com/tulpje/tulpje/commit/daea733d61211ce7ed415d9605b1c84c84ac0ab0))
 * chore: add brief setup instructions to the README ([`e7d4584`](https://github.com/tulpje/tulpje/commit/e7d4584b2ac2f1de88dddef1994beeeec3e0e5af))
 * chore: bump version to 0.19.1-beta.2 ([`c91791a`](https://github.com/tulpje/tulpje/commit/c91791a757e4f271aec033a792300ce08d4938f0))
 * chore(handler/pk): additional trace logging for fronter positions ([`d067a1f`](https://github.com/tulpje/tulpje/commit/d067a1f86d4cb12015a3343ae5facac262ae78d3))
 * fix(handler/pk): correctly preserve fronter order ([`4fde536`](https://github.com/tulpje/tulpje/commit/4fde536c60691e1d85649751dc4b9804a2df4e9e))
 * fix(handler/pk): create channels with correct permissions ([`d5f0d81`](https://github.com/tulpje/tulpje/commit/d5f0d819bba1570e36419c70b32e1f5322c36610))
 * fix(handler/pk): better error handling on updating fronters ([`bb0a2cc`](https://github.com/tulpje/tulpje/commit/bb0a2cce6a87820e37fe5c4733653790d7edd5fd))
 * refactor(gateway): dedicated function for handling gateway messages ([`3361684`](https://github.com/tulpje/tulpje/commit/336168441cb9f98c521095b1f2ee97750a316504))
 * refactor(gateway): move shard management into dedicated struct ([`7468367`](https://github.com/tulpje/tulpje/commit/7468367dc88b43dee2fa8696fa4038e993ff7617))
 * feat(gateway): implement graceful shutdown ([`74693c3`](https://github.com/tulpje/tulpje/commit/74693c39d236a793defa2d967bb80d7a45119640))
 * refactor(gateway): use `tokio::select!`'s else branch to break out of loop ([`2e5792e`](https://github.com/tulpje/tulpje/commit/2e5792e4a288da85f0b3565fafb9d98ec21715f0))
 * fix(gateway): don't clone Latency for every event send to `ShardReporter` ([`e3df646`](https://github.com/tulpje/tulpje/commit/e3df6467494d4b0c025e3a9be514221ee1ee275d))
 * refactor(gateway): move `ParsedEvent` to dedicated file ([`743b89c`](https://github.com/tulpje/tulpje/commit/743b89c104ba43130789ea31f018672cccd66868))
 * refactor(gateway): don't use `TryFrom` for parsing Message into `ParsedEvent` ([`4dc8573`](https://github.com/tulpje/tulpje/commit/4dc8573ba915b15e972e356e61156bf0e55d9837))
 * fix(gateway): remove unnecessary `pub` visibility on `ShardReporter` ([`52de55b`](https://github.com/tulpje/tulpje/commit/52de55b9221399600a0365adc14f395276d2ab53))
 * refactor(gateway): rename `ShardManager` to `ShardReporter` ([`0587f03`](https://github.com/tulpje/tulpje/commit/0587f03d81ec9447e1776cdd7bb7bc769c70a2ac))
 * fix(gateway): don't panic on join errors, log them ([`45368b5`](https://github.com/tulpje/tulpje/commit/45368b53759fe89965d0084f8e4d57b7eb977917))
 * refactor(gateway): return join handle from `ShardManagerHandle::new` ([`7190c77`](https://github.com/tulpje/tulpje/commit/7190c77d24a825ba87fb2aab72ac88cb7c9aaf4c))
 * refactor(gateway): refactor event parsing and processing ([`1a2dc33`](https://github.com/tulpje/tulpje/commit/1a2dc33939d230c9b761070d4a8104365b17d1ec))
 * fix(gateway): shut down amqp task before joining it ([`1e927c6`](https://github.com/tulpje/tulpje/commit/1e927c6c0d9daead13ffa818a58d49f1d4c5af03))
 * fix(build)!: remove musl cross compilation ([`713993c`](https://github.com/tulpje/tulpje/commit/713993c4e4c7209b47f18e21e4d3106e5efa1332))
 * feat(gateway): enable `zstd` compression in `twilight-gateway` ([`46a4d01`](https://github.com/tulpje/tulpje/commit/46a4d01c0bad39e2495f3e90816537526a5d2fc7))
 * feat(build): add cachix caching to `build-docker.sh` ([`44b185f`](https://github.com/tulpje/tulpje/commit/44b185fba37e9a5aa34fe047bfef90bced937c6e))
 * refactor(build): use `nix run` to run handler/gateway locally ([`b9b6a47`](https://github.com/tulpje/tulpje/commit/b9b6a47f9bad47c7ab513de71097fc8cb7fafc0a))
 * fix: switch from `ring` to aws-lc-rs` ([`d6567bb`](https://github.com/tulpje/tulpje/commit/d6567bb6ae2c863ca415d5c066a0befbef6acf36))
 * chore(deps): update `twilight-cache-inmemory`, `twilight-gateway`, `twilight-http` and `twilight-model` from 0.17.0 to 0.17.1 ([`e3b1c33`](https://github.com/tulpje/tulpje/commit/e3b1c33fb5adb0e92cd1c161d0e317ab31dada4e))
 * chore(deps): make `chrono` a workspace dependency ([`9151b03`](https://github.com/tulpje/tulpje/commit/9151b03b0bd779f31a4c79042792cdff0182679c))
 * chore(deps): bump `tracing` from 0.1.43 to 0.1.44 ([`68f82a9`](https://github.com/tulpje/tulpje/commit/68f82a938a463c8b6ccb55099f90011efe9ddb55))
 * chore(deps): make `tracing` and `tracing-subscriber` workspace dependencies ([`234032f`](https://github.com/tulpje/tulpje/commit/234032f52f8e4016c537f3a4f60bc911a3c74b6a))
 * chore(deps): bump `serde_json` from 1.0.145 to 1.0.149 ([`6d14aa8`](https://github.com/tulpje/tulpje/commit/6d14aa807aa592227f3daa61543b2b5004b0fb02))
 * chore(deps): update `reqwest` from 0.12.25 to 0.13.1 ([`901fa50`](https://github.com/tulpje/tulpje/commit/901fa50eda5ad69c7dd5aecd40d0059bc5654b09))
 * chore(deps): make `serde` and `serde_json` workspace dependencies ([`52f8711`](https://github.com/tulpje/tulpje/commit/52f8711b9f31a4edf8fbe3c4b5de9f84f55d309f))
 * chore(deps): update `redis-rs` from 0.32.7 to 1.0.1 ([`ea32c13`](https://github.com/tulpje/tulpje/commit/ea32c1382b88364a12a64e75127238f05696ec09))
 * chore(deps): bump reqwest from 0.12.24 to 0.12.25 ([`ecec8d9`](https://github.com/tulpje/tulpje/commit/ecec8d9c8b5cd47d5a2aa88cf5f31749656575a8))
 * chore(deps): bump tracing-subscriber from 0.3.20 to 0.3.22 ([`d836c1e`](https://github.com/tulpje/tulpje/commit/d836c1ef2321ffeea6b6102d04be74f8940051d4))
 * chore(deps): bump metrics-exporter-prometheus from 0.18.0 to 0.18.1 ([`9f7b3e6`](https://github.com/tulpje/tulpje/commit/9f7b3e6b76623df6db819222e1b3377d3a700de0))
 * chore(deps): bump uuid from 1.18.1 to 1.19.0 ([`80a14f3`](https://github.com/tulpje/tulpje/commit/80a14f33fab65dd6c7aff93af878a4d9ffa0d670))
 * chore(deps): bump metrics-exporter-prometheus from 0.17.2 to 0.18.0 ([`77672c4`](https://github.com/tulpje/tulpje/commit/77672c4e4033d01670ad2a5a3ca5d616586e4956))
 * chore(deps): bump tracing from 0.1.41 to 0.1.43 ([`d94fd01`](https://github.com/tulpje/tulpje/commit/d94fd0157b04ddcc536343ef2612c1890104d164))
 * chore(deps): bump metrics from 0.24.2 to 0.24.3 ([`9a29cd4`](https://github.com/tulpje/tulpje/commit/9a29cd4b1a10ebe564190ab607735c818ff428c3))
 * chore(deps): bump redis from 0.32.6 to 0.32.7 ([`1fb3982`](https://github.com/tulpje/tulpje/commit/1fb3982f1c363ec1488b2e9801e812afa85d34d6))
 * build(deps): bump rsa from 0.9.7 to 0.9.10 ([`7309ff0`](https://github.com/tulpje/tulpje/commit/7309ff0dec42e42e479e214164ac2bfecbf09811))
</details>

## [0.19.0-rc.2] - 2026-10-04

### Added

 - Support prereleases in `release.py`

### Fixed

 - Don't exclude alpha/beta/rc tags in changelog
 - Always include prereleases
 - Re-add `metrics` feature flag, was renamed not removed

### Commit Details

<details><summary>view details</summary>

 * fix(build/release): don't exclude alpha/beta/rc tags in changelog ([`b4838f6`](https://github.com/tulpje/tulpje/commit/b4838f6b2c5888d7b5edd2384356b56deebf9adb))
 * fix(build/release): always include prereleases ([`ffd1a29`](https://github.com/tulpje/tulpje/commit/ffd1a29d08527e0834d0d0d38caedd1a1ba30f17))
 * feat(build): support prereleases in `release.py` ([`9b3fc72`](https://github.com/tulpje/tulpje/commit/9b3fc7283a0246fc8676b45502ed7c530ff4141f))
 * fix(http-proxy): re-add `metrics` feature flag, was renamed not removed ([`6f02f6f`](https://github.com/tulpje/tulpje/commit/6f02f6f2f012bd150f99f60ccafbc77b1034ef8c))
</details>

## [0.19.0-rc.1] - 2026-10-04

### Breaking Changes

 - Update twilight dependencies to 0.17.0
 - Remove unused parse_task_slot

### Added

 - Add missing fields to component structs
 - Add `cargo edit` and `cargo machete`
 - Add cargo-outdated to devenv packages
 - Additional comments and cleanup
 - Additional logging in `release.py

### Changed

 - Use gateway-queue fork using twilight 0.17
 - Update to rust 1.91.0
 - Bump `tokio-util` from 0.7.16 to 0.7.17
 - Make `tokio-util` a workspace dependency
 - Bump `reqwest` from 0.12.23 to 0.12.24
 - Make `reqwest` a workspace dependency
 - Bump `amqprs` from 2.1.2 to 2.1.3
 - Bump `regex` from 1.11.3 to 1.12.2
 - Bump `metrics-process` from 2.4.1 to 2.4.2
 - Bump `tokio` from 1.47.1 to 1.48.0
 - Make `tokio` a workspace dependency
 - Configure tls correctly for 0.17
 - Update to 0.17.0
 - Update to latest version
 - Update flake inputs to latest
 - Bump `async-trait` from 0.1.86 to 0.1.89
 - Bump `sqlx` from 0.8.5 to 0.8.6
 - Bump `regex` from 1.11.3 to 1.11.5
 - Bump `serde_json` from 1.0.140 to 1.0.145
 - Bump `serde` from 1.0.219 to 1.0.228
 - Bump `reqwest` from 0.12.15 to 0.12.23
 - Bump tokio from 1.44.2 to 1.47.1
 - Bump tokio-util from 0.7.14 to 0.7.16
 - Bump chrono from 0.4.40 to 0.4.42
 - Bump `uuid` from 0.16.0 to 0.18.1
 - Bump `metrics-process from 2.4.0 to 2.4.1
 - Bump `metrics-exporter-prometheus` from 0.16.2 to 0.17.1
 - Move `metrics-exporter-prometheus` crate to workspace deps
 - Bump redis from 0.29.1 to 0.32.6
 - Move redis crate to workspace deps
 - Update to rust 1.90.0, fix lint warnings, `cargo fmt`
 - `clippy::collapsible_if`
 - `cargo fmt`
 - Rust edition 2024
 - Pass env vars to child process instead of directly setting
 - Specify edition on workspace level
 - Bump cargo feature resolver from 2 to 3
 - Move twilight-* crates to workspace deps
 - Update amqprs from 2.1.0 to 2.1.2
 - Update flake inputs to latest
 - Update to rust 1.89.0
 - Rust-like output for `release.py`

### Fixed

 - Use primary_color for member roles
 - ReadyInfo no longer needs to be dereferenced
 - Also use non-annotated git tags to determine version
 - Log invalid semver tags and skip them instead of crashing in release script
 - Bump tracing subscriber from 0.3.19 to 0.3.20
 - Hanging connections

### Removed

 - Remove unused `crate` argument from `release.py`
 - Remove clippy warning that no longer exists
 - Remove feature flag removed by upstream
 - Remove tls feature flags that got removed in twilight 0.17
 - Remove unused deps

### Commit Details

<details><summary>view details</summary>

 * fix(build): remove unused `crate` argument from `release.py` ([`a19c950`](https://github.com/tulpje/tulpje/commit/a19c950780e49985189c3c2a88ef7e363b9d4011))
 * chore: use gateway-queue fork using twilight 0.17 ([`67ccc6d`](https://github.com/tulpje/tulpje/commit/67ccc6db7315ef42f7a537d8a4ea9c662767e821))
 * fix(handler): remove clippy warning that no longer exists ([`523e486`](https://github.com/tulpje/tulpje/commit/523e486e1ffc729b5d3607c58e1a0e3ededf30a7))
 * chore: update to rust 1.91.0 ([`fdd37f3`](https://github.com/tulpje/tulpje/commit/fdd37f345517849f5442512b733b0a4c5b7a9fa4))
 * chore(deps): bump `tokio-util` from 0.7.16 to 0.7.17 ([`4111db1`](https://github.com/tulpje/tulpje/commit/4111db1c97dee14a5927438fe15530daffbd7379))
 * chore(deps): make `tokio-util` a workspace dependency ([`1665fbe`](https://github.com/tulpje/tulpje/commit/1665fbe97bd472c878daff8024660078a2cd961e))
 * chore(deps): bump `reqwest` from 0.12.23 to 0.12.24 ([`1634635`](https://github.com/tulpje/tulpje/commit/16346358a5f3658f408bc4034483b131486996fb))
 * chore(deps): make `reqwest` a workspace dependency ([`d3a1cff`](https://github.com/tulpje/tulpje/commit/d3a1cff296b6128b28a5ee9c8101710782300927))
 * chore(deps): bump `amqprs` from 2.1.2 to 2.1.3 ([`5bcdb68`](https://github.com/tulpje/tulpje/commit/5bcdb687007854d1b425cd9eafc7c422ac858d1b))
 * chore(deps): bump `regex` from 1.11.3 to 1.12.2 ([`5ad15e2`](https://github.com/tulpje/tulpje/commit/5ad15e2a74cb6fa4dcbb2f68b4dcb92300fe603c))
 * chore(deps): bump `metrics-process` from 2.4.1 to 2.4.2 ([`b813789`](https://github.com/tulpje/tulpje/commit/b8137897a268a8510cb33b196627dd2e8d4cfcc8))
 * chore(deps): bump `tokio` from 1.47.1 to 1.48.0 ([`059826a`](https://github.com/tulpje/tulpje/commit/059826ac41b0535bd9d1e4bc0194d08ffff076e9))
 * chore(deps): make `tokio` a workspace dependency ([`894a678`](https://github.com/tulpje/tulpje/commit/894a678d0bdad4c1dee99bfd9fb64c2fc0b71013))
 * fix(handler/emoji): add missing fields to component structs ([`49949c3`](https://github.com/tulpje/tulpje/commit/49949c340d3cdcaed419c654ec325ba605186986))
 * chore: configure tls correctly for 0.17 ([`2f8c177`](https://github.com/tulpje/tulpje/commit/2f8c17793e791b0b26f9acf936dfcd0bade2a931))
 * fix(http-proxy): remove feature flag removed by upstream ([`06226ae`](https://github.com/tulpje/tulpje/commit/06226aeec817ed5ee512ff1d1ffc4fe151d8fae4))
 * chore(http-proxy): update to 0.17.0 ([`5fc93fb`](https://github.com/tulpje/tulpje/commit/5fc93fb08e5097c2a3c4cb27bd92efb790aebd84))
 * chore(gateway-queue): update to latest version ([`ac5e6f0`](https://github.com/tulpje/tulpje/commit/ac5e6f0bf2e3cf66f8e01222f88535f26c1fe5a5))
 * fix(handler/pk): use primary_color for member roles ([`2a3d2b7`](https://github.com/tulpje/tulpje/commit/2a3d2b78c1b6cd7c188a262479e2fd44db770c96))
 * fix: remove tls feature flags that got removed in twilight 0.17 ([`ee69ff8`](https://github.com/tulpje/tulpje/commit/ee69ff8eb83304319482b74630b088f6ef0d05cc))
 * fix(gateway): ReadyInfo no longer needs to be dereferenced ([`b442e25`](https://github.com/tulpje/tulpje/commit/b442e25de158793ab5de46a981707bd57c6ce55b))
 * chore!: update twilight dependencies to 0.17.0 ([`0dcb4ab`](https://github.com/tulpje/tulpje/commit/0dcb4abcc88eb5f5754eb548f283f36ff60aff4e))
 * chore(deps): update flake inputs to latest ([`4b0ef6c`](https://github.com/tulpje/tulpje/commit/4b0ef6c6c70be58a49b077b55b37c3689d360ef7))
 * chore(deps): bump `async-trait` from 0.1.86 to 0.1.89 ([`652f101`](https://github.com/tulpje/tulpje/commit/652f10112fd52fc077d78ea135d903cd978667a6))
 * chore(deps): bump `sqlx` from 0.8.5 to 0.8.6 ([`ce98dea`](https://github.com/tulpje/tulpje/commit/ce98dea3965c97f364ef187860db71959173b061))
 * chore(deps): bump `regex` from 1.11.3 to 1.11.5 ([`6ae2e68`](https://github.com/tulpje/tulpje/commit/6ae2e68b156c20100ad4c89f7fae496d558e2111))
 * chore(deps): bump `serde_json` from 1.0.140 to 1.0.145 ([`7e2baa3`](https://github.com/tulpje/tulpje/commit/7e2baa398dae6719dd07d53875f6e1639875d1df))
 * chore(deps): bump `serde` from 1.0.219 to 1.0.228 ([`5511665`](https://github.com/tulpje/tulpje/commit/55116655b37504981043e098367cfd8df29e8691))
 * chore(deps): bump `reqwest` from 0.12.15 to 0.12.23 ([`d3196af`](https://github.com/tulpje/tulpje/commit/d3196afdcf81f1037f67f912ea57f4ac791a74f3))
 * build(deps): bump tokio from 1.44.2 to 1.47.1 ([`ab3d857`](https://github.com/tulpje/tulpje/commit/ab3d8574b5a4dd5af4600b7beba9013119fd231f))
 * build(deps): bump tokio-util from 0.7.14 to 0.7.16 ([`fa43736`](https://github.com/tulpje/tulpje/commit/fa437366b1cd4604209ca991d2687cc460b65b1a))
 * build(deps): bump chrono from 0.4.40 to 0.4.42 ([`e15d587`](https://github.com/tulpje/tulpje/commit/e15d587bcdc426c41c903b5917c0873bc0d7f3b4))
 * chore(deps): bump `uuid` from 0.16.0 to 0.18.1 ([`62feb29`](https://github.com/tulpje/tulpje/commit/62feb298de8626a4ed5772997a490309de49be70))
 * chore(deps): bump `metrics-process from 2.4.0 to 2.4.1 ([`b0368c3`](https://github.com/tulpje/tulpje/commit/b0368c3c0b945531f5cd0bad42c18e80932e85ff))
 * chore(deps): bump `metrics-exporter-prometheus` from 0.16.2 to 0.17.1 ([`dd6b8db`](https://github.com/tulpje/tulpje/commit/dd6b8db3d18d48f455d47257176cd18abe10bbec))
 * chore(deps): move `metrics-exporter-prometheus` crate to workspace deps ([`d77d45e`](https://github.com/tulpje/tulpje/commit/d77d45e8591a307268b46e79834269b7843e2189))
 * chore(deps): bump redis from 0.29.1 to 0.32.6 ([`aeac3b5`](https://github.com/tulpje/tulpje/commit/aeac3b5c7d122eb6123e6848d52959d2e232b6c4))
 * chore(deps): move redis crate to workspace deps ([`0299f80`](https://github.com/tulpje/tulpje/commit/0299f80d7e8a099677875d2cc6c9cf96b880508e))
 * feat(build): add `cargo edit` and `cargo machete` ([`ef96960`](https://github.com/tulpje/tulpje/commit/ef969601d0ee757aec981389bdbc2885d5cbc7c7))
 * chore(deps): remove unused deps ([`972ad83`](https://github.com/tulpje/tulpje/commit/972ad8327bb1f1b8f3ae7025c16056de00f9a9ff))
 * chore(build): update to rust 1.90.0, fix lint warnings, `cargo fmt` ([`b850737`](https://github.com/tulpje/tulpje/commit/b8507378e832c0dee7bfedc74ebc72d3ad250235))
 * fix(build): also use non-annotated git tags to determine version ([`704f62f`](https://github.com/tulpje/tulpje/commit/704f62f9e84f7ccae7e53e51512e11b950c86f71))
 * chore(lint): `clippy::collapsible_if` ([`3280fa2`](https://github.com/tulpje/tulpje/commit/3280fa2aff1e50057231438153466c6839807201))
 * chore: `cargo fmt` ([`a08aa81`](https://github.com/tulpje/tulpje/commit/a08aa8152bc0422ea2a1c3740cfd59a098e26e58))
 * chore(build): rust edition 2024 ([`b54cdcf`](https://github.com/tulpje/tulpje/commit/b54cdcf811280075cc5db4387b4efb9df6e3e887))
 * refactor!: remove unused parse_task_slot ([`53b04a5`](https://github.com/tulpje/tulpje/commit/53b04a57d5fdbdfdedac1cb9a1b5e1a8577cbf9c))
 * refactor(secret-loader): pass env vars to child process instead of directly setting ([`2849500`](https://github.com/tulpje/tulpje/commit/28495008e3f872e91c6e66c2335a566f8d6665ef))
 * refactor(build): specify edition on workspace level ([`751c335`](https://github.com/tulpje/tulpje/commit/751c335316d1e9b4440e98e7435fa38ed1ea8c09))
 * chore(build): bump cargo feature resolver from 2 to 3 ([`7d109b9`](https://github.com/tulpje/tulpje/commit/7d109b9ad1bc61a6cace9d67b580458b9d8a4243))
 * feat(build): add cargo-outdated to devenv packages ([`f1d7a3d`](https://github.com/tulpje/tulpje/commit/f1d7a3daae60ec6f7b046c8e838db9520a4abe23))
 * chore(deps): move twilight-* crates to workspace deps ([`d36e8d5`](https://github.com/tulpje/tulpje/commit/d36e8d565d66b066284387a721bc3cc013e5365d))
 * chore(reconnecting-amqp/deps): update amqprs from 2.1.0 to 2.1.2 ([`7706b0b`](https://github.com/tulpje/tulpje/commit/7706b0b29ba2d4b7c8e9f5256f436f4ca46f5fb1))
 * chore(deps): update flake inputs to latest ([`a57fdbb`](https://github.com/tulpje/tulpje/commit/a57fdbb1288b193eaf58cd1fee1c31cd46e5fa97))
 * fix(build): log invalid semver tags and skip them instead of crashing in release script ([`e735131`](https://github.com/tulpje/tulpje/commit/e735131d9caad7a38da3f60c5c107a8a159657db))
 * chore(deps): update to rust 1.89.0 ([`758d607`](https://github.com/tulpje/tulpje/commit/758d60757ee94812f95f6aef627e806325137f61))
 * chore(nix): additional comments and cleanup ([`8cfd3e9`](https://github.com/tulpje/tulpje/commit/8cfd3e9e6abd9e14df1c28789882dc9c5bdb5b8d))
 * fix(deps): bump tracing subscriber from 0.3.19 to 0.3.20 ([`3691f1d`](https://github.com/tulpje/tulpje/commit/3691f1dc9d1a4b8cef7fde7aa97f672478153b33))
 * fix(gateway): hanging connections ([`5635aa6`](https://github.com/tulpje/tulpje/commit/5635aa6aa56de9cbecbafaefcaaeed0ef75fc0a2))
 * feat(build): additional logging in `release.py ([`b693181`](https://github.com/tulpje/tulpje/commit/b69318182345c83a9e4b0fa2d08d6580456c9918))
 * feat(build): rust-like output for `release.py` ([`d756e33`](https://github.com/tulpje/tulpje/commit/d756e331f01973612843f560707ab27129cdaa94))
</details>

## [0.18.0] - 2026-10-04

### Breaking Changes

 - Split `reconnecting-amqp` into separate crate

### Added

 - Add logging to `contrib/release.py`
 - Add start up messages containing version to gateway/handler
 - Add type safety to state transitions
 - Add `AmqpHandle::wait_start` that waits for amqp to connect

### Changed

 - Bump metrics from 0.24.1 to 0.24.2
 - Format `release.py` using `ruff`
 - Use python-semver in `release.py`
 - Use uv for `release.py`
 - Bump sqlx from 0.8.4 to 0.8.5
 - Use state machine, only reopen channel if channel closed
 - Rewrite using an event handler loop
 - Implement reconnection logic for amqp
 - Bump sqlx from 0.8.3 to 0.8.4
 - Make metrics listen address configurable
 - Move shared amqp logic into tulpje-shared

### Fixed

 - Fix changelog generation
 - Fix skopeo command

### Commit Details

<details><summary>view details</summary>

 * build: fix changelog generation ([`ddd452a`](https://github.com/tulpje/tulpje/commit/ddd452aa39ad26f934f6ca286228901acff19721))
 * build: add logging to `contrib/release.py` ([`5747a49`](https://github.com/tulpje/tulpje/commit/5747a497535bc1a80f129a9ae32ae64825aea4be))
 * feat: add start up messages containing version to gateway/handler ([`9f427b8`](https://github.com/tulpje/tulpje/commit/9f427b8abf72a6e6755177b22c877e18397dc9a6))
 * build(deps): bump metrics from 0.24.1 to 0.24.2 ([`b66f706`](https://github.com/tulpje/tulpje/commit/b66f7067b5c3b8fa3292e57fa65f6b93502cd4d3))
 * refactor!: split `reconnecting-amqp` into separate crate ([`5e8941b`](https://github.com/tulpje/tulpje/commit/5e8941b7179484fa095ef65d14230875b28e9503))
 * feat(shared/amqp): add type safety to state transitions ([`c1dfd8e`](https://github.com/tulpje/tulpje/commit/c1dfd8ec741228f7c5cde7dde8ca68c975383ca1))
 * chore(style): format `release.py` using `ruff` ([`c097470`](https://github.com/tulpje/tulpje/commit/c09747055702727e9788a3805044cfff1886fd09))
 * refactor(build): use python-semver in `release.py` ([`5408032`](https://github.com/tulpje/tulpje/commit/540803241c61f52a36cbad9ab9afdd7c0e1b86fa))
 * feat(build): use uv for `release.py` ([`0dda274`](https://github.com/tulpje/tulpje/commit/0dda2749fabd8aa8cbb6519fec81417ba9545b41))
 * build(deps): bump sqlx from 0.8.4 to 0.8.5 ([`9b0f20f`](https://github.com/tulpje/tulpje/commit/9b0f20f0a381c8e5474fb49a0b2448ff90a06ca3))
 * feat(shared/amqp): add `AmqpHandle::wait_start` that waits for amqp to connect ([`27c21ff`](https://github.com/tulpje/tulpje/commit/27c21ff28fd4ae4a7e6a4c02e13da47e0943cdfe))
 * refactor(shared/amqp): use state machine, only reopen channel if channel closed ([`80977ef`](https://github.com/tulpje/tulpje/commit/80977efd13a39d533d8a5f5842853cc104dfb69d))
 * refactor(shared/amqp): rewrite using an event handler loop ([`bdb5062`](https://github.com/tulpje/tulpje/commit/bdb5062ffe73566914cc2b3f2bed245485bbbea5))
 * feat(shared): implement reconnection logic for amqp ([`3b70812`](https://github.com/tulpje/tulpje/commit/3b708121045381253cc9b86924641ecc7e4d03cb))
 * build(deps): bump sqlx from 0.8.3 to 0.8.4 ([`fa618eb`](https://github.com/tulpje/tulpje/commit/fa618eb13f6879e0133368910c59f1f949946900))
 * feat(shared): make metrics listen address configurable ([`7651cbd`](https://github.com/tulpje/tulpje/commit/7651cbd7b03d8884ca6fa6e79a764d2b23bb414f))
 * refactor: move shared amqp logic into tulpje-shared ([`9a88105`](https://github.com/tulpje/tulpje/commit/9a88105812bf7c269de44d2b5d69ecf542910cbe))
 * build(github): fix skopeo command ([`f277654`](https://github.com/tulpje/tulpje/commit/f27765408d83bdfd60ca3efc1dd8426c17fefb84))
</details>

## [0.17.1] - 2026-10-04

### Added

 - Add cachix-action
 - Add nix-community substituter
 - Add github ci and docker image workflows

### Changed

 - Use personal cachix cache
 - Bump tokio from 1.43.0 to 1.44.2
 - Get local development with docker working again
 - Update twilight-gateway-queue to 5f5e0c1

### Fixed

 - Make dependabot target dev branch
 - Don't show double : in image name
 - Fix ability to run `docker compose up`

### Commit Details

<details><summary>view details</summary>

 * build(github): add cachix-action ([`c12b916`](https://github.com/tulpje/tulpje/commit/c12b916bae10169a7532115c8609c3ce3fd8dad2))
 * build: use personal cachix cache ([`4d2f584`](https://github.com/tulpje/tulpje/commit/4d2f584e3e5a6e8eefa27ca303238b38a672577b))
 * fix(build): add nix-community substituter ([`8a063ea`](https://github.com/tulpje/tulpje/commit/8a063eae4915c041f54cabfe501665d74a859789))
 * feat(ci): add github ci and docker image workflows ([`9010c52`](https://github.com/tulpje/tulpje/commit/9010c525ee459dfffac4f9f6f8a611dcf80e00d5))
 * build(deps): bump tokio from 1.43.0 to 1.44.2 ([`a737982`](https://github.com/tulpje/tulpje/commit/a737982c92cbe89eac6ab94f4b848dd53207a768))
 * fix(ci): make dependabot target dev branch ([`e2cff35`](https://github.com/tulpje/tulpje/commit/e2cff3504280091aa0e8600788505d8ffcd80676))
 * build: get local development with docker working again ([`4089d19`](https://github.com/tulpje/tulpje/commit/4089d19a8bc915442309d25df49827ba45b8ead9))
 * chore: update twilight-gateway-queue to 5f5e0c1 ([`a22cac8`](https://github.com/tulpje/tulpje/commit/a22cac8652c6c54105510147dba4a42c471f1811))
 * fix(build/push): don't show double : in image name ([`7a6dbef`](https://github.com/tulpje/tulpje/commit/7a6dbefb52662d94ab6fd17175ea8c8693775a39))
 * wip: fix ability to run `docker compose up` ([`ff01dba`](https://github.com/tulpje/tulpje/commit/ff01dba9de98dc57c047855a94b09faaf84e5d0e))
</details>

## [0.17.0] - 2026-10-04

### Breaking Changes

 - Switch to nix based images and devenv
 - Remove dotenvy
 - Use figment instead of serde_envfile

### Added

 - Add rust-toolchain.toml

### Changed

 - Move TASK_SLOT parsing into shared crate
 - Use single build.rs for both handler & gateway
 - Replace vergen_gitcl with simple code doing the same thing
 - Skip hidden files (dotfiles)
 - Make secret path configurable using SECRET_LOADER_PATH env var
 - Bump tokio-util from 0.7.13 to 0.7.14
 - Bump chrono from 0.4.39 to 0.4.40
 - Bump uuid from 1.15.1 to 1.16.0
 - Bump serde_json from 1.0.138 to 1.0.140
 - Bump reqwest from 0.12.9 to 0.12.15

### Fixed

 - Don't need init on the twilight containers
 - Don't error with custom IMAGE_TAG
 - Redis should have feature `tokio-comp` not `aio`

### Commit Details

<details><summary>view details</summary>

 * feat!: switch to nix based images and devenv ([`3c8a473`](https://github.com/tulpje/tulpje/commit/3c8a473b4190a4f4999e779b5e024029871969fd))
 * fix(compose): don't need init on the twilight containers ([`b30b0a7`](https://github.com/tulpje/tulpje/commit/b30b0a71f25407b4a27e54ab4fb6b1e2cf245db4))
 * refactor: move TASK_SLOT parsing into shared crate ([`b1f4bac`](https://github.com/tulpje/tulpje/commit/b1f4baca1d0e8ffa7ce0cf4456f29084574f61b8))
 * refactor!: remove dotenvy ([`51ed67d`](https://github.com/tulpje/tulpje/commit/51ed67df4a1dbd2a2c7360e32b76a3dce6a955c6))
 * refactor!: use figment instead of serde_envfile ([`f14eb13`](https://github.com/tulpje/tulpje/commit/f14eb130e0928df275de7821e17feaa7709d1e31))
 * refactor: use single build.rs for both handler & gateway ([`7b6770e`](https://github.com/tulpje/tulpje/commit/7b6770ed2faa95f4d173c65e1e7f56f36ee37547))
 * refactor: replace vergen_gitcl with simple code doing the same thing ([`5bdbe15`](https://github.com/tulpje/tulpje/commit/5bdbe15e3228bbca10f49b6eac68d0aeda2db01d))
 * feat(utils/secret-loader): skip hidden files (dotfiles) ([`092e19f`](https://github.com/tulpje/tulpje/commit/092e19fc16fe144d9f8be2165716d6680f503a0b))
 * feat(utils/secret-loader): make secret path configurable using SECRET_LOADER_PATH env var ([`6fd939b`](https://github.com/tulpje/tulpje/commit/6fd939b8f56e13a11707e909ac0919869183b5a8))
 * chore(build): add rust-toolchain.toml ([`887510d`](https://github.com/tulpje/tulpje/commit/887510dbb284a7cb5a5c797904874ab41c09d7e3))
 * fix(build/push): don't error with custom IMAGE_TAG ([`4f93180`](https://github.com/tulpje/tulpje/commit/4f93180800820aa4b284644c9ad3ea4fab46dc2a))
 * fix(handler): redis should have feature `tokio-comp` not `aio` ([`5a478cb`](https://github.com/tulpje/tulpje/commit/5a478cb694650f5d1b22d8265a40f2a409f3d60e))
 * build(deps): bump tokio-util from 0.7.13 to 0.7.14 ([`02f275c`](https://github.com/tulpje/tulpje/commit/02f275cc0f23d67b8ea733ad5734433ba4e9fa60))
 * build(deps): bump chrono from 0.4.39 to 0.4.40 ([`dc8a0dd`](https://github.com/tulpje/tulpje/commit/dc8a0dd32f6ae05b609de9c5c7f13c7339778f29))
 * build(deps): bump uuid from 1.15.1 to 1.16.0 ([`01e5716`](https://github.com/tulpje/tulpje/commit/01e57160b2f973d121e5d1e175830f88571eb2de))
 * build(deps): bump serde_json from 1.0.138 to 1.0.140 ([`b92e976`](https://github.com/tulpje/tulpje/commit/b92e9766acfafb5444bdbb591256170f36790723))
 * build(deps): bump reqwest from 0.12.9 to 0.12.15 ([`56172e7`](https://github.com/tulpje/tulpje/commit/56172e70a441f3b9ecff16c232fe8771ead40a96))
</details>

## [0.16.0] - 2026-10-04

### Breaking Changes

 - Use redis-rs directly instead of through bb8 pool

### Changed

 - Bump vergen-gitcl from 1.0.2 to 1.0.5
 - Bump redis from 0.28.2 to 0.29.1
 - Bump sqlx from 0.8.2 to 0.8.3
 - Bump uuid from 1.13.2 to 1.15.1
 - Bump ring from 0.17.8 to 0.17.13
 - Bump serde from 1.0.216 to 1.0.219
 - Bump uuid from 1.11.0 to 1.13.2
 - Bump async-trait from 0.1.83 to 0.1.86
 - Bump serde_json from 1.0.133 to 1.0.138
 - Bump metrics-exporter-prometheus from 0.16.0 to 0.16.2
 - Bump tokio from 1.42.0 to 1.43.0
 - Disable chrono wasmbind feature for our code
 - Enable dependabot

### Fixed

 - Update tokio-websockets to v0.11.3
 - Don't use a subshell while parsing .env into env vars

### Removed

 - Remove indirect dependency on aws-lc-rs

### Commit Details

<details><summary>view details</summary>

 * build(deps): bump vergen-gitcl from 1.0.2 to 1.0.5 ([`2468a18`](https://github.com/tulpje/tulpje/commit/2468a187b90d3dfba328a430ee09ebaea882b151))
 * build(deps): bump redis from 0.28.2 to 0.29.1 ([`a65610b`](https://github.com/tulpje/tulpje/commit/a65610b39b50e851ef96207d4c4da63f544f94f2))
 * build(deps): bump sqlx from 0.8.2 to 0.8.3 ([`fe0f4c7`](https://github.com/tulpje/tulpje/commit/fe0f4c74a4376febb8ac6640e30f7f158045482c))
 * build(deps): bump uuid from 1.13.2 to 1.15.1 ([`3a109cf`](https://github.com/tulpje/tulpje/commit/3a109cfabbe514953ce1cbac848ec6ef221c653f))
 * build(deps): bump ring from 0.17.8 to 0.17.13 ([`858ecbe`](https://github.com/tulpje/tulpje/commit/858ecbed10e873ae4562155fe33cd99be76999cf))
 * build(deps): bump serde from 1.0.216 to 1.0.219 ([`cc80138`](https://github.com/tulpje/tulpje/commit/cc8013804aac926c4104e61d4196c72c2ba3faa9))
 * build(deps): bump uuid from 1.11.0 to 1.13.2 ([`d5cd49f`](https://github.com/tulpje/tulpje/commit/d5cd49fa324bc2e9e0f9467fadcac0150aa4cafe))
 * build(deps): bump async-trait from 0.1.83 to 0.1.86 ([`f2d8e69`](https://github.com/tulpje/tulpje/commit/f2d8e692d9c43887e8a7b5c665f60d9fc33252e3))
 * build(deps): bump serde_json from 1.0.133 to 1.0.138 ([`78203bb`](https://github.com/tulpje/tulpje/commit/78203bb7396b63807cc1103dece994c06202ef03))
 * build(deps): bump metrics-exporter-prometheus from 0.16.0 to 0.16.2 ([`5e6cfb6`](https://github.com/tulpje/tulpje/commit/5e6cfb60419590f326a84b3991033ace9dec53f0))
 * build(deps): bump tokio from 1.42.0 to 1.43.0 ([`52b47ea`](https://github.com/tulpje/tulpje/commit/52b47eabe48162dced27a91a92eeef68ef6d160b))
 * chore: disable chrono wasmbind feature for our code ([`2f7c237`](https://github.com/tulpje/tulpje/commit/2f7c237f764ec32709d87580fa7c2f06f81d527d))
 * fix: update tokio-websockets to v0.11.3 ([`eace74f`](https://github.com/tulpje/tulpje/commit/eace74fb4f444948f14bd57aa23e3f5106a78050))
 * build: enable dependabot ([`2a21c5b`](https://github.com/tulpje/tulpje/commit/2a21c5bdd2f455bc85e1537da829f6fc4ab8a8be))
 * fix(build): don't use a subshell while parsing .env into env vars ([`deaa38d`](https://github.com/tulpje/tulpje/commit/deaa38d197a0f4a42e7ffd1a0c61ba20d23fbebc))
 * refactor!: use redis-rs directly instead of through bb8 pool ([`12add45`](https://github.com/tulpje/tulpje/commit/12add4574d435e2c8f86ba139502f042847e9111))
 * fix(shared): remove indirect dependency on aws-lc-rs ([`99d0541`](https://github.com/tulpje/tulpje/commit/99d05412d556c3a41539fec2feaa3db6b4a147d9))
</details>

## [0.15.0] - 2026-10-04

### Breaking Changes

 - Use env var for RUST_LOG instead of secret
 - Update twilight to 0.16.0

### Added

 - Add length limit to fronter category name

### Changed

 - Correctly tag independent crates without a release
 - Correctly detect tags for independent crates
 - Reduce log level for gateway messages to trace
 - Reduce amqp message logging level to trace
 - Use cache for checking if emojis belong to a guild
 - Implemented tulpje-cache, a redis based caching library

### Fixed

 - Update references to PluralKit command names in error messages

### Removed

 - Remove unnecessary env var expansion

### Commit Details

<details><summary>view details</summary>

 * build(release): correctly tag independent crates without a release ([`ae57012`](https://github.com/tulpje/tulpje/commit/ae57012822d137f1809aa5cefa802ce9c3604d5f))
 * build(release): correctly detect tags for independent crates ([`19a65d6`](https://github.com/tulpje/tulpje/commit/19a65d629356d6a7d7ba6da8f24953ae664ee0e2))
 * build(compose)!: use env var for RUST_LOG instead of secret ([`4dcc07e`](https://github.com/tulpje/tulpje/commit/4dcc07e904934683d02a4116f4b1b09b0cb21498))
 * build(compose): remove unnecessary env var expansion ([`ea4dd12`](https://github.com/tulpje/tulpje/commit/ea4dd1273c33b0c36919edfdf08c3cef4ae24c49))
 * fix(handler/pk): add length limit to fronter category name ([`6d08e5d`](https://github.com/tulpje/tulpje/commit/6d08e5d344ad98d251c73b8e16ced8598a8bfaa8))
 * fix(handler/pk): update references to PluralKit command names in error messages ([`dd8a89f`](https://github.com/tulpje/tulpje/commit/dd8a89f93d43e6efbaeb8c9ab5d93747de05804d))
 * chore(gateway): reduce log level for gateway messages to trace ([`1a39e98`](https://github.com/tulpje/tulpje/commit/1a39e98bbe7a416aac5b4bb4fa73cae2518539cc))
 * chore: reduce amqp message logging level to trace ([`513a1f7`](https://github.com/tulpje/tulpje/commit/513a1f775b8addd8bdc093e133cca9ca12edbceb))
 * feat(handler/emoji): use cache for checking if emojis belong to a guild ([`d6420f1`](https://github.com/tulpje/tulpje/commit/d6420f1f6b42069bd7e6f94e3def4c303ef8e503))
 * feat: implemented tulpje-cache, a redis based caching library ([`d11b12e`](https://github.com/tulpje/tulpje/commit/d11b12ed73a8fec4c80368e62e324a3b69536002))
 * chore!: update twilight to 0.16.0 ([`e4f10ea`](https://github.com/tulpje/tulpje/commit/e4f10eab776ab0a1529025ba7b8e9cb20c165153))
</details>

## [0.14.1] - 2026-10-04

### Changed

 - Clean up emoji stats on GuildCreate and GuildEmojisUpdate events

### Fixed

 - Validate the emoji stats embed
 - Don't show pagination/sorting when emoji stats are empty

### Removed

 - Remove emoji stats cleanup task, handled on event now

### Commit Details

<details><summary>view details</summary>

 * fix(handler): validate the emoji stats embed ([`dfd1645`](https://github.com/tulpje/tulpje/commit/dfd1645ce72cb1b8dba36f256211340c7681d90a))
 * fix(handler): don't show pagination/sorting when emoji stats are empty ([`21b657a`](https://github.com/tulpje/tulpje/commit/21b657adaf5a89fc6e0d0c92ae1f8dbe964514a9))
 * chore(handler): remove emoji stats cleanup task, handled on event now ([`8f8a76c`](https://github.com/tulpje/tulpje/commit/8f8a76cf0e3236a0f70b66daaffe28ba0756c4d3))
 * feat(handler): clean up emoji stats on GuildCreate and GuildEmojisUpdate events ([`12a646c`](https://github.com/tulpje/tulpje/commit/12a646c4ed3ccfd330c0b6545f320e072788f397))
</details>

## [0.14.0] - 2026-10-04

### Added

 - Added manual and automatic removal of emoji stats for deleted emojis
 - Add missing commas in RELEASE_FILENAME_MATCHLIST_WORKSPACE

### Changed

 - Implement pagination for `/emoji stats`
 - Split modules::stats into multiple files
 - Implement fallback for /stats when we can't get stats from redis
 - Split core module into multiple files

### Fixed

 - Always source .env from project root
 - Don't hardcode independent crates in RELEASE_FILENAME_MATCHLIST_WORKSPACE
 - Use latest main tag (vX.Y.Z) in push.sh

### Commit Details

<details><summary>view details</summary>

 * fix(build): always source .env from project root ([`d7ca4ba`](https://github.com/tulpje/tulpje/commit/d7ca4bae09197d476013112e59b8c790104246d2))
 * feat(handler): added manual and automatic removal of emoji stats for deleted emojis ([`b0f6904`](https://github.com/tulpje/tulpje/commit/b0f6904cadef61acd4b8102e0b0b1bcbff78e708))
 * feat(handler): implement pagination for `/emoji stats` ([`063e310`](https://github.com/tulpje/tulpje/commit/063e3107e36b633c7d568c458f12d1e7bfee3d2b))
 * refactor(handler): split modules::stats into multiple files ([`17cbe71`](https://github.com/tulpje/tulpje/commit/17cbe710da7209ae7bd149db4ef9db29bf45f5fa))
 * feat(handler): implement fallback for /stats when we can't get stats from redis ([`f472ebf`](https://github.com/tulpje/tulpje/commit/f472ebfb31a8b36adf8b6868c2982d1d987334f2))
 * refactor(handler): split core module into multiple files ([`0ccfabe`](https://github.com/tulpje/tulpje/commit/0ccfabecec2adf40f78d3ea2139269d02e709e3f))
 * fix(build): don't hardcode independent crates in RELEASE_FILENAME_MATCHLIST_WORKSPACE ([`4539228`](https://github.com/tulpje/tulpje/commit/4539228a6207cb408e8f2047554ba0e6cf7e826a))
 * fix(build): add missing commas in RELEASE_FILENAME_MATCHLIST_WORKSPACE ([`c83c106`](https://github.com/tulpje/tulpje/commit/c83c106749ff8d79a5e3b026a6d8f1e06ac157e7))
 * fix(build): use latest main tag (vX.Y.Z) in push.sh ([`d74caf3`](https://github.com/tulpje/tulpje/commit/d74caf3aca486d11796eacade37f085da51169a7))
</details>

## [0.13.0] - 2026-10-04

### Breaking Changes

 - Use subcommands and subcommand groups
 - Added support for subcommands and subcommand groups

### Changed

 - Specify GitHub release title

### Fixed

 - Reset minor/patch levels when bumping versions

### Commit Details

<details><summary>view details</summary>

 * fix(build): reset minor/patch levels when bumping versions ([`96bd60c`](https://github.com/tulpje/tulpje/commit/96bd60c9cce45568e392ef0ec95a70fdd25b0dfb))
 * feat(handler)!: use subcommands and subcommand groups ([`23ceadd`](https://github.com/tulpje/tulpje/commit/23ceadde444896be6f8784e0cb88542048f7e28e))
 * feat(framework)!: added support for subcommands and subcommand groups ([`175c77a`](https://github.com/tulpje/tulpje/commit/175c77a9e031c0fda73ccc2a566eac72d3cb4bbc))
 * build: specify GitHub release title ([`2652cde`](https://github.com/tulpje/tulpje/commit/2652cde710df4e47dbc2373a341bbff383722a23))
</details>

## [0.12.1] - 2026-10-04

### Changed

 - Don't use \`cross\` for compiling to x86_64-unknown-linux-musl

### Fixed

 - Exit when we receive an empty message from the shard
 - Fix should_release not being taken into account when releasing
 - Version bump didn't take `cargo semver-checks` into account
 - Create_changelog_update accepts Iterable[CrateInfo]

### Commit Details

<details><summary>view details</summary>

 * fix(gateway): exit when we receive an empty message from the shard ([`550098a`](https://github.com/tulpje/tulpje/commit/550098a64b2c63cbc9eb67b7a1a19cf382ecaea4))
 * build: don't use \`cross\` for compiling to x86_64-unknown-linux-musl ([`741f569`](https://github.com/tulpje/tulpje/commit/741f5696700721f680f503b2396754c8ea14776c))
 * fix(build): fix should_release not being taken into account when releasing ([`06fda35`](https://github.com/tulpje/tulpje/commit/06fda353c1e83d520ff602fdbaa77974e7eaf342))
 * fix(build): version bump didn't take `cargo semver-checks` into account ([`2187e40`](https://github.com/tulpje/tulpje/commit/2187e40e747fb4b0e127db234a65a1f60724830f))
 * fix(build): create_changelog_update accepts Iterable[CrateInfo] ([`bfe4a5d`](https://github.com/tulpje/tulpje/commit/bfe4a5d971b8a8c59b70bfb1115c9a0e9f308baf))
</details>

## [0.12.0] - 2026-10-04

### Breaking Changes

 - Move sqlx data to the tulpje-handler crate as they're part of that anyway

### Added

 - Add release tooling

### Fixed

 - Set a default for HANDLER_COUNT and don't override SHARD_COUNT from .env
 - Revert "don't clear target/release, unneeded after removal of amqp feature"
 - Fix crash when unable to parse gateway payload, log error instead
 - Use fork of pkrs that's actually published to crates.io

### Commit Details

<details><summary>view details</summary>

 * build: add release tooling ([`61ae5ae`](https://github.com/tulpje/tulpje/commit/61ae5ae8b3651f161b30eee425b7f010bd241b7c))
 * fix(build): set a default for HANDLER_COUNT and don't override SHARD_COUNT from .env ([`936aa2b`](https://github.com/tulpje/tulpje/commit/936aa2bdb5a2aafa0da8badc1933895bf2d638b0))
 * fix: revert "don't clear target/release, unneeded after removal of amqp feature" ([`0c0ae0c`](https://github.com/tulpje/tulpje/commit/0c0ae0c6a6e2dcf509e5f261f753ab20e7b0c861))
 * fix(handler): fix crash when unable to parse gateway payload, log error instead ([`bd95703`](https://github.com/tulpje/tulpje/commit/bd957031c7a34e5ed24ee5ad223caefbce42f090))
 * chore!: move sqlx data to the tulpje-handler crate as they're part of that anyway ([`3214d95`](https://github.com/tulpje/tulpje/commit/3214d95eb52bab36845c3ff02aed91be6f7c312e))
 * fix(handler): use fork of pkrs that's actually published to crates.io ([`103dc52`](https://github.com/tulpje/tulpje/commit/103dc5252a612a1fc8c7e35fb7df98b51dc61026))
</details>

## [0.11.0] - 2025-01-05

### Breaking Changes

 - Move DisordEventMeta to tulpje-framework and rename it Metadata
 - Rewrite the deploy and push scripts to use bash
 - Remove features to choose amqp implementation, just use amqprs

### Added

 - Add version constraints to workspace dependencies
 - Add missing package metadata

### Changed

 - Mark all crates as publishable
 - Don't make main() return Result, use .expect() to add info to errors
 - Implement additional metrics and show them in /processes
 - Implement version!() macro to get version from vergen env vars
 - Simplify MetricsManager and move the tokio::spawn call outside of it
 - Implement and use ToRedisArgs/FromRedisValue for ShardState and Metrics
 - Allow specifying image tag using IMAGE_TAG= in push.sh
 - Don't clear target/release, unneeded after removal of amqp feature

### Fixed

 - Don't rebuild if migrations change
 - Check mutually-exclusive features in build.rs

### Removed

 - Remove redlight from RUST_LOG in .example.env
 - Remove remnants of never implemented cache feature

### Commit Details

<details><summary>view details</summary>

 * chore: add version constraints to workspace dependencies ([`4d4c6f2`](https://github.com/tulpje/tulpje/commit/4d4c6f29f3f9ef0541462ff510db5f9ba47ddf49))
 * chore: mark all crates as publishable ([`c348f46`](https://github.com/tulpje/tulpje/commit/c348f46db387d6d70b50302d1b52bce8cbd37b58))
 * refactor: don't make main() return Result, use .expect() to add info to errors ([`371972e`](https://github.com/tulpje/tulpje/commit/371972e6eddfc29f7188b97420772ecb52bf6a01))
 * refactor!: move DisordEventMeta to tulpje-framework and rename it Metadata ([`0b52c83`](https://github.com/tulpje/tulpje/commit/0b52c837279db332f18c9af3fc4bac1a16167972))
 * fix: add missing package metadata ([`1a696c0`](https://github.com/tulpje/tulpje/commit/1a696c07dd05da3874f6ac1dbb60bc0df71f8128))
 * feat: implement additional metrics and show them in /processes ([`3dbdb99`](https://github.com/tulpje/tulpje/commit/3dbdb99906ac8ae4dfb93d8d881478a798a4acac))
 * feat(shared): implement version!() macro to get version from vergen env vars ([`b150a0d`](https://github.com/tulpje/tulpje/commit/b150a0df6f6723911c97ddcee1f3d055f668696b))
 * refactor(shared): simplify MetricsManager and move the tokio::spawn call outside of it ([`3ab74ac`](https://github.com/tulpje/tulpje/commit/3ab74acd488ca6e52a077e134120fc56f44d08b1))
 * feat(shared): implement and use ToRedisArgs/FromRedisValue for ShardState and Metrics ([`c03190f`](https://github.com/tulpje/tulpje/commit/c03190fc73cdbecfe39336be02463c8e0963fd87))
 * feat(build): allow specifying image tag using IMAGE_TAG= in push.sh ([`2d0c593`](https://github.com/tulpje/tulpje/commit/2d0c593d52380ed54aeec9ea045f4c7779f96cd8))
 * chore: remove redlight from RUST_LOG in .example.env ([`1c1db10`](https://github.com/tulpje/tulpje/commit/1c1db10fef0bd62ca6fc86daaf1d42e52b7fcd81))
 * chore(build)!: rewrite the deploy and push scripts to use bash ([`39341ff`](https://github.com/tulpje/tulpje/commit/39341ff0e131211d9ab884f33895458890acbf45))
 * build: don't clear target/release, unneeded after removal of amqp feature ([`4903c83`](https://github.com/tulpje/tulpje/commit/4903c83867f176bf0ab87ced97ee3edaa7c3cc8e))
 * fix(gateway): remove remnants of never implemented cache feature ([`2bafa74`](https://github.com/tulpje/tulpje/commit/2bafa745c42a08ea6955fd76278101574c51ef5e))
 * refactor!: remove features to choose amqp implementation, just use amqprs ([`2be4035`](https://github.com/tulpje/tulpje/commit/2be403523723f942653225c4a7e773f41430d864))
 * fix(gateway): don't rebuild if migrations change ([`ead5e70`](https://github.com/tulpje/tulpje/commit/ead5e708c5b7705e6077da762dcc830a59bc90fa))
 * fix(build): check mutually-exclusive features in build.rs ([`4d4c864`](https://github.com/tulpje/tulpje/commit/4d4c8645f99da12b33fc5b27a9136a8410fe32e3))
</details>

## [0.10.0] - 2025-01-05

### Fixed

 - Don't source SHARD_COUNT/HANDLER_COUNT from .env
 - Only run tasks on the "primary" handler (handler_id=0)
 - Only register commands on the "primary" handler (handler_id=0)
 - SHARD_ID and HANDLER_ID should actually be 0 not 1 by default
 - Better error messages in framework setup function

### Commit Details

<details><summary>view details</summary>

 * fix(dev): don't source SHARD_COUNT/HANDLER_COUNT from .env ([`2cebbfe`](https://github.com/tulpje/tulpje/commit/2cebbfea69f0bccc5299e1a42caa414529d7578d))
 * fix(handler): only run tasks on the "primary" handler (handler_id=0) ([`a71eb2f`](https://github.com/tulpje/tulpje/commit/a71eb2fc45f1fa3eda0da1b89be19d502a539b90))
 * fix(handler): only register commands on the "primary" handler (handler_id=0) ([`fc18b61`](https://github.com/tulpje/tulpje/commit/fc18b613d46aa891f81c57f0c147e41ef021663a))
 * fix(dev): SHARD_ID and HANDLER_ID should actually be 0 not 1 by default ([`bef7375`](https://github.com/tulpje/tulpje/commit/bef737588989ad50ec545dc74097993ffc5e85cb))
 * fix(handler): better error messages in framework setup function ([`8bc7d2a`](https://github.com/tulpje/tulpje/commit/8bc7d2a3cada27dedeccbd3d498fdcf55ba21f83))
</details>

## [0.9.0] - 2025-01-05

### Breaking Changes

 - Rework sending messages into framework, and refactor Scheduler to follow similar conventions to Dispatch

### Added

 - Add error message if .current_user_application() fails

### Fixed

 - Use .expect() instead of ? in main for extra error info
 - Set SHARD_ID and HANDLER_ID to 1 in run-local.sh
 - Don't fetch process metrics twice, and correctly calculate cpu/mem usage

### Commit Details

<details><summary>view details</summary>

 * fix(handler): use .expect() instead of ? in main for extra error info ([`293a5f8`](https://github.com/tulpje/tulpje/commit/293a5f8b32dd404157a3d65b34ba83c163f06d17))
 * feat(framework)!: rework sending messages into framework, and refactor Scheduler to follow similar conventions to Dispatch ([`b2ada33`](https://github.com/tulpje/tulpje/commit/b2ada33c3d0694092fc64c5017e7c9c1164045a9))
 * fix(dev): set SHARD_ID and HANDLER_ID to 1 in run-local.sh ([`b9f9fee`](https://github.com/tulpje/tulpje/commit/b9f9fee72f150f2ba4d526f4ea62be4ab5a9e431))
 * fix(handler): add error message if .current_user_application() fails ([`a54174c`](https://github.com/tulpje/tulpje/commit/a54174ca80022a6c760b2c349aa48a5fc25d0e4c))
 * fix(handler): don't fetch process metrics twice, and correctly calculate cpu/mem usage ([`9695c14`](https://github.com/tulpje/tulpje/commit/9695c14620a7cb20b5d10cd73dae062cb5fc8fca))
</details>

## [0.8.0] - 2025-01-04

### Breaking Changes

 - Implement framework with main loop and shutdown functionality

### Fixed

 - Allow configuring handler count in deploy script

### Commit Details

<details><summary>view details</summary>

 * fix(deploy): allow configuring handler count in deploy script ([`51f0d20`](https://github.com/tulpje/tulpje/commit/51f0d200a00daeb9e5d895615a5c65356f86a032))
 * refactor(framework)!: implement framework with main loop and shutdown functionality ([`395361e`](https://github.com/tulpje/tulpje/commit/395361e5be847a656fee2a76ac81ac1f1d253e19))
</details>

## [0.7.0] - 2025-01-04

### Added

 - Add explicit scaling support and store the handler count/id

### Changed

 - Implement /processes for bot process stats and re-implement cpu/mem usage for /stats
 - Implement basic memory/cpu usage tracking for bot processes
 - Enable clippy::missing_assert_message
 - Enable clippy::mod_module_files

### Fixed

 - SHARD_ID env var should be uppercase
 - Enable clippy::explicit_iter_loop clippy::explicit_into_iter_loop and fix warnings
 - Wrap Registry in an Arc to avoid expensive `clone` operations
 - Enable clippy::redundant_closure and fix warnings
 - Enable clippy::or_fun_call and fix warnings
 - Enable clippy::option_if_let_else and fix warnings
 - Enable clippy::if_then_some_else_none and fix warnings
 - Enable clippy::match_bool and fix warnings
 - Enable clippy::indexing_slicing and fix warnings
 - Enable clippy::manual_assert and fix warnings
 - Enable clippy::redundant_else and fix warnings
 - Enable clippy::partial_pub_fields and fix warnings
 - Enable clippy::unwrap_in_result and fix warnings
 - Enable clippy::cast_lossless and clippy::cast_precision_loss and fix warnings
 - Enable clippy::integer_division and fix warnings
 - Enable clippy::unneeded_field_pattern and fix warnings
 - Enable clippy::get_unwrap and fix warnings
 - Enable clippy::ignored_unit_patterns and fix warnings
 - Enable clippy::clone_on_ref_ptr and fix warnings
 - Enable clippy::needless_for_each and fix warnings
 - Enable clippy::redundant_clone and fix warnings
 - Enable clippy::renamed_function_params and fix warnings
 - Enable clippy::use_self and fix warnings
 - Enable clippy::needless_pass_by_value and fix warnings
 - Enable clippy::from_iter_instead_of_collect and fix warnings
 - Enable clippy::manual_string_new and fix warnings
 - Enable clippy::allow_attributes and fix warnings
 - Enable clippy::implicit_clone and fix warnings
 - Enable clippy::unused_trait_names and fix warnings
 - Use assert! instead of assert_eq! if checking for true/false
 - Enable clippy::semicolon_if_nothing_returned and fix warnings

### Removed

 - Remove outdated comment

### Commit Details

<details><summary>view details</summary>

 * feat(handler): implement /processes for bot process stats and re-implement cpu/mem usage for /stats ([`ebc1be6`](https://github.com/tulpje/tulpje/commit/ebc1be6ac4a8d1332806190eff4671bb334f6c5f))
 * feat: implement basic memory/cpu usage tracking for bot processes ([`ff62a76`](https://github.com/tulpje/tulpje/commit/ff62a76093760e5920a6e72974118bc536722062))
 * feat(handler): add explicit scaling support and store the handler count/id ([`47a51f0`](https://github.com/tulpje/tulpje/commit/47a51f093a8e7fa4a1cc6c8a88569a4ca8fe62a3))
 * fix(gateway): SHARD_ID env var should be uppercase ([`e00c3f9`](https://github.com/tulpje/tulpje/commit/e00c3f9291ce3cfeb181681ca77fc9188bd9d603))
 * fix(lint): enable clippy::explicit_iter_loop clippy::explicit_into_iter_loop and fix warnings ([`ef47d2b`](https://github.com/tulpje/tulpje/commit/ef47d2bd8b47105ddcf7d0cac8342efea64b6896))
 * fix(handler): wrap Registry in an Arc to avoid expensive `clone` operations ([`b73330b`](https://github.com/tulpje/tulpje/commit/b73330b1d818b17424dae5f2ba81e44b2bd8c39e))
 * chore: remove outdated comment ([`3b0e668`](https://github.com/tulpje/tulpje/commit/3b0e6685869a8d7650e31f405fd1c74837f9d77d))
 * fix(lint): enable clippy::redundant_closure and fix warnings ([`f95b528`](https://github.com/tulpje/tulpje/commit/f95b5283dc0f8c911f1b0cf60e7db73d602808ef))
 * fix(lint): enable clippy::or_fun_call and fix warnings ([`6835e9e`](https://github.com/tulpje/tulpje/commit/6835e9ef07bd852baf5ad0d16adb6b06e54c5488))
 * fix(lint): enable clippy::option_if_let_else and fix warnings ([`c2642eb`](https://github.com/tulpje/tulpje/commit/c2642eb655619cb211effd50de71fce9ecfc82a2))
 * fix(lint): enable clippy::if_then_some_else_none and fix warnings ([`eb1f119`](https://github.com/tulpje/tulpje/commit/eb1f119235ac8d0a8696cf997c57a563122fc134))
 * fix(lint): enable clippy::match_bool and fix warnings ([`97d2512`](https://github.com/tulpje/tulpje/commit/97d2512579f9d48783a738a0694c74daedeb7c93))
 * fix(lint): enable clippy::indexing_slicing and fix warnings ([`f8616a3`](https://github.com/tulpje/tulpje/commit/f8616a3b99743cd29ca4b738d10e752832a3b3fd))
 * chore(lint): enable clippy::missing_assert_message ([`cdd861d`](https://github.com/tulpje/tulpje/commit/cdd861d0fe51e23f57e67df6f4f5f524eee00eb0))
 * fix(lint): enable clippy::manual_assert and fix warnings ([`42c1913`](https://github.com/tulpje/tulpje/commit/42c19139534e842d95264b389eae30c42e1db29a))
 * fix(lint): enable clippy::redundant_else and fix warnings ([`26b270d`](https://github.com/tulpje/tulpje/commit/26b270dd8577fb20d18205def99a8e4e279f22bc))
 * fix(lint): enable clippy::partial_pub_fields and fix warnings ([`5a6b027`](https://github.com/tulpje/tulpje/commit/5a6b0279e4e5abee9270e8917dc4a54777b7236f))
 * fix(lint): enable clippy::unwrap_in_result and fix warnings ([`3d227fc`](https://github.com/tulpje/tulpje/commit/3d227fc02c4d8562457d783b8c3df8f271938787))
 * fix(lint): enable clippy::cast_lossless and clippy::cast_precision_loss and fix warnings ([`2267afc`](https://github.com/tulpje/tulpje/commit/2267afc8331c86d2cffa4b479d31b33dda187399))
 * fix(lint): enable clippy::integer_division and fix warnings ([`bff06f0`](https://github.com/tulpje/tulpje/commit/bff06f02bba4661c064ca6dc9309e5afc4b6d741))
 * fix(lint): enable clippy::unneeded_field_pattern and fix warnings ([`82cfc35`](https://github.com/tulpje/tulpje/commit/82cfc358edf6e46132ef4a5e696bc2e1b1945d80))
 * fix(lint): enable clippy::get_unwrap and fix warnings ([`741c3b1`](https://github.com/tulpje/tulpje/commit/741c3b1d655293ffe51d65858ed9d19bbd13f0f7))
 * fix(lint): enable clippy::ignored_unit_patterns and fix warnings ([`80d3d79`](https://github.com/tulpje/tulpje/commit/80d3d797fc6d0fcf685f143de2c1a633e328970c))
 * fix(lint): enable clippy::clone_on_ref_ptr and fix warnings ([`cc81e10`](https://github.com/tulpje/tulpje/commit/cc81e10285528b3b388c1395abd6b9df7559e6d9))
 * fix(lint): enable clippy::needless_for_each and fix warnings ([`bff5128`](https://github.com/tulpje/tulpje/commit/bff51285ae69387e4be2c46762fe33cc735f55b7))
 * fix(lint): enable clippy::redundant_clone and fix warnings ([`add7613`](https://github.com/tulpje/tulpje/commit/add7613e3b9853b2a1222ebaa0e242b74d5201c3))
 * chore(lint): enable clippy::mod_module_files ([`fd55d60`](https://github.com/tulpje/tulpje/commit/fd55d6041f82fa83c94090d4fb68859c6e4e8c16))
 * fix(lint): enable clippy::renamed_function_params and fix warnings ([`cbeff51`](https://github.com/tulpje/tulpje/commit/cbeff5186998cb0ab64f66213d1e8f25668daf5b))
 * fix(lint): enable clippy::use_self and fix warnings ([`7cd259c`](https://github.com/tulpje/tulpje/commit/7cd259c01257974bfce83a5fa3b8ead12a9e4382))
 * fix(lint): enable clippy::needless_pass_by_value and fix warnings ([`9450337`](https://github.com/tulpje/tulpje/commit/94503372fd008f5cfa59df230852a2fc334706f1))
 * fix(lint): enable clippy::from_iter_instead_of_collect and fix warnings ([`518e7f2`](https://github.com/tulpje/tulpje/commit/518e7f2cbebc7e7537e87aafce74d57c7cd7dfdc))
 * fix(lint): enable clippy::manual_string_new and fix warnings ([`456a341`](https://github.com/tulpje/tulpje/commit/456a34148176aa9a5c5349c970c465ace8fa4e63))
 * fix(lint): enable clippy::allow_attributes and fix warnings ([`d4e4c28`](https://github.com/tulpje/tulpje/commit/d4e4c2864e27759502201feb2ec8d3d9859d77b9))
 * fix(lint): enable clippy::implicit_clone and fix warnings ([`46ac7bf`](https://github.com/tulpje/tulpje/commit/46ac7bf67ca6365b3cd588a77a2ff48d3b93c3cd))
 * fix(lint): enable clippy::unused_trait_names and fix warnings ([`5ff5049`](https://github.com/tulpje/tulpje/commit/5ff504921e4c0d454e2d2411e751cfcc91d0b901))
 * fix(lint): use assert! instead of assert_eq! if checking for true/false ([`82fc0f6`](https://github.com/tulpje/tulpje/commit/82fc0f66b15c48e001a999cf452f122c12a83c35))
 * fix(lint): enable clippy::semicolon_if_nothing_returned and fix warnings ([`3cf919b`](https://github.com/tulpje/tulpje/commit/3cf919bc5b962e183e435394c796574d3771f452))
</details>

## [0.6.0] - 2025-01-03

### Changed

 - Don't hardcode guild module list
 - Rework module system, registry, and task scheduler
 - Cargo fmt

### Fixed

 - Don't update fronters for guilds that don't have the pluralkit module enabled

### Commit Details

<details><summary>view details</summary>

 * feat(handler): don't hardcode guild module list ([`9d1892c`](https://github.com/tulpje/tulpje/commit/9d1892ce74d25164f3cfe7e02e0ba8b3df3891ed))
 * fix(handler/pluralkit): don't update fronters for guilds that don't have the pluralkit module enabled ([`afb817c`](https://github.com/tulpje/tulpje/commit/afb817cf8b525521981fc68a3a93d08c13f334b8))
 * refactor(framework): rework module system, registry, and task scheduler ([`5bddf5b`](https://github.com/tulpje/tulpje/commit/5bddf5bb38be14622bdc777c31eb5c68637919a7))
 * chore: cargo fmt ([`8c75889`](https://github.com/tulpje/tulpje/commit/8c75889f5f737ff610d61a33bfb11f4854ddd137))
</details>

## [0.5.0] - 2025-01-03

### Breaking Changes

 - Don't pass context in constructor

### Changed

 - Per-guild commands

### Fixed

 - After defer we should use ctx.update
 - Actually send user errors back to the user

### Commit Details

<details><summary>view details</summary>

 * fix(handler/pk): after defer we should use ctx.update ([`1f13896`](https://github.com/tulpje/tulpje/commit/1f13896e3f596665ccf8ae3d7dac799bdeaf9dd5))
 * fix(handler/pk): actually send user errors back to the user ([`97005c6`](https://github.com/tulpje/tulpje/commit/97005c65332be179959ec9b51efd4f00222c9cdf))
 * feat: per-guild commands ([`b5de362`](https://github.com/tulpje/tulpje/commit/b5de36220153cc69a77d6189774d93f80ff050ef))
 * refactor(framework)!: don't pass context in constructor ([`3ad9713`](https://github.com/tulpje/tulpje/commit/3ad97134dd72ff3132a40e1bc81799162733134c))
</details>

## [0.4.2] - 2025-01-02

### Added

 - Add targets and feature permutations to contrib/check.sh
 - Add metrics to gateway and handler

### Changed

 - Implement emoji cloning on right-click menu
 - Use futures_util instead of futures
 - Use custom IdentifyProperties identifying as tulpje
 - Use Config::presence to set the presence, instead of manually on first ready
 - Update README with additional info

### Fixed

 - Guild IDs can't be 0, so dummy ID should be 1 at least
 - Initialise cache before redis connection to avoid borrow issue

### Removed

 - Remove Cargo.lock from crate, should only be in root

### Commit Details

<details><summary>view details</summary>

 * feat(handler): implement emoji cloning on right-click menu ([`587c047`](https://github.com/tulpje/tulpje/commit/587c047b797a0779dabbf0caf8f41d225e9723ea))
 * fix(handler): guild IDs can't be 0, so dummy ID should be 1 at least ([`5e9eb96`](https://github.com/tulpje/tulpje/commit/5e9eb962872cdae835a720c295622b588b0701be))
 * feat!(gateway): remove incomplete/broken cache feature ([`e05546b`](https://github.com/tulpje/tulpje/commit/e05546b85f913087287aee6492459f43926da9b1))
 * build: add targets and feature permutations to contrib/check.sh ([`09df3eb`](https://github.com/tulpje/tulpje/commit/09df3ebcbcadff674ab5208a14dffd8d2402e465))
 * fix(gateway): initialise cache before redis connection to avoid borrow issue ([`b2280ef`](https://github.com/tulpje/tulpje/commit/b2280ef925627430ff296f6e8d9f3c7b3c86e515))
 * feat: add metrics to gateway and handler ([`1f226a5`](https://github.com/tulpje/tulpje/commit/1f226a50a4817f5872bb9258eeb64d91ce9aa41d))
 * chore(handler): use futures_util instead of futures ([`e5996ee`](https://github.com/tulpje/tulpje/commit/e5996ee74b90a241c4b0d4eff95312bb1d7a248f))
 * feat: use custom IdentifyProperties identifying as tulpje ([`4b86638`](https://github.com/tulpje/tulpje/commit/4b86638834735f076efef280c8d59776de6f70f5))
 * feat: use Config::presence to set the presence, instead of manually on first ready ([`a12dc95`](https://github.com/tulpje/tulpje/commit/a12dc950807cd649deae2d222bf6b50ebc221bd4))
 * fix: remove Cargo.lock from crate, should only be in root ([`a9ce9ec`](https://github.com/tulpje/tulpje/commit/a9ce9ec97ffc3b4bc986d12fbabea56b5e4b13af))
 * chore: update README with additional info ([`096130d`](https://github.com/tulpje/tulpje/commit/096130d3dc111c57e653e653983cd239659eafd7))
</details>

## [0.4.1] - 2024-12-31

### Added

 - Added script to build and push images
 - Add RUSTSEC-2024-0384 to cargo-audit ignore list

### Changed

 - Script to run through a bunch of checks, useful before tag/deploy/etc
 - Features to pick between lapin and amqprs for amqp implementation
 - Update miniz_oxide to 0.8.2 version as 0.8.1 was yanked
 - Rename docker-compose.yml to compose.yml
 - Pass around twilight_model::id::Id instead of raw u64 values
 - We're just called Tulpje now

### Fixed

 - Ignore rsa advisory as we don't use it
 - Specify name in docker-compose.yml otherwise some scripts will break if folder has a different name

### Commit Details

<details><summary>view details</summary>

 * feat: added script to build and push images ([`1479b6d`](https://github.com/tulpje/tulpje/commit/1479b6d0f4861fa716d1af3eab84b5cde256c120))
 * feat: script to run through a bunch of checks, useful before tag/deploy/etc ([`f16e8e2`](https://github.com/tulpje/tulpje/commit/f16e8e21858f6a353c55bb68fdb478b8aa881d1a))
 * feat: features to pick between lapin and amqprs for amqp implementation ([`1cf397e`](https://github.com/tulpje/tulpje/commit/1cf397e90f113775403c010a65e88d6651695212))
 * chore: add RUSTSEC-2024-0384 to cargo-audit ignore list ([`fa687a3`](https://github.com/tulpje/tulpje/commit/fa687a32549995f26915ed7cc0cbb7d6ab5e671b))
 * fix: ignore rsa advisory as we don't use it ([`1e424c1`](https://github.com/tulpje/tulpje/commit/1e424c1453d0a71301fd5df9f11351fa52ec2539))
 * chore: update miniz_oxide to 0.8.2 version as 0.8.1 was yanked ([`b1df51a`](https://github.com/tulpje/tulpje/commit/b1df51a54ed5050cf4272776596569db9337e449))
 * chore: rename docker-compose.yml to compose.yml ([`7d426b1`](https://github.com/tulpje/tulpje/commit/7d426b19d05a09a4244fe5be27f25989fea12bc5))
 * feat: pass around twilight_model::id::Id instead of raw u64 values ([`45733bf`](https://github.com/tulpje/tulpje/commit/45733bf742b1e46376682090c2721af0fc30d336))
 * fix: specify name in docker-compose.yml otherwise some scripts will break if folder has a different name ([`9e6d783`](https://github.com/tulpje/tulpje/commit/9e6d7833ba0258df97773f6b887d1c2ea4aa5cea))
 * chore: we're just called Tulpje now ([`ce04bb2`](https://github.com/tulpje/tulpje/commit/ce04bb2beb7448673e27410b4268955a50c42f09))
</details>

## [0.4.0] - 2024-12-30

### Changed

 - PluralKit module
 - Task scheduling using cron syntax
 - Suppress clippy::single_match warning
 - Implement emoji cloning
 - Macros for making registering handlers slightly nicer
 - Implemented basic command and event handling framework

### Fixed

 - Don't specify a version for workspace packages, they're in sync anyway
 - Don't create a twilight_http::Client that we never use
 - Thread safetey ugh headaches

### Commit Details

<details><summary>view details</summary>

 * fix(handler): don't specify a version for workspace packages, they're in sync anyway ([`6c5aff4`](https://github.com/tulpje/tulpje/commit/6c5aff4c8b91f42e8472c83a10a73e45f38f7823))
 * feat(handler): PluralKit module ([`e2345fb`](https://github.com/tulpje/tulpje/commit/e2345fbcb4e7631eb02da2b980f5960b6db804af))
 * feat(framework): task scheduling using cron syntax ([`b3c11ec`](https://github.com/tulpje/tulpje/commit/b3c11ec50c667855f3b3034a68015db6b466e1f1))
 * fix(gateway): don't create a twilight_http::Client that we never use ([`46571fa`](https://github.com/tulpje/tulpje/commit/46571fa6b4ee793bb5d976ad496c6a3a2eefc478))
 * style(gateway): suppress clippy::single_match warning ([`4447262`](https://github.com/tulpje/tulpje/commit/44472620aea36fd296e1b95dce8a53e25436a03f))
 * fix: thread safetey ugh headaches ([`7c2f831`](https://github.com/tulpje/tulpje/commit/7c2f831d3b1480c516c1c716d1c4221d3e9970ac))
 * feat(handler): implement emoji cloning ([`ff52f17`](https://github.com/tulpje/tulpje/commit/ff52f1750e8bacdcada503f627027337a09730ae))
 * feat(framework): macros for making registering handlers slightly nicer ([`0ad6ee5`](https://github.com/tulpje/tulpje/commit/0ad6ee59d7f359c575fb467dba44c8e1a4f59397))
 * feat(framework): implemented basic command and event handling framework ([`4438e03`](https://github.com/tulpje/tulpje/commit/4438e0306c591c74e6597d4d79ef4c729d2af5b0))
</details>

## [0.3.0] - 2024-12-20

### Breaking Changes

 - Split stats commands into its own module

### Changed

 - Bump version to 0.3.0
 - Expand env vars when creating secrets
 - Implement emoji usage tracking and /emoji-stats
 - Update to twilight 0.16.0-rc.1

### Fixed

 - Clippy warnings

### Commit Details

<details><summary>view details</summary>

 * chore: bump version to 0.3.0 ([`dd8e838`](https://github.com/tulpje/tulpje/commit/dd8e83814f1533b16d9cd3d3e61dd5521cd24c4a))
 * feat(deploy): expand env vars when creating secrets ([`8898b1d`](https://github.com/tulpje/tulpje/commit/8898b1d50f23a60569651f8724165417698febf5))
 * feat(handler): implement emoji usage tracking and /emoji-stats ([`167d6ca`](https://github.com/tulpje/tulpje/commit/167d6ca8786af4caf4d00037d3fdb98cd652cef0))
 * chore(wip): update to twilight 0.16.0-rc.1 ([`689814a`](https://github.com/tulpje/tulpje/commit/689814a4d35b42227380b5a245149a18a8a4d1a5))
 * chore!: split stats commands into its own module ([`aa4eaf7`](https://github.com/tulpje/tulpje/commit/aa4eaf7f3ec570b4ed883ab2e7a4f68c051f44e8))
 * fix: clippy warnings ([`1178a64`](https://github.com/tulpje/tulpje/commit/1178a647670943c9a4acb57672983795cd6d0f4b))
</details>

## [0.2.0] - 2024-12-20

### Added

 - Add heartbeat_interval in ShardState, add ShardState::is_up() for better determining if shard is up
 - Add .env.example

### Changed

 - Bump version to 0.2.0
 - Inherit package version from workspace
 - Implement tests for ShardState::is_up()
 - Use ShardState::is_up() to display whether shards are up
 - Show per-shard guild count in /shards
 - Show guild count in /stats
 - Track guilds shard is in, and store count in ShardState
 - Implement /stats and /shards commands
 - Store shard state in redis
 - Implement docker swarm deployment
 - Contrib/run-local.sh utility that sets service IPs correctly for local dev (not in container)
 - Run in scratch containers
 - Store latency info in redis
 - Handle Ready event and setting presence correctly
 - Get shard_id/shard_count from env vars
 - Also track shard_id in DiscordEvent
 - Use serde_envfile and dotenvy for config and env parsing
 - Use upstream rabbitmq image
 - Implement using twilight-rs/gateway-queue for session rate limiting
 - Improve comments and logging
 - Use serde_envfile and dotenvy for config and env parsing
 - Implement rudimentary gateway and handler processes

### Fixed

 - Use base rabbitmq image and don't expose management port
 - Specify user in pg_isready in postgres healthcheck

### Removed

 - Remove unused import

### Commit Details

<details><summary>view details</summary>

 * chore: bump version to 0.2.0 ([`4da0d43`](https://github.com/tulpje/tulpje/commit/4da0d4322d8f76de3b6c1d141f5cd409d772869f))
 * chore: inherit package version from workspace ([`b9a454e`](https://github.com/tulpje/tulpje/commit/b9a454e47631af5ae279ceff7a0fe4a0e376e6fc))
 * feat(shared): implement tests for ShardState::is_up() ([`90cc570`](https://github.com/tulpje/tulpje/commit/90cc57039d4a506f15bb3146b299bbde431cad0e))
 * feat(handler): use ShardState::is_up() to display whether shards are up ([`621dcb4`](https://github.com/tulpje/tulpje/commit/621dcb47e128b5bb93a504ec9ed391a6046e7155))
 * feat(gateway): add heartbeat_interval in ShardState, add ShardState::is_up() for better determining if shard is up ([`6249261`](https://github.com/tulpje/tulpje/commit/62492611b82a8e1254a0af794448ff95c4410852))
 * feat(handler): show per-shard guild count in /shards ([`19a976f`](https://github.com/tulpje/tulpje/commit/19a976f2bde5d4414a02e3916cc3baf504700be4))
 * feat(handler): show guild count in /stats ([`15171ed`](https://github.com/tulpje/tulpje/commit/15171ed24837c239d1cf8290a3606b37c8dfb7ce))
 * feat(gateway): track guilds shard is in, and store count in ShardState ([`62f8dc0`](https://github.com/tulpje/tulpje/commit/62f8dc0ff5410e5898aa003be0dfc75ff85db366))
 * feat(handler): implement /stats and /shards commands ([`7413853`](https://github.com/tulpje/tulpje/commit/74138531435e7eda2da7c9281244a73e5e0372dd))
 * feat(gateway): store shard state in redis ([`ad93204`](https://github.com/tulpje/tulpje/commit/ad93204eea77963be56f1c803d9f269e6519a455))
 * chore: add .env.example ([`3bf09d8`](https://github.com/tulpje/tulpje/commit/3bf09d8d2dd8033d5428bfdd9ec0d61d7753cc1c))
 * feat: implement docker swarm deployment ([`3639e55`](https://github.com/tulpje/tulpje/commit/3639e557af99ead3e8c99fe24be3e1dde7939d5e))
 * feat: contrib/run-local.sh utility that sets service IPs correctly for local dev (not in container) ([`0d8b9f9`](https://github.com/tulpje/tulpje/commit/0d8b9f9158b4fb1ef5499772e52c512e922f1dc0))
 * feat(docker): run in scratch containers ([`5f64008`](https://github.com/tulpje/tulpje/commit/5f640081d5f5ebb2e7a7827edb5162270edc50e3))
 * fix(compose): use base rabbitmq image and don't expose management port ([`3557844`](https://github.com/tulpje/tulpje/commit/3557844267ad82ec80fa420fa6367f6c0cbcadc8))
 * fix(compose): specify user in pg_isready in postgres healthcheck ([`9e3909b`](https://github.com/tulpje/tulpje/commit/9e3909b086db77db8bb2cabdaf873df5d43dab1f))
 * feat(gateway): store latency info in redis ([`6e1a30f`](https://github.com/tulpje/tulpje/commit/6e1a30f75827be2f89b0f4231f1d9b131b350eef))
 * feat(gateway): handle Ready event and setting presence correctly ([`2e997b7`](https://github.com/tulpje/tulpje/commit/2e997b772c4f1f4df1bc0ef4efea776e5c43e4b3))
 * feat(gateway): get shard_id/shard_count from env vars ([`3f7fc3c`](https://github.com/tulpje/tulpje/commit/3f7fc3c704e6b4b5587d8e15bb15ffdb5c228308))
 * feat: also track shard_id in DiscordEvent ([`6255eed`](https://github.com/tulpje/tulpje/commit/6255eed11639190b184d6f8db8d37ce5076bb494))
 * feat(handler): use serde_envfile and dotenvy for config and env parsing ([`40db75b`](https://github.com/tulpje/tulpje/commit/40db75b1dda091a9ecc96a705a3bd13220f28b67))
 * chore: use upstream rabbitmq image ([`2105c76`](https://github.com/tulpje/tulpje/commit/2105c76703f09c7fb45867c3290645ccb770fdce))
 * feat(gateway): implement using twilight-rs/gateway-queue for session rate limiting ([`bdcfd1f`](https://github.com/tulpje/tulpje/commit/bdcfd1f4f98ec4b364a5d0095d310d5c86fd647f))
 * chore(gateway): improve comments and logging ([`9cfad73`](https://github.com/tulpje/tulpje/commit/9cfad73ce1dbdd7fde19b94a3b2bbe5090c24d80))
 * fix(gateway): remove unused import ([`a974f23`](https://github.com/tulpje/tulpje/commit/a974f2350c054560bd901a44fdd18bb427870633))
 * feat(gateway): use serde_envfile and dotenvy for config and env parsing ([`f92b12e`](https://github.com/tulpje/tulpje/commit/f92b12e16412b69b705b1a25489c643a4339a7df))
 * feat: implement rudimentary gateway and handler processes ([`bdebcab`](https://github.com/tulpje/tulpje/commit/bdebcaba1d8f9f85a6105775c10a676490319abe))
</details>
<!-- generated by git-cliff -->
