//! Serde wrappers for error trees.
//!
//! It's useful to be able to serialize error trees over the wire, or to store them in a database.
//! This crate provides a way to do that using serde.

mod adapter;
mod string_tree;

pub use adapter::*;
pub use string_tree::*;
