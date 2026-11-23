MISB ST 0806.4

STANDARD

27 February 2014 Remote Video Terminal Metadata Set

# 1 Scope

This MISB Standard defines a Local Set (LS) to be used with a Remote Video Terminal (RVT), provides the relationship between the RVT LS and other relevant standards, and gives implementation guidance for the RVT LS.

# 2 References

## 2.1 Normative References

The following references and the references contained therein are normative.

[1] MISB ST 0603.2 Common Time Reference for Digital Motion Imagery Using Coordinated Universal Time (UTC), Feb 2014

[2] ISO/IEC 13818-1:2013, Information Technology – Generic coding of moving pictures and associated audio information: Systems

[3] MISB ST 0107.2 Bit and Byte Order for Metadata in Motion Imagery Files and Streams, Feb 2014

[4] SMPTE RP 210v13:2012, Metadata Element Dictionary

[5] MISB ST 0807.13 MISB KLV Metadata Dictionary, Feb 2014

[6] SMPTE ST 336:2007, Data Encoding Protocol Using Key-Length-Value

## 2.2 Informative References

[7] MISB ST 0601.7 UAS Datalink Local Metadata Set, Feb 2014

[8] MISB TRM 1006 Key-Length-Value (KLV) Users Guide, Jul 2012

[9] DMA TM8358.1 (Ed. 1), Datums, Ellipsoids, Grids, and Grid Reference Systems, Sep 1990

[10] MIL-STD-2525B: Detection of Common Warfighting Symbology, Sep 2011

# 3 Revision History

| Revision | Date | Summary of Changes |
|---|---|---|
| ST 0806.4 | 02/27/2014 | New UL keys in the RVT LS for Tags 3, 4, 7, 18, 19, 20, 12<br>Tag 10 name changed to agree with dictionary name (another “Video File Format” key exists)<br>AOI Local Set Tags 2-5 renamed per RP210 usage<br>EARS format for requirements<br>Updated references<br>Text common to KLV usage deleted in lieu of reference to similar in MISB TRM 1006<br>Document renamed |

# 4 Acronyms

|  |  |
|---|---|
| AOI | Area of Interest |
| KLV | Key-Length-Value |
| LS | Local Set |
| MISB | Motion Imagery Standards Board |
| OSRVT | One System Remote Video Terminal |
| POI | Point of Interest |
| ROVER | Remote Operations Video Enhanced Receiver |
| RVT | Remote Video Terminal |
| US | Universal Set |

# 5 Introduction

Users of the ROVER (Remotely Operated Video Enhanced Receiver) require additional metadata elements unique to its mission needs than are specified in MISB ST 0601 [7]. The purpose of this Standard is to formalize a method of communicating with a Remote Video Terminal (ROVER or other related programs, such as the One System Remote Video Terminal (OSRVT)), and create a configuration-managed metadata standard to meet its needs with minimal impacts to ST 0601 users.

The RVT Local Set (LS) defined in this Standard can stand as an independent local set, or be embedded within other metadata sets. This provides the ability for system designers to produce or receive one, or a mix of metadata elements from other sets based upon program requirements.

# 6 LS Packet Structure

In using KLV, Local Sets are more bit-efficient than specifying individual keys and afford a great deal of flexibility in implementation, allowing users to tailor implementations to their specific and changing needs. For the construction of a local set please refer to [8].

| Requirement |  |
|---|---|
| ST 0806.4-01 | Each independent RVT Local Set shall contain a Precision Time Stamp in accordance with MISB ST 0603 [1]. |
| ST 0806.4-02 | The Precision Time Stamp shall be the first element in the Local Set. |
| ST 0806.4-03 | Each independent RVT Local Set shall contain a checksum in accordance with ISO/IEC 13818-1 [2]. |
| ST 0806.4-04 | The Local Set checksum shall be the last element in the Local Set. |

The Precision Time Stamp represents the time of birth of the metadata within the LS packet. Any combination of metadata items can be included in a RVT Local Set packet. Aside from the Precision Time Stamp and the checksum items within the RVT LS can be arranged in any order.

## 6.1 Bit and Byte Ordering

| Requirement |  |
|---|---|
| ST 0806.4-05 | All metadata shall be expressed in accordance with MISB ST 0107 [3]. |

# 7 RVT Local Set Conventions

This section defines the RVT Local Set (LS). The tags that are supported in this LS are defined and mapped to metadata items in the SMPTE KLV Metadata Dictionary (SMPTE RP 210 [4]) or in MISB ST 0807 [5].

## 7.1 RVT LS Universal Keys

| Requirement |  |
|---|---|
| ST 0806.4-06 | The 16-byte Universal Key for the RVT Local Set shall be 06.0E.2B.34.02.0B.01.01.0E.01.03.01.02.00.00.00 (CRC 17945) |

Please note that an earlier version of the RVT LS Universal Key is not within the MISB Metadata Dictionary domain and was never registered with SMPTE; therefore, it should not be used.

The elements of the RVT LS are defined in Table 8-1. Subordinate data structures for the RVT LS are described in Section 7.3 with element listings listed in Table 8-2, Table 8-3 and Table 8-4.

## 7.2 RVT LS Tag Format

Tag numbers listed within this document are all in decimal unless preceded by “0x” or “0b” for hexadecimal or binary (respectively).

## 7.3 RVT LS Subordinate Sets

The RVT LS makes use of three smaller Local Sets. These embedded local sets are:

1. Point of Interest (POI) Local Set
2. Area of Interest (AOI) Local Set
3. User Defined Local Set

### 7.3.1 Point of Interest (POI) Local Set

The elements of the Point of Interest Local Set are defined in Table 8-2.

| Requirement |  |
|---|---|
| ST 0806.4-07 | The 16-byte Universal Key for the Point of Interest (POI) Local Set shall be 06.0E.2B.34.02.0B.01.01.0E.01.03.01.0C.00.00.00 (CRC 58435) |
| ST 0806.4-08 | A Point of Interest (POI) Local Set shall contain the POI/AOI Number (Tag ID 1) item. |
| ST 0806.4-09 | A Point of Interest (POI) Local Set shall contain the POI Latitude (Tag ID 2) item. |
| ST 0806.4-10 | A Point of Interest (POI) Local Set shall contain the POI Longitude (Tag ID 3) item. |
| ST 0806.4-11 | An item within a Point of Interest (POI) Local Set shall be present only one time within the POI LS. |

POI items with Tag ID 4 through Tag ID 10 are optional.

### 7.3.2 Area of Interest (AOI) Local Set

The elements of the Area of Interest Local Set are defined in Table 8-3.

| Requirement |  |
|---|---|
| ST 0806.4-12 | The 16-byte Universal Key for the Area of Interest (AOI) Local Set shall be 06.0E.2B.34.02.0B.01.01.0E.01.03.01.0D.00.00.00 (CRC 37623) |
| ST 0806.4-13 | An Area of Interest (AOI) Local Set shall contain the POI/AOI Number (Tag ID 1) item. |
| ST 0806.4-14 | An Area of Interest (AOI) Local Set shall contain the Corner Latitude Point 1 (Decimal Degrees) (Tag ID 2) item. |
| ST 0806.4-15 | An Area of Interest (AOI) Local Set shall contain the Corner Longitude Point 1 (Decimal Degrees) (Tag ID 3) item. |
| ST 0806.4-16 | An Area of Interest (AOI) Local Set shall contain the Corner Latitude Point 3 (Decimal Degrees) (Tag ID 4) item. |
| ST 0806.4-17 | An Area of Interest (AOI) Local Set shall contain the Corner Longitude Point 3 (Decimal Degrees) (Tag ID 5) item. |
| ST 0806.4-18 | An Area of Interest (AOI) Local Set shall contain the POI/AOI Type (Tag ID 6) item. |
| ST 0806.4-19 | An item within an Area of Interest (AOI) Local Set shall be present only one time within the AOI LS. |

AOI items with Tag ID 7 through Tag ID 10 are optional.

### 7.3.3 User Defined Local Set

The elements of the User Defined Local Set are defined in Table 8-4.

| Requirement |  |
|---|---|
| ST 0806.4-20 | The 16-byte Universal Key for the User Defined Local Set shall be 06.0E.2B.34.02.0B.01.01.0E.01.03.01.0F.00.00.00 (CRC 32671) |
| ST 0806.4-21 | A User Defined Local Set shall contain the Numeric ID for Data (Tag ID 1) as the first element in the User Defined Data LS. |
| ST 0806.4-22 | A User Defined Local Set shall contain the User Data (Tag ID 2) as the second element in the User Defined Data LS. |
| ST 0806.4-23 | Only the Numeric ID for Data (Tag ID 1) and the User Data (Tag ID 2) shall be present in a User Defined LS. |
| ST 0806.4-24 | An item within a User Defined Data Local Set shall be present only one time within the User Defined LS. |

The content of the User Defined LS will be determined out of field. This LS may not be used to substitute for any valid KLV element or construct defined in any other MISB specification.

### 7.3.4 Subordinate Set Example

Figure 7-1 depicts an example RVT LS packet containing two Point of Interest local sets.

Figure 7-1: Example RVT Subordinate Set Packet

Note how both POI LS packets contain all mandatory items, and POI 2 adds information for MIL-STD-2525B [10] symbology. Also, note that the RVT LS packet contains the mandatory timestamp and checksum metadata items.

The other Subordinate Sets closely mimic the RVT LS packet structure shown in Figure 7-1 according to the guidelines portrayed in Section 7.3.

## 7.4 RVT LS Duplication of Tags

| Requirement |  |
|---|---|
| ST 0806.4-25 | Any metadata item that is not a subordinate set item shall appear only once within a RVT Local Set. |

For instance, Tag ID 3 representing Platform True Airspeed can only appear once in an RVT LS packet whereas Tag ID 12, representing a Point of Interest Local Set instance, can appear multiple times to convey information for multiple points of interest.

# 8 RVT Local Set Tables

Table 8-1: RVT Local Set

| Tag ID | Key Value (hex) | Key Name | Units | Format | Length in Bytes | Notes |
|---|---|---|---|---|---|---|
|  | 06 0E 2B 34 02 0B 01 01 0E 01 03 01 02 00 00 00 (CRC 17945) | RVT KLV Dictionary | None | N/A | Variable | This is the Universal Key for the RVT LS |
| 1 | 06 0E 2B 34 01 01 01 01 0E 01 02 03 10 00 00 00 (CRC 46679) | CRC 32 | None | uint32 | 4 | Performed on entire LS packet, including 16-byte US key. Note: This is Not the same Checksum as is used in STANDARD 0601. See the Appendix. This checksum must appear as that last item in an RVT LS pack when used. |
| 2 | 06 0E 2B 34 01 01 01 03 07 02 01 01 01 05 00 00 (CRC 64827) | User Defined Time Stamp -Microseconds Since 1970 | Micro-seconds | uint64 | 8 | Represents the Coordinated Universal Time (UTC) in Microseconds elapsed since midnight (00:00:00), January 1, 1970 (the UNIX Epoch). Defined as the Precision Time Stamp in MISB ST 0603 [1]. Resolution: 1 microsecond. Note: This timestamp must appear as the first item in an RVT LS pack when used. |
| 3 | 06 0E 2B 34 01 01 01 01 0E 01 01 01 0A 01 00 00 (CRC 30728) | Platform True Airspeed | Meters / Second | uint16 | 2 | True airspeed (TAS) of platform. Indicated Airspeed adjusted for temperature and altitude. 1 m/s = 1.94384449 knots. Resolution: 1 meter/second. |
| 4 | 06 0E 2B 34 01 01 01 01 0E 01 01 01 0B 01 00 00 (CRC 3772) | Platform Indicated Airspeed | Meters / Second | uint16 | 2 | Indicated airspeed (IAS) of platform. Derived from Pilot tube and static pressure sensors. 1 m/s = 1.94384449 knots. Resolution: 1 meter/second. |
| 5 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 14 00 00 00 (CRC 45638) | Telemetry Accuracy Indicator | None | uint8 | 1 | Reserved for future use |
| 6 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 15 00 00 00 (CRC 50418) | Frag Circle Radius | Meters | uint16 | 2 | Size of fragmentation circle selected by the aircrew. Resolution: 1 meter |
| 7 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 09 00 00 00 (CRC 36967) | Frame Code | None | uint32 | 4 | Range is from 0 to 4,294,967,296. Counter runs at 60 Hz |
| 8 | 06 0E 2B 34 01 01 01 01 0E 01 02 03 03 00 00 00 (CRC 13868) | UAS LS Version Number | Number | uint8 | 1 | Version number of the LS document used to generate a source of LS KLV metadata. 0 is pre-release, initial release (0806.0), or test data. 1..255 corresponds to document revisions 1 thru 255. This version is represented by 0d02. |
| 9 | 06 0E 2B 34 01 01 01 01 0E 01 01 01 19 00 00 00 (CRC 53059) | Video Data rate | bps or Hz | uint32 | 4 | Video data rate (Digital only), or Analog FM |
| 10 | 06 0E 2B 34 01 01 01 03 04 01 0B 01 00 00 00 00 (CRC 63114) | Digital Video File Format | String | ISO-7 | Max. 127 | Video Compression being used. Maximum 127 characters. Examples: MPEG2, MPEG4, H.264, Analog FM (non-compressed) As this list is not exhaustive, other values or variants are also acceptable. |
| 11 | 06 0E 2B 34 02 0B 01 01 0E 01 03 01 0F 00 00 00 (CRC 32671) | User Defined LS | Varies | N/A | V | Local set key to include User Defined data items within the RVT KLV Dictionary. Use the values of the items specified within the User Defined Data Packet. The length field is the size of all items to be packaged within this tag. |
| 12 | 06 0E 2B 34 02 0B 01 01 0E 01 03 01 0C 00 00 00 (CRC 58435) | Point of Interest LS | None | N/A | V | Local set key to include POI items within RVT KLV Dictionary. Use POI local set tags within a POI packet. The length field is the size of all POI items to be packaged within this tag. |
| 13 | 06 0E 2B 34 02 0B 01 01 0E 01 03 01 0D 00 00 00 (CRC 37623) | Area of Interest LS | None | N/A | V | Local set key to include AOI items within RVT KLV Dictionary. Use AOI local set tags within an AOI packet. The length field is the size of all AOI items to be packaged within this tag. |
| 14 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 0A 00 00 00 (CRC 3003) | MGRS Zone | None | uint8 | 1 | AIRCRAFT: First two characters of Aircraft MGRS coordinates, UTM zone 01 through 60 |
| 15 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 0B 00 00 00 (CRC 32015) | MGRS Latitude Band and Grid Square | String | ISO-7 | 3 | AIRCRAFT: Third, fourth and fifth characters of Aircraft MGRS coordinates. Third character is the alpha code for the latitude band (A through Z, omitting I and O). Fourth and fifth characters are the 2-digit alpha code for the grid square designator (WGS 84). Note that latitude bands A & B correspond to Antarctic UPS regions and Y & Z correspond to Artic UPS regions. |
| 16 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 0C 00 00 00 (CRC 11298) | MGRS Easting | Meters | uint24 | 3 | AIRCRAFT: Sixth through tenth character of Aircraft MGRS coordinates. Range is from 0 to 99,999 representing the 5-digit Easting value in meters. Resolution: 1 meter |
| 17 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 0D 00 00 00 (CRC 23190) | MGRS Northing | Meters | uint24 | 3 | AIRCRAFT: Eleventh through fifteenth character of Aircraft MGRS coordinates. Range is from 0 to 99,999 representing the 5-digit Northing value in meters. Resolution: 1 meter |
| 18 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 0A 01 00 00 (CRC 15499) | MGRS Zone Second Value | None | uint8 | 1 | FRAME CENTER: First two characters of Frame Center MGRS coordinates, UTM zone 01 through 60 |
| 19 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 0B 01 00 00 (CRC 19007) | MGRS Latitude Band and Grid Square Second Value | String | ISO-7 | 3 | FRAME CENTER: Third, fourth and fifth characters of Frame Center MGRS coordinates. Third character is the alpha code for the latitude band (for UTM: C through X, omitting I and O; for UPS: A, B, Y or Z). Fourth and fifth characters are the 2-digit alpha code for the grid square designator (WGS 84). |
| 20 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 0C 01 00 00 (CRC 6930) | MGRS Easting Second Value | Meters | uint24 | 3 | FRAME CENTER: Sixth through tenth character of Frame Center MGRS coordinates. Range is from 0 to 99,999 representing the 5-digit Easting value in meters. Resolution: 1 meter |
| 21 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 0D 01 00 00 (CRC 28070) | MGRS Northing Second Value | Meters | uint24 | 3 | FRAME CENTER: Eleventh through fifteenth character of Frame Center MGRS coordinates. Range is from 0 to 99,999 representing the 5-digit Northing value in meters. Resolution: 1 meter |

Table 8-2: Point of Interest (POI) Local Set

| Tag ID | Key Value (hex) | Key Name | Units | Format | Length in Bytes | Notes |
|---|---|---|---|---|---|---|
|  | 06 0E 2B 34 02 0B 01 01 0E 01 03 01 0C 00 00 00 (CRC 58435) | Point of Interest Local Set | None | N/A | Variable | Universal Key for the Point of Interest Local Set |
| 1 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 16 00 00 00 (CRC 24366) | POI/AOI Number | Number | uint16 | 2 | POI Number<br>** REQUIRED when sending a POI ** |
| 2 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 17 00 00 00 (CRC 10650) | POI Latitude | Degrees | int32 | 4 | POI Latitude. Based on WGS84 ellipsoid. Map -(2^31-1)..(2^31-1) to +/- 90. Use -(2^31) as an "error" indicator. -(2^31) = 0x80000000. Resolution: ~42 nano degrees. ** REQUIRED when sending a POI ** |
| 3 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 18 00 00 00 (CRC 64884) | POI Longitude | Degrees | int32 | 4 | POI Longitude. Based on WGS84 ellipsoid. Map -(2^31-1)..(2^31-1) to +/- 180. Use -(2^31) as an "error" indicator. -(2^31) = 0x80000000. Resolution: ~84 nano degrees.<br>** REQUIRED when sending a POI ** |
| 4 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 19 00 00 00 (CRC 35776) | POI Altitude | Meters | uint16 | 2 | Altitude of POI as measured from Mean Sea Level (MSL). Map 0..(2^16-1) to -900..19000 meters. 1 meter = 3.2808399 feet. Resolution: ~0.3 meters. |
| 5 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 1A 00 00 00 (CRC 4124) | POI/AOI Type | None | int8 | 1 | Target Identifier: 1=“Friendly”, 2=“Hostile”, 3="Target", or 4=“Unknown” |
| 6 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 1B 00 00 00 (CRC 26280) | POI/AOI Text | String | ISO-7 | Max. 2048 | User Defined String |
| 7 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 1C 00 00 00 (CRC 14213) | POI Source Icon | String | ISO-7 | Max. 127 | Per MIL-STD-2525B. Maximum 127 characters. Icon used in FalconView |
| 8 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 1D 00 00 00 (CRC 16689) | POI/AOI Source ID | String | ISO-7 | Max. 255 | User Defined String |
| 9 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 1E 00 00 00 (CRC 56045) | POI/AOI Label | String | ISO-7 | 16 | User Defined String |
| 10 | 06 0E 2B 34 01 01 01 01 0E 01 04 03 01 00 00 00 (CRC 22181) | Operation ID | String | ISO-7 | Max. 127 | Operation ID is the identifier for the duration of the supporting mission or event associated with the Point of Interest; this is not the platform mission designation |

Table 8-3: Area of Interest (AOI) Local Set

| Tag ID | Key Value (hex) | Key Name | Units | Format | Length in Bytes | Notes |
|---|---|---|---|---|---|---|
|  | 06 0E 2B 34 02 0B 01 01 0E 01 03 01 0D 00 00 00 (CRC 37623) | Area of Interest Local Set | None | N/A | Variable | Universal Key for the Area of Interest Local Set |
| 1 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 16 00 00 00 (CRC 24366) | POI/AOI Number | Number | uint16 | 2 | AOI Number<br>** REQUIRED when sending an AOI ** |
| 2 | 06 0E 2B 34 01 01 01 03 07 01 02 01 03 07 01 00 (CRC 23392) | Corner Latitude Point 1 (Decimal Degrees)1 | Degrees | int32 | 4 | NW corner of AOI. Based on WGS84 ellipsoid. Map -(2^31-1)..(2^31-1) to +/- 90. Use -(2^31) as an "error" indicator. -(2^31) = 0x80000000. Resolution: ~42 nano degrees.<br>** REQUIRED when sending an AOI ** |
| 3 | 06 0E 2B 34 01 01 01 03 07 01 02 01 03 0B 01 00 (CRC 11777) | Corner Longitude Point 1 (Decimal Degrees)2 | Degrees | int32 | 4 | NW corner of AOI. Based on WGS84 ellipsoid. Map -(2^31-1)..(2^31-1) to +/- 180. Use -(2^31) as an "error" indicator. -(2^31) = 0x80000000. Resolution: ~84 nano degrees.<br>** REQUIRED when sending an AOI ** |
| 4 | 06 0E 2B 34 01 01 01 03 07 01 02 01 03 09 01 00 (CRC 16481) | Corner Latitude Point 3 (Decimal Degrees)3 | Degrees | int32 | 4 | SE corner of AOI. Based on WGS84 ellipsoid. Map -(2^31-1)..(2^31-1) to +/- 90. Use -(2^31) as an "error" indicator. -(2^31) = 0x80000000. Resolution: ~42 nano degrees.<br>** REQUIRED when sending an AOI ** |
| 5 | 06 0E 2B 34 01 01 01 03 07 01 02 01 03 0D 01 00 (CRC 40097) | Corner Longitude Point 3 (Decimal Degrees)4 | Degrees | int32 | 4 | SE corner of AOI. Based on WGS84 ellipsoid. Map -(2^31-1)..(2^31-1) to +/- 180. Use -(2^31) as an "error" indicator. -(2^31) = 0x80000000. Resolution: ~84 nano degrees.<br>** REQUIRED when sending an AOI ** |
| 6 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 1A 00 00 00 (CRC 4124) | POI/AOI Type | None | int8 | 1 | Target Identifier: 1=“Friendly”, 2=“Hostile”, 3="Reserved", or 4=“Unknown”<br>** REQUIRED when sending an AOI ** |
| 7 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 1B 00 00 00 (CRC 26280) | POI/AOI Text | String | ISO-7 | Max. 2048 | User Defined String |
| 8 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 1D 00 00 00 (CRC 16689) | POI/AOI Source ID | String | ISO-7 | Max. 255 | User Defined String |
| 9 | 06 0E 2B 34 01 01 01 01 0E 01 01 03 1E 00 00 00 (CRC 56045) | POI/AOI Label | String | ISO-7 | 16 | User Defined String. |
| 10 | 06 0E 2B 34 01 01 01 01 0E 01 04 03 01 00 00 00 (CRC 22181) | Operation ID | String | ISO-7 | Max. 127 | Operation ID is the identifier for the duration of the supporting mission or event associated with the Area of Interest. This is not the platform mission designation. |

1 Corner Latitude Point 1 is the same as the Upper Left Latitude
2 Corner Longitude Point 1 is the same as the Upper Left Longitude
3 Corner Latitude Point 3 is the same as the Lower Right Latitude
4 Corner Longitude Point 3 is the same as the Lower Right Longitude

Table 8-4: User Defined Local Set

| Tag ID | Key Value (hex) | Key Name | Units | Format | Length in Bytes | Notes |
|---|---|---|---|---|---|---|
|  | 06 0E 2B 34 02 0B 01 01 0E 01 03 01 0F 00 00 00 (CRC 32671) | User Defined Data LS | None | N/A | Variable | This is the Universal Key for the User Defined Data LS |
| 1 | 06 0E 2B 34 01 01 01 01 0E 01 02 03 11 00 00 00 (CRC 49379) | Numeric ID for Data Type | N/A | uint8 | 1 | Numeric identifier with data type.<br>Bit ordering MSB first: 87654321<br>Bits 8 & 7 set the data type:<br>= 00 for strings<br>= 01 for INT<br>= 10 for UINT<br>= 11 for Experimental.<br>Bits 1 to 6 are the integer numeric ID for the user defined data ranging from 0 to 63 (64 possible user data items for each type)<br>** REQUIRED when sending User Defined Data ** |
| 2 | 06 0E 2B 34 01 01 01 01 0E 01 02 03 12 00 00 00 (CRC 23359) | User Data | N/A | N/A | V | User Data. Data type defined in byte 1 of this packet with variant (uint16, uint32, etc.) extracted from the overall pack length.<br>** REQUIRED when sending User Defined Data ** |

# 9 Appendix: Calculating the RVT LS CRC

A Cyclic Redundancy Check (CRC) is a type of hash function used to produce a checksum. The checksum is used to detect errors after transmission or storage. A CRC is computed and appended before transmission or storage, and verified afterwards by recipient to confirm that no changes occurred on transit. The CRC used is the CRC-32-ISO/IEC 13818-1 [2].

Below is an example of C++ code necessary to calculate the CRC-32 on a block of data:

```cpp
class CRC32MPEG
{
private: // CONSTANTS

     static const unsigned long DEFAULT_POLYNOMIAL = 0x04c11db7L;
     static const unsigned long CRC_ACCUM_INIT = 0xFFFFFFFF;

public: // FUNCTIONS

     CRC32MPEG(void);
     unsigned long Update(char *data_blk_ptr, int data_blk_size);
     inline void Init(void) { m_crcAccum = CRC_ACCUM_INIT; }

private: // VARIABLES

     unsigned long m_crcAccum;
     static const unsigned long m_crcTable[256];
};

/* MPEG CRC-32 Table */
const unsigned long CRC32MPEG::m_crcTable[256] = {
   0x00000000, 0x04C11DB7, 0x09823B6E, 0x0D4326D9, 0x130476DC, 0x17C56B6B,
   0x1A864DB2, 0x1E475005, 0x2608EDB8, 0x22C9F00F, 0x2F8AD6D6, 0x2B4BCB61,
   0x350C9B64, 0x31CD86D3, 0x3C8EA00A, 0x384FBDBD, 0x4C11DB70, 0x48D0C6C7,
   0x4593E01E, 0x4152FDA9, 0x5F15ADAC, 0x5BD4B01B, 0x569796C2, 0x52568B75,
   0x6A1936C8, 0x6ED82B7F, 0x639B0DA6, 0x675A1011, 0x791D4014, 0x7DDC5DA3,
   0x709F7B7A, 0x745E66CD, 0x9823B6E0, 0x9CE2AB57, 0x91A18D8E, 0x95609039,
   0x8B27C03C, 0x8FE6DD8B, 0x82A5FB52, 0x8664E6E5, 0xBE2B5B58, 0xBAEA46EF,
   0xB7A96036, 0xB3687D81, 0xAD2F2D84, 0xA9EE3033, 0xA4AD16EA, 0xA06C0B5D,
   0xD4326D90, 0xD0F37027, 0xDDB056FE, 0xD9714B49, 0xC7361B4C, 0xC3F706FB,
   0xCEB42022, 0xCA753D95, 0xF23A8028, 0xF6FB9D9F, 0xFBB8BB46, 0xFF79A6F1,
   0xE13EF6F4, 0xE5FFEB43, 0xE8BCCD9A, 0xEC7DD02D, 0x34867077, 0x30476DC0,
   0x3D044B19, 0x39C556AE, 0x278206AB, 0x23431B1C, 0x2E003DC5, 0x2AC12072,
   0x128E9DCF, 0x164F8078, 0x1B0CA6A1, 0x1FCDBB16, 0x018AEB13, 0x054BF6A4,
   0x0808D07D, 0x0CC9CDCA, 0x7897AB07, 0x7C56B6B0, 0x71159069, 0x75D48DDE,
   0x6B93DDDB, 0x6F52C06C, 0x6211E6B5, 0x66D0FB02, 0x5E9F46BF, 0x5A5E5B08,
   0x571D7DD1, 0x53DC6066, 0x4D9B3063, 0x495A2DD4, 0x44190B0D, 0x40D816BA,
   0xACA5C697, 0xA864DB20, 0xA527FDF9, 0xA1E6E04E, 0xBFA1B04B, 0xBB60ADFC,
   0xB6238B25, 0xB2E29692, 0x8AAD2B2F, 0x8E6C3698, 0x832F1041, 0x87EE0DF6,
   0x99A95DF3, 0x9D684044, 0x902B669D, 0x94EA7B2A, 0xE0B41DE7, 0xE4750050,
   0xE9362689, 0xEDF73B3E, 0xF3B06B3B, 0xF771768C, 0xFA325055, 0xFEF34DE2,
   0xC6BCF05F, 0xC27DEDE8, 0xCF3ECB31, 0xCBFFD686, 0xD5B88683, 0xD1799B34,
   0xDC3ABDED, 0xD8FBA05A, 0x690CE0EE, 0x6DCDFD59, 0x608EDB80, 0x644FC637,
   0x7A089632, 0x7EC98B85, 0x738AAD5C, 0x774BB0EB, 0x4F040D56, 0x4BC510E1,
   0x46863638, 0x42472B8F, 0x5C007B8A, 0x58C1663D, 0x558240E4, 0x51435D53,
   0x251D3B9E, 0x21DC2629, 0x2C9F00F0, 0x285E1D47, 0x36194D42, 0x32D850F5,
   0x3F9B762C, 0x3B5A6B9B, 0x0315D626, 0x07D4CB91, 0x0A97ED48, 0x0E56F0FF,
   0x1011A0FA, 0x14D0BD4D, 0x19939B94, 0x1D528623, 0xF12F560E, 0xF5EE4BB9,
   0xF8AD6D60, 0xFC6C70D7, 0xE22B20D2, 0xE6EA3D65, 0xEBA91BBC, 0xEF68060B,
   0xD727BBB6, 0xD3E6A601, 0xDEA580D8, 0xDA649D6F, 0xC423CD6A, 0xC0E2D0DD,
   0xCDA1F604, 0xC960EBB3, 0xBD3E8D7E, 0xB9FF90C9, 0xB4BCB610, 0xB07DABA7,
   0xAE3AFBA2, 0xAAFBE615, 0xA7B8C0CC, 0xA379DD7B, 0x9B3660C6, 0x9FF77D71,
   0x92B45BA8, 0x9675461F, 0x8832161A, 0x8CF30BAD, 0x81B02D74, 0x857130C3,
   0x5D8A9099, 0x594B8D2E, 0x5408ABF7, 0x50C9B640, 0x4E8EE645, 0x4A4FFBF2,
   0x470CDD2B, 0x43CDC09C, 0x7B827D21, 0x7F436096, 0x7200464F, 0x76C15BF8,
   0x68860BFD, 0x6C47164A, 0x61043093, 0x65C52D24, 0x119B4BE9, 0x155A565E,
   0x18197087, 0x1CD86D30, 0x029F3D35, 0x065E2082, 0x0B1D065B, 0x0FDC1BEC,
   0x3793A651, 0x3352BBE6, 0x3E119D3F, 0x3AD08088, 0x2497D08D, 0x2056CD3A,
   0x2D15EBE3, 0x29D4F654, 0xC5A92679, 0xC1683BCE, 0xCC2B1D17, 0xC8EA00A0,
   0xD6AD50A5, 0xD26C4D12, 0xDF2F6BCB, 0xDBEE767C, 0xE3A1CBC1, 0xE760D676,
   0xEA23F0AF, 0xEEE2ED18, 0xF0A5BD1D, 0xF464A0AA, 0xF9278673, 0xFDE69BC4,
   0x89B8FD09, 0x8D79E0BE, 0x803AC667, 0x84FBDBD0, 0x9ABC8BD5, 0x9E7D9662,
   0x933EB0BB, 0x97FFAD0C, 0xAFB010B1, 0xAB710D06, 0xA6322BDF, 0xA2F33668,
   0xBCB4666D, 0xB8757BDA, 0xB5365D03, 0xB1F740B4
};

CRC32MPEG::CRC32MPEG(void)
: m_crcAccum(CRC_ACCUM_INIT) {
}
unsigned long CRC32MPEG::Update(char *data_blk_ptr, int data_blk_size) {
  for(int j = 0; j < data_blk_size; j++) {
    int i = ((int)(m_crcAccum >> 24) ^ *data_blk_ptr++) & 0xff;
    m_crcAccum = (m_crcAccum << 8) ^ m_crcTable[i];
  }
  return m_crcAccum;
}

```
