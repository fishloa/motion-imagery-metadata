use crate::support::{ber_item, series};
use motion_imagery_metadata::{St0601Value, decode};

fn algorithm(id: u8, name: &str, version: &str, class: &str, frames: u8) -> Vec<u8> {
    [
        ber_item(1, &[id]),
        ber_item(2, name.as_bytes()),
        ber_item(3, version.as_bytes()),
        ber_item(4, class.as_bytes()),
        ber_item(5, &[frames]),
    ]
    .concat()
}

fn ontology(id: u8, parent: Option<u8>, iri: &str, entity: &str) -> Vec<u8> {
    let mut body = ber_item(1, &[id]);
    if let Some(parent) = parent {
        body.extend(ber_item(2, &[parent]));
    }
    body.extend(ber_item(3, iri.as_bytes()));
    body.extend(ber_item(4, entity.as_bytes()));
    body
}

fn vmti_with(items: &[Vec<u8>]) -> Vec<u8> {
    items.concat()
}

fn vmti(bytes: &[u8]) -> motion_imagery_metadata::St0903Vmti<'_> {
    match decode(74, bytes) {
        St0601Value::Nested(set) => set.vmti().expect("ST 0903 view"),
        other => panic!("{other:?}"),
    }
}

#[test]
fn algorithm_series_spec_examples() {
    // ST 0903.6 §10.8: algorithmId 9, name k6_yolo_9000_tracker, version
    // 2.6a, class kalmann, nFrames 10.
    let entries = [
        algorithm(9, "k6_yolo_9000_tracker", "2.6a", "kalmann", 10),
        algorithm(3, "yolo-v8n", "8.1.0", "detector", 1),
    ];
    let bytes = vmti_with(&[ber_item(102, &series(&entries))]);
    let set = vmti(&bytes);

    let algorithms: Vec<_> = set.algorithms().map(Result::unwrap).collect();
    assert_eq!(algorithms.len(), 2);
    let first = algorithms[0];
    assert_eq!(first.id(), Some(9));
    assert_eq!(first.name(), Some("k6_yolo_9000_tracker"));
    assert_eq!(first.version(), Some("2.6a"));
    assert_eq!(first.class(), Some("kalmann"));
    assert_eq!(first.frames(), Some(10));

    let second = set.algorithm(3).expect("algorithm 3");
    assert_eq!(second.name(), Some("yolo-v8n"));
    assert_eq!(second.class(), Some("detector"));
    assert_eq!(set.algorithm(4), None);
}

#[test]
fn ontology_series_entries_and_parents() {
    let entries = [
        ontology(1, None, "https://example.org/uas", "Vehicle"),
        ontology(2, Some(1), "https://example.org/uas", "Car"),
    ];
    let bytes = vmti_with(&[ber_item(103, &series(&entries))]);
    let set = vmti(&bytes);

    let ontologies: Vec<_> = set.ontologies().map(Result::unwrap).collect();
    assert_eq!(ontologies.len(), 2);
    assert_eq!(ontologies[0].id(), Some(1));
    assert_eq!(ontologies[0].parent_id(), None);
    assert_eq!(
        ontologies[0].ontology_iri(),
        Some("https://example.org/uas")
    );
    assert_eq!(ontologies[0].entity_iri(), Some("Vehicle"));
    assert_eq!(ontologies[1].parent_id(), Some(1));
    assert_eq!(ontologies[1].entity_iri(), Some("Car"));
}

#[test]
fn absent_series_are_empty() {
    let bytes = vmti_with(&[ber_item(2, &[0; 8])]);
    let set = vmti(&bytes);
    assert_eq!(set.algorithms().count(), 0);
    assert_eq!(set.ontologies().count(), 0);
    assert_eq!(set.algorithm(1), None);
}

#[test]
fn malformed_series_element_is_an_error_then_the_end() {
    let bytes = vmti_with(&[ber_item(102, &[5, 1, 2]), ber_item(103, &[9, 1])]);
    let set = vmti(&bytes);
    assert_eq!(
        set.algorithms().map(|r| r.is_err()).collect::<Vec<_>>(),
        [true]
    );
    assert_eq!(
        set.ontologies().map(|r| r.is_err()).collect::<Vec<_>>(),
        [true]
    );
}
