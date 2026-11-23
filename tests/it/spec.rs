use crate::support::{enum_rows, section, spec, tag_rows, tagged_rows};
use motion_imagery_metadata::{
    St0102Classification, St0102CountryCodingMethod, St0102ObjectCountryCodingMethod, St0601Value,
};

#[test]
fn st0102_enumerations_match_the_transcription() {
    let Some(text) = spec(
        "misc/misb/st0102-security-ls.md",
        "st0102_enumerations_match_the_transcription",
    ) else {
        return;
    };
    let methods = enum_rows(section(&text, "## 3. Item 2", "## 4. Item 12"));
    assert_eq!(methods.len(), 16, "item 2 enumeration rows");
    for (value, label) in methods {
        assert_eq!(
            St0102CountryCodingMethod::from_u8(value).name(),
            label,
            "item 2 0x{value:02X}"
        );
    }

    let object = enum_rows(section(&text, "## 4. Item 12", "## 5. Item 13"));
    assert_eq!(object.len(), 16, "item 12 enumeration rows");
    for (value, label) in object {
        assert_eq!(
            St0102ObjectCountryCodingMethod::from_u8(value).name(),
            label,
            "item 12 0x{value:02X}"
        );
    }

    let classification = text
        .lines()
        .find(|line| line.starts_with("| 1 | Security Classification"))
        .expect("item 1 table row");
    let cells: Vec<&str> = classification
        .trim_matches('|')
        .split('|')
        .map(str::trim)
        .collect();
    let allowed: Vec<&str> = cells[3].split(';').map(str::trim).collect();
    assert_eq!(allowed.len(), 5, "item 1 allowed values");
    for entry in allowed {
        let (code, label) = entry.split_once(' ').expect("code and label");
        let value = u8::from_str_radix(code.trim_matches('`').trim_start_matches("0x"), 16)
            .expect("hex value");
        assert_eq!(
            St0102Classification::from_u8(value).name(),
            label.trim_end_matches("//"),
            "item 1 0x{value:02X}"
        );
    }
    assert_eq!(St0102Classification::from_u8(0).name(), "reserved");
}

#[test]
fn st0903_tables_cover_every_decoded_and_raw_item() {
    let Some(text) = spec(
        "misc/misb/st0903-vmti-ls.md",
        "st0903_tables_cover_every_decoded_and_raw_item",
    ) else {
        return;
    };
    let vmti_rows = tagged_rows(section(&text, "## 2. VMTI LS", "## 3. VTarget Pack"));
    let target_rows = tagged_rows(section(&text, "## 3. VTarget Pack", "## 4. Algorithm LS"));

    const VMTI_DECODED: &[(u32, &str)] = &[
        (2, "precisionTimeStamp"),
        (3, "vmtiSystemName"),
        (4, "vmtiLsVersionNum"),
        (5, "totalNumTargetsDetected"),
        (6, "numTargetsReported"),
        (8, "frameWidth"),
        (9, "frameHeight"),
        (10, "vmtiSourceSensor"),
        (11, "vmtiHorizontalFov"),
        (12, "vmtiVerticalFov"),
        (101, "vTargetSeries"),
    ];
    const VMTI_RAW: &[(u32, &str)] = &[
        (13, "miisId"),
        (102, "algorithmSeries"),
        (103, "ontologySeries"),
    ];
    const VMTI_STANDALONE_ONLY: &[(u32, &str)] = &[(1, "checkSum")];

    for (tag, name) in VMTI_DECODED
        .iter()
        .chain(VMTI_RAW)
        .chain(VMTI_STANDALONE_ONLY)
    {
        assert!(
            vmti_rows.iter().any(|(t, n)| t == tag && n == name),
            "VMTI item {tag} `{name}` missing from the transcription"
        );
    }
    for (tag, name) in &vmti_rows {
        assert!(
            VMTI_DECODED
                .iter()
                .chain(VMTI_RAW)
                .chain(VMTI_STANDALONE_ONLY)
                .any(|(t, n)| t == tag && n == name),
            "VMTI item {tag} `{name}` is neither typed nor documented raw"
        );
    }

    const TARGET_DECODED: &[(u32, &str)] = &[
        (1, "targetCentroid"),
        (2, "boundingBoxTopLeft"),
        (3, "boundingBoxBottomRight"),
        (4, "targetPriority"),
        (5, "targetConfidenceLevel"),
        (6, "targetHistory"),
        (8, "targetColor"),
        (10, "targetLocationOffsetLat"),
        (11, "targetLocationOffsetLon"),
        (12, "targetHae"),
        (17, "targetLocation"),
        (19, "centroidPixRow"),
        (20, "centroidPixCol"),
        (22, "algorithmId"),
    ];
    const TARGET_RAW: &[(u32, &str)] = &[
        (7, "percentageOfTargetPixels"),
        (9, "targetIntensity"),
        (13, "boundingBoxTopLeftLatOffset"),
        (14, "boundingBoxTopLeftLonOffset"),
        (15, "boundingBoxBottomRightLatOffset"),
        (16, "boundingBoxBottomRightLonOffset"),
        (18, "geospatialContourSeries"),
        (23, "detectionStatus"),
        (101, "vMask"),
        (104, "vTracker"),
        (105, "vChip"),
        (106, "vChipSeries"),
        (107, "vObjectSeries"),
    ];
    for (tag, name) in TARGET_DECODED.iter().chain(TARGET_RAW) {
        assert!(
            target_rows.iter().any(|(t, n)| t == tag && n == name),
            "VTarget item {tag} `{name}` missing from the transcription"
        );
    }
    for (tag, name) in &target_rows {
        assert!(
            TARGET_DECODED
                .iter()
                .chain(TARGET_RAW)
                .any(|(t, n)| t == tag && n == name),
            "VTarget item {tag} `{name}` is neither typed nor documented raw"
        );
    }
}

fn nested_standard_name(tag: u32) -> &'static str {
    match tag {
        48 => "ST 0102",
        73 => "ST 0806",
        74 => "ST 0903",
        95 => "ST 1206",
        97 => "ST 1002",
        98 => "ST 1601",
        99 => "ST 1602",
        100 | 101 => "ST 1607",
        other => panic!("tag {other} is not a nested Local Set"),
    }
}

fn item_value(encoding: &str, bytes: &str) -> Vec<u8> {
    match encoding {
        "utf8" => b"x".to_vec(),
        "time" => vec![0; 8],
        "uint" | "int" => vec![0],
        "linear-u" | "linear-s" => {
            let len = bytes.parse::<usize>().expect("fixed linear byte length");
            vec![0; len]
        }
        "imapb" => vec![0, 0],
        "security-set" | "vmti-set" | "set" => vec![0x01, 0x01, 0x01],
        "pack" | "list" | "bytes" | "deprecated" => vec![0x01, 0x02],
        other => panic!("unknown encoding `{other}`"),
    }
}

#[test]
fn st0601_tag_table_kinds_match_the_decoder() {
    let Some(text) = spec(
        "misc/misb/st0601-tag-table.md",
        "st0601_tag_table_kinds_match_the_decoder",
    ) else {
        return;
    };
    let rows = tag_rows(&text);
    assert_eq!(rows.len(), 143, "ST 0601 items 1-143");

    for row in rows {
        let tag = row[0].parse::<u32>().expect("tag number");
        let encoding = row[3].as_str();
        let value = item_value(encoding, &row[4]);
        let decoded = motion_imagery_metadata::decode(tag, &value);
        match encoding {
            "utf8" => assert!(matches!(decoded, St0601Value::Text(_)), "tag {tag}"),
            "time" => assert!(matches!(decoded, St0601Value::Time(_)), "tag {tag}"),
            "int" => assert!(matches!(decoded, St0601Value::Signed(_)), "tag {tag}"),
            // `misb` 0.2 leaves the Laser PRF Code as raw bytes; it is a
            // documented decoder gap, surfaced as a pack rather than guessed.
            "uint" if tag == 62 => assert!(matches!(decoded, St0601Value::Pack(_)), "tag {tag}"),
            "uint" => assert!(matches!(decoded, St0601Value::Unsigned(_)), "tag {tag}"),
            "linear-u" | "linear-s" | "imapb" => {
                assert!(matches!(decoded, St0601Value::Number(_)), "tag {tag}");
            }
            "security-set" | "vmti-set" | "set" => match decoded {
                St0601Value::Nested(set) => {
                    assert_eq!(
                        set.standard().name(),
                        nested_standard_name(tag),
                        "tag {tag}"
                    );
                    assert_eq!(
                        set.security().is_some(),
                        encoding == "security-set",
                        "tag {tag}"
                    );
                    assert_eq!(set.vmti().is_some(), encoding == "vmti-set", "tag {tag}");
                    assert_eq!(set.raw(), value.as_slice(), "tag {tag}");
                }
                other => panic!("tag {tag}: {other:?}"),
            },
            "pack" | "list" | "bytes" | "deprecated" => match decoded {
                St0601Value::Pack(pack) => {
                    assert_eq!(pack.tag(), tag);
                    assert_eq!(pack.raw(), value.as_slice());
                }
                other => panic!("tag {tag}: {other:?}"),
            },
            other => panic!("tag {tag}: unknown encoding `{other}`"),
        }
    }
}
