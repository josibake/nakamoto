//! Nakamoto's client library.
#![allow(clippy::inconsistent_struct_constructor)]
#![allow(clippy::type_complexity)]
#![deny(missing_docs, unsafe_code)]
mod client;
mod error;
mod peer;
mod service;

pub use client::*;
pub mod handle;
pub mod event;

#[cfg(test)]
mod tests;
