# motion-imagery-metadata

Typed MISB ST 0601 UAS Datalink Local Set item values, decoded by the
[`misb`](https://crates.io/crates/misb) crate.

MISB ST 0601 is the telemetry a UAS payload computer carries next to full-motion
video in STANAG 4609: platform and sensor attitude, frame-centre position,
timestamps, and nested Local Sets for security markings and moving-target
indications. This crate turns one item — a BER-OID tag and its value bytes —
into a typed value:

- scaled numbers, unsigned/signed integers, UTF-8 text, and UTC instants
  (microseconds since the POSIX epoch);
- a nested Local Set with its standard named (`NestedSet::standard`), plus the
  typed ST 0102 Security (`NestedSet::security`) and ST 0903 VMTI
  (`NestedSet::vmti`) views;
- a pack or list, with its tag and raw value (`Pack::tag`, `Pack::raw`);
- `Bytes` for a tag ST 0601 does not define.

```rust
use motion_imagery_metadata::{decode, St0601Value};

let heading = decode(5, &[0x71, 0xC2]);
assert!(matches!(heading, St0601Value::Number(degrees) if (degrees - 159.9744).abs() < 1e-3));
```

## Scope

This crate decodes item *values*. KLV framing (SMPTE ST 336 / ST 0107: the
Universal Label, BER length, BER-OID tags and the UAS Local Set item loop) lives
in `transmux`. Enable that crate's `motion-imagery-metadata` feature to call
`LocalSetItem::value_typed()`, which delegates here:

```toml
transmux = { version = "0.32", features = ["motion-imagery-metadata"] }
```

## Dependencies

```toml
motion-imagery-metadata = "0.1"
```

Depends only on `broadcast-common` (workspace foundation) and `misb`. No `misb`
type appears in the public API or in any trait impl.

## What stays raw, and why (#4041)

The tag table, the linear/IMAPB scalings and the ST 0102/ST 0903 item decoders
are the `misb` crate's (ST 0601.19, ST 0102.12, ST 0903.6, ST 1201.5). Items it
leaves as bytes are not re-decoded by hand here:

| Items | Result | Why |
|---|---|---|
| 73, 95, 97–101 | `Nested` with its standard named | Those nested standards are not modelled by the workspace; `NestedSet::raw` keeps the value. |
| 60, 61, 81, 102, 115, 116, 121, 122, 127, 128, 130, 138–143 | `Pack` | Packs and lists need typed accessors that are a later, additive change; `Pack::raw` keeps the value. |
| 62, 66, 94 | `Pack` | `misb` 0.2 leaves the Laser PRF Code, the deprecated covariance item and the MIIS Core Identifier as bytes; surfaced as raw rather than guessed. |
| any other tag | `Bytes` | ST 0601 does not define it. |

## Spec citations

- MISB ST 0601.19: `private/specs-verified/misc/misb/st0601-tag-table.md`
- MISB ST 0102.12: `private/specs-verified/misc/misb/st0102-security-ls.md`
- MISB ST 0903.6: `private/specs-verified/misc/misb/st0903-vmti-ls.md`
- MISB ST 1201.5: `private/specs-verified/misc/misb/st1201-imapb.md`

## License

MIT OR Apache-2.0.
