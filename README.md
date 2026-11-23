# motion-imagery-metadata

Typed MISB ST 0601 UAS Datalink Local Set item values, decoded by the
[`misb`](https://crates.io/crates/misb) crate.

MISB ST 0601 is the telemetry a UAS payload computer carries next to full-motion
video in STANAG 4609: platform and sensor attitude, frame-centre position,
timestamps, and nested Local Sets for security markings and moving-target
indications. This crate turns one item, a BER-OID tag and its value bytes, into
a typed value.

## Usage

```rust
use motion_imagery_metadata::{decode, St0601Value};

let heading = decode(5, &[0x71, 0xC2]);
assert!(matches!(heading, St0601Value::Number(degrees) if (degrees - 159.9744).abs() < 1e-3));
```

The same code is built as the `readme` example.

## Scope

This crate decodes item *values*. KLV framing (SMPTE ST 336 / ST 0107: the
Universal Label, BER length, BER-OID tags and the UAS Local Set item loop) lives
in `transmux`. Enable that crate's `motion-imagery-metadata` feature to call
`LocalSetItem::value_typed()`, which delegates here:

```toml
transmux = { version = "0.33", features = ["motion-imagery-metadata"] }
```

## Supported standards

- **MISB ST 0601.19** UAS Datalink Local Set: scaled numbers, unsigned and
  signed integers, UTF-8 text, and UTC instants (`St0601Value::Time`, a
  `jiff::Timestamp`).
- **MISB ST 0102.12** Security Metadata Local Set: `NestedSet::security()`
  returns the typed `St0102Security` view.
- **MISB ST 0903.6** Video Moving Target Indicator Local Set:
  `NestedSet::vmti()` returns `St0903Vmti`, with typed VMTI items, targets and
  raw views for items the decoder does not interpret.
- **Other nested Local Sets** (ST 0806, ST 1206, ST 1002, ST 1601, ST 1602,
  ST 1607): named by `NestedSet::standard()` and available uninterpreted
  through `NestedSet::raw()`.
- **Packs and lists**: tag and raw value through `Pack::tag()` and
  `Pack::raw()`.
- **Undefined tags**: `St0601Value::Bytes`.

## Dependencies

```toml
motion-imagery-metadata = "0.1"
```

Depends on `broadcast-common`, `misb` and `jiff`. No `misb` type appears in the
public API or in any trait impl.

## Credits

The ST 0601 tag table and the ST 0102 / ST 0903 / ST 1201 decoders come from
the [`misb`](https://crates.io/crates/misb) crate by Matt Busel.

MISB standards are published by the NSG MISB. This repository contains no spec
text; the test suite reads optional spec transcriptions from a directory named
by `MIM_PRIVATE_DIR`.

## License

MIT OR Apache-2.0.
