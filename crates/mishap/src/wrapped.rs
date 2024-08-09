use crate::Mishap;
use err_tree::ErrorTree;
use std::fmt::{self, Write};

/// Extension trait for wrapping error trees with ad-hoc messages.
pub trait WrapErrorTree<T, E>: private::Sealed {
    /// Wrap the error tree with a new ad-hoc message.
    fn wrap_error_tree<D>(self, msg: D) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static;

    /// Wrap the error tree with a new ad-hoc message that is evaluated lazily only once an error
    /// does occur.
    fn wrap_error_tree_with<D, F>(self, f: F) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> D;
}

/// Extension trait for wrapping lists or other iterators of error trees with ad-hoc messages.
pub trait WrapErrorTrees<T, E>: private::Sealed {
    /// Wrap the error tree list with a new ad-hoc message.
    fn wrap_error_trees<D>(self, msg: D) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static;

    /// Wrap the error tree list with a new ad-hoc message that is evaluated lazily only once an error
    /// does occur.
    fn wrap_error_trees_with<D, F>(self, f: F) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> D;
}

/// Extension trait for wrapping individual errors with ad-hoc messages.
pub trait WrapError<T, E>: private::Sealed {
    /// Wrap the error value with a new ad-hoc message.
    fn wrap_error<D>(self, msg: D) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static;

    /// Wrap the error value with a new ad-hoc message that is evaluated lazily only once an error
    /// does occur.
    fn wrap_error_with<D, F>(self, f: F) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> D;
}

/// Extension trait for wrapping lists or other iterators of errors with ad-hoc messages.
pub trait WrapErrors<T, E>: private::Sealed {
    /// Wrap the error list with a new ad-hoc message.
    fn wrap_errors<D>(self, msg: D) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static;

    /// Wrap the error list with a new ad-hoc message that is evaluated lazily only once an error
    /// does occur.
    fn wrap_errors_with<D, F>(self, f: F) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> D;
}

/// Extension trait for wrapping [`anyhow::Error`] errors.
pub trait WrapAnyhow<T>: private::Sealed {
    /// Wrap the error value with a new ad-hoc message.
    fn wrap_anyhow<D>(self, msg: D) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static;

    /// Wrap the error value with a new ad-hoc message that is evaluated lazily only once an error
    /// does occur.
    fn wrap_anyhow_with<D, F>(self, f: F) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> D;
}

/// Extension trait for wrapping [`anyhow::Error`] error lists or other iterators.
pub trait WrapAnyhows<T>: private::Sealed {
    /// Wrap the anyhow list with a new ad-hoc message.
    fn wrap_anyhows<D>(self, msg: D) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static;

    /// Wrap the anyhow list with a new ad-hoc message that is evaluated lazily only once an error
    /// does occur.
    fn wrap_anyhows_with<D, F>(self, f: F) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> D;
}

mod ext {
    use super::*;

    pub(crate) trait StdError {
        #[track_caller]
        fn ext_mishap<D>(self, msg: D) -> Mishap
        where
            D: fmt::Display + Send + Sync + 'static;
    }

    impl<E> StdError for E
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        fn ext_mishap<D>(self, msg: D) -> Mishap
        where
            D: fmt::Display + Send + Sync + 'static,
        {
            Mishap::from_error_and_msg(msg, self)
        }
    }

    pub(crate) trait StdErrorList {
        #[track_caller]
        fn ext_mishap_list<D>(self, msg: D) -> Mishap
        where
            D: fmt::Display + Send + Sync + 'static;
    }

    impl<I, E> StdErrorList for I
    where
        I: IntoIterator<Item = E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        fn ext_mishap_list<D>(self, msg: D) -> Mishap
        where
            D: fmt::Display + Send + Sync + 'static,
        {
            Mishap::from_errors_and_msg(msg, self)
        }
    }
}

impl<T, E> WrapError<T, E> for Result<T, E>
where
    E: ext::StdError,
{
    fn wrap_error<D>(self, msg: D) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|error| error.ext_mishap(msg))
    }

    fn wrap_error_with<D, F>(self, f: F) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> D,
    {
        self.map_err(|error| error.ext_mishap(f()))
    }
}

impl<T, E> WrapErrors<T, E> for Result<T, E>
where
    E: ext::StdErrorList,
{
    fn wrap_errors<D>(self, msg: D) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|error| error.ext_mishap_list(msg))
    }

    fn wrap_errors_with<D, F>(self, f: F) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> D,
    {
        self.map_err(|error| error.ext_mishap_list(f()))
    }
}

impl<T, ET> WrapErrorTree<T, ET> for Result<T, ET>
where
    ET: ErrorTree + 'static,
{
    fn wrap_error_tree<D>(self, msg: D) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|error| Mishap::from_error_tree_and_msg(msg, error))
    }

    fn wrap_error_tree_with<D, F>(self, f: F) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> D,
    {
        self.map_err(|error| Mishap::from_error_tree_and_msg(f(), error))
    }
}

impl<T, I, ET> WrapErrorTrees<T, ET> for Result<T, I>
where
    I: IntoIterator<Item = ET>,
    ET: ErrorTree + 'static,
{
    fn wrap_error_trees<D>(self, msg: D) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|error| Mishap::from_error_trees_and_msg(msg, error))
    }

    fn wrap_error_trees_with<D, F>(self, f: F) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> D,
    {
        self.map_err(|error| Mishap::from_error_trees_and_msg(f(), error))
    }
}

impl<T> WrapAnyhow<T> for anyhow::Result<T> {
    fn wrap_anyhow<D>(self, msg: D) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|error| Mishap::from_anyhow_and_msg(msg, error))
    }

    fn wrap_anyhow_with<D, F>(self, f: F) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> D,
    {
        self.map_err(|error| Mishap::from_anyhow_and_msg(f(), error))
    }
}

impl<I, T> WrapAnyhows<T> for Result<T, I>
where
    I: IntoIterator<Item = anyhow::Error>,
{
    fn wrap_anyhows<D>(self, msg: D) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|error| Mishap::from_anyhows_and_msg(msg, error))
    }

    fn wrap_anyhows_with<D, F>(self, f: F) -> Result<T, Mishap>
    where
        D: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> D,
    {
        self.map_err(|error| Mishap::from_anyhows_and_msg(f(), error))
    }
}

pub(crate) struct WrappedTree<D, E> {
    pub(crate) msg: D,
    // TODO: inline this/DST using unsafe code maybe?
    pub(crate) sources: Box<[E]>,
}

impl<D, E> WrappedTree<D, E> {
    pub(crate) fn new(msg: D, sources: impl IntoIterator<Item = E>) -> Self {
        Self {
            msg,
            sources: sources.into_iter().collect(),
        }
    }
}

impl<D, E> fmt::Display for WrappedTree<D, E>
where
    D: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.msg.fmt(f)
    }
}

impl<D, E> fmt::Debug for WrappedTree<D, E>
where
    D: fmt::Display,
    E: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Error")
            .field("context", &Quoted(&self.msg))
            .field("sources", &self.sources)
            .finish()
    }
}

impl<D, E> ErrorTree for WrappedTree<D, E>
where
    D: Send + Sync + fmt::Display,
    E: ErrorTree + 'static,
{
    fn sources(&self) -> Box<dyn Iterator<Item = err_tree::ErrorTreeSource<'_>> + '_> {
        Box::new(
            self.sources
                .iter()
                .map(|error| err_tree::ErrorTreeSource::Tree(error)),
        )
    }
}

struct Quoted<D>(D);

impl<D> fmt::Debug for Quoted<D>
where
    D: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_char('"')?;
        Quoted(&mut *formatter).write_fmt(format_args!("{}", self.0))?;
        formatter.write_char('"')?;
        Ok(())
    }
}

impl Write for Quoted<&mut fmt::Formatter<'_>> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        fmt::Display::fmt(&s.escape_debug(), self.0)
    }
}

pub(crate) mod private {
    pub trait Sealed {}

    impl<T, E> Sealed for Result<T, E> {}

    impl Sealed for super::Mishap {}
}