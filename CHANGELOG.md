# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `decode(tag, value)` returns a typed MISB ST 0601 item value: scaled numbers,
  integers, text, UTC instants in microseconds, nested Local Sets with their
  standard named (typed ST 0102 Security and ST 0903 VMTI views for tags 48 and
  74), and packs/lists with their tag and raw value. Decoding is delegated to
  the `misb` crate; no `misb` type is public. Items that decoder leaves raw stay
  raw and are documented (#4041).
