MISB ST 0902.8

STANDARD

Motion Imagery Sensor Minimum Metadata Set 1 November 2018

1 Scope

This standard defines the Motion Imagery Sensor Minimum Metadata Set (MISMMS), which enables the basic capabilities of Situational Awareness Discovery & Retrieval and Cross￾Domain,, Dissemination. In addition, the standard provides direction on the encoding of the MISMMS for low-bitrate to high-bitrate scenarios to support a transition path from analog

systems towards all digital Motion Imagery Systems.

2 References

[1] MISB ST 0601.14 UAS Datalink Local Set, Nov 2018.

[2] MISB MISP-2019.1 Motion Imagery Standards Profile, Nov 2018.

[3] MISB ST 0107.3 KLV Metadata in Motion Imagery, Nov 2018.

[4] MISB ST 0102.12 Security Metadata Universal and Local Sets for Motion Imagery Data,

Jun 2017.

[5] MISB ST 1204.1 Motion Imagery Identification System (MIIS) Core Identifier, Oct 2013.

3 Acronyms

BER Basic Encoding Rules

D&R Discovery and Retrieval

FoV Field of View

HAE Height above Ellipsoid

ISR Intelligence, Surveillance and Reconnaissance

KLV Key Length Value

LS Local Set

MFG Metadata Focus Group

MIIS Motion Imagery Identification System

MISB Motion Imagery Standards Board

MISMMS Motion Imagery Sensor Minimum Metadata Set

MISP Motion Imagery Standards Profile

1 November 2018 Motion Imagery Standards Board 1

| MSL | Mean Sea Level |
|---|---|
| NATO | North Atlantic Treaty Organization |
| NGA | National Geospatial-Intelligence Agency |
| NIAT | NGA Interoperability Action Team |
| NTB | NITF Technical Board |
| SA | Situational Awareness |
| STANAG | Standardization Agreement |
| TLV | Tag Length Value |
| UL | Universal Label |

USCENTCOM U.S. Central Command

| ZLI | Zero-Length | Item ( See MISB ST 0601 [1]) |
|---|---|---|
| 4 Revision | History |  |
| Revision | Date | Summary of Changes |
|  | • | Added Requirement -05 |
| ST 0902.8 | 11/01/2018 • | Deprecated Requirement -02 as it is a definition |
|  | • | Changed “element” to “item” consistent with SMPTE usage |
| 5 | Introduction, Intent |, and General Implementation Rules |

The MISMMS consists of metadata items from MISB ST 0601 [1], which enable the baseline functionality required for Discovery & Retrieval / Cross-Domain Dissemination of source imagery and the Situational Awareness Product for ISR mission accomplishment as defined in

the MISP [2].

The initial MISMMS was developed jointly by the Motion Imagery Standards Board (MISB), the NITFS Technical Board (NTB), the NGA Interoperability Action Team (NIAT), and the Metadata Focus Group (MFG) with additional subject matter expert input from U.S. Central

Command (USCENTCOM), NATO Standardization Agreement (STANAG) 4586, and the

Unmanned Systems community.

As Motion Imagery systems begin to migrate to all-digital architectures, some systems in transition still require the consistent preservation of certain analog system characteristics. This standard identifies how to encode the source-derived metadata from a Motion Imagery sensor

into a MISB KLV metadata set.

6 Motion Imagery Sensor Minimum Metadata Set

The MISMMS is a Local Set profile of MISB ST 0601 with all items mandatory unless

conditionally dependent. Thus, it uses the ST 0601 Universal Label.

The KLV Universal Label for the MISMMS is defined in MISB ST 0601 [1] as :

1 November 2018 Motion Imagery Standards Board 2

06.0E.2B.34.02.0B.01.01.0E.01.03.01.01.00.00.00 ( CRC 56773)

Requirement(s)

ST 0902.3-01 All metadata shall be expressed in accordance with MISB ST 0107 [3].

ST 0902.3-03 The items of the MISMMS as defined in MISB ST 0902 Table 1 shall be populated in accordance with MISB ST 0601 requirements.

ST 0902.3-04 All metadata items contained in the MISMMS shall be reported no less than once every thirty (30) seconds under all circumstances.

ST 0902.8-05 No Zero-Length items (ZLI) shall be used to meet minimum reporting

requirements.

Please refer to MISB ST 0601 [1] for further information on each item. Of interest, Tag 48 of MISB ST 0601 contains a nested security Local Set as defined in MISB ST 0102 [4]. Please refer to MISB ST 0102 for further information regarding each security item. MISB ST 0601 also contains reference to a Motion Imagery Core Identifier as defined in MISB ST 1204 [5]. Please refer to MISB ST 1204 for further information regarding the Motion Imagery Core Identifier.

Note 1 : Platform Pitch Angle ( Tag 6 | 90), Platform Roll Angle ( Tag 7 | 91), Sensor True Altitude as MSL ( Tag 15) | Sensor Ellipsoid Height as HAE ( Tag 75) | Sensor Ellipsoid Height Extended as HAE ( Tag 104), Frame Center Elevation as MSL ( Tag 25) | Frame Center Height Above Ellipsoid ( Tag 78), and Target Width ( Tag 22) | Target Width Extended ( Tag 96) are governed by an “inclusive or” within MISB ST 0601 with one exception. The use of Tag 75 and Tag 104 is governed by an “exclusive OR.”

Note 2 : A Motion Imagery Core Identifier ( Tag 94) is required as the Motion Imagery data is disseminated from the “system”, where the system is the composite of the platform and

control station (i.e. Ground Control Station).

6.1 Local Set Items

The MISMMS items are listed in Table 1, which has the following column designations :

• Tag is the Local Set tag number for the item in the MISMMS.

• Item Name is the name of the item.

• Range & Units provides the allowed range and unit of specification for the value.

• Type indicates the data type of the item.

• Length is the number of bytes allowed for the item.

1 November 2018 Motion Imagery Standards Board 3

Table 1 : Summary of MISMMS Items

Length

| Tag | Item Name | Range & Units | Type |
|---|---|---|---|
|  |  |  | (Bytes) |
| 1 | Checksum | None | uint16 2 |
| 2 | Precision Time Stamp | Microseconds | uint64 8 |
| 3 | Mission ID | None | ISO 646 variable |
| 5 | Platform Heading Angle | 0-360 Degrees | uint16 2 |
| 6 \| | Platform Pitch Angle (Short) | +/- 20 Degrees | int16 2 |
| 90 | Platform Pitch Angle (Full) | +/- 90 Degrees | int32 4 |
| 7 \| | Platform Roll Angle (Short) | +/- 50 Degrees | int16 2 |
| 91 | Platform Roll Angle (Full) | +/- 90 Degrees | int32 4 |
| 10 | Platform Designation | None | ISO 646 variable |
| 11 | Image Source Sensor | None | ISO 646 variable |
| 12 | Image Coordinate System | None | ISO 646 variable |
| 13 | Sensor Latitude | +/- 90 Degrees | int32 4 |
| 14 | Sensor Longitude | +/- 180 Degrees | int32 4 |
| 15 \| | Sensor True Altitude (MSL) | -900 to 19000m | uint16 2 |
| 75 \| | Sensor Ellipsoid Height (HAE) | -900 to 19000m | uint16 2 |
| 104 | Sensor Ellipsoid Height Extended (HAE |) -900 to 40000m | IMAPB 2 |
| 16 | Sensor Horizontal FoV | 0 to 180 Degrees | uint16 2 |
| 17 | Sensor Vertical FoV | 0 to 180 Degrees | uint16 2 |
| 18 | Sensor Relative Azimuth Angle | 0 to 360 Degrees | uint32 4 |
| 19 | Sensor Relative Elevation Angle | +/- 180 Degrees | int32 4 |
| 20 | Sensor Relative Roll Angle | 0 to 360 Degrees | uint32 4 |
| 21 | Slant Range | 0 to 5000000 m | uint32 4 |
| 22 \| | Target Width | 0 to 10000 m | uint16 2 |
| 96 | Target Width Extended | 0 to 1500000 m | IMAPB 3 |
| 23 | Frame Center Latitude | +/- 90 Degrees | int32 4 |
| 24 | Frame Center Longitude | +/- 180 Degrees | int32 4 |
| 25 \| | Frame Center Elevation (MSL) |  |  |
|  |  | -900 to 19000 m | uint16 2 |
| 78 | Frame Center Height Above Ellipsoid | (HAE) |  |
| 48/1 | Security Classification | Look Up Table | uint8 1 |

Classifying Country & Releasing Instructions

| 48/2 | Look Up Table | uint8 | 1 |
|---|---|---|---|
|  | Country Coding Method |  |  |
| 48/3 | Classifying Country None | ISO 646 | variable |
| 48/4 | Security-SCI/SHI Information None | ISO 646 | variable |
| 48/5 | Caveats None | ISO 646 | variable |
| 48/6 | Releasing Instructions None | ISO 646 | variable |
| 48/12 | Object Country Coding Method Look Up Table | uint8 | 1 |
| 48/13 | Object Country Codes None | UTF-16 | variable |
| 48/22 | Security Metadata Version Integer | uint16 | 2 |
| 65 | UAS Local Set Version Integer | uint8 | 1 |
| 94 | Motion Imagery Core Identifier None | binary | 50 |
| 1 November | 2018 Motion Imagery Standards Board |  | 4 |

7 Deprecated Requirements

Requirement -02 was removed as per recent MISB practices where Universal Keys are defined within a dictionary and thus not considered requirements.

Requirement

| ST 0902.3-02 | The MISMMS shall use MISB ST 0601 [1] Local Set 16-byte Universal Key |
|---|---|
| (Deprecated) | ( 06.0E.2B.34.02.0B.01.01.0E.01.03.01.01.00.00.00 ( CRC 56773)) for its |
|  | implementation. |
| 8 Annex | A : Recommended MISMMS Item Frequency – Informative |

This section addresses the presence and frequency of individual data items within the MISMMS.

Some metadata items change more frequently than others. When sending metadata in bandwidth￾constrained environments, it is more efficient to transmit only the dynamic items (“Dynamic”)

with frequently changing values, and the less dynamic, constant items (“Constant”) less often. This section provides guidance in the form of a “recommended update rate” for each item of the

MISMMS.

Some of the security metadata items may be omitted as they are conditional. Further direction on this topic is found in MISB ST 0102.

Recommendation Summary for MISMMS Frequency ( see Table 2) :

1. Include Tags 3, 10, 11, 12, 48 sub-tags & 94 once every 10 seconds

2. Include all other items as often as possible, within the available bandwidth and up to the frame rate

The rate at which “Dynamic” items update varies depending on the available bandwidth of the

system, and how frequently the metadata values are refreshed. In some systems, it is desirable to have metadata updated for each Motion Imagery frame. For a specific worked example, see Annex B : Example MISMMS Item Rate Calculations – Informative.

Note : It is not mandatory that each metadata packet contain every metadata item; Annex B demonstrates the viability of transmitting the MISMMS in a bandwidth-constrained

environment. Given enough bandwidth other metadata packet configurations (e.g. more packets each containing fewer items) are permissible.

1 November 2018 Motion Imagery Standards Board 5

Table 2 : Recommended Update Rate for MISMMS Items

| Max Length | Rec Update |
|---|---|
| Tag # Item Name |  |
| (Bytes) | Interval |
| 1 Checksum 2 | Fast |
| 2 Precision Time Stamp 8 | Fast |
| 3 Mission ID 127 | 10 s |
| 5 Platform Heading Angle 2 | Fast |
| 6 \| Platform Pitch Angle (Short) 2 | Fast |
| 90 Platform Pitch Angle (Full) 4 | Fast |
| 7 \| Platform Roll Angle (Short) 2 | Fast |
| 91 Platform Roll Angle (Full) 4 | Fast |
| 10 Platform Designation 127 | 10 s |
| 11 Image Source Sensor 127 | 10 s |
| 12 Image Coordinate System 127 | 10 s |
| 13 Sensor Latitude 4 | Fast |
| 14 Sensor Longitude 4 | Fast |
| 15 \| Sensor True Altitude (MSL) 2 |  |
| 75 \| Sensor Ellipsoid Height (HAE) 2 | Fast |
| 104 Sensor Ellipsoid Height Extended (HAE) 2 |  |
| 16 Sensor Horizontal FoV 2 | Fast |
| 17 Sensor Vertical FoV 2 | Fast |
| 18 Sensor Relative Azimuth Angle 4 | Fast |
| 19 Sensor Relative Elevation Angle 4 | Fast |
| 20 Sensor Relative Roll Angle 4 | Fast |
| 21 Slant Range 4 | Fast |
| 22 \| Target Width 2 | Fast |
| 96 Target Width Extended 3 | Fast |
| 23 Frame Center Latitude 4 | Fast |
| 24 Frame Center Longitude 4 | Fast |
| 25 \| Frame Center Elevation (MSL) 2 |  |
|  | Fast |
| 78 Frame Center Height Above Ellipsoid (HAE) 2 |  |
| 48/1 Security Classification 1 | 10 s |

Classifying Country & Releasing Instructions Country

| 48/2 | 1 | 10 s |
|---|---|---|
|  | Coding Method |  |
| 48/3 | Classifying Country 6* | 10 s |
| 48/4 | Security-SCI/SHI Information 40* | 10 s |
| 48/5 | Caveats 32* | 10 s |
| 48/6 | Releasing Instructions 40* | 10 s |
| 48/12 | Object Country Coding Method 1 | 10 s |
| 48/13 | Object Country Codes 40* | 10 s |
| 48/22 | Security Metadata Version 2 | 10 s |
| 65 | UAS Local Set Version 1 | Fast |
| 94 | Motion Imagery Core Identifier 50 | 10 s |
| 39 | Tags Total 797 |  |
| 1 November | 2018 Motion Imagery Standards Board | 6 |

Note 3 : * indicates the maximum length for these items are only representative for the example

in Appendix B as MISB ST 0102 does not define a maximum length.

9 Annex B : Example MISMMS Item Rate Calculations – Informative

This section provides bandwidth budget calculations, assuming a 9600 bits-per-second channel

for metadata.

Calculations are presented for two general scenarios :

1. Scenario 1 : Assumes all content is always present in each KLV packet

2. Scenario 2 : Assumes content is present at variable rates in each KLV packet

9.1 Scenario 1 : All Content Always Present

9.1.1 Worst Case

In Table 3 the bandwidth usage for the worst case of this scenario is computed, which includes

all tags from Table 2 in every packet.

Table 3 : Example bandwidth computation for worst-case usage

Bytes Comments

| Local Set Key | 16 |
|---|---|
| Local Set Length | 2 |
| Tags | 39 Each Tag is 1 byte |
| Tag's Length | 39 Each Length is 1 byte |
| Tag's Payload | 797 See Table 2 |
| Security Set Tag | 1 1-byte Tag not shown in table |
| Security Set Length | 2 2 bytes, since Security Set is larger than 127 bytes ( per BER-OID) |
| Total | 896 Bytes |
| Total | 7168 Bits |
| Total with overhead | 8960 Serial overhead is 1 Start bit and 1 Stop bit per byte |

With serial transmission overhead, 8960 bits will enable one metadata update-per-second at 9600

bits-per-second.

9.1.2 Typical Case

As shown in Table 4, to reduce bandwidth a typical case will limit strings to 20 bytes, eliminate

Tags 6, 7, 15, 22, 25 & 75, and reduce the MIIS to 34 bytes.

1 November 2018 Motion Imagery Standards Board 7

Table 4 : Typical case for Scenario 1

| Tag # | Item Name | Length (Bytes) |
|---|---|---|
| 1 | Checksum | 2 |
| 2 | Precision Time Stamp | 8 |
| 3 | Mission ID | 20 |
| 5 | Platform Heading Angle | 2 |
| 90 | Platform Pitch Angle (Full) | 4 |
| 91 | Platform Roll Angle (Full) | 4 |
| 10 | Platform Designation | 20 |
| 11 | Image Source Sensor | 20 |
| 12 | Image Coordinate System | 20 |
| 13 | Sensor Latitude | 4 |
| 14 | Sensor Longitude | 4 |
| 104 | Sensor Ellipsoid Height Extended (HAE) | 2 |
| 16 | Sensor Horizontal FoV | 2 |
| 17 | Sensor Vertical FoV | 2 |
| 18 | Sensor Relative Azimuth Angle | 4 |
| 19 | Sensor Relative Elevation Angle | 4 |
| 20 | Sensor Relative Roll Angle | 4 |
| 21 | Slant Range | 4 |
| 96 | Target Width Extended | 3 |
| 23 | Frame Center Latitude | 4 |
| 24 | Frame Center Longitude | 4 |
| 78 | Frame Center Height Above Ellipsoid (HAE) | 2 |
| 48/1 | Security Classification | 1 |
| 48/2 | Classifying Country & Releasing Instructions Country | Coding Method 1 |
| 48/3 | Classifying Country | 6 |
| 48/4 | Security-SCI/SHI Information | 20 |
| 48/5 | Caveats | 20 |
| 48/6 | Releasing Instructions | 20 |
| 48/12 | Object Country Coding Method | 1 |
| 48/13 | Object Country Codes | 20 |
| 48/22 | Security Metadata Version | 2 |
| 65 | UAS Local Set Version | 1 |
| 94 | Motion Imagery Core Identifier | 34 |
| 33 | Tags | Total 269 |

Table 5 computes the bandwidth usage for the typical case of this scenario, which includes all

tags from Table 4 in every packet.

1 November 2018 Motion Imagery Standards Board 8

Table 5 : Example bandwidth computation for typical-case usage

Bytes Comments

| Local Set Key | 16 |
|---|---|
| Local Set Length | 2 |
| Tags | 33 Each Tag is 1 byte |
| Tag's Length | 33 Each Length is 1 byte |
| Tag's Payload | 269 See Table 4 |
| Security Set Tag | 1 1-byte Tag not shown in table |
| Security Set Length | 1 1 byte, since Security Set is smaller than 128 bytes ( per BER-OID) |
| Total | 355 Bytes |
| Total | 2840 Bits |
| Total with overhead | 3550 Serial overhead is 1 Start bit and 1 Stop bit per byte |

With serial transmission overhead, 3550 bits will enable two metadata updates per second at

9600 bits per second.

9.2 Scenario 2 : Content Present at Variable Rates

When the data rates of individual metadata items are varied, there are more options. One option is to define two different metadata lists : Dynamic and Constant. Dynamic metadata is the subset of metadata where values change frequently -- see items listed as “Fast” in the “Rec Update

Interval” column of Table 2. Constant metadata values do not change very often. From these

subsets, two KLV sets are used : Dynamic set and Dynamic-plus-Constant. The Dynamic set is transmitted as fast as possible, while the Dynamic-plus-Constant set is only transmitted every 30 seconds to meet the metadata requirements. The Dynamic-plus-Constant set for this example is the same as the Typical Case from Scenario 1, and for this example, will be the “Full-Typical” set. Table 6 lists the items of the Dynamic set for this example scenario.

1 November 2018 Motion Imagery Standards Board 9

Table 6 : Dynamic-only metadata

| Tag # | Item Name | Length (Bytes) |
|---|---|---|
| 1 | Checksum | 2 |
| 2 | Precision Time Stamp | 8 |
| 5 | Platform Heading Angle | 2 |
| 90 | Platform Pitch Angle (Full) | 4 |
| 91 | Platform Roll Angle (Full) | 4 |
| 13 | Sensor Latitude | 4 |
| 14 | Sensor Longitude | 4 |
| 104 | Sensor Ellipsoid Height Extended (HAE) | 2 |
| 16 | Sensor Horizontal FoV | 2 |
| 17 | Sensor Vertical FoV | 2 |
| 18 | Sensor Relative Azimuth Angle | 4 |
| 19 | Sensor Relative Elevation Angle | 4 |
| 20 | Sensor Relative Roll Angle | 4 |
| 21 | Slant Range | 4 |
| 96 | Target Width Extended | 3 |
| 23 | Frame Center Latitude | 4 |
| 24 | Frame Center Longitude | 4 |
| 78 | Frame Center Height Above Ellipsoid (HAE) | 2 |
| 65 | UAS Local Set Version | 1 |
| 19 | Tags | Total 64 |

Table 7 shows the bandwidth usage computed for the dynamic metadata in this scenario, which

includes all tags from Table 6 in every packet.

Table 7 : Example bandwidth computation for Dynamic metadata

Bytes Comments

| Local Set Key | 16 |
|---|---|
| Local Set Length | 1 |
| Tags | 19 Each Tag is 1 byte |
| Tag's Length | 19 Each Length is 1 byte |
| Tag's Payload | 64 See Table 2 |
| Total | 119 Bytes |
| Total | 952 Bits |
| Total with overhead | 1190 Serial overhead is 1 Start bit and 1 Stop bit per byte |
| 1 November 2018 | Motion Imagery Standards Board 10 |

Table 8 shows rate calculations for this scenario when the Full-Typical metadata combine with

the dynamic metadata over a 10 second interval.

Table 8 : Rate Calculation for Dynamic & Full-Typical Packets

| Description | Totals |
|---|---|
| Bits / 10 s Interval @ 9600 bits per second | 96000 |
| 1 packet of Full-Typical bits with Serial Overhead | 3550 |
| Bits Remaining after Full-Typical Packet | 92450 |
| Bits used with Dynamic packet with Serial Overhead | 1190 |
| Possible Dynamic Packets / 10 s Interval in Bits Remaining | 77.689 |
| Whole Dynamic Packets / 10 s Interval in Bits Remaining | 77 |

That is, in a 10-second interval, it is possible to send one Full-Typical MISMMS packet followed

by 77 Dynamic MISMMS packets.

If each packet ( Dynamic or Full-Typical) is separated by an equal-sized gap in time, then the rate calculations are as shown in Table 9 :

Table 9 : Rate Calculation for Dynamic & Full-Typical Packets with Gaps

| Description | Totals |
|---|---|
| Whole Dynamic Packets / 10 s | 77 |
| Bits / 10 s @ 9600 bits per second | 96000 |
| Bits / 77 Dynamic Packets | 91630 |
| Bits / 1 Full-Typical Packet | 3550 |
| Bits Remaining | 820 |
| # of Gaps | 78 |
| Bits / Gap | 10.51282 |
| Gap Time @ 9600 bps (microseconds) | 1095.085 |

This packet schedule repeats every 10 seconds ( see Figure 1) :

| Full￾Typical | Gap Dynamic | Gap | Dynamic … | Gap Dynamic Gap |
|---|---|---|---|---|
| 1 | 2 |  | 3 | 78 |
|  |  | 10 | Seconds |  |

Figure 1 : Time Interval with Dynamic Packets and Gaps

1 November 2018 Motion Imagery Standards Board 11

10 Annex C : Example MISMMS Data Packets – Informative

This section constructs hypothetical example MISMMS data packets and presents the encoding. All values shown in Table 10 are for illustration purposes only. The Tag-Length-Value (TLV) encoded data are shown as hexadecimal bytes. Programmer’s Note : The “Interpretation” of the

“Value” is shown in full precision, beyond an item’s resolution so programmers can verify they are using the right formulas. The number of significant digits expressed is determined as follows :

1) Based on the dynamic range and the precision needed the number of bits in an integer is

determined. 2) The precision, and the maximum value determines the type of value to use ( single

precision float vice double).

The type of value determines the number of digits ( 7 to 9 for single, 15 to 17 for double) needed. The use of 9 digits and 17 digits account for any rounding issues in the final digits. The final one

or two digits may be different for different complier optimization/hardware.

Table 10 : Example “Dynamic & Constant” MISMMS Packet Data

| Tag # | Item Name | Value | Interpretation | TLV | Hex Bytes |
|---|---|---|---|---|---|
|  |  | 1,231,798,102,000,000 | Mon Jan 12 2009 | 02 08 | 00 04 60 |
| 2 | Precision Time Stamp |  |  |  |  |
|  |  | microseconds | 22:08:22 | 50 58 | 4E 01 80 |

03 0A 4D 69 73

| 3 | Mission ID | Mission 12 | Mission 12 | 73 69 | 6F 6E 20 |
|---|---|---|---|---|---|
|  |  |  |  | 31 32 |  |
| 5 | Platform Heading Angle | 0x71C2 | 159.974365 Degrees | 05 02 | 71 C2 |
| 6 | Platform Pitch Angle (Short) | 0xFD3D | -0.431531724 Degrees | 06 02 | FD 3D |
| 7 | Platform Roll Angle (Short) | 0x08B8 | 3.40586566 Degrees | 07 02 | 08 B8 |

0A 08 50 72 65 10 Platform Designation Predator Predator 64 61 74 6F 72 0B 07 45 4F 20 11 Image Source Sensor EO Nose EO Nose 4E 6F 73 65 0C 0E 47 65 6F 64 65 74 69 63 12 Image Coordinate System Geodetic WGS84 Geodetic WGS84 20 57 47 53 38 34

| 60.176822966978335 | 0D 04 55 95 | B6 |
|---|---|---|
| 13 Sensor Latitude 0x5595B66D |  |  |
| Degrees | 6D |  |
| 128.42675904204452 | 0E 04 5B 53 | 60 |
| 14 Sensor Longitude 0x5B5360C4 |  |  |
| Degrees | C4 |  |
| 15 Sensor True Altitude (MSL) 0xC221 14190.7195 Meters | 0F 02 C2 21 |  |
| 16 Sensor Horizontal FoV 0xCD9C 144.571298 Degrees | 10 02 CD 9C |  |
| 17 Sensor Vertical FoV 0xD917 152.643626 Degrees | 11 02 D9 17 |  |
| 160.71921143697557 | 12 04 72 4A | 0A |
| 18 Sensor Rel. Az. Angle 0x724A0A20 |  |  |
| Degrees | 20 |  |

-168.79232483394085 13 04 87 F8 4B

| 19 | Sensor Rel. El. Angle | 0x87F84B86 |
|---|---|---|
|  |  | Degrees 86 |
|  |  | 176.86543764939194 14 04 7D C5 5E |
| 20 | Sensor Rel. Roll Angle | 0x7DC55ECE |
|  |  | Degrees CE |
|  |  | 68590.983298744770 15 04 03 83 09 |
| 21 | Slant Range | 0x03830926 |
|  |  | Meters 26 |
| 1 | November 2018 | Motion Imagery Standards Board 12 |

22 Target Width 0x1281 722.819867 Meters 16 02 12 81 -10.542388633146132 17 04 F1 01 A2

| 23 | Frame Center Lat. | 0xF101A229 |
|---|---|---|
|  |  | Degrees 29 |
|  |  | 29.157890122923014 18 04 14 BC 08 |
| 24 | Frame Center Lon. | 0x14BC082B |
|  |  | Degrees 2B |
| 25 | Frame Center El. (MSL) | 0x34F3 3216.03723 Meters 19 02 34 F3 |
| 48/1 | Security Classification | 0x01 UNCLASSIFIED// 01 01 01 |
|  | Classifying Country and |  |
|  |  | STANAG-1059 Three |
| 48/2 | Releasing Instructions | 0x07 02 01 07 |
|  |  | Letter |
|  | Country Coding Method |  |

03 05 2F 2F 55

| 48/3 | Classifying Country | //USA | //USA |
|---|---|---|---|
|  |  |  | 53 41 |
|  | Security-SCI/SHI |  |  |
| 48/4 |  | <None> | <None> |
|  | Information |  |  |
| 48/5 | Caveats | <None> | <None> |
| 48/6 | Releasing Instructions | <None> | <None> |
|  | Object Country Coding |  | STANAG-1059 Three |
| 48/12 |  | 0x07 | 0C 01 07 |
|  | Method |  | Letter |

0D 06 00 55 00

| 48/13 | Object Country Codes | \u0055\u0053\u0041 | USA |
|---|---|---|---|
|  |  |  | 53 00 41 |
|  |  |  | MISB Standard |
| 48/22 | Security Metadata Version | 10 | 16 02 00 0A |
|  |  |  | 0102.10 |
| 65 | UAS Local Set Version | 0x06 | MISB Standard 0601.6 41 01 06 |

5E 22 01 70 F5 92 F0 23 73 36 4A F8 AA 91 62

| Motion Imagery Core | C0 0F 2E B2 DA |
|---|---|
| 94 |  |
| Identifier | 16 B7 43 41 00 |

08 41 A0 BE 36 5B 5A B9 6A 36 45 1 Checksum 0xAA43 0xAA43 01 02 AA 43

The TLV bytes are appended end-to-end, and together form the Value portion of the

encompassing KLV packet. In the “Dynamic & Constant” packet, there are 210 bytes of TLV data – encoded as the length of the KLV packet. Lengths up to 127 bytes are encoded in BER

short form. Lengths of 128 bytes or greater are encoded in BER long form. In this case, 210 bytes are encoded as the BER long form length of 0x81D2.

The Local Set begins with its 16-byte UL key, followed by the length of the data set (0x81D2), which is then followed by the TLV triplets listed above in order. In hex, the complete MISMMS “Dynamic & Constant” example KLV packet is :

1 November 2018 Motion Imagery Standards Board 13

06 0E 2B 34 02 0B 01 01 0E 01 03 01 01 00 00 00

| 81 D2 02 08 00 04 60 50 58 4E 01 80 03 0A 4D 69 | Legend |
|---|---|
| 73 73 69 6F 6E 20 31 32 05 02 71 C2 06 02 FD 3D | Key ( 16 byte SMPTE Universl Label |
| 07 02 08 B8 0A 08 50 72 65 64 61 74 6F 72 0B 07 | Length ( BER long form) |
| 45 4F 20 4E 6F 73 65 0C 0E 47 65 6F 64 65 74 69 | Length ( BER short form) |
| 63 20 57 47 53 38 34 0D 04 55 95 B6 6D 0E 04 5B | Tag ( Local Set Identifier) |
| 53 60 C4 0F 02 C2 21 10 02 CD 9C 11 02 D9 17 12 | Value ( Interpretation depends on tag data type) |
| 04 72 4A 0A 20 13 04 87 F8 4B 86 14 04 00 00 00 | Grid Patterned Colors ( Denotes nested metadata Set) |

00 15 04 03 83 09 26 16 02 12 81 17 04 F1 01 A2 29 18 04 14 BC 08 2B 19 02 34 F3 30 1C 01 01 01 02 01 07 03 05 2F 2F 55 53 41 0C 01 07 0D 06 00 55 00 53 00 41 16 02 00 0A 41 01 06 5E 22 01 70 F5 92 F0 23 73 36 4A F8 AA 91 62 C0 0F 2E B2 DA 16 B7 43 41 00 08 41 A0 BE 36 5B 5A B9 6A 36 45 01 02 AA 43

The next example, which uses the values in Table 11, illustrates the details of a hypothetical “Dynamic Only” MISMMS data packet. Programmer’s Note : The “Interpretation” of the “Value” is shown in full precision, beyond an item’s resolution so programmers can verify they

are using the right formulas.

Table 11 : Example “Dynamic Only” MISMMS Packet Data

| Tag # | Item Name | Value | Interpretation | TLV Hex Bytes |
|---|---|---|---|---|
|  |  | 1,231,798,102,000,000 | Mon Jan 12 2009 | 02 08 00 04 60 |
| 2 | Precision Time Stamp |  |  |  |
|  |  | microseconds | 22:08:22 | 50 58 4E 01 80 |
| 5 | Platform Heading Angle | 0x71C2 | 159.974365 Degrees | 05 02 71 C2 |
|  | Platform Pitch Angle |  |  |  |
| 6 |  | 0xFD3D | -0.431531724 Degrees | 06 02 FD 3D |
|  | (Short) |  |  |  |
| 7 | Platform Roll Angle (Short |) 0x08B8 | 3.40586566 Degrees | 07 02 08 B8 |
|  |  |  | 60.176822966978335 | 0D 04 55 95 B6 |
| 13 | Sensor Latitude | 0x5595B66D |  |  |
|  |  |  | Degrees | 6D |
|  |  |  | 128.42675904204452 | 0E 04 5B 53 60 |
| 14 | Sensor Longitude | 0x5B5360C4 |  |  |
|  |  |  | Degrees | C4 |
| 15 | Sensor True Altitude (MSL |) 0xC221 | 14190.7195 Meters | 0F 02 C2 21 |
| 16 | Sensor Horizontal FoV | 0xCD9C | 144.571298 Degrees | 10 02 CD 9C |
| 17 | Sensor Vertical FoV | 0xD917 | 152.643626 Degrees | 11 02 D9 17 |
|  |  |  | 160.71921143697557 | 12 04 72 4A 0A |
| 18 | Sensor Rel. Az. Angle | 0x724A0A20 |  |  |
|  |  |  | Degrees | 20 |
|  |  |  | -168.79232483394085 | 13 04 87 F8 4B |
| 19 | Sensor Rel. El. Angle | 0x87F84B86 |  |  |
|  |  |  | Degrees | 86 |
|  |  |  | 176.86543764939194 | 14 04 7D C5 5E |
| 20 | Sensor Rel. Roll Angle | 0x7DC55ECE |  |  |
|  |  |  | Degrees | CE |
|  |  |  | 68590.983298744770 | 15 04 03 83 09 |
| 21 | Slant Range | 0x03830926 |  |  |
|  |  |  | Meters | 26 |
| 22 | Target Width | 0x1281 | 722.819867 Meters | 16 02 12 81 |
|  |  |  | -10.542388633146132 | 17 04 F1 01 A2 |
| 23 | Frame Center Lat. | 0xF101A229 |  |  |
|  |  |  | Degrees | 29 |
|  |  |  | 29.157890122923014 | 18 04 14 BC 08 |
| 24 | Frame Center Lon. | 0x14BC082B |  |  |
|  |  |  | Degrees | 2B |
| 25 | Frame Center El. (MSL) | 0x34F3 | 3216.03723 Meters | 19 02 34 F3 |
| 1 November | 2018 | Motion Imagery | Standards Board | 14 |

65 UAS Local Set Version 0x06 MISB Standard 0601.6 41 01 06 1 Checksum 0xC850 0xC850 01 02 C8 50

Again, the TLV triplets are appended end-to-end; together they form the Value portion of the encompassing KLV packet. In the “Dynamic Only” packet, there are 97 bytes of TLV data – encoded as the length of the KLV packet. As the length of 97 bytes is less than 128 bytes, it is encoded as the BER short form length of 0x61.

The Local Set begins with its 16-byte UL key, followed by the length 0x61, which is then followed by all the TLV triplets as listed above in order. In hex, the complete MISMMS “Dynamic Only” example KLV packet is :

06 0E 2B 34 02 0B 01 01 0E 01 03 01 01 00 00 00

| 61 02 08 00 04 60 50 58 4E 01 80 05 02 71 C2 06 | Legend |
|---|---|
| 02 FD 3D 07 02 08 B8 0D 04 55 95 B6 6D 0E 04 5B | Key ( 16 byte SMPTE Universl Label |
| 53 60 C4 0F 02 C2 21 10 02 CD 9C 11 02 D9 17 12 | Length ( BER long form) |
| 04 72 4A 0A 20 13 04 87 F8 4B 86 14 04 00 00 00 | Length ( BER short form) |
| 00 15 04 03 83 09 26 16 02 12 81 17 04 F1 01 A2 | Tag ( Local Set Identifier) |
| 29 18 04 14 BC 08 2B 19 02 34 F3 41 01 06 01 02 | Value ( Interpretation depends on tag data type) |

C8 50

11 Annex D : Basis for Metadata Inclusion in MISMMS – Informative

This section provides motivation for the inclusion of each item in the MISMMS. Please note that the metadata items enumerated herein constitute the threshold set to meet two primary ISR

missions : Discovery & Retrieval (D&R) and ISR Situational Awareness ( ISR SA). The D&R mission refers to storage of Motion Imagery within an archive, and subsequent search and access to the archive. Users needing additional items are encouraged to use other items from MISB ST

0601, or other MISB metadata items as appropriate.

ISR SA as defined in the MISB is “the human perception of the elements of the operational

environment in the context of the forces, space, and time, the comprehension of their meaning, and the projection of their status in the near future.” SA is subjective; it is perhaps easier to

define a SA Product in objective terms.

The MISB defines a SA Product as “a concise, transportable summary of the state of friendly and

enemy elements conveyed through information such as Motion Imagery, imagery, or other data that can contribute to the development of SA either locally or at some distant node.”

In other words, a SA Product is a collection of those objectives – quantifiable pieces of information that assist someone in the formations of SA for a specific context. In the context of

ISR, a SA Product answers three questions regarding an asset : who is it, where is it, and what is it doing ?

Items from MISB ST 0601 have been chosen as needed to support the D&R and ISR SA tasks as

follows :

Checksum ( Tag 1) : This item is necessary to ensure the data contained in an instance of a MISB ST 0601 Local Set has not been corrupted during transmission.

1 November 2018 Motion Imagery Standards Board 15

Precision Time Stamp ( Tag 2) : This item is necessary as it indicates the time for which all other items in each MISB ST 0601 Local Set are valid – geospatial information must have a

temporal component.

Mission ID ( Tag 3) : This item is the basis for many D&R queries.

Platform Heading / Platform Pitch / Platform Roll ( Tags 5, 6 | 90, 7 | 91) : These items define the orientation of an airborne asset, which can be used to predict its future position.

Platform Designation ( Tag 10) : This item supports both D&R queries and ISR SA regarding

the presence of friendly assets.

Image Source Sensor ( Tag 11) : As some platforms have multiple sensors, information that indicates what sensor is used refines the information included under Platform Designation ( Tag

10) for both D&R and ISR SA.

Image Coordinate System ( Tag 12) : Defines a reference coordinate system for all measurements related to the Earth. It is an essential item for clearly defining measurements such

as latitude, longitude, elevation, altitude, etc. fundamental to D&R and ISR SA.

Sensor Latitude / Sensor Longitude / Sensor Altitude ( Tags 13, 14, 15 | 75) : These items define the position of the active sensor and can be combined with Platform heading / pitch / roll

to predict future position of the sensor.

Sensor Field of View-Horizontal / Sensor Field of View-Vertical ( Tags 16-17) : These two items define the size of the field-of-view of the active sensor. This supports D&R queries related to pixel resolution of the target, and ISR SA by assisting in defining the area of interest for the

asset.

Sensor Relative Azimuth Angle / Sensor Relative Elevation Angle / Sensor Relative Roll

Angle ( Tags 18-20) : When combined with Platform heading / pitch / roll, these items comprise

the composite pointing vector of the sensor, which is a key component of ISR SA.

Slant Range ( Tag 21) : While this information can be derived from other information regarding

the position & orientation of the platform along with a terrain model, an independent measurement of the slant range to the target ( or image center) improves confidence in the

position knowledge.

Target Width ( Tag 22) : This item provides the analyst a quick reference of scale and ground

sample distance of a primary object within the viewing area. It is a component of ISR SA.

Frame Center Latitude / Frame Center Longitude / Frame Center Elevation ( Tags 23, 24, 25 | 78) : These items give the center-point of the imaging sensor (which, when combined with the field-of-view items, define where the sensor is pointing) in support of D&R queries.

Security Classification / Classifying Country & Releasing Instructions Country Coding Method / Classifying Country / SCI-SHI Information / Caveats / Releasing Instructions

( Tags 48/1-48/6) : Classification information is required by D&R systems to determine appropriate distribution limits. This is a fundamental requirement of all data in the NSG.

Object Country Coding Method / Object Country Code ( Tags 48/12-48/13) : These items

support a fundamental D&R query regarding the country being imaged and support security /

releasability decisions.

1 November 2018 Motion Imagery Standards Board 16

Security Metadata Version ( Tag 48/22) : This item ensures proper interpretation of the items that comprise the security Local Set as the standard evolves over time.

UAS Local Set Version ( Tag 65) : This item ensures proper interpretation of the items that comprise the UAS Datalink Local Set as the standard evolves over time.

Motion Imagery Core Identifier ( Tag 94) : This item is a unique identifier for sensor data based upon the composite collection system and enables enterprise management of Motion

Imagery data.

1 November 2018 Motion Imagery Standards Board 17