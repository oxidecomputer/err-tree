//! **Mishap** is a trait object-based library for dynamically handling [`err_tree::ErrorTree`]
//! instances.
//!
//! Think of it as the equivalent to [`anyhow`] for `ErrorTree` instances.
//!
//! # Details
//!
//! Use `Result<T, mishap::Mishap>`, or equivalently `mishap::Result<T>`, as the return type of
//! functions that may produce error trees.
//!
//! Note that unlike [`anyhow::Error`], [`Mishap`] does not implement `From` for arbitrary error
//! trees or errors. Instead, you must call one of the `wrap_` methods to convert one or more errors
//! or error trees into a `Mishap`.
//!
//! ```
//! use mishap::{Result, WrapError};
//!
//! # #[derive(serde::Deserialize)]
//! # struct BookList;
//!
//! fn get_book_list() -> Result<BookList> {
//!      let json = std::fs::read_to_string("book-list.json")
//!         .wrap_error("failed to read file")?;
//!      let list: BookList = serde_json::from_str(&json)
//!         .wrap_error("failed to parse JSON")?;
//!      Ok(list)
//! }
//! ```
//!
//! TODO: continue this documentation.

mod mishap;
mod wrapped;

pub use mishap::*;
pub use wrapped::*;

/// A type alias for `Result<T, Mishap>`.
pub type Result<T, E = Mishap> = std::result::Result<T, E>;

/// Equivalent to `Ok::<_, mishap::Mishap>(value)`.
///
/// This simplifies creation of a `mishap::Result` in places where type inference
/// cannot deduce the `E` type of the result &mdash; without needing to write
/// `Ok::<_, mishap::Mishap>(value)`.
///
/// One might think that `mishap::Result::Ok(value)` would work in such cases
/// but it does not.
///
/// ```console
/// error[E0282]: type annotations needed for `std::result::Result<i32, E>`
///   --> src/main.rs:11:13
///    |
/// 11 |     let _ = mishap::Result::Ok(1);
///    |         -   ^^^^^^^^^^^^^^^^^^ cannot infer type for type parameter `E` declared on the enum `Result`
///    |         |
///    |         consider giving this pattern the explicit type `std::result::Result<i32, E>`, where the type parameter `E` is specified
/// ```
#[allow(non_snake_case)]
pub fn Ok<T>(t: T) -> Result<T> {
    Result::Ok(t)
}
