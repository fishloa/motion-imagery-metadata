use broadcast_common::Parse;
use broadcast_common::klv::{KlvItem, decode_ber_length};

const BER_OID_CONTINUATION: u8 = 0x80;
const BER_LOW7_MASK: u8 = 0x7F;

pub fn ber_item(tag: u32, value: &[u8]) -> Vec<u8> {
    let mut out = oid(tag);
    out.push(u8::try_from(value.len()).expect("test item fits one BER length byte"));
    out.extend_from_slice(value);
    out
}

pub fn pack(id: u64, items: &[Vec<u8>]) -> Vec<u8> {
    let mut body = oid(u32::try_from(id).expect("test target id fits u32"));
    for item in items {
        body.extend_from_slice(item);
    }
    let mut out = vec![u8::try_from(body.len()).expect("test pack fits one BER length byte")];
    out.extend_from_slice(&body);
    out
}

pub fn series(entries: &[Vec<u8>]) -> Vec<u8> {
    let mut out = Vec::new();
    for entry in entries {
        out.push(u8::try_from(entry.len()).expect("test entry fits one BER length byte"));
        out.extend_from_slice(entry);
    }
    out
}

pub fn spec(rel: &str, test: &str) -> Option<String> {
    let path = private_dir(test)?.join("specs-verified").join(rel);
    match std::fs::read_to_string(&path) {
        Ok(text) => Some(text),
        Err(error) => panic!("{test}: cannot read {}: {error}", path.display()),
    }
}

pub fn fixture(rel: &str) -> Vec<u8> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join(rel);
    match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) => panic!("cannot read {}: {error}", path.display()),
    }
}

fn private_dir(test: &str) -> Option<std::path::PathBuf> {
    let Some(dir) = std::env::var_os("MIM_PRIVATE_DIR") else {
        if std::env::var_os("MIM_REQUIRE_PRIVATE").as_deref()
            == Some(std::ffi::OsStr::new("1"))
        {
            panic!("{test}: MIM_PRIVATE_DIR is required when MIM_REQUIRE_PRIVATE=1");
        }
        use std::io::Write as _;
        let _ = writeln!(std::io::stderr(), "SKIP {test}: MIM_PRIVATE_DIR not set");
        return None;
    };
    Some(std::path::PathBuf::from(dir))
}

pub fn section<'a>(text: &'a str, start: &str, end: &str) -> &'a str {
    let from = text.find(start).expect("section start");
    let rest = &text[from..];
    let to = rest.find(end).unwrap_or(rest.len());
    &rest[..to]
}

pub fn enum_rows(section: &str) -> Vec<(u8, String)> {
    section
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if !line.starts_with("| `0x") {
                return None;
            }
            let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
            if cells.len() != 2 {
                return None;
            }
            let hex = cells[0].trim_matches('`').trim_start_matches("0x");
            let value = u8::from_str_radix(hex, 16).ok()?;
            Some((value, cells[1].to_string()))
        })
        .collect()
}

pub fn tagged_rows(section: &str) -> Vec<(u32, String)> {
    section
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if !line.starts_with('|') {
                return None;
            }
            let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
            let tag = cells.first()?.parse::<u32>().ok()?;
            let name = cells.get(1)?.to_string();
            if name.is_empty() || name.starts_with('(') || name.starts_with('-') {
                return None;
            }
            Some((tag, name))
        })
        .collect()
}

pub fn tag_rows(text: &str) -> Vec<Vec<String>> {
    text.lines()
        .filter_map(|line| {
            let line = line.trim();
            if !line.starts_with('|') {
                return None;
            }
            let cells: Vec<String> = line
                .trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().to_string())
                .collect();
            cells.first()?.parse::<u32>().ok()?;
            Some(cells)
        })
        .collect()
}

pub fn uas_items(packet: &[u8]) -> Vec<(u32, &[u8])> {
    let item = KlvItem::parse(packet).expect("UAS Local Set packet");
    let mut body = item.value;
    let mut items = Vec::new();
    while !body.is_empty() {
        let (tag, tag_len) = read_oid(body);
        let (len, len_len) = decode_ber_length(&body[tag_len..]).expect("BER length");
        let start = tag_len + len_len;
        let len = usize::try_from(len).expect("item length fits usize");
        let end = start + len;
        items.push((tag, &body[start..end]));
        body = &body[end..];
    }
    items
}

fn oid(tag: u32) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut value = tag;
    loop {
        digits.push((value & u32::from(BER_LOW7_MASK)) as u8);
        value >>= 7;
        if value == 0 {
            break;
        }
    }
    digits.reverse();
    let last = digits.len() - 1;
    for (i, digit) in digits.iter_mut().enumerate() {
        if i != last {
            *digit |= BER_OID_CONTINUATION;
        }
    }
    digits
}

fn read_oid(bytes: &[u8]) -> (u32, usize) {
    let mut value = 0u32;
    for (i, byte) in bytes.iter().enumerate() {
        value = (value << 7) | u32::from(byte & BER_LOW7_MASK);
        if byte & BER_OID_CONTINUATION == 0 {
            return (value, i + 1);
        }
    }
    panic!("truncated BER-OID tag");
}
