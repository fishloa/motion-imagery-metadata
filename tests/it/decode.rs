use crate::support::{ber_item, pack, series};
use motion_imagery_metadata::{
    St0102Classification, St0102CountryCodingMethod, St0102ObjectCountryCodingMethod, St0601Value,
    St0903GeoPoint, St0903TargetItem, St0903VmtiItem,
};

#[test]
fn scaled_number() {
    match motion_imagery_metadata::decode(5, &[0x71, 0xC2]) {
        St0601Value::Number(degrees) => {
            assert!((degrees - 159.9744).abs() < 1e-3, "{degrees}");
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn precision_time_round_trips_exact_microseconds() {
    for micros in [0u64, 42, 1_245_257_585_099_653] {
        match motion_imagery_metadata::decode(2, &micros.to_be_bytes()) {
            St0601Value::Time(ts) => assert_eq!(
                ts.as_microsecond(),
                i64::try_from(micros).expect("test instant fits i64")
            ),
            other => panic!("{micros}: {other:?}"),
        }
    }
}

#[test]
fn time_above_the_signed_range_is_invalid() {
    assert_eq!(
        motion_imagery_metadata::decode(2, &u64::MAX.to_be_bytes()),
        St0601Value::Invalid
    );
}

#[test]
fn time_outside_the_jiff_range_is_invalid() {
    assert_eq!(
        motion_imagery_metadata::decode(2, &253_402_300_800_000_000u64.to_be_bytes()),
        St0601Value::Invalid
    );
}

#[test]
fn two_byte_tag_decodes_as_time() {
    match motion_imagery_metadata::decode(131, &42u64.to_be_bytes()) {
        St0601Value::Time(ts) => assert_eq!(ts.as_microsecond(), 42),
        other => panic!("{other:?}"),
    }
}

#[test]
fn undefined_tag_is_bytes() {
    assert_eq!(
        motion_imagery_metadata::decode(200, &[1, 2]),
        St0601Value::Bytes(&[1, 2])
    );
}

#[test]
fn sentinel_is_out_of_range() {
    assert_eq!(
        motion_imagery_metadata::decode(6, &[0x80, 0x00]),
        St0601Value::OutOfRange
    );
}

#[test]
fn wrong_length_precision_time_is_invalid() {
    assert_eq!(
        motion_imagery_metadata::decode(2, &[0]),
        St0601Value::Invalid
    );
}

#[test]
fn nested_security_set_is_typed() {
    let value = [0x01, 0x01, 0x01];
    match motion_imagery_metadata::decode(48, &value) {
        St0601Value::Nested(set) => {
            assert_eq!(set.raw(), &value);
            let security = set.security().expect("ST 0102 view");
            assert_eq!(
                security.classification(),
                Some(St0102Classification::Unclassified)
            );
            assert_eq!(
                St0102Classification::Unclassified.to_string(),
                "UNCLASSIFIED"
            );
            assert!(set.vmti().is_none());
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn nested_vmti_set_is_typed() {
    match motion_imagery_metadata::decode(74, &[0x01, 0x01, 0x01]) {
        St0601Value::Nested(set) => {
            let vmti = set.vmti().expect("ST 0903 view");
            assert_eq!(vmti.timestamp(), None);
            assert!(set.security().is_none());
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn packs_and_lists_keep_their_tag_and_raw_value() {
    for tag in [
        60u32, 61, 62, 66, 81, 94, 102, 115, 116, 121, 122, 127, 128, 130,
    ] {
        match motion_imagery_metadata::decode(tag, &[0x12, 0x34]) {
            St0601Value::Pack(pack) => {
                assert_eq!(pack.tag(), tag);
                assert_eq!(pack.raw(), &[0x12, 0x34]);
            }
            other => panic!("tag {tag}: {other:?}"),
        }
    }
    for tag in 138u32..=143 {
        match motion_imagery_metadata::decode(tag, &[0x12, 0x34]) {
            St0601Value::Pack(pack) => {
                assert_eq!(pack.tag(), tag);
                assert_eq!(pack.raw(), &[0x12, 0x34]);
            }
            other => panic!("tag {tag}: {other:?}"),
        }
    }
}

#[test]
fn undecoded_nested_sets_still_name_their_standard() {
    for (tag, name) in [
        (73u32, "ST 0806"),
        (95, "ST 1206"),
        (97, "ST 1002"),
        (98, "ST 1601"),
        (99, "ST 1602"),
        (100, "ST 1607"),
        (101, "ST 1607"),
    ] {
        match motion_imagery_metadata::decode(tag, &[0x01, 0x01, 0x01]) {
            St0601Value::Nested(set) => {
                assert_eq!(set.standard().name(), name, "tag {tag}");
                assert_eq!(set.raw(), &[0x01, 0x01, 0x01], "tag {tag}");
                assert!(set.security().is_none(), "tag {tag}");
                assert!(set.vmti().is_none(), "tag {tag}");
            }
            other => panic!("tag {tag}: {other:?}"),
        }
    }
}

#[test]
fn st0102_security_set_decodes_every_typed_item() {
    // Values from ST 0102.12 Table 2 and the §6.1 examples.
    let mut body = Vec::new();
    body.extend(ber_item(1, &[0x01]));
    body.extend(ber_item(2, &[0x0D]));
    body.extend(ber_item(3, b"//CZE"));
    body.extend(ber_item(12, &[0x0D]));
    body.extend(ber_item(13, &[0x00, b'C', 0x00, b'Z', 0x00, b'E']));
    body.extend(ber_item(22, 12u16.to_be_bytes().as_slice()));

    match motion_imagery_metadata::decode(48, &body) {
        St0601Value::Nested(set) => {
            let security = set.security().expect("ST 0102 view");
            assert_eq!(
                security.classification(),
                Some(St0102Classification::Unclassified)
            );
            assert_eq!(
                security.country_coding_method(),
                Some(St0102CountryCodingMethod::GencTwoLetter)
            );
            assert_eq!(security.classifying_country(), Some("//CZE"));
            assert_eq!(
                security.object_country_coding_method(),
                Some(St0102ObjectCountryCodingMethod::GencTwoLetter)
            );
            let codes = security
                .object_country_codes()
                .expect("object country codes");
            assert_eq!(codes.chars().collect::<String>(), "CZE");
            assert!(codes.eq_str("CZE"));
            assert_eq!(security.version(), Some(12));
            assert_eq!(security.sci_shi_information(), None);
            assert_eq!(security.caveats(), None);
            assert_eq!(security.releasing_instructions(), None);
            assert_eq!(security.classified_by(), None);
            assert_eq!(security.derived_from(), None);
            assert_eq!(security.classification_reason(), None);
            assert_eq!(security.declassification_date(), None);
            assert_eq!(security.classification_and_marking_system(), None);
            assert_eq!(security.classification_comments(), None);
            assert_eq!(security.country_coding_method_version_date(), None);
            assert_eq!(security.object_country_coding_method_version_date(), None);
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn st0903_vmti_set_decodes_every_typed_item() {
    // Values from ST 0903.6 §10 item examples and §10.8 Algorithm LS
    // examples.
    let algorithm = [
        ber_item(1, &[0x09]),
        ber_item(2, b"k6_yolo_9000_tracker"),
        ber_item(3, b"2.6a"),
        ber_item(4, b"kalmann"),
        ber_item(5, &[0x0A]),
    ]
    .concat();
    let algorithm_series = series(&[algorithm]);

    let target = pack(
        1234,
        &[
            ber_item(1, &[0x06, 0x40, 0x00]),
            ber_item(2, &[0x06, 0x40, 0x00]),
            ber_item(3, &[0x06, 0x40, 0x00]),
            ber_item(4, &[0x1B]),
            ber_item(5, &[0x50]),
            ber_item(6, &[0x0A, 0xCD]),
            ber_item(7, &[0x32]),
            ber_item(8, &[0xDA, 0xA5, 0x20]),
            ber_item(9, &[0x33, 0x54]),
            ber_item(10, &[0x3A, 0x66, 0x67]),
            ber_item(11, &[0x3E, 0x66, 0x67]),
            ber_item(12, &[0x2A, 0x94]),
            ber_item(19, &[0x03, 0x68]),
            ber_item(20, &[0x04, 0x71]),
            ber_item(22, &[0x09]),
            ber_item(23, &[0x01]),
        ],
    );

    let mut body = Vec::new();
    body.extend(ber_item(2, &0x0003_8244_30F6_CE40u64.to_be_bytes()));
    body.extend(ber_item(3, b"DSTO_ADSS_VMTI"));
    body.extend(ber_item(4, &[0x06]));
    body.extend(ber_item(5, &[0x1C]));
    body.extend(ber_item(6, &[0x0E]));
    body.extend(ber_item(8, &[0x07, 0x80]));
    body.extend(ber_item(9, &[0x04, 0x38]));
    body.extend(ber_item(10, b"EO Nose"));
    body.extend(ber_item(11, &[0x06, 0x40]));
    body.extend(ber_item(12, &[0x05, 0x00]));
    body.extend(ber_item(101, &target));
    body.extend(ber_item(102, &algorithm_series));

    match motion_imagery_metadata::decode(74, &body) {
        St0601Value::Nested(set) => {
            let vmti = set.vmti().expect("ST 0903 view");
            assert_eq!(
                vmti.timestamp().map(|ts| ts.as_microsecond()),
                Some(987_654_321_000_000)
            );
            assert_eq!(vmti.system_name(), Some("DSTO_ADSS_VMTI"));
            assert_eq!(vmti.version(), Some(6));
            assert_eq!(vmti.total_targets_detected(), Some(28));
            assert_eq!(vmti.targets_reported(), Some(14));
            assert_eq!(vmti.frame_number(), None);
            assert_eq!(vmti.frame_width(), Some(1920));
            assert_eq!(vmti.frame_height(), Some(1080));
            assert_eq!(vmti.source_sensor(), Some("EO Nose"));
            assert!((vmti.horizontal_fov().expect("hfov") - 12.5).abs() < 1e-6);
            assert!((vmti.vertical_fov().expect("vfov") - 10.0).abs() < 1e-6);
            assert_eq!(vmti.raw(St0903VmtiItem::MiisId), None);
            assert_eq!(vmti.raw(St0903VmtiItem::OntologySeries), None);
            assert_eq!(
                vmti.raw(St0903VmtiItem::AlgorithmSeries),
                Some(algorithm_series.as_slice())
            );

            let mut targets = vmti.targets();
            let target = targets.next().expect("one target").expect("valid target");
            assert!(targets.next().is_none());

            assert_eq!(target.id(), 1234);
            assert_eq!(target.centroid_pixel(), Some(409_600));
            assert_eq!(target.centroid_row_col(1920), Some((872, 1137)));
            assert_eq!(target.bounding_box(), Some((409_600, 409_600)));
            assert_eq!(target.priority(), Some(27));
            assert_eq!(target.confidence(), Some(80));
            assert_eq!(target.history(), Some(2765));
            assert_eq!(target.color(), Some(0x00DA_A520));
            let (lat, lon) = target.location_offset().expect("location offset");
            assert!((lat - 10.0).abs() < 1e-6, "{lat}");
            assert!((lon - 12.0).abs() < 1e-6, "{lon}");
            assert!((target.height().expect("height") - 10_000.0).abs() < 1e-6);
            assert_eq!(target.location(), None);
            assert_eq!(target.algorithm_id(), Some(9));
            assert_eq!(
                target.undecoded(St0903TargetItem::PercentageOfTargetPixels),
                Some([0x32].as_slice())
            );
            assert_eq!(
                target.undecoded(St0903TargetItem::TargetIntensity),
                Some([0x33, 0x54].as_slice())
            );
            assert_eq!(
                target.undecoded(St0903TargetItem::DetectionStatus),
                Some([0x01].as_slice())
            );
            assert_eq!(
                target.undecoded(St0903TargetItem::GeospatialContourSeries),
                None
            );

            let center = St0903GeoPoint::new(38.0, -77.0, None);
            let ground = target
                .ground_position(Some(center))
                .expect("ground position");
            assert!((ground.lat() - 48.0).abs() < 1e-6);
            assert!((ground.lon() - -65.0).abs() < 1e-6);
            assert_eq!(ground.height(), Some(10_000.0));
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn display_matches_name() {
    for value in [1u8, 5, 0, 0xFF] {
        let classification = St0102Classification::from_u8(value);
        let country = St0102CountryCodingMethod::from_u8(value);
        let object = St0102ObjectCountryCodingMethod::from_u8(value);
        for (shown, name, reserved) in [
            (
                classification.to_string(),
                classification.name(),
                matches!(classification, St0102Classification::Reserved(_)),
            ),
            (
                country.to_string(),
                country.name(),
                matches!(country, St0102CountryCodingMethod::Reserved(_)),
            ),
            (
                object.to_string(),
                object.name(),
                matches!(object, St0102ObjectCountryCodingMethod::Reserved(_)),
            ),
        ] {
            if reserved {
                assert_eq!(shown, format!("{name}(0x{value:02X})"));
            } else {
                assert_eq!(shown, name);
            }
        }
    }
    assert_eq!(
        St0102Classification::from_u8(0).to_string(),
        "reserved(0x00)"
    );
    assert_eq!(
        St0903VmtiItem::MiisId.to_string(),
        St0903VmtiItem::MiisId.name()
    );
    assert_eq!(
        St0903TargetItem::VChip.to_string(),
        St0903TargetItem::VChip.name()
    );
    match motion_imagery_metadata::decode(73, &[]) {
        St0601Value::Nested(set) => assert_eq!(set.standard().to_string(), "ST 0806"),
        other => panic!("{other:?}"),
    }
}

#[test]
fn vmti_timestamp_outside_the_jiff_range_is_none() {
    for micros in [u64::MAX, 253_402_300_800_000_000] {
        let body = ber_item(2, &micros.to_be_bytes());
        match motion_imagery_metadata::decode(74, &body) {
            St0601Value::Nested(set) => {
                assert_eq!(
                    set.vmti().expect("ST 0903 view").timestamp(),
                    None,
                    "{micros}"
                );
            }
            other => panic!("{other:?}"),
        }
    }
}
