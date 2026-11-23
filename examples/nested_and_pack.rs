//! Read the typed ST 0102 Security set and the raw MIIS identifier from a real
//! UAS Datalink packet.
//!
//! Run with `cargo run -p motion-imagery-metadata --example nested_and_pack`.
//! The fixture lives in the private submodule; without a checkout the example
//! reports that and exits 0.

mod common;

use motion_imagery_metadata::St0601Value;

const PACKET: &str = "misb/DynamicConstantMISMMSPacketData.bin";

fn main() {
    let Some(packet) = common::packet(PACKET) else {
        println!("{PACKET}: private fixtures are not checked out; skipping");
        return;
    };
    for (tag, value) in common::items(&packet) {
        match motion_imagery_metadata::decode(tag, value) {
            St0601Value::Number(degrees) if tag == 5 => {
                println!("platform heading: {degrees:.5} degrees");
            }
            St0601Value::Nested(set) if tag == 48 => {
                let security = set.security().expect("tag 48 is the ST 0102 set");
                println!(
                    "security: {:?}, classifying country {:?}, ST 0102 version {:?}",
                    security.classification(),
                    security.classifying_country(),
                    security.version()
                );
            }
            St0601Value::Pack(pack) if tag == 94 => {
                println!("MIIS Core Identifier: {} raw bytes", pack.raw().len());
            }
            _ => {}
        }
    }
}
