use crate::{ErrorTree, ErrorTreeSource};
use std::{error, fmt};

/// Wraps an error to implement [`ErrorTree`] on it.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErrorWrapper<E>(E);

impl<E: error::Error> ErrorWrapper<E> {
    /// Create a new error wrapper.
    #[inline]
    pub fn new(error: E) -> Self {
        ErrorWrapper(error)
    }

    /// Get the wrapped error.
    #[inline]
    pub fn into_inner(self) -> E {
        self.0
    }

    /// Access the wrapped error.
    #[inline]
    pub fn as_inner(&self) -> &E {
        &self.0
    }
}

impl<E: error::Error> From<E> for ErrorWrapper<E> {
    fn from(e: E) -> Self {
        ErrorWrapper(e)
    }
}

impl<E: error::Error> fmt::Debug for ErrorWrapper<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<E: error::Error> fmt::Display for ErrorWrapper<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<E: error::Error> error::Error for ErrorWrapper<E> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.0.source()
    }
}

impl<E: error::Error + Send + Sync> ErrorTree for ErrorWrapper<E> {
    fn sources(&self) -> Box<dyn Iterator<Item = ErrorTreeSource<'_>> + '_> {
        Box::new(self.0.source().map(ErrorTreeSource::Error).into_iter())
    }
}

/// Wraps an [`ErrorTree`] to implement [`Error`](std::error::Error) on it.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErrorTreeWrapper<E> {
    inner: E,
    // This is a representation of
}
