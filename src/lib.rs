//! Top level crate for data analysis and manipulation tool built on rust
#![warn(clippy::pedantic, clippy::perf, clippy::nursery)]
#![warn(missing_docs)]
#![allow(
    clippy::must_use_candidate,
    clippy::implicit_hasher,
    clippy::use_self,
    clippy::module_name_repetitions,
    clippy::doc_markdown
)]
pub mod core;
pub mod enums;
pub mod io;
mod marcos;
mod plots;
pub mod prelude;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate ndarray;
