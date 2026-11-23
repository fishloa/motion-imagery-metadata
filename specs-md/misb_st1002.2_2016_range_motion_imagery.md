MISB ST 1002.2

STANDARD

28 June 2016

Range Motion Imagery

## 1 Scope

This standard describes Range Motion Imagery, its format and supporting metadata.

Range Motion Imagery is a temporal sequence of Range Images. Each Range Image is a collection of Range Measurements from a sensor to target scene. A Range Measurement is the distance (e.g. meters) from an object (or area) in the scene to the sensor. The KLV structures of this Standard are intended to allow for flexibility, efficient packing, and future extensions. Range Motion Imagery can be used standalone or in collaboration with other Motion Imagery. MISB ST 1107 Metric Geopositioning Metadata Set [1] provides the basis for collaborating with other Motion Imagery types.

This standard describes the: Perspective Range Motion Imagery and Depth Range Motion Imagery; the collection methods of Range Motion Imagery; the formats used for storing or transmitting Range Motion Imagery; the supporting metadata needed for Range Motion Imagery including, temporal, uncertainty and compression parameters; and the alignment to Collaborative Imagery.

## 2 References

[1] MISB ST 1107.2 Metric Geopositioning Metadata Set, Jun 2015.

[2] MISB ST 1202.2 Generalized Transformation Parameters, Feb 2015.

[3] SMPTE ST 336:2007 Data Encoding Protocol Using Key-Length-Value.

[4] SMPTE RP 210v13:2012 Metadata Element Dictionary.

[5] MISB ST 0807.17 MISB KLV Metadata Registry, Jun 2016.

[6] MISB ST 0107.2 Bit and Byte Order for Metadata in Motion Imagery Files and Streams, Feb 2014.

[7] MISB RP 0701 Common Metadata System: Structure, Aug 2007.

[8] MISB ST 0603.4 MISP Time System and Timestamps, Feb 2016.

[9] MISB MISP-2016.3: Motion Imagery Handbook, Jun 2016.

[10] ISO/IEC ISO/IEC 8824-1:2008 (ITU-T X.690) Information Technology - Abstract Syntax Notation One (ASN.1) - Encoding Rules - BER-OID Sub-Identifier defined in Section 8.19.2, 2008.

[11] MISB ST 1303 Multi-Dimensional Array Pack, Feb 2014.

[12] MISB ST 1201.2 Floating Point to Integer Mapping, Oct 2015.

## 3 Modifications and Changes

| Revision | Date | Summary of Changes |
|---|---|---|
| 1002.2 | 06/28/2016 | • Clarified data type of SPRM locations. Removed BER-OID in description for these values and replaced with IEEE floating point to match the table description<br>• Fixed consistency of wording used for “Data Section” vice “Section Data”<br>• Updated references<br>• Updated Image Time to refer to Precision Time Stamp. (removed the term “POSIX Microseconds”)<br>• Updated the term “frame” to “image” for consistency<br>• Updated line/sample terminology to row/column terminology to match other MISP standards and the Motion Imagery Handbook<br>• Deprecated REQ -01 (UL is defined in MISB ST 0807) |

## 4 Definitions and Acronyms

|  |  |
|---|---|
| KLV | Key-Length-Value |
| MISB | Motion Imagery Standards Board |
| MISP | Motion Imagery Standards Profile |
| RP | Recommended Practice |
| SACP | Single Aim Center Pixel Range Measurement |
| SMPTE | Society of Motion Picture and Television Engineers |
| SPRM | Single Point Range Measurement |
| ST | Standard |
| VLP | Variable Length Pack |

## 5 Introduction

Range Motion Imagery is a series of Images where the value of the Range Image Pixels is the distance from the Sensor to a Scene Area. Two primary types of Range Imagery, and multiple sources of Range Imagery are discussed in the following sections.

### 5.1 Range Imagery Types

Range Imagery data values can represent two different types of measurements: Perspective Range Measurements or Depth Range Measurements. Perspective Range Measurements represent measured or computed distances from the Scene Areas to the perspective center of the sensor, as shown in Figure 1.

Perspective Range Measurement Perspective Center

Figure 1: Illustration of Range Measurements (profile view)

Perspective Range data can be directly integrated with a rigorous sensor model without further processing.

Depth Range Measurements represent measured or computed distances from the Scene Areas to a plane parallel to the focal plane, as shown in Figure 2. In this standard, the plane parallel to the focal plane is called the Backplane.

Depth Range Measurement Backplane

Figure 2: Illustration of Depth Range Measurements (profile view)

Depth data are more commonly known as depth fields or depth maps. Although depth data cannot be integrated into a rigorous sensor model, depth data can be converted to range data; however, the algorithm for conversion is not defined in this version.

### 5.2 Range Imagery Sources

Range Imagery may be collected directly from a Range Sensor or Computationally Extracted from another imagery source (e.g. Camera) or data source (e.g. Point Cloud).

#### 5.2.1 Range Sensor

When Range Imagery is collected directly from a Range Sensor, it can be either stand-alone or related to another Sensor -- called a Collaborative Sensor. Visible Light and IR cameras are both examples of Collaborative Sensors. Range Imagery from a Collaborative Sensor is either co-boresighted or non-boresighted with the Collaborative Sensor. In co-boresighted Range Imagery, the sensor collects data through the same physical aperture as a Collaborative Sensor, which provides identical geometry to the Collaborative Sensor’s collected image. Figure 3 illustrates a co-boresighted system. Although the geometry is the same, the Range and Collaborative Images may not have the same orientation, pixel sizes or magnification through the optical path. These effects can be modeled using the Generalized Transformation (MISB ST 1202 [2]), which is a mathematical process that maps the data from one image perspective to another. The parameters from the Generalized Transformation can be included with the Range data for later processing. Range Image Sensor (e.g. LIDAR Sensor)

Lens Scene

Figure 3: Co-Boresighted System of Sensors

Collaborative Sensor (e.g. Visible Light CCD) Light Splitter

When the Range Imagery does not meet the requirements for being co-boresighted, the Range Imagery is called non-boresighted, and a transformation is needed to align the Range Imagery with the Collaborative Imagery. Figure 4 illustrates a non-boresighted system of Sensors.

Lens

Range Image Sensor Scene (e.g. LIDAR Sensor)

Collaborative Sensor (e.g. Visible Light CCD)

Lens

Figure 4: Non-Boresighted System of Sensors

The transformation which maps the data from one image perspective to another is described in MISB ST 1202.

##### 5.2.1.1 Single Point Range Measurement

In Range Imagery collected using a Collaborative Sensor, the Range Image dimensions (rows and columns) may not be the same as the as the Collaborative Sensor. A Range Image can be under-sampled or over-sampled as compared to the Collaborative Sensor’s Image. The minimum size of the Range Image can be as small as one range measurement; this is a special case of the Range Image data called a Single Point Range Measurement.

A Single Point Range Measurement is associated with a Collaborative Sensor’s Image. The Single Point Range Measurement is a measurement from either the perspective center, or Backplane of the Collaborative Sensor, through the image plane to a point on the scene. Because the Single Point Range Measurement may not be in the exact center of the Collaborative Sensor’s image, the row and column location of the image intersection pixel is also part of this measurement. When the Single Point Range Measurement passes through the center pixel of the image, the range measurement is called a Single Aim Center Pixel Range Measurement, or SACP Range Measurement. Figure 5 and Figure 6 illustrate a Single Point Range Measurement and a SACP Range Measurement respectively. The illustrations show the Single Point Range Measurements as Perspective Range Measurements; however, the Single Point Range Measurements can also be Depth Range Measurements.

Image Intersection Image Intersection Pixel (Off-Center) Pixel (On-Center) Single Point Range

Single Point Range Measurement Measurement

Collaborative

Collaborative

Sensor Image

Sensor Image

Figure 5: Single Point Range Measurement

Figure 6: SACP Range Measurement

#### 5.2.2 Computationally Extracted

Computationally Extracted Range Imagery is an alternative to active sensor measuring.

Computational extracted means inferring the range data from optical flow, or other image/data processing techniques.

### 5.3 Range Imagery Data Uncertainty

Since Range Imagery is a set of measured or computed distances, there are errors associated with these values. To provide the most utility of the Perspective Range or Depth Range data, this document includes the formatting method for the uncertainty of the Range Imagery at each pixel in the range image. The uncertainty is computed, estimated or measured (or a combination of the three) by the sensor developer. The uncertainty is a system’s ability to measure range reliably, and varies for each ranging system. The Range Measurement uncertainty is expressed as a standard deviation (sigma, i) in meters along the Range Measurement vector.

## 6 Range Motion Imagery Data Representation

The Range Image Data Representation has two use cases: Range Image and Single Point Range Measurement. When formatting a Range Image, a two-dimensional array of data along with its uncertainty, transformation information, and support values are packaged together. Alternatively, when transmitting Single Point Range Measurement (see Section 5.2.1.1), a single Range value along with its uncertainty, row and column locations are packaged together.

Range Motion Imagery is a series of Range Images (or Single Point Range Measurements) with each Range Image formatted using KLV in a Local Set (see SMPTE ST 336 [3]). The Range Motion Imagery Local Set contains different categories of data to support the two different use cases; these categories are listed in Table 1.

Table 1: Categories of data included in Range Motion Imagery Local Set

| Category | Description |
|---|---|
| Administration | Administrative Information for the Local Set (i.e. Version number and CRC). Required in all use cases. |
| Support | Information for describing the Range Motion Imagery (e.g. Image Time, Range Imagery Type, etc.). Required in all use cases. |
| Single Point Range | Information for describing and supporting the Single Point Range Measurement use case. |
| Range Image | Information for describing and supporting the Range Image Data use case. |

The metadata elements in the Range Motion Imagery Local Set are listed in Table 2, which has the following columns:

- Tag ID is the Local Set tag for the element value.
- Key is the KLV registry key for that element, either from SMPTE RP 210 [4] or MISB ST 0807 [5].
- Name is the name of the dictionary element (from the viewpoint of this document) along with the official name from the KLV registry (in parenthesis) if the names are different.
- Category indicates the purpose or use case of the data element and the section number for the full description.
- Data Type is the type of data for the element. Note: VLP = Variable Length Pack

Additionally, “Single Point Range Measurement” is abbreviated as SPRM in this table.

Table 2: Range Image Local Set

| Local Set Key | Local Set Name |
|---|---|
| 06.0E.2B.34.02.0B.01.01.0E.01.03.03.0C.00.00.00 (CRC 41152) | Range Image Local Set |

| Tag¹ | Name | Key | Category | Format | M/O |
|---|---|---|---|---|---|
| 1 | Range Image Precision Time Stamp | 06.0E.2B.34.01.01.01.03.07.02.01.01.01.05.00.00 (CRC 64827) | Support (6.2.1) | UINT64 | MAN. |
| 11 | Document Version | 06.0E.2B.34.01.01.01.01.0E.01.02.05.05.00.00.00 (CRC 56368) | Administration (6.1.1) | BER-OID | MAN. |
| 12 | Range Image Enumerations | 06.0E.2B.34.01.01.01.01.0E.01.02.03.60.00.00.00 (CRC 62498) | Support (6.2.2) | BER-OID | MAN. |
| 13 | SPRM (Range Measurement) | 06.0E.2B.34.01.01.01.01.0E.01.01.03.3E.04.00.00 (CRC 12643) | Single Point Range (6.3.1) | FLOAT | OPT. |
| 14 | SPRM Uncertainty (Range Measurement Uncertainty) | 06.0E.2B.34.01.01.01.01.0E.01.01.03.3E.05.00.00 (CRC 1619) | Single Point Range (6.3.2) | FLOAT | OPT. |
| 15 | SPRM Row Coordinate (Measured Row Coordinate for Range) | 06.0E.2B.34.01.01.01.01.0E.01.02.05.07.00.00.00 (CRC 12632) | Single Point Range (6.3.3) | FLOAT | OPT. |
| 16 | SPRM Column Coordinate (Measured Column Coordinate for Range) | 06.0E.2B.34.01.01.01.01.0E.01.02.05.08.00.00.00 (CRC 58806) | Single Point Range (6.3.3) | FLOAT | OPT. |
| 17 | Number of Sections in X | 06.0E.2B.34.01.01.01.01.0E.01.01.03.3E.00.00.00 (CRC 60835) | Range Image (6.4.2) | BER-OID | OPT. |
| 18 | Number of Sections in Y | 06.0E.2B.34.01.01.01.01.0E.01.01.03.3E.01.00.00 (CRC 55955) | Range Image (6.4.2) | BER-OID | OPT. |
| 19 | Generalized Transformation Local Set | 06.0E.2B.34.02.0B.01.01.0E.01.03.05.05.00.00.00 (CRC 40498) | Range Image (6.4.3) | Local Set | OPT. |
| 20 | Section Data Variable Length Pack | 06.0E.2B.34.02.04.01.01.0E.01.03.03.01.00.00.00 (CRC 4725) | Range Image (6.4.1) | VLP | OPT. |
| 21 | CRC-16-CCITT | 06.0E.2B.34.01.01.01.01.0E.01.02.03.5E.00.00.00 (CRC 31377) | Administration (6.1.2) | UINT16 | MAN. |

1 Some Tag ID’s have been deprecated from the Local Set because they were used in an earlier version, which has now been replaced by this version. If new items are added to this set do NOT use the following tag ids: 2, 3, 4, 5, 6, 7, 8,9,10, 51, 52, 53, or 54.

The Universal Label (Key) for the Range Image Local Set is (see MISB ST 0807 [5]):

06.0E.2B.34.02.0B.01.01.0E.01.03.03.0C.00.00.00 (CRC 41152)

The Range Image Local Set has the following data formatting requirement.

| Requirement |  |
|---|---|
| ST 1002.1-02 | All Range Image Local Set metadata shall be expressed in accordance with MISB ST 0107 [6]. |

### 6.1 Administrative Information

There are two Administrative Information items in the Range Image Local Set: Version Number and CRC Error Detection.

#### 6.1.1 Version Number

The Version Number of the Local Set is the same as the minor version number of this document. For example, with MISB ST 1002.1, the version number value is ‘1’, with ST 1002.2; the version number value is ‘2’, etc.

| Requirement |  |
|---|---|
| ST 1002.1-03 | A Range Image Local Set shall include a Version Number. |

A Range Imager Local Set parser should validate the version number to ensure that the parser is properly parsing the Local Set, and the Section Data Variable Length Pack (defined in Section 6.4.1).

#### 6.1.2 CRC Error Detection

To help detect erroneous metadata after transmission, a 2-byte CRC is included as the last item. The CRC is computed across the entire Local Set packet starting with the 16-byte Local Set key and ending with the length field of the CRC data element. Figure 7 illustrates the data range the checksum is performed over.

Local Set Key BER T|L|V T|L|V T|L|V

Value T | L | CRC

16-bytes Length Timestamp Metadata Metadata

CRC is Computed from the start of the 16 byte key through the Length Value of the CRC tag

Figure 7: Bytes included in the CRC computation

If the calculated CRC of a received packet does not match the CRC stored in the packet, the packet is discarded as being invalid.

| Requirement |  |
|---|---|
| ST 1002.1-04 | A Range Image Local Set shall have a CRC computed and included in accordance with MISB ST 1002. |

Refer to MISB RP 0701 [7] Appendix C for details and sample code for computing the CRC.

### 6.2 Range Motion Imagery Support Data

Range Motion Imagery requires support data to properly interpret the Range Imagery. Two support data elements are included in the Local Set: Range Image Precision Time Stamp and Range Encoding Enumerations.

#### 6.2.1 Range Image Precision Time Stamp

The Range Image Precision Time Stamp is the time when the measurements of the Range Image occurred. This time information is used to coordinate the Range Image with other sources of data, such as a collaborative sensors image or other sensor data. The time value is an invocation of the MISP Precision Time Stamp, a 64-bit unsigned integer that represents the number of microseconds since midnight of January 1 st 1980 without leap-seconds, as defined in MISB ST 0603 [8] and detailed in the Motion Imagery Handbook [9].

| Requirement(s) |  |
|---|---|
| ST 1002.1-05 | A Range Image Local Set shall include a Precision Time Stamp as defined in MISB ST 0603. |
| ST 1002.1-06 | The Precision Time Stamp shall appear as the first metadata item within a Range Image Local Set. |

Positioning the Precision Time Stamp tag as the first item facilitates rapidly checking whether the Local Set matches the desired time for processing a collaborative image.

#### 6.2.2 Range Image Enumerations

Range Image Enumerations is an integer value representing enumerated values. To enable possible future extension, this value is formatted as a BER-OID sub-identifier, so before the number is interpreted it needs to be converted to a standard integer (see [10] for further information). As shown in Table 3, Range Image Enumerations contains three separate enumerated values: Range Image Source, Range Image Data Type, and Range Image Compression Method.

Table 3: Range Image Enumerations

MSB ← Bits → LSB

| 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|
| Range Image Source | Range Imagery Data Type |  |  | Range Imagery Compression Method |  |  |
| 0 = Computationally Extracted<br>1 = Range Sensor | 000 = Perspective Range Image<br>001 = Depth Range Image<br>010 - 111 = Reserved |  |  | 000 = No Compression<br>001 = Planar Fit<br>010 - 111 = Reserved |  |  |

Range Image Source declares how the Range Imagery was created, either from a Range Sensor or Computationally Extracted, as described in Section 5. This enumeration has two values, so it consumes one bit of the Range Imager Enumerations value.

Range Imagery Data Type declares the type of Range Imagery, either Perspective Range Image or Depth Range Image, as described in Section 5. To allow for further types to be defined in the future this enumeration has eight values (0 through 7), where currently only two values are defined: 0=Perspective Range Image and 1=Depth Range Image.

Range Imagery Compression Method declares the method of compression used for reducing the number of bytes of the Range Image. One method of compression is called Planar Fit described in Section 7. To allow for further compression techniques to be defined in the future, this enumeration has eight values (0 through 7), where currently only two values are defined: 0=No Compression and 1=Planar Fit.

The Range Image Enumeration value is the combination of the Range Image Source, Range Imagery Data Type, and the Range Imagery Compression Method. The least significant 7 bits of the Range Imagery Enumeration value are shown in Table 3.

When the Range Image Enumeration value is formatted into BER-OID the eighth most significant bit, which is equal to zero (0), is added. It is possible that this value could grow to two or more bytes in future versions of this standard, in which case the BER-OID parsing rules would be used (see [10] for further information). When two or more bytes are used the least significant byte will always be parsed as described in Table 3; all future additions will add more significant bytes and bits.

### 6.3 Single Point Range Measurements

Single Point Range Measurements are one use case of the Range Image Local Set. Single Point Range Data is composed of the Single Point Range Measurement, Single Point Range Measurement Uncertainty and Single Point Range Measurement Row and Column Coordinates.

#### 6.3.1 Single Point Range Measurement

The Single Point Range Measurement is the measure of distance (in meters) from either the principle point, or Backplane of a Collaborative Sensor through the image plane to a point in the scene; see 5.2.1.1. This value can be either a 32-bit or 64-bit IEEE floating point value.

#### 6.3.2 Single Point Range Measurement Uncertainty

The Single Point Range Measurement Uncertainty is the uncertainty (sigma,$\sigma$) of the Single Point Range Measurement data, in meters, along the measured vector from either the perspective center or Backplane to the scene. This value is a can be either a 32-bit or 64-bit IEEE floating point value. The uncertainty is determined by the implementing system and must follow the guidelines in Section 5.3.

#### 6.3.3 Single Point Range Measurement Row and Column Coordinates

The Single Point Range Measurement is not necessarily measured directly through the center of the Collaborative Image; therefore, the location within the image needs to be indicated. The Single Point Range Measurement Row and Column Coordinates are the coordinates within the Collaborative Sensor’s Image where the measurement was taken (see Section 5.2.1.1). These values are either 32-bit or 64-bit IEEE floating point values. If the Row and Column values are omitted from the Range Image Local Set, then the default values are set to the center of the Collaborative Sensors Image.

| Requirement(s) |  |
|---|---|
| ST 1002.1-07 | When Single Point Range Measurement Row Coordinate is not included in the Range Image Local Set, then the default value shall be set to the center Row of the image. |
| ST 1002.1-08 | When Single Point Range Measurement Column Coordinate is not included in the Range Image Local Set, then the default value shall be set to the center Column of the image. |

### 6.4 Range Image

Range Images are one use case of the Range Imagery Local Set, where more than one range measurement is obtained. A Range Image is an array of Range Measurements each of which is the measure of the distance (in meters) from either the principle point, or Backplane to a point in the scene (see Section 5.2.1.1). The Range Image is formatted as a two-dimensional array of data for a given time period (see Section 6.2.1).

Range Imagery Data is a rectangular array of Range Measurements as described in Section 5.1. Range Imagery Data can be formatted in whole or in separate parts, where each part is called a Section. Sections are rectangular areas that when combined together form the full image. Each Section can be compressed to provide the most efficient transmission and storage. Sections have two different layouts, simple or complex. A simple Section layout divides the image into either horizontal or vertical strips. All horizontal strips have the same width as the full image but can vary in height as needed as illustrated in Figure 8. All vertical strips have the same height as the full image but can vary in width as illustrated in Figure 9.

Full Image Width Full Image Width 1,1 2,1 Number of Sections in Y Number of Sections in Y

Full Image Height Full Image Height

3,1 4,1 1,1 1,2 1,3 (5) (1) 5,1 Number of Sections in X Number of Sections in X (1) (3)

Figure 8: Illustration of five Horizontal Sections

Figure 9: Illustration of three Vertical Sections

A complex Section layout divides the image into various size rectangles where each rectangle does not necessarily span the full image width or height as illustrated in Figure 10.

1,1 1,2 2,1 2,2 2,3 3,1 3,2

Figure 10: Illustration of a Complex Section layout

Currently, this standard only allows simple Section layout; however complex Section layout may be allowed in the future.

| Requirement |  |
|---|---|
| ST 1002.1-09 | Range Images shall only be formatted using the Simple Section layout method. |

When the Range Image is formatted into separate Sections, the Range Image Local Set will contain multiple Section Data Variable Length Packs. Specifically, the number of Section Data Variable Length Packs will be the Number of Sections in X multiplied by the Number of Sections in Y.

The Range Image use case includes additional Range Image parameters to help compress and interpret the Range Image. The Range Image use case consists of the Section Data, Number of Sections in X and Y, and Generalized Transformation data.

#### 6.4.1 Section Data Variable Length Pack

Section data, along with its supporting information is formatted in a Variable Length Pack (VLP) (see SMPTE ST 336) called the Section Data VLP. The information in each Section Data VLP includes: Section coordinates, Section data array, uncertainty values, and optional compression parameters. The optional compression parameters are truncated from the VLP (along with their lengths) as a group (in a similar fashion to MISB RP 0701 Floating Length Packs). Figure 11 illustrates a Section Data VLP with each item in the VLP prefixed with its item length. The green items indicate required values (along with their lengths in blue); the pink item can be “zero-ized” (see Section 6.4.1.3); and, the yellow items can be truncated (along with their lengths).

Data Section Variable Length Pack (VLP)

| Key | Length | L1 | Section # X | L2 | Section # Y | L3 | Array of Measured Values | L4 | Array of Uncertainty Values | L5 | Plane X Scale Factor | L6 | Plane Y Scale Factor | L7 | Plane Constant Value |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|  |  |  | (BER-OID) |  | (BER-OID) |  | (MDARRAY – ST 1303) |  | (MDARRAY – ST 1303) |  | (FLOAT) |  | (FLOAT) |  | (FLOAT) |

Color Code: KLV Key | VLP Length | Item Length | Required | Required but can be zero length | Optional

Figure 11: Illustration of Section Data Variable Length Pack

The items contained in the Section Data VLP are listed in Table 4, which has the following columns:

- Key is the KLV registry key for that item, found in SMPTE RP 210 or MISB ST 0807.
- Name is the name of the registry element (from the viewpoint of this document) along with the official name from the KLV registry (in parenthesis) if the names are different.
- Notes provides additional information about the value.
- Format is the data type used for the value.
- RZO indicates if a value is Mandatory (M), Zeroizable (Z) or Optional (O).

Table 4: Section Data Variable Length Pack (VLP)

| Key | Name | Symbol/Notes | Format | MZO |
|---|---|---|---|---|
| 06.0E.2B.34.01.01.01.01.0E.01.01.03.3E.02.00.00 (CRC 33731) | Section Number X | Coordinates of the Section within the full image | BER-OID | M |
| 06.0E.2B.34.01.01.01.01.0E.01.01.03.3E.03.00.00 (CRC 46323) | Section Number Y |  | BER-OID | M |
| 06.0E.2B.34.02.05.01.01.0E.01.03.03.06.00.00.00 (CRC 39697) | Array of Range Measurements | Formatted in ST 1303 [11] (Includes dimensions, and IMAPB parameters) | MDARRAY (See Section 6.4.1.2) | M |
| 06.0E.2B.34.02.05.01.01.0E.01.03.03.06.00.00.00 (CRC 39697) | Array of Uncertainty Values | Formatted in ST1303 [11] (Includes dimensions, and IMAPB parameters) | MDARRAY (See Section 6.4.1.3) | Z |
| 06.0E.2B.34.01.01.01.01.0E.01.02.02.81.2D.00.00 (CRC 26810) | Plane X-scale Factor | a in Equation 8 | Float 32 or Float 64 | O |
| 06.0E.2B.34.01.01.01.01.0E.01.02.02.81.2E.00.00 (CRC 12778) | Plane Y-scale Factor | b in Equation 8 | Float 32 or Float 64 | O |
| 06.0E.2B.34.01.01.01.01.0E.01.02.02.81.2F.00.00 (CRC 1754) | Plane constant value | c in Equation 8 | Float 32 or Float 64 | O |

##### 6.4.1.1 Section Coordinates

Section Coordinates define the location of where, in the full image data, the Section belongs. Section numbers always begin in the upper left of the full image and increment towards the lower right. With simple Section Layouts, one of the Section Coordinates will always be one. For example, Figure 8 shows five sections, where sending the bottommost Section, Section Number X equals 1 and Section Number Y equals 5. To reassemble the full image, all Sections need to be assembled in order from the lowest value X/Y Section Coordinates to the largest X/Y Section Coordinates. The Section Number X and Y values are encoded as BER-OID sub-identifier values, so they can accommodate any Section size.

| Requirement(s) |  |
|---|---|
| ST 1002.1-10 | Section Data Variable Length Packs shall always contain Section Number X and Section Number Y. |
| ST 1002.1-11 | One of the Section Coordinate Values, either Section Number X or Section Number Y, shall be one (1). |
| ST 1002.1-12 | When reassembling a full image all Sections shall be assembled in order from the lowest value X/Y Section Coordinates to the largest X/Y Section Coordinates. |

##### 6.4.1.2 Array of Range Measurements

Array of Range Measurements contains either the original range measurements, or the adjusted range measurements produced from the plane subtraction processing (see Section 7). Any adjusted range measurements in the array are computed using Equation 9. In order to reverse the computations, the compression parameters are used from Section 6.4.1.4.

The array is formatted according to MISB ST 1303, and optionally includes the MISB ST 1201 [12] IMAPB Float-to-Integer min, max values. The ST 1303 MDARRAY parameters for the Data Array are:

MDARRAY(06.0E.2B.34.01.01.01.01.0E.01.01.03.3E.04.00.00 (CRC 12643), 2, NoteA, NoteA, NoteB, NoteC, NoteC)

NoteA: This value is system dependent (i.e. dependent on the size of the sensor); the system provider specifies the proper size.

NoteB: The Element Size is based on whether the system provider is formatting the data as floating-point values or packing the values using IMAPB. When using floating point values, the Element Size will be either 4 or 8 bytes. When using IMAPB the system provider specifies the appropriate precision for the system along with the minimum and maximum values of the array (see NoteC) to compute the appropriate Element Size.

NoteC: The Min and Max values are determined by the bounds of the array data before mapping is applied. For Plane Subtraction the Min, Max values are computed after the Plane Subtraction has been performed.

The Range Image Data has a reserved item to represent the Positive Quiet Not-a-Number (+QNaN) value. The integer representation of values with reserved bits is described in MISB ST 1201. The +QNaN signifies that no reliable range information exists at a particular pixel within the range image.

##### 6.4.1.3 Array of Uncertainty Values

The Array of Uncertainty Values is formatted using MISB ST 1303 and optionally includes the IMAPB min, max values.

| Requirement |  |
|---|---|
| ST 1002.1-13 | The number of elements in the Section Data VLP - Array of Uncertainty Values shall match the number of elements in the Array of Measured Values. |

The ST 1303 MDARRAY parameters for the Uncertainty Array are:

MDARRAY(06.0E.2B.34.01.01.01.01.0E.01.01.03.3E.05.00.00 (CRC 1619), 2, NoteD, NoteD, NoteE, NoteF, NoteF)

NoteD: This value is system dependent (i.e. dependent on the size of the sensor); the system provider determines the proper size. This value must match the Data Array value in Section 6.4.1.2 - NoteA.

NoteE: The Element Size is based on whether the system provider is formatting the data as floating point values or packing the values using IMAPB. When using floating point values, the Element Size will be either 4 or 8 bytes. When using IMAPB the system provider specifies the appropriate uncertainty precision for the specific system along with the minimum and maximum values of the array (see NoteC) to compute the appropriate Element Size.

NoteF: The Min and Max values are determined by the bounds of the array data before mapping is applied; for uncertainty data the smallest Min value allowed is zero.

| Requirement(s) |  |
|---|---|
| ST 1002.1-14 | The smallest Min value for the Range Measurement Uncertainly data shall be zero (0). |
| ST 1002.1-15 | If a Range Image does not have an array of uncertainty values, then the length for this Variable Length Pack item shall be set to zero. |

The Range Image Uncertainty data has a reserved item to represent the Positive Quiet Not-a-Number (+QNaN) value. The integer representation of values with reserved bits is described in MISB ST 1201. The +QNaN signifies that no reliable uncertainty information exists at a particular pixel within the range image.

When a Range Image does not have an array of uncertainty values, the length for this Variable Length Pack item is set to zero. When parsing a Section Data VLP, a zero length for the Array of Uncertainty Values indicates there is not an uncertainty array “value”, so the next item in the Section Data VLP is processed.

##### 6.4.1.4 Compression Parameters

The optional compression parameters are values used to perform Plane Subtraction (see Section 7) on the Section data array. Plane Subtraction reduces the dynamic range of the Range Measurement values, thereby reducing the number of bytes needed to represent each value. Plane Subtraction only applies to the Section Data array; not the Section Data uncertainty values. Plane Subtraction is performed by first computing a reference plane based on a two-dimensional least-squares estimate of a Section’s data, and then subtracting the reference plane from the range values in the Section. This reduces a larger dynamic range of values, which can be further processed using the Floating Point to Integer Mapping (MISB ST 1201) referenced in MISB ST 1303. Plane Subtraction is fully described in Section 7. Three compression parameters are specified: Plane X-scale (a in Equation 8), Plane Y-scale Factor (b in Equation 8), and Plane constant value (c in Equation 8). Each of these values is either a 32-bit or 64-bit IEEE floating point value.

#### 6.4.2 Number of Sections in X and Y

The full Range Image (and Range Uncertainty) if divided into parts -- called Sections may enable better Plane Subtraction algorithm performance. The Number of Sections in X is the count of Sections in the X direction, and the Number of Sections in Y is the count of Sections in the Y direction. For this version of the standard, only the simple Section layout is used, so either the Number of Sections in X or the Number of Sections in Y must be one as shown in Figure 8 and Figure 9. For example, in Figure 8, the Number of Sections in X equals one and the Number of Sections in Y equals five. In Figure 9, the Number of Sections in X equals 3 and the Number of Sections in Y equals 1. Each value is an unsigned integer, represented in BER-OID sub-identifier format. If the Number of Sections in X or Number of Sections in Y are not included in the Local Set, then the values are defaulted to be one (1).

#### 6.4.3 Generalized Transformation

The Generalized Transformation is a mathematical transformation used to project information, points, or lines from one image plane into a second image plane. The use of the Generalized Transformation Local Set, MISB ST 1202, is for specific cases when aligning a Range Image to a Collaborative Sensors Image. In this case, the Range Image is said to be a child of the Collaborative Image (i.e. the parent image), so the Child-Parent Transformation (CPT) enumeration 2 defined in Table 1 of MISB ST 1202, is used.

The boresighted imaging case is a hardware solution, where multiple focal plane arrays are simultaneously imaging through a single aperture, see Section 5.2. The hardware limits the variation in the perspective centers and the principal axis of the system to be coincident. Therefore, the three-dimensional scene is imaged on two, dependent, focal plane arrays, where the dependency is in the geometry. These focal planes do not necessarily have the same orientation or identical pixel sizes. Also, the magnification of the two optical paths can be different causing different image scales. These effects can be sufficiently modeled using the Generalized Transformation. The Value portion of the ST 1202 Local Set is directly inserted into the Range Image Local Set without any changes.

## 7 Plane Subtraction Compression

Range Images have a unique property such that the number of bytes can be reduced when transmitting or storing the data. Typically, the surfaces of a scene are fairly regular, so the Range Image data is relatively uniform (especially in a local area) and smooth (unlike an optical image). For example, an optical image may have textures where adjacent pixels vary greatly in value (i.e.

a white pixel next to a black pixel next to a red pixel, etc.). With Range Images the adjacent pixels are relatively close in value (i.e. 120 meters, next to 125 meters, next to 115 meters, etc.) Figure 12 illustrates a Range Image of the ground from an airborne system.

Top View Side View

Range Image

Figure 12: Illustration of Airborne Range Image

Because of this relative uniformity, Range Measurements can be fitted to mathematical plane, which can be subtracted from the original Range Image producing a residual image with smaller numerical range. The individual values then require less bytes providing a level of compression over the original Range Image. A simplified example is a one-dimensional set of Range Data as shown Figure 13.

In Figure 13, 1024 Range Measurements (blue lines) are plotted. Three of the range measurements (red, purple and green lines) show the relationship between the physical Range values and the graph. In this example, assume that the Range value is accurate to one meter and that the number of bytes to store each range value is based on the dynamic range of the measurements, which in this case is 545 meters. The value of 545 meters requires two bytes (using integers). The total number of bytes for this set of range values is, therefore, 1024 measures x 2 bytes per measure = 2048 bytes.

By fitting a line to this data set (using a least squares estimate), as shown in Figure 14, and then subtracting each range point from the line estimate, the dynamic range is reduced to 76 meters, which only requires one byte per measurement.

Side View Range Examples 600 Range Measurement 500 400 300 200 Ranges are not to Scale 100 0 100 200 300 400 500 600 700 800 900 1000 Sample Number

Figure 13: One-Dimensional Range Measurements Illustration

Original Range Values (R) & Linear Squares Estimate Adjusted Range Values (T) Min=100, Max=645 Min=-26, Max=50 Dynamic Range = 545 Dynamic Range = 76 600 600 500 500

Range Measure Range Measure

400 400 300 300 200 200 100 100 0 0 0 200 400 600 800 1000 0 200 400 600 800 1000 Sample Number Sample Number

Figure 14: Illustration of Dynamic Range Reduction (The values in the graph on the right (Adjusted Range Values) were increased by the Minimum (-26) so all the values on the blue line would be visible in a graph with the same dynamic range as the original data on the left.)

The fitted line information must accompany the adjusted-valued data in order to reconstruct the original data; this information requires 4 bytes (2 bytes for the slope and 2 bytes for the y-intercept of a line equation of the form y = ax + b). The total number of bytes for the adjusted range values is: 1024 measures x 1 byte per measure + 2 * 2 bytes = 1028 bytes, which reduces the overall number of bytes by nearly 50%.

The greatest benefit is gained when the Range Measurements demonstrate a gradual slope with minimal perturbing features. Airborne examples of gradually sloping terrain are: (1) flat, level ground viewed from an oblique angle; and (2) a hill or mountain side.

The mathematical algorithm for this example is shown in the following steps:

1) Let R be the Range Measurements ri. F is the location, i, of each ri in the series along with a constant value of one.

$$ F = \begin{bmatrix}1 & 1\\ 2 & 1\\ \vdots & \vdots\\ N & 1\end{bmatrix} \qquad \text{Equation 1} $$

2) Compute the linear least squares fit, generating the a, b coefficients for the $y = a * i + b$ linear estimate equation.

$$ \begin{bmatrix}a\\ b\end{bmatrix} = (F^{T} * F)^{-1} * F^{T} * R \qquad \text{Equation 2} $$

3) For each Range Measurement subtract the linear estimate. Let P be the adjusted Range Measurements, pi.

$$ p_i = r_i - (a * i + b) \qquad \text{Equation 3} $$

### 7.1 Plane Subtraction

The previous one-dimensional case can be extended to two dimensions; however, with actual range data it is possible to have unknown values in the Range Measurements. To compensate for missing data a data mask, M, is created to “zero out” and essentially eliminate these values from the computations. The mathematical algorithm is shown below in the following steps:

1) Compute Mask M from Range Measurements

$$ M = \begin{bmatrix}m_{1,1}\\ m_{2,1}\\ m_{3,1}\\ \vdots\\ m_{N,1}\\ m_{1,2}\\ m_{2,2}\\ \vdots\\ m_{N,2}\\ m_{1,3}\\ \vdots\\ m_{N,M}\end{bmatrix} \qquad \text{Equation 4} $$

$$ \text{Where } m_{i,j} = \begin{cases}1 & \text{if } r_{i,j} \text{ is a valid value}\\ 0 & \text{if } r_{i,j} \text{ is a NaN value}\end{cases} \qquad \text{Equation 5} $$

2) Let R be the Range Measurements of a two-dimensional NxM image with each element denoted as ri,j. $\vec{R}$ is formed from R by vectorizing the image into one long vector and applying the mask. Note: $\circledast$ is point-wise multiplication.

$$ \vec{R} = \begin{bmatrix}r_{1,1}\\ r_{2,1}\\ r_{3,1}\\ \vdots\\ r_{N,1}\\ r_{1,2}\\ r_{2,2}\\ \vdots\\ r_{N,2}\\ r_{1,3}\\ \vdots\\ r_{N,M}\end{bmatrix} \circledast M \qquad \text{Equation 6} $$

3) F is the location, i,j of each ri,j in the series along with a constant value of one. Note: $\circledast$ is point-wise multiplication.

$$ F = \begin{bmatrix}1 & 1 & 1\\ 2 & 1 & 1\\ 3 & 1 & 1\\ \vdots & \vdots & \vdots\\ N & 1 & 1\\ 1 & 2 & 1\\ 2 & 2 & 1\\ \vdots & \vdots & \vdots\\ N & 2 & 1\\ 1 & 3 & 1\\ \vdots & \vdots & \vdots\\ N & M & 1\end{bmatrix} \circledast \begin{bmatrix}M & M & M\end{bmatrix} \qquad \text{Equation 7} $$

4) Compute the planer least squares fit using $y = a * i + b * j + c$ to compute the a, b and c coefficients for the planer estimate equation.

$$ \begin{bmatrix}a\\ b\\ c\end{bmatrix} = (F^{T} * F)^{-1} * F^{T} * \vec{R} \qquad \text{Equation 8} $$

5) For each Range Measurement subtract the planer estimate. Let P be the Adjusted Range Measurements with each element of the image denoted as pi,j.

$$ p_{i,j} = r_{i,j} - (a * i + b * j + c) \qquad \text{Equation 9} $$

Normally, it is not recommended to use least squares to produce the coefficients because of poor conditioning1 of the matrix in Equation 8; however, in this situation the results only produce a reference plane estimate that is used in reverse (Equation 10) for the decompressing and obtaining the original Range Measurements. In this case, the complete accuracy of the estimate is not a concern with this type of data. The least squares method is employed here because of its simplicity; other methods, such as QR factorization or SVD, can be used to get better estimates of the coefficients with a cost in complexity and computation time.

### 7.2 Reverse Plane Subtraction

To reverse the plane subtraction processing the following equation is used:

$$ r_{i,j} = p_{i,j} + (a * i + b * j + c) \qquad \text{Equation 10} $$

The parameters a, b and c are stored with the adjusted range measurements.

## 8 Deprecated Requirements

The following requirement is not needed as this UL Key is defined in the MISB ST 0807 registry.

| Requirement |  |
|---|---|
| ST 1002.1-01 (Deprecated) | The Range Image Local Set shall have the UL Key 06.0E.2B.34.02.0B.01.01.0E.01.03.03.0C.00.00.00 (CRC 41152). |

1 See advanced Linear Algebra textbooks for discussion on Condition Numbers of matrices.
