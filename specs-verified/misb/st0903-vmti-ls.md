# MISB ST 0903.6 VMTI Local Set — item tables

> **Source.** MISB ST 0903.6 "Video Moving Target Indicator and Track Metadata",
> Motion Imagery Standards Board, 21 October 2021. Vendored PDF, 72 pages:
> `specs/misb_st0903.6_vmti_local_set.pdf`
> (SHA-256 `aa09465cd6d2da3b04bdff33bfee090cfbe061a62c1f6ce2a40f8611d9f9ed07`).
> Public US-Government document published by the NGA/NSG registry
> (`https://nsgreg.nga.mil/misb.jsp`). Obtained 2026-11-23 from the public mirror
> `https://media.githubusercontent.com/media/nitsuga/libmisbklv/main/references/ST0903.6.pdf`
> because `nsgreg.nga.mil` drops the TLS handshake from this network.
>
> **How it was produced.** Table 9 (pages 24–25) and Table 10 (pages 31–33) were
> converted with
> `cd ~/Projects/pdf2md && uv run pdf2md convert <pdf> -o <md> --engine hybrid --pages 24-25 --report`
> and `--pages 31-33` (both exit 0); Tables 16/17 were read from the PDF text
> layer with `pdftotext -layout` and the printed pages. The converter split
> several rows; every field below was re-read from the printed table.
>
> **Not covered here.** §9 algorithms and prose, the VMask/VObject/VFeature/
> VTracker/VChip Local Set tables (11–15), and the per-item encoder prose. Only
> the VMTI LS, VTarget Pack, and the two Series element tables the `misb` crate
> leaves raw (Algorithm LS and Ontology LS) are transcribed.

## 1. Local Set keys

| Key | Name |
|---|---|
| `06.0E.2B.34.02.0B.01.01.0E.01.03.03.06.00.00.00` (CRC 51307) | VMTI Local Set |

## 2. VMTI LS (Table 9)

`V` means variable; `V32`/`V128` are variable with the printed maximum.

| Tag | Name | Units | Format | Length | Description |
|---|---|---|---|---|---|
| 1 | checkSum | None | uint | 2 | Checksum, standalone-VMTI only |
| 2 | precisionTimeStamp | µs | uint | 8 | Microseconds since the 1970 epoch (ST 0603) |
| 3 | vmtiSystemName | N/A | utf8 | V32 | Name and/or description of the VMTI system |
| 4 | vmtiLsVersionNum | N/A | uint | V2 | VMTI LS version |
| 5 | totalNumTargetsDetected | N/A | uint | V3 | Total targets in the processing model |
| 6 | numTargetsReported | N/A | uint | V3 | Targets reported after culling |
| 7 | (DEPRECATED) | — | — | — | Deprecated by ST 0903.6 |
| 8 | frameWidth | pixels | uint | V3 | Motion imagery frame width |
| 9 | frameHeight | pixels | uint | V3 | Motion imagery frame height |
| 10 | vmtiSourceSensor | N/A | utf8 | V128 | VMTI source sensor |
| 11 | vmtiHorizontalFov | ° | IMAPB | 2 | Horizontal field of view |
| 12 | vmtiVerticalFov | ° | IMAPB | 2 | Vertical field of view |
| 13 | miisId | N/A | binary | V | MIIS Core Identifier (MISB ST 1204) |
| 101 | vTargetSeries | N/A | Series | V | VTarget Packs ordered as a Series |
| 102 | algorithmSeries | N/A | Series | V | Series of one or more Algorithm LS |
| 103 | ontologySeries | N/A | Series | V | Series of one or more Ontology LS |

The `misb` crate decodes tags 2–12 and the VTarget Series targets below. It leaves
items 13, 102 and 103 as raw bytes (`VmtiSet::raw`), and has no accessor for the
standalone checksum (tag 1).

## 3. VTarget Pack (Table 10)

The pack is a BER-OID `targetId` (mandatory, Item 1 of the pack) followed by one
or more TLV items (Item 2). Item 21 is deprecated in ST 0903.6.

| Tag | Name | Units | Format | Length | Description |
|---|---|---|---|---|---|
| N/A | targetId | N/A | BER-OID | V9 | Mandatory first value in a VTarget Pack |
| 1 | targetCentroid | pixel number | uint | V6 | Target position as a pixel number |
| 2 | boundingBoxTopLeft | pixel number | uint | V6 | Top-left corner pixel number |
| 3 | boundingBoxBottomRight | pixel number | uint | V6 | Bottom-right corner pixel number |
| 4 | targetPriority | N/A | uint | 1 | Target priority/validity |
| 5 | targetConfidenceLevel | N/A | uint | 1 | Target confidence |
| 6 | targetHistory | N/A | uint | V2 | Previous detections of the same target |
| 7 | percentageOfTargetPixels | N/A | uint | 1 | Target pixels / bounding-box pixels × 100 |
| 8 | targetColor | N/A | uint | 3 | Dominant colour (0xRRGGBB) |
| 9 | targetIntensity | N/A | uint | V3 | Dominant intensity |
| 10 | targetLocationOffsetLat | ° | IMAPB | 3 | Latitude offset from frame centre |
| 11 | targetLocationOffsetLon | ° | IMAPB | 3 | Longitude offset from frame centre |
| 12 | targetHae | m | IMAPB | 2 | Height above the WGS84 ellipsoid |
| 13 | boundingBoxTopLeftLatOffset | ° | IMAPB | 3 | Top-left latitude offset |
| 14 | boundingBoxTopLeftLonOffset | ° | IMAPB | 3 | Top-left longitude offset |
| 15 | boundingBoxBottomRightLatOffset | ° | IMAPB | 3 | Bottom-right latitude offset |
| 16 | boundingBoxBottomRightLonOffset | ° | IMAPB | 3 | Bottom-right longitude offset |
| 17 | targetLocation | N/A | Location | V | Latitude, longitude, height (location pack) |
| 18 | geospatialContourSeries | N/A | Series | V | Geospatial boundary |
| 19 | centroidPixRow | N/A | uint | V4 | Centroid row |
| 20 | centroidPixCol | N/A | uint | V4 | Centroid column |
| 21 | (DEPRECATED) | — | — | — | Deprecated by ST 0903.6 |
| 22 | algorithmId | N/A | uint | V3 | Algorithm that detected the target |
| 23 | detectionStatus | N/A | uint | 1 | Inactive, Active-Moving, Dropped, Active-Stopped, Active-Coasting |
| 101 | vMask | N/A | Local Set | V | Target perimeter mask |
| 102 | (DEPRECATED) | — | — | — | Deprecated by ST 0903.6 |
| 103 | (DEPRECATED) | — | — | — | Deprecated by ST 0903.6 |
| 104 | vTracker | N/A | Local Set | V | Track information |
| 105 | vChip | N/A | Local Set | V | Underlying pixel values |
| 106 | vChipSeries | N/A | Series | V | Series of one or more VChip LS |
| 107 | vObjectSeries | N/A | Series | V | Series of one or more VObject LS |

## 4. Algorithm LS (Table 16, carried raw in item 102)

`V8` = variable, maximum 8 bytes.

| Tag | Name | Units | Format | Length | Description |
|---|---|---|---|---|---|
| 1 | algorithmId | N/A | uint | V8 | Unique identifier within the series |
| 2 | name | N/A | utf8 | V | Algorithm name |
| 3 | version | N/A | utf8 | V | Algorithm version |
| 4 | class | N/A | utf8 | V | Algorithm type (detector, classifier, …) |
| 5 | nFrames | N/A | uint | V8 | Frames the algorithm operates over |

Worked per-item examples (§10.8.1–§10.8.5): algorithmId `9` → `0x09`; name
`k6_yolo_9000_tracker` (`0x6B36 5F79 … 636B 6572`, 20 bytes); version `2.6a` →
`0x322E 3661`; class `kalmann` → `0x6B61 6C6D 616E 6E`; nFrames `10` → `0x0A`.

## 5. Ontology LS (Table 17, carried raw in item 103)

| Tag | Name | Units | Format | Length | Description |
|---|---|---|---|---|---|
| 1 | ontologyId | N/A | uint | V8 | Unique identifier within the series |
| 2 | parentId | N/A | uint | V8 | Link to a related Ontology LS |
| 3 | ontologyIRI | N/A | utf8 | V | IRI of the OWL ontology |
| 4 | entityIRI | N/A | utf8 | V | IRI of an entity in the ontology |
| 5 | versionIRI | N/A | utf8 | V | IRI of the ontology version |
| 6 | label | N/A | utf8 | V | Entity label defined in the ontology |

## 6. Value conversions used by the decoders

- Item 2 precisionTimeStamp: microseconds since 1970-01-01T00:00:00Z, no leap
  seconds (§10.1.2). Example: April 19 2001 04:25:21.000000 GMT
  (`987654321000000`) → `0x0003 8244 30F6 CE40`.
- Items 11/12: `IMAPB(0, 180, 2)`.
- VTarget items 10/11: `IMAPB(-19.2, 19.2, 3)` (§10.2.2.11/12). Example
  `10.00` → `0x3A66 67`, `12.00` → `0x3E66 67`.
- VTarget item 12 targetHae: `IMAPB(-900, 19000, 2)`; example `10000` →
  `0x2A94` (§10.2.2.13).
- VTarget item 17 targetLocation: a Location pack whose first three fields are
  latitude `IMAPB(-90, 90, 4)`, longitude `IMAPB(-180, 180, 4)` and height
  `IMAPB(-900, 19000, 2)`, followed by the optional sigma/rho fields. The
  `misb` crate decodes only the first 10 bytes (latitude, longitude, height);
  the sigma/rho tail is left undecoded.

## 7. Worked example used by the tests

The committed `DynamicConstantMISMMSPacketData.bin` carries ST 0601 tag 48; its
28-byte ST 0102 set is asserted directly: classification `0x01` (UNCLASSIFIED),
country coding method `0x07` (1059 Three Letter), classifying country `//USA`,
object country coding method `0x07` (1059 Three Letter), object country codes
`USA` in UTF-16BE, version `10` (`0x000A`). `DynamicOnlyMISMMSPacketData.bin`
carries neither tag 48 nor tag 74.

No committed fixture carries ST 0601 tag 74, so the VMTI test builds a UAS
Datalink LS packet around the values printed by this standard:

- VMTI LS set items from §10.1: item 2 `0x0003 8244 30F6 CE40`
  (987654321000000 µs), item 3 `DSTO_ADSS_VMTI`, item 4 `0x06`, item 5 `0x1C`,
  item 6 `0x0E`, item 8 `0x0780`, item 9 `0x0438`, item 10 `EO Nose`, item 11
  `0x0640` (12.5°), item 12 `0x0500` (10.0°).
- One VTarget pack from §10.2.2, target ID 1234 (`0x8952`): centroid and both
  bounding-box corners `0x064000` (409600), priority `0x1B` (27), confidence
  `0x50` (80%), history `0x0ACD` (2765), percentage `0x32` (50%), colour
  `0xDAA520` (218,165,32), intensity `0x3354` (13140), offsets `0x3A6667`
  (10.00°) and `0x3E6667` (12.00°), height `0x2A94` (10000 m), centroid row/col
  `0x0368`/`0x0471` (872/1137), algorithm ID 9, detection status 1
  (Active-Moving).
- Item 102 from the §10.8 per-item examples (`algorithmId` 9, name
  `k6_yolo_9000_tracker`, version `2.6a`, class `kalmann`, nFrames 10).
