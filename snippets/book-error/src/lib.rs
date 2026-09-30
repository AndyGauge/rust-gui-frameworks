//! One error type for the whole book.
//!
//! Every framework has its own error type. An application needs one. The
//! pattern is an enum with a variant per source, a `From` impl for each so `?`
//! converts automatically, and a way to add context on the way up.

mod context;
mod convert;
mod error;

pub use context::Context;
pub use error::{Error, Result};
