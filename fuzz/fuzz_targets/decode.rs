#![no_main]

use libfuzzer_sys::fuzz_target;
use motion_imagery_metadata::{St0601Value, St0903GeoPoint, St0903TargetItem, St0903VmtiItem};

const TARGET_ITEMS: [St0903TargetItem; 13] = [
    St0903TargetItem::PercentageOfTargetPixels,
    St0903TargetItem::TargetIntensity,
    St0903TargetItem::BoundingBoxTopLeftLatOffset,
    St0903TargetItem::BoundingBoxTopLeftLonOffset,
    St0903TargetItem::BoundingBoxBottomRightLatOffset,
    St0903TargetItem::BoundingBoxBottomRightLonOffset,
    St0903TargetItem::GeospatialContourSeries,
    St0903TargetItem::DetectionStatus,
    St0903TargetItem::VMask,
    St0903TargetItem::VTracker,
    St0903TargetItem::VChip,
    St0903TargetItem::VChipSeries,
    St0903TargetItem::VObjectSeries,
];

const VMTI_ITEMS: [St0903VmtiItem; 3] = [
    St0903VmtiItem::MiisId,
    St0903VmtiItem::AlgorithmSeries,
    St0903VmtiItem::OntologySeries,
];

fuzz_target!(|input: (u32, &[u8])| {
    let (extra_tag, value) = input;
    for tag in (0..=u32::from(u8::MAX)).chain([extra_tag]) {
        let decoded = motion_imagery_metadata::decode(tag, value);
        let _ = format!("{decoded:?}");
        let St0601Value::Nested(set) = decoded else {
            continue;
        };
        if let Some(vmti) = set.vmti() {
            for item in VMTI_ITEMS {
                let _ = vmti.raw(item);
            }
            let width = vmti.frame_width().unwrap_or(0);
            for target in vmti.targets().flatten() {
                let _ = format!("{target:?}");
                let _ = target.centroid_row_col(width);
                let _ = target.ground_position(Some(St0903GeoPoint::new(0.0, 0.0, None)));
                for item in TARGET_ITEMS {
                    let _ = target.undecoded(item);
                }
            }
        }
    }
});
