use motion_imagery_metadata::{Pack, St0102ObjectCountryCodingMethod, St0601Value, decode};

// KWIVER h264_klv.ts packet values, as cross-checked by misb's own tests.
const KW_FRAME_RATE: &[u8] = &[131, 212, 96, 135, 105];
const KW_HORIZON: &[u8] = &[0, 36, 56, 0];
const KW_COMMAND: &[u8] = &[
    5, 17, 70, 108, 121, 32, 116, 111, 32, 87, 97, 121, 112, 111, 105, 110, 116, 32, 49, 0, 0, 0,
    0, 0, 0, 0, 0,
];
const KW_VERIFIED: &[u8] = &[3, 7];
const KW_ACTIVE_WAVE: &[u8] = &[1, 3];
const KW_COUNTRY: &[u8] = &[1, 14, 3, 67, 65, 78, 0, 3, 70, 82, 65];
const KW_WAVELENGTHS: &[u8] = &[13, 21, 0, 0, 7, 208, 0, 0, 15, 160, 78, 78, 73, 82];
const KW_AIRBASE: &[u8] = &[
    11, 64, 107, 194, 9, 25, 189, 165, 84, 7, 14, 0, 11, 64, 120, 60, 184, 25, 162, 146, 116, 7,
    198, 0,
];
const KW_PAYLOADS: &[u8] = &[
    3, 18, 0, 0, 15, 86, 73, 83, 32, 78, 111, 115, 101, 32, 67, 97, 109, 101, 114, 97, 21, 1, 0,
    18, 65, 67, 77, 69, 32, 86, 73, 83, 32, 77, 111, 100, 101, 108, 32, 49, 50, 51, 20, 2, 0, 17,
    65, 67, 77, 69, 32, 73, 82, 32, 77, 111, 100, 101, 108, 32, 52, 53, 54,
];
const KW_ACTIVE_PAYLOADS: &[u8] = &[11];
const KW_WAYPOINTS: &[u8] = &[
    15, 0, 0, 1, 3, 64, 113, 216, 148, 25, 189, 191, 231, 8, 152, 0, 15, 1, 0, 2, 2, 64, 113, 211,
    136, 25, 188, 206, 36, 8, 252, 0, 15, 2, 127, 255, 1, 64, 113, 227, 8, 25, 191, 44, 27, 7, 208,
    0, 15, 3, 255, 254, 0, 64, 113, 229, 175, 25, 191, 90, 167, 9, 96, 0,
];
const KW_VIEW_DOMAIN: &[u8] = &[6, 52, 128, 0, 75, 0, 0, 6, 26, 64, 0, 12, 128, 0];

fn pack(tag: u32, raw: &[u8]) -> Pack<'_> {
    match decode(tag, raw) {
        St0601Value::Pack(pack) => pack,
        other => panic!("tag {tag}: {other:?}"),
    }
}

fn accessors(p: &Pack<'_>) -> Vec<u32> {
    let mut tags = Vec::new();
    if p.image_horizon().is_some() {
        tags.push(81);
    }
    if p.control_commands().is_some() {
        tags.push(115);
    }
    if p.verified_commands().is_some() {
        tags.push(116);
    }
    if p.active_wavelengths().is_some() {
        tags.push(121);
    }
    if p.country_codes().is_some() {
        tags.push(122);
    }
    if p.frame_rate().is_some() {
        tags.push(127);
    }
    if p.wavelengths().is_some() {
        tags.push(128);
    }
    if p.airbase_locations().is_some() {
        tags.push(130);
    }
    if p.payloads().is_some() {
        tags.push(138);
    }
    if p.active_payloads().is_some() {
        tags.push(139);
    }
    if p.waypoints().is_some() {
        tags.push(141);
    }
    if p.view_domain().is_some() {
        tags.push(142);
    }
    tags
}

fn close(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

#[test]
fn only_the_matching_accessor_answers() {
    let valid: [(u32, &[u8]); 12] = [
        (81, KW_HORIZON),
        (115, KW_COMMAND),
        (116, KW_VERIFIED),
        (121, KW_ACTIVE_WAVE),
        (122, KW_COUNTRY),
        (127, KW_FRAME_RATE),
        (128, KW_WAVELENGTHS),
        (130, KW_AIRBASE),
        (138, KW_PAYLOADS),
        (139, KW_ACTIVE_PAYLOADS),
        (141, KW_WAYPOINTS),
        (142, KW_VIEW_DOMAIN),
    ];
    for (tag, raw) in valid {
        assert_eq!(accessors(&pack(tag, raw)), [tag], "tag {tag}");
    }
    assert!(accessors(&pack(60, KW_FRAME_RATE)).is_empty());
}

#[test]
fn malformed_bytes_read_as_none() {
    let cases: [(u32, &[u8]); 11] = [
        (81, &[1, 2, 3]),
        (115, &[5]),
        (116, &[0x81]),
        (121, &[1, 0x81]),
        (122, &KW_COUNTRY[..3]),
        (127, &[0x81]),
        (128, &KW_WAVELENGTHS[..10]),
        (130, &[5, 1, 2, 3, 4, 5]),
        (138, &[0x81]),
        (141, &KW_WAYPOINTS[..20]),
        (142, &[3, 1, 2, 3]),
    ];
    for (tag, raw) in cases {
        assert!(accessors(&pack(tag, raw)).is_empty(), "tag {tag}");
    }
}

#[test]
fn frame_rate_is_a_fraction_with_default_denominator() {
    let rate = pack(127, KW_FRAME_RATE).frame_rate().unwrap();
    assert_eq!((rate.numerator(), rate.denominator()), (60_000, 1_001));
    assert!(close(rate.fps().unwrap(), 59.94006, 1e-5));

    // ST 0601.19 §8.127: 30000/1001 as BER-OIDs.
    let ntsc = pack(127, &[0x81, 0xEA, 0x30, 0x87, 0x69])
        .frame_rate()
        .unwrap();
    assert_eq!((ntsc.numerator(), ntsc.denominator()), (30_000, 1_001));
    assert!(close(ntsc.fps().unwrap(), 29.97003, 1e-5));

    let no_denominator = pack(127, &[25]).frame_rate().unwrap();
    assert_eq!(no_denominator.denominator(), 1);
    assert_eq!(no_denominator.fps(), Some(25.0));

    assert_eq!(pack(127, &[30, 0]).frame_rate().unwrap().fps(), None);
}

#[test]
fn horizon_pixel_pack_pixels_only() {
    let h = pack(81, KW_HORIZON).image_horizon().unwrap();
    assert_eq!((h.x0(), h.y0(), h.x1(), h.y1()), (0, 36, 56, 0));
    assert_eq!((h.start(), h.end()), (None, None));
}

#[test]
fn horizon_pixel_pack_with_ground_positions() {
    let flint = |degrees: f64, max: f64| ((degrees / max) * f64::from(i32::MAX)) as i32;
    let mut raw = vec![0, 40, 100, 45];
    raw.extend_from_slice(&flint(38.9, 90.0).to_be_bytes());
    raw.extend_from_slice(&flint(-77.1, 180.0).to_be_bytes());
    raw.extend_from_slice(&[0x80, 0, 0, 0, 0x80, 0, 0, 0]);
    let h = pack(81, &raw).image_horizon().unwrap();
    assert_eq!((h.x0(), h.y0(), h.x1(), h.y1()), (0, 40, 100, 45));
    let start = h.start().unwrap();
    assert!(close(start.lat(), 38.9, 1e-6) && close(start.lon(), -77.1, 1e-6));
    assert_eq!(h.end(), None);
}

#[test]
fn control_command_text_id_and_time() {
    let c = pack(115, KW_COMMAND).control_commands().unwrap();
    assert_eq!(c.id(), 5);
    assert_eq!(c.text(), "Fly to Waypoint 1");
    assert_eq!(c.time().unwrap().as_microsecond(), 0);

    let no_time = pack(115, &KW_COMMAND[..19]).control_commands().unwrap();
    assert_eq!(no_time.time(), None);
}

#[test]
fn oid_lists_yield_each_id() {
    let verified: Vec<_> = pack(116, KW_VERIFIED)
        .verified_commands()
        .unwrap()
        .collect();
    assert_eq!(verified, [3, 7]);
    let wavelengths: Vec<_> = pack(121, KW_ACTIVE_WAVE)
        .active_wavelengths()
        .unwrap()
        .collect();
    assert_eq!(wavelengths, [1, 3]);
    let multibyte: Vec<_> = pack(116, &[0x81, 0x00, 5])
        .verified_commands()
        .unwrap()
        .collect();
    assert_eq!(multibyte, [128, 5]);
}

#[test]
fn country_codes_by_position() {
    // ST 0601.19 §8.122 example: Canada, (unknown country), France.
    let c = pack(122, KW_COUNTRY).country_codes().unwrap();
    assert_eq!(
        c.coding_method(),
        Some(St0102ObjectCountryCodingMethod::GencThreeLetter)
    );
    assert_eq!(c.overflight(), Some("CAN"));
    assert_eq!(c.operator(), None);
    assert_eq!(c.manufacture(), Some("FRA"));

    let short = pack(122, &[1, 14, 3, 85, 83, 65]).country_codes().unwrap();
    assert_eq!(short.overflight(), Some("USA"));
    assert_eq!((short.operator(), short.manufacture()), (None, None));
}

#[test]
fn wavelengths_list_records() {
    let w: Vec<_> = pack(128, KW_WAVELENGTHS).wavelengths().unwrap().collect();
    assert_eq!(w.len(), 1);
    assert_eq!(w[0].id(), 21);
    assert_eq!((w[0].min_nm(), w[0].max_nm()), (1000.0, 2000.0));
    assert_eq!(w[0].name(), "NNIR");
}

#[test]
fn airbase_locations_takeoff_and_recovery() {
    let a = pack(130, KW_AIRBASE).airbase_locations().unwrap();
    let t = a.takeoff().unwrap();
    let r = a.recovery().unwrap();
    assert!(close(t.lat(), 38.841858983, 1e-8) && close(t.lon(), -77.036784172, 1e-8));
    assert_eq!(t.height(), Some(3.0));
    assert!(close(r.lat(), 38.939352989, 1e-8) && close(r.lon(), -77.459811211, 1e-8));
    assert_eq!(r.height(), Some(95.0));

    let one = pack(130, &KW_AIRBASE[..12]).airbase_locations().unwrap();
    assert_eq!(one.recovery(), one.takeoff());
}

#[test]
fn payload_list_and_active_payloads() {
    let list = pack(138, KW_PAYLOADS).payloads().unwrap();
    assert_eq!(list.declared_count(), 3);
    let names: Vec<_> = list.clone().map(|p| p.name()).collect();
    assert_eq!(
        names,
        ["VIS Nose Camera", "ACME VIS Model 123", "ACME IR Model 456"]
    );
    let first = list.clone().next().unwrap();
    assert_eq!((first.id(), first.payload_type()), (0, 0));
    assert_eq!(list.get(1).unwrap().name(), "ACME VIS Model 123");
    assert!(list.get(9).is_none());

    let active = pack(139, KW_ACTIVE_PAYLOADS).active_payloads().unwrap();
    assert_eq!(active.ids().collect::<Vec<_>>(), [0, 1, 3]);
    assert!(active.is_active(1) && !active.is_active(2) && !active.is_active(64));
}

#[test]
fn waypoint_list_records() {
    let w: Vec<_> = pack(141, KW_WAYPOINTS).waypoints().unwrap().collect();
    assert_eq!(w.len(), 4);
    let want = [
        (0, 1, 3, 38.88942193984985, -77.03516221046448, 200.0),
        (1, 2, 2, 38.889267921447754, -77.04991817474365, 250.0),
        (2, 32767, 1, 38.88974094390869, -77.01293301582336, 100.0),
        (3, -2, 0, 38.889821887016296, -77.01009202003479, 300.0),
    ];
    for (got, (id, order, info, lat, lon, height)) in w.iter().zip(want) {
        assert_eq!((got.id(), got.order(), got.info()), (id, order, Some(info)));
        let p = got.location().unwrap();
        assert!(close(p.lat(), lat, 1e-9) && close(p.lon(), lon, 1e-9));
        assert_eq!(p.height(), Some(height));
    }
    assert!(w[0].manual() && w[0].ad_hoc());
    assert!(!w[1].manual() && w[1].ad_hoc());
    assert!(w[2].manual() && !w[2].ad_hoc());

    let bare: Vec<_> = pack(141, &[3, 7, 0, 1]).waypoints().unwrap().collect();
    assert_eq!(
        (
            bare[0].id(),
            bare[0].order(),
            bare[0].info(),
            bare[0].location()
        ),
        (7, 1, None, None)
    );
}

#[test]
fn view_domain_intervals() {
    let v = pack(142, KW_VIEW_DOMAIN).view_domain().unwrap();
    let az = v.azimuth().unwrap();
    assert_eq!((az.start(), az.range()), (210.0, 300.0));
    let el = v.elevation().unwrap();
    assert_eq!((el.start(), el.range()), (-75.0, 50.0));
    assert_eq!(v.roll(), None);
}
