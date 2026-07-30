//! Fallout script preparation slice.
//!
//! Wave 1 catalogs structure only. Source and compiled bytes remain opaque
//! until later frontends lower them into the engine-owned IR.
#![allow(dead_code)]

mod attachments;
mod catalog;
pub(crate) mod record;

#[cfg(test)]
mod tests;
