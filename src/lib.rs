//! MISB KLV motion-imagery metadata: typed UAS Datalink Local Set item values.
//!
//! MISB ST 0601 carries the telemetry a UAS payload computer emits next to
//! full-motion video (STANAG 4609): platform and sensor attitude, frame-centre
//! position, timestamps, and nested Local Sets for security markings and
//! moving-target indications. This crate takes one ST 0601 item, its BER-OID
//! tag and its value bytes, and returns a typed [`St0601Value`]: the scaled
//! number the tag table defines, text, a UTC instant as a `jiff::Timestamp`, a
//! nested Local Set, or the raw bytes of a pack/list the decoder does not
//! interpret.
//!
//! # Scope
//!
//! This crate decodes item *values*. KLV framing (SMPTE ST 336 / ST 0107: the
//! Universal Label, BER length, BER-OID tags and the UAS Local Set item loop)
//! lives in `transmux`; a caller that already has a `transmux::klv::LocalSetItem`
//! enables that crate's `motion-imagery-metadata` feature and calls
//! `LocalSetItem::value_typed()`, which delegates here.
//!
//! # Delegation
//!
//! The tag table, the linear/IMAPB scalings and the ST 0102/ST 0903 item
//! decoders are the `misb` crate's (ST 0601.19 / ST 0102.12 / ST 0903.6 /
//! ST 1201.5). No `misb` type appears in this crate's public API or in any
//! trait impl; every public type here owns its shape.
//!
//! # What stays raw, and why
//!
//! Items the `misb` decoder leaves as bytes are not re-decoded by hand.
//! Nested sets this crate does not model come back as
//! [`St0601Value::Nested`] with their standard named; their value is reachable
//! through [`NestedSet::raw`]. Packs and lists come back as
//! [`St0601Value::Pack`], reachable through [`Pack::raw`]; typed pack
//! accessors are a later additive change. A tag ST 0601 does not define is
//! [`St0601Value::Bytes`].
//!
//! # Standards
//!
//! - MISB ST 0601.19 UAS Datalink Local Set.
//! - MISB ST 0102.12 Security Metadata Local Set.
//! - MISB ST 0903.6 Video Moving Target Indicator Local Set.
//! - MISB ST 1201.5 Floating Point to Integer Mapping.
//!
//! The standards are published by the NSG MISB.
//!
//! `no_std` + `alloc`.

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

extern crate alloc;

mod st0102;
mod st0903;

pub use st0102::{
    St0102Classification, St0102CountryCodingMethod, St0102ObjectCountryCodes,
    St0102ObjectCountryCodingMethod, St0102Security,
};
pub use st0903::{
    St0903GeoPoint, St0903Target, St0903TargetError, St0903TargetItem, St0903Targets, St0903Vmti,
    St0903VmtiItem,
};

use broadcast_common::impl_spec_display;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

/// Which standard a nested ST 0601 Local Set item carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum NestedStandard {
    /// MISB ST 0102 Security Local Set.
    Security,
    /// MISB ST 0806 RVT Local Set.
    Rvt,
    /// MISB ST 0903 VMTI Local Set.
    Vmti,
    /// MISB ST 1206 SAR Motion Imagery Local Set.
    SarMotionImagery,
    /// MISB ST 1002 Range Image Local Set.
    RangeImage,
    /// MISB ST 1601 Geo-Registration Local Set.
    GeoRegistration,
    /// MISB ST 1602 Composite Imaging Local Set.
    CompositeImaging,
    /// MISB ST 1607 Segment Local Set.
    Segment,
    /// MISB ST 1607 Amend Local Set.
    Amend,
}

impl NestedStandard {
    /// The standard's designation.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Security => "ST 0102",
            Self::Rvt => "ST 0806",
            Self::Vmti => "ST 0903",
            Self::SarMotionImagery => "ST 1206",
            Self::RangeImage => "ST 1002",
            Self::GeoRegistration => "ST 1601",
            Self::CompositeImaging => "ST 1602",
            Self::Segment | Self::Amend => "ST 1607",
        }
    }
}

impl_spec_display!(NestedStandard);

/// A nested ST 0601 Local Set item (tags 48, 73, 74, 95 and 97 through 101).
///
/// The standard is always named; the raw value is always available. The
/// security and VMTI views are present only for tags 48 and 74.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct NestedSet<'a> {
    standard: NestedStandard,
    raw: &'a [u8],
    security: Option<St0102Security<'a>>,
    vmti: Option<St0903Vmti<'a>>,
}

impl<'a> NestedSet<'a> {
    /// The standard the nested set follows.
    pub fn standard(&self) -> NestedStandard {
        self.standard
    }

    /// The nested set's value bytes. This is the escape hatch for items the
    /// workspace does not model; it is not interpreted.
    pub fn raw(&self) -> &'a [u8] {
        self.raw
    }

    /// The ST 0102 Security view (tag 48 only).
    pub fn security(&self) -> Option<St0102Security<'a>> {
        self.security
    }

    /// The ST 0903 VMTI view (tag 74 only).
    pub fn vmti(&self) -> Option<St0903Vmti<'a>> {
        self.vmti
    }
}

/// A pack or list ST 0601 item whose typed accessors do not exist yet.
///
/// `tag` identifies the item and `raw` is its value, uninterpreted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Pack<'a> {
    tag: u32,
    raw: &'a [u8],
}

impl<'a> Pack<'a> {
    /// The BER-OID item tag.
    pub fn tag(&self) -> u32 {
        self.tag
    }

    /// The pack's value bytes, uninterpreted.
    pub fn raw(&self) -> &'a [u8] {
        self.raw
    }
}

/// A decoded MISB ST 0601 Local Set item value.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum St0601Value<'a> {
    /// A scaled real number in the tag's unit.
    Number(f64),
    /// An unsigned integer.
    Unsigned(u64),
    /// A signed integer.
    Signed(i64),
    /// UTF-8 text.
    Text(&'a str),
    /// A UTC instant.
    Time(jiff::Timestamp),
    /// A nested Local Set, with its standard named.
    Nested(NestedSet<'a>),
    /// A pack or list whose typed accessors do not exist yet.
    Pack(Pack<'a>),
    /// Raw bytes of a tag ST 0601 does not define.
    Bytes(&'a [u8]),
    /// The sender marked the value as out of range or not available.
    OutOfRange,
    /// The value does not fit its tag (wrong length, not text, not a number).
    Invalid,
}

// Nested Local Set tags from the ST 0601.19 tag table.
const TAG_SECURITY_LOCAL_SET: u32 = 48;
const TAG_RVT_LOCAL_SET: u32 = 73;
const TAG_VMTI_LOCAL_SET: u32 = 74;
const TAG_SAR_MOTION_IMAGERY_LOCAL_SET: u32 = 95;
const TAG_RANGE_IMAGE_LOCAL_SET: u32 = 97;
const TAG_GEO_REGISTRATION_LOCAL_SET: u32 = 98;
const TAG_COMPOSITE_IMAGING_LOCAL_SET: u32 = 99;
const TAG_SEGMENT_LOCAL_SET: u32 = 100;
const TAG_AMEND_LOCAL_SET: u32 = 101;

// Pack and list tags from the ST 0601.19 tag table, plus defined items
// `misb` 0.2 leaves as bytes.
const TAG_WEAPON_LOAD: u32 = 60;
const TAG_WEAPON_FIRED: u32 = 61;
const TAG_LASER_PRF_CODE: u32 = 62;
const TAG_TARGET_LOCATION_COVARIANCE: u32 = 66;
const TAG_IMAGE_HORIZON_PIXEL_PACK: u32 = 81;
const TAG_MIIS_CORE_IDENTIFIER: u32 = 94;
const TAG_SDCC_FLP: u32 = 102;
const TAG_CONTROL_COMMAND: u32 = 115;
const TAG_CONTROL_COMMAND_VERIFICATION: u32 = 116;
const TAG_ACTIVE_WAVELENGTH_LIST: u32 = 121;
const TAG_COUNTRY_CODES: u32 = 122;
const TAG_SENSOR_FRAME_RATE_PACK: u32 = 127;
const TAG_WAVELENGTHS_LIST: u32 = 128;
const TAG_AIRBASE_LOCATIONS: u32 = 130;
const TAG_PAYLOAD_LIST: u32 = 138;
const TAG_ACTIVE_PAYLOADS: u32 = 139;
const TAG_WEAPONS_STORES: u32 = 140;
const TAG_WAYPOINT_LIST: u32 = 141;
const TAG_VIEW_DOMAIN: u32 = 142;
const TAG_METADATA_SUBSTREAM_ID_PACK: u32 = 143;

fn nested_standard(tag: u32) -> Option<NestedStandard> {
    match tag {
        TAG_SECURITY_LOCAL_SET => Some(NestedStandard::Security),
        TAG_RVT_LOCAL_SET => Some(NestedStandard::Rvt),
        TAG_VMTI_LOCAL_SET => Some(NestedStandard::Vmti),
        TAG_SAR_MOTION_IMAGERY_LOCAL_SET => Some(NestedStandard::SarMotionImagery),
        TAG_RANGE_IMAGE_LOCAL_SET => Some(NestedStandard::RangeImage),
        TAG_GEO_REGISTRATION_LOCAL_SET => Some(NestedStandard::GeoRegistration),
        TAG_COMPOSITE_IMAGING_LOCAL_SET => Some(NestedStandard::CompositeImaging),
        TAG_SEGMENT_LOCAL_SET => Some(NestedStandard::Segment),
        TAG_AMEND_LOCAL_SET => Some(NestedStandard::Amend),
        _ => None,
    }
}

fn is_pack(tag: u32) -> bool {
    matches!(
        tag,
        TAG_WEAPON_LOAD
            | TAG_WEAPON_FIRED
            | TAG_LASER_PRF_CODE
            | TAG_TARGET_LOCATION_COVARIANCE
            | TAG_IMAGE_HORIZON_PIXEL_PACK
            | TAG_MIIS_CORE_IDENTIFIER
            | TAG_SDCC_FLP
            | TAG_CONTROL_COMMAND
            | TAG_CONTROL_COMMAND_VERIFICATION
            | TAG_ACTIVE_WAVELENGTH_LIST
            | TAG_COUNTRY_CODES
            | TAG_SENSOR_FRAME_RATE_PACK
            | TAG_WAVELENGTHS_LIST
            | TAG_AIRBASE_LOCATIONS
            | TAG_PAYLOAD_LIST
            | TAG_ACTIVE_PAYLOADS
            | TAG_WEAPONS_STORES
            | TAG_WAYPOINT_LIST
            | TAG_VIEW_DOMAIN
            | TAG_METADATA_SUBSTREAM_ID_PACK
    )
}

/// Decode one MISB ST 0601 Local Set item value.
///
/// `tag` is the item's BER-OID tag and `value` its own value bytes (without
/// the tag or the BER length). The scaling, sentinels and text/number handling
/// are the ST 0601.19 tag table's; see [`St0601Value`] for the result shape.
pub fn decode(tag: u32, value: &[u8]) -> St0601Value<'_> {
    use misb::st0601::Value;

    match misb::st0601::decode_tag(tag, value) {
        Value::Float(v) => St0601Value::Number(v),
        Value::Uint(v) => St0601Value::Unsigned(v),
        Value::Int(v) => St0601Value::Signed(v),
        Value::Text(_) => core::str::from_utf8(value)
            .map(St0601Value::Text)
            .unwrap_or(St0601Value::Invalid),
        Value::Time(micros) => {
            let instant = i64::try_from(micros)
                .ok()
                .and_then(|micros| jiff::Timestamp::from_microsecond(micros).ok());
            match instant {
                Some(ts) => St0601Value::Time(ts),
                None => St0601Value::Invalid,
            }
        }
        Value::Security(set) => St0601Value::Nested(NestedSet {
            standard: NestedStandard::Security,
            raw: value,
            security: Some(St0102Security::from_misb(set)),
            vmti: None,
        }),
        Value::Vmti(set) => St0601Value::Nested(NestedSet {
            standard: NestedStandard::Vmti,
            raw: value,
            security: None,
            vmti: Some(St0903Vmti::from_misb(set)),
        }),
        Value::Bytes(_) => match nested_standard(tag) {
            Some(standard) => St0601Value::Nested(NestedSet {
                standard,
                raw: value,
                security: None,
                vmti: None,
            }),
            None if is_pack(tag) => St0601Value::Pack(Pack { tag, raw: value }),
            None => St0601Value::Bytes(value),
        },
        Value::OutOfRange => St0601Value::OutOfRange,
        Value::Invalid => St0601Value::Invalid,
    }
}
