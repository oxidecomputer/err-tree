use anyhow::anyhow;
use err_tree::ErrorTreeExt;
use mishap::Mishap;

#[test]
fn test_complex() {
    let error = anyhow!("anyhow error");
    let error2 = error.context("anyhow error2");
    let mishap1 = Mishap::from_anyhow_and_msg("mishap1 line1\nmishap1 line2", error2);
    let mishap2 = Mishap::from_error_tree_and_msg("mishap2 line1\n\nmishap2 line 2", mishap1);

    let mishap3 = Mishap::from_msg("mishap3 line1\nmishap3 line2");
    let mishap4 = Mishap::from_error_trees_and_msg("parent", [mishap2, mishap3]);
    let mishap5 = Mishap::from_error_tree_and_msg("mishap5 line1\nmishap5 line2", mishap4);

    let mishap6 = Mishap::from_msg("mishap6 line1\nmishap6 line2");
    let mishap7 = Mishap::from_error_tree_and_msg("mishap7 line1\nmishap7 line2", mishap6);

    let error3 = anyhow!("anyhow error3");
    let error4 = anyhow!("anyhow error4");

    let mishap8 = Mishap::from_anyhows_and_msg("mishap8 line1\nmishap8 line2", [error3, error4]);

    let top_level = Mishap::from_error_trees_and_msg(
        "top-level line1\ntop-level line2",
        [mishap5, mishap7, mishap8],
    );

    assert_outputs(top_level, "complex");
}

fn assert_outputs(mishap: Mishap, filename_prefix: &str) {
    expectorate::assert_contents(
        format!("tests/outputs/{filename_prefix}-display.txt"),
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

#[test]
fn test_single_source() {
    let error = anyhow!("anyhow error");
    let error2 = error.context("anyhow error2");
    let error3 = error2.context("anyhow error3");
    println!("error2: {:#?}", error3);
    let mishap1 = Mishap::from_anyhow_and_msg("mishap1 line1\nmishap1 line2", error3);
    let mishap2 = Mishap::from_error_tree_and_msg("mishap2 line1\nmishap2 line2", mishap1);

    assert_outputs(mishap2, "single-source");
}
