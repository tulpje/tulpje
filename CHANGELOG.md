# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


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

### Removed

 - Remove unused file module/module.rs

### Commit Details

<details><summary>view details</summary>

 * fix(build): reset minor/patch levels when bumping versions ([`96bd60c`](https://github.com/tulpje/tulpje/commit/96bd60c9cce45568e392ef0ec95a70fdd25b0dfb))
 * feat(handler)!: use subcommands and subcommand groups ([`23ceadd`](https://github.com/tulpje/tulpje/commit/23ceadde444896be6f8784e0cb88542048f7e28e))
 * feat(framework)!: added support for subcommands and subcommand groups ([`175c77a`](https://github.com/tulpje/tulpje/commit/175c77a9e031c0fda73ccc2a566eac72d3cb4bbc))
 * chore: remove unused file module/module.rs ([`04c74f5`](https://github.com/tulpje/tulpje/commit/04c74f51c1cfd8314762ac4b73c6ca2b3a3e8d3a))
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

### Changed

 - Version tulpje-framework separately from the bot

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
 * chore: version tulpje-framework separately from the bot ([`445a87a`](https://github.com/tulpje/tulpje/commit/445a87ab0dcc685cbe3b394722cbebf5618e7a69))
 * chore!: move sqlx data to the tulpje-handler crate as they're part of that anyway ([`3214d95`](https://github.com/tulpje/tulpje/commit/3214d95eb52bab36845c3ff02aed91be6f7c312e))
 * fix(handler): use fork of pkrs that's actually published to crates.io ([`103dc52`](https://github.com/tulpje/tulpje/commit/103dc5252a612a1fc8c7e35fb7df98b51dc61026))
</details>

## [0.11.0] - 2025-01-05

### Breaking Changes

 - Move DisordEventMeta to tulpje-framework and rename it Metadata
 - Remove unused InteractionHandler trait
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
 * chore(framework)!: remove unused InteractionHandler trait ([`ec4266e`](https://github.com/tulpje/tulpje/commit/ec4266eea5f2035dbe4ab3c32c27aad8c61f4579))
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

### Breaking Changes

 - Mark builder methods as #[must_use]

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
 - Mark contexts/handlers as Sync + Send
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
 * fix(framework): mark contexts/handlers as Sync + Send ([`e052727`](https://github.com/tulpje/tulpje/commit/e052727c5e4a2c0e8030640f66e3166248e911c8))
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
 * feat(framework)!: mark builder methods as #[must_use] ([`f0c535a`](https://github.com/tulpje/tulpje/commit/f0c535ad9e3a3a53f28bcee2192ec827ad1926eb))
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
 - Disallow adding tasks after starting scheduler

### Changed

 - Per-guild commands

### Fixed

 - After defer we should use ctx.update
 - Actually send user errors back to the user
 - InteractionRegistry::get should not be &mut

### Commit Details

<details><summary>view details</summary>

 * fix(handler/pk): after defer we should use ctx.update ([`1f13896`](https://github.com/tulpje/tulpje/commit/1f13896e3f596665ccf8ae3d7dac799bdeaf9dd5))
 * fix(handler/pk): actually send user errors back to the user ([`97005c6`](https://github.com/tulpje/tulpje/commit/97005c65332be179959ec9b51efd4f00222c9cdf))
 * fix(framework): InteractionRegistry::get should not be &mut ([`90a1978`](https://github.com/tulpje/tulpje/commit/90a19785823f55386a1a6731ab7b1639af28a082))
 * feat: per-guild commands ([`b5de362`](https://github.com/tulpje/tulpje/commit/b5de36220153cc69a77d6189774d93f80ff050ef))
 * refactor(framework)!: don't pass context in constructor ([`3ad9713`](https://github.com/tulpje/tulpje/commit/3ad97134dd72ff3132a40e1bc81799162733134c))
 * feat(framework)!: disallow adding tasks after starting scheduler ([`4559347`](https://github.com/tulpje/tulpje/commit/455934756c14ae6b6459dafc5bad5fb7b49358d4))
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

### Added

 - Added `CommandContext::defer` helper method
 - Added `CommandContext::update` method to update the current interaction's message (after defer)
 - Added helper methods to get command options

### Changed

 - PluralKit module
 - Task scheduling using cron syntax
 - Suppress clippy::single_match warning
 - Implement emoji cloning
 - Helper method to create CommandContext from base context
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
 * feat(framework): added `CommandContext::defer` helper method ([`1bfc43a`](https://github.com/tulpje/tulpje/commit/1bfc43aa7c04c67ed68d5d0d634293cf3a7da9bc))
 * feat(framework): added `CommandContext::update` method to update the current interaction's message (after defer) ([`56e2696`](https://github.com/tulpje/tulpje/commit/56e26960a6f420555b682336bc50bbd73b212632))
 * feat(framework): added helper methods to get command options ([`10db396`](https://github.com/tulpje/tulpje/commit/10db396651c225ad5bfa81b808ec76119a2ce55f))
 * feat(framework): helper method to create CommandContext from base context ([`8f01a15`](https://github.com/tulpje/tulpje/commit/8f01a15f6ae0f4a316d3631995349499132130a2))
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
