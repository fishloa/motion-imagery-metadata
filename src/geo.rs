//! The WGS84 position shared by the ST 0601 packs and the ST 0903 VMTI packs.
//!
//! The `misb` crate's position type is converted at this boundary and never
//! returned to callers.

/// A WGS84 position in degrees and metres above the ellipsoid.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct GeoPoint {
    lat: f64,
    lon: f64,
    height: Option<f64>,
}

impl GeoPoint {
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

pub(crate) fn from_misb(point: misb::GeoPoint) -> GeoPoint {
    GeoPoint::new(point.lat, point.lon, point.height)
}

pub(crate) fn into_misb(point: GeoPoint) -> misb::GeoPoint {
    misb::GeoPoint {
        lat: point.lat(),
        lon: point.lon(),
        height: point.height(),
    }
}
