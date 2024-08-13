use crate::Ser;
use err_tree::{ErrorTree, ErrorTreeSource};
use serde::{Deserialize, Serialize};
use std::fmt;

/// An [`ErrorTree`] instance that can be serialized and deserialized.
///
/// The output format is compatible with the one used by the [`Ser`] adapter.
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct SerdeErrorTree {
    /// The message for this node in the error tree.
    pub msg: String,

    /// The sources of this node.
    pub sources: Vec<SerdeErrorTree>,
}

impl SerdeErrorTree {
    /// Creates a new [`SerdeErrorTree`] from an arbitrary error tree.
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
    pub fn from_msg_and_sources(msg: impl Into<String>, sources: Vec<SerdeErrorTree>) -> Self {
        Self {
            msg: msg.into(),
            sources,
        }
    }

    /// Creates a new [`SerdeErrorTree`] from an error.
    pub fn from_error<E: std::error::Error>(error: E) -> Self {
        // Can't use `err_tree::ErrorWrapper` here because that requires the error to be
        // `Send + Sync`.
        let source = error.source().map(Self::from_error);
        Self::from_msg_and_sources(error.to_string(), source.into_iter().collect())
    }
}

impl fmt::Display for SerdeErrorTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl ErrorTree for SerdeErrorTree {
    fn sources(&self) -> Box<dyn Iterator<Item = ErrorTreeSource<'_>> + '_> {
        Box::new(
            self.sources
                .iter()
                .map(|error| ErrorTreeSource::Tree(error)),
        )
    }
}

impl Serialize for SerdeErrorTree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use the serializer we already have, which we know has the same format as this one. (We
        // test out roundtrips as part of our tests.)
        Ser::new(self).serialize(serializer)
    }
}
