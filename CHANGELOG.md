# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Typed accessors on `Pack` for the ST 0601.19 packs and lists that `misb`
  decodes: `image_horizon`, `control_commands`, `verified_commands`,
  `active_wavelengths`, `country_codes`, `frame_rate`, `wavelengths`,
  `airbase_locations`, `payloads`, `active_payloads`, `waypoints` and
  `view_domain`. Each returns `None` for another tag or malformed bytes.
  Lists are iterators that do not allocate.
- `GeoPoint`, the WGS84 position shared by the ST 0601 and ST 0903 views.
  `St0903GeoPoint` is now an alias of it.
- `St0903Vmti::algorithms()`, `algorithm(id)` and `ontologies()` for the ST
  0903.6 Algorithm Series (item 102) and Ontology Series (item 103), with
  `St0903Algorithm` and `St0903Ontology` views. Malformed elements yield
  `St0903SeriesError`.

### Changed

- Decoding uses `misb` 0.5 (was 0.2). No change to this crate's API or output.
- `St0903VmtiItem` no longer has `AlgorithmSeries` or `OntologySeries`: items
  102 and 103 are typed now. Matches on the enum drop those arms.

## [0.1.0] - 2026-11-23

### Added

- `decode(tag, value)` returns a typed MISB ST 0601 item value: scaled numbers,
  integers, text, UTC instants as `jiff::Timestamp`, nested Local Sets with
  their standard named (typed ST 0102 Security and ST 0903 VMTI views for tags
  48 and 74), and packs/lists with their tag and raw value. Decoding is
  delegated to the `misb` crate; no `misb` type is public. Items that decoder
  leaves raw stay raw and are documented.
- `St0903Vmti::timestamp()` returns a `jiff::Timestamp`, matching
  `St0601Value::Time`; a value outside its range is `None`.
