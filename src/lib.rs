//! Implementation of the DDS RTPS protocol

#![feature(derive_default_enum)]
#![deny(
    clippy::all,
    missing_debug_implementations,
    missing_docs,
    clippy::cargo
)]
#![warn(clippy::pedantic)]
#![allow(dead_code)] // this is temporary

mod behaviour;
mod discovery;
mod messages;
pub mod structure;
