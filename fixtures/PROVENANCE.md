# fixtures — MISB ST 0601 UAS Datalink Local Set packets

Two real UAS Datalink Local Set KLV packets, stored without an MPEG-2 TS
carrier. They exercise the public `decode` path end to end and are small enough
to keep in this repository.

## Provenance

- **Payload origin**: MISB ST 0902.5 Annex C, the "Dynamic Only" and
  "Dynamic and Constant" MISMMS packet data examples (a free US-Government
  publication). The per-item values are the ones ST 0601 prints as examples:
  tag 5 `71 C2` = 159.974365 degrees, tag 6 `FD 3D` = -0.431531724 degrees.
- **Copied from**: <https://github.com/paretech/klvdata>,
  `data/DynamicOnlyMISMMSPacketData.bin` and
  `data/DynamicConstantMISMMSPacketData.bin`.
- **Licence**: MIT. Copyright (c) 2016 Matthew Pare (paretech@gmail.com). The
  full permission notice is in [`LICENSE-klvdata`](LICENSE-klvdata), fetched
  from <https://raw.githubusercontent.com/paretech/klvdata/master/LICENSE.txt>.
- **Retrieved**: 2026-11-23.

## Files

| File | Bytes | SHA-256 |
|---|---|---|
| `misb/DynamicOnlyMISMMSPacketData.bin` | 114 | `c991fd59aaf4b6e32a0632769fe5e12de7ab670dd54b0001b2837407ea08b264` |
| `misb/DynamicConstantMISMMSPacketData.bin` | 228 | `73c235eb38a1b80e0a7d0f8a65e013099d602d093d72d34561dde4cec0d5a04a` |

## Known defect

`DynamicConstantMISMMSPacketData.bin` does not verify its own tag-1 Checksum
(`0xAA43` stored, `0x3E1E` computed). Compared with the packet printed in
ST 0902.5 Annex C (`specs-md/misb_st0902.5_2015_motion_imagery_sensor_minimum_metadata_set.md`),
the file differs in exactly four bytes, offsets 125-128: the tag 20 Sensor
Relative Azimuth value is `7D C5 5E CE` in the file and `00 00 00 00` in the
standard. The printed packet verifies its checksum (`0xAA43`); the file changed
the value without recomputing it. The packet is kept unmodified so the bytes
stay traceable; tests assert only the values they need rather than the
checksum. `DynamicOnlyMISMMSPacketData.bin` is byte-identical to the standard's
printed packet.
