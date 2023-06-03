# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.0.5] 
### Added
- benchmarks (vs nom)

### Changed
- ParseError to ParsingError and now in prelude
- Cursor now in prelude

## [0.0.4] 
### Added
- parse_opt_with and parse_opt_selected
- cookbook ch_8_alternate_opts

## [0.0.3] 
### Added
- Pipe seperator on trace/log
- char(ch) matches on a single char
- introduced CHANGELOG.md

### Fixed
- license links in README

### Changed
- log target renamed to "dc" => use RUST_LOG=dc=trace for capturing logging 
- ParseError and Cursor are in module prelude.dc
- rename section_ to ch_ in cookbook

## [0.0.2] - 2023-05-18
Added documentation

## [0.0.1] - 2023-05-18
Initial release
