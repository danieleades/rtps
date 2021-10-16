//! Implementation of the DDS RTPS protocol

#![feature(derive_default_enum)]
#![deny(
    clippy::all,
    missing_debug_implementations,
    missing_docs,
    clippy::cargo,
    unsafe_code,
)]
#![warn(clippy::pedantic)]
#![allow(dead_code)]

pub mod messages;
pub mod structure;
