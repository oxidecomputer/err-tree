#[test]
fn test_serialize_complex() {
    let mishap = mishap_testdata::complex();
    let ser = serde_err_tree::Ser::new(mishap);
    let json = serde_json::to_string_pretty(&ser).unwrap();
    expectorate::assert_contents("tests/outputs/serialize-complex.json", &json);
}
