//! MISB ST 0903.6 VMTI Local Set — a typed, borrowed view.
//!
//! The `misb` crate parses the Local Set framing, the VTarget packs and their
//! typed items; this module exposes that model without leaking the crate's
//! types. ST 0601 carries the set as tag 74, so it is reached through
//! [`NestedSet::vmti`](crate::NestedSet::vmti) after [`decode`](crate::decode).
//!
//! Item numbers, names and worked example encodings: MISB ST 0903.6 Tables 9,
//! 10, 16 and 17 and the §10 item definitions.
//!
//! Items the decoder does not interpret stay raw and are reachable through
//! [`St0903Vmti::raw`] and [`St0903Target::undecoded`]; nothing below decodes
//! them by hand.
//!
//! `no_std` + `alloc`.

use core::fmt;

use broadcast_core::impl_spec_display;

use crate::geo::{self, GeoPoint};

/// A VMTI LS item that `misb` leaves as raw bytes (ST 0903.6 item 13).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum St0903VmtiItem {
    /// MIIS Core Identifier (MISB ST 1204 binary identifier).
    MiisId,
}

impl St0903VmtiItem {
    /// The `name()` label.
    pub fn name(&self) -> &'static str {
        match self {
            Self::MiisId => "miisId",
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

/// A VTarget location position: the same shape as [`GeoPoint`].
pub type St0903GeoPoint = GeoPoint;

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
        self.target.location().map(geo::from_misb)
    }

    /// The ground position: item 17 when present, otherwise the offsets of
    /// items 10/11 added to `frame_center`.
    pub fn ground_position(&self, frame_center: Option<St0903GeoPoint>) -> Option<St0903GeoPoint> {
        self.target
            .ground_position(frame_center.map(geo::into_misb))
            .map(geo::from_misb)
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

/// An Algorithm or Ontology LS in a series did not parse (malformed length).
/// The underlying framing error's detail is not exposed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct St0903SeriesError;

impl fmt::Display for St0903SeriesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("malformed ST 0903 series element")
    }
}

impl core::error::Error for St0903SeriesError {}

/// One Algorithm LS (ST 0903.6 Table 16): the detector or tracker that
/// produced targets.
#[derive(Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct St0903Algorithm<'a> {
    algorithm: misb::st0903::Algorithm<'a>,
}

impl<'a> St0903Algorithm<'a> {
    fn from_misb(algorithm: misb::st0903::Algorithm<'a>) -> Self {
        Self { algorithm }
    }

    /// Algorithm ID (item 1), the value targets refer to.
    pub fn id(&self) -> Option<u64> {
        self.algorithm.id()
    }

    /// Algorithm name (item 2).
    pub fn name(&self) -> Option<&'a str> {
        self.algorithm.name()
    }

    /// Algorithm version (item 3).
    pub fn version(&self) -> Option<&'a str> {
        self.algorithm.version()
    }

    /// Algorithm type, such as detector or classifier (item 4).
    pub fn class(&self) -> Option<&'a str> {
        self.algorithm.class()
    }

    /// Number of frames the algorithm operates over (item 5).
    pub fn frames(&self) -> Option<u64> {
        self.algorithm.frames()
    }
}

impl fmt::Debug for St0903Algorithm<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("St0903Algorithm")
            .field("id", &self.id())
            .field("name", &self.name())
            .field("version", &self.version())
            .field("class", &self.class())
            .field("frames", &self.frames())
            .finish()
    }
}

/// Iterator over the Algorithm LSs of a [`St0903Vmti`].
#[derive(Clone)]
#[non_exhaustive]
pub struct St0903Algorithms<'a> {
    algorithms: misb::st0903::SetSeries<'a, misb::st0903::Algorithm<'a>>,
}

impl<'a> Iterator for St0903Algorithms<'a> {
    type Item = Result<St0903Algorithm<'a>, St0903SeriesError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.algorithms.next().map(|result| {
            result
                .map(St0903Algorithm::from_misb)
                .map_err(|_| St0903SeriesError)
        })
    }
}

/// One Ontology LS (ST 0903.6 Table 17): the class of a target as an entry in
/// an OWL ontology.
#[derive(Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct St0903Ontology<'a> {
    ontology: misb::st0903::Ontology<'a>,
}

impl<'a> St0903Ontology<'a> {
    fn from_misb(ontology: misb::st0903::Ontology<'a>) -> Self {
        Self { ontology }
    }

    /// Ontology ID (item 1).
    pub fn id(&self) -> Option<u64> {
        self.ontology.id()
    }

    /// Parent ID (item 2): the ontology entry this one refines.
    pub fn parent_id(&self) -> Option<u64> {
        self.ontology.parent_id()
    }

    /// IRI of the OWL ontology (item 3).
    pub fn ontology_iri(&self) -> Option<&'a str> {
        self.ontology.iri()
    }

    /// IRI of the entity within the ontology (item 4).
    pub fn entity_iri(&self) -> Option<&'a str> {
        self.ontology.class()
    }
}

impl fmt::Debug for St0903Ontology<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("St0903Ontology")
            .field("id", &self.id())
            .field("parent_id", &self.parent_id())
            .field("ontology_iri", &self.ontology_iri())
            .field("entity_iri", &self.entity_iri())
            .finish()
    }
}

/// Iterator over the Ontology LSs of a [`St0903Vmti`].
#[derive(Clone)]
#[non_exhaustive]
pub struct St0903Ontologies<'a> {
    ontologies: misb::st0903::SetSeries<'a, misb::st0903::Ontology<'a>>,
}

impl<'a> Iterator for St0903Ontologies<'a> {
    type Item = Result<St0903Ontology<'a>, St0903SeriesError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ontologies.next().map(|result| {
            result
                .map(St0903Ontology::from_misb)
                .map_err(|_| St0903SeriesError)
        })
    }
}

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

    /// Precision Time Stamp (item 2); `None` when absent or outside the
    /// range a `jiff::Timestamp` represents.
    pub fn timestamp(&self) -> Option<jiff::Timestamp> {
        let micros = i64::try_from(self.set.timestamp()?).ok()?;
        jiff::Timestamp::from_microsecond(micros).ok()
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

    /// The Algorithm Series (item 102): the detectors and trackers that
    /// produced the targets. A target names its algorithm with
    /// [`St0903Target::algorithm_id`].
    pub fn algorithms(&self) -> St0903Algorithms<'a> {
        St0903Algorithms {
            algorithms: self.set.algorithms(),
        }
    }

    /// The algorithm with Algorithm ID `id`, if the Algorithm Series has it.
    pub fn algorithm(&self, id: u64) -> Option<St0903Algorithm<'a>> {
        self.set.algorithm(id).map(St0903Algorithm::from_misb)
    }

    /// The Ontology Series (item 103): the classes a target may belong to.
    pub fn ontologies(&self) -> St0903Ontologies<'a> {
        St0903Ontologies {
            ontologies: self.set.ontologies(),
        }
    }

    /// The raw value of an item `misb` leaves undecoded; `None` when absent.
    /// The bytes are not interpreted here.
    pub fn raw(&self, item: St0903VmtiItem) -> Option<&'a [u8]> {
        self.set.raw(match item {
            St0903VmtiItem::MiisId => misb::st0903::Tag::MiisId,
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
