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
