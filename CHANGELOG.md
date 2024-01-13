# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added a changelog.
- Added the <code>fmt</code> make target.
- Added GitHub CI/CD workflows.
- Added GitLab CI/CD workflows.
- Added a TOC to the main <code>README.md</code> file.
- Added AlmaLinux prerequisites installation instructions.
- Added CentOS prerequisites installation instructions.
- Added Debian GNU/Linux prerequisites installation instructions.
- Added Gentoo Linux prerequisites installation instructions.
- Added RockyLinux prerequisites installation instructions.
- Added Ubuntu prerequisites installation instructions.
- Added a YouTube tutorials section to the main <code>README.md</code> file.

### Fixed

- Fixed a Microsoft Windows build failure caused by a typo in a variable name inside the GNU Make build script.
- Fixed FreeBSD prerequisites installation instructions.
- Some other minor build and documentation fixes.
- Code formatting fixes.

### Changed

- Version is now extracted from Git.
- The user name in the url now can either be set in plaintext or as a base64 string. The application takes care of which is which automatically.
- For unknown users we won't be showing the 'Hello {UNKNOWN_USER}!' prompt anymore and instead opt for a plain 'Hello!'.
- The index page has aesthetically been improved from a white/black foreground/background to an evil-hacker black/green terminal theme.
- Moved the license section from the main <code>README.md</code> file to a separate <code>LICENSE.md</code> file.
- Rust installation instructions.

## [0.1.0] - 2024-01-08

### Added

- Initial public release.