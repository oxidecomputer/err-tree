use crate::Ser;
use err_tree::{ErrorTree, ErrorTreeSource};
use serde::{Deserialize, Serialize};
use std::fmt;

/// An [`ErrorTree`] instance where all elements are strings.
///
/// This tree allows for access to all its sources.
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct StringErrorTree {
    /// The message for this node in the error tree.
    pub msg: String,

    /// The sources of this node.
    pub sources: Vec<StringErrorTree>,
}

impl StringErrorTree {
    /// Creates a new string error tree from an arbitrary error tree.
    pub fn new<ET: ErrorTree>(tree: ET) -> Self {
        Self {
            msg: tree.to_string(),
            sources: tree
                .sources()
                .map(|source| match source {
                    ErrorTreeSource::Error(error) => Self::from_error(error),
                    ErrorTreeSource::Tree(tree) => Self::new(tree),
                })
                .collect(),
        }
    }

    /// Creates a new error tree with the given message and sources.
    pub fn from_msg_and_sources(msg: impl Into<String>, sources: Vec<StringErrorTree>) -> Self {
        Self {
            msg: msg.into(),
            sources,
        }
    }

    /// Creates a new string error tree from an error.
    pub fn from_error<E: std::error::Error>(error: E) -> Self {
        // Can't use `err_tree::ErrorWrapper` here because that requires the error to be
        // `Send + Sync`.
        let source = error.source().map(Self::from_error);
        Self::from_msg_and_sources(error.to_string(), source.into_iter().collect())
    }
}

impl fmt::Display for StringErrorTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl ErrorTree for StringErrorTree {
    fn sources(&self) -> Box<dyn Iterator<Item = ErrorTreeSource<'_>> + '_> {
        Box::new(
            self.sources
                .iter()
                .map(|error| ErrorTreeSource::Tree(error)),
        )
    }
}

impl Serialize for StringErrorTree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use the serializer we already have, which we know has the same format as this one. (We
        // test out roundtrips as part of our tests.)
        Ser::new(self).serialize(serializer)
    }
}
