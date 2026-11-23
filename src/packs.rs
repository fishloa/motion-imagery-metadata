//! Typed views of the ST 0601 packs and lists.
//!
//! [`decode`](crate::decode) returns these items as [`Pack`]; the accessors
//! below parse the raw bytes on demand with the `misb` crate. Each returns
//! `None` when the pack's tag is not that item or the bytes are malformed.
//! Lists iterate without allocating.
//!
//! Layouts follow MISB ST 0601.19 §8.

use misb::st0601::packs as decoder;

use crate::geo::{self, GeoPoint};
use crate::{
    Pack, TAG_ACTIVE_PAYLOADS, TAG_ACTIVE_WAVELENGTH_LIST, TAG_AIRBASE_LOCATIONS,
    TAG_CONTROL_COMMAND, TAG_CONTROL_COMMAND_VERIFICATION, TAG_COUNTRY_CODES,
    TAG_IMAGE_HORIZON_PIXEL_PACK, TAG_PAYLOAD_LIST, TAG_SENSOR_FRAME_RATE_PACK, TAG_VIEW_DOMAIN,
    TAG_WAVELENGTHS_LIST, TAG_WAYPOINT_LIST,
};

fn timestamp(micros: u64) -> Option<jiff::Timestamp> {
    i64::try_from(micros)
        .ok()
        .and_then(|micros| jiff::Timestamp::from_microsecond(micros).ok())
}

/// Image Horizon Pixel Pack (tag 81, ST 0601.19 §8.81): where the horizon
/// crosses the image, optionally with the ground position of each end.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct St0601HorizonPixelPack {
    x0: u8,
    y0: u8,
    x1: u8,
    y1: u8,
    start: Option<GeoPoint>,
    end: Option<GeoPoint>,
}

impl St0601HorizonPixelPack {
    /// Start point, percent of image width from the left.
    pub fn x0(&self) -> u8 {
        self.x0
    }

    /// Start point, percent of image height from the top.
    pub fn y0(&self) -> u8 {
        self.y0
    }

    /// End point, percent of image width from the left.
    pub fn x1(&self) -> u8 {
        self.x1
    }

    /// End point, percent of image height from the top.
    pub fn y1(&self) -> u8 {
        self.y1
    }

    /// Ground position of the start point, in the 20-byte form only.
    pub fn start(&self) -> Option<GeoPoint> {
        self.start
    }

    /// Ground position of the end point, in the 20-byte form only.
    pub fn end(&self) -> Option<GeoPoint> {
        self.end
    }
}

/// Control Command (tag 115, ST 0601.19 §8.115): a command sent to the
/// aircraft.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct St0601ControlCommand<'a> {
    id: u64,
    text: &'a str,
    time: Option<u64>,
}

impl<'a> St0601ControlCommand<'a> {
    /// Command id, referred to by the Control Command Verification List.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// The command, as text.
    pub fn text(&self) -> &'a str {
        self.text
    }

    /// When the command was sent; `None` when absent or outside the range a
    /// `jiff::Timestamp` represents.
    pub fn time(&self) -> Option<jiff::Timestamp> {
        self.time.and_then(timestamp)
    }
}

/// A list of BER-OID integers: Control Command Verification List (tag 116)
/// and Active Wavelength List (tag 121).
#[derive(Clone)]
#[non_exhaustive]
pub struct St0601Oids<'a> {
    inner: decoder::OidList<'a>,
}

impl Iterator for St0601Oids<'_> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.inner.next()
    }
}

impl core::fmt::Debug for St0601Oids<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// Country Codes (tag 122, ST 0601.19 §8.122).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct St0601CountryCodes<'a> {
    coding_method: u64,
    overflight: Option<&'a str>,
    operator: Option<&'a str>,
    manufacture: Option<&'a str>,
}

impl<'a> St0601CountryCodes<'a> {
    /// How the codes are written: the ST 0102 country coding method value,
    /// for example 14 for GENC three-letter codes.
    pub fn coding_method(&self) -> u64 {
        self.coding_method
    }

    /// Country being flown over.
    pub fn overflight(&self) -> Option<&'a str> {
        self.overflight
    }

    /// Country of the operator.
    pub fn operator(&self) -> Option<&'a str> {
        self.operator
    }

    /// Country the aircraft was made in.
    pub fn manufacture(&self) -> Option<&'a str> {
        self.manufacture
    }
}

/// Sensor Frame Rate Pack (tag 127, ST 0601.19 §8.127): frames per second as
/// a fraction, so 59.94 is 60000/1001.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct St0601FrameRate {
    numerator: u64,
    denominator: u64,
}

impl St0601FrameRate {
    /// Numerator.
    pub fn numerator(&self) -> u64 {
        self.numerator
    }

    /// Denominator; 1 when the sender leaves it out.
    pub fn denominator(&self) -> u64 {
        self.denominator
    }

    /// Frames per second; `None` when the denominator is 0.
    pub fn fps(&self) -> Option<f64> {
        (self.denominator != 0).then(|| self.numerator as f64 / self.denominator as f64)
    }
}

/// One record of the Wavelengths List (tag 128).
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct St0601Wavelength<'a> {
    id: u64,
    min_nm: f64,
    max_nm: f64,
    name: &'a str,
}

impl<'a> St0601Wavelength<'a> {
    /// Wavelength id, as used in the Active Wavelength List (tag 121).
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Lower edge of the band, nanometres.
    pub fn min_nm(&self) -> f64 {
        self.min_nm
    }

    /// Upper edge of the band, nanometres.
    pub fn max_nm(&self) -> f64 {
        self.max_nm
    }

    /// Name of the band.
    pub fn name(&self) -> &'a str {
        self.name
    }
}

/// Wavelengths List (tag 128): the bands beyond the predefined ones.
#[derive(Clone)]
#[non_exhaustive]
pub struct St0601Wavelengths<'a> {
    inner: decoder::Wavelengths<'a>,
}

impl<'a> Iterator for St0601Wavelengths<'a> {
    type Item = St0601Wavelength<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|w| St0601Wavelength {
            id: w.id,
            min_nm: w.min_nm,
            max_nm: w.max_nm,
            name: w.name,
        })
    }
}

impl core::fmt::Debug for St0601Wavelengths<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// Airbase Locations (tag 130, ST 0601.19 §8.130): take-off and recovery
/// airfield.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct St0601AirbaseLocations {
    takeoff: Option<GeoPoint>,
    recovery: Option<GeoPoint>,
}

impl St0601AirbaseLocations {
    /// Take-off location; `None` when the sender says it is unknown.
    pub fn takeoff(&self) -> Option<GeoPoint> {
        self.takeoff
    }

    /// Recovery location; `None` when unknown. When the sender leaves it out,
    /// this is the take-off location.
    pub fn recovery(&self) -> Option<GeoPoint> {
        self.recovery
    }
}

/// One record of the Payload List (tag 138).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct St0601Payload<'a> {
    id: u64,
    payload_type: u64,
    name: &'a str,
}

impl<'a> St0601Payload<'a> {
    /// Payload id, the bit number in Active Payloads (tag 139).
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Payload type (ST 0601 enumeration: 0 is electro-optical).
    pub fn payload_type(&self) -> u64 {
        self.payload_type
    }

    /// Name, for example the sensor model.
    pub fn name(&self) -> &'a str {
        self.name
    }
}

/// Payload List (tag 138): the payload count the sender gives and the
/// payload records.
#[derive(Clone)]
#[non_exhaustive]
pub struct St0601Payloads<'a> {
    inner: decoder::PayloadList<'a>,
}

impl<'a> St0601Payloads<'a> {
    /// Number of payloads on the aircraft, as sent; it may exceed the number
    /// of records.
    pub fn declared_count(&self) -> u64 {
        self.inner.count
    }

    /// The payload with id `id`.
    pub fn get(&self, id: u64) -> Option<St0601Payload<'a>> {
        self.inner.get(id).map(|p| St0601Payload {
            id: p.id,
            payload_type: p.payload_type,
            name: p.name,
        })
    }
}

impl<'a> Iterator for St0601Payloads<'a> {
    type Item = St0601Payload<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|p| St0601Payload {
            id: p.id,
            payload_type: p.payload_type,
            name: p.name,
        })
    }
}

impl core::fmt::Debug for St0601Payloads<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// Active Payloads (tag 139): a bit field; bit n set when payload n of the
/// Payload List is on.
#[derive(Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct St0601ActivePayloads<'a> {
    inner: decoder::ActivePayloads<'a>,
}

impl St0601ActivePayloads<'_> {
    /// Whether payload `id` is on.
    pub fn is_active(&self, id: u64) -> bool {
        self.inner.is_active(id)
    }

    /// The ids of the active payloads, lowest first.
    pub fn ids(&self) -> impl Iterator<Item = u64> + '_ {
        self.inner.ids()
    }
}

impl core::fmt::Debug for St0601ActivePayloads<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.ids()).finish()
    }
}

/// One record of the Waypoint List (tag 141).
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct St0601Waypoint {
    id: u64,
    order: i16,
    info: Option<u64>,
    location: Option<GeoPoint>,
}

impl St0601Waypoint {
    /// Waypoint id.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Prosecution order, signed, as the sender numbers it.
    pub fn order(&self) -> i16 {
        self.order
    }

    /// Info bits as sent; `None` when not sent.
    pub fn info(&self) -> Option<u64> {
        self.info
    }

    /// Position, height above the ellipsoid; `None` when not sent.
    pub fn location(&self) -> Option<GeoPoint> {
        self.location
    }

    /// Info bit 0: the aircraft flies to this waypoint under manual control.
    pub fn manual(&self) -> bool {
        self.info.unwrap_or(0) & 1 != 0
    }

    /// Info bit 1: the waypoint was added in flight, not in the plan.
    pub fn ad_hoc(&self) -> bool {
        self.info.unwrap_or(0) & 2 != 0
    }
}

/// Waypoint List (tag 141): the flight plan, in the order sent.
#[derive(Clone)]
#[non_exhaustive]
pub struct St0601Waypoints<'a> {
    inner: decoder::Waypoints<'a>,
}

impl Iterator for St0601Waypoints<'_> {
    type Item = St0601Waypoint;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|w| St0601Waypoint {
            id: w.id,
            order: w.order,
            info: w.info,
            location: w.location.map(geo::from_misb),
        })
    }
}

impl core::fmt::Debug for St0601Waypoints<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// One axis of the View Domain: the sensor can point from `start` over
/// `range` degrees.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct St0601ViewInterval {
    start: f64,
    range: f64,
}

impl St0601ViewInterval {
    /// First angle of the domain, degrees.
    pub fn start(&self) -> f64 {
        self.start
    }

    /// Extent from `start`, degrees (0 to 360).
    pub fn range(&self) -> f64 {
        self.range
    }
}

/// View Domain (tag 142, ST 0601.19 §8.142): how far the sensor can turn
/// relative to the platform.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct St0601ViewDomain {
    azimuth: Option<St0601ViewInterval>,
    elevation: Option<St0601ViewInterval>,
    roll: Option<St0601ViewInterval>,
}

impl St0601ViewDomain {
    /// Relative azimuth domain.
    pub fn azimuth(&self) -> Option<St0601ViewInterval> {
        self.azimuth
    }

    /// Relative elevation domain.
    pub fn elevation(&self) -> Option<St0601ViewInterval> {
        self.elevation
    }

    /// Relative roll domain.
    pub fn roll(&self) -> Option<St0601ViewInterval> {
        self.roll
    }
}

fn interval(view: Option<decoder::ViewInterval>) -> Option<St0601ViewInterval> {
    view.map(|v| St0601ViewInterval {
        start: v.start,
        range: v.range,
    })
}

impl<'a> Pack<'a> {
    fn value(&self, tag: u32) -> Option<&'a [u8]> {
        (self.tag() == tag).then(|| self.raw())
    }

    /// Image Horizon Pixel Pack (tag 81).
    pub fn image_horizon(&self) -> Option<St0601HorizonPixelPack> {
        let pack =
            decoder::HorizonPixelPack::parse(self.value(TAG_IMAGE_HORIZON_PIXEL_PACK)?).ok()?;
        Some(St0601HorizonPixelPack {
            x0: pack.x0,
            y0: pack.y0,
            x1: pack.x1,
            y1: pack.y1,
            start: pack.start.map(geo::from_misb),
            end: pack.end.map(geo::from_misb),
        })
    }

    /// Control Command (tag 115), one command per item.
    pub fn control_commands(&self) -> Option<St0601ControlCommand<'a>> {
        let command = decoder::ControlCommand::parse(self.value(TAG_CONTROL_COMMAND)?).ok()?;
        Some(St0601ControlCommand {
            id: command.id,
            text: command.text,
            time: command.time,
        })
    }

    /// Control Command Verification List (tag 116): the ids of the commands
    /// the aircraft acknowledged.
    pub fn verified_commands(&self) -> Option<St0601Oids<'a>> {
        let inner = decoder::OidList::parse(self.value(TAG_CONTROL_COMMAND_VERIFICATION)?).ok()?;
        Some(St0601Oids { inner })
    }

    /// Active Wavelength List (tag 121): the ids of the wavelengths in use.
    pub fn active_wavelengths(&self) -> Option<St0601Oids<'a>> {
        let inner = decoder::OidList::parse(self.value(TAG_ACTIVE_WAVELENGTH_LIST)?).ok()?;
        Some(St0601Oids { inner })
    }

    /// Country Codes (tag 122).
    pub fn country_codes(&self) -> Option<St0601CountryCodes<'a>> {
        let codes = decoder::CountryCodes::parse(self.value(TAG_COUNTRY_CODES)?).ok()?;
        Some(St0601CountryCodes {
            coding_method: codes.coding_method,
            overflight: codes.overflight,
            operator: codes.operator,
            manufacture: codes.manufacture,
        })
    }

    /// Sensor Frame Rate Pack (tag 127).
    pub fn frame_rate(&self) -> Option<St0601FrameRate> {
        let rate = decoder::FrameRate::parse(self.value(TAG_SENSOR_FRAME_RATE_PACK)?).ok()?;
        Some(St0601FrameRate {
            numerator: rate.numerator,
            denominator: rate.denominator,
        })
    }

    /// Wavelengths List (tag 128).
    pub fn wavelengths(&self) -> Option<St0601Wavelengths<'a>> {
        let inner = decoder::Wavelengths::parse(self.value(TAG_WAVELENGTHS_LIST)?).ok()?;
        Some(St0601Wavelengths { inner })
    }

    /// Airbase Locations (tag 130).
    pub fn airbase_locations(&self) -> Option<St0601AirbaseLocations> {
        let airbases = decoder::AirbaseLocations::parse(self.value(TAG_AIRBASE_LOCATIONS)?).ok()?;
        Some(St0601AirbaseLocations {
            takeoff: airbases.takeoff.map(geo::from_misb),
            recovery: airbases.recovery.map(geo::from_misb),
        })
    }

    /// Payload List (tag 138).
    pub fn payloads(&self) -> Option<St0601Payloads<'a>> {
        let inner = decoder::PayloadList::parse(self.value(TAG_PAYLOAD_LIST)?).ok()?;
        Some(St0601Payloads { inner })
    }

    /// Active Payloads (tag 139).
    pub fn active_payloads(&self) -> Option<St0601ActivePayloads<'a>> {
        let bits = self.value(TAG_ACTIVE_PAYLOADS)?;
        Some(St0601ActivePayloads {
            inner: decoder::ActivePayloads::new(bits),
        })
    }

    /// Waypoint List (tag 141).
    pub fn waypoints(&self) -> Option<St0601Waypoints<'a>> {
        let inner = decoder::Waypoints::parse(self.value(TAG_WAYPOINT_LIST)?).ok()?;
        Some(St0601Waypoints { inner })
    }

    /// View Domain (tag 142).
    pub fn view_domain(&self) -> Option<St0601ViewDomain> {
        let view = decoder::ViewDomain::parse(self.value(TAG_VIEW_DOMAIN)?).ok()?;
        Some(St0601ViewDomain {
            azimuth: interval(view.azimuth),
            elevation: interval(view.elevation),
            roll: interval(view.roll),
        })
    }
}
