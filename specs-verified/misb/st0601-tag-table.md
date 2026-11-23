# MISB ST 0601.19 UAS Datalink Local Set — item (tag) table

> **Source.** MISB ST 0601.19 "UAS Datalink Local Set", Motion Imagery Standards Board,
> 02 March 2023 (revision history in §4 also records two later amendments of the same
> edition: 05/10/2024 Figure 17 corner-4 tag values, 06/11/2025 Item 42 MSL-or-HAE
> ambiguity). Vendored PDF, 226 pages:
> `specs/misb_st0601.19_2023_uas_datalink_local_set.pdf`
> (SHA-256 `e53c1e7bfdda888d5946610f89a8146a3f339394e1b127807302676c0cfb92b1`).
> Public US-Government document published by the NGA/NSG registry
> (`https://nsgreg.nga.mil/misb.jsp`). Obtained 2026-11-22 from the public mirror
> `https://media.githubusercontent.com/media/nitsuga/libmisbklv/main/references/ST0601.19.pdf`
> because `nsgreg.nga.mil` drops the TLS handshake from this network (TCP connects,
> then no ServerHello; `gwg.nga.mil` returns 403 for the document paths). The mirror
> file is byte-identical to the edition above (title page, §4 revision history and
> the 226-page pagination match).
>
> **How it was produced.** §7 Table 1 (pages 14–18) and §8 Items 1–143 (pages 35–225)
> were converted with
> `cd ~/Projects/pdf2md && uv run pdf2md convert <pdf> -o <md> --engine hybrid --pages 35-225 --report`
> and `--pages 14-18` (both exit 0, no verifier warnings). Where the converter merged
> table cells, the fields were re-read from the PDF text layer (pypdf) and from the
> printed per-item tables. §2 below is the normalised, machine-readable projection of
> that text: the rust-broadcast table-driven test in `transmux/tests/it/klv.rs`
> (`mod st0601_spec`) parses it at runtime and compares every tag's decoded kind and
> scaling against it.
>
> **Not covered here.** §6 Report-on-Change (SDCC/MUL) pack semantics, §7.2–§7.5
> geometry/guidance prose, the nested sets' own item tables (ST 0102, 0806, 0903,
> 1002, 1010, 1204, 1206, 1601, 1602, 1607) and the Requirements of §5. Only the
> Local Set item definitions (§7 Table 1 + §8) are transcribed.

## 1. Format of §2

Each row is one Local Set item:

- **Tag** — the BER-OID item key as printed; tags 1–127 are one byte, 128–143 two.
- **Name** — the §8 item heading.
- **Units** — the §8 `Units` cell in ASCII (`deg` = Degree (°), `us` = Microseconds (µs)).
- **Encoding** — how the *KLV value* is stored, projected from the §8 `Format` cell plus
  the item's mapping bullet:
  `utf8`, `uint`, `int` (unmapped integer), `linear-u` / `linear-s` (fixed linear
  mapping onto an unsigned / two's-complement integer, the item's "Map A..B to C..D"
  bullet), `imapb` (MISB ST 1201 IMAPB with an explicit `IMAPB(min,max,length)`),
  `time` (MISP time: µs since 1970-01-01, mapped as an unsigned integer),
  `security-set` / `vmti-set` (nested ST 0102 / ST 0903 Local Set),
  `set` / `pack` / `list` / `bytes` (nested Local Set of another standard, a
  defined-length pack (the §8 `dlp`), a variable-length pack/list (`vlp`, `list`),
  an opaque byte string), `deprecated` (§8 gives no value).
- **Bytes** — the KLV value length from the §8 `Length`/`Max Length` cells; `nominal`
  is the fixed length, `max` the largest allowed for a variable-length item.
- **Soft min** / **Soft max** — the software-value range: for `linear-*`/`imapb` the
  item's own `Software` Min/Max cells (the "Map … to …" target range), for
  `uint`/`int`/`time` the KLV integer range, `N/A` where the format has no range.
- **Offset** — the §8 `Offset` cell (applied after the linear mapping), `None`/`N/A`
  as printed.
- **Resolution** — the §8 `Resolution` cell, `~` kept.
- **Special values** — the §8 `Special Values` cell: the in-band sentinel(s) and
  their meaning. A sentinel is decoded as "out of range"/"N/A", not as a number.
- **Notes** — the standard a nested set follows, or the enumeration a code item uses.

## 2. §7 Table 1 + §8 items 1–143

| Tag | Name | Units | Encoding | Bytes | Soft min | Soft max | Offset | Resolution | Special values | Notes |
|---:|---|---|---|---|---|---|---|---|---|---|
| 1 | Checksum | N/A | uint | 2 | 0 | 65535 | N/A | N/A | None | 16-bit running sum of the whole packet |
| 2 | Precision Time Stamp | us | time | 8 | 0 | 18446744073709551615 | N/A | 1 microsecond | None | mandatory first item |
| 3 | Mission ID | N/A | utf8 | max 127 | N/A | N/A | N/A | N/A | N/A | free text |
| 4 | Platform Tail Number | N/A | utf8 | max 127 | N/A | N/A | N/A | N/A | N/A | free text |
| 5 | Platform Heading Angle | deg | linear-u | 2 | 0 | 360 | None | ~5.5 millidegrees | None | relative to True North |
| 6 | Platform Pitch Angle | deg | linear-s | 2 | -20 | 20 | None | ~610 microdegrees | 0x8000 = "Out of Range" | range-restricted variant of Item 90 |
| 7 | Platform Roll Angle | deg | linear-s | 2 | -50 | 50 | None | ~1525 microdegrees | 0x8000 = "Out of Range" | range-restricted variant of Item 91 |
| 8 | Platform True Airspeed | m/s | uint | 1 | 0 | 255 | None | 1 meter/second | None | |
| 9 | Platform Indicated Airspeed | m/s | uint | 1 | 0 | 255 | None | 1 meter/second | None | |
| 10 | Platform Designation | N/A | utf8 | max 127 | N/A | N/A | N/A | N/A | N/A | free text |
| 11 | Image Source Sensor | N/A | utf8 | max 127 | N/A | N/A | N/A | N/A | N/A | free text |
| 12 | Image Coordinate System | N/A | utf8 | max 127 | N/A | N/A | N/A | N/A | N/A | e.g. "Geodetic WGS84" |
| 13 | Sensor Latitude | deg | linear-s | 4 | -90 | 90 | None | ~42 nanodegrees | 0x80000000 = "Reserved" | WGS84 |
| 14 | Sensor Longitude | deg | linear-s | 4 | -180 | 180 | None | ~84 nanodegrees | 0x80000000 = "Reserved" | WGS84 |
| 15 | Sensor True Altitude | m | linear-u | 2 | -900 | 19000 | -900 | ~0.3 meters | None | MSL |
| 16 | Sensor Horizontal Field of View | deg | linear-u | 2 | 0 | 180 | None | ~2.7 millidegrees | None | |
| 17 | Sensor Vertical Field of View | deg | linear-u | 2 | 0 | 180 | None | ~2.7 millidegrees | None | |
| 18 | Sensor Relative Azimuth Angle | deg | linear-u | 4 | 0 | 360 | None | ~84 nanodegrees | None | |
| 19 | Sensor Relative Elevation Angle | deg | linear-s | 4 | -180 | 180 | None | ~84 nanodegrees | 0x80000000 = "Reserved" | |
| 20 | Sensor Relative Roll Angle | deg | linear-u | 4 | 0 | 360 | None | ~84 nanodegrees | None | |
| 21 | Slant Range | m | linear-u | 4 | 0 | 5000000 | None | ~1.2 millimeters | None | |
| 22 | Target Width | m | linear-u | 2 | 0 | 10000 | None | ~0.16 meters | None | |
| 23 | Frame Center Latitude | deg | linear-s | 4 | -90 | 90 | None | ~42 nanodegrees | 0x80000000 = "N/A (Off-Earth)" | WGS84 |
| 24 | Frame Center Longitude | deg | linear-s | 4 | -180 | 180 | None | ~84 nanodegrees | 0x80000000 = "N/A (Off-Earth)" | WGS84 |
| 25 | Frame Center Elevation | m | linear-u | 2 | -900 | 19000 | -900 | ~0.3 meters | None | MSL |
| 26 | Offset Corner Latitude Point 1 | deg | linear-s | 2 | -0.075 | 0.075 | None | ~1.2 microdegrees | 0x8000 = "N/A (Off-Earth)" | used with Item 23 |
| 27 | Offset Corner Longitude Point 1 | deg | linear-s | 2 | -0.075 | 0.075 | None | ~1.2 microdegrees | 0x8000 = "N/A (Off-Earth)" | used with Item 24 |
| 28 | Offset Corner Latitude Point 2 | deg | linear-s | 2 | -0.075 | 0.075 | None | ~1.2 microdegrees | 0x8000 = "N/A (Off-Earth)" | used with Item 23 |
| 29 | Offset Corner Longitude Point 2 | deg | linear-s | 2 | -0.075 | 0.075 | None | ~1.2 microdegrees | 0x8000 = "N/A (Off-Earth)" | used with Item 24 |
| 30 | Offset Corner Latitude Point 3 | deg | linear-s | 2 | -0.075 | 0.075 | None | ~1.2 microdegrees | 0x8000 = "N/A (Off-Earth)" | used with Item 23 |
| 31 | Offset Corner Longitude Point 3 | deg | linear-s | 2 | -0.075 | 0.075 | None | ~1.2 microdegrees | 0x8000 = "N/A (Off-Earth)" | used with Item 24 |
| 32 | Offset Corner Latitude Point 4 | deg | linear-s | 2 | -0.075 | 0.075 | None | ~1.2 microdegrees | 0x8000 = "N/A (Off-Earth)" | used with Item 23 |
| 33 | Offset Corner Longitude Point 4 | deg | linear-s | 2 | -0.075 | 0.075 | None | ~1.2 microdegrees | 0x8000 = "N/A (Off-Earth)" | used with Item 24 |
| 34 | Icing Detected | code | uint | 1 | 0 | 2 | None | N/A | None | 0 off, 1 no icing, 2 icing |
| 35 | Wind Direction | deg | linear-u | 2 | 0 | 360 | None | ~5.5 millidegrees | None | |
| 36 | Wind Speed | m/s | linear-u | 1 | 0 | 100 | None | ~0.4 meters/second | None | |
| 37 | Static Pressure | mbar | linear-u | 2 | 0 | 5000 | None | ~0.08 millibar | None | |
| 38 | Density Altitude | m | linear-u | 2 | -900 | 19000 | -900 | ~0.3 meters | None | |
| 39 | Outside Air Temperature | C | int | 1 | -128 | 127 | None | 1 degree Celsius | None | |
| 40 | Target Location Latitude | deg | linear-s | 4 | -90 | 90 | None | ~42 nanodegrees | 0x80000000 = "N/A (Off-Earth)" | WGS84 |
| 41 | Target Location Longitude | deg | linear-s | 4 | -180 | 180 | None | ~84 nanodegrees | 0x80000000 = "N/A (Off-Earth)" | WGS84 |
| 42 | Target Location Elevation | m | linear-u | 2 | -900 | 19000 | -900 | ~0.3 meters | None | MSL or HAE (2025 amendment) |
| 43 | Target Track Gate Width | pixels | linear-u | 1 | 0 | 510 | None | 2 pixels | None | |
| 44 | Target Track Gate Height | pixels | linear-u | 1 | 0 | 510 | None | 2 pixels | None | |
| 45 | Target Error Estimate - CE90 | m | linear-u | 2 | 0 | 4095 | None | ~0.0624 meters | None | |
| 46 | Target Error Estimate - LE90 | m | linear-u | 2 | 0 | 4095 | None | 0.0625 meters | None | |
| 47 | Generic Flag Data | N/A | uint | 1 | 0 | 63 | N/A | N/A | None | bit flags, bits 0-5 (Table 3) |
| 48 | Security Local Set | N/A | security-set | max unlimited | N/A | N/A | N/A | N/A | N/A | MISB ST 0102 Local Set |
| 49 | Differential Pressure | mbar | linear-u | 2 | 0 | 5000 | None | ~0.08 millibar | None | |
| 50 | Platform Angle of Attack | deg | linear-s | 2 | -20 | 20 | None | ~610 microdegrees | 0x8000 = "Out of Range" | |
| 51 | Platform Vertical Speed | m/s | linear-s | 2 | -180 | 180 | None | ~0.0055 meters/second | 0x8000 = "Out of Range" | |
| 52 | Platform Sideslip Angle | deg | linear-s | 2 | -20 | 20 | None | ~610 microdegrees | 0x8000 = "Out of Range" | |
| 53 | Airfield Barometric Pressure | mbar | linear-u | 2 | 0 | 5000 | None | ~0.08 millibar | None | |
| 54 | Airfield Elevation | m | linear-u | 2 | -900 | 19000 | -900 | ~0.3 meters | None | |
| 55 | Relative Humidity | % | linear-u | 1 | 0 | 100 | None | ~0.4 % | None | |
| 56 | Platform Ground Speed | m/s | uint | 1 | 0 | 255 | None | 1 meter/second | None | |
| 57 | Ground Range | m | linear-u | 4 | 0 | 5000000 | None | ~1.2 millimeters | None | |
| 58 | Platform Fuel Remaining | kg | linear-u | 2 | 0 | 10000 | None | ~0.16 kilograms | None | |
| 59 | Platform Call Sign | N/A | utf8 | max 127 | N/A | N/A | N/A | N/A | N/A | free text |
| 60 | Weapon Load | N/A | pack | 2 | 0 | 65535 | N/A | N/A | None | nibble pack: station/substation/type/variant |
| 61 | Weapon Fired | N/A | pack | 1 | 0 | 255 | N/A | N/A | None | nibble pack: station/substation |
| 62 | Laser PRF Code | N/A | uint | 2 | 0 | 65535 | None | N/A | None | digits 1-8 only (111..8888) |
| 63 | Sensor Field of View Name | N/A | uint | 1 | 0 | 7 | None | N/A | None | enumeration 0-7 |
| 64 | Platform Magnetic Heading | deg | linear-u | 2 | 0 | 360 | None | ~5.5 millidegrees | None | |
| 65 | UAS Datalink LS Version Number | N/A | uint | 1 | 0 | 255 | None | N/A | None | mandatory; 0 pre-release, N = ST 0601.N |
| 66 | Deprecated (Target Location Covariance Matrix) | N/A | deprecated | N/A | N/A | N/A | N/A | N/A | N/A | superseded by Item 102 |
| 67 | Alternate Platform Latitude | deg | linear-s | 4 | -90 | 90 | None | ~42 nanodegrees | 0x80000000 = "Reserved" | WGS84 |
| 68 | Alternate Platform Longitude | deg | linear-s | 4 | -180 | 180 | None | ~84 nanodegrees | 0x80000000 = "Reserved" | WGS84 |
| 69 | Alternate Platform Altitude | m | linear-u | 2 | -900 | 19000 | -900 | ~0.3 meters | None | |
| 70 | Alternate Platform Name | N/A | utf8 | max 127 | N/A | N/A | N/A | N/A | N/A | free text |
| 71 | Alternate Platform Heading | deg | linear-u | 2 | 0 | 360 | None | ~5.5 millidegrees | None | |
| 72 | Event Start Time - UTC | us | time | 8 | 0 | 18446744073709551615 | N/A | 1 microsecond | None | |
| 73 | RVT Local Set | N/A | set | max unlimited | N/A | N/A | N/A | N/A | N/A | MISB ST 0806 Local Set |
| 74 | VMTI Local Set | N/A | vmti-set | max unlimited | N/A | N/A | N/A | N/A | N/A | MISB ST 0903 Local Set |
| 75 | Sensor Ellipsoid Height | m | linear-u | 2 | -900 | 19000 | -900 | ~0.3 meters | None | WGS84 |
| 76 | Alternate Platform Ellipsoid Height | m | linear-u | 2 | -900 | 19000 | -900 | ~0.3 meters | None | WGS84 |
| 77 | Operational Mode | N/A | uint | 1 | 0 | 5 | N/A | N/A | None | enumeration 0-5 |
| 78 | Frame Center Height Above Ellipsoid | m | linear-u | 2 | -900 | 19000 | -900 | ~0.3 meters | None | WGS84 |
| 79 | Sensor North Velocity | m/s | linear-s | 2 | -327 | 327 | None | ~1 cm/sec | 0x8000 = "Out of Range" | |
| 80 | Sensor East Velocity | m/s | linear-s | 2 | -327 | 327 | None | ~1 cm/sec | 0x8000 = "Out of Range" | |
| 81 | Image Horizon Pixel Pack | N/A | pack | max 20 | N/A | N/A | N/A | N/A | N/A | defined-length pack (x0,y0,x1,y1 + optional lat/lon) |
| 82 | Corner Latitude Point 1 (Full) | deg | linear-s | 4 | -90 | 90 | None | ~42 nanodegrees | 0x80000000 = "N/A (Off-Earth)" | WGS84 |
| 83 | Corner Longitude Point 1 (Full) | deg | linear-s | 4 | -180 | 180 | None | ~84 nanodegrees | 0x80000000 = "N/A (Off-Earth)" | WGS84 |
| 84 | Corner Latitude Point 2 (Full) | deg | linear-s | 4 | -90 | 90 | None | ~42 nanodegrees | 0x80000000 = "N/A (Off-Earth)" | WGS84 |
| 85 | Corner Longitude Point 2 (Full) | deg | linear-s | 4 | -180 | 180 | None | ~84 nanodegrees | 0x80000000 = "N/A (Off-Earth)" | WGS84 |
| 86 | Corner Latitude Point 3 (Full) | deg | linear-s | 4 | -90 | 90 | None | ~42 nanodegrees | 0x80000000 = "N/A (Off-Earth)" | WGS84 |
| 87 | Corner Longitude Point 3 (Full) | deg | linear-s | 4 | -180 | 180 | None | ~84 nanodegrees | 0x80000000 = "N/A (Off-Earth)" | WGS84 |
| 88 | Corner Latitude Point 4 (Full) | deg | linear-s | 4 | -90 | 90 | None | ~42 nanodegrees | 0x80000000 = "N/A (Off-Earth)" | WGS84 |
| 89 | Corner Longitude Point 4 (Full) | deg | linear-s | 4 | -180 | 180 | None | ~84 nanodegrees | 0x80000000 = "N/A (Off-Earth)" | WGS84 |
| 90 | Platform Pitch Angle (Full) | deg | linear-s | 4 | -90 | 90 | None | ~42 nanodegrees | 0x80000000 = "Out of Range" | full-range variant of Item 6 |
| 91 | Platform Roll Angle (Full) | deg | linear-s | 4 | -90 | 90 | None | ~42 nanodegrees | 0x80000000 = "Out of Range" | full-range variant of Item 7 |
| 92 | Platform Angle of Attack (Full) | deg | linear-s | 4 | -90 | 90 | None | ~42 nanodegrees | 0x80000000 = "Out of Range" | full-range variant of Item 50 |
| 93 | Platform Sideslip Angle (Full) | deg | linear-s | 4 | -180 | 180 | None | ~84 nanodegrees | 0x80000000 = "Out of Range" | full-range variant of Item 52 |
| 94 | MIIS Core Identifier | N/A | bytes | max 50 | N/A | N/A | N/A | N/A | N/A | MISB ST 1204 binary Core ID |
| 95 | SAR Motion Imagery Local Set | N/A | set | max unlimited | N/A | N/A | N/A | N/A | N/A | MISB ST 1206 Local Set |
| 96 | Target Width Extended | m | imapb | max 8 | 0 | 1500000 | N/A | 3 bytes = 0.25 meters | None | IMAPB(0,1500000,length) |
| 97 | Range Image Local Set | N/A | set | max unlimited | N/A | N/A | N/A | N/A | N/A | MISB ST 1002 Local Set |
| 98 | Geo-Registration Local Set | N/A | set | max unlimited | N/A | N/A | N/A | N/A | N/A | MISB ST 1601 Local Set |
| 99 | Composite Imaging Local Set | N/A | set | max unlimited | N/A | N/A | N/A | N/A | N/A | MISB ST 1602 Local Set |
| 100 | Segment Local Set | N/A | set | max unlimited | N/A | N/A | N/A | N/A | N/A | MISB ST 1607 Segment Local Set |
| 101 | Amend Local Set | N/A | set | max unlimited | N/A | N/A | N/A | N/A | N/A | MISB ST 1607 Amend Local Set |
| 102 | SDCC-FLP | N/A | pack | max unlimited | N/A | N/A | N/A | N/A | N/A | MISB ST 1010 floating length pack |
| 103 | Density Altitude Extended | m | imapb | max 8 | -900 | 40000 | N/A | 3 bytes = 78.125 mm | None | IMAPB(-900,40000,length) |
| 104 | Sensor Ellipsoid Height Extended | m | imapb | max 8 | -900 | 40000 | N/A | 3 bytes = 78.125 mm | None | IMAPB(-900,40000,length) |
| 105 | Alternate Platform Ellipsoid Height Extended | m | imapb | max 8 | -900 | 40000 | N/A | 3 bytes = 78.125 mm | None | IMAPB(-900,40000,length) |
| 106 | Stream Designator | N/A | utf8 | max 127 | N/A | N/A | N/A | N/A | N/A | free text |
| 107 | Operational Base | N/A | utf8 | max 127 | N/A | N/A | N/A | N/A | N/A | free text |
| 108 | Broadcast Source | N/A | utf8 | max 127 | N/A | N/A | N/A | N/A | N/A | free text |
| 109 | Range To Recovery Location | km | imapb | max 4 | 0 | 21000 | N/A | 4 bytes = 1.525 cm | None | IMAPB(0,21000,length) |
| 110 | Time Airborne | s | uint | max 4 | 0 | 4294967295 | N/A | 1 second | None | |
| 111 | Propulsion Unit Speed | rpm | uint | max 4 | 0 | 4294967295 | N/A | 1 revolution/minute | None | |
| 112 | Platform Course Angle | deg | imapb | max 8 | 0 | 360 | N/A | 2 bytes = 16.625 millidegrees | None | IMAPB(0,360,length) |
| 113 | Altitude Above Ground Level (AGL) | m | imapb | max 4 | -900 | 40000 | N/A | 2 bytes = 2.0 meters | None | IMAPB(-900,40000,length) |
| 114 | Radar Altimeter | m | imapb | max 4 | -900 | 40000 | N/A | 2 bytes = 2.0 meters | None | IMAPB(-900,40000,length); AGL |
| 115 | Control Command | N/A | pack | max unlimited | N/A | N/A | N/A | N/A | N/A | defined-length pack (command id + vendor string) |
| 116 | Control Command Verification List | N/A | pack | max unlimited | N/A | N/A | N/A | N/A | N/A | defined-length pack of command ids |
| 117 | Sensor Azimuth Rate | deg/s | imapb | max 4 | -1000 | 1000 | N/A | 2 bytes = 0.0625 deg/s | None | IMAPB(-1000,1000,length) |
| 118 | Sensor Elevation Rate | deg/s | imapb | max 4 | -1000 | 1000 | N/A | 2 bytes = 0.0625 deg/s | None | IMAPB(-1000,1000,length) |
| 119 | Sensor Roll Rate | deg/s | imapb | max 4 | -1000 | 1000 | N/A | 2 bytes = 0.0625 deg/s | None | IMAPB(-1000,1000,length) |
| 120 | On-board MI Storage Percent Full | % | imapb | max 3 | 0 | 100 | N/A | 3 bytes = 1.5E-5 percent | None | IMAPB(0,100,length) |
| 121 | Active Wavelength List | N/A | list | max unlimited | N/A | N/A | N/A | N/A | N/A | variable-length pack list |
| 122 | Country Codes | N/A | list | max unlimited | N/A | N/A | N/A | N/A | N/A | variable-length pack list (3-letter codes) |
| 123 | Number of NAVSATs in View | count | uint | 1 | 0 | 255 | N/A | 1 | None | |
| 124 | Positioning Method Source | N/A | uint | 1 | 1 | 255 | N/A | N/A | None | enumeration |
| 125 | Platform Status | N/A | uint | 1 | 0 | 12 | N/A | N/A | None | enumeration 0-12 |
| 126 | Sensor Control Mode | N/A | uint | 1 | 0 | 6 | N/A | N/A | None | enumeration 0-6 |
| 127 | Sensor Frame Rate Pack | N/A | pack | max 16 | N/A | N/A | N/A | N/A | N/A | defined-length pack (numerator/denominator) |
| 128 | Wavelengths List | N/A | list | max unlimited | N/A | N/A | N/A | N/A | N/A | variable-length pack list |
| 129 | Target ID | N/A | utf8 | max 32 | N/A | N/A | N/A | N/A | N/A | free text |
| 130 | Airbase Locations | N/A | list | max 24 | N/A | N/A | N/A | N/A | N/A | variable-length pack list |
| 131 | Take-off Time | us | time | max 8 | 0 | 18446744073709551615 | N/A | 1 microsecond | None | MISP time, like Item 2 |
| 132 | Transmission Frequency | MHz | imapb | max 4 | 1 | 99999 | N/A | 3 bytes = 15.625 KHz | None | IMAPB(1,99999,length) |
| 133 | On-board MI Storage Capacity | GB | uint | max 4 | 0 | 4294967295 | N/A | 1 Gigabyte | None | |
| 134 | Zoom Percentage | % | imapb | max 4 | 0 | 100 | N/A | 2 bytes = .0039% | None | IMAPB(0,100,length) |
| 135 | Communications Method | N/A | utf8 | max 127 | N/A | N/A | N/A | N/A | N/A | free text |
| 136 | Leap Seconds | s | int | max 4 | -2147483648 | 2147483647 | N/A | 1 Second | None | |
| 137 | Correction Offset | us | int | max 8 | -9223372036854775808 | 9223372036854775807 | N/A | 1 microsecond | None | |
| 138 | Payload List | N/A | list | max unlimited | N/A | N/A | N/A | N/A | N/A | variable-length pack list |
| 139 | Active Payloads | N/A | bytes | max unlimited | N/A | N/A | N/A | N/A | N/A | bit set of payload ids |
| 140 | Weapons Stores | N/A | list | max unlimited | N/A | N/A | N/A | N/A | N/A | variable-length pack list |
| 141 | Waypoint List | N/A | list | max unlimited | N/A | N/A | N/A | N/A | N/A | variable-length pack list |
| 142 | View Domain | N/A | list | max unlimited | N/A | N/A | N/A | N/A | N/A | variable-length pack (four IMAPB bounds) |
| 143 | Metadata Substream ID Pack | N/A | pack | max 17 | N/A | N/A | N/A | N/A | N/A | defined-length pack (id + 16-byte UUID) |

## 3. Decoder coverage gaps (documented, not silent)

The `misb` 0.2.0 crate this workspace decodes through leaves some items raw. Every
gap below is one of the two shapes the crate documents (packs and lists are
"deliberately uninterpreted"; nested Local Sets are left to their own standard):

- **Nested Local Sets decoded** — tags 48 and 74 are `St0601Value::Security`
  (`St0102Security`, ST 0102 Table 2) and `St0601Value::Vmti` (`St0903Vmti`,
  ST 0903 Tables 9/10); the items those standards define are typed.
- **Nested Local Sets left raw** — tags 73, 95, 97, 98, 99, 100 and 101 (ST 0806,
  1206, 1002, 1601, 1602, 1607) come back as `St0601Value::Nested` with the
  standard named by `NestedSet`; their bytes are not decoded.
- **Packs and lists left raw** — tags 60, 61 (nibble packs), 81, 102, 115, 116, 127,
  143 (defined-length packs), 121, 122, 128, 130, 138, 140, 141, 142 (variable-length
  pack lists), 139 (bit set) and 94 (ST 1204 binary identifier).
- **One plain integer left raw** — tag 62 `Laser PRF Code` is a plain unsigned 16-bit
  value in §8 (`Format uint16`, `KLVval = SoftVal`), yet the decoder returns raw
  bytes. This is the only item whose *format* is numeric and whose kind is not; it is
  pinned by the test as a known decoder gap, not silently accepted.
- **`Deprecated`** — tag 66 has no value in §8 and decodes as raw bytes.

The sentinel distinction is also lossy: §8 gives several different in-band sentinels
(`"Out of Range"`, `"Reserved"`, `"N/A (Off-Earth)"`), and the decoder collapses all
of them into one "out of range" outcome. The transcription keeps the per-item
meaning; the decoded API does not carry it.
