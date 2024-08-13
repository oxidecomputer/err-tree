use err_tree::ErrorTreeExt;
use mishap::Mishap;

#[test]
fn test_complex() {
    assert_outputs(mishap_testdata::complex(), "complex");
}

#[test]
fn test_single_source() {
    assert_outputs(mishap_testdata::single_source(), "single-source");
}

fn assert_outputs(mishap: Mishap, filename_prefix: &str) {
    expectorate::assert_contents(
        format!("tests/outputs/{filename_prefix}-display.txt"),
        &mishap.to_string(),
    );
    expectorate::assert_contents(
        format!("tests/outputs/{filename_prefix}-display-alternate.txt"),
        &mishap.to_string(),
    );
    expectorate::assert_contents(
        format!("tests/outputs/{filename_prefix}-display-tree.txt"),
        &mishap.display_tree().to_string(),
    );
    expectorate::assert_contents(
        format!("tests/outputs/{filename_prefix}-debug.txt"),
        &format!("{:?}", mishap),
    );
    expectorate::assert_contents(
        format!("tests/outputs/{filename_prefix}-debug-alternate.txt"),
        &format!("{:#?}", mishap),
    );
}
