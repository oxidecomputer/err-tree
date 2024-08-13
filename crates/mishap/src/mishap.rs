use crate::WrappedTree;
use anyhow::anyhow;
use err_tree::{ErrorTree, ErrorTreeExt, ErrorTreeSource};
use std::fmt;

/// A generic tree of errors, where each error can have any number of sources.
///
/// Like [`anyhow::Error`], but with a tree structure.
///
/// For example:
///
/// * An `anyhow::Error` or other Rust error is logically a chain of errors.
/// * A `Vec<anyhow::Error>` is a list of errors, each of which is a chain.
#[must_use = "this `Mishap` may represent an error and should be handled"]
pub struct Mishap {
    // Use a Box here to keep the size of the struct small.
    //
    // TODO: it would be nice to use something like anyhow's custom vtables for
    // less pointer-chasing.
    kind: Box<TreeImpl>,
}

impl Mishap {
    pub fn from_msg<D>(msg: D) -> Self
    where
        D: fmt::Debug + fmt::Display + Send + Sync + 'static,
    {
        Self {
            kind: TreeImpl::new_chain(anyhow!(msg)),
        }
    }

    pub fn from_anyhow(error: anyhow::Error) -> Self {
        Self {
            kind: TreeImpl::new_chain(error),
        }
    }

    pub fn from_anyhow_and_msg<D>(msg: D, error: anyhow::Error) -> Self
    where
        D: fmt::Display + Send + Sync + 'static,
    {
        Self {
            kind: TreeImpl::new_chain(error.context(msg)),
        }
    }

    pub fn from_anyhows_and_msg<D, I>(msg: D, sources: I) -> Self
    where
        D: fmt::Display + Send + Sync + 'static,
        I: IntoIterator<Item = anyhow::Error>,
    {
        Self {
            kind: TreeImpl::new_wrapped_tree(msg, sources),
        }
    }

    pub fn from_error<E>(error: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self {
            kind: TreeImpl::new_chain(anyhow!(error)),
        }
    }

    pub fn from_error_and_msg<D, E>(msg: D, error: E) -> Self
    where
        D: fmt::Display + Send + Sync + 'static,
        E: std::error::Error + Send + Sync + 'static,
    {
        Self {
            kind: TreeImpl::new_chain(anyhow!(error).context(msg)),
        }
    }

    pub fn from_errors_and_msg<D, I, E>(msg: D, sources: I) -> Self
    where
        D: fmt::Display + Send + Sync + 'static,
        I: IntoIterator<Item = E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        Self {
            kind: TreeImpl::new_wrapped_tree(msg, sources.into_iter().map(|e| anyhow!(e))),
        }
    }

    pub fn from_error_tree<ET>(tree: ET) -> Self
    where
        ET: ErrorTree + 'static,
    {
        Self {
            kind: TreeImpl::new_tree(tree),
        }
    }

    pub fn from_error_tree_and_msg<D, ET>(msg: D, tree: ET) -> Self
    where
        D: fmt::Display + Send + Sync + 'static,
        ET: ErrorTree + 'static,
    {
        Self {
            kind: TreeImpl::new_wrapped_tree(msg, [tree]),
        }
    }

    pub fn from_error_trees_and_msg<D, I, ET>(msg: D, sources: I) -> Self
    where
        D: fmt::Display + Send + Sync + 'static,
        I: IntoIterator<Item = ET>,
        ET: ErrorTree + 'static,
    {
        Self {
            kind: TreeImpl::new_wrapped_tree(msg, sources),
        }
    }

    /// Constructs a tree from a borrowed error, effectively cloning it by stringifying it.
    ///
    /// This doesn't currently preserve `Debug` information.
    pub fn from_borrowed_error(error: &dyn std::error::Error) -> Self {
        let mut chain = vec![error];

        // Construct a tree by stringifying the error.
        #[allow(deprecated)]
        while let Some(cause) = chain.last().expect("at least one item added").cause() {
            chain.push(cause);
        }

        let mut next = anyhow!(chain.pop().expect("at least one item added").to_string());
        while let Some(cause) = chain.pop() {
            let error = next.context(cause.to_string());
            next = error;
        }

        Self {
            kind: TreeImpl::new_chain(next),
        }
    }

    /// Constructs a tree from a borrowed tree, effectively cloning it by stringifying it.
    ///
    /// This doesn't currently preserve `Debug` information.
    pub fn from_borrowed_tree(tree: &dyn ErrorTree) -> Self {
        // Construct a tree by stringifying the tree of errors.
        let sources = tree.sources().map(|source| match source {
            ErrorTreeSource::Error(error) => Self::from_borrowed_error(error),
            ErrorTreeSource::Tree(tree) => Self::from_borrowed_tree(tree),
        });
        Self {
            kind: TreeImpl::new_wrapped_tree(tree.to_string(), sources),
        }
    }

    // The Vec represents a chain of causes rather than siblings.
    pub fn from_msg_and_cause_chain<I, D>(msg: D, cause_chain: I) -> Self
    where
        I: DoubleEndedIterator<Item = D>,
        D: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        let mut next: Option<anyhow::Error> = None;
        for cause in cause_chain.into_iter().rev().chain(std::iter::once(msg)) {
            let error = match next {
                Some(next) => next.context(cause),
                None => anyhow!(cause),
            };
            next = Some(error);
        }

        Self {
            kind: TreeImpl::new_chain(next.unwrap()),
        }
    }

    pub fn wrap_single<D>(self, msg: D) -> Self
    where
        D: fmt::Display + Send + Sync + 'static,
    {
        Self {
            kind: TreeImpl::new_wrapped_tree(msg, [self]),
        }
    }
}

impl fmt::Debug for Mishap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // Similar to anyhow, in this case use the underlying Debug
            // impl.
            return self.kind.fmt(f);
        }

        fmt::Display::fmt(&self.display_tree(), f)
    }
}

impl fmt::Display for Mishap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self.kind {
            TreeImpl::Error(error) => error.fmt(f),
            TreeImpl::Tree(tree) => tree.fmt(f),
        }
    }
}

impl ErrorTree for Mishap {
    fn sources(&self) -> Box<dyn Iterator<Item = ErrorTreeSource<'_>> + '_> {
        match &*self.kind {
            TreeImpl::Error(error) => {
                Box::new(error.source().into_iter().map(ErrorTreeSource::Error))
            }
            TreeImpl::Tree(tree) => tree.sources(),
        }
    }
}

enum TreeImpl {
    /// A chain of errors as an anyhow::Error.
    Error(anyhow::Error),

    /// An error tree.
    Tree(Box<dyn ErrorTree>),
}

impl TreeImpl {
    fn new_chain(error: anyhow::Error) -> Box<Self> {
        Box::new(TreeImpl::Error(error))
    }

    fn new_tree(tree: impl ErrorTree + 'static) -> Box<Self> {
        Box::new(TreeImpl::Tree(Box::new(tree)))
    }

    fn new_wrapped_tree<D, ET>(msg: D, sources: impl IntoIterator<Item = ET>) -> Box<Self>
    where
        D: fmt::Display + Send + Sync + 'static,
        ET: ErrorTree + 'static,
    {
        let sources: Box<[_]> = sources.into_iter().collect();
        if sources.is_empty() {
            // No sources can be simplified to an anyhow error.
            return TreeImpl::new_chain(anyhow!(msg.to_string()));
        }
        Box::new(TreeImpl::Tree(Box::new(WrappedTree::new(msg, sources))))
    }
}

impl fmt::Debug for TreeImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Don't include the outer Error() and Tree() to reduce nesting.
            TreeImpl::Error(error) => error.fmt(f),
            TreeImpl::Tree(tree) => tree.fmt(f),
        }
    }
}
