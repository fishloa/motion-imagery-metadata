**MISB ST 0902.5**

**STANDARD**

**Motion Imagery Sensor Minimum Metadata Set**

**29 October 2015**

# 1 Scope

This Standard defines the Motion Imagery Sensor Minimum Metadata Set (MISMMS) that enables the basic capabilities of Situational Awareness, Discovery & Retrieval, and Cross-Domain Dissemination. In addition, it provides direction on the encoding of the MISMMS for low-bitrate to high-bitrate scenarios to support a transition path from analog systems towards all digital motion imagery systems.

# 2 References

## 2.1 Normative References

The following references and the references contained therein are normative.

- [1] MISP-2016.1, Motion Imagery Standards Profile, Oct 2015
- [2] MISB ST 0601.9, UAS Datalink Local Set, Oct 2015
- [3] MISB ST 0102.11, Security Metadata Universal and Local Sets for Digital Motion Imagery, Oct 2014
- [4] MISB ST 1204.1, Motion Imagery Identification System (MIIS) – Core Identifier, Oct 2013
- [5] MISB ST 0107.2, Bit and Byte Order for Metadata in Motion Imagery Files and Streams, Feb 2014

# 3 Terms, Acronyms and Definitions

|  |  |
|---|---|
| BER | Basic Encoding Rules |
| D&R | Discovery and Retrieval |
| FoV | Field of View |
| HAE | Height above Ellipsoid |
| ISR | Intelligence, Surveillance and Reconnaissance |
| KLV | Key Length Value |
| LS | Local Set |
| MFG | Metadata Focus Group |
| MIIS | Motion Imagery Identification System |
| MISB | Motion Imagery Standards Board |
| MISMMS | Motion Imagery Sensor Minimum Metadata Set |
| MISP | Motion Imagery Standards Profile |
| MSL | Mean Sea Level |
| NATO | North Atlantic Treaty Organization |
| NGA | National Geospatial-Intelligence Agency |
| NIAT | NGA Interoperability Action Team |
| NTB | NITF Technical Board |
| SA | Situational Awareness |
| STANAG | Standardization Agreement |
| TLV | Tag Length Value |
| UL | Universal Label |
| USCENTCOM | U.S. Central Command |

# 4 Revision History

| Revision | Date | Summary of Changes |
|---|---|---|
| 0902.5 | 10/29/2015 | • Allowed distance-restricted or distance-extended representations of Target Width<br>• Changed UNIX Time Stamp to Precision Time Stamp<br>• Updated acronyms list |

# 5 Introduction, Intent, and General Implementation Rules

The MISMMS consists of metadata elements from MISB ST 0601[2] that enable the baseline functionality required for Discovery & Retrieval / Cross-Domain Dissemination of source imagery and the Situational Awareness Product for ISR mission accomplishment as defined in the MISP[1].

The initial MISMMS was developed jointly by the Motion Imagery Standards Board (MISB), NITFS Technical Board (NTB), NGA Interoperability Action Team (NIAT), and Metadata Focus Group (MFG) with additional subject matter expert input from U.S. Central Command (USCENTCOM), NATO Standardization Agreement (STANAG) 4586, and the Unmanned Systems community.

As motion imagery systems begin to migrate to all-digital architectures, there are still some systems in transition which require the consistent preservation of certain analog system characteristics. This Standard identifies a way to encode the original, source-derived metadata from a motion imagery sensor into a standard KLV digital metadata set.

# 6 Motion Imagery Sensor Minimum Metadata Set

The intent of the MISMMS is to profile MISB ST 0601 as a Local Set representation with all elements mandatory unless conditionally dependent.

| Requirement(s) |  |
|---|---|
| ST 0902.3-01 | All metadata shall be expressed in accordance with MISB ST 0107[5]. |
| ST 0902.3-02 | The MISMMS shall use MISB ST 0601[2] Local Set 16-byte Universal Key (06.0E.2B.34.02.0B.01.01.0E.01.03.01.01.00.00.00 (CRC 56773)) for its implementation. |
| ST 0902.3-03 | The elements of the MISMMS as defined in MISB ST 0902 Table 1 shall be populated in accordance with MISB ST 0601 requirements. |
| ST 0902.3-04 | All metadata elements contained in the MISMMS shall be reported no less than once every thirty (30) seconds under all circumstances. |

Please refer to MISB ST 0601[2] for further information on each element. Of interest, Tag 48 of MISB ST 0601 contains a nested security Local Set as defined in MISB ST 0102[3]. Please refer to MISB ST 0102 for further information regarding each security element. MISB ST 0601 also contains reference to a Motion Imagery Core Identifier as defined in MISB ST 1204[4]. Please refer to MISB ST 1204 for further information regarding the Motion Imagery Core Identifier.

Note 1: Platform Pitch Angle (Tag 6 | 90), Platform Roll Angle (Tag 7 | 91), Sensor True Altitude as MSL (Tag 15) | Sensor Ellipsoid Height as HAE (Tag 75), Frame Center Elevation as MSL (Tag 25) | Frame Center Height Above Ellipsoid (Tag 78), and Target Width (Tag 22) | Target Width Extended (Tag 96) are governed by an “inclusive or” within MISB ST 0601.

Note 2: Motion Imagery Core Identifier (Tag 94) is required as the Motion Imagery data is disseminated from the “system” where the system is the composite of the platform and control station (i.e. Ground Control Station).

Table 1: Summary of MISMMS Tags

| Tag # | Tag Name | Range & Units | Length (Bytes) |
|---|---|---|---|
| 1 | Checksum | None | 2 |
| 2 | Precision Time Stamp | Microseconds | 8 |
| 3 | Mission ID | String | 1-127 |
| 5 | Platform Heading Angle | 0-360 Degrees | 2 |
| 6 \| | Platform Pitch Angle (Short) | +/- 20 Degrees | 2 |
| 90 | Platform Pitch Angle (Full) | +/- 90 Degrees | 4 |
| 7 \| | Platform Roll Angle (Short) | +/- 50 Degrees | 2 |
| 91 | Platform Roll Angle (Full) | +/- 90 Degrees | 4 |
| 10 | Platform Designation | String | 1-127 |
| 11 | Image Source Sensor | String | 1-127 |
| 12 | Image Coordinate System | String | 1-127 |
| 13 | Sensor Latitude | +/- 90 Degrees | 4 |
| 14 | Sensor Longitude | +/- 180 Degrees | 4 |
| 15 \| | Sensor True Altitude (MSL) | -900 to 19000m | 2 |
| 75 | Sensor Ellipsoid Height (HAE) |  |  |
| 16 | Sensor Horizontal FoV | 0 to 180 Degrees | 2 |
| 17 | Sensor Vertical FoV | 0 to 180 Degrees | 2 |
| 18 | Sensor Relative Azimuth Angle | 0 to 360 Degrees | 4 |
| 19 | Sensor Relative Elevation Angle | +/- 180 Degrees | 4 |
| 20 | Sensor Relative Roll Angle | 0 to 360 Degrees | 4 |
| 21 | Slant Range | 0 to 5000000 m | 4 |
| 22 \| | Target Width | 0 to 10000 m | 2 |
| 96 | Target Width Extended | 0 to 1500000 m | 3 |
| 23 | Frame Center Latitude | +/0 90 Degrees | 4 |
| 24 | Frame Center Longitude | +/- 180 Degrees | 4 |
| 25 \| | Frame Center Elevation (MSL) | -900 to 19000 m | 2 |
| 78 | Frame Center Height Above Ellipsoid (HAE) |  |  |
| 48/1 | Security Classification | Look Up Table | 1 |
| 48/2 | Classifying Country & Releasing Instructions Country Coding Method | Look Up Table | 1 |
| 48/3 | Classifying Country | String | Variable |
| 48/4 | Security-SCI/SHI Information | String | Variable |
| 48/5 | Caveats | String | Variable |
| 48/6 | Releasing Instructions | String | Variable |
| 48/12 | Object Country Coding Method | Look Up Table | 1 |
| 48/13 | Object Country Codes | String | Variable |
| 48/22 | Security Metadata Version | Integer | 2 |
| 65 | UAS Local Set Version | Integer | 1 |
| 94 | Motion Imagery Core Identifier | None | 50 |

# 7 Annex A: Recommended MISMMS Element Frequency – Informative

This section addresses the presence and frequency of individual data elements within the MISMMS.

Some metadata elements change more rapidly than others. When sending metadata in bandwidth-constrained environments, it is more efficient to dedicate the bulk of the available bandwidth to the dynamic items (“Dynamic”) with rapidly changing values, and include the constant items (“Constant”) less often. This section provides guidance in the form of a “recommended update rate” for each element of the MISMMS.

Some of the security metadata elements may be omitted as they are conditional. Further direction on this topic is found in MISB ST 0102.

Recommendation Summary for MISMMS Frequency (see Table 2):

1. Include Tags 3, 10, 11, 12, 48 sub-tags & 94 once every 10 seconds

2. Include all other elements as often as possible, within the available bandwidth and up to the frame rate

The rate at which “Dynamic” items update varies depending on the available bandwidth of the system, and how frequently the metadata values are refreshed. In some systems, it is desirable to have metadata updated for each motion imagery frame. For a specific worked example, see Annex B: Example MISMMS Element Rate Calculations – Informative.

Please note, it is not mandatory to make each metadata packet contain every metadata element; this bandwidth study demonstrates the viability of transmitting the MISMMS in a bandwidth-constrained environment. If the bandwidth supports it, other metadata packet configurations (e.g. more packets each containing fewer elements) are permissible.

Table 2: Recommended Update Rate for MISMMS Elements

| Tag # | Tag Name | Max Length (Bytes) | Rec Update Interval |
|---|---|---|---|
| 1 | Checksum | 2 | Fast |
| 2 | Precision Time Stamp | 8 | Fast |
| 3 | Mission ID | 127 | 10 s |
| 5 | Platform Heading Angle | 2 | Fast |
| 6 \| | Platform Pitch Angle (Short) | 2 | Fast |
| 90 | Platform Pitch Angle (Full) | 4 | Fast |
| 7 \| | Platform Roll Angle (Short) | 2 | Fast |
| 91 | Platform Roll Angle (Full) | 4 | Fast |
| 10 | Platform Designation | 127 | 10 s |
| 11 | Image Source Sensor | 127 | 10 s |
| 12 | Image Coordinate System | 127 | 10 s |
| 13 | Sensor Latitude | 4 | Fast |
| 14 | Sensor Longitude | 4 | Fast |
| 15 \| | Sensor True Altitude (MSL) | 2 | Fast |
| 75 | Sensor Ellipsoid Height (HAE) |  |  |
| 16 | Sensor Horizontal FoV | 2 | Fast |
| 17 | Sensor Vertical FoV | 2 | Fast |
| 18 | Sensor Relative Azimuth Angle | 4 | Fast |
| 19 | Sensor Relative Elevation Angle | 4 | Fast |
| 20 | Sensor Relative Roll Angle | 4 | Fast |
| 21 | Slant Range | 4 | Fast |
| 22 \| | Target Width | 2 | Fast |
| 96 | Target Width Extended | 3 | Fast |
| 23 | Frame Center Latitude | 4 | Fast |
| 24 | Frame Center Longitude | 4 | Fast |
| 25 \| | Frame Center Elevation (MSL) | 2 | Fast |
| 78 | Frame Center Height Above Ellipsoid (HAE) |  |  |
| 48/1 | Security Classification | 1 | 10 s |
| 48/2 | Classifying Country & Releasing Instructions Country Coding Method | 1 | 10 s |
| 48/3 | Classifying Country | 6* | 10 s |
| 48/4 | Security-SCI/SHI Information | 40* | 10 s |
| 48/5 | Caveats | 32* | 10 s |
| 48/6 | Releasing Instructions | 40* | 10 s |
| 48/12 | Object Country Coding Method | 1 | 10 s |
| 48/13 | Object Country Codes | 40* | 10 s |
| 48/22 | Security Metadata Version | 2 | 10 s |
| 65 | UAS Local Set Version | 1 | Fast |
| 94 | Motion Imagery Core Identifier | 50 | 10 s |

Note 3: * indicates the maximum length for these elements are only representative for the example in Appendix B as MISB ST 0102 does not define a maximum length.

# 8 Annex B: Example MISMMS Element Rate Calculations – Informative

This section provides bandwidth budget calculations, assuming a 9600 bits-per-second channel for metadata.

Calculations are presented for two general scenarios:

1. Scenario 1: Assumes all content is present in each KLV packet

2. Scenario 2: Assumes all content is not present in each KLV packet

## 8.1 Scenario 1: All Content Always Present

Worst Case: 18 bytes (16-byte Local Set UL Key + 2-byte Local Set Length) + 77 bytes (38 1-byte Tags, 38 1-byte Lengths & Security Local Set (1-byte Tag & 2-byte Length)) + 795 bytes (Payload) = 892 bytes = 7136 bits. Assuming serial transmission overhead of 1 start bit and 1 stop bit per byte, there are 8920 bits total. This allows for one metadata update per second at 9600 bits per second.

Typical Case: Limit Strings to 20 bytes, eliminate Tags 6, 7, 15, 22 & 25, and reduce MIIS to 34 bytes. 18 bytes (16-byte Local Set UL Key + 2-byte Local Set Length) + 68 bytes (33 1-byte Tags, 33 1-byte Lengths & Security Local Set (1-byte Tag & 1-byte Length)) + 269 bytes (Payload) = 355 bytes = 2840 bits. Serial overhead factor of 1.25 assumed, yielding 3550 bits total. Enables two metadata updates per second at 9600 bits per second.

## 8.2 Scenario 2: Content Present at Variable Rates

If the data rates of individual metadata elements are varied, there are more options. In particular, Table 3 shows rate calculations when the constraints in the Typical Case are combined with the update rates of Table 2.

Table 3: Rate Calculation for “Dynamic Only” & “Dynamic & Constant” Packets

| Description | Totals | “Dynamic Only” | “Dynamic & Constant” |
|---|---|---|---|
| Bytes for Data Fields |  | 64 | 269 |
| Bytes for Local Set Key & Length |  | 17 | 18 |
| Bytes for Tags & Lengths |  | 38 | 68 |
| Bytes to Transmit |  | 119 | 355 |
| Bits to Transmit |  | 952 | 2840 |
| Bits for Serial Overhead1 for 8-N-1 |  | 238 | 710 |
| Bits Sent per KLV Packet |  | 1190 | 3550 |
| Bits / 10 s Interval @ 9600 bits per second | 96000 |  |  |
| Bits Remaining After “Dynamic & Constant” Packet | 92460 |  |  |
| Possible “Dynamic Only” Packets / 10 s Interval in Bits Remaining | 77.689 |  |  |
| Whole “Dynamic Only” Packets / 10 s Interval in Bits Remaining | 77 |  |  |

1 Assumes serial (RS-232 or RS-485) transmission with 1 start bit, 0 parity bits, and 1 stop bit for each byte (i.e. 10 bits transmitted for every 8 bits of data)

That is, in a 10-second interval, it is possible to send 1 “Dynamic & Constant” MISMMS packet followed by 77 “Dynamic Only” MISMMS packets.

If each packet (“Dynamic Only” or “Dynamic & Constant”) is separated by an equal sized gap in time, then the rate calculations are as shown in Table 4:

Table 4: Rate Calculation for “Dynamic Only” & “Dynamic & Constant” Packets with Gaps

| Description | Totals |
|---|---|
| Whole “Dynamic Only” Packets / 10 s | 77 |
| Bits / 10 s @ 9600 bits per second | 96000 |
| Bits / 78 “Dynamic Only” Packets | 91630 |
| Bits / 1 “Dynamic & Constant” Packet | 3550 |
| Bits Remaining | 820 |
| # of Gaps | 78 |
| Bits / Gap | 10.51282 |
| Gap Time @ 9600 bps (microseconds) | 1095.085 |

This packet schedule repeats every 10 seconds. Figure 1 illustrates:

```
Dynamic &
Constant     Gap        Dynamic
                         Only       Gap       Dynamic
                                              Only     …    Gap    Dynamic
                                                                       Only     Gap
  1                         2                  3                       78
                                 10 Seconds
```

Figure 1: Time Interval with “Dynamic Only” Packets and Gaps

# 9 Annex C: Example MISMMS Data Packets – Informative

This section constructs hypothetical example MISMMS data packets and presents the encoding. All values shown in Table 5 are for illustration purposes only. The Tag-Length-Value (TLV) encoded data are shown as hexadecimal bytes.

Table 5: Example “Dynamic & Constant” MISMMS Packet Data

| Tag # | Tag Name | Value | Interpretation | TLV Hex Bytes |
|---|---|---|---|---|
| 2 | Precision Time Stamp | 1,231,798,102,000,000 microseconds | Mon Jan 12 2009 22:08:22 | 02 08 00 04 60 50 58 4E 01 80 |
| 3 | Mission ID | Mission 12 | Mission 12 | 03 0A 4D 69 73 73 69 6F 6E 20 31 32 |
| 5 | Platform Heading Angle | 0x71C2 | 159.974 Degrees | 05 02 71 C2 |
| 6 | Platform Pitch Angle (Short) | 0xFD3D | -0.4315 Degrees | 06 02 FD 3D |
| 7 | Platform Roll Angle (Short) | 0x08B8 | 3.406 Degrees | 07 02 08 B8 |
| 10 | Platform Designation | Predator | Predator | 0A 08 50 72 65 64 61 74 6F 72 |
| 11 | Image Source Sensor | EO Nose | EO Nose | 0B 07 45 4F 20 4E 6F 73 65 |
| 12 | Image Coordinate System | Geodetic WGS84 | Geodetic WGS84 | 0C 0E 47 65 6F 64 65 74 69 63 20 57 47 53 38 34 |
| 13 | Sensor Latitude | 0x5595B66D | 60.17682297 Degrees | 0D 04 55 95 B6 6D |
| 14 | Sensor Longitude | 0x5B5360C4 | 128.42675904 Degrees | 0E 04 5B 53 60 C4 |
| 15 | Sensor True Altitude (MSL) | 0xC221 | 14190.7 Meters | 0F 02 C2 21 |
| 16 | Sensor Horizontal FoV | 0xCD9C | 144.571 Degrees | 10 02 CD 9C |
| 17 | Sensor Vertical FoV | 0xD917 | 152.644 Degrees | 11 02 D9 17 |
| 18 | Sensor Rel. Az. Angle | 0x724A0A20 | 160.71921144 Degrees | 12 04 72 4A 0A 20 |
| 19 | Sensor Rel. El. Angle | 0x87F84B86 | -168.79232483 Degrees | 13 04 87 F8 4B 86 |
| 20 | Sensor Rel. Roll Angle | 0x7DC55ECE | 176.86543765 Degrees | 14 04 7D C5 5E CE |
| 21 | Slant Range | 0x03830926 | 68590.983 Meters | 15 04 03 83 09 26 |
| 22 | Target Width | 0x1281 | 722.8 Meters | 16 02 12 81 |
| 23 | Frame Center Lat. | 0xF101A229 | -10.54238863 Degrees | 17 04 F1 01 A2 29 |
| 24 | Frame Center Lon. | 0x14BC082B | 29.15789012 Degrees | 18 04 14 BC 08 2B |
| 25 | Frame Center El. (MSL) | 0x34F3 | 3216.0 Meters | 19 02 34 F3 |
| 48/1 | Security Classification | 0x01 | UNCLASSIFIED// | 01 01 01 |
| 48/2 | Classifying Country and Releasing Instructions Country Coding Method | 0x07 | STANAG-1059 Three Letter | 02 01 07 |
| 48/3 | Classifying Country | //USA | //USA | 03 05 2F 2F 55 53 41 |
| 48/4 | Security-SCI/SHI Information | <None> | <None> |  |
| 48/5 | Caveats | <None> | <None> |  |
| 48/6 | Releasing Instructions | <None> | <None> |  |
| 48/12 | Object Country Coding Method | 0x07 | STANAG-1059 Three Letter | 0C 01 07 |
| 48/13 | Object Country Codes | \u0055\u0053\u0041 | USA | 0D 06 00 55 00 53 00 41 |
| 48/22 | Security Metadata Version | 10 | MISB Standard 0102.10 | 16 02 00 0A |
| 65 | UAS Local Set Version | 0x06 | MISB Standard 0601.6 | 41 01 06 |
| 94 | Motion Imagery Core Identifier |  |  | 5E 22 01 70 F5 92 F0 23 73 36 4A F8 AA 91 62 C0 0F 2E B2 DA 16 B7 43 41 00 08 41 A0 BE 36 5B 5A B9 6A 36 45 |
| 1 | Checksum | 0xAA43 | 0xAA43 | 01 02 AA 43 |

The TLV bytes are appended end-to-end, and together form the value portion of the encompassing KLV packet. In the “Dynamic & Constant” packet, there are 210 bytes of TLV data – encoded as the length of the KLV packet. Lengths up to 127 bytes are encoded as the BER short form length. Lengths of 128 bytes or higher are encoded as the BER long form length. In this case, 210 bytes is encoded as the BER long form length of 0x81D2.

The Local Set begins with its 16-byte UL key, followed by the length of the data set (0x81D2), which is then followed by the TLV value bytes listed above in order. In hex, the complete MISMMS “Dynamic & Constant” example KLV packet is:

```
06 0E 2B 34 02 0B 01 01 0E 01 03 01 01 00 00 00
81 D2 02 08 00 04 60 50 58 4E 01 80 03 0A 4D 69
73 73 69 6F 6E 20 31 32 05 02 71 C2 06 02 FD 3D
07 02 08 B8 0A 08 50 72 65 64 61 74 6F 72 0B 07
45 4F 20 4E 6F 73 65 0C 0E 47 65 6F 64 65 74 69
63 20 57 47 53 38 34 0D 04 55 95 B6 6D 0E 04 5B
53 60 C4 0F 02 C2 21 10 02 CD 9C 11 02 D9 17 12
04 72 4A 0A 20 13 04 87 F8 4B 86 14 04 00 00 00
00 15 04 03 83 09 26 16 02 12 81 17 04 F1 01 A2
29 18 04 14 BC 08 2B 19 02 34 F3 30 1C 01 01 01
02 01 07 03 05 2F 2F 55 53 41 0C 01 07 0D 06 00
55 00 53 00 41 16 02 00 0A 41 01 06 5E 22 01 70
F5 92 F0 23 73 36 4A F8 AA 91 62 C0 0F 2E B2 DA
16 B7 43 41 00 08 41 A0 BE 36 5B 5A B9 6A 36 45
01 02 AA 43
```

Legend

- Key (16 byte SMPTE Universl Label): light green
- Length (BER long form): purple
- Length (BER short form): light pink
- Tag (Local Set Identifier): blue
- Value (Interpretation depends on tag data type): orange
- Grid Patterned Colors (Denotes nested metadata Set): grid (dotted) pattern

The next example, which uses the values in Table 6, illustrates the details of a hypothetical “Dynamic Only” MISMMS data packet.

Table 6: Example “Dynamic Only” MISMMS Packet Data

| Tag # | Tag Name | Value | Interpretation | TLV Hex Bytes |
|---|---|---|---|---|
| 2 | Precision Time Stamp | 1,231,798,102,000,000 microseconds | Mon Jan 12 2009 22:08:22 | 02 08 00 04 60 50 58 4E 01 80 |
| 5 | Platform Heading Angle | 0x71C2 | 159.974 Degrees | 05 02 71 C2 |
| 6 | Platform Pitch Angle (Short) | 0xFD3D | -0.4315 Degrees | 06 02 FD 3D |
| 7 | Platform Roll Angle (Short) | 0x08B8 | 3.406 Degrees | 07 02 08 B8 |
| 13 | Sensor Latitude | 0x5595B66D | 60.17682297 Degrees | 0D 04 55 95 B6 6D |
| 14 | Sensor Longitude | 0x5B5360C4 | 128.42675904 Degrees | 0E 04 5B 53 60 C4 |
| 15 | Sensor True Altitude (MSL) | 0xC221 | 14190.7 Meters | 0F 02 C2 21 |
| 16 | Sensor Horizontal FoV | 0xCD9C | 144.571 Degrees | 10 02 CD 9C |
| 17 | Sensor Vertical FoV | 0xD917 | 152.644 Degrees | 11 02 D9 17 |
| 18 | Sensor Rel. Az. Angle | 0x724A0A20 | 160.71921144 Degrees | 12 04 72 4A 0A 20 |
| 19 | Sensor Rel. El. Angle | 0x87F84B86 | -168.79232483 Degrees | 13 04 87 F8 4B 86 |
| 20 | Sensor Rel. Roll Angle | 0x7DC55ECE | 176.86543765 Degrees | 14 04 7D C5 5E CE |
| 21 | Slant Range | 0x03830926 | 68590.983 Meters | 15 04 03 83 09 26 |
| 22 | Target Width | 0x1281 | 722.8 Meters | 16 02 12 81 |
| 23 | Frame Center Lat. | 0xF101A229 | -10.54238863 Degrees | 17 04 F1 01 A2 29 |
| 24 | Frame Center Lon. | 0x14BC082B | 29.15789012 Degrees | 18 04 14 BC 08 2B |
| 25 | Frame Center El. (MSL) | 0x34F3 | 3216.0 Meters | 19 02 34 F3 |
| 65 | UAS Local Set Version | 0x06 | MISB Standard 0601.6 | 41 01 06 |
| 1 | Checksum | 0xC850 | 0xC850 | 01 02 C8 50 |

Again, the TLV value bytes are appended end-to-end; together they form the value portion of the encompassing KLV packet. In the “Dynamic Only” packet, there are 97 bytes of TLV data – encoded as the length of the KLV packet. As the length of 97 bytes is less than 128 bytes, it is encoded as the BER short form length of 0x61.

The Local Set begins with its 16-byte UL key, followed by the length 0x61, which is then followed by all the TLV value bytes as listed above in order. In hex, the complete MISMMS “Dynamic Only” example KLV packet is:

```
06 0E 2B 34 02 0B 01 01 0E 01 03 01 01 00 00 00
61 02 08 00 04 60 50 58 4E 01 80 05 02 71 C2 06
02 FD 3D 07 02 08 B8 0D 04 55 95 B6 6D 0E 04 5B
53 60 C4 0F 02 C2 21 10 02 CD 9C 11 02 D9 17 12
04 72 4A 0A 20 13 04 87 F8 4B 86 14 04 00 00 00
00 15 04 03 83 09 26 16 02 12 81 17 04 F1 01 A2
29 18 04 14 BC 08 2B 19 02 34 F3 41 01 06 01 02
C8 50
```

Legend

- Key (16 byte SMPTE Universl Label): light green
- Length (BER long form): purple
- Length (BER short form): light pink
- Tag (Local Set Identifier): blue
- Value (Interpretation depends on tag data type): orange

# 10 Annex D: Basis for Metadata Inclusion in MISMMS – Informative

This section provides motivation for the inclusion of each item in the MISMMS. Please note that the metadata tags enumerated herein constitute the threshold set to meet two primary ISR missions. Users needing additional elements are encouraged to use other elements from MISB ST 0601 or other MISB metadata elements as appropriate.

In general, the MISMMS is designed to support two primary missions: Discovery & Retrieval (D&R) and ISR Situational Awareness (ISR SA). The D&R mission refers to storage of motion imagery within an archive, and in particular, subsequent search and access to the archive.

SA is defined in the MISP as “the human perception of the elements of the operational environment in the context of the forces, space, and time, the comprehension of their meaning, and the projection of their status in the near future.” SA is subjective; it is perhaps easier to define SA Product in objective terms.

The MISP defines a SA Product as “a concise, transportable summary of the state of friendly and enemy elements conveyed through information such as Motion Imagery, imagery, or other data that can contribute to the development of SA either locally or at some distant node.”

In other words, a SA Product is a collection of those objectives – quantifiable pieces of information that assist someone in the formations of SA for a specific context. In the context of ISR, a SA Product answers three questions regarding an asset: who is it, where is it, and what is it doing?

Elements from MISB ST 0601 have been chosen as needed to support the D&R and ISR SA tasks as follows:

Checksum (Tag 1): This element is necessary to ensure the data contained in an instance of a MISB ST 0601 Local Set has not been corrupted during transmission.

Precision Time Stamp (Tag 2): This element is necessary as it addresses the time for which all other elements in a given MISB ST 0601 Local Set are valid – geospatial information must have a temporal component.

Mission ID (Tag 3): This element is the basis for many D&R queries.

Platform Heading / Platform Pitch / Platform Roll (Tags 5, 6 | 90, 7 | 91): These elements define the orientation of an airborne asset which can be used to predict its future position.

Platform Designation (Tag 10): This element supports both D&R queries and ISR SA regarding the presence of friendly assets.

Image Source Sensor (Tag 11): As some platforms have multiple sensors, information that addresses what sensor is used refines the information included under Platform Designation (Tag 10) for both D&R and ISR SA.

Image Coordinate System (Tag 12): Defines a reference coordinate system for all measurements related to the Earth. It is an essential element in clearly defining measurements such as Latitude, Longitude, elevation, altitude, etc. fundamental to D&R and ISR SA.

Sensor Latitude / Sensor Longitude / Sensor Altitude (Tags 13, 14, 15 | 75): These elements define the position of the active sensor and can be combined with Platform heading / pitch / roll to predict future position of the sensor.

Sensor Field of View-Horizontal / Sensor Field of View-Vertical (Tags 16-17): These two elements define the size of the field-of-view of the active sensor. This supports D&R queries related to pixel resolution of the target and ISR SA by assisting in defining the area of interest for the asset.

Sensor Relative Azimuth Angle / Sensor Relative Elevation Angle / Sensor Relative Roll Angle (Tags 18-20): When combined with Platform Heading, Pitch & Roll, these elements comprise the composite pointing vector of the sensor which is a key component of ISR SA.

Slant Range (Tag 21): While this information can be derived from other information regarding the position & orientation of the platform along with a terrain model, an independent measurement of the slant range to the target (or image center) improves confidence in the position knowledge.

Target Width (Tag 22): This element provides the analyst a quick reference of scale and ground sample distance of a primary object within the viewing area. It is a component of ISR SA.

Frame Center Latitude / Frame Center Longitude / Frame Center Elevation (Tags 23, 24, 25 | 78): These elements give the center-point of the imaging sensor (which, when combined with the field-of-view elements, define where the sensor is pointing) in support of D&R queries.

Security Classification / Classifying Country & Releasing Instructions Country Coding Method / Classifying Country / SCI-SHI Information / Caveats / Releasing Instructions (Tags 48/1-48/6): Classification information is required by D&R systems to determine appropriate distribution limits. This is a fundamental requirement of all data in the NSG.

Object Country Coding Method / Object Country Code (48/12-48/13): These elements support a fundamental D&R query regarding the country being imaged and support security / releasability decisions.

Security Metadata Version (Tag 48/22): This element ensures proper interpretation of the elements that comprise the Security Local Set as the standard evolves over time.

UAS Local Set Version (Tag 65): This element ensures proper interpretation of the elements that comprise the UAS Datalink Local Set as the standard evolves over time.

Motion Imagery Core Identifier (Tag 94): This element is a unique identifier for sensor data that is based upon the composite collection system and enables enterprise management of motion imagery data.
