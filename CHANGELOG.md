# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.1](https://github.com/tulilirockz/juicerss/compare/v1.1.0...v1.1.1) - 2024-12-11

### Fixed

- *(deps)* update rust crate tokio-stream to v0.1.17
- *(deps)* update rust crate serde to v1.0.216
- *(deps)* update rust crate feed-rs to v2.3.0
- *(deps)* update rust crate clap to v4.5.23
- *(deps)* update rust crate clap to v4.5.22
- *(deps)* update rust crate tokio to v1.42.0
- *(deps)* update rust crate html2text to v0.13.5
- *(deps)* update rust crate html2text to v0.13.4
- *(deps)* update rust crate clap to v4.5.21
- *(deps)* update rust crate feed-rs to v2.2.0

### Other

- *(deps)* update softprops/action-gh-release action to v2.2.0
- *(deps)* update marcoieni/release-plz-action action to v0.5.85
- *(deps)* update marcoieni/release-plz-action action to v0.5.84
- bump spec file release

## [1.1.0](https://github.com/tulilirockz/juicerss/compare/v1.0.3...v1.1.0) - 2024-11-12

### Added

- alignment configuration for articles
- separate configuration into its own module and add scrolling factor + line configuration

### Fixed

- *(deps)* update rust crate serde to v1.0.215
- *(deps)* update rust crate html2text to v0.13.3
- *(deps)* update rust crate tokio to v1.41.1

### Other

- *(deps)* update softprops/action-gh-release action to v2.1.0
- *(deps)* update marcoieni/release-plz-action action to v0.5.83
- *(deps)* update softprops/action-gh-release action to v2.0.9
- initial installation instructions
- *(spec)* bump version to 1.0.3
- *(release-publish)* properly push tag
- *(release-publish)* make it so workflow_dispatch can specify tag
- publish artifacts on release

## [1.0.3](https://github.com/tulilirockz/juicerss/compare/v1.0.2...v1.0.3) - 2024-10-30

### Fixed

- *(ci)* release assets action

## [1.0.2](https://github.com/tulilirockz/juicerss/compare/v1.0.1...v1.0.2) - 2024-10-30

### Fixed

- *(spec)* use vendored dependencies for building

### Other

- release-publish workflow for binaries and vendor
- *(deps)* update actions/checkout action to v4

## [1.0.1](https://github.com/tulilirockz/juicerss/compare/v1.0.0...v1.0.1) - 2024-10-29

### Other

- release v1.0.0

## [1.0.0](https://github.com/tulilirockz/juicerss/releases/tag/v1.0.0) - 2024-10-29

### Other

- clippy pedantic fixes
- demo after description + configuring steps
- release workflow
- demo + format example
- first version + RPM
