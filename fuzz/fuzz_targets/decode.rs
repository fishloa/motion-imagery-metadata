#![no_main]

use libfuzzer_sys::fuzz_target;
use motion_imagery_metadata::{Pack, St0601Value, St0903GeoPoint, St0903TargetItem, St0903VmtiItem};

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

const VMTI_ITEMS: [St0903VmtiItem; 1] = [St0903VmtiItem::MiisId];

fn exercise_pack(pack: Pack<'_>) {
    let _ = format!("{:?}", pack.image_horizon());
    let _ = format!("{:?}", pack.control_commands());
    let _ = pack.verified_commands().map(Iterator::count);
    let _ = pack.active_wavelengths().map(Iterator::count);
    let _ = format!("{:?}", pack.country_codes());
    let _ = format!("{:?}", pack.frame_rate().and_then(|rate| rate.fps()));
    let _ = pack.wavelengths().map(Iterator::count);
    let _ = format!("{:?}", pack.airbase_locations());
    if let Some(payloads) = pack.payloads() {
        let _ = payloads.declared_count();
        let _ = payloads.get(1);
        let _ = payloads.count();
    }
    if let Some(active) = pack.active_payloads() {
        let _ = active.is_active(1);
        let _ = active.ids().count();
    }
    let _ = pack.waypoints().map(Iterator::count);
    let _ = format!("{:?}", pack.view_domain());
}

fuzz_target!(|input: (u32, &[u8])| {
    let (extra_tag, value) = input;
    for tag in (0..=u32::from(u8::MAX)).chain([extra_tag]) {
        let decoded = motion_imagery_metadata::decode(tag, value);
        let _ = format!("{decoded:?}");
        let set = match decoded {
            St0601Value::Pack(pack) => {
                exercise_pack(pack);
                continue;
            }
            St0601Value::Nested(set) => set,
            _ => continue,
        };
        if let Some(vmti) = set.vmti() {
            for item in VMTI_ITEMS {
                let _ = vmti.raw(item);
            }
            for algorithm in vmti.algorithms().flatten() {
                let _ = format!("{algorithm:?}");
                let _ = algorithm.id().map(|id| vmti.algorithm(id));
            }
            for ontology in vmti.ontologies().flatten() {
                let _ = format!("{ontology:?}");
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
