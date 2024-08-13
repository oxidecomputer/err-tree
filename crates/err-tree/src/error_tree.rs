use crate::{ErrorTreeDisplay, ErrorTreeSourceDisplay};
use std::{fmt, sync::Arc};

/// An error tree.
///
/// Similar to [`std::error::Error`], except each error can return a list of sources rather than
/// zero or one.
///
/// This is useful for representing a tree of errors obtained by gathering individual errors, where
/// each error can have any number of sources.
///
/// # Similarities and differences with `std::error::Error`
///
/// Like [`std::error::Error`], this trait:
///
/// * Requires that the error implement [`Debug`](fmt::Debug) and [`Display`](fmt::Display).
/// * Is object-safe.
///
/// Unlike [`std::error::Error`], this trait requires [`Send`] and [`Sync`] to be implemented.
pub trait ErrorTree: fmt::Debug + fmt::Display + Send + Sync {
    /// Returns all the lower-level sources of this error.
    ///
    /// This is similar to [`std::error::Error::source`], except it returns an
    /// iterator of all the causes, rather than just one.
    fn sources(&self) -> Box<dyn Iterator<Item = ErrorTreeSource<'_>> + '_>;

    /// Converts the error tree into a boxed trait object.
    fn into_boxed(self) -> Box<dyn ErrorTree>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

impl<T> ErrorTree for Box<T>
where
    T: ErrorTree,
{
    #[inline]
    fn sources(&self) -> Box<dyn Iterator<Item = ErrorTreeSource<'_>> + '_> {
        (**self).sources()
    }

    fn into_boxed(self) -> Box<dyn ErrorTree>
    where
        T: 'static,
    {
        self
    }
}

impl ErrorTree for Box<dyn ErrorTree> {
    #[inline]
    fn sources(&self) -> Box<dyn Iterator<Item = ErrorTreeSource<'_>> + '_> {
        (**self).sources()
    }

    fn into_boxed(self) -> Box<dyn ErrorTree> {
        self
    }
}

impl<T> ErrorTree for Arc<T>
where
    T: ErrorTree + ?Sized,
{
    #[inline]
    fn sources(&self) -> Box<dyn Iterator<Item = ErrorTreeSource<'_>> + '_> {
        (**self).sources()
    }
}

impl<'a, T> ErrorTree for &'a T
where
    T: ErrorTree + ?Sized,
{
    #[inline]
    fn sources(&self) -> Box<dyn Iterator<Item = ErrorTreeSource<'_>> + '_> {
        (**self).sources()
    }
}

impl<'a, T> ErrorTree for &'a mut T
where
    T: ErrorTree + ?Sized,
{
    #[inline]
    fn sources(&self) -> Box<dyn Iterator<Item = ErrorTreeSource<'_>> + '_> {
        (**self).sources()
    }
}

/// Extension trait for [`ErrorTree`] to provide additional methods.
pub trait ErrorTreeExt: ErrorTree {
    /// Displays the error tree in a tree-like format.
    #[inline]
    fn display_tree(&self) -> ErrorTreeDisplay<'_, Self> {
        ErrorTreeDisplay::new(self)
    }
}

impl<T: ErrorTree + ?Sized> ErrorTreeExt for T {}

/// The source of an error in an error tree.
///
/// Returned by [`ErrorTree::sources`].
#[derive(Clone, Copy, Debug)]
pub enum ErrorTreeSource<'a> {
    /// A [`std::error::Error`] source, representing a chain of errors.
    Error(&'a (dyn std::error::Error + 'static)),

    /// An error tree source.
    Tree(&'a (dyn ErrorTree + 'static)),
}

impl<'a> ErrorTreeSource<'a> {
    /// Returns an iterator of the underlying sources.
    pub fn sources(self) -> Box<dyn Iterator<Item = ErrorTreeSource<'a>> + 'a> {
        match self {
            ErrorTreeSource::Error(error) => {
                Box::new(error.source().into_iter().map(ErrorTreeSource::Error))
            }
            ErrorTreeSource::Tree(tree) => tree.sources(),
        }
    }

    /// Displays the error source in a tree-like format.
    pub fn display_tree(self) -> ErrorTreeSourceDisplay<'a> {
        ErrorTreeSourceDisplay::new(self)
    }
}

impl<'a> fmt::Display for ErrorTreeSource<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorTreeSource::Error(error) => error.fmt(f),
            ErrorTreeSource::Tree(tree) => tree.fmt(f),
        }
    }
}
