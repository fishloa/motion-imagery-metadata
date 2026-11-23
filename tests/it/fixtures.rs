use crate::support::{fixture, uas_items};
use motion_imagery_metadata::{
    St0102Classification, St0102CountryCodingMethod, St0102ObjectCountryCodingMethod, St0601Value,
};

const DYNAMIC_ONLY: &str = "misb/DynamicOnlyMISMMSPacketData.bin";
const DYNAMIC_CONSTANT: &str = "misb/DynamicConstantMISMMSPacketData.bin";

fn packet(name: &str) -> Vec<u8> {
    fixture(name)
}

#[test]
fn committed_uas_packet_decodes_its_security_set() {
    let bytes = packet(DYNAMIC_CONSTANT);
    let items = uas_items(&bytes);
    assert!(
        !items.iter().any(|(tag, _)| *tag == 74),
        "the committed packet carries no ST 0903 set"
    );
    let (_, value) = items
        .iter()
        .find(|(tag, _)| *tag == 48)
        .expect("committed packet carries ST 0601 tag 48");
    match motion_imagery_metadata::decode(48, value) {
        St0601Value::Nested(set) => {
            let security = set.security().expect("ST 0102 view");
            assert_eq!(
                security.classification(),
                Some(St0102Classification::Unclassified)
            );
            assert_eq!(
                security.country_coding_method(),
                Some(St0102CountryCodingMethod::Stanag1059ThreeLetter)
            );
            assert_eq!(security.classifying_country(), Some("//USA"));
            assert_eq!(
                security.object_country_coding_method(),
                Some(St0102ObjectCountryCodingMethod::Stanag1059ThreeLetter)
            );
            assert_eq!(
                security
                    .object_country_codes()
                    .expect("object country codes")
                    .chars()
                    .collect::<String>(),
                "USA"
            );
            assert_eq!(security.version(), Some(10));
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn committed_uas_packets_decode_their_heading_and_a_binary_identifier() {
    for name in [DYNAMIC_ONLY, DYNAMIC_CONSTANT] {
        let bytes = packet(name);
        let items = uas_items(&bytes);
        let (_, heading) = items
            .iter()
            .find(|(tag, _)| *tag == 5)
            .expect("committed packet carries ST 0601 tag 5");
        match motion_imagery_metadata::decode(5, heading) {
            St0601Value::Number(degrees) => {
                assert!((degrees - 159.9744).abs() < 1e-3, "{name}: {degrees}");
            }
            other => panic!("{name}: {other:?}"),
        }

        if name == DYNAMIC_CONSTANT {
            let (_, identifier) = items
                .iter()
                .find(|(tag, _)| *tag == 94)
                .expect("the constant packet carries the MIIS Core Identifier");
            match motion_imagery_metadata::decode(94, identifier) {
                St0601Value::Pack(pack) => {
                    assert_eq!(pack.tag(), 94);
                    assert_eq!(pack.raw(), *identifier);
                }
                other => panic!("{name}: {other:?}"),
            }
        }
    }
}

fn stored_and_computed_checksum(packet: &[u8]) -> (u16, u16) {
    let (body, stored) = packet.split_at(packet.len() - 2);
    (
        u16::from_be_bytes([stored[0], stored[1]]),
        misb::klv::checksum(body),
    )
}

#[test]
fn dynamic_only_packet_verifies_its_checksum() {
    let (stored, computed) = stored_and_computed_checksum(&packet(DYNAMIC_ONLY));
    assert_eq!(computed, stored);
}

#[test]
fn dynamic_constant_packet_carries_a_bad_checksum() {
    let (stored, computed) = stored_and_computed_checksum(&packet(DYNAMIC_CONSTANT));
    assert_eq!(stored, 0xAA43);
    assert_eq!(computed, 0x3E1E);
}

#[test]
fn dynamic_constant_packet_differs_from_st0902_annex_c_only_in_tag_20() {
    const TAG_20_VALUE: core::ops::Range<usize> = 125..129;
    let mut bytes = packet(DYNAMIC_CONSTANT);
    assert_eq!(
        bytes[TAG_20_VALUE.start - 2..TAG_20_VALUE.start],
        [0x14, 0x04]
    );
    assert_eq!(bytes[TAG_20_VALUE], [0x7D, 0xC5, 0x5E, 0xCE]);
    bytes[TAG_20_VALUE].fill(0);
    let (stored, computed) = stored_and_computed_checksum(&bytes);
    assert_eq!(computed, stored);
}
