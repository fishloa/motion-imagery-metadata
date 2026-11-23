use broadcast_common::Parse;
use broadcast_common::klv::{KlvItem, decode_ber_length};

const BER_OID_CONTINUATION: u8 = 0x80;
const BER_LOW7_MASK: u8 = 0x7F;

pub fn packet(name: &str) -> Vec<u8> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join(name);
    match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) => panic!("read {}: {error}", path.display()),
    }
}

pub fn items(packet: &[u8]) -> Vec<(u32, &[u8])> {
    let item = KlvItem::parse(packet).expect("UAS Local Set packet");
    let mut body = item.value;
    let mut items = Vec::new();
    while !body.is_empty() {
        let (tag, tag_len) = oid(body);
        let (len, len_len) = decode_ber_length(&body[tag_len..]).expect("BER length");
        let start = tag_len + len_len;
        let len = usize::try_from(len).expect("item length fits usize");
        let end = start + len;
        items.push((tag, &body[start..end]));
        body = &body[end..];
    }
    items
}

fn oid(bytes: &[u8]) -> (u32, usize) {
    let mut value = 0u32;
    for (i, byte) in bytes.iter().enumerate() {
        value = (value << 7) | u32::from(byte & BER_LOW7_MASK);
        if byte & BER_OID_CONTINUATION == 0 {
            return (value, i + 1);
        }
    }
    panic!("truncated BER-OID tag");
}
