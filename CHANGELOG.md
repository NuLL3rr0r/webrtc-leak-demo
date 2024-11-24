# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added a changelog.
- Added the `fmt-check` make target.
- Added GitHub CI/CD workflows.
- Added GitLab CI/CD workflows.
- Added a TOC to the main [`README.md`](README.md) file.
- Added AlmaLinux prerequisites installation instructions.
- Added CentOS prerequisites installation instructions.
- Added Debian GNU/Linux prerequisites installation instructions.
- Added Gentoo Linux prerequisites installation instructions.
- Added RockyLinux prerequisites installation instructions.
- Added Ubuntu prerequisites installation instructions.
- Updated the [`README.md`](README.md) file to include a new [YouTube Tutorials](README.md/#youtube-tutorials) section, providing direct links to both the English and Farsi tutorial videos.
- Introduced `DISABLE_VULNERABILITY_AUDIT` to allow users to explicitly disable vulnerability audits, providing a workaround for build failures caused by detected vulnerabilities.
- Introduced `RUST_VERSION_OVERRIDE` to enable users to explicitly specify a Rust version, addressing potential build failures with newer Rust versions.

### Fixed

- Fix a Microsoft Windows build failure caused by a typo in a variable name inside the GNU Make build script.
- Fix FreeBSD prerequisites installation instructions.
- Fix base64 deprectation warnings.
- Fix a username recording bug where any plain or base64-decoded usernames are reported as `{UNKNOWN_USER}`.
- Some other minor build and documentation fixes.
- Code formatting fixes.

### Changed

- Version is now extracted from Git.
- The user name in the url now can either be set in plaintext or as a base64 string. The application takes care of which is which automatically.
- For unknown users we won't be showing the 'Hello {UNKNOWN_USER}!' prompt anymore and instead opt for a plain 'Hello!'.
- The index page has aesthetically been improved from a white/black foreground/background to an evil-hacker black/green terminal theme.
- Moved the license section from the main [`README.md`](README.md) file to a separate [`LICENSE.md`](LICENSE.md) file.
- Rust installation instructions.
- Bumped actix-web to `v4.9.0`.
- Bumped base64 to `v0.22.1`.
- Bumped clap to `v4.5.21`.
- Bumped maxminddb to `v0.24.0`.
- Bumped parking_lot to `v0.12.3`.
- Bumped regex to `v1.11.1`.
- Bumped reqwest to `v0.12.9`.
- Bumped serde to `v1.0.215`.
- Bumped tempfile to `v3.14.0`.
- Bumped url to `v2.5.3`.

## [0.1.0] - 2024-01-08

### Added

- Initial public release.
