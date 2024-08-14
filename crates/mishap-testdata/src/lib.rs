//! Test data for mishap.

use anyhow::anyhow;
use mishap::Mishap;

pub fn complex() -> Mishap {
    let error = anyhow!("anyhow error");
    let error2 = error.context("anyhow error2");
    let mishap1 = Mishap::from_msg_and_anyhow("mishap1 line1\nmishap1 line2", error2);
    let mishap2 = Mishap::from_msg_and_error_tree("mishap2 line1\n\nmishap2 line 2", mishap1);

    let mishap3 = Mishap::from_msg("mishap3 line1\nmishap3 line2");
    let mishap4 = Mishap::from_msg_and_error_trees("mishap4", [mishap2, mishap3]);
    let mishap5 = Mishap::from_msg_and_error_tree("mishap5 line1\nmishap5 line2", mishap4);

    let mishap6 = Mishap::from_msg("mishap6 line1\nmishap6 line2");
    let mishap7 = Mishap::from_msg_and_error_tree("mishap7 line1\nmishap7 line2", mishap6);

    let error3 = anyhow!("anyhow error3");
    let error4 = anyhow!("anyhow error4");

    let mishap8 = Mishap::from_msg_and_anyhows("mishap8 line1\nmishap8 line2", [error3, error4]);

    Mishap::from_msg_and_error_trees(
        "top-level line1\ntop-level line2",
        [mishap5, mishap7, mishap8],
    )
}

pub fn single_source() -> Mishap {
    let error = anyhow!("anyhow error");
    let error2 = error.context("anyhow error2");
    let error3 = error2.context("anyhow error3");
    let mishap1 = Mishap::from_msg_and_anyhow("mishap1 line1\nmishap1 line2", error3);
    Mishap::from_msg_and_error_tree("mishap2 line1\nmishap2 line2", mishap1)
}
