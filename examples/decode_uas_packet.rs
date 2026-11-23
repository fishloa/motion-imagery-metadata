//! Decode every item of the committed MISB UAS Datalink Local Set fixtures.
//!
//! Run with `cargo run -p motion-imagery-metadata --example decode_uas_packet`.

mod common;

const PACKETS: &[&str] = &[
    "misb/DynamicOnlyMISMMSPacketData.bin",
    "misb/DynamicConstantMISMMSPacketData.bin",
];

fn main() {
    for name in PACKETS {
        let packet = common::packet(name);
        println!("{name}");
        for (tag, value) in common::items(&packet) {
            let decoded = motion_imagery_metadata::decode(tag, value);
            println!("  tag {tag:>3}: {decoded:?}");
        }
    }
}
