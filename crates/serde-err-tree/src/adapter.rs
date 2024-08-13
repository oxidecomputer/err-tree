use err_tree::{ErrorTree, ErrorTreeSource};
use serde::{
    ser::{SerializeSeq, SerializeStruct},
    Serialize, Serializer,
};

/// A wrapper type which implements [`Serialize`] for arbitrary error trees.
pub struct Ser<ET> {
    et: ET,
}

impl<ET> Ser<ET> {
    pub fn new(et: ET) -> Self {
        Self { et }
    }

    pub fn into_inner(self) -> ET {
        self.et
    }
}

impl<ET: ErrorTree> Serialize for Ser<ET> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Walk the tree and its sources.

        let mut map = serializer.serialize_struct("ErrorTree", 2)?;
        map.serialize_field(&"msg", &self.et.to_string())?;
        map.serialize_field(&"sources", &SerSources { tree: &self.et })?;

        map.end()
    }
}

impl<ET> From<ET> for Ser<ET> {
    fn from(et: ET) -> Self {
        Self::new(et)
    }
}

struct SerSources<ET> {
    tree: ET,
}

impl<ET: ErrorTree> Serialize for SerSources<ET> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let sources = self.tree.sources();
        let mut seq = serializer.serialize_seq(Some(sources.size_hint().0))?;
        for source in sources {
            seq.serialize_element(&SerSource { source })?;
        }
        seq.end()
    }
}

struct SerSource<'a> {
    source: ErrorTreeSource<'a>,
}

impl<'a> Serialize for SerSource<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.source {
            ErrorTreeSource::Error(error) => SerError { error }.serialize(serializer),
            ErrorTreeSource::Tree(tree) => Ser::new(tree).serialize(serializer),
        }
    }
}

// TODO: worth exposing this?
struct SerError<'a> {
    error: &'a (dyn std::error::Error + 'static),
}

impl<'a> Serialize for SerError<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Use the same serialization format as error trees with one source.
        let mut map = serializer.serialize_struct("ErrorTree", 2)?;
        map.serialize_field(&"msg", &self.error.to_string())?;
        map.serialize_field(
            "sources",
            &SerErrorSources {
                source: self.error.source(),
            },
        )?;
        map.end()
    }
}

struct SerErrorSources<'a> {
    source: Option<&'a (dyn std::error::Error + 'static)>,
}

impl<'a> Serialize for SerErrorSources<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let iter = self.source.into_iter();
        let mut seq = serializer.serialize_seq(Some(iter.size_hint().0))?;
        if let Some(error) = self.source {
            seq.serialize_element(&SerError { error })?;
        }
        seq.end()
    }
}
