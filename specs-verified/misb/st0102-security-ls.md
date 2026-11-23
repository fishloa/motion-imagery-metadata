# MISB ST 0102.12 Security Metadata Local Set — item table

> **Source.** MISB ST 0102.12 "Security Metadata Universal and Local Sets for
> Motion Imagery Data", Motion Imagery Standards Board, 22 June 2017. Vendored
> PDF, 18 pages: `specs/misb_st0102.12_security_local_set.pdf`
> (SHA-256 `20d40b5237cdcd2f486547add8eee238e37d5a6b11b7e0aca306be0785eca267`).
> Public US-Government document published by the NGA/NSG registry
> (`https://nsgreg.nga.mil/misb.jsp`). Obtained 2026-11-23 from the public mirror
> `https://media.githubusercontent.com/media/nitsuga/libmisbklv/main/references/ST0102.12.pdf`
> because `nsgreg.nga.mil` drops the TLS handshake from this network (the same
> route recorded in the ST 0601.19 provenance).
>
> **How it was produced.** §6.7 Table 2 (pages 13–14) was converted with
> `cd ~/Projects/pdf2md && uv run pdf2md convert <pdf> -o <md> --engine hybrid --pages 13-14 --report`
> (exit 0). The converter merged several cells; every field below was re-read
> from the PDF text layer with `pdftotext -layout` and the printed table.
>
> **Not covered here.** §6.8 Universal/Local Set conversion prose, §6.9 mixed
> country coding and §6.10 GENC prose. Only Table 2 (the Local Set items) is
> transcribed.

## 1. Local Set key

| Key | Name |
|---|---|
| `06.0E.2B.34.02.03.01.01.0E.01.03.03.02.00.00.00` (CRC 40980) | Security Metadata Local Set |

## 2. Items (Table 2)

Length is the KLV value length in bytes; `V` means variable.

| Tag | Name | Data type / references | Allowed values | Length | Required |
|---|---|---|---|---|---|
| 1 | Security Classification | uint8 | `0x01` UNCLASSIFIED//; `0x02` RESTRICTED//; `0x03` CONFIDENTIAL//; `0x04` SECRET//; `0x05` TOP SECRET// | 1 | Required |
| 2 | Classifying Country and Releasing Instructions Country Coding Method | uint8 | see §3 | 1 | Required |
| 3 | Classifying Country | text from the appropriate standard preceded by `//` (ISO/IEC 646) | e.g. `//CZE`, `//GB` | V | Required |
| 4 | Security-SCI/SHI information | ISO/IEC 646 | Security Ref | V | Context |
| 5 | Caveats | ISO/IEC 646 | Security Ref | V | Context |
| 6 | Releasing Instructions | ISO/IEC 646 | Security Refs | V | Context |
| 7 | Classified By | ISO/IEC 646 | Security Refs | V | Context |
| 8 | Derived From | ISO/IEC 646 | Security Refs | V | Context |
| 9 | Classification Reason | ISO/IEC 646 | Security Refs | V | Context |
| 10 | Declassification Date | ISO/IEC 646 | `YYYYMMDD` | 8 | Context |
| 11 | Classification and Marking System | ISO/IEC 646 | (none) | V | Context |
| 12 | Object Country Coding Method | uint8 | see §4 | 1 | Required |
| 13 | Object Country Codes | RFC 2781 (UTF-16) | country codes separated by `;` | V | Required |
| 14 | Classification Comments | ISO/IEC 646 | (none) | V | Optional |
| 22 | Version | uint16 | ST 0102 revision number; ST 0102.10 is `0x000A` | 2 | Required |
| 23 | Classifying Country and Releasing Instructions Country Coding Method Version Date | ISO/IEC 646 | `YYYY-MM-DD` | 10 | Optional |
| 24 | Object Country Coding Method Version Date | ISO/IEC 646 | `YYYY-MM-DD` | 10 | Optional |

Tags 19 (Stream ID), 20 (Transport Stream ID) and 21 (Item Designator ID) are
**deleted** in ST 0102.12 (§4 revision history: "Deleted keys: UMID, Stream ID,
Transport Stream ID, Item Designator"), so they are not items of this edition.

## 3. Item 2 — Classifying Country and Releasing Instructions Country Coding Method

| Value | Meaning |
|---|---|
| `0x01` | ISO-3166 Two Letter |
| `0x02` | ISO-3166 Three Letter |
| `0x03` | FIPS 10-4 Two Letter |
| `0x04` | FIPS 10-4 Four Letter |
| `0x05` | ISO-3166 Numeric |
| `0x06` | 1059 Two Letter |
| `0x07` | 1059 Three Letter |
| `0x08` | Omitted Value |
| `0x09` | Omitted Value |
| `0x0A` | FIPS 10-4 Mixed |
| `0x0B` | ISO 3166 Mixed |
| `0x0C` | STANAG 1059 Mixed |
| `0x0D` | GENC Two Letter |
| `0x0E` | GENC Three Letter |
| `0x0F` | GENC Numeric |
| `0x10` | GENC Mixed |

## 4. Item 12 — Object Country Coding Method

| Value | Meaning |
|---|---|
| `0x01` | ISO-3166 Two Letter |
| `0x02` | ISO-3166 Three Letter |
| `0x03` | ISO-3166 Numeric |
| `0x04` | FIPS 10-4 Two Letter |
| `0x05` | FIPS 10-4 Four Letter |
| `0x06` | 1059 Two Letter |
| `0x07` | 1059 Three Letter |
| `0x08` | Omitted Value |
| `0x09` | Omitted Value |
| `0x0A` | Omitted Value |
| `0x0B` | Omitted Value |
| `0x0C` | Omitted Value |
| `0x0D` | GENC Two Letter |
| `0x0E` | GENC Three Letter |
| `0x0F` | GENC Numeric |
| `0x40` | GENC AdminSub |

## 5. Item 13 encoding

§6.1.13: multiple object country codes are separated by `;` with no spaces and
concatenated in one item. Table 2 cites RFC 2781, i.e. UTF-16; the `misb` crate
accepts both UTF-16BE (even length, high bytes zero) and single-byte text from
older senders.

## 6. Worked examples printed by the standard

- §6.1.2: "Example of Country Coding Method: GENC Two Letter" (item 2 `0x0D`).
- §6.1.3: "Example of classifying country: `//CZE` (GENC code), `//GB`
  (ISO-3166 code)" (item 3).
- §6.1.10: item 10 format `YYYYMMDD`.
- §6.1.15: item 22 is the ST 0102 revision number.
