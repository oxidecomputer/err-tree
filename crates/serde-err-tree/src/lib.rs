//! Serde wrappers for [`ErrorTree`](err_tree::ErrorTree) instances.
//!
//! It's often useful to be able to serialize error trees over the wire, or to store them in a
//! database. This crate provides a way to do that using [`serde`].

mod adapter;
mod tree;

pub use adapter::*;
pub use tree::*;
