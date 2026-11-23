//! MISB ST 0903.6 VMTI Local Set — a typed, borrowed view.
//!
//! The `misb` crate parses the Local Set framing, the VTarget packs and their
//! typed items; this module exposes that model without leaking the crate's
//! types. ST 0601 carries the set as tag 74, so it is reached through
//! [`NestedSet::vmti`](crate::NestedSet::vmti) after [`decode`](crate::decode).
//!
//! Item numbers, names and worked example encodings:
//! `private/specs-verified/misc/misb/st0903-vmti-ls.md` (MISB ST 0903.6
//! Tables 9, 10, 16 and 17 and §10 item definitions).
//!
//! Items the decoder does not interpret stay raw and are reachable through
//! [`St0903Vmti::raw`] and [`St0903Target::undecoded`]; nothing below decodes
//! them by hand.
//!
//! `no_std` + `alloc`.

use core::fmt;

use broadcast_common::impl_spec_display;

/// A VMTI LS item that `misb` leaves as raw bytes (ST 0903.6 items 13, 102
/// and 103).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum St0903VmtiItem {
    /// MIIS Core Identifier (MISB ST 1204 binary identifier).
    MiisId,
    /// Series of Algorithm Local Sets.
    AlgorithmSeries,
    /// Series of Ontology Local Sets.
    OntologySeries,
}

impl St0903VmtiItem {
    /// The `name()` label.
    pub fn name(&self) -> &'static str {
        match self {
            Self::MiisId => "miisId",
            Self::AlgorithmSeries => "algorithmSeries",
            Self::OntologySeries => "ontologySeries",
        }
    }
}

impl_spec_display!(St0903VmtiItem);

/// A VTarget item that `misb` leaves as raw bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum St0903TargetItem {
    /// Percentage of Target Pixels (item 7).
    PercentageOfTargetPixels,
    /// Target Intensity (item 9).
    TargetIntensity,
    /// Bounding Box Top Left Latitude Offset (item 13).
    BoundingBoxTopLeftLatOffset,
    /// Bounding Box Top Left Longitude Offset (item 14).
    BoundingBoxTopLeftLonOffset,
    /// Bounding Box Bottom Right Latitude Offset (item 15).
    BoundingBoxBottomRightLatOffset,
    /// Bounding Box Bottom Right Longitude Offset (item 16).
    BoundingBoxBottomRightLonOffset,
    /// Geospatial Contour Series (item 18).
    GeospatialContourSeries,
    /// Detection Status (item 23).
    DetectionStatus,
    /// Target perimeter mask (item 101).
    VMask,
    /// Track information (item 104).
    VTracker,
    /// Underlying pixel values (item 105).
    VChip,
    /// Series of VChip Local Sets (item 106).
    VChipSeries,
    /// Series of VObject Local Sets (item 107).
    VObjectSeries,
}

impl St0903TargetItem {
    /// The `name()` label.
    pub fn name(&self) -> &'static str {
        match self {
            Self::PercentageOfTargetPixels => "percentageOfTargetPixels",
            Self::TargetIntensity => "targetIntensity",
            Self::BoundingBoxTopLeftLatOffset => "boundingBoxTopLeftLatOffset",
            Self::BoundingBoxTopLeftLonOffset => "boundingBoxTopLeftLonOffset",
            Self::BoundingBoxBottomRightLatOffset => "boundingBoxBottomRightLatOffset",
            Self::BoundingBoxBottomRightLonOffset => "boundingBoxBottomRightLonOffset",
            Self::GeospatialContourSeries => "geospatialContourSeries",
            Self::DetectionStatus => "detectionStatus",
            Self::VMask => "vMask",
            Self::VTracker => "vTracker",
            Self::VChip => "vChip",
            Self::VChipSeries => "vChipSeries",
            Self::VObjectSeries => "vObjectSeries",
        }
    }

    fn tag(self) -> u32 {
        match self {
            Self::PercentageOfTargetPixels => 7,
            Self::TargetIntensity => 9,
            Self::BoundingBoxTopLeftLatOffset => 13,
            Self::BoundingBoxTopLeftLonOffset => 14,
            Self::BoundingBoxBottomRightLatOffset => 15,
            Self::BoundingBoxBottomRightLonOffset => 16,
            Self::GeospatialContourSeries => 18,
            Self::DetectionStatus => 23,
            Self::VMask => 101,
            Self::VTracker => 104,
            Self::VChip => 105,
            Self::VChipSeries => 106,
            Self::VObjectSeries => 107,
        }
    }
}

impl_spec_display!(St0903TargetItem);

/// A WGS84 position decoded from a VTarget location pack.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct St0903GeoPoint {
    lat: f64,
    lon: f64,
    height: Option<f64>,
}

impl St0903GeoPoint {
    /// A position in degrees and metres above the WGS84 ellipsoid.
    pub fn new(lat: f64, lon: f64, height: Option<f64>) -> Self {
        Self { lat, lon, height }
    }

    /// Latitude in degrees.
    pub fn lat(&self) -> f64 {
        self.lat
    }

    /// Longitude in degrees.
    pub fn lon(&self) -> f64 {
        self.lon
    }

    /// Height above the WGS84 ellipsoid in metres, when the pack carries it.
    pub fn height(&self) -> Option<f64> {
        self.height
    }
}

fn geo_from_misb(point: misb::GeoPoint) -> St0903GeoPoint {
    St0903GeoPoint::new(point.lat, point.lon, point.height)
}

fn geo_into_misb(point: St0903GeoPoint) -> misb::GeoPoint {
    misb::GeoPoint {
        lat: point.lat(),
        lon: point.lon(),
        height: point.height(),
    }
}

/// One VTarget record: the pack's `targetId` and its typed items.
#[derive(Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct St0903Target<'a> {
    target: misb::st0903::VTarget<'a>,
}

impl<'a> St0903Target<'a> {
    /// The target ID (BER-OID encoded in the pack).
    pub fn id(&self) -> u64 {
        self.target.id
    }

    /// Target Centroid as a pixel number (item 1).
    pub fn centroid_pixel(&self) -> Option<u64> {
        self.target.centroid_pixel()
    }

    /// Centroid as 1-based `(row, column)`: items 19/20 when present,
    /// otherwise derived from the pixel number and `frame_width`.
    pub fn centroid_row_col(&self, frame_width: u64) -> Option<(u64, u64)> {
        self.target.centroid_row_col(frame_width)
    }

    /// Bounding box as pixel numbers (items 2 and 3).
    pub fn bounding_box(&self) -> Option<(u64, u64)> {
        self.target.bounding_box()
    }

    /// Target Priority (item 4).
    pub fn priority(&self) -> Option<u8> {
        self.target.priority()
    }

    /// Target Confidence Level in percent (item 5).
    pub fn confidence(&self) -> Option<u8> {
        self.target.confidence()
    }

    /// Target History in frames (item 6).
    pub fn history(&self) -> Option<u64> {
        self.target.history()
    }

    /// Target Colour as `0xRRGGBB` (item 8).
    pub fn color(&self) -> Option<u32> {
        self.target.color()
    }

    /// Target Location Offset in degrees from the frame centre (items 10 and
    /// 11).
    pub fn location_offset(&self) -> Option<(f64, f64)> {
        self.target.location_offset()
    }

    /// Target Height above the WGS84 ellipsoid in metres (item 12).
    pub fn height(&self) -> Option<f64> {
        self.target.height()
    }

    /// Target Location pack (item 17). The pack's optional sigma/rho fields
    /// are not decoded.
    pub fn location(&self) -> Option<St0903GeoPoint> {
        self.target.location().map(geo_from_misb)
    }

    /// The ground position: item 17 when present, otherwise the offsets of
    /// items 10/11 added to `frame_center`.
    pub fn ground_position(&self, frame_center: Option<St0903GeoPoint>) -> Option<St0903GeoPoint> {
        self.target
            .ground_position(frame_center.map(geo_into_misb))
            .map(geo_from_misb)
    }

    /// Algorithm ID (item 22).
    pub fn algorithm_id(&self) -> Option<u64> {
        self.target.algorithm_id()
    }

    /// The raw value of an item `misb` leaves undecoded; `None` when absent.
    /// The bytes are not interpreted here.
    pub fn undecoded(&self, item: St0903TargetItem) -> Option<&'a [u8]> {
        self.target
            .items()
            .flatten()
            .find(|raw| raw.tag == item.tag())
            .map(|raw| raw.value)
    }
}

impl fmt::Debug for St0903Target<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("St0903Target")
            .field("id", &self.id())
            .field("centroid_pixel", &self.centroid_pixel())
            .field("bounding_box", &self.bounding_box())
            .field("priority", &self.priority())
            .field("confidence", &self.confidence())
            .field("history", &self.history())
            .field("color", &self.color())
            .field("location_offset", &self.location_offset())
            .field("height", &self.height())
            .field("location", &self.location())
            .field("algorithm_id", &self.algorithm_id())
            .finish()
    }
}

/// Iterator over the VTarget packs of a [`St0903Vmti`].
#[derive(Clone)]
#[non_exhaustive]
pub struct St0903Targets<'a> {
    targets: misb::st0903::Targets<'a>,
}

impl<'a> Iterator for St0903Targets<'a> {
    type Item = Result<St0903Target<'a>, St0903TargetError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.targets.next().map(|result| {
            result
                .map(|target| St0903Target { target })
                .map_err(|_| St0903TargetError)
        })
    }
}

/// A VTarget pack did not parse (malformed series length, length or target
/// ID). The underlying framing error's detail is not exposed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct St0903TargetError;

impl fmt::Display for St0903TargetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("malformed ST 0903 VTarget pack")
    }
}

impl core::error::Error for St0903TargetError {}

/// A decoded MISB ST 0903.6 VMTI Local Set (ST 0601 tag 74).
#[derive(Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct St0903Vmti<'a> {
    set: misb::st0903::VmtiSet<'a>,
}

impl<'a> St0903Vmti<'a> {
    pub(crate) fn from_misb(set: misb::st0903::VmtiSet<'a>) -> Self {
        Self { set }
    }

    /// Precision Time Stamp (item 2), microseconds since 1970 UTC.
    pub fn timestamp(&self) -> Option<u64> {
        self.set.timestamp()
    }

    /// VMTI System Name (item 3).
    pub fn system_name(&self) -> Option<&'a str> {
        self.set.system_name()
    }

    /// VMTI LS Version (item 4).
    pub fn version(&self) -> Option<u64> {
        self.set.version()
    }

    /// Total Number of Targets Detected (item 5).
    pub fn total_targets_detected(&self) -> Option<u64> {
        self.set.total_targets_detected()
    }

    /// Number of Targets Reported (item 6).
    pub fn targets_reported(&self) -> Option<u64> {
        self.set.targets_reported()
    }

    /// Motion Imagery Frame Number (item 7, deprecated in ST 0903.6).
    pub fn frame_number(&self) -> Option<u64> {
        self.set.frame_number()
    }

    /// Frame Width in pixels (item 8).
    pub fn frame_width(&self) -> Option<u64> {
        self.set.frame_width()
    }

    /// Frame Height in pixels (item 9).
    pub fn frame_height(&self) -> Option<u64> {
        self.set.frame_height()
    }

    /// VMTI Source Sensor (item 10).
    pub fn source_sensor(&self) -> Option<&'a str> {
        self.set.source_sensor()
    }

    /// Horizontal field of view in degrees (item 11).
    pub fn horizontal_fov(&self) -> Option<f64> {
        self.set.horizontal_fov()
    }

    /// Vertical field of view in degrees (item 12).
    pub fn vertical_fov(&self) -> Option<f64> {
        self.set.vertical_fov()
    }

    /// The targets in the VTarget Series (item 101).
    pub fn targets(&self) -> St0903Targets<'a> {
        St0903Targets {
            targets: self.set.targets(),
        }
    }

    /// The raw value of an item `misb` leaves undecoded; `None` when absent.
    /// The bytes are not interpreted here.
    pub fn raw(&self, item: St0903VmtiItem) -> Option<&'a [u8]> {
        self.set.raw(match item {
            St0903VmtiItem::MiisId => misb::st0903::Tag::MiisId,
            St0903VmtiItem::AlgorithmSeries => misb::st0903::Tag::AlgorithmSeries,
            St0903VmtiItem::OntologySeries => misb::st0903::Tag::OntologySeries,
        })
    }
}

impl fmt::Debug for St0903Vmti<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("St0903Vmti")
            .field("timestamp", &self.timestamp())
            .field("system_name", &self.system_name())
            .field("version", &self.version())
            .field("total_targets_detected", &self.total_targets_detected())
            .field("targets_reported", &self.targets_reported())
            .field("frame_number", &self.frame_number())
            .field("frame_width", &self.frame_width())
            .field("frame_height", &self.frame_height())
            .field("source_sensor", &self.source_sensor())
            .field("horizontal_fov", &self.horizontal_fov())
            .field("vertical_fov", &self.vertical_fov())
            .field("targets", &self.targets().count())
            .finish()
    }
}
