use crate::{ErrorTree, ErrorTreeSource};

impl ErrorTree for anyhow::Error {
    fn sources(&self) -> Box<dyn Iterator<Item = ErrorTreeSource<'_>> + '_> {
        // Represent a standard error as a chain of errors.
        Box::new(self.source().into_iter().map(ErrorTreeSource::Error))
    }
}
