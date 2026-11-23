//! Decode every item of the private MISB UAS Datalink Local Set fixtures.
//!
//! Run with `cargo run -p motion-imagery-metadata --example decode_uas_packet`.
//! The fixtures live in the private submodule; without a checkout the example
//! reports that and exits 0.

mod common;

const PACKETS: &[&str] = &[
    "misb/DynamicOnlyMISMMSPacketData.bin",
    "misb/DynamicConstantMISMMSPacketData.bin",
];

fn main() {
    for name in PACKETS {
        let Some(packet) = common::packet(name) else {
            println!("{name}: private fixtures are not checked out; skipping");
            continue;
        };
        println!("{name}");
        for (tag, value) in common::items(&packet) {
            let decoded = motion_imagery_metadata::decode(tag, value);
            println!("  tag {tag:>3}: {decoded:?}");
        }
    }
}
