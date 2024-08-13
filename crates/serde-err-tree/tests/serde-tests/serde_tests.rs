use mishap::Mishap;
use pretty_assertions::assert_eq;
use serde_err_tree::{Ser, StringErrorTree};

#[test]
fn test_complex() {
    test_impl(mishap_testdata::complex(), "complex");
}

#[test]
fn test_single_source() {
    test_impl(mishap_testdata::single_source(), "single-source");
}

fn test_impl(mishap: Mishap, filename_prefix: &str) {
    let ser = Ser::new(&mishap);
    let json = serde_json::to_string_pretty(&ser).unwrap();
    expectorate::assert_contents(
        format!("tests/outputs/{filename_prefix}-serialize.json"),
        &json,
    );

    // Try roundtripping to `StringErrorTree` and back.
    let tree: StringErrorTree = serde_json::from_str(&json).unwrap();
    let ser = Ser::new(&tree);
    let string_json = serde_json::to_string_pretty(&ser).unwrap();
    assert_eq!(json, string_json);

    let tree2: StringErrorTree = serde_json::from_str(&string_json).unwrap();
    assert_eq!(tree, tree2, "trees match after roundtrip");

    // Try constructing a string error tree directly from the mishap.
    let tree3 = StringErrorTree::new(&mishap);
    assert_eq!(tree, tree3, "trees match when constructed directly");
}
