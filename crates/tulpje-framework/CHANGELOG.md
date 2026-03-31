# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [framework-v0.17.0-rc.1] - 2026-10-04

### Added

 - Add tracing spans to handlers
 - Add `twilight-standby` for waiting for events

### Changed

 - Also log event type when an event handler fails to run
 - Lower log level for informative messages
 - Implement `EventContext::from_context`

### Fixed

 - Don't error in `process_interaction` when `twilight-standby` processed one

### Commit Details

<details><summary>view details</summary>

 * feat(tulpje-framework): add tracing spans to handlers ([`1d0e0d6`](https://github.com/tulpje/tulpje/commit/1d0e0d6891599d21e394a0b50668e6649fb5c1b3))
 * chore(tulpje-framework): also log event type when an event handler fails to run ([`75289d8`](https://github.com/tulpje/tulpje/commit/75289d885d14016f957418c8a7af85ac38952a70))
 * chore(tulpje-framework): lower log level for informative messages ([`dadacb0`](https://github.com/tulpje/tulpje/commit/dadacb084d701f189a0437537853435ccecd94c7))
 * fix(tulpje-framework): don't error in `process_interaction` when `twilight-standby` processed one ([`2de03ab`](https://github.com/tulpje/tulpje/commit/2de03ab9192ae29e6d0233e41e04021c81f5a2b7))
 * refactor(framework): implement `EventContext::from_context` ([`8ac6848`](https://github.com/tulpje/tulpje/commit/8ac684857585bb956b0a7d01887d29fcbfda547a))
 * feat(tulpje-framework): add `twilight-standby` for waiting for events ([`a5330dd`](https://github.com/tulpje/tulpje/commit/a5330ddc76ac569262d6c7a15d1ee1b40ab11475))
</details>

## [framework-v0.16.1-rc.1] - 2026-10-04

### Added

 - Add `Deref` impl for `Color

### Changed

 - Use `wild` linker
 - Set `publish` in each crate's `Cargo.toml`
 - Use the Color deref impl
 - Move `Color` struct and role colors into `tulpje-framework`

### Fixed

 - Derive `Copy` and `Clone` for `Color
 - Don't bubble up command error if it's already handled in the command handler

### Commit Details

<details><summary>view details</summary>

 * chore(build): use `wild` linker ([`be4dd98`](https://github.com/tulpje/tulpje/commit/be4dd98c6090e3ac7ae0b0e24dac850f193031d5))
 * chore: set `publish` in each crate's `Cargo.toml` ([`e2d986c`](https://github.com/tulpje/tulpje/commit/e2d986c8f9560959dec509f114687dd3023e5c5f))
 * chore: use the Color deref impl ([`a8d76cb`](https://github.com/tulpje/tulpje/commit/a8d76cbe9013a8355a3baacf615a7a446d9c97ca))
 * feat(framework): add `Deref` impl for `Color ([`324518b`](https://github.com/tulpje/tulpje/commit/324518b1c4dda2d4375d55b2381ceb756b63e15f))
 * fix(framework): derive `Copy` and `Clone` for `Color ([`5aedef2`](https://github.com/tulpje/tulpje/commit/5aedef2ee36f3dee9ac49eda1ceeff67d1a0c4a8))
 * refactor: move `Color` struct and role colors into `tulpje-framework` ([`2c8cf4a`](https://github.com/tulpje/tulpje/commit/2c8cf4adf75ec1e49919be409115fbabecaeb412))
 * fix(framework): don't bubble up command error if it's already handled in the command handler ([`495c0ed`](https://github.com/tulpje/tulpje/commit/495c0ed207532c362cb0afa9cbe37be539e7c4a9))
</details>

## [framework-v0.16.0-rc.2] - 2026-10-04

### Fixed

 - Correctly use `Span`s with async functions

### Removed

 - Remove stray log statement

### Commit Details

<details><summary>view details</summary>

 * fix: correctly use `Span`s with async functions ([`1f61330`](https://github.com/tulpje/tulpje/commit/1f61330e614948c0874f8784680f4f1aed14aed0))
 * fix(framework): remove stray log statement ([`b600070`](https://github.com/tulpje/tulpje/commit/b600070087e77454b63dea734eb53ec03cceeef2))
</details>

## [framework-v0.16.0-rc.1] - 2026-10-04

### Added

 - Add tracing::span to event handling

### Changed

 - Configurable extra data in error message
 - Send an error reference to discord instead of the actual error message
 - Front change notifications
 - Switch to using `mod.rs` files

### Commit Details

<details><summary>view details</summary>

 * feat(framework): configurable extra data in error message ([`c38250d`](https://github.com/tulpje/tulpje/commit/c38250d08ebab423b9a4106209d6cdf12f607a2b))
 * feat(framework): send an error reference to discord instead of the actual error message ([`4f44e1b`](https://github.com/tulpje/tulpje/commit/4f44e1b08d7292783b17e6152dac4d96c164587a))
 * feat(framework): add tracing::span to event handling ([`4bfa125`](https://github.com/tulpje/tulpje/commit/4bfa125fa567b20115a0a2c09281c154333e9d9d))
 * feat(pluralkit): front change notifications ([`bd16e3d`](https://github.com/tulpje/tulpje/commit/bd16e3dc421311b85b68058fd9f7467c890ff284))
 * refactor: switch to using `mod.rs` files ([`e7614fa`](https://github.com/tulpje/tulpje/commit/e7614fa7ae6bed2ad1b5113d4d5efd3e351b9b15))
</details>

## [framework-v0.15.1] - 2026-10-04

### Changed

 - Send command errors to discord
 - Make `chrono` a workspace dependency
 - Make `tracing` and `tracing-subscriber` workspace dependencies
 - Make `serde` and `serde_json` workspace dependencies
 - Bump uuid from 1.18.1 to 1.19.0
 - Bump tracing from 0.1.41 to 0.1.43

### Commit Details

<details><summary>view details</summary>

 * feat(framework): send command errors to discord ([`5f3db99`](https://github.com/tulpje/tulpje/commit/5f3db99a34f888a0ba18fad0c19b32a3a95ba820))
 * chore(deps): make `chrono` a workspace dependency ([`9151b03`](https://github.com/tulpje/tulpje/commit/9151b03b0bd779f31a4c79042792cdff0182679c))
 * chore(deps): make `tracing` and `tracing-subscriber` workspace dependencies ([`234032f`](https://github.com/tulpje/tulpje/commit/234032f52f8e4016c537f3a4f60bc911a3c74b6a))
 * chore(deps): make `serde` and `serde_json` workspace dependencies ([`52f8711`](https://github.com/tulpje/tulpje/commit/52f8711b9f31a4edf8fbe3c4b5de9f84f55d309f))
 * chore(deps): bump uuid from 1.18.1 to 1.19.0 ([`80a14f3`](https://github.com/tulpje/tulpje/commit/80a14f33fab65dd6c7aff93af878a4d9ffa0d670))
 * chore(deps): bump tracing from 0.1.41 to 0.1.43 ([`d94fd01`](https://github.com/tulpje/tulpje/commit/d94fd0157b04ddcc536343ef2612c1890104d164))
</details>

## [framework-v0.15.0-rc.1] - 2026-10-04

### Changed

 - Make `tokio-util` a workspace dependency
 - Make `tokio` a workspace dependency
 - Configure tls correctly for 0.17
 - Bump `serde` from 1.0.219 to 1.0.228
 - Bump tokio from 1.44.2 to 1.47.1
 - Bump tokio-util from 0.7.14 to 0.7.16
 - Bump chrono from 0.4.40 to 0.4.42
 - Bump `uuid` from 0.16.0 to 0.18.1
 - Update to rust 1.90.0, fix lint warnings, `cargo fmt`
 - `clippy::collapsible_if`
 - `cargo fmt`
 - Specify edition on workspace level
 - Move twilight-* crates to workspace deps

### Fixed

 - Unbox modal interaction data
 - Fix large enum variant warning on `SchedulerTaskMessage::Enable`
 - Box large error variants
 - Hanging connections

### Removed

 - Remove tls feature flags that got removed in twilight 0.17

### Commit Details

<details><summary>view details</summary>

 * chore(deps): make `tokio-util` a workspace dependency ([`1665fbe`](https://github.com/tulpje/tulpje/commit/1665fbe97bd472c878daff8024660078a2cd961e))
 * chore(deps): make `tokio` a workspace dependency ([`894a678`](https://github.com/tulpje/tulpje/commit/894a678d0bdad4c1dee99bfd9fb64c2fc0b71013))
 * chore: configure tls correctly for 0.17 ([`2f8c177`](https://github.com/tulpje/tulpje/commit/2f8c17793e791b0b26f9acf936dfcd0bade2a931))
 * fix: remove tls feature flags that got removed in twilight 0.17 ([`ee69ff8`](https://github.com/tulpje/tulpje/commit/ee69ff8eb83304319482b74630b088f6ef0d05cc))
 * fix(framework): unbox modal interaction data ([`59d044e`](https://github.com/tulpje/tulpje/commit/59d044ea2400b6781686922248f53461f1e208c0))
 * chore(deps): bump `serde` from 1.0.219 to 1.0.228 ([`5511665`](https://github.com/tulpje/tulpje/commit/55116655b37504981043e098367cfd8df29e8691))
 * build(deps): bump tokio from 1.44.2 to 1.47.1 ([`ab3d857`](https://github.com/tulpje/tulpje/commit/ab3d8574b5a4dd5af4600b7beba9013119fd231f))
 * build(deps): bump tokio-util from 0.7.14 to 0.7.16 ([`fa43736`](https://github.com/tulpje/tulpje/commit/fa437366b1cd4604209ca991d2687cc460b65b1a))
 * build(deps): bump chrono from 0.4.40 to 0.4.42 ([`e15d587`](https://github.com/tulpje/tulpje/commit/e15d587bcdc426c41c903b5917c0873bc0d7f3b4))
 * chore(deps): bump `uuid` from 0.16.0 to 0.18.1 ([`62feb29`](https://github.com/tulpje/tulpje/commit/62feb298de8626a4ed5772997a490309de49be70))
 * chore(build): update to rust 1.90.0, fix lint warnings, `cargo fmt` ([`b850737`](https://github.com/tulpje/tulpje/commit/b8507378e832c0dee7bfedc74ebc72d3ad250235))
 * fix(lint): fix large enum variant warning on `SchedulerTaskMessage::Enable` ([`c62115b`](https://github.com/tulpje/tulpje/commit/c62115b2f6da0930eee8f2491f80905582d67732))
 * fix(lint): box large error variants ([`c8a5b4e`](https://github.com/tulpje/tulpje/commit/c8a5b4e992e544969cab7503efabce29eadb958c))
 * chore(lint): `clippy::collapsible_if` ([`3280fa2`](https://github.com/tulpje/tulpje/commit/3280fa2aff1e50057231438153466c6839807201))
 * chore: `cargo fmt` ([`a08aa81`](https://github.com/tulpje/tulpje/commit/a08aa8152bc0422ea2a1c3740cfd59a098e26e58))
 * refactor(build): specify edition on workspace level ([`751c335`](https://github.com/tulpje/tulpje/commit/751c335316d1e9b4440e98e7435fa38ed1ea8c09))
 * chore(deps): move twilight-* crates to workspace deps ([`d36e8d5`](https://github.com/tulpje/tulpje/commit/d36e8d565d66b066284387a721bc3cc013e5365d))
 * fix(gateway): hanging connections ([`5635aa6`](https://github.com/tulpje/tulpje/commit/5635aa6aa56de9cbecbafaefcaaeed0ef75fc0a2))
</details>

## [framework-v0.14.2] - 2026-10-04

### Changed

 - Bump tokio-util from 0.7.13 to 0.7.14
 - Bump chrono from 0.4.39 to 0.4.40
 - Bump uuid from 1.15.1 to 1.16.0

### Commit Details

<details><summary>view details</summary>

 * build(deps): bump tokio-util from 0.7.13 to 0.7.14 ([`02f275c`](https://github.com/tulpje/tulpje/commit/02f275cc0f23d67b8ea733ad5734433ba4e9fa60))
 * build(deps): bump chrono from 0.4.39 to 0.4.40 ([`dc8a0dd`](https://github.com/tulpje/tulpje/commit/dc8a0dd32f6ae05b609de9c5c7f13c7339778f29))
 * build(deps): bump uuid from 1.15.1 to 1.16.0 ([`01e5716`](https://github.com/tulpje/tulpje/commit/01e57160b2f973d121e5d1e175830f88571eb2de))
</details>

## [framework-v0.14.1] - 2026-10-04

### Changed

 - Bump uuid from 1.13.2 to 1.15.1
 - Bump serde from 1.0.216 to 1.0.219
 - Bump uuid from 1.11.0 to 1.13.2
 - Bump tokio from 1.42.0 to 1.43.0
 - Disable chrono wasmbind feature for our code

### Commit Details

<details><summary>view details</summary>

 * build(deps): bump uuid from 1.13.2 to 1.15.1 ([`3a109cf`](https://github.com/tulpje/tulpje/commit/3a109cfabbe514953ce1cbac848ec6ef221c653f))
 * build(deps): bump serde from 1.0.216 to 1.0.219 ([`cc80138`](https://github.com/tulpje/tulpje/commit/cc8013804aac926c4104e61d4196c72c2ba3faa9))
 * build(deps): bump uuid from 1.11.0 to 1.13.2 ([`d5cd49f`](https://github.com/tulpje/tulpje/commit/d5cd49fa324bc2e9e0f9467fadcac0150aa4cafe))
 * build(deps): bump tokio from 1.42.0 to 1.43.0 ([`52b47ea`](https://github.com/tulpje/tulpje/commit/52b47eabe48162dced27a91a92eeef68ef6d160b))
 * chore: disable chrono wasmbind feature for our code ([`2f7c237`](https://github.com/tulpje/tulpje/commit/2f7c237f764ec32709d87580fa7c2f06f81d527d))
</details>

## [framework-v0.14.0] - 2026-10-04

### Breaking Changes

 - Update twilight to 0.16.0

### Removed

 - Remove unnecessary logging of unhandled events

### Commit Details

<details><summary>view details</summary>

 * fix(framework): remove unnecessary logging of unhandled events ([`adb7e15`](https://github.com/tulpje/tulpje/commit/adb7e15463ba861b9cf821a93529c93f44dc2237))
 * chore!: update twilight to 0.16.0 ([`e4f10ea`](https://github.com/tulpje/tulpje/commit/e4f10eab776ab0a1529025ba7b8e9cb20c165153))
</details>

## [framework-v0.13.0] - 2026-10-04

### Breaking Changes

 - Added support for subcommands and subcommand groups

### Removed

 - Remove unused file module/module.rs

### Commit Details

<details><summary>view details</summary>

 * feat(framework)!: added support for subcommands and subcommand groups ([`175c77a`](https://github.com/tulpje/tulpje/commit/175c77a9e031c0fda73ccc2a566eac72d3cb4bbc))
 * chore: remove unused file module/module.rs ([`04c74f5`](https://github.com/tulpje/tulpje/commit/04c74f51c1cfd8314762ac4b73c6ca2b3a3e8d3a))
</details>

## [framework-v0.11.1] - 2026-10-04

### Breaking Changes

 - Move DisordEventMeta to tulpje-framework and rename it Metadata
 - Remove unused InteractionHandler trait
 - Rework sending messages into framework, and refactor Scheduler to follow similar conventions to Dispatch
 - Implement framework with main loop and shutdown functionality
 - Mark builder methods as #[must_use]
 - Don't pass context in constructor
 - Disallow adding tasks after starting scheduler

### Added

 - Add missing package metadata
 - Added `CommandContext::defer` helper method
 - Added `CommandContext::update` method to update the current interaction's message (after defer)
 - Added helper methods to get command options

### Changed

 - Version tulpje-framework separately from the bot
 - Mark all crates as publishable
 - Run `cargo fmt`
 - Refactor the scheduler so we can actually enable/disable tasks, even when the scheduler isn't running yet
 - Don't hardcode guild module list
 - Rework module system, registry, and task scheduler
 - Cargo fmt
 - Per-guild commands
 - PluralKit module
 - Task scheduling using cron syntax
 - Helper method to create CommandContext from base context
 - Macros for making registering handlers slightly nicer
 - Implemented basic command and event handling framework

### Fixed

 - Mark contexts/handlers as Sync + Send
 - Enable clippy::option_if_let_else and fix warnings
 - Enable clippy::manual_assert and fix warnings
 - Enable clippy::partial_pub_fields and fix warnings
 - Enable clippy::clone_on_ref_ptr and fix warnings
 - Enable clippy::redundant_clone and fix warnings
 - Enable clippy::needless_pass_by_value and fix warnings
 - Enable clippy::semicolon_if_nothing_returned and fix warnings
 - InteractionRegistry::get should not be &mut
 - Thread safetey ugh headaches

### Removed

 - Remove unused macros

### Commit Details

<details><summary>view details</summary>

 * chore: version tulpje-framework separately from the bot ([`445a87a`](https://github.com/tulpje/tulpje/commit/445a87ab0dcc685cbe3b394722cbebf5618e7a69))
 * chore: mark all crates as publishable ([`c348f46`](https://github.com/tulpje/tulpje/commit/c348f46db387d6d70b50302d1b52bce8cbd37b58))
 * refactor!: move DisordEventMeta to tulpje-framework and rename it Metadata ([`0b52c83`](https://github.com/tulpje/tulpje/commit/0b52c837279db332f18c9af3fc4bac1a16167972))
 * chore(framework)!: remove unused InteractionHandler trait ([`ec4266e`](https://github.com/tulpje/tulpje/commit/ec4266eea5f2035dbe4ab3c32c27aad8c61f4579))
 * fix: add missing package metadata ([`1a696c0`](https://github.com/tulpje/tulpje/commit/1a696c07dd05da3874f6ac1dbb60bc0df71f8128))
 * feat(framework)!: rework sending messages into framework, and refactor Scheduler to follow similar conventions to Dispatch ([`b2ada33`](https://github.com/tulpje/tulpje/commit/b2ada33c3d0694092fc64c5017e7c9c1164045a9))
 * refactor(framework)!: implement framework with main loop and shutdown functionality ([`395361e`](https://github.com/tulpje/tulpje/commit/395361e5be847a656fee2a76ac81ac1f1d253e19))
 * chore(framework): remove unused macros ([`0dde937`](https://github.com/tulpje/tulpje/commit/0dde937f8befcfb6c5f2bd89483e1a95d9673b62))
 * chore(style): run `cargo fmt` ([`6ddf81d`](https://github.com/tulpje/tulpje/commit/6ddf81d7692f574dce9f648e68510a9f1e9ec469))
 * fix(framework): mark contexts/handlers as Sync + Send ([`e052727`](https://github.com/tulpje/tulpje/commit/e052727c5e4a2c0e8030640f66e3166248e911c8))
 * fix(lint): enable clippy::option_if_let_else and fix warnings ([`c2642eb`](https://github.com/tulpje/tulpje/commit/c2642eb655619cb211effd50de71fce9ecfc82a2))
 * fix(lint): enable clippy::manual_assert and fix warnings ([`42c1913`](https://github.com/tulpje/tulpje/commit/42c19139534e842d95264b389eae30c42e1db29a))
 * fix(lint): enable clippy::partial_pub_fields and fix warnings ([`5a6b027`](https://github.com/tulpje/tulpje/commit/5a6b0279e4e5abee9270e8917dc4a54777b7236f))
 * fix(lint): enable clippy::clone_on_ref_ptr and fix warnings ([`cc81e10`](https://github.com/tulpje/tulpje/commit/cc81e10285528b3b388c1395abd6b9df7559e6d9))
 * feat(framework)!: mark builder methods as #[must_use] ([`f0c535a`](https://github.com/tulpje/tulpje/commit/f0c535ad9e3a3a53f28bcee2192ec827ad1926eb))
 * fix(lint): enable clippy::redundant_clone and fix warnings ([`add7613`](https://github.com/tulpje/tulpje/commit/add7613e3b9853b2a1222ebaa0e242b74d5201c3))
 * fix(lint): enable clippy::needless_pass_by_value and fix warnings ([`9450337`](https://github.com/tulpje/tulpje/commit/94503372fd008f5cfa59df230852a2fc334706f1))
 * fix(lint): enable clippy::semicolon_if_nothing_returned and fix warnings ([`3cf919b`](https://github.com/tulpje/tulpje/commit/3cf919bc5b962e183e435394c796574d3771f452))
 * refactor(scheduler): refactor the scheduler so we can actually enable/disable tasks, even when the scheduler isn't running yet ([`62c852a`](https://github.com/tulpje/tulpje/commit/62c852a4ab9f2be4d4c5819a404c67eacf4401a5))
 * feat(handler): don't hardcode guild module list ([`9d1892c`](https://github.com/tulpje/tulpje/commit/9d1892ce74d25164f3cfe7e02e0ba8b3df3891ed))
 * refactor(framework): rework module system, registry, and task scheduler ([`5bddf5b`](https://github.com/tulpje/tulpje/commit/5bddf5bb38be14622bdc777c31eb5c68637919a7))
 * chore: cargo fmt ([`8c75889`](https://github.com/tulpje/tulpje/commit/8c75889f5f737ff610d61a33bfb11f4854ddd137))
 * fix(framework): InteractionRegistry::get should not be &mut ([`90a1978`](https://github.com/tulpje/tulpje/commit/90a19785823f55386a1a6731ab7b1639af28a082))
 * feat: per-guild commands ([`b5de362`](https://github.com/tulpje/tulpje/commit/b5de36220153cc69a77d6189774d93f80ff050ef))
 * refactor(framework)!: don't pass context in constructor ([`3ad9713`](https://github.com/tulpje/tulpje/commit/3ad97134dd72ff3132a40e1bc81799162733134c))
 * feat(framework)!: disallow adding tasks after starting scheduler ([`4559347`](https://github.com/tulpje/tulpje/commit/455934756c14ae6b6459dafc5bad5fb7b49358d4))
 * feat(handler): PluralKit module ([`e2345fb`](https://github.com/tulpje/tulpje/commit/e2345fbcb4e7631eb02da2b980f5960b6db804af))
 * feat(framework): task scheduling using cron syntax ([`b3c11ec`](https://github.com/tulpje/tulpje/commit/b3c11ec50c667855f3b3034a68015db6b466e1f1))
 * fix: thread safetey ugh headaches ([`7c2f831`](https://github.com/tulpje/tulpje/commit/7c2f831d3b1480c516c1c716d1c4221d3e9970ac))
 * feat(framework): added `CommandContext::defer` helper method ([`1bfc43a`](https://github.com/tulpje/tulpje/commit/1bfc43aa7c04c67ed68d5d0d634293cf3a7da9bc))
 * feat(framework): added `CommandContext::update` method to update the current interaction's message (after defer) ([`56e2696`](https://github.com/tulpje/tulpje/commit/56e26960a6f420555b682336bc50bbd73b212632))
 * feat(framework): added helper methods to get command options ([`10db396`](https://github.com/tulpje/tulpje/commit/10db396651c225ad5bfa81b808ec76119a2ce55f))
 * feat(framework): helper method to create CommandContext from base context ([`8f01a15`](https://github.com/tulpje/tulpje/commit/8f01a15f6ae0f4a316d3631995349499132130a2))
 * feat(framework): macros for making registering handlers slightly nicer ([`0ad6ee5`](https://github.com/tulpje/tulpje/commit/0ad6ee59d7f359c575fb467dba44c8e1a4f59397))
 * feat(framework): implemented basic command and event handling framework ([`4438e03`](https://github.com/tulpje/tulpje/commit/4438e0306c591c74e6597d4d79ef4c729d2af5b0))
</details>
<!-- generated by git-cliff -->
