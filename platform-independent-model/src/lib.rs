//! Implementation of the DDS RTPS protocol

#![feature(derive_default_enum)]
#![deny(
    clippy::all,
    missing_debug_implementations,
    missing_docs,
    clippy::cargo
)]
#![warn(clippy::pedantic)]

pub mod messages;
pub mod structure;
