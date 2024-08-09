use crate::{ErrorTree, ErrorTreeSource};
use indent_write::fmt::IndentWriter;
use std::fmt::{self, Write};

/// A displayer for error trees, including their sources, in a tree-like format.
#[derive(Clone, Copy, Debug)]
pub struct ErrorTreeDisplay<'a, ET: ?Sized> {
    tree: &'a ET,
}

impl<'a, ET: ErrorTree + ?Sized> ErrorTreeDisplay<'a, ET> {
    /// Create a new displayer for the given error tree.
    #[inline]
    pub fn new(tree: &'a ET) -> Self {
        Self { tree }
    }
}

impl<'a, ET: ErrorTree + ?Sized> fmt::Display for ErrorTreeDisplay<'a, ET> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display_tree(f, &self.tree)
    }
}

/// A displayer for [`ErrorTreeSource`] in a tree-like format.
#[derive(Clone, Copy, Debug)]
pub struct ErrorTreeSourceDisplay<'a> {
    source: ErrorTreeSource<'a>,
}

impl<'a> ErrorTreeSourceDisplay<'a> {
    /// Create a new displayer for the given error tree source.
    #[inline]
    pub fn new(source: ErrorTreeSource<'a>) -> Self {
        Self { source }
    }
}

impl<'a> fmt::Display for ErrorTreeSourceDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.source {
            ErrorTreeSource::Error(error) => display_error(f, error),
            ErrorTreeSource::Tree(tree) => display_tree(f, tree),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DisplayKind {
    Single,
    Multi,
}

fn display_tree(f: &mut dyn fmt::Write, tree: &dyn ErrorTree) -> fmt::Result {
    write!(f, "{}", tree)?;

    let mut sources = tree.sources().peekable();

    // The behavior depends on the number of sources:
    let Some(first_source) = sources.next() else {
        // * With zero sources, we can return early.
        return Ok(());
    };

    writeln!(f, "\n\nCaused by:\n")?;

    let mut indent = IndentWriter::new("  ", f);

    if sources.peek().is_none() {
        // * With exactly one source, we can display it as a chain.
        display_nested_source(&mut indent, first_source, DisplayKind::Single)?;
    } else {
        // * With more than one source, we need to display it as a tree.
        display_nested_source(&mut indent, first_source, DisplayKind::Multi)?;
        for source in sources {
            display_nested_source(&mut indent, source, DisplayKind::Multi)?;
        }
    }

    Ok(())
}

fn display_error(f: &mut dyn fmt::Write, error: &dyn std::error::Error) -> fmt::Result {
    write!(f, "{}", error)?;

    let Some(source) = error.source() else {
        return Ok(());
    };

    writeln!(f, "\n\nCaused by:")?;

    display_nested_error(f, source, DisplayKind::Single)
}

fn display_nested_source(
    f: &mut dyn fmt::Write,
    source: ErrorTreeSource<'_>,
    parent_kind: DisplayKind,
) -> fmt::Result {
    match source {
        ErrorTreeSource::Error(error) => display_nested_error(f, error, parent_kind),
        ErrorTreeSource::Tree(tree) => display_nested_tree(f, tree, parent_kind),
    }
}

fn display_nested_tree(
    mut f: &mut dyn fmt::Write,
    tree: &dyn ErrorTree,
    parent_kind: DisplayKind,
) -> fmt::Result {
    let mut indent = IndentWriter::new_skip_initial("  ", f);
    match parent_kind {
        DisplayKind::Single => {
            writeln!(indent, "- {}", tree)?;
            f = indent.into_inner();
        }
        DisplayKind::Multi => {
            writeln!(indent, "+ {}", tree)?;
            f = indent.into_inner();
        }
    }

    let mut sources = tree.sources().peekable();

    // The behavior depends on the number of sources:
    let Some(first_source) = sources.next() else {
        // * With zero sources, we can return early.
        return Ok(());
    };

    if sources.peek().is_none() {
        // * With exactly one source, we can display it as a chain.
        match parent_kind {
            DisplayKind::Single => {
                // Single -> single displays can avoid the extra indentation.
                display_nested_source(f, first_source, DisplayKind::Single)?;
            }
            DisplayKind::Multi => {
                // Multi -> single displays need to add an extra indent.
                let mut indent = IndentWriter::new("  ", f);
                display_nested_source(&mut indent, first_source, DisplayKind::Single)?;
            }
        }
    } else {
        // * With more than one source, we need to display it as a tree -- this
        //   always adds extra indentation.
        let mut indent = IndentWriter::new("  ", f);
        display_nested_source(&mut indent, first_source, DisplayKind::Multi)?;
        for source in sources {
            display_nested_source(&mut indent, source, DisplayKind::Multi)?;
        }
    }

    Ok(())
}

fn display_nested_error(
    mut f: &mut dyn fmt::Write,
    error: &dyn std::error::Error,
    parent_kind: DisplayKind,
) -> fmt::Result {
    match parent_kind {
        DisplayKind::Single => {
            let mut indent = IndentWriter::new_skip_initial("  ", f);
            writeln!(indent, "- {}", error)?;
            f = indent.into_inner();

            let mut next = error.source();

            while let Some(source) = next {
                let mut indent = IndentWriter::new_skip_initial("  ", f);
                writeln!(indent, "- {}", source)?;
                next = source.source();
                f = indent.into_inner();
            }
        }
        DisplayKind::Multi => {
            let mut indent = IndentWriter::new_skip_initial("  ", f);
            writeln!(indent, "+ {}", error)?;
            f = indent.into_inner();

            let mut next = error.source();

            while let Some(source) = next {
                // Add an extra indent to show that this is nested.
                let mut indent = IndentWriter::new_skip_initial("    ", f);
                writeln!(indent, "  - {}", source)?;
                next = source.source();
                f = indent.into_inner();
            }
        }
    }

    Ok(())
}
