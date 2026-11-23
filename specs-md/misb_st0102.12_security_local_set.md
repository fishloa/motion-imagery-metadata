**MISB ST 0102.12**

**STANDARD**

**Security Metadata Universal and Local Sets for Motion Imagery Data**

**22 June 2017**

# 1 Scope

This standard defines a Security Metadata Universal Set and a Security Metadata Local Set encoded as KLV (Key-Length-Value) elements for marking Motion Imagery Data with security classification information.

The methods used to gather security information, mark Motion Imagery Data with security information, and display security information are the responsibility of application system developers in concert with appropriate security officials. Originators and users of Motion Imagery Data are responsible for the proper handling, and ultimately, for the use and disposition of classified information. This standard is not a security manual or instruction on when or how to use security markings or caveats. Use of this standard does not ensure a Motion Imagery System will be accredited by security officials.

Before version 0102.5, this document was a Recommended Practice (RP). These earlier versions of this standard are referred to as RP 0102.x in this document.

Note: References [1] [2] are subject to being superseded by ODNI ICDs.

# 2 References

[1] DNI, Controlled Access Program Coordination Office (CAPCO), (U) Intelligence Community Authorized Classification and Control Markings Register and Manual Volume 5, Edition 1 (Ver 5.1), 30 Dec 2011.

[2] Director of Central Intelligence Directive (DCID) 6/3, Security Requirements for Interconnected Information Systems, 24 May 2000.

[3] SMPTE ST 336:2007 Data Encoding Protocol Using Key-Length-Value.

[4] SMPTE RP 210v13:2012 Metadata Element Dictionary.

[5] MISB ST 0807.19 MISB KLV Metadata Registry, Feb 2017.

[6] NGA.STND.0033_3.0.1 | GENC Edition 3.0, Geopolitical Entities, Names, and Codes (GENC) Standard, Edition 3.0.1 8 Jul 2016.

[7] Federal Information Processing Standards (FIPS) Publication 10-4, Countries, Dependencies, Areas of Special Sovereignty, and Their Principal Administrative Divisions, National Institute of Standards and Technology, 23 Mar 2006.

[8] Geopolitical Entities and Codes Register (GEC) https://nsgreg.nga.mil/genc/registers.jsp?register=GEC.

[9] MISB ST 0107.2 Bit and Byte Order for Metadata in Motion Imagery Files and Streams, Feb 2014.

[10] CAPCO Authorized Classification and Control Markings Register

[11] DOD Manual 5200.01 Volume 1, DoD Information Security Program: Overview, Classification, and Declassification, 24 Feb 2012.

[12] DOD Manual 5200.01 Volume 2, DoD Information Security Program: Marking of Classified Information, 19 Mar 2013.

[13] DOD Manual 5200.01 Volume 3, DoD Information Security Program: Protection of Classified Information, 19 Mar 2013.

[14] DOD Manual 5200.01 Volume 4, DoD Information Security Program: Controlled Unclassified Information (CUI), 24 Feb 2012.

[15] ISO 3166-1:2013 Codes for the representation of names of countries and their subdivisions - Part 1: Country Codes.

[16] ISO 3166-2:2013 Codes for the representation of names of countries and their subdivisions – Part 2: Country Subdivision Codes.

[17] STANAG 1059 Ed 8 Letter Codes for Geographical Entities, 19 Feb 2004.

[18] ISO/IEC 646:1991 Information Technology – ISO 7-bit coded character set for information exchange.

[19] SMPTE ST 335:2012 Metadata Element Dictionary Structure.

[20] Director of Central Intelligence Directive (DCID)1/7, 30 Jun 1998.

[21] DOD Directive (ASD (NII)), Sharing Data, Information, and Information Technology (IT) Services in the Department of Defense, Number 8320.2, 5 Aug 2013.

[22] DOD Net-Centric Data Strategy, 9 May 2003.

[23] Executive Order (EO) 13526 – Classified National Security Information, 29 Dec 2009. [24] DOD Directive 5100.55, United States Authority for North Atlantic Treaty Organization Affairs (USSAN), 27 Feb 2006.

[25] Imagery Policy Series, Particular Section 6, National Airborne Reconnaissance Imagery 2005.

[26] IETF RFC 2781 UTF-16, and encoding of ISO 10646, Feb 2000.

[27] DOD Discovery Metadata Specification (DDMS), Version 4.1, 12 Jun 2012.

[28] Memorandum for Department of Defense Executive Agent for Information Technology Standards Subject: Mandating the Use of Country Code Standards within the Department of Defense, 23 Jan 2013.

[29] SMPTE ST 330-2004 Unique Material Identifier (UMID).

[30] DOD Instruction, Number 3115.15, Geospatial Intelligence (GEOINT), 6 Dec 2011.

# 3 Acronyms

|  |  |
|---|---|
| CAPCO | Controlled Access Program Coordination Office |
| DNI | Director of National Intelligence |
| DOD | Department of Defense |
| GEC | Geopolitical Entities and Codes |
| GENC | Geopolitical Entities, Names, and Codes |
| ICD | Intelligence Community Directive |
| IEC | International Electrotechnical Commission |
| IETF | Internet Engineering Task Force |
| ISO | International Organization for Standardization |
| KLV | Key Length Value |
| LS | Local Set |
| MISB | Motion Imagery Standards Board |
| MISP | Motion Imagery Standards Profile |
| ODNI | Office of the Director of National Intelligence |
| RP | Recommended Practice |
| SCI | Sensitive Compartmented Information |
| SMPTE | Society of Motion Picture and Television Engineers |
| STANAG | STANdardization Agreement |
| US | Universal Set |
| UMID | Unique Material IDentifier |

Current system

A Motion Imagery System currently under an acquisition or block upgrade process as of the date of this document.

Legacy system

A Motion Imagery System in Operations and/or Maintenance as of the date of this document.

Motion Imagery Data

Three components: Motion Imagery, Metadata and/or Audio (see MISP). Motion Imagery Data may be transported within various types of containers as allowed by the MISP.

Motion Imagery System

A system providing the functionality of collecting, encoding, processing, controlling, exploiting, viewing, and/or storing Motion Imagery Data.

# 4 Revision History

| Revision | Date | Notes |
|---|---|---|
| ST 0102.12 | 06/22/2017 | • Deprecated REQ’s: 1, 12, 28 through 48, 52, 53<br>• Added REQ’s: 65, 66<br>• Eliminated linking of security set to transport stream, elementary stream and individual metadata items<br>• Deleted keys: UMID, Stream ID, Transport Stream ID, Item Designator<br>• Removed FIPS 10-4 reference; replaced with GEC reference<br>• Added notation within text indicating GEC as replacing FIPS 10-4<br>• Revised references to ST 0807.19, NGA.STND.0033_3.0.1, ISO 3166-1:2013, ISO 3166-2:2013; added GEC and removed ST 330:2011, 138181-1:2013, SMPTE 377-1:2011, SMPTE 338:2004 & MISB ST 1402<br>• Revised text; introduced definitions |

# 5 Introduction

This standard defines the content and application of a KLV-encoded Security Metadata Universal Set and a Security Metadata Local Set for classifying Motion Imagery Data. The individual elements in these KLV sets are encoded per the SMPTE ST 336 [3] rules for KLV, and are registered in either the SMPTE Metadata Dictionary [4] or the MISB KLV Metadata Dictionary [5]. The semantic content of security metadata is governed by this standard.

Section 6.10 discusses the GENC Standard [6] and its impact within the United States. Usage of the GENC standard may break backwards compatibility with legacy systems based on FIPS Publication 10-4 [7] (Note: GEC [8] is now the informal successor to FIPS 10-4). Unfortunately, this is unavoidable. The DoD Executive Agent for Information Technology Standards has mandated the use of GENC within the USDOD effective January 2013. With the advent of the GENC standard, requirements and recommended practices are discussed for interacting with coalition partners and legacy systems.

# 6 Security Metadata Universal Set and Local Set for Motion Imagery Data

| Requirement |  |
|---|---|
| ST 0102.10-02 | All security metadata shall be expressed in accordance with MISB ST 0107 [9]. |

## 6.1 Security Metadata Elements

The following security metadata elements are intended to comprise information needed to comply with CAPCO [1] [10], the DoD Information Security Program [11] [12] [13] [14] and other normative referenced security directives [2]. These normative documents govern which fields are mandatory, and which fields are optional. Security requirements may dictate which entries are mandatory. In all applications, the presence or absence of certain metadata will depend on the context of the application and its unique security requirements. Whenever there is conflict between this standard and directions of Security officials on the required presence or absence of entries, the direction of Security officials takes precedence.

| Requirement |  |
|---|---|
| ST 0102.10-55 | All Motion Imagery Data shall be marked at their proper classification level with appropriate handling, caveats and releasing instructions. |

### 6.1.1 Security Classification

The Security Classification metadata element represents the overall security classification of the Motion Imagery Data in accordance with U.S. and NATO classification guidance. Values allowed include: TOP SECRET, SECRET, CONFIDENTIAL, RESTRICTED, and UNCLASSIFIED (all caps) followed by a double forward slash “//”. This is a mandatory entry in a Security Metadata set.

| Requirement |  |
|---|---|
| ST 0102.10-03 | Motion Imagery Data shall contain the Security Classification metadata element. |

### 6.1.2 Classifying Country and Releasing Instructions Country Coding Method

This metadata element identifies the country coding method for the Classifying Country (Par. 6.1.3) and Releasing Instructions (Par. 6.1.6) metadata. The Country Coding Method value allows GEC two-letter or four-letter alphabetic country code (legacy systems only); ISO-3166 [15] [16] two-letter, three-letter, or three-digit numeric; STANAG 1059 [17] two-letter or three-letter codes; and GENC two-letter, three-letter or three-digit numeric. GENC administrative subdivision codes are not applicable.

Example of Country Coding Method: GENC Two Letter

| Requirement |  |
|---|---|
| ST 0102.10-04 | Motion Imagery Data shall contain the Classifying Country or Releasing Instructions Country Coding Method metadata element. |

### 6.1.3 Classifying Country

The Classifying Country metadata element contains a value for the classifying country code preceded by a double slash "//."

Example of classifying country: //CZE (Example of GENC code)

//GB (Example of ISO-3166 code)

| Requirement |  |
|---|---|
| ST 0102.10-05 | Motion Imagery Data shall contain the Classifying Country metadata element. |

### 6.1.4 Sensitive Compartmented Information (SCI) / Special Handling Instructions (SHI) Information

| Requirement(s) |  |
|---|---|
| ST 0102.10-06 | When special handling instructions or compartmented information are used, Motion Imagery Data shall contain the Sensitive Compartmented Information (SCI) / Special Handling Instructions (SHI) metadata element. |
| ST 0102.10-07 | When used, SCI/SHI digraphs, trigraphs, or compartment names shall be added identifying a single or a combination of special handling instructions. |
| ST 0102.10-08 | A single entry shall be ended with a double forward slash “//”. |
| ST 0102.10-09 | Multiple digraphs, trigraphs, or compartment names shall be separated by a single forward slash “/”. |
| ST 0102.10-10 | Multiple digraphs, trigraphs, or compartment names shall be ended with a double forward slash “//”. |
| ST 0102.10-11 | Multiple SCI/SHI digraphs, trigraphs, or compartment names shall be concatenated in one metadata element free text [18] entry. |

### 6.1.5 Caveats

The Caveats metadata element represents pertinent caveats (or code words) from each category of the appropriate security entity register. Entries in this field may be abbreviated or spelled out as free text [18] entries.

| Requirement(s) |  |
|---|---|
| ST 0102.10-13 | When caveats or code words are used, all Motion Imagery Data shall contain the Caveats metadata element. |
| ST 0102.10-14 | The Caveats metadata element shall be used to indicate FOR OFFICIAL USE ONLY (FOUO) data. |

### 6.1.6 Releasing Instructions

The Releasing Instructions metadata element contains a list of country codes to indicate the countries to which the Motion Imagery Data is releasable.

| Requirement(s) |  |
|---|---|
| ST 0102.10-15 | When releasing instructions are used, all Motion Imagery Data shall contain the Releasing Instructions metadata element. |
| ST 0102.10-16 | Multiple country codes shall be separated by a blank (space – NOT underscore). |
| ST 0102.10-17 | Multiple country codes shall be concatenated in one Releasing Instructions metadata element. |

The use of blank spaces to separate country codes, instead of semi-colons or other characters, is to comply with security guidelines, and to allow parsing of fields by automated security screening systems. Various countries and international organizations have differing requirements regarding the proper encoding of releasing instructions. Systems need to follow the security guidelines appropriate to their mission.

| Requirement(s) |  |
|---|---|
| ST 0102.11-63 | Systems shall populate the country codes in the Releasing Instructions metadata element in accordance with the security guidelines (national or international) appropriate to their mission. |
| ST 0102.11-64 | U.S. systems shall populate the country codes in the Releasing Instructions metadata element in accordance with DOD Manual 5200.01 Volume 2. |

### 6.1.7 Classified By

The Classified By metadata element identifies the name and type of authority used to classify the Motion Imagery Data. The metadata element is free text and can contain either the original classification authority name and position or personal identifier, or the title of the document or security classification guide used to classify the data.

### 6.1.8 Derived From

The Derived From metadata element contains information about the original source of data from which the classification was derived. The metadata element is free text [18].

### 6.1.9 Classification Reason

The Classification Reason metadata element contains the reason for classification or a citation from a document. The metadata element is free text [18].

### 6.1.10 Declassification Date

The Declassification Date metadata element provides a date when the classified material may be automatically declassified.

| Requirement |  |
|---|---|
| ST 0102.10-22 | The Declassification Date metadata element format shall be YYYYMMDD. |

### 6.1.11 Classification and Marking System

The Classification and Marking System metadata element identifies the classification or marking system used in the Security Metadata set as determined by the appropriate security entity for the country originating the data.

| Requirement |  |
|---|---|
| ST 0102.10-21 | The Classification or Marking System metadata element shall be free text. |

### 6.1.12 Object Country Coding Method

The Object Country Coding Method metadata element identifies the coding method for the Object Country Code (Par. 6.1.13) metadata. This element allows use of GEC two-letter or four-letter alphabetic country code (legacy systems only); ISO-3166 two-letter, three-letter, or three-digit numeric; STANAG 1059 two-letter or three-letter codes; and GENC two-letter, three-letter, three-digit numeric or administrative subdivisions. Use of this element in version 6 of this Standard and later is mandatory. In version 5 and earlier, it was optional; its absence indicates the default GENC two-letter coding method was used in the Object Country Code element. See also Section 6.9.

### 6.1.13 Object Country Codes

The Object Country Codes metadata element contains a value identifying the country (or countries), which is the object of the Motion Imagery Data.

| Requirement(s) |  |
|---|---|
| ST 0102.10-23 | Motion Imagery Data shall contain the Object Country Code metadata element. |
| ST 0102.10-24 | Multiple Object Country Codes shall be separated by a semi-colon “;” (no spaces). |
| ST 0102.10-25 | Multiple Object Country Codes shall be concatenated in one Object Country Code metadata element entry. |
| ST 0102.10-26 | The object country code of the geographic region lying under the center of the image frame shall populate the Object Country Code metadata element. |

The object country codes of other represented geographic regions may be included in addition to the country code of the geographic region under the center of the image frame. Note: The use of the semi-colon to separate country codes, instead of blanks or other characters, is to allow processing by current, automated imagery processing and management tools.

### 6.1.14 Classification Comments

The Classification Comments metadata element allows for security related comments and format changes necessary in the future. This field may be used in addition to those required by appropriate security entity and is optional.

| Requirement |  |
|---|---|
| ST 0102.10-27 | The Classification Comments metadata element shall only be used to convey non-essential security-related information. |

### 6.1.15 Version

The Version metadata element indicates the version number of MISB ST 0102 referenced.

| Requirement(s) |  |
|---|---|
| ST 0102.10-56 | The Security Metadata Version shall be included in all Security Metadata Universal and Local Sets utilizing version four (4) of the security metadata or later. |
| ST 0102.10-57 | When the Security Metadata Version is not found in the Security Metadata, version three (3) shall be assumed. |

### 6.1.16 Classifying Country and Releasing Instructions Country Coding Method Version Date

This metadata element provides the effective date (promulgation date) of the source (FIPS 10-4, ISO 3166, GENC 2.0, or STANAG 1059) used for the Classifying Country and Releasing Instructions Country Coding Method. As ISO 3166 is updated by dated circulars, not by version revision, the ISO 8601 YYYY-MM-DD formatted date is used.

### 6.1.17 Object Country Coding Method Version Date

The Object Country Coding Method Version Date metadata element is the effective date (promulgation date) of the source (FIPS 10-4, ISO 3166, GENC 2.0, or STANAG 1059) used for the Object Country Coding Method. As ISO 3166 is updated by dated circulars, not by version revision, the ISO 8601 YYYY-MM-DD formatted date is used.

## 6.2 Security Metadata Repetition Rate

| Requirement(s) |  |
|---|---|
| ST 0102.10-49 | A Security Metadata set shall be repeated / updated whenever classification, special handling instructions, releasability, or other mandatory fields change value. |
| ST 0102.10-50 | Security Metadata Sets shall be repeated no less than every thirty (30) seconds. |

Applications producing short Motion Imagery Data clips or segments of a few seconds in duration may need to repeat Security Metadata Sets as often as every frame.

## 6.3 Unclassified Motion Imagery Data

| Requirement(s) |  |
|---|---|
| ST 0102.10-54 | Unclassified Motion Imagery Data shall be marked with Security Metadata. |
| ST 0102.10-51 | When Motion Imagery Data is unclassified, the Security Metadata Set value shall be "UNCLASSIFIED//" for Security Classification. |

Other entries in the set which limit or clarify the classification are optional.

## 6.4 Partial Security Metadata Universal and Local Sets

For some operational situations or applications not all metadata elements in Section 6.1 may be required. The originator and his cognizant Security official are responsible to ensure all appropriate security entries are populated.

## 6.5 Absence of Security Metadata Universal or Local Sets

The proper insertion/extraction of Security Metadata sets into/from Motion Imagery is the responsibility of system developers. Bit stream originators and system developers are responsible to incorporate continual checks for Security Metadata in their applications. The absence of Security Metadata does not signify Motion Imagery Data as Unclassified.

## 6.6 Security Metadata Universal Set Elements

The Security Metadata Universal Set 16-byte Universal Label Key is registered in MISB ST 0807 as:

06 0E 2B 34 02 01 01 01 02 08 02 00 00 00 00 00 (CRC 31942).

| Requirement |  |
|---|---|
| ST 0102.12-65 | The Security Metadata Universal Set shall conform to SMPTE ST 336 KLV encoding rules. |

Metadata elements comprising the Security Metadata Universal Set are listed in Table 1. SMPTE ST 335 [19] defines the length values of metadata items as “stated” as opposed to “normative”. Certain data types may have a flexible length. The value within the Length column in Table 1 specifies either a stated or “Variable” number of bytes. When the value is “Variable”, the length is determined by parsing the encoded BER length as specified in the KLV packet. Metadata elements are indicated as Required, by Context, or Optional.

Table 1: Security Metadata Universal Set Elements

| Universal Set Key | Name |
|---|---|
| 06.0E.2B.34.02.01.01.01.02.08.02.00.00.00.00.00 (CRC 31942) | Security Metadata Universal Set |

Constituent Elements

| 16-byte UL | Name | Data Type or References | Allowed Values or References | Length (Bytes) | Required/ Optional/ Context |
|---|---|---|---|---|---|
| 06 0E 2B 34<br>01 01 01 03<br>02 08 02 01<br>00 00 00 00<br>(CRC 5487) | Security Classification | ISO/IEC 646 [18]<br>Text | TOP SECRET//<br>SECRET//<br>CONFIDENTIAL//<br>RESTRICTED//<br>UNCLASSIFIED// | Variable | Required |
| 06 0E 2B 34<br>01 01 01 03<br>07 01 20 01<br>02 07 00 00<br>(CRC 41133) | Classifying Country and Releasing Instructions Country Coding Method | ISO/IEC 646 [18] | ISO-3166 Two Letter<br>ISO-3166 Three Letter<br>ISO-3166 Numeric<br>FIPS 10-4 Two Letter<br>FIPS 10-4 Four Letter<br>1059 Two Letter<br>1059 Three Letter<br>GENC Two Letter<br>GENC Three Letter<br>GENC Numeric<br>FIPS 10-4 Mixed<br>ISO 3166 Mixed<br>STANAG 1059 Mixed<br>GENC Mixed | Variable | Required |
| 06 0E 2B 34<br>01 01 01 03<br>07 01 20 01<br>02 08 00 00<br>(CRC 35996) | Classifying Country | ISO/IEC 646 [18]<br>Text from the appropriate standard preceded by ‘//’ | FIPS 10-4 [7]<br>ISO-3166 [15] [16]<br>GENC [6]<br>STANAG 1059 [17] | Variable | Required |
| 06 0E 2B 34<br>01 01 01 01<br>0E 01 02 03<br>02 00 00 00<br>(CRC 16536) | Security-SCI/SHI Information | ISO/IEC 646 [18] | Security Ref [1] | Variable | Context |
| 06 0E 2B 34<br>01 01 01 03<br>02 08 02 02<br>00 00 00 00<br>(CRC 64445) | Caveats | ISO/IEC 646 [18] | Security Ref [1] | Variable | Context |
| 06 0E 2B 34<br>01 01 01 03<br>07 01 20 01<br>02 09 00 00<br>(CRC 48044) | Releasing Instructions | ISO/IEC 646 [18] | Security Refs [1] [20, 21, 22, 23, 24, 25] | Variable | Context |
| 06 0E 2B 34<br>01 01 01 03<br>02 08 02 03<br>00 00 00 00<br>(CRC 20972) | Classified By | ISO/IEC 646 [18] | Security Refs [1] [24] | Variable | Context |
| 06 0E 2B 34<br>01 01 01 03<br>02 08 02 06<br>00 00 00 00<br>(CRC 29371) | Derived From | ISO/IEC 646 [18] | Security Refs [1], [24] | Variable | Context |
| 06 0E 2B 34<br>01 01 01 03<br>02 08 02 04<br>00 00 00 00<br>(CRC 13880) | Classification Reason | ISO/IEC 646 [18] | Security Refs [1] [24] | Variable | Context |
| 06 0E 2B 34<br>01 01 01 03<br>02 08 02 05<br>00 00 00 00<br>(CRC 40041) | Declassification Date | ISO/IEC 646 [18] | YYYYMMDD | 8 | Context |
| 06 0E 2B 34<br>01 01 01 03<br>02 08 02 08<br>00 00 00 00<br>(CRC 48403) | Classification and Marking System | ISO/IEC 646 [18] | N/A | Variable | Context |
| 06 0E 2B 34<br>01 01 01 03<br>07 01 20 01<br>02 06 00 00<br>(CRC 38813) | Object Country Coding Method | ISO/IEC 646 [18]<br>Text | ISO-3166 Two Letter<br>ISO-3166 Three Letter<br>ISO-3166 Numeric<br>FIPS 10-4 Two Letter<br>FIPS 10-4 Four Letter<br>1059 Two Letter<br>1059 Three Letter<br>GENC Two Letter<br>GENC Three Letter<br>GENC Numeric<br>GENC AdminSub | Variable | Required |
| 06 0E 2B 34<br>01 01 01 03<br>07 01 20 01<br>02 01 01 00<br>(CRC 8508) | Object Country Codes | RFC 2781 [26] [27] | Refs [15] [16] [28] [29] | Variable | Required |
| 06 0E 2B 34<br>01 01 01 03<br>02 08 02 07<br>00 00 00 00<br>(CRC 55530) | Classification Comments | ISO/IEC 646 [18] | N/A | Variable | Optional |
| 06 0E 2B 34<br>01 01 01 01<br>0E 01 02 05<br>04 00 00 00<br>(CRC 43652) | Version | uint16 | Value is version number of this document; e. g. for ST 0102.10, this value is 0x000A | 2 | Required |
| 06 0E 2B 34<br>01 01 01 01<br>0E 01 04 03<br>03 00 00 00<br>(CRC 48077) | Classifying Country and Releasing Instructions Country Coding Method Version Date | ISO/IEC 646 [18] | YYYY-MM-DD | 10 | Optional |
| 06 0E 2B 34<br>01 01 01 01<br>0E 01 04 03<br>04 00 00 00<br>(CRC 60128) | Object Country Coding Method Version Date | ISO/IEC 646 [18] | YYYY-MM-DD | 10 | Optional |

## 6.7 Security Metadata Local Set Elements

The Security Metadata Local Set 16-byte Universal Label Key is registered in MISB ST 0807 as: 06 0E 2B 34 02 03 01 01 0E 01 03 03 02 00 00 00 (CRC 40980).

| Requirement |  |
|---|---|
| ST 0102.12-66 | The Security Metadata Local Set shall conform to SMPTE ST 336 [3] KLV encoding rules. |

Comment for version Standard 0102.5: In creating the key for the Security Metadata Local Set in version RP 0102.4, it was necessary to use the DRAFT of SMPTE RP 336M-2007. That draft contained ambiguous information, which led to the incorrect assignment of bytes five and six in the Security Metadata Local Set. The final version of SMPTE RP 336M removed the ambiguity and it became apparent the Security Metadata Local Set Key needed to be updated.

Metadata elements allowed in a Security Metadata Local Set are listed in Table 2.

Table 2: Security Metadata Local Set Elements

| Local Set Key | Name |
|---|---|
| 06.0E.2B.34.02.03.01.01.0E.01.03.03.02.00.00.00 (CRC 40980) | Security Metadata Local Set |

Constituent Elements

| Tag | Name | Data Type or References | Allowed Values or References | Length (Bytes) | Required/ Optional/ Context |
|---|---|---|---|---|---|
| 1 | Security Classification | uint8 | UNCLASSIFIED// (0x01)<br>RESTRICTED// (0x02)<br>CONFIDENTIAL// (0x03)<br>SECRET// (0x04)<br>TOP SECRET// (0x05) | 1 | Required |
| 2 | Classifying Country and Releasing Instructions Country Coding Method | uint8 | ISO-3166 Two Letter (0x01)<br>ISO-3166 Three Letter (0x02)<br>FIPS 10-4 Two Letter (0x03)<br>FIPS 10-4 Four Letter (0x04)<br>ISO-3166 Numeric (0x05)<br>1059 Two Letter (0x06)<br>1059 Three Letter (0x07)<br>Omitted Value (0x08)<br>Omitted Value (0x09)<br>FIPS 10-4 Mixed (0x0A)<br>ISO 3166 Mixed (0x0B)<br>STANAG 1059 Mixed (0x0C)<br>GENC Two Letter (0x0D)<br>GENC Three Letter (0x0E)<br>GENC Numeric (0x0F)<br>GENC Mixed (0x10) | 1 | Required |
| 3 | Classifying Country | Text from the appropriate standard preceded by ‘//’ | FIPS 10-4 [7]<br>ISO-3166 [15] [16]<br>STANAG 1059 [17]<br>GENC [6] | Variable | Required |
| 4 | Security-SCI/SHI information | ISO/IEC 646 [18] | Security Ref [1] | Variable | Context |
| 5 | Caveats | ISO/IEC 646 [18] | Security Ref [1] | Variable | Context |
| 6 | Releasing Instructions | ISO/IEC 646 [18] | Security Refs [1] [20, 21, 22, 23, 24, 25] | Variable | Context |
| 7 | Classified By | ISO/IEC 646 [18] | Security Refs [1] [24] | Variable | Context |
| 8 | Derived From | ISO/IEC 646 [18] | Security Refs [1], [24] | Variable | Context |
| 9 | Classification Reason | ISO/IEC 646 [18] | Security Refs [1], [24] | Variable | Context |
| 10 | Declassification Date | ISO/IEC 646 [18] | YYYYMMDD | 8 | Context |
| 11 | Classification and Marking System | ISO/IEC 646 [18] | N/A | Variable | Context |
| 12 | Object Country Coding Method | uint8 | ISO-3166 Two Letter (0x01)<br>ISO-3166 Three Letter (0x02)<br>ISO-3166 Numeric (0x03)<br>FIPS 10-4 Two Letter (0x04)<br>FIPS 10-4 Four Letter (0x05)<br>1059 Two Letter (0x06)<br>1059 Three Letter (0x07)<br>Omitted Value (0x08)<br>Omitted Value (0x09)<br>Omitted Value (0x0A)<br>Omitted Value (0x0B)<br>Omitted Value (0x0C)<br>GENC Two Letter (0x0D)<br>GENC Three Letter (0x0E)<br>GENC Numeric (0x0F)<br>GENC AdminSub (0x40) | 1 | Required |
| 13 | Object Country Codes | RFC 2781 [26] [27] | Refs [15] [16] [28] [29] | Variable | Required |
| 14 | Classification Comments | ISO/IEC 646 [18] | N/A | Variable | Optional |
| 22 | Version | uint16 | Value is version number of this document; e. g. for ST 0102.10, this value is 0x000A | 2 | Required |
| 23 | Classifying Country and Releasing Instructions Country Coding Method Version Date | ISO/IEC 646 [18] | YYYY-MM-DD | 10 | Optional |
| 24 | Object Country Coding Method Version Date | ISO/IEC 646 [18] | YYYY-MM-DD | 10 | Optional |

## 6.8 Conversion of Security Metadata Elements between Universal and Local Sets

For bandwidth efficiency, some elements in the Local Set are formatted differently than the Universal Set equivalent. This section provides conversion information for the differing items.

### 6.8.1 Security Classification

From Universal Set to Local Set:

Convert string to corresponding unsigned integer.

From Local Set to Universal Set:

Convert unsigned integer to corresponding uppercase string.

### 6.8.2 Classifying Country and Releasing Instructions Country Code

From Universal Set to Local Set:

Convert string to corresponding unsigned integer.

From Local Set to Universal Set:

Convert unsigned integer to corresponding uppercase string.

### 6.8.3 Object Country Coding Method

From Universal Set to Local Set:

Convert string to corresponding unsigned integer.

From Local Set to Universal Set:

Convert unsigned integer to corresponding uppercase string.

## 6.9 “Mixed” Country Coding Method

The CAPCO Authorized Classification and Control Marking Register Annex F is the approved source of (coalition and other multi-national organization) tetragraphs.

| Requirement |  |
|---|---|
| ST 0102.10-58 | The Mixed Country Coding Method shall be used to support di- or tri-graphs (but not both) from GEC, ISO 3166, GENC and STANAG 1059, respectively, and approved tetragraphs in the same field. |

For example, a Tag 2 value of “0C” would indicated the payload of Tag 6 consists of STANAG 1059 di-graphs or tri-graphs (but not both), and one or more tetragraphs from the CAPCO Authorized Classification and Control Marking Register.

CAPCO requires document header/footer and portion mark classification country codes be ISO 3166 trigraphs.

## 6.10 Geopolitical Entities, Names, and Codes (GENC) Standard

The FIPS 10-4 standard has been retired for use by all U.S. government agencies, services and commands by the Department of State (Note: GEC is now the informal successor to FIPS 10-4). The GENC standard was created to replace FIPS 10-4 and is based on ISO-3166. A memorandum from the DoD Executive Agent for Information Technology Standards [28] has mandated the use of GENC and ISO-3166 for all country code uses in U.S. DoD information systems. New systems undergoing acquisition or block upgrades are expected to comply with GENC by December 2014.

The GENC standard is a profile of ISO-3166 and uses the codes out of ISO-3166-1 and ISO-3166-2. ISO-3166-1 defines country codes and ISO-3166-2 adds codes for administrative subdivisions (e.g. states, provinces, etc.). The GENC interpretation of these codes does differ from the ISO standard. Many times, the differences are subtle and related to the proper names or shorthand names of countries. Other times the differences are more significant. GENC adds additional codes and disallows others from the ISO standards. The NSG registry has established a website describing the content of GENC, and it is the definitive standard for GENC.

U.S. Agencies, Commands and Services are required to use GENC and ISO-3166 in their systems. For backwards compatibility, this standard allows usage of FIPS 10-4 within legacy systems producing Motion Imagery Data.

| Requirement(s) |  |
|---|---|
| ST 0102.10-59 | Current and future U.S. systems producing Motion Imagery Data shall adhere to guidance as directed by the Memorandum for Department of Defense Executive Agent for Information Technology Standards, Subject: Mandating the Use of Country Code Standards within the Department of Defense for GENC and ISO- 3166 country codes. |
| ST 0102.10-60 | All current systems that do not use GENC or ISO-3166 shall use country codes as defined in GENC and ISO-3166 at their next block upgrade. |
| ST 0102.10-61 | All current systems that consume Motion Imagery Data shall understand and properly interpret all country coding methods except for GENC Administrative Subdivisions (GENC AdminSub) code. |
| ST 0102.10-62 | All current systems that consume Motion Imagery Data shall understand and properly interpret at least the country code portion of a GENC Administrative Subdivisions (GENC AdSub) code. |

# Appendix A Deprecated Requirements

| Requirement(s) |  |
|---|---|
| ST 0102.10-18 | For U.S. systems, the country code of the originating country shall appear first as required by DOD Manual 5200.01 Volume 2. |
| ST 0102.10-19 | After the country code of the originating country the country codes of other countries to which the data are releasable shall appear in alphabetical order. |
| ST 0102.10-20 | After the country codes of other countries, the codes of any non-state organizations (such as NATO) to which the data are releasable shall appear in alphabetical order. |
| ST 0102.10-01 | All MPEG-2 transport streams or files shall be tagged with security metadata in accordance with MISB ST 0102. |
| ST 0102.10-12 | The Sensitive Compartmented Information (SCI) / Special Handling Instructions (SHI) metadata element shall only be present when the material in the transport stream or file requires special handling. |
| ST 0102.10-28 | The Security Metadata Universal Set 16-byte Universal Label Key shall be 06 0E 2B 34 02 01 01 01 02 08 02 00 00 00 00 00 (CRC 31942). |
| ST 0102.10-29 | Required security and linking information shall be contained entirely within a Security Metadata Universal Set that conforms to SMPTE ST 336 KLV Universal Set encoding rules. |
| ST 0102.10-30 | The Security Metadata Local Set 16-byte Universal Label Key shall be 06 0E 2B 34 02 03 01 01 0E 01 03 03 02 00 00 00 (CRC 40980). |
| ST 0102.10-31 | Required security and linking information shall be contained entirely within a Security Metadata Local Set that conforms to SMPTE ST 336 KLV Local Set encoding rules. |
| ST 0102.10-32 | When used, the 32-byte Unique Material Identifier (UMID) defined by SMPTE ST 330 shall identify the essence to which security metadata is linked. |
| ST 0102.10-33 | When used, the stream_id shall be the value for the Stream ID metadata element. |
| ST 0102.10-34 | When used, the stream_id shall link the security metadata to the corresponding elementary stream essence. |
| ST 0102.10-35 | When used, the transport_stream_id shall be the Value for the Transport Stream ID. |
| ST 0102.10-36 | When used, the 16-byte Universal Label Key for the element, set or pack to which the Security Metadata Set is linked shall be the value of the Universal Label Key ID. |
| ST 0102.10-37 | When indicating the security classification of individual MPEG-2 streams, files or MXF-based files, the appropriate link metadata elements shall be contained within a Security Metadata Set. |
| ST 0102.10-38 | When the same Security Metadata Set applies to multiple Elementary Streams, the Security Metadata Set shall contain each of the UMIDs or Stream IDs separately in the Security Set. |
| ST 0102.10-39 | The Security Metadata Sets for the Transport Stream shall convey all the security information for the highest classification Elementary Stream or metadata contained in the Transport Stream. |
| ST 0102.10-40 | The UMID or stream_id shall be used for directly linking Security metadata to referenced Program Streams in their entirety. |
| ST 0102.10-41 | The Security Metadata Sets for the Program Stream shall convey all the security information for the highest classification Elementary Stream or metadata contained in the Program Stream. |
| ST 0102.10-42 | To indicate the security classification of individual metadata elements, a collection of metadata elements, or entire metadata streams, the appropriate link metadata elements shall be contained within a Security Metadata Set. |
| ST 0102.10-43 | When only a single metadata element is associated with a Security Metadata Set, the Security Metadata Set shall contain the Universal Label Key ID the Value of which is the 16-byte Universal Label Key for the single metadata element. |
| ST 0102.10-44 | When a subset of metadata elements within a set or pack is linked to a Security Metadata Set, the Security Metadata Set shall contain each individual Universal Label Key ID for the metadata elements to which it is linked. |
| ST 0102.10-45 | When all metadata in a set or pack is associated with a Security Metadata Set, the set or pack shall contain the Security Metadata Set with a Universal Label Key ID whose value is the Universal Label Key for the set or pack. |
| ST 0102.10-46 | When all metadata in an Elementary Stream (ES) is associated with the same Security Metadata Set, the two sets of metadata shall be associated using the methods in Section 6.4.1 of MISB ST 0102.10. |
| ST 0102.10-47 | When links are not used in the Security Metadata Set, all the security information shall be considered to apply to all the essence and metadata in the MPEG-2 Transport Stream, MPEG-2 Program Stream, or file. |
| ST 0102.10-48 | When essence or metadata linkages are not made within a Security Metadata Universal or Local Set, the Security Metadata shall apply to all essence and metadata in the MPEG-2 stream or file. |
| ST 0102.10-52 | The Version metadata element of the Security Metadata Universal and Local Set for MISB RP 0102.4, MISB ST 0102.5, and later versions shall be required. |
| ST 0102.10-53 | MISB RP 0102.3 shall be assumed for the Version metadata element in the absence of MISB RP 0102.5, MISB ST 0102.5 or a later version of MISB ST 0102. |
