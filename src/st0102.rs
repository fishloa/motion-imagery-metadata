//! MISB ST 0102.12 Security Metadata Local Set — a typed, borrowed view.
//!
//! The `misb` crate parses the Local Set framing and the item model; this
//! module exposes that model without leaking the crate's types or the raw
//! item bytes. ST 0601 carries the set as tag 48, so it is reached through
//! [`NestedSet::security`](crate::NestedSet::security) after
//! [`decode`](crate::decode).
//!
//! Item numbers, names and value enumerations: `private/specs-verified/misc/misb/st0102-security-ls.md`
//! (MISB ST 0102.12 Table 2 and §6.1). ST 0102.12 deletes the stream,
//! transport-stream and item-designator keys (its §4 revision history), so
//! they are not part of this view.
//!
//! `no_std` + `alloc`.

use core::fmt;

use broadcast_common::impl_spec_display;

/// Security Classification (ST 0102.12 item 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum St0102Classification {
    /// `UNCLASSIFIED`
    Unclassified,
    /// `RESTRICTED`
    Restricted,
    /// `CONFIDENTIAL`
    Confidential,
    /// `SECRET`
    Secret,
    /// `TOP SECRET`
    TopSecret,
    /// A value ST 0102.12 does not define.
    Reserved(u8),
}

impl St0102Classification {
    /// From the item 1 byte.
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => Self::Unclassified,
            2 => Self::Restricted,
            3 => Self::Confidential,
            4 => Self::Secret,
            5 => Self::TopSecret,
            other => Self::Reserved(other),
        }
    }

    /// The `name()` label.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Unclassified => "UNCLASSIFIED",
            Self::Restricted => "RESTRICTED",
            Self::Confidential => "CONFIDENTIAL",
            Self::Secret => "SECRET",
            Self::TopSecret => "TOP SECRET",
            Self::Reserved(_) => "reserved",
        }
    }
}

impl_spec_display!(St0102Classification, Reserved);

/// Classifying Country and Releasing Instructions Country Coding Method
/// (ST 0102.12 item 2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum St0102CountryCodingMethod {
    /// `ISO-3166 Two Letter`
    Iso3166TwoLetter,
    /// `ISO-3166 Three Letter`
    Iso3166ThreeLetter,
    /// `FIPS 10-4 Two Letter`
    Fips10_4TwoLetter,
    /// `FIPS 10-4 Four Letter`
    Fips10_4FourLetter,
    /// `ISO-3166 Numeric`
    Iso3166Numeric,
    /// `1059 Two Letter`
    Stanag1059TwoLetter,
    /// `1059 Three Letter`
    Stanag1059ThreeLetter,
    /// `Omitted Value`
    Omitted,
    /// `FIPS 10-4 Mixed`
    Fips10_4Mixed,
    /// `ISO 3166 Mixed`
    Iso3166Mixed,
    /// `STANAG 1059 Mixed`
    Stanag1059Mixed,
    /// `GENC Two Letter`
    GencTwoLetter,
    /// `GENC Three Letter`
    GencThreeLetter,
    /// `GENC Numeric`
    GencNumeric,
    /// `GENC Mixed`
    GencMixed,
    /// A value ST 0102.12 does not define.
    Reserved(u8),
}

impl St0102CountryCodingMethod {
    /// From the item 2 byte.
    pub fn from_u8(value: u8) -> Self {
        match value {
            0x01 => Self::Iso3166TwoLetter,
            0x02 => Self::Iso3166ThreeLetter,
            0x03 => Self::Fips10_4TwoLetter,
            0x04 => Self::Fips10_4FourLetter,
            0x05 => Self::Iso3166Numeric,
            0x06 => Self::Stanag1059TwoLetter,
            0x07 => Self::Stanag1059ThreeLetter,
            0x08 | 0x09 => Self::Omitted,
            0x0A => Self::Fips10_4Mixed,
            0x0B => Self::Iso3166Mixed,
            0x0C => Self::Stanag1059Mixed,
            0x0D => Self::GencTwoLetter,
            0x0E => Self::GencThreeLetter,
            0x0F => Self::GencNumeric,
            0x10 => Self::GencMixed,
            other => Self::Reserved(other),
        }
    }

    /// The `name()` label.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Iso3166TwoLetter => "ISO-3166 Two Letter",
            Self::Iso3166ThreeLetter => "ISO-3166 Three Letter",
            Self::Fips10_4TwoLetter => "FIPS 10-4 Two Letter",
            Self::Fips10_4FourLetter => "FIPS 10-4 Four Letter",
            Self::Iso3166Numeric => "ISO-3166 Numeric",
            Self::Stanag1059TwoLetter => "1059 Two Letter",
            Self::Stanag1059ThreeLetter => "1059 Three Letter",
            Self::Omitted => "Omitted Value",
            Self::Fips10_4Mixed => "FIPS 10-4 Mixed",
            Self::Iso3166Mixed => "ISO 3166 Mixed",
            Self::Stanag1059Mixed => "STANAG 1059 Mixed",
            Self::GencTwoLetter => "GENC Two Letter",
            Self::GencThreeLetter => "GENC Three Letter",
            Self::GencNumeric => "GENC Numeric",
            Self::GencMixed => "GENC Mixed",
            Self::Reserved(_) => "reserved",
        }
    }
}

impl_spec_display!(St0102CountryCodingMethod, Reserved);

/// Object Country Coding Method (ST 0102.12 item 12).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum St0102ObjectCountryCodingMethod {
    /// `ISO-3166 Two Letter`
    Iso3166TwoLetter,
    /// `ISO-3166 Three Letter`
    Iso3166ThreeLetter,
    /// `ISO-3166 Numeric`
    Iso3166Numeric,
    /// `FIPS 10-4 Two Letter`
    Fips10_4TwoLetter,
    /// `FIPS 10-4 Four Letter`
    Fips10_4FourLetter,
    /// `1059 Two Letter`
    Stanag1059TwoLetter,
    /// `1059 Three Letter`
    Stanag1059ThreeLetter,
    /// `Omitted Value`
    Omitted,
    /// `GENC Two Letter`
    GencTwoLetter,
    /// `GENC Three Letter`
    GencThreeLetter,
    /// `GENC Numeric`
    GencNumeric,
    /// `GENC AdminSub`
    GencAdminSub,
    /// A value ST 0102.12 does not define.
    Reserved(u8),
}

impl St0102ObjectCountryCodingMethod {
    /// From the item 12 byte.
    pub fn from_u8(value: u8) -> Self {
        match value {
            0x01 => Self::Iso3166TwoLetter,
            0x02 => Self::Iso3166ThreeLetter,
            0x03 => Self::Iso3166Numeric,
            0x04 => Self::Fips10_4TwoLetter,
            0x05 => Self::Fips10_4FourLetter,
            0x06 => Self::Stanag1059TwoLetter,
            0x07 => Self::Stanag1059ThreeLetter,
            0x08..=0x0C => Self::Omitted,
            0x0D => Self::GencTwoLetter,
            0x0E => Self::GencThreeLetter,
            0x0F => Self::GencNumeric,
            0x40 => Self::GencAdminSub,
            other => Self::Reserved(other),
        }
    }

    /// The `name()` label.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Iso3166TwoLetter => "ISO-3166 Two Letter",
            Self::Iso3166ThreeLetter => "ISO-3166 Three Letter",
            Self::Iso3166Numeric => "ISO-3166 Numeric",
            Self::Fips10_4TwoLetter => "FIPS 10-4 Two Letter",
            Self::Fips10_4FourLetter => "FIPS 10-4 Four Letter",
            Self::Stanag1059TwoLetter => "1059 Two Letter",
            Self::Stanag1059ThreeLetter => "1059 Three Letter",
            Self::Omitted => "Omitted Value",
            Self::GencTwoLetter => "GENC Two Letter",
            Self::GencThreeLetter => "GENC Three Letter",
            Self::GencNumeric => "GENC Numeric",
            Self::GencAdminSub => "GENC AdminSub",
            Self::Reserved(_) => "reserved",
        }
    }
}

impl_spec_display!(St0102ObjectCountryCodingMethod, Reserved);

/// Object Country Codes (ST 0102.12 item 13), decoded from RFC 2781 UTF-16
/// (or single-byte text from older senders).
#[derive(Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct St0102ObjectCountryCodes<'a> {
    codes: misb::st0102::CountryCodes<'a>,
}

impl<'a> St0102ObjectCountryCodes<'a> {
    /// The code characters.
    pub fn chars(&self) -> impl Iterator<Item = char> + 'a {
        self.codes.chars()
    }

    /// Compares the decoded characters with `other`.
    pub fn eq_str(&self, other: &str) -> bool {
        self.codes.eq_str(other)
    }
}

impl fmt::Debug for St0102ObjectCountryCodes<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("St0102ObjectCountryCodes")
            .field(&self.chars().collect::<alloc::string::String>())
            .finish()
    }
}

/// A decoded MISB ST 0102.12 Security Local Set (ST 0601 tag 48).
#[derive(Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct St0102Security<'a> {
    set: misb::st0102::SecuritySet<'a>,
}

impl<'a> St0102Security<'a> {
    pub(crate) fn from_misb(set: misb::st0102::SecuritySet<'a>) -> Self {
        Self { set }
    }

    /// Security Classification (item 1).
    pub fn classification(&self) -> Option<St0102Classification> {
        self.set.classification().map(|c| match c {
            misb::st0102::Classification::Unclassified => St0102Classification::Unclassified,
            misb::st0102::Classification::Restricted => St0102Classification::Restricted,
            misb::st0102::Classification::Confidential => St0102Classification::Confidential,
            misb::st0102::Classification::Secret => St0102Classification::Secret,
            misb::st0102::Classification::TopSecret => St0102Classification::TopSecret,
            misb::st0102::Classification::Other(byte) => St0102Classification::Reserved(byte),
        })
    }

    /// Classifying Country and Releasing Instructions Country Coding Method
    /// (item 2).
    pub fn country_coding_method(&self) -> Option<St0102CountryCodingMethod> {
        self.set
            .country_coding_method()
            .map(St0102CountryCodingMethod::from_u8)
    }

    /// Classifying Country (item 3).
    pub fn classifying_country(&self) -> Option<&'a str> {
        self.text(misb::st0102::Tag::ClassifyingCountry)
    }

    /// Security-SCI/SHI information (item 4).
    pub fn sci_shi_information(&self) -> Option<&'a str> {
        self.text(misb::st0102::Tag::SciShiInformation)
    }

    /// Caveats (item 5).
    pub fn caveats(&self) -> Option<&'a str> {
        self.text(misb::st0102::Tag::Caveats)
    }

    /// Releasing Instructions (item 6).
    pub fn releasing_instructions(&self) -> Option<&'a str> {
        self.text(misb::st0102::Tag::ReleasingInstructions)
    }

    /// Classified By (item 7).
    pub fn classified_by(&self) -> Option<&'a str> {
        self.text(misb::st0102::Tag::ClassifiedBy)
    }

    /// Derived From (item 8).
    pub fn derived_from(&self) -> Option<&'a str> {
        self.text(misb::st0102::Tag::DerivedFrom)
    }

    /// Classification Reason (item 9).
    pub fn classification_reason(&self) -> Option<&'a str> {
        self.text(misb::st0102::Tag::ClassificationReason)
    }

    /// Declassification Date (item 10).
    pub fn declassification_date(&self) -> Option<&'a str> {
        self.text(misb::st0102::Tag::DeclassificationDate)
    }

    /// Classification and Marking System (item 11).
    pub fn classification_and_marking_system(&self) -> Option<&'a str> {
        self.text(misb::st0102::Tag::ClassificationAndMarkingSystem)
    }

    /// Object Country Coding Method (item 12).
    pub fn object_country_coding_method(&self) -> Option<St0102ObjectCountryCodingMethod> {
        self.set
            .raw(misb::st0102::Tag::ObjectCountryCodingMethod)
            .and_then(single_byte)
            .map(St0102ObjectCountryCodingMethod::from_u8)
    }

    /// Object Country Codes (item 13).
    pub fn object_country_codes(&self) -> Option<St0102ObjectCountryCodes<'a>> {
        self.set
            .object_country_codes()
            .map(|codes| St0102ObjectCountryCodes { codes })
    }

    /// Classification Comments (item 14).
    pub fn classification_comments(&self) -> Option<&'a str> {
        self.text(misb::st0102::Tag::ClassificationComments)
    }

    /// ST 0102 version (item 22); `None` when absent or wider than the item's
    /// 16-bit width.
    pub fn version(&self) -> Option<u16> {
        self.set
            .version()
            .and_then(|value| u16::try_from(value).ok())
    }

    /// Classifying Country and Releasing Instructions Country Coding Method
    /// Version Date (item 23).
    pub fn country_coding_method_version_date(&self) -> Option<&'a str> {
        self.text(misb::st0102::Tag::CountryCodingMethodVersionDate)
    }

    /// Object Country Coding Method Version Date (item 24).
    pub fn object_country_coding_method_version_date(&self) -> Option<&'a str> {
        self.text(misb::st0102::Tag::ObjectCountryCodingMethodVersionDate)
    }

    fn text(&self, tag: misb::st0102::Tag) -> Option<&'a str> {
        self.set.text(tag)
    }
}

fn single_byte(raw: &[u8]) -> Option<u8> {
    match raw {
        [byte] => Some(*byte),
        _ => None,
    }
}

impl fmt::Debug for St0102Security<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("St0102Security")
            .field("classification", &self.classification())
            .field("country_coding_method", &self.country_coding_method())
            .field("classifying_country", &self.classifying_country())
            .field("sci_shi_information", &self.sci_shi_information())
            .field("caveats", &self.caveats())
            .field("releasing_instructions", &self.releasing_instructions())
            .field("classified_by", &self.classified_by())
            .field("derived_from", &self.derived_from())
            .field("classification_reason", &self.classification_reason())
            .field("declassification_date", &self.declassification_date())
            .field(
                "classification_and_marking_system",
                &self.classification_and_marking_system(),
            )
            .field(
                "object_country_coding_method",
                &self.object_country_coding_method(),
            )
            .field("object_country_codes", &self.object_country_codes())
            .field("classification_comments", &self.classification_comments())
            .field("version", &self.version())
            .field(
                "country_coding_method_version_date",
                &self.country_coding_method_version_date(),
            )
            .field(
                "object_country_coding_method_version_date",
                &self.object_country_coding_method_version_date(),
            )
            .finish()
    }
}
